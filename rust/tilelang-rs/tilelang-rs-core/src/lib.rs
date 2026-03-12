use std::string::String as StdString;

pub use tilelang_rs_ffi as ffi;

use tvm_ffi::{AnyValue, Array, Map, String as FfiString};

pub type Result<T> = tvm_ffi::Result<T>;

pub const PHASE: &str = "phase4";

pub mod runtime;

pub trait DLDataTypeExt {
    /// Return a vector variant with `lanes` lanes, e.g. `T::INT8.with_lanes(4)` → `int8x4`.
    fn with_lanes(self, lanes: u16) -> tvm_ffi::DLDataType;
}

impl DLDataTypeExt for tvm_ffi::DLDataType {
    fn with_lanes(self, lanes: u16) -> tvm_ffi::DLDataType {
        tvm_ffi::DLDataType { lanes, ..self }
    }
}

pub trait IntoPrimExpr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr;
}

impl IntoPrimExpr for ffi::ir::PrimExpr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self
    }
}

impl IntoPrimExpr for &ffi::ir::PrimExpr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.clone()
    }
}

impl IntoPrimExpr for ffi::tir::Var {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.into()
    }
}

impl IntoPrimExpr for &ffi::tir::Var {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.clone().into()
    }
}

impl IntoPrimExpr for ffi::ir::IntImm {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.into()
    }
}

macro_rules! impl_into_prim_expr_for_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoPrimExpr for $ty {
                fn into_prim_expr(self) -> ffi::ir::PrimExpr {
                    int64_imm(self as i64)
                }
            }
        )*
    };
}

impl_into_prim_expr_for_int!(i32, i64, isize, u32, u64, usize);

/// Newtype wrapping `PrimExpr` to support arithmetic operator overloads.
///
/// Converts via `From<ffi::tir::Var>`, `From<ffi::ir::PrimExpr>`, and `From<i64>` etc.
#[derive(Clone)]
pub struct Expr(pub ffi::ir::PrimExpr);

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Expr").field(&"<PrimExpr>").finish()
    }
}

impl Expr {
    pub fn into_inner(self) -> ffi::ir::PrimExpr {
        self.0
    }
}

impl From<ffi::ir::PrimExpr> for Expr {
    fn from(expr: ffi::ir::PrimExpr) -> Self {
        Self(expr)
    }
}

impl From<ffi::tir::Var> for Expr {
    fn from(var: ffi::tir::Var) -> Self {
        Self(var.into())
    }
}

impl From<&ffi::tir::Var> for Expr {
    fn from(var: &ffi::tir::Var) -> Self {
        Self(var.clone().into())
    }
}

impl IntoPrimExpr for Expr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.0
    }
}

impl IntoPrimExpr for &Expr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.0.clone()
    }
}

macro_rules! impl_arith_for_expr {
    ($trait:ident, $method:ident, $ffi_fn:path) => {
        impl<R: IntoPrimExpr> std::ops::$trait<R> for Expr {
            type Output = Expr;
            fn $method(self, rhs: R) -> Expr {
                binary_arith_expr(self.0, rhs.into_prim_expr(), $ffi_fn)
            }
        }
    };
}

impl_arith_for_expr!(Add, add, ffi::tir::Add);
impl_arith_for_expr!(Sub, sub, ffi::tir::Sub);
impl_arith_for_expr!(Mul, mul, ffi::tir::Mul);
impl_arith_for_expr!(Div, div, ffi::tir::Div);

/// Converts one or more DSL values into a TVM `Array<PrimExpr>`.
///
/// - Scalar types (`T: IntoPrimExpr`) become a 1-element array.
/// - `[T; N]` arrays become an N-element array.
/// - Tuples `(A, B)`, `(A, B, C)`, `(A, B, C, D)` become 2/3/4-element arrays.
pub trait IntoPrimExprs {
    fn into_prim_exprs(self) -> Array<ffi::ir::PrimExpr>;
}

impl<T: IntoPrimExpr> IntoPrimExprs for T {
    fn into_prim_exprs(self) -> Array<ffi::ir::PrimExpr> {
        Array::new(vec![self.into_prim_expr()])
    }
}

impl<T: IntoPrimExpr, const N: usize> IntoPrimExprs for [T; N] {
    fn into_prim_exprs(self) -> Array<ffi::ir::PrimExpr> {
        Array::new(self.into_iter().map(|e| e.into_prim_expr()).collect())
    }
}

impl<A: IntoPrimExpr, B: IntoPrimExpr> IntoPrimExprs for (A, B) {
    fn into_prim_exprs(self) -> Array<ffi::ir::PrimExpr> {
        Array::new(vec![self.0.into_prim_expr(), self.1.into_prim_expr()])
    }
}

impl<A: IntoPrimExpr, B: IntoPrimExpr, C: IntoPrimExpr> IntoPrimExprs for (A, B, C) {
    fn into_prim_exprs(self) -> Array<ffi::ir::PrimExpr> {
        Array::new(vec![
            self.0.into_prim_expr(),
            self.1.into_prim_expr(),
            self.2.into_prim_expr(),
        ])
    }
}

