use std::string::String as StdString;

pub use tilelang_rs_ffi as ffi;
use tvm_ffi::{
    error::{TYPE_ERROR, VALUE_ERROR},
    object::ObjectRef,
    AnyValue, Array, DLDataTypeExt, Map, String as FfiString,
};

pub type Result<T> = tvm_ffi::Result<T>;

pub const PHASE: &str = "phase4";

pub trait IntoPrimExpr {
    fn into_prim_expr(self) -> Result<ffi::ir::PrimExpr>;
}

impl IntoPrimExpr for ffi::ir::PrimExpr {
    fn into_prim_expr(self) -> Result<ffi::ir::PrimExpr> {
        Ok(self)
    }
}

impl IntoPrimExpr for ffi::tir::Var {
    fn into_prim_expr(self) -> Result<ffi::ir::PrimExpr> {
        Ok(ffi::ir::PrimExpr::from_object(self.as_object_ref().clone()))
    }
}

impl IntoPrimExpr for ffi::ir::IntImm {
    fn into_prim_expr(self) -> Result<ffi::ir::PrimExpr> {
        Ok(ffi::ir::PrimExpr::from_object(self.as_object_ref().clone()))
    }
}

macro_rules! impl_into_prim_expr_for_int {
    ($($ty:ty),* $(,)?) => {
        $(
            impl IntoPrimExpr for $ty {
                fn into_prim_expr(self) -> Result<ffi::ir::PrimExpr> {
                    int64_imm(self as i64)
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
    pub fn enter(frame: ffi::script::ir_builder::IRBuilderFrame) -> Result<Self> {
        ffi::script::ir_builder::IRBuilderFrameEnter(frame.clone())?;
        Ok(Self { frame: Some(frame) })
    }

    pub fn exit(mut self) -> Result<()> {
        self.exit_inner()
    }

    fn exit_inner(&mut self) -> Result<()> {
        if let Some(frame) = self.frame.take() {
            ffi::script::ir_builder::IRBuilderFrameExit(frame)?;
        }
        Ok(())
    }
}

impl Drop for FrameGuard {
    fn drop(&mut self) {
        let _ = self.exit_inner();
    }
}

pub trait ForDsl {
    fn into_for_frame(self) -> Result<ffi::script::ir_builder::tir::ForFrame>;
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
    fn into_for_frame(self) -> Result<ffi::script::ir_builder::tir::ForFrame> {
        ffi::script::ir_builder::tir::Serial(
            self.start.into_prim_expr()?,
            self.stop.into_prim_expr()?,
            self.annotations,
            self.step,
        )
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
    fn into_for_frame(self) -> Result<ffi::script::ir_builder::tir::ForFrame> {
        ffi::script::ir_builder::tir::Vectorized(
            self.start.into_prim_expr()?,
            self.stop.into_prim_expr()?,
            self.annotations,
            self.step,
        )
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
    fn into_for_frame(self) -> Result<ffi::script::ir_builder::tir::ForFrame> {
        ffi::script::ir_builder::tir::Unroll(
            self.start.into_prim_expr()?,
            self.stop.into_prim_expr()?,
            self.annotations,
            self.step,
        )
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
    fn into_for_frame(self) -> Result<ffi::script::ir_builder::tir::ForFrame> {
        let mut extents = Vec::with_capacity(N);
        for extent in self.extents {
            extents.push(extent.into_prim_expr()?);
        }
        ffi::tl::Parallel(Array::new(extents), self.annotations)
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
    fn into_for_frame(self) -> Result<ffi::script::ir_builder::tir::ForFrame> {
        ffi::tl::Pipelined(
            self.start.into_prim_expr()?,
            self.stop.into_prim_expr()?,
            self.num_stages,
            self.order,
            self.stage,
            self.sync,
            self.group,
        )
    }
}

#[derive(Clone)]
pub enum PredExpr {
    Const(bool),
    Expr(ffi::ir::PrimExpr),
}

pub trait IntoPredExpr {
    fn into_pred_expr(self) -> Result<PredExpr>;
}

impl IntoPredExpr for PredExpr {
    fn into_pred_expr(self) -> Result<PredExpr> {
        Ok(self)
    }
}

impl IntoPredExpr for bool {
    fn into_pred_expr(self) -> Result<PredExpr> {
        Ok(PredExpr::Const(self))
    }
}

impl IntoPredExpr for ffi::ir::PrimExpr {
    fn into_pred_expr(self) -> Result<PredExpr> {
        Ok(PredExpr::Expr(self))
    }
}

impl PredExpr {
    pub fn to_prim_expr(self) -> Result<ffi::ir::PrimExpr> {
        match self {
            Self::Const(value) => bool_imm(value),
            Self::Expr(expr) => Ok(expr),
        }
    }

    pub fn as_const(&self) -> Option<bool> {
        match self {
            Self::Const(value) => Some(*value),
            Self::Expr(_) => None,
        }
    }
}

pub struct FromLoopVars;

impl FromLoopVars {
    pub fn bind1(vars: Array<ffi::tir::Var>) -> Result<ffi::tir::Var> {
        expect_loop_vars(vars.clone(), 1)?;
        vars.get(0)
    }

    pub fn bind2(vars: Array<ffi::tir::Var>) -> Result<(ffi::tir::Var, ffi::tir::Var)> {
        expect_loop_vars(vars.clone(), 2)?;
        Ok((vars.get(0)?, vars.get(1)?))
    }

    pub fn bind3(
        vars: Array<ffi::tir::Var>,
    ) -> Result<(ffi::tir::Var, ffi::tir::Var, ffi::tir::Var)> {
        expect_loop_vars(vars.clone(), 3)?;
        Ok((vars.get(0)?, vars.get(1)?, vars.get(2)?))
    }

    pub fn bind4(
        vars: Array<ffi::tir::Var>,
    ) -> Result<(ffi::tir::Var, ffi::tir::Var, ffi::tir::Var, ffi::tir::Var)> {
        expect_loop_vars(vars.clone(), 4)?;
        Ok((vars.get(0)?, vars.get(1)?, vars.get(2)?, vars.get(3)?))
    }
}

pub struct BuilderContext {
    name: StdString,
    builder: Option<ffi::script::ir_builder::IRBuilder>,
    module_frame: Option<FrameGuard>,
}

impl BuilderContext {
    pub fn new(name: impl Into<StdString>) -> Result<Self> {
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
        let module_base = ffi::script::ir_builder::IRBuilderFrame::from_object(
            module_frame.as_object_ref().clone(),
        );
        let module_guard = match FrameGuard::enter(module_base) {
            Ok(guard) => guard,
            Err(err) => {
                let _ = ffi::script::ir_builder::IRBuilderExit(builder);
                return Err(err);
            }
        };

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

    pub fn enter_frame(
        &self,
        frame: ffi::script::ir_builder::IRBuilderFrame,
    ) -> Result<FrameGuard> {
        FrameGuard::enter(frame)
    }

    pub fn with_frame<T, F>(
        &self,
        frame: ffi::script::ir_builder::IRBuilderFrame,
        f: F,
    ) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        let guard = self.enter_frame(frame)?;
        let body_result = f();
        let exit_result = guard.exit();

        match (body_result, exit_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(err), _) => Err(err),
            (Ok(_), Err(err)) => Err(err),
        }
    }

    pub fn for_each<D, T, F>(&self, dsl: D, f: F) -> Result<T>
    where
        D: ForDsl,
        F: FnOnce(&Self, Array<ffi::tir::Var>) -> Result<T>,
    {
        let frame = dsl.into_for_frame()?;
        let vars = frame.vars()?;
        let base =
            ffi::script::ir_builder::IRBuilderFrame::from_object(frame.as_object_ref().clone());
        self.with_frame(base, || f(self, vars))
    }

    pub fn with_tir_prim_func<T, F>(&self, name: &str, is_private: bool, f: F) -> Result<T>
    where
        F: FnOnce(&ffi::script::ir_builder::tir::PrimFuncFrame) -> Result<T>,
    {
        let prim_func_frame = ffi::script::ir_builder::tir::PrimFunc(is_private)?;
        let prim_func_base = ffi::script::ir_builder::IRBuilderFrame::from_object(
            prim_func_frame.as_object_ref().clone(),
        );

        self.with_frame(prim_func_base, || {
            ffi::script::ir_builder::tir::FuncName(FfiString::from(name))?;
            f(&prim_func_frame)
        })
    }

    pub fn if_stmt<C, T, F, E>(&self, cond: C, then_branch: F, else_branch: Option<E>) -> Result<T>
    where
        C: IntoPredExpr,
        F: FnOnce() -> Result<T>,
        E: FnOnce() -> Result<T>,
    {
        let cond = cond.into_pred_expr()?.to_prim_expr()?;
        let if_frame = ffi::script::ir_builder::tir::If(cond)?;
        let if_base =
            ffi::script::ir_builder::IRBuilderFrame::from_object(if_frame.as_object_ref().clone());

        self.with_frame(if_base, || {
            let then_frame = ffi::script::ir_builder::tir::Then()?;
            let then_base = ffi::script::ir_builder::IRBuilderFrame::from_object(
                then_frame.as_object_ref().clone(),
            );
            let then_result = self.with_frame(then_base, then_branch);

            match (then_result, else_branch) {
                (Ok(value), None) => Ok(value),
                (Err(err), _) => Err(err),
                (Ok(_), Some(else_branch)) => {
                    let else_frame = ffi::script::ir_builder::tir::Else()?;
                    let else_base = ffi::script::ir_builder::IRBuilderFrame::from_object(
                        else_frame.as_object_ref().clone(),
                    );
                    self.with_frame(else_base, else_branch)
                }
            }
        })
    }

    pub fn finish_ir_module(mut self) -> Result<ffi::ir::IRModule> {
        if let Some(module_frame) = self.module_frame.take() {
            module_frame.exit()?;
        }

        let builder = self
            .builder
            .take()
            .expect("BuilderContext finished more than once");
        let module_obj = ffi::script::ir_builder::IRBuilderGet(builder.clone())?;
        ffi::script::ir_builder::IRBuilderExit(builder)?;
        downcast_ir_module(module_obj)
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

pub fn debug_print<T>(object: T) -> Result<StdString>
where
    T: Into<ObjectRef>,
{
    let printed = ffi::ir::DebugPrint(object.into())?;
    Ok(printed.as_str().to_owned())
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

    pub fn select<C, T, E>(cond: C, then_value: T, else_value: E) -> Result<ffi::ir::PrimExpr>
    where
        C: IntoPredExpr,
        T: IntoPrimExpr,
        E: IntoPrimExpr,
    {
        let cond = cond.into_pred_expr()?.to_prim_expr()?;
        let span = empty_span()?;
        let expr = ffi::tir::Select(
            cond,
            then_value.into_prim_expr()?,
            else_value.into_prim_expr()?,
            span,
        )?;
        Ok(ffi::ir::PrimExpr::from_object(expr.as_object_ref().clone()))
    }
}

pub mod pred {
    use super::*;

    pub fn to_ir_bool<C>(cond: C) -> Result<PredExpr>
    where
        C: IntoPredExpr,
    {
        cond.into_pred_expr()
    }

    pub fn eq<L, R>(lhs: L, rhs: R) -> Result<PredExpr>
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::EQ)
    }

    pub fn ne<L, R>(lhs: L, rhs: R) -> Result<PredExpr>
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::NE)
    }

    pub fn lt<L, R>(lhs: L, rhs: R) -> Result<PredExpr>
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::LT)
    }

    pub fn le<L, R>(lhs: L, rhs: R) -> Result<PredExpr>
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::LE)
    }

    pub fn gt<L, R>(lhs: L, rhs: R) -> Result<PredExpr>
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::GT)
    }

    pub fn ge<L, R>(lhs: L, rhs: R) -> Result<PredExpr>
    where
        L: IntoPrimExpr,
        R: IntoPrimExpr,
    {
        binary_pred(lhs, rhs, ffi::tir::GE)
    }

    pub fn not<C>(cond: C) -> Result<PredExpr>
    where
        C: IntoPredExpr,
    {
        match cond.into_pred_expr()? {
            PredExpr::Const(value) => Ok(PredExpr::Const(!value)),
            PredExpr::Expr(expr) => {
                let span = empty_span()?;
                let not = ffi::tir::Not(expr, span)?;
                Ok(PredExpr::Expr(ffi::ir::PrimExpr::from_object(
                    not.as_object_ref().clone(),
                )))
            }
        }
    }

    pub fn and<L, R, F>(lhs: L, rhs: F) -> Result<PredExpr>
    where
        L: IntoPredExpr,
        R: IntoPredExpr,
        F: FnOnce() -> Result<R>,
    {
        let lhs = lhs.into_pred_expr()?;
        match lhs {
            PredExpr::Const(false) => Ok(PredExpr::Const(false)),
            PredExpr::Const(true) => rhs()?.into_pred_expr(),
            PredExpr::Expr(lhs_expr) => {
                let rhs = rhs()?.into_pred_expr()?;
                match rhs {
                    PredExpr::Const(false) => Ok(PredExpr::Const(false)),
                    PredExpr::Const(true) => Ok(PredExpr::Expr(lhs_expr)),
                    PredExpr::Expr(rhs_expr) => {
                        let span = empty_span()?;
                        let and = ffi::tir::And(lhs_expr, rhs_expr, span)?;
                        Ok(PredExpr::Expr(ffi::ir::PrimExpr::from_object(
                            and.as_object_ref().clone(),
                        )))
                    }
                }
            }
        }
    }

    pub fn or<L, R, F>(lhs: L, rhs: F) -> Result<PredExpr>
    where
        L: IntoPredExpr,
        R: IntoPredExpr,
        F: FnOnce() -> Result<R>,
    {
        let lhs = lhs.into_pred_expr()?;
        match lhs {
            PredExpr::Const(true) => Ok(PredExpr::Const(true)),
            PredExpr::Const(false) => rhs()?.into_pred_expr(),
            PredExpr::Expr(lhs_expr) => {
                let rhs = rhs()?.into_pred_expr()?;
                match rhs {
                    PredExpr::Const(true) => Ok(PredExpr::Const(true)),
                    PredExpr::Const(false) => Ok(PredExpr::Expr(lhs_expr)),
                    PredExpr::Expr(rhs_expr) => {
                        let span = empty_span()?;
                        let or = ffi::tir::Or(lhs_expr, rhs_expr, span)?;
                        Ok(PredExpr::Expr(ffi::ir::PrimExpr::from_object(
                            or.as_object_ref().clone(),
                        )))
                    }
                }
            }
        }
    }
}

