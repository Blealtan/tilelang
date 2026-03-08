use std::string::String as StdString;

use std::sync::LazyLock;
pub use tilelang_rs_ffi as ffi;

use tvm_ffi::{AnyValue, Array, DLDataTypeExt, Map, String as FfiString};

pub type Result<T> = tvm_ffi::Result<T>;

pub const PHASE: &str = "phase4";

pub mod runtime;

pub trait IntoPrimExpr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr;
}

pub trait IntoDType {
    fn into_dtype(self) -> Result<tvm_ffi::DLDataType>;
}

impl IntoDType for tvm_ffi::DLDataType {
    fn into_dtype(self) -> Result<tvm_ffi::DLDataType> {
        Ok(self)
    }
}

impl IntoDType for &str {
    fn into_dtype(self) -> Result<tvm_ffi::DLDataType> {
        tvm_ffi::DLDataType::try_from_str(self)
    }
}

impl IntoDType for String {
    fn into_dtype(self) -> Result<tvm_ffi::DLDataType> {
        tvm_ffi::DLDataType::try_from_str(&self)
    }
}

impl IntoPrimExpr for ffi::ir::PrimExpr {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self
    }
}

impl IntoPrimExpr for ffi::tir::Var {
    fn into_prim_expr(self) -> ffi::ir::PrimExpr {
        self.into()
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
                    int64_imm(self as i64).expect("int literal to PrimExpr should never fail")
                }
            }
        )*
    };
}

impl_into_prim_expr_for_int!(i32, i64, isize, u32, u64, usize);

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
        let vars = for_frame_vars(&frame);
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
        let vars = for_frame_vars(&frame);
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
        let vars = for_frame_vars(&frame);
        (frame.into(), vars)
    }
}

pub struct ParallelDsl<E, const N: usize> {
    extents: [E; N],
    annotations: Map<FfiString, AnyValue>,
}

impl<E, const N: usize> ParallelDsl<E, N> {
    pub fn with_annotations(mut self, annotations: Map<FfiString, AnyValue>) -> Self {
        self.annotations = annotations;
        self
    }
}

impl<E, const N: usize> ForDsl for ParallelDsl<E, N>
where
    E: IntoPrimExpr,
{
    fn enter(
        self,
    ) -> (
        ffi::script::ir_builder::IRBuilderFrame,
        Array<ffi::tir::Var>,
    ) {
        let extents: Vec<ffi::ir::PrimExpr> = self
            .extents
            .into_iter()
            .map(|e| e.into_prim_expr())
            .collect();
        let frame = ffi::tl::Parallel(Array::new(extents), self.annotations)
            .expect("Parallel frame construction should not fail");
        let vars = for_frame_vars(&frame);
        (frame.into(), vars)
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
        let vars = for_frame_vars(&frame);
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
            Self::Const(value) => bool_imm(value).expect("bool_imm should not fail"),
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
        let span = empty_span().expect("empty_span should not fail");
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
        downcast_ir_module(module_obj).expect("IRBuilder should produce an ir.IRModule")
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

    pub fn parallel<E, const N: usize>(extents: [E; N]) -> ParallelDsl<E, N>
    where
        E: IntoPrimExpr,
    {
        ParallelDsl {
            extents,
            annotations: empty_annotations().expect("failed to create empty loop annotations"),
        }
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

    pub fn int32() -> tvm_ffi::DLDataType {
        tvm_ffi::DLDataType::try_from_str("int32").expect("int32 should be a valid dtype")
    }

    pub fn float32() -> tvm_ffi::DLDataType {
        tvm_ffi::DLDataType::try_from_str("float32").expect("float32 should be a valid dtype")
    }

    pub fn alloc_var<D, I>(dtype: D, init: I) -> LocalVar
    where
        D: IntoDType,
        I: IntoPrimExpr,
    {
        let dtype = dtype
            .into_dtype()
            .expect("alloc_var: dtype conversion should not fail");
        let buffer = ffi::script::ir_builder::tir::AllocBuffer(
            scalar_index(),
            dtype,
            None,
            Array::new(vec![]),
            int64_imm(0).expect("int64_imm(0) should not fail"),
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
        let span = empty_span().expect("empty_span should not fail");
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
                let span = empty_span().expect("empty_span should not fail");
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
                        let span = empty_span().expect("empty_span should not fail");
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
                        let span = empty_span().expect("empty_span should not fail");
                        let or =
                            ffi::tir::Or(lhs_expr, rhs_expr, span).expect("Or should not fail");
                        PredExpr::Expr(or.into())
                    }
                }
            }
        }
    }
}

static FORFRAME_VARS_GETTER: LazyLock<tvm_ffi::object_wrapper::FieldGetter<Array<ffi::tir::Var>>> =
    LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.ForFrame", "vars")
            .expect("ForFrame.vars field must be registered in TVM reflection")
    });

fn for_frame_vars(frame: &ffi::script::ir_builder::tir::ForFrame) -> Array<ffi::tir::Var> {
    let obj_ref: tvm_ffi::object::ObjectRef = frame.clone().into();
    FORFRAME_VARS_GETTER
        .get(&obj_ref)
        .expect("ForFrame.vars must be accessible")
}

fn empty_annotations() -> Result<Map<FfiString, AnyValue>> {
    Map::new(Vec::<(FfiString, AnyValue)>::new())
}

fn scalar_index() -> Array<ffi::ir::PrimExpr> {
    Array::new(vec![
        int64_imm(0).expect("scalar index construction should succeed")
    ])
}

fn empty_span() -> Result<ffi::ir::Span> {
    let source = ffi::ir::SourceName(FfiString::from("tilelang-rs-core"))?;
    ffi::ir::Span(source, 0, 0, 0, 0)
}

fn int64_imm(value: i64) -> Result<ffi::ir::PrimExpr> {
    let dtype = tvm_ffi::DLDataType::try_from_str("int64")?;
    let span = empty_span()?;
    let imm = ffi::ir::IntImm(dtype, value, span)?;
    Ok(imm.into())
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

fn bool_imm(value: bool) -> Result<ffi::ir::PrimExpr> {
    let dtype = tvm_ffi::DLDataType::try_from_str("bool")?;
    let span = empty_span()?;
    let imm = ffi::ir::IntImm(dtype, if value { 1 } else { 0 }, span)?;
    Ok(imm.into())
}

fn binary_pred<L, R, F, O>(lhs: L, rhs: R, op: F) -> PredExpr
where
    L: IntoPrimExpr,
    R: IntoPrimExpr,
    F: FnOnce(ffi::ir::PrimExpr, ffi::ir::PrimExpr, ffi::ir::Span) -> Result<O>,
    O: Into<ffi::ir::PrimExpr>,
{
    let span = empty_span().expect("empty_span should not fail");
    let expr = op(lhs.into_prim_expr(), rhs.into_prim_expr(), span)
        .expect("binary predicate construction should not fail");
    PredExpr::Expr(expr.into())
}

fn downcast_ir_module(object: tvm_ffi::object::ObjectRef) -> Result<ffi::ir::IRModule> {
    use tvm_ffi::error::TYPE_ERROR;
    object
        .try_into()
        .map_err(|_object: tvm_ffi::object::ObjectRef| {
            tvm_ffi::Error::new(TYPE_ERROR, "IRBuilder did not produce an ir.IRModule", "")
        })
}