impl<A: IntoPrimExpr, B: IntoPrimExpr, C: IntoPrimExpr, D: IntoPrimExpr> IntoPrimExprs
    for (A, B, C, D)
{
    fn into_prim_exprs(self) -> Array<ffi::ir::PrimExpr> {
        Array::new(vec![
            self.0.into_prim_expr(),
            self.1.into_prim_expr(),
            self.2.into_prim_expr(),
            self.3.into_prim_expr(),
        ])
    }
}

/// Zero-sized placeholder for a buffer parameter that has not yet been declared.
///
/// Call `.declare(shape, dtype)` on a `Tensor` to get a [`PendingBuffer`] that will be
/// automatically named by the `#[tl_ir]` macro's `named_let` transform.
/// Call `.declare_named(shape, dtype, name)` to register with an explicit name immediately.
///
/// ```ignore
/// let a = Tensor::new();
/// let a = a.declare(&n, T::FLOAT32);   // #[tl_ir] injects name "a" → Arg("a", buffer)
/// ```
pub struct Tensor;

impl Tensor {
    pub fn new() -> Self {
        Tensor
    }

    /// Declare this tensor as a buffer without registering its name yet.
    ///
    /// Returns a [`PendingBuffer`] whose `Arg` registration is deferred.  When used
    /// inside a `#[tl_ir]` function, the `named_let` macro transform fires and calls
    /// [`NameableKind::apply`] to register the correct name automatically.
    ///
    /// ```ignore
    /// let a = Tensor::new().declare(&n, T::FLOAT32);  // macro injects Arg("a", buf)
    /// ```
    ///
    /// Must be called inside a `with_tir_prim_func` context.
    pub fn declare(self, shape: impl IntoPrimExprs, dtype: tvm_ffi::DLDataType) -> PendingBuffer {
        let shape_exprs = shape.into_prim_exprs();
        let buffer = ffi::script::ir_builder::tir::Buffer(
            shape_exprs,
            dtype,
            FfiString::from("_"),
            None,
            None,
            None,
            FfiString::from("global"),
            0i64,
            0i64,
            FfiString::from(""),
            None,
        )
        .expect("Buffer declaration should not fail");
        PendingBuffer { inner: buffer }
    }

    /// Declare with an explicit name (Phase-2 compatibility).
    ///
    /// Registers `Arg(name, buffer)` immediately. Prefer the no-name variant
    /// [`declare`][Tensor::declare] with `#[tl_ir]` for automatic naming.
    ///
    /// Must be called inside a `with_tir_prim_func` context.
    pub fn declare_named(
        self,
        shape: impl IntoPrimExprs,
        dtype: tvm_ffi::DLDataType,
        name: &str,
    ) -> Buffer {
        Buffer::from_declaration(shape, dtype, name)
    }
}

impl Default for Tensor {
    fn default() -> Self {
        Tensor
    }
}

/// Types that can receive an IR name from the `#[tl_ir]` `named_let` transform.
///
/// Implement this trait (plus an inherent `__tl_named_tag(&self) -> NameableKind` method)
/// to make a type participate in automatic name injection.  The associated type `Named`
/// is the fully-named output — often the same type or a richer variant.
///
/// # Extending named_let to new types
///
/// 1. Define an inherent method `pub fn __tl_named_tag(&self) -> NameableKind { NameableKind }`.
/// 2. Implement `Nameable` and set `type Named` to whatever the post-naming type should be.
/// 3. Put the naming logic in `apply_name`.
///
/// No changes to [`NameableKind`] or the macro are required.
pub trait Nameable {
    /// The type produced after the name has been applied.
    type Named;
    /// Consume `self`, record `name` in the IR, and return the named variant.
    fn apply_name(self, name: &str) -> Self::Named;
}

/// A buffer that has been allocated but not yet registered as a function argument.
///
/// Obtained via [`Tensor::declare`] (no-name variant).  The `#[tl_ir]` macro's
/// `named_let` transform will call [`NameableKind::apply`] to register the correct
/// name and return a [`Buffer`].
pub struct PendingBuffer {
    inner: ffi::tir::Buffer,
}

impl PendingBuffer {
    /// Autoref tag method for named_let specialization.
    ///
    /// Returns [`NameableKind`], causing the macro-generated `apply` call to invoke
    /// [`Nameable::apply_name`] with the binding variable's name.
    #[doc(hidden)]
    pub fn __tl_named_tag(&self) -> NameableKind {
        NameableKind
    }
}

impl Nameable for PendingBuffer {
    type Named = Buffer;

    fn apply_name(self, name: &str) -> Buffer {
        ffi::script::ir_builder::tir::Arg(FfiString::from(name), self.inner.clone().into())
            .expect("Arg registration should not fail");
        Buffer { inner: self.inner }
    }
}

