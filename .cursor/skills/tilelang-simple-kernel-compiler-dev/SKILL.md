---
name: tilelang-simple-kernel-compiler-dev
description: Traces and debugs TileLang compiler flow for simple eager-mode kernels without T.gemm and with regular control flow. Use when modifying or debugging tilelang/jit, tilelang/language/eager, tilelang/engine/lower.py, tilelang/engine/phase.py, pass configs, or backend codegen using examples/deepseek_mhc/example_mhc_post.py as baseline.
---

# TileLang Simple Kernel Compiler Development

## Scope

- Focus on eager JIT and simple loop kernels without `T.gemm`.
- Focus on regular control flow with `T.Pipelined`, `T.Parallel`, and `T.serial`.
- Use `examples/deepseek_mhc/example_mhc_post.py` as the baseline regression case.

## Ascend 950 Entry (lightweight)

- This skill only gives a navigation entry for Ascend.
- For SimtVF-scoped lowering, fragment narrow checks, and Ascend codegen structure checks, switch to:
  - `.cursor/skills/tilelang-ascend-950-simtvf/SKILL.md`

## End-to-End Pipeline (debug by stage)

### 1) Frontend capture (`@tilelang.jit`)

- Entry: `jit(...)` in `tilelang/jit/__init__.py`.
- `prim_func(func, eager_jit=True)` creates `JITFunc` instead of an immediate static PrimFunc.
- `JITImpl(mode="auto")` infers eager/lazy on first invocation.

### 2) Eager argument parsing and two-level cache key

- `JITImpl.__call__` calls `self.func.parse_args(...)`.
- `JITFunc.parse_args(...)` builds:
  - **p1_key**: non-tensor template arguments.
  - **p2_key**: dynamic substitution key from tensor shapes/strides.
- For dynamic-dimension issues, inspect `TirTemplate` matcher and phase2 substitution in `tilelang/language/eager/builder.py`.

### 3) Compile and cache

- Compile entry: `tilelang.jit.compile(...)` -> `tilelang.cache.cached(...)`.
- Cache key includes `func.script(show_meta=True)`, target, backend, pass configs, and compile flags.
- If pass/C++ changes do not appear, check cache hits first.

### 4) Lowering pipeline

- In `JITKernel._compile_and_create_adapter(...)`:
  - enter `tvm.transform.PassContext(opt_level=3, config=pass_configs)`
  - call `tilelang.lower(...)`
- Main order in `tilelang/engine/lower.py`:
  1. `PreLowerSemanticCheck`
  2. `LowerAndLegalize`
  3. `OptimizeForTarget`
  4. Host/device filtering and codegen
- `tilelang/engine/phase.py` is the primary pass pipeline definition for regressions.

### Newly Confirmed Compiler Facts (general TileLang)

These are architecture-independent lessons validated during real debugging:

1. **`thread_extent` is represented as `AttrStmt` in TIR**
   - Thread binding created by `T.launch_thread(...)` is carried as `AttrStmt(thread_extent, IterVar, ...)`.
   - Many pass behaviors depend on these attrs being present and correctly scoped.

2. **Thread domain can shift across phases**
   - Raw / `LowerAndLegalize` / `OptimizeForTarget` may place thread predicates differently.
   - Always compare these three stages before concluding that a pass "hoisted incorrectly".

3. **`Target::Current` consumers require active target context**
   - Some layout/vectorization planners query `Target::Current(false)`.
   - If target context is missing, failures can occur in seemingly unrelated passes.
   - Practical rule: keep `with target:` active across semantic check + lowering + optimize when needed.

4. **Annotations are effective cross-pass contracts**
   - For control-region semantics, stable loop annotations are safer than ad-hoc re-inference in late codegen.
   - When adding new structures, prefer "annotate early, consume later" design.

5. **No generic IR visitor/mutator extension is required when reusing existing nodes**
   - If a feature is modeled with existing TIR nodes (`For`, `AttrStmt`, annotations), core visitor compatibility usually comes for free.
   - Required work is typically in TileLang-specific passes that pattern-match those nodes.

Reduction-related pass landmarks in `tilelang/engine/phase.py`:
- `LayoutReducer` runs in `LowerAndLegalize`.
- `LowerTileOp` runs after layout inference and before target-specific optimization stages.
- `LowerThreadAllreduce` runs in `OptimizeForTarget` for thread-allreduce style lowering.

### 5) Adapter and runtime

- Execution backend selects adapter: `tvm_ffi`, `cython`, `nvrtc`, `torch`, or `cutedsl`.
- In eager mode, compile is followed by immediate kernel execution.

