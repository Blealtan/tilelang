---
name: tilelang-simple-kernel-usage
description: Designs, explains, and debugs simple TileLang eager-mode kernels without T.gemm and with regular control flow. Use when users ask to write, modify, optimize, or explain kernels like examples/deepseek_mhc/example_mhc_post.py using T.Kernel, T.copy, T.Pipelined, T.Parallel, and T.serial.
---

# TileLang Simple Kernel Usage

## Scope

- Only simple kernels: no `T.gemm`, regular loop-based control flow.
- Primary reference pattern: `examples/deepseek_mhc/example_mhc_post.py`.
- Prioritize readability, correctness, and easy verification before tuning.

## Ascend 950 Entry (lightweight)

- This skill does not expand Ascend-specific semantics in detail.
- If the request mentions `T.SimtVF`, `target="ascend"`, or `.asc` source generation, switch to the project skill:
  - `.cursor/skills/tilelang-ascend-950-simtvf/SKILL.md`

## General TileLang Compiler Notes

- Thread binding in TIR is carried by `AttrStmt` (for example `thread_extent` on an `IterVar`).
- If generated code looks different from source loops, inspect staged IR in order:
  - Raw TIR
  - After `LowerAndLegalize`
  - After `OptimizeForTarget`
- Treat loop annotations as compiler contracts: structure and metadata should stay explicit in frontend code.

## Quick Workflow

Copy this checklist and track progress:

```text
Task Progress:
- [ ] 1) Decompose formula into parallel axes, reduction axes, and output shape
- [ ] 2) Build eager kernel skeleton (dynamic dims + Tensor annotations + T.Kernel)
- [ ] 3) Define shared/fragment dataflow and loop structure
- [ ] 4) Write PyTorch reference and validate with assert_close
- [ ] 5) Apply only small-step tuning (threads/h_blk/pass_configs)
```

## Step 1: Model first, then map to loops

- Decompose math into three levels:
  - **Block level**: what one `T.Kernel` instance computes.
  - **Parallel level**: element updates in `T.Parallel(...)`.
  - **Serial level**: loops written with `T.serial(...)`.
- For dynamic batch/token dimensions, start with `n = T.dynamic("num_tokens")`.
- Keep static dimensions (for example `hc`, `hidden`) as Python parameters for tuning.

## Step 2: Use a stable eager skeleton

Default skeleton:

```python
@tilelang.jit(pass_configs={
    tilelang.PassConfigKey.TL_DISABLE_WARP_SPECIALIZED: True,
    tilelang.PassConfigKey.TL_DISABLE_TMA_LOWER: True,
})
def kernel(A, B, C, D, Out, hc: int, hidden: int, n_thr: int = 128, h_blk: int = 1024):
    n = T.dynamic("num_tokens")
    h = hidden
    h_blk = math.gcd(hidden, h_blk)
    A: T.Tensor((n, hc, hc), T.float32)
    ...
    with T.Kernel(n, threads=n_thr) as i_n:
        ...
```

Key points:
- Use `h_blk = math.gcd(hidden, h_blk)` to keep tile behavior stable.
- Prefer thread counts that are multiples of 32.

## Step 3: Keep explicit copy-compute-store dataflow

For this scope, use explicit two-level staging:

- Global to shared: `T.copy(global, shared)`
- Shared to fragment: `T.copy(shared, fragment)`
- Fragment compute: `T.Parallel` + `T.serial`
- Fragment to shared to global: `T.copy(fragment, shared)` then `T.copy(shared, global)`

Recommended scopes:
- Staging buffers: `T.alloc_shared(..., dtype)`
- Compute temporaries: `T.alloc_fragment(..., accum_dtype)`
- Accumulate in `float32`, then cast to output dtype when needed.

## Step 4: Loop structure convention

- Outer tile loop: `for i in T.Pipelined(T.ceildiv(total, tile), num_stages=2):`
- Middle parallel loops: `for p0, p1 in T.Parallel(...):`
- Inner serial loops: `for k in T.serial(...)`.

Avoid complex branching in this skill scope. If unavoidable, state that the request is out of scope and ask whether to switch skills.

## Reduction options in this scope

Use this decision order for simple kernels:

1. **Manual reduction in serial loops**
   - Pattern: update scalar/local accumulators inside `T.serial(...)`.
   - Best for explicit math control and easy debugging.
   - Typical in GEMV-like kernels where K is tiled and accumulated.

2. **`T.reduce_sum` / `T.reduce_min` / `T.reduce_max`**
   - Pattern: compute elementwise data first, then call `T.reduce_*` on a selected dimension.
   - Good for concise expression of dimension reduction.
   - Example style: `examples/norm/rms_norm.py`.

3. **`T.alloc_reducer` + `T.finalize_reducer`**
   - Pattern: allocate reducer buffer, update it in `T.Parallel`, finalize once, then write out.
   - Good for tile-level or block-level reduction organization.
   - Example style: `examples/gemv/example_gemv.py` (`gemv_alloc_reducer`).

Reducer usage rules:
- Always initialize reducer state explicitly (for example `T.clear(...)`).
- Always pair reducer updates with `T.finalize_reducer(...)` before consuming results.
- Keep reducer accumulation dtype stable (usually `float32` for numeric stability).

## Step 5: Correctness and regression

Always include:
- A semantically equivalent PyTorch reference implementation.
- Fixed random seed and at least 2-3 shape cases.
- `torch.testing.assert_close(...)` verification.

Even for performance-only changes, keep correctness checks.

## Response Format

Organize outputs in this order:

1. Problem decomposition (axes and layout)
2. Kernel plan (Tensor annotations and loop structure)
3. Reduction method choice (if needed)
4. Dataflow plan (global/shared/fragment)
5. Correctness plan (reference and tests)
6. Optional tuning knobs (simple only)

## Guardrails

- Do not introduce `T.gemm`, `gemm_v2`, or `wgmma`.
- Do not mix complex control flow and advanced scheduling tricks into this scope.
- If tensorcore/gemm paths are required, clearly mark out-of-scope and suggest switching skills.
- When using `alloc_reducer`, do not omit `T.finalize_reducer(...)`.