/// Autoref specialization tag for types that implement [`Nameable`].
///
/// Returned by the **inherent** `__tl_named_tag` method on nameable types.
/// [`NameableKind::apply`] delegates to [`Nameable::apply_name`] — no changes here
/// are needed when adding new nameable types.
pub struct NameableKind;

/// Autoref specialization tag (passthrough) for types that do not implement [`Nameable`].
///
/// Returned by the blanket [`PassthroughTag`] trait impl for all non-nameable types.
pub struct PassthroughKind;

impl NameableKind {
    /// Delegate to [`Nameable::apply_name`] — open for extension without modification.
    pub fn apply<T: Nameable>(self, val: T, name: &str) -> T::Named {
        val.apply_name(name)
    }
}

impl PassthroughKind {
    /// Passthrough: return `val` unchanged.
    pub fn apply<T>(self, val: T, _name: &str) -> T {
        val
    }
}

/// Fallback trait for named_let autoref specialization.
///
/// Blanket-implemented for all types.  Types that want named_let injection define an
/// **inherent** `__tl_named_tag` method instead (inherent methods take priority over
/// trait methods in Rust method resolution, providing the specialization effect).
pub trait PassthroughTag {
    #[doc(hidden)]
    fn __tl_named_tag(&self) -> PassthroughKind {
        PassthroughKind
    }
}

impl<T> PassthroughTag for T {}

/// A declared buffer parameter with load/store operations.
///
/// Obtained by calling [`Tensor::declare`].
#[derive(Clone)]
pub struct Buffer {
    inner: ffi::tir::Buffer,
}

impl Buffer {
    pub fn new(inner: ffi::tir::Buffer) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &ffi::tir::Buffer {
        &self.inner
    }

    /// Internal constructor — shared by `Tensor::declare`.
    fn from_declaration(
        shape: impl IntoPrimExprs,
        dtype: tvm_ffi::DLDataType,
        name: &str,
    ) -> Buffer {
        let shape_exprs = shape.into_prim_exprs();
        let buffer = ffi::script::ir_builder::tir::Buffer(
            shape_exprs,
            dtype,
            FfiString::from(name),
            None,
            None,
            None,
            FfiString::from("global"),
            0i64,
            0i64,
            FfiString::from(""),
            None,
        )
        .expect("Buffer declaration should not fail");

        ffi::script::ir_builder::tir::Arg(FfiString::from(name), buffer.clone().into())
            .expect("Arg registration should not fail");

        Buffer { inner: buffer }
    }

    /// Load a scalar or indexed value from this buffer.
    pub fn load_at(&self, indices: impl IntoPrimExprs) -> Expr {
        let span = empty_span();
        let indices = indices.into_prim_exprs();
        let load = ffi::tir::BufferLoad(self.inner.clone(), indices, None, span)
            .expect("BufferLoad should not fail");
        Expr(load.into())
    }

    /// Store a value into this buffer at the given indices.
    pub fn store_at(&self, indices: impl IntoPrimExprs, value: impl IntoPrimExpr) {
        let indices = indices.into_prim_exprs();
        ffi::script::ir_builder::tir::BufferStore(
            self.inner.clone(),
            value.into_prim_expr(),
            indices,
            None,
        )
        .expect("BufferStore should not fail");
    }
}

pub struct FrameGuard {
    frame: Option<ffi::script::ir_builder::IRBuilderFrame>,
}

impl FrameGuard {
    pub fn enter(frame: ffi::script::ir_builder::IRBuilderFrame) -> Self {
        ffi::script::ir_builder::IRBuilderFrameEnter(frame.clone())
            .expect("IRBuilderFrameEnter should not fail");
        Self { frame: Some(frame) }
    }

    pub fn exit(mut self) {
        self.exit_inner();
    }

    fn exit_inner(&mut self) {
        if let Some(frame) = self.frame.take() {
            ffi::script::ir_builder::IRBuilderFrameExit(frame)
                .expect("IRBuilderFrameExit should not fail");
        }
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        self.exit_inner();
    }
}

pub trait ForDsl {
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    );
}

pub struct SerialDsl<S, E> {
    start: S,
    stop: E,
    annotations: Option<Map<FfiString, AnyValue>>,
    step: Option<ffi::ir::PrimExpr>,
}

impl<S, E> SerialDsl<S, E> {
    pub fn with_annotations(mut self, annotations: Map<FfiString, AnyValue>) -> Self {
        self.annotations = Some(annotations);
        self
    }
}

impl<S, E> ForDsl for SerialDsl<S, E>
where
    S: IntoPrimExpr,
    E: IntoPrimExpr,
{
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let frame = ffi::script::ir_builder::tir::Serial(
            self.start.into_prim_expr(),
            self.stop.into_prim_expr(),
            self.annotations,
            self.step,
        )
        .expect("Serial frame construction should not fail");
        let vars = frame.get_vars();
        (frame.into(), vars)
    }
}

pub struct VectorizedDsl<S, E> {
    start: S,
    stop: E,
    annotations: Option<Map<FfiString, AnyValue>>,
    step: Option<ffi::ir::PrimExpr>,
}