fn empty_annotations() -> Result<Map<FfiString, AnyValue>> {
    Map::new(Vec::<(FfiString, AnyValue)>::new())
}

fn empty_span() -> Result<ffi::ir::Span> {
    let source = ffi::ir::SourceName(FfiString::from("tilelang-rs-core"))?;
    ffi::ir::Span(source, 0, 0, 0, 0)
}

fn int64_imm(value: i64) -> Result<ffi::ir::PrimExpr> {
    let dtype = tvm_ffi::DLDataType::try_from_str("int64")?;
    let span = empty_span()?;
    let imm = ffi::ir::IntImm(dtype, value, span)?;
    Ok(ffi::ir::PrimExpr::from_object(imm.as_object_ref().clone()))
}

fn expect_loop_vars(vars: Array<ffi::tir::Var>, expected: usize) -> Result<()> {
    let actual = vars.len();
    if actual == expected {
        return Ok(());
    }

    Err(tvm_ffi::Error::new(
        VALUE_ERROR,
        &format!(
            "expected {} loop vars, but got {} from ForFrame",
            expected, actual
        ),
        "",
    ))
}

fn bool_imm(value: bool) -> Result<ffi::ir::PrimExpr> {
    let dtype = tvm_ffi::DLDataType::try_from_str("bool")?;
    let span = empty_span()?;
    let imm = ffi::ir::IntImm(dtype, if value { 1 } else { 0 }, span)?;
    Ok(ffi::ir::PrimExpr::from_object(imm.as_object_ref().clone()))
}

fn binary_pred<L, R, F, O>(lhs: L, rhs: R, op: F) -> Result<PredExpr>
where
    L: IntoPrimExpr,
    R: IntoPrimExpr,
    F: FnOnce(ffi::ir::PrimExpr, ffi::ir::PrimExpr, ffi::ir::Span) -> Result<O>,
    O: Into<ObjectRef>,
{
    let span = empty_span()?;
    let expr = op(lhs.into_prim_expr()?, rhs.into_prim_expr()?, span)?;
    Ok(PredExpr::Expr(ffi::ir::PrimExpr::from_object(expr.into())))
}

fn downcast_ir_module(object: ObjectRef) -> Result<ffi::ir::IRModule> {
    object.try_into().map_err(|_object: ObjectRef| {
        tvm_ffi::Error::new(TYPE_ERROR, "IRBuilder did not produce an ir.IRModule", "")
    })
}