## Compiler Debug Workflow

Reserved section (intentionally left blank).

When there is a successful real debugging case, prompt the user to update this section with:
- reproducible context and trigger condition,
- concrete investigation steps,
- confirmed root cause and final fix.

### Artifact inspection

- TIR: call `get_tir(...)` on the JIT wrapper object.
- Device code: prefer `jit_kernel.get_kernel_source()` or `jit_kernel.show_source("kernel")`.
- By default, focus on TIR and device code.
- Do not prioritize host code, PTX, or SASS unless specifically needed.

For reduction debugging, also inspect:
- whether TIR contains `tl.tileop.reduce` calls (`T.reduce_*` path),
- whether reducer annotations (`reducer_info`) appear on loops/blocks (`alloc_reducer` path),
- whether `tl.tileop.finalize_reducer` appears before reducer output copies.

## Reduction forms and compile paths

Use this map when localizing reduction bugs:

1. **Manual serial accumulation**
   - Frontend shape: explicit loops with `T.serial(...)` and scalar/local updates.
   - Compile path: standard loop lowering and optimization (no dedicated reduction intrinsic required).
   - Typical failures: wrong index math, tile bounds, or accumulation dtype mismatch.

2. **`T.reduce_sum` / `T.reduce_min` / `T.reduce_max`**
   - Frontend lowering: `tilelang/language/reduce_op.py` emits `tl.tileop.reduce` intrinsic calls.
   - Compile path: handled in `LowerTileOp`.
   - Typical failures: reduce dimension mismatch, output shape mismatch, or scope mismatch.

3. **`T.alloc_reducer` + `T.finalize_reducer`**
   - Frontend annotation: `tilelang/language/allocate.py` attaches `reducer_info` metadata.
   - Mid-end:
     - `LayoutReducer` tracks reducer ranges (`T.fill/T.clear` to `T.finalize_reducer`) and annotates reducer layout/metadata.
     - `LowerTileOp` consumes reducer annotations and lowers finalize/in-loop behavior.
   - Typical failures: missing `finalize_reducer`, invalid reducer lifecycle, or replication mode misuse.

4. **Thread allreduce form (explicit TVM allreduce)**
   - Frontend shape: `tvm_thread_allreduce` pattern (for example in split-k variants).
   - Compile path: lowered by `LowerThreadAllreduce`.
   - Typical failures: thread binding/reduce scope mismatch.

## Pass Config Triage (for simple kernels)

Use these toggles first:

- `TL_DISABLE_WARP_SPECIALIZED=True`
- `TL_DISABLE_TMA_LOWER=True`
- `TL_DISABLE_LOOP_UNSWITCHING=True` (if needed)
- `TL_DISABLE_DATA_RACE_CHECK=True` (diagnostic isolation only)

If the issue disappears after one toggle, the root cause is usually around that pass path.

## Symptom to likely location

- Dynamic shape binding mismatch: `tilelang/language/eager/builder.py`
- Unexpected `T.copy`/broadcast behavior: `tilelang/language/copy_op.py` plus legalization passes
- Pipeline/barrier/sync issues: `PipelinePlanning`, `InjectSoftwarePipeline`, `ThreadSync` in `tilelang/engine/phase.py`
- Compile flag not applied: compile callbacks in `tilelang/engine/lower.py` and `PassConfigKey.TL_DEVICE_COMPILE_FLAGS`
- Stale behavior: cache key/path in `tilelang/cache/kernel_cache.py`
- `T.reduce_*` behavior mismatch: `tilelang/language/reduce_op.py` and `src/transform/lower_tile_op.cc`
- `alloc_reducer` lifecycle/layout mismatch: `tilelang/language/allocate.py` and `src/transform/layout_reducer.cc`

## Change Checklist for PR

- [ ] `examples/deepseek_mhc/example_mhc_post.py` still passes
- [ ] `examples/norm/rms_norm.py` reduction behavior remains correct (for `T.reduce_*` path)
- [ ] `examples/gemv/example_gemv.py` reducer flow remains correct (for `alloc_reducer` path)
- [ ] Add or update relevant tests under `testing/python/...`
- [ ] Compare key artifacts before/after (at least TIR or kernel source)
- [ ] Confirm no accidental dependency on `T.gemm` path

## Guardrails

- This skill covers simple-kernel compiler paths only, not tensorcore/gemm specialization.
- For architecture-specific paths (for example Hopper-only behavior), confirm scope before deep changes.
- Fix priority: correctness > diagnosability > performance.