impl<S, E> VectorizedDsl<S, E> {
    pub fn with_annotations(mut self, annotations: Map<FfiString, AnyValue>) -> Self {
        self.annotations = Some(annotations);
        self
    }
}

impl<S, E> ForDsl for VectorizedDsl<S, E>
where
    S: IntoPrimExpr,
    E: IntoPrimExpr,
{
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let frame = ffi::script::ir_builder::tir::Vectorized(
            self.start.into_prim_expr(),
            self.stop.into_prim_expr(),
            self.annotations,
            self.step,
        )
        .expect("Vectorized frame construction should not fail");
        let vars = frame.get_vars();
        (frame.into(), vars)
    }
}

pub struct UnrollDsl<S, E> {
    start: S,
    stop: E,
    annotations: Option<Map<FfiString, AnyValue>>,
    step: Option<ffi::ir::PrimExpr>,
}

impl<S, E> UnrollDsl<S, E> {
    pub fn with_annotations(mut self, annotations: Map<FfiString, AnyValue>) -> Self {
        self.annotations = Some(annotations);
        self
    }
}

impl<S, E> ForDsl for UnrollDsl<S, E>
where
    S: IntoPrimExpr,
    E: IntoPrimExpr,
{
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let frame = ffi::script::ir_builder::tir::Unroll(
            self.start.into_prim_expr(),
            self.stop.into_prim_expr(),
            self.annotations,
            self.step,
        )
        .expect("Unroll frame construction should not fail");
        let vars = frame.get_vars();
        (frame.into(), vars)
    }
}

pub struct ParallelDsl {
    extents: Array<ffi::ir::PrimExpr>,
    annotations: Map<FfiString, AnyValue>,
}

impl ParallelDsl {
    pub fn with_annotations(mut self, annotations: Map<FfiString, AnyValue>) -> Self {
        self.annotations = annotations;
        self
    }
}

impl ForDsl for ParallelDsl {
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let frame = ffi::tl::Parallel(self.extents, self.annotations)
            .expect("Parallel frame construction should not fail");
        let vars = frame.get_vars();
        (frame.into(), vars)
    }
}

/// DSL builder for `tl.KernelLaunch`. Construct via `T::kernel(grid)` and call
/// `.threads(n)` to set the thread count.
pub struct KernelDsl {
    grid: Array<ffi::ir::PrimExpr>,
    thread_extents: Array<ffi::ir::PrimExpr>,
}

impl KernelDsl {
    /// Set the 1-D thread count (blockDim.x). Defaults to 128.
    pub fn threads<E: IntoPrimExpr>(mut self, n: E) -> Self {
        self.thread_extents = Array::new(vec![n.into_prim_expr(), int64_imm(1), int64_imm(1)]);
        self
    }
}

impl ForDsl for KernelDsl {
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let frame =
            ffi::tl::KernelLaunch(self.grid, Some(self.thread_extents), empty_annotations())
                .expect("KernelLaunch should not fail");

        // frames[0..(len-4)] are the block-dimension LaunchThreadFrames;
        // the last 4 are threadIdx.x/y/z plus the attr/block frame.
        let all_frames = frame.get_frames();
        let n_frames = all_frames.len();
        let n_block = n_frames.saturating_sub(4);

        let mut block_vars = Vec::new();
        for i in 0..n_block {
            let tir_frame = all_frames.get(i).expect("frame index must be valid");
            let launch_frame = ffi::script::ir_builder::tir::LaunchThreadFrame::try_from(tir_frame)
                .unwrap_or_else(|_| panic!("block frame should be a LaunchThreadFrame"));
            block_vars.push(launch_frame.get_iter_var().get_var());
        }

        (frame.into(), Array::new(block_vars))
    }
}

pub struct PipelinedDsl<S, E> {
    start: S,
    stop: E,
    num_stages: i64,
    order: Array<ffi::ir::PrimExpr>,
    stage: Array<ffi::ir::PrimExpr>,
    sync: Array<Array<ffi::ir::PrimExpr>>,
    group: Array<Array<ffi::ir::PrimExpr>>,
}

impl<S, E> PipelinedDsl<S, E> {
    pub fn num_stages(mut self, num_stages: i64) -> Self {
        self.num_stages = num_stages;
        self
    }
}

impl<S, E> ForDsl for PipelinedDsl<S, E>
where
    S: IntoPrimExpr,
    E: IntoPrimExpr,
{
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let frame = ffi::tl::Pipelined(
            self.start.into_prim_expr(),
            self.stop.into_prim_expr(),
            self.num_stages,
            self.order,
            self.stage,
            self.sync,
            self.group,
        )
        .expect("Pipelined frame construction should not fail");
        let vars = frame.get_vars();
        (frame.into(), vars)
    }
}

#[derive(Clone)]
pub enum PredExpr {
    Const(bool),
    Expr(ffi::ir::PrimExpr),
}

pub trait IntoPredExpr {
    fn into_pred_expr(self) -> PredExpr;
}

impl IntoPredExpr for PredExpr {
    fn into_pred_expr(self) -> PredExpr {
        self
    }
}

