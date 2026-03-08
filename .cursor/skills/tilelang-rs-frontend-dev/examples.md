# Examples

## When Editing `core`

Use this path when adding or changing:

- loop DSL APIs
- predicate helpers
- `alloc_var`
- runtime autoload
- `BuilderContext`

Checklist:

```text
Rust frontend core change
- [ ] Update core API
- [ ] Update core smoke/runtime tests
- [ ] Run core tests
- [ ] Check whether macro lowering or public re-exports need updates
```

## When Editing `#[tl_ir]`

Use this path when changing:

- syntax restrictions
- AST rewrite rules
- function wrapping
- `for` / `if` lowering

Checklist:

```text
Rust frontend macro change
- [ ] Update precheck or lowering code
- [ ] Add/adjust trybuild tests
- [ ] Add/adjust runtime IR test if semantics changed
- [ ] Run macro tests
```

## Good Change Pattern

- Add the reusable semantic primitive in `tilelang-rs-core`
- Make the macro lower into that primitive
- Add one compile-time test and one runtime IR test
- Re-run `./scripts/test_frontend.sh`

## Avoid

- Duplicating runtime loading logic in tests or examples
- Adding new uppercase DSL modules instead of `language as T`
- Accepting `T::parallel(m, n)` or tuple input for `parallel`
- Expanding macro behavior without a matching runtime IR assertion