impl IntoPredExpr for bool {
    fn into_pred_expr(self) -> PredExpr {
        PredExpr::Const(self)
    }
}

impl IntoPredExpr for ffi::ir::PrimExpr {
    fn into_pred_expr(self) -> PredExpr {
        PredExpr::Expr(self)
    }
}

impl PredExpr {
    pub fn to_prim_expr(self) -> ffi::ir::PrimExpr {
        match self {
            Self::Const(value) => bool_imm(value),
            Self::Expr(expr) => expr,
        }
    }

    pub fn as_const(&self) -> Option<bool> {
        match self {
            Self::Const(value) => Some(*value),
            Self::Expr(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct LocalVar {
    buffer: ffi::tir::Buffer,
}

impl LocalVar {
    pub fn new(buffer: ffi::tir::Buffer) -> Self {
        Self { buffer }
    }

    pub fn buffer(&self) -> &ffi::tir::Buffer {
        &self.buffer
    }

    pub fn load(&self) -> ffi::ir::PrimExpr {
        let span = empty_span();
        let load = ffi::tir::BufferLoad(self.buffer.clone(), scalar_index(), None, span)
            .expect("BufferLoad should not fail");
        load.into()
    }

    pub fn store<V>(&self, value: V)
    where
        V: IntoPrimExpr,
    {
        ffi::script::ir_builder::tir::BufferStore(
            self.buffer.clone(),
            value.into_prim_expr(),
            scalar_index(),
            None,
        )
        .expect("BufferStore should not fail");
    }
}

pub struct FromLoopVars;

impl FromLoopVars {
    pub fn bind1(vars: Array<ffi::tir::Var>) -> ffi::tir::Var {
        assert_loop_vars(vars.clone(), 1);
        vars.get(0).expect("index 0 must exist after count check")
    }

    pub fn bind2(vars: Array<ffi::tir::Var>) -> (ffi::tir::Var, ffi::tir::Var) {
        assert_loop_vars(vars.clone(), 2);
        (
            vars.get(0).expect("index 0 must exist"),
            vars.get(1).expect("index 1 must exist"),
        )
    }

    pub fn bind3(vars: Array<ffi::tir::Var>) -> (ffi::tir::Var, ffi::tir::Var, ffi::tir::Var) {
        assert_loop_vars(vars.clone(), 3);
        (
            vars.get(0).expect("index 0 must exist"),
            vars.get(1).expect("index 1 must exist"),
            vars.get(2).expect("index 2 must exist"),
        )
    }

    pub fn bind4(
        vars: Array<ffi::tir::Var>,
    ) -> (ffi::tir::Var, ffi::tir::Var, ffi::tir::Var, ffi::tir::Var) {
        assert_loop_vars(vars.clone(), 4);
        (
            vars.get(0).expect("index 0 must exist"),
            vars.get(1).expect("index 1 must exist"),
            vars.get(2).expect("index 2 must exist"),
            vars.get(3).expect("index 3 must exist"),
        )
    }
}

pub struct BuilderContext {
    name: StdString,
    builder: Option<ffi::script::ir_builder::IRBuilder>,
    module_frame: Option<FrameGuard>,
}

impl BuilderContext {
    pub fn new(name: impl Into<StdString>) -> Result<Self> {
        runtime::ensure_runtime_loaded()?;
        let name = name.into();
        let builder = ffi::script::ir_builder::IRBuilder()?;
        ffi::script::ir_builder::IRBuilderEnter(builder.clone())?;

        let module_frame = match ffi::script::ir_builder::ir::IRModule() {
            Ok(frame) => frame,
            Err(err) => {
                let _ = ffi::script::ir_builder::IRBuilderExit(builder);
                return Err(err);
            }
        };
        let module_guard = FrameGuard::enter(module_frame.into());

        Ok(Self {
            name,
            builder: Some(builder),
            module_frame: Some(module_guard),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn builder(&self) -> &ffi::script::ir_builder::IRBuilder {
        self.builder
            .as_ref()
            .expect("BuilderContext builder accessed after finish_ir_module")
    }

    pub fn enter_frame(&self, frame: ffi::script::ir_builder::IRBuilderFrame) -> FrameGuard {
        FrameGuard::enter(frame)
    }

    pub fn with_frame<T, F>(&self, frame: ffi::script::ir_builder::IRBuilderFrame, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let guard = self.enter_frame(frame);
        let value = f();
        guard.exit();
        value
    }

    pub fn for_each<D, T, F>(&self, dsl: D, f: F) -> T
    where
        D: ForDsl,
        F: FnOnce(&Self, Array<ffi::tir::Var>) -> T,
    {
        let (base, vars) = dsl.enter();
        self.with_frame(base, || f(self, vars))
    }

    pub fn with_tir_prim_func<T, F>(&self, name: &str, is_private: bool, f: F) -> T
    where
        F: FnOnce(&ffi::script::ir_builder::tir::PrimFuncFrame) -> T,
    {
        let prim_func_frame = ffi::script::ir_builder::tir::PrimFunc(is_private)
            .expect("PrimFunc frame construction should not fail");

        self.with_frame(prim_func_frame.clone().into(), || {
            ffi::script::ir_builder::tir::FuncName(FfiString::from(name))
                .expect("FuncName should not fail");
            f(&prim_func_frame)
        })
    }

    pub fn if_stmt<C, T, F, E>(&self, cond: C, then_branch: F, else_branch: Option<E>) -> T
    where
        C: IntoPredExpr,
        F: FnOnce() -> T,
        E: FnOnce() -> T,
    {
        let cond = cond.into_pred_expr().to_prim_expr();
        let if_frame =
            ffi::script::ir_builder::tir::If(cond).expect("If frame construction should not fail");

        self.with_frame(if_frame.into(), || {
            let then_frame = ffi::script::ir_builder::tir::Then()
                .expect("Then frame construction should not fail");
            let value = self.with_frame(then_frame.into(), then_branch);

            if let Some(else_branch) = else_branch {
                let else_frame = ffi::script::ir_builder::tir::Else()
                    .expect("Else frame construction should not fail");
                self.with_frame(else_frame.into(), else_branch);
            }

            value
        })
    }

    pub fn finish_ir_module(mut self) -> ffi::ir::IRModule {
        if let Some(module_frame) = self.module_frame.take() {
            module_frame.exit();
        }

        let builder = self
            .builder
            .take()
            .expect("BuilderContext finished more than once");
        let module_obj = ffi::script::ir_builder::IRBuilderGet(builder.clone())
            .expect("IRBuilderGet should not fail");
        ffi::script::ir_builder::IRBuilderExit(builder).expect("IRBuilderExit should not fail");
        module_obj
            .try_into()
            .unwrap_or_else(|_| panic!("IRBuilder should produce an ir.IRModule"))
    }
}

impl Drop for BuilderContext {
    fn drop(&mut self) {
        self.module_frame.take();
        if let Some(builder) = self.builder.take() {
            let _ = ffi::script::ir_builder::IRBuilderExit(builder);
        }
    }
}

pub fn debug_print<T>(object: T) -> StdString
where
    T: Into<tvm_ffi::object::ObjectRef>,
{
    let printed = ffi::ir::DebugPrint(object.into()).expect("DebugPrint should not fail");
    printed.as_str().to_owned()
}

pub mod language {
    use super::*;

    pub fn serial<S, E>(start: S, stop: E) -> SerialDsl<S, E>
    where
        S: IntoPrimExpr,
        E: IntoPrimExpr,
    {
        SerialDsl {
            start,
            stop,
            annotations: None,
            step: None,
        }
    }

    pub fn vectorized<S, E>(start: S, stop: E) -> VectorizedDsl<S, E>
    where
        S: IntoPrimExpr,
        E: IntoPrimExpr,
    {
        VectorizedDsl {
            start,
            stop,
            annotations: None,
            step: None,
        }
    }

    pub fn unroll<S, E>(start: S, stop: E) -> UnrollDsl<S, E>
    where
        S: IntoPrimExpr,
        E: IntoPrimExpr,
    {
        UnrollDsl {
            start,
            stop,
            annotations: None,
            step: None,
        }
    }

    pub fn parallel(extents: impl IntoPrimExprs) -> ParallelDsl {
        ParallelDsl {
            extents: extents.into_prim_exprs(),
            annotations: empty_annotations(),
        }
    }

    /// Construct a `KernelDsl` that maps to `tl.KernelLaunch`.
    ///
    /// `grid` is the block-grid extent (scalar for 1-D, array/tuple for N-D).
    /// Call `.threads(n)` to set the thread count (default 128).
    ///
    /// ```ignore
    /// for bx in T::kernel(T::ceildiv(&n, block_n)).threads(128) { ... }
    /// ```
    pub fn kernel(grid: impl IntoPrimExprs) -> KernelDsl {
        KernelDsl {
            grid: grid.into_prim_exprs(),
            thread_extents: Array::new(vec![int64_imm(128), int64_imm(1), int64_imm(1)]),
        }
    }

    /// Create a dynamic (symbolic) integer variable, e.g. `T::dynamic("n", T::INT64)`.
    pub fn dynamic(name: impl Into<StdString>, dtype: tvm_ffi::DLDataType) -> ffi::tir::Var {
        let span = empty_span();
        let name_ffi = FfiString::from(name.into().as_str());
        let dtype_any = tvm_ffi::AnyValue::from(tvm_ffi::Any::from(dtype));
        ffi::tir::Var(name_ffi, dtype_any, span).expect("Var construction should not fail")
    }

    /// Ceiling division: `ceildiv(a, b)` → `(a + b - 1) / b`.
    pub fn ceildiv<A: IntoPrimExpr, B: IntoPrimExpr>(a: A, b: B) -> Expr {
        let span = empty_span();
        let result = ffi::tir::_OpCeilDiv(a.into_prim_expr(), b.into_prim_expr(), span)
            .expect("CeilDiv should not fail");
        Expr(result)
    }

    pub fn pipelined<S, E>(start: S, stop: E) -> PipelinedDsl<S, E>
    where
        S: IntoPrimExpr,
        E: IntoPrimExpr,
    {
        PipelinedDsl {
            start,
            stop,
            num_stages: 0,
            order: Array::new(vec![]),
            stage: Array::new(vec![]),
            sync: Array::new(vec![]),
            group: Array::new(vec![]),
        }
    }

    const fn dl(code: tvm_ffi::DLDataTypeCode, bits: u8) -> tvm_ffi::DLDataType {
        tvm_ffi::DLDataType {
            code: code as u8,
            bits,
            lanes: 1,
        }
    }

    // ── bool ─────────────────────────────────────────────────────────────────
    pub const BOOL: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLBool, 1);

    // ── signed integers ───────────────────────────────────────────────────────
    pub const INT4: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLInt, 4);
    pub const INT8: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLInt, 8);
    pub const INT16: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLInt, 16);
    pub const INT32: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLInt, 32);
    pub const INT64: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLInt, 64);

    // ── unsigned integers ─────────────────────────────────────────────────────
    pub const UINT8: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLUInt, 8);
    pub const UINT16: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLUInt, 16);
    pub const UINT32: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLUInt, 32);
    pub const UINT64: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLUInt, 64);

    // ── standard floats ───────────────────────────────────────────────────────
    pub const FLOAT16: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat, 16);
    pub const FLOAT32: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat, 32);
    pub const FLOAT64: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat, 64);
    pub const BFLOAT16: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLBfloat, 16);

    // ── float8 variants ───────────────────────────────────────────────────────
    pub const FLOAT8_E3M4: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e3m4, 8);
    pub const FLOAT8_E4M3: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e4m3, 8);
    pub const FLOAT8_E4M3B11FNUZ: tvm_ffi::DLDataType =
        dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e4m3b11fnuz, 8);
    pub const FLOAT8_E4M3FN: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e4m3fn, 8);
    pub const FLOAT8_E4M3FNUZ: tvm_ffi::DLDataType =
        dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e4m3fnuz, 8);
    pub const FLOAT8_E5M2: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e5m2, 8);
    pub const FLOAT8_E5M2FNUZ: tvm_ffi::DLDataType =
        dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e5m2fnuz, 8);
    pub const FLOAT8_E8M0FNU: tvm_ffi::DLDataType =
        dl(tvm_ffi::DLDataTypeCode::kDLFloat8_e8m0fnu, 8);

    // ── float6 variants ───────────────────────────────────────────────────────
    pub const FLOAT6_E2M3FN: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat6_e2m3fn, 6);
    pub const FLOAT6_E3M2FN: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat6_e3m2fn, 6);

    // ── float4 variants ───────────────────────────────────────────────────────
    pub const FLOAT4_E2M1FN: tvm_ffi::DLDataType = dl(tvm_ffi::DLDataTypeCode::kDLFloat4_e2m1fn, 4);

    pub fn alloc_var<I>(dtype: tvm_ffi::DLDataType, init: I) -> LocalVar
    where
        I: IntoPrimExpr,
    {
        let buffer = ffi::script::ir_builder::tir::AllocBuffer(
            scalar_index(),
            dtype,
            None,
            Array::new(vec![]),
            int64_imm(0),
            FfiString::from("local.var"),
            -1,
            0,
            FfiString::from("default"),
            None,
        )
        .expect("AllocBuffer should not fail");
        let local_var = LocalVar::new(buffer);
        local_var.store(init);
        local_var
    }

    pub fn select<C, T, E>(cond: C, then_value: T, else_value: E) -> ffi::ir::PrimExpr
    where
        C: IntoPredExpr,
        T: IntoPrimExpr,
        E: IntoPrimExpr,
    {
        let cond = cond.into_pred_expr().to_prim_expr();
        let span = empty_span();
        let expr = ffi::tir::Select(
            cond,
            then_value.into_prim_expr(),
            else_value.into_prim_expr(),
            span,
        )
        .expect("Select should not fail");
        expr.into()
    }
}

pub mod pred {
    use super::*;

    pub fn to_ir_bool<C>(cond: C) -> PredExpr
    where
        C: IntoPredExpr,
    {
        cond.into_pred_expr()
    }

    pub fn eq<L, R>(lhs: L, rhs: R) -> PredExpr
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::EQ)
    }

    pub fn ne<L, R>(lhs: L, rhs: R) -> PredExpr
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::NE)
    }

    pub fn lt<L, R>(lhs: L, rhs: R) -> PredExpr
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::LT)
    }

    pub fn le<L, R>(lhs: L, rhs: R) -> PredExpr
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::LE)
    }

    pub fn gt<L, R>(lhs: L, rhs: R) -> PredExpr
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::GT)
    }

    pub fn ge<L, R>(lhs: L, rhs: R) -> PredExpr
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::GE)
    }

    pub fn not<C>(cond: C) -> PredExpr
    where
        C: IntoPredExpr,
    {
        match cond.into_pred_expr() {
            PredExpr::Const(value) => PredExpr::Const(!value),
            PredExpr::Expr(expr) => {
                let span = empty_span();
                let not = ffi::tir::Not(expr, span).expect("Not should not fail");
                PredExpr::Expr(not.into())
            }
        }
    }

    pub fn and<L, R, F>(lhs: L, rhs: F) -> PredExpr
    where
        L: IntoPredExpr,
        R: IntoPredExpr,
        F: FnOnce() -> R,
    {
        let lhs = lhs.into_pred_expr();
        match lhs {
            PredExpr::Const(false) => PredExpr::Const(false),
            PredExpr::Const(true) => rhs().into_pred_expr(),
            PredExpr::Expr(lhs_expr) => {
                let rhs = rhs().into_pred_expr();
                match rhs {
                    PredExpr::Const(false) => PredExpr::Const(false),
                    PredExpr::Const(true) => PredExpr::Expr(lhs_expr),
                    PredExpr::Expr(rhs_expr) => {
                        let span = empty_span();
                        let and =
                            ffi::tir::And(lhs_expr, rhs_expr, span).expect("And should not fail");
                        PredExpr::Expr(and.into())
                    }
                }
            }
        }
    }

    pub fn or<L, R, F>(lhs: L, rhs: F) -> PredExpr
    where
        L: IntoPredExpr,
        R: IntoPredExpr,
        F: FnOnce() -> R,
    {
        let lhs = lhs.into_pred_expr();
        match lhs {
            PredExpr::Const(true) => PredExpr::Const(true),
            PredExpr::Const(false) => rhs().into_pred_expr(),
            PredExpr::Expr(lhs_expr) => {
                let rhs = rhs().into_pred_expr();
                match rhs {
                    PredExpr::Const(true) => PredExpr::Const(true),
                    PredExpr::Const(false) => PredExpr::Expr(lhs_expr),
                    PredExpr::Expr(rhs_expr) => {
                        let span = empty_span();
                        let or =
                            ffi::tir::Or(lhs_expr, rhs_expr, span).expect("Or should not fail");
                        PredExpr::Expr(or.into())
                    }
                }
            }
        }
    }
}

fn empty_annotations() -> Map<FfiString, AnyValue> {
    Map::new(Vec::<(FfiString, AnyValue)>::new()).expect("empty Map construction should not fail")
}

fn scalar_index() -> Array<ffi::ir::PrimExpr> {
    Array::new(vec![int64_imm(0)])
}

fn empty_span() -> ffi::ir::Span {
    let source = ffi::ir::SourceName(FfiString::from("tilelang-rs-core"))
        .expect("SourceName construction should not fail");
    ffi::ir::Span(source, 0, 0, 0, 0).expect("Span construction should not fail")
}

fn int64_imm(value: i64) -> ffi::ir::PrimExpr {
    let dtype = tvm_ffi::DLDataType {
        code: tvm_ffi::DLDataTypeCode::kDLInt as u8,
        bits: 64,
        lanes: 1,
    };
    let span = empty_span();
    ffi::ir::IntImm(dtype, value, span)
        .expect("IntImm construction should not fail")
        .into()
}

fn assert_loop_vars(vars: Array<ffi::tir::Var>, expected: usize) {
    let actual = vars.len();
    assert!(
        actual == expected,
        "expected {} loop vars from ForFrame, got {}",
        expected,
        actual,
    );
}

fn bool_imm(value: bool) -> ffi::ir::PrimExpr {
    let dtype = tvm_ffi::DLDataType {
        code: tvm_ffi::DLDataTypeCode::kDLBool as u8,
        bits: 1,
        lanes: 1,
    };
    let span = empty_span();
    ffi::ir::IntImm(dtype, if value { 1 } else { 0 }, span)
        .expect("BoolImm construction should not fail")
        .into()
}

fn binary_arith_expr<F, O>(lhs: ffi::ir::PrimExpr, rhs: ffi::ir::PrimExpr, op: F) -> Expr
where
    F: FnOnce(ffi::ir::PrimExpr, ffi::ir::PrimExpr, ffi::ir::Span) -> Result<O>,
    O: Into<ffi::ir::PrimExpr>,
{
    let span = empty_span();
    let result = op(lhs, rhs, span).expect("binary arithmetic should not fail");
    Expr(result.into())
}

fn binary_pred<L, R, F, O>(lhs: L, rhs: R, op: F) -> PredExpr
where
    L: IntoPrimExpr,
    R: IntoPrimExpr,
    F: FnOnce(ffi::ir::PrimExpr, ffi::ir::PrimExpr, ffi::ir::Span) -> Result<O>,
    O: Into<ffi::ir::PrimExpr>,
{
    let span = empty_span();
    let expr = op(lhs.into_prim_expr(), rhs.into_prim_expr(), span)
        .expect("binary predicate construction should not fail");
    PredExpr::Expr(expr.into())
}
