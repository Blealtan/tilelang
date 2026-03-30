#![allow(unused_imports)]
#![allow(non_snake_case, nonstandard_style)]

use std::sync::LazyLock;
use tvm_ffi::{Any, AnyView, Function, Result};

pub mod ir {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_IR_DICTATTRSGETDICT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.DictAttrsGetDict").expect("missing global function")
    });
    pub fn DictAttrsGetDict(
        _0: crate::ir::DictAttrs,
    ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
        let func = &*FUNC_IR_DICTATTRSGETDICT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>);
        typed(_0.into())
    }

    static FUNC_IR_ENVFUNCGET: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.EnvFuncGet").expect("missing global function"));
    pub fn EnvFuncGet(_0: tvm_ffi::String) -> Result<crate::ir::EnvFunc> {
        let func = &*FUNC_IR_ENVFUNCGET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::ir::EnvFunc>);
        typed(_0)
    }

    static FUNC_IR_ENVFUNCCALL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.EnvFuncCall").expect("missing global function"));
    pub fn EnvFuncCall(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_IR_ENVFUNCCALL;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_IR_ENVFUNCGETFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.EnvFuncGetFunction").expect("missing global function")
    });
    pub fn EnvFuncGetFunction(_0: crate::ir::EnvFunc) -> Result<tvm_ffi::Function> {
        let func = &*FUNC_IR_ENVFUNCGETFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Function>);
        typed(_0.into())
    }

    static FUNC_IR_GLOBALVAR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.GlobalVar").expect("missing global function"));
    pub fn GlobalVar(_0: tvm_ffi::String) -> Result<crate::ir::GlobalVar> {
        let func = &*FUNC_IR_GLOBALVAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::ir::GlobalVar>);
        typed(_0)
    }

    static FUNC_IR_DEBUGPRINT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.DebugPrint").expect("missing global function"));
    pub fn DebugPrint(_0: tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::String> {
        let func = &*FUNC_IR_DEBUGPRINT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::String>);
        typed(_0)
    }

    static FUNC_IR_FLOATIMM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.FloatImm").expect("missing global function"));
    pub fn FloatImm(
        _0: tvm_ffi::DLDataType,
        _1: f64,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::FloatImm> {
        let func = &*FUNC_IR_FLOATIMM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, f64, tvm_ffi::object::ObjectRef) -> Result<crate::ir::FloatImm>);
        typed(_0, _1, _2.into())
    }

    static FUNC_IR_INTIMM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.IntImm").expect("missing global function"));
    pub fn IntImm(
        _0: tvm_ffi::DLDataType,
        _1: i64,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::IntImm> {
        let func = &*FUNC_IR_INTIMM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, i64, tvm_ffi::object::ObjectRef) -> Result<crate::ir::IntImm>);
        typed(_0, _1, _2.into())
    }

    static FUNC_IR_RANGE_FROM_MIN_EXTENT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Range_from_min_extent").expect("missing global function")
    });
    pub fn Range_from_min_extent(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::Range> {
        let func = &*FUNC_IR_RANGE_FROM_MIN_EXTENT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::Range>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_IR_RANGE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.Range").expect("missing global function"));
    pub fn Range(
        _0: crate::ir::PrimExpr,
        _1: Option<crate::ir::PrimExpr>,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::Range> {
        let func = &*FUNC_IR_RANGE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                Option<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::ir::Range>
        );
        typed(_0.into(), _1, _2.into())
    }

    static FUNC_IR_BASEFUNC_ATTRS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.BaseFunc_Attrs").expect("missing global function")
    });
    pub fn BaseFunc_Attrs(_0: crate::ir::BaseFunc) -> Result<crate::ir::DictAttrs> {
        let func = &*FUNC_IR_BASEFUNC_ATTRS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::DictAttrs>);
        typed(_0.into())
    }

    static FUNC_IR_BASEFUNCCOPY: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.BaseFuncCopy").expect("missing global function"));
    pub fn BaseFuncCopy(_0: crate::ir::BaseFunc) -> Result<crate::ir::BaseFunc> {
        let func = &*FUNC_IR_BASEFUNCCOPY;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::BaseFunc>);
        typed(_0.into())
    }

    static FUNC_IR_BASEFUNCWITHATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.BaseFuncWithAttr").expect("missing global function")
    });
    pub fn BaseFuncWithAttr(
        _0: crate::ObjectRValueRef,
        _1: tvm_ffi::String,
        _2: tvm_ffi::AnyValue,
    ) -> Result<crate::ir::BaseFunc> {
        let func = &*FUNC_IR_BASEFUNCWITHATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::AnyValue) -> Result<crate::ir::BaseFunc>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_BASEFUNCWITHATTRS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.BaseFuncWithAttrs").expect("missing global function")
    });
    pub fn BaseFuncWithAttrs(
        _0: crate::ObjectRValueRef,
        _1: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    ) -> Result<crate::ir::BaseFunc> {
        let func = &*FUNC_IR_BASEFUNCWITHATTRS;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            ) -> Result<crate::ir::BaseFunc>
        );
        typed(_0.into(), _1)
    }

    static FUNC_IR_BASEFUNCWITHOUTATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.BaseFuncWithoutAttr").expect("missing global function")
    });
    pub fn BaseFuncWithoutAttr(
        _0: crate::ObjectRValueRef,
        _1: tvm_ffi::String,
    ) -> Result<crate::ir::BaseFunc> {
        let func = &*FUNC_IR_BASEFUNCWITHOUTATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::BaseFunc>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_VDEVICE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.VDevice").expect("missing global function"));
    pub fn VDevice(
        _0: crate::target::Target,
        _1: i64,
        _2: tvm_ffi::String,
    ) -> Result<crate::ir::VDevice> {
        let func = &*FUNC_IR_VDEVICE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<crate::ir::VDevice>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_DUMMYGLOBALINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.DummyGlobalInfo").expect("missing global function")
    });
    pub fn DummyGlobalInfo() -> Result<crate::ir::DummyGlobalInfo> {
        let func = &*FUNC_IR_DUMMYGLOBALINFO;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::ir::DummyGlobalInfo>);
        typed()
    }

    static FUNC_IR_GLOBALVARSUPPLY_NAMESUPPLY: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.GlobalVarSupply_NameSupply").expect("missing global function")
    });
    pub fn GlobalVarSupply_NameSupply(
        _0: crate::ir::NameSupply,
    ) -> Result<crate::ir::GlobalVarSupply> {
        let func = &*FUNC_IR_GLOBALVARSUPPLY_NAMESUPPLY;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::GlobalVarSupply>);
        typed(_0.into())
    }

    static FUNC_IR_GLOBALVARSUPPLY_IRMODULE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.GlobalVarSupply_IRModule").expect("missing global function")
    });
    pub fn GlobalVarSupply_IRModule(_0: crate::ir::IRModule) -> Result<crate::ir::GlobalVarSupply> {
        let func = &*FUNC_IR_GLOBALVARSUPPLY_IRMODULE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::GlobalVarSupply>);
        typed(_0.into())
    }

    static FUNC_IR_GLOBALVARSUPPLY_IRMODULES: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.GlobalVarSupply_IRModules").expect("missing global function")
    });
    pub fn GlobalVarSupply_IRModules(
        _0: tvm_ffi::Array<crate::ir::IRModule>,
    ) -> Result<crate::ir::GlobalVarSupply> {
        let func = &*FUNC_IR_GLOBALVARSUPPLY_IRMODULES;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(tvm_ffi::Array<crate::ir::IRModule>) -> Result<crate::ir::GlobalVarSupply>
        );
        typed(_0)
    }

    static FUNC_IR_GLOBALVARSUPPLY_FRESHGLOBAL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.GlobalVarSupply_FreshGlobal").expect("missing global function")
    });
    pub fn GlobalVarSupply_FreshGlobal(
        _0: crate::ir::GlobalVarSupply,
        _1: tvm_ffi::String,
        _2: bool,
    ) -> Result<crate::ir::GlobalVar> {
        let func = &*FUNC_IR_GLOBALVARSUPPLY_FRESHGLOBAL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<crate::ir::GlobalVar>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_GLOBALVARSUPPLY_UNIQUEGLOBALFOR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.GlobalVarSupply_UniqueGlobalFor").expect("missing global function")
    });
    pub fn GlobalVarSupply_UniqueGlobalFor(
        _0: crate::ir::GlobalVarSupply,
        _1: tvm_ffi::String,
        _2: bool,
    ) -> Result<crate::ir::GlobalVar> {
        let func = &*FUNC_IR_GLOBALVARSUPPLY_UNIQUEGLOBALFOR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<crate::ir::GlobalVar>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_GLOBALVARSUPPLY_RESERVEGLOBALVAR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.GlobalVarSupply_ReserveGlobalVar")
            .expect("missing global function")
    });
    pub fn GlobalVarSupply_ReserveGlobalVar(
        _0: crate::ir::GlobalVarSupply,
        _1: crate::ir::GlobalVar,
        _2: bool,
    ) -> Result<()> {
        let func = &*FUNC_IR_GLOBALVARSUPPLY_RESERVEGLOBALVAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool) -> Result<()>);
        typed(_0.into(), _1.into(), _2)
    }

    static FUNC_IR_IRMODULE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.IRModule").expect("missing global function"));
    pub fn IRModule(
        _0: tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
        _1: tvm_ffi::object::ObjectRef,
        _2: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_IRMODULE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
            ) -> Result<crate::ir::IRModule>
        );
        typed(_0, _1, _2)
    }

    static FUNC_IR_MODULE_CLONE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.Module_Clone").expect("missing global function"));
    pub fn Module_Clone(_0: crate::ir::IRModule) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_MODULE_CLONE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into())
    }

    static FUNC_IR_MODULE_ADD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.Module_Add").expect("missing global function"));
    pub fn Module_Add(
        _0: crate::ir::IRModule,
        _1: crate::ir::GlobalVar,
        _2: tvm_ffi::object::ObjectRef,
        _3: bool,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_MODULE_ADD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1.into(), _2, _3)
    }

    static FUNC_IR_MODULE_REMOVE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_Remove").expect("missing global function")
    });
    pub fn Module_Remove(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_IR_MODULE_REMOVE;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_IR_MODULE_CONTAINS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_Contains").expect("missing global function")
    });
    pub fn Module_Contains(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_IR_MODULE_CONTAINS;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_IR_MODULE_GETGLOBALVAR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_GetGlobalVar").expect("missing global function")
    });
    pub fn Module_GetGlobalVar(
        _0: crate::ir::IRModule,
        _1: tvm_ffi::String,
    ) -> Result<crate::ir::GlobalVar> {
        let func = &*FUNC_IR_MODULE_GETGLOBALVAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::GlobalVar>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_MODULE_GETGLOBALVARS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_GetGlobalVars").expect("missing global function")
    });
    pub fn Module_GetGlobalVars(
        _0: crate::ir::IRModule,
    ) -> Result<tvm_ffi::Array<crate::ir::GlobalVar>> {
        let func = &*FUNC_IR_MODULE_GETGLOBALVARS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::GlobalVar>>);
        typed(_0.into())
    }

    static FUNC_IR_MODULE_CONTAINGLOBALVAR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_ContainGlobalVar").expect("missing global function")
    });
    pub fn Module_ContainGlobalVar(_0: crate::ir::IRModule, _1: tvm_ffi::String) -> Result<bool> {
        let func = &*FUNC_IR_MODULE_CONTAINGLOBALVAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<bool>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_MODULE_LOOKUP: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_Lookup").expect("missing global function")
    });
    pub fn Module_Lookup(
        _0: crate::ir::IRModule,
        _1: crate::ir::GlobalVar,
    ) -> Result<crate::ir::BaseFunc> {
        let func = &*FUNC_IR_MODULE_LOOKUP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::BaseFunc>);
        typed(_0.into(), _1.into())
    }

    static FUNC_IR_MODULE_LOOKUP_STR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_Lookup_str").expect("missing global function")
    });
    pub fn Module_Lookup_str(
        _0: crate::ir::IRModule,
        _1: tvm_ffi::String,
    ) -> Result<crate::ir::BaseFunc> {
        let func = &*FUNC_IR_MODULE_LOOKUP_STR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::BaseFunc>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_MODULE_FROMEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_FromExpr").expect("missing global function")
    });
    pub fn Module_FromExpr(
        _0: crate::ir::RelaxExpr,
        _1: tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_MODULE_FROMEXPR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
            ) -> Result<crate::ir::IRModule>
        );
        typed(_0.into(), _1)
    }

    static FUNC_IR_MODULE_UPDATE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_Update").expect("missing global function")
    });
    pub fn Module_Update(_0: crate::ir::IRModule, _1: crate::ir::IRModule) -> Result<()> {
        let func = &*FUNC_IR_MODULE_UPDATE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_IR_MODULE_UPDATEFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_UpdateFunction").expect("missing global function")
    });
    pub fn Module_UpdateFunction(
        _0: crate::ir::IRModule,
        _1: crate::ir::GlobalVar,
        _2: crate::ir::BaseFunc,
    ) -> Result<()> {
        let func = &*FUNC_IR_MODULE_UPDATEFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_IR_MODULE_UPDATEGLOBALINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_UpdateGlobalInfo").expect("missing global function")
    });
    pub fn Module_UpdateGlobalInfo(
        _0: crate::ir::IRModule,
        _1: tvm_ffi::String,
        _2: tvm_ffi::Array<crate::ir::GlobalInfo>,
    ) -> Result<()> {
        let func = &*FUNC_IR_MODULE_UPDATEGLOBALINFO;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::String,
                tvm_ffi::Array<crate::ir::GlobalInfo>,
            ) -> Result<()>
        );
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_MODULE_GETATTRS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_GetAttrs").expect("missing global function")
    });
    pub fn Module_GetAttrs(_0: crate::ir::IRModule) -> Result<tvm_ffi::object::ObjectRef> {
        let func = &*FUNC_IR_MODULE_GETATTRS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::object::ObjectRef>);
        typed(_0.into())
    }

    static FUNC_IR_MODULE_WITHATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_WithAttr").expect("missing global function")
    });
    pub fn Module_WithAttr(
        _0: crate::ObjectRValueRef,
        _1: tvm_ffi::String,
        _2: tvm_ffi::AnyValue,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_MODULE_WITHATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::AnyValue) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_MODULE_WITHOUTATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_WithoutAttr").expect("missing global function")
    });
    pub fn Module_WithoutAttr(
        _0: crate::ObjectRValueRef,
        _1: tvm_ffi::String,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_MODULE_WITHOUTATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_MODULE_WITHATTRS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_WithAttrs").expect("missing global function")
    });
    pub fn Module_WithAttrs(
        _0: crate::ObjectRValueRef,
        _1: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_IR_MODULE_WITHATTRS;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            ) -> Result<crate::ir::IRModule>
        );
        typed(_0.into(), _1)
    }

    static FUNC_IR_MODULE_GETATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_GetAttr").expect("missing global function")
    });
    pub fn Module_GetAttr(
        _0: crate::ir::IRModule,
        _1: tvm_ffi::String,
    ) -> Result<tvm_ffi::object::ObjectRef> {
        let func = &*FUNC_IR_MODULE_GETATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<tvm_ffi::object::ObjectRef>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_NAMESUPPLY: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.NameSupply").expect("missing global function"));
    pub fn NameSupply(_0: tvm_ffi::String) -> Result<crate::ir::NameSupply> {
        let func = &*FUNC_IR_NAMESUPPLY;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::ir::NameSupply>);
        typed(_0)
    }

    static FUNC_IR_NAMESUPPLY_FRESHNAME: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.NameSupply_FreshName").expect("missing global function")
    });
    pub fn NameSupply_FreshName(
        _0: crate::ir::NameSupply,
        _1: tvm_ffi::String,
        _2: bool,
        _3: bool,
    ) -> Result<tvm_ffi::String> {
        let func = &*FUNC_IR_NAMESUPPLY_FRESHNAME;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool, bool) -> Result<tvm_ffi::String>);
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_IR_NAMESUPPLY_RESERVENAME: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.NameSupply_ReserveName").expect("missing global function")
    });
    pub fn NameSupply_ReserveName(
        _0: crate::ir::NameSupply,
        _1: tvm_ffi::String,
        _2: bool,
    ) -> Result<tvm_ffi::String> {
        let func = &*FUNC_IR_NAMESUPPLY_RESERVENAME;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<tvm_ffi::String>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_NAMESUPPLY_CONTAINSNAME: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.NameSupply_ContainsName").expect("missing global function")
    });
    pub fn NameSupply_ContainsName(
        _0: crate::ir::NameSupply,
        _1: tvm_ffi::String,
        _2: bool,
    ) -> Result<bool> {
        let func = &*FUNC_IR_NAMESUPPLY_CONTAINSNAME;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<bool>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_IR_LISTOPNAMES: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.ListOpNames").expect("missing global function"));
    pub fn ListOpNames() -> Result<tvm_ffi::Array<tvm_ffi::String>> {
        let func = &*FUNC_IR_LISTOPNAMES;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<tvm_ffi::Array<tvm_ffi::String>>);
        typed()
    }

    static FUNC_IR_GETOP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.GetOp").expect("missing global function"));
    pub fn GetOp(_0: tvm_ffi::String) -> Result<crate::ir::Op> {
        let func = &*FUNC_IR_GETOP;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::ir::Op>);
        typed(_0)
    }

    static FUNC_IR_OPGETATTR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.OpGetAttr").expect("missing global function"));
    pub fn OpGetAttr(_0: crate::ir::Op, _1: tvm_ffi::String) -> Result<tvm_ffi::AnyValue> {
        let func = &*FUNC_IR_OPGETATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<tvm_ffi::AnyValue>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_OPHASATTR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.OpHasAttr").expect("missing global function"));
    pub fn OpHasAttr(_0: crate::ir::Op, _1: tvm_ffi::String) -> Result<bool> {
        let func = &*FUNC_IR_OPHASATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<bool>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_OPSETATTR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.OpSetAttr").expect("missing global function"));
    pub fn OpSetAttr(
        _0: crate::ir::Op,
        _1: tvm_ffi::String,
        _2: tvm_ffi::AnyValue,
        _3: i64,
    ) -> Result<()> {
        let func = &*FUNC_IR_OPSETATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::AnyValue, i64) -> Result<()>);
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_IR_OPRESETATTR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.OpResetAttr").expect("missing global function"));
    pub fn OpResetAttr(_0: crate::ir::Op, _1: tvm_ffi::String) -> Result<()> {
        let func = &*FUNC_IR_OPRESETATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_REGISTEROP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.RegisterOp").expect("missing global function"));
    pub fn RegisterOp(_0: tvm_ffi::String, _1: tvm_ffi::String) -> Result<()> {
        let func = &*FUNC_IR_REGISTEROP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::String) -> Result<()>);
        typed(_0, _1)
    }

    static FUNC_IR_OPADDARGUMENT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.OpAddArgument").expect("missing global function")
    });
    pub fn OpAddArgument(
        _0: crate::ir::Op,
        _1: tvm_ffi::String,
        _2: tvm_ffi::String,
        _3: tvm_ffi::String,
    ) -> Result<()> {
        let func = &*FUNC_IR_OPADDARGUMENT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::String, tvm_ffi::String) -> Result<()>);
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_IR_OPSETSUPPORTLEVEL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.OpSetSupportLevel").expect("missing global function")
    });
    pub fn OpSetSupportLevel(_0: crate::ir::Op, _1: i64) -> Result<()> {
        let func = &*FUNC_IR_OPSETSUPPORTLEVEL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_OPSETNUMINPUTS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.OpSetNumInputs").expect("missing global function")
    });
    pub fn OpSetNumInputs(_0: crate::ir::Op, _1: i64) -> Result<()> {
        let func = &*FUNC_IR_OPSETNUMINPUTS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_OPSETATTRSTYPEKEY: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.OpSetAttrsTypeKey").expect("missing global function")
    });
    pub fn OpSetAttrsTypeKey(_0: crate::ir::Op, _1: tvm_ffi::String) -> Result<()> {
        let func = &*FUNC_IR_OPSETATTRSTYPEKEY;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_REGISTEROPATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.RegisterOpAttr").expect("missing global function")
    });
    pub fn RegisterOpAttr(
        _0: tvm_ffi::String,
        _1: tvm_ffi::String,
        _2: tvm_ffi::AnyValue,
        _3: i64,
    ) -> Result<()> {
        let func = &*FUNC_IR_REGISTEROPATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::String, tvm_ffi::AnyValue, i64) -> Result<()>);
        typed(_0, _1, _2, _3)
    }

    static FUNC_IR_REGISTEROPLOWERINTRINSIC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.RegisterOpLowerIntrinsic").expect("missing global function")
    });
    pub fn RegisterOpLowerIntrinsic(
        _0: tvm_ffi::String,
        _1: tvm_ffi::Function,
        _2: tvm_ffi::String,
        _3: i64,
    ) -> Result<()> {
        let func = &*FUNC_IR_REGISTEROPLOWERINTRINSIC;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::Function, tvm_ffi::String, i64) -> Result<()>);
        typed(_0, _1, _2, _3)
    }

    static FUNC_IR_MODULE_REPLACEGLOBALVARS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.Module_ReplaceGlobalVars").expect("missing global function")
    });
    pub fn Module_ReplaceGlobalVars(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_IR_MODULE_REPLACEGLOBALVARS;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_IR_SPAN: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.Span").expect("missing global function"));
    pub fn Span(
        _0: crate::ir::SourceName,
        _1: i64,
        _2: i64,
        _3: i64,
        _4: i64,
    ) -> Result<crate::ir::Span> {
        let func = &*FUNC_IR_SPAN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64, i64, i64) -> Result<crate::ir::Span>);
        typed(_0.into(), _1, _2, _3, _4)
    }

    static FUNC_IR_SEQUENTIALSPAN: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.SequentialSpan").expect("missing global function")
    });
    pub fn SequentialSpan(
        _0: tvm_ffi::Array<crate::ir::Span>,
    ) -> Result<crate::ir::SequentialSpan> {
        let func = &*FUNC_IR_SEQUENTIALSPAN;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(tvm_ffi::Array<crate::ir::Span>) -> Result<crate::ir::SequentialSpan>
        );
        typed(_0)
    }

    static FUNC_IR_SOURCENAME: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.SourceName").expect("missing global function"));
    pub fn SourceName(_0: tvm_ffi::String) -> Result<crate::ir::SourceName> {
        let func = &*FUNC_IR_SOURCENAME;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::ir::SourceName>);
        typed(_0)
    }

    static FUNC_IR_TUPLETYPE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.TupleType").expect("missing global function"));
    pub fn TupleType(_0: tvm_ffi::Array<crate::ir::Type>) -> Result<crate::ir::TupleType> {
        let func = &*FUNC_IR_TUPLETYPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(tvm_ffi::Array<crate::ir::Type>) -> Result<crate::ir::TupleType>
        );
        typed(_0)
    }

    static FUNC_IR_TENSORMAPTYPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.TensorMapType").expect("missing global function")
    });
    pub fn TensorMapType(_0: crate::ir::Span) -> Result<crate::ir::TensorMapType> {
        let func = &*FUNC_IR_TENSORMAPTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::TensorMapType>);
        typed(_0.into())
    }

    static FUNC_IR_FUNCTYPE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.FuncType").expect("missing global function"));
    pub fn FuncType(
        _0: tvm_ffi::Array<crate::ir::Type>,
        _1: crate::ir::Type,
    ) -> Result<crate::ir::FuncType> {
        let func = &*FUNC_IR_FUNCTYPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::Type>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::ir::FuncType>
        );
        typed(_0, _1.into())
    }

    static FUNC_IR_POINTERTYPE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.PointerType").expect("missing global function"));
    pub fn PointerType(_0: crate::ir::Type, _1: tvm_ffi::String) -> Result<crate::ir::PointerType> {
        let func = &*FUNC_IR_POINTERTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::PointerType>);
        typed(_0.into(), _1)
    }

    static FUNC_IR_PRIMTYPE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("ir.PrimType").expect("missing global function"));
    pub fn PrimType(_0: tvm_ffi::DLDataType) -> Result<crate::ir::PrimType> {
        let func = &*FUNC_IR_PRIMTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType) -> Result<crate::ir::PrimType>);
        typed(_0)
    }

    static FUNC_IR_EXPRSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("ir.ExprStructInfo").expect("missing global function")
    });
    pub fn ExprStructInfo(_0: crate::ir::RelaxExpr) -> Result<crate::ir::StructInfo> {
        let func = &*FUNC_IR_EXPRSTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::StructInfo>);
        typed(_0.into())
    }

    pub mod analysis {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_IR_ANALYSIS_COLLECTCALLMAP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("ir.analysis.CollectCallMap").expect("missing global function")
        });
        pub fn CollectCallMap(
            _0: crate::ir::IRModule,
        ) -> Result<tvm_ffi::Map<crate::ir::GlobalVar, tvm_ffi::Array<crate::ir::GlobalVar>>>
        {
            let func = &*FUNC_IR_ANALYSIS_COLLECTCALLMAP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<crate::ir::GlobalVar, tvm_ffi::Array<crate::ir::GlobalVar>>>);
            typed(_0.into())
        }
    }
}
pub mod relax {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_RELAX_STRUCTINFOISBASEOF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.StructInfoIsBaseOf").expect("missing global function")
    });
    pub fn StructInfoIsBaseOf(
        _0: crate::ir::StructInfo,
        _1: crate::ir::StructInfo,
    ) -> Result<bool> {
        let func = &*FUNC_RELAX_STRUCTINFOISBASEOF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_VMLINK: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.VMLink").expect("missing global function"));
    pub fn VMLink(
        _0: crate::relax::ExecBuilder,
        _1: crate::target::Target,
        _2: Option<tvm_ffi::Module>,
        _3: tvm_ffi::Array<tvm_ffi::Module>,
        _4: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Tensor>,
    ) -> Result<tvm_ffi::Module> {
        let func = &*FUNC_RELAX_VMLINK;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                Option<tvm_ffi::Module>,
                tvm_ffi::Array<tvm_ffi::Module>,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Tensor>,
            ) -> Result<tvm_ffi::Module>
        );
        typed(_0.into(), _1.into(), _2, _3, _4)
    }

    static FUNC_RELAX_VMCODEGEN: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.VMCodeGen").expect("missing global function"));
    pub fn VMCodeGen(
        _0: crate::relax::ExecBuilder,
        _1: crate::ir::IRModule,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_RELAX_VMCODEGEN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_VMTIRCODEGEN: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.VMTIRCodeGen").expect("missing global function")
    });
    pub fn VMTIRCodeGen(
        _0: crate::relax::ExecBuilder,
        _1: crate::ir::IRModule,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_RELAX_VMTIRCODEGEN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXECBUILDERCREATE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderCreate").expect("missing global function")
    });
    pub fn ExecBuilderCreate() -> Result<crate::relax::ExecBuilder> {
        let func = &*FUNC_RELAX_EXECBUILDERCREATE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::relax::ExecBuilder>);
        typed()
    }

    static FUNC_RELAX_EXECBUILDERCONVERTCONSTANT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderConvertConstant").expect("missing global function")
    });
    pub fn ExecBuilderConvertConstant(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_RELAX_EXECBUILDERCONVERTCONSTANT;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_RELAX_EXECBUILDEREMITFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderEmitFunction").expect("missing global function")
    });
    pub fn ExecBuilderEmitFunction(
        _0: crate::relax::ExecBuilder,
        _1: tvm_ffi::String,
        _2: i64,
        _3: Option<tvm_ffi::Array<tvm_ffi::String>>,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDEREMITFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::String,
                i64,
                Option<tvm_ffi::Array<tvm_ffi::String>>,
            ) -> Result<()>
        );
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_RELAX_EXECBUILDERENDFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderEndFunction").expect("missing global function")
    });
    pub fn ExecBuilderEndFunction(
        _0: crate::relax::ExecBuilder,
        _1: tvm_ffi::String,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDERENDFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDERDECLAREFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderDeclareFunction").expect("missing global function")
    });
    pub fn ExecBuilderDeclareFunction(
        _0: crate::relax::ExecBuilder,
        _1: tvm_ffi::String,
        _2: i64,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDERDECLAREFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, i64) -> Result<()>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_RELAX_EXECBUILDEREMITCALL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderEmitCall").expect("missing global function")
    });
    pub fn ExecBuilderEmitCall(
        _0: crate::relax::ExecBuilder,
        _1: tvm_ffi::String,
        _2: tvm_ffi::Array<crate::ir::IntImm>,
        _3: i64,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDEREMITCALL;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::String,
                tvm_ffi::Array<crate::ir::IntImm>,
                i64,
            ) -> Result<()>
        );
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_RELAX_EXECBUILDEREMITRET: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderEmitRet").expect("missing global function")
    });
    pub fn ExecBuilderEmitRet(_0: crate::relax::ExecBuilder, _1: i64) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDEREMITRET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDEREMITGOTO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderEmitGoto").expect("missing global function")
    });
    pub fn ExecBuilderEmitGoto(_0: crate::relax::ExecBuilder, _1: i64) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDEREMITGOTO;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<()>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDEREMITIF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderEmitIf").expect("missing global function")
    });
    pub fn ExecBuilderEmitIf(_0: crate::relax::ExecBuilder, _1: i64, _2: i64) -> Result<()> {
        let func = &*FUNC_RELAX_EXECBUILDEREMITIF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64) -> Result<()>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_RELAX_EXECBUILDERR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderR").expect("missing global function")
    });
    pub fn ExecBuilderR(_0: crate::relax::ExecBuilder, _1: i64) -> Result<i64> {
        let func = &*FUNC_RELAX_EXECBUILDERR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<i64>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDERIMM: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderImm").expect("missing global function")
    });
    pub fn ExecBuilderImm(_0: crate::relax::ExecBuilder, _1: i64) -> Result<i64> {
        let func = &*FUNC_RELAX_EXECBUILDERIMM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<i64>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDERC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderC").expect("missing global function")
    });
    pub fn ExecBuilderC(_0: crate::relax::ExecBuilder, _1: i64) -> Result<i64> {
        let func = &*FUNC_RELAX_EXECBUILDERC;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<i64>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDERF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderF").expect("missing global function")
    });
    pub fn ExecBuilderF(_0: crate::relax::ExecBuilder, _1: tvm_ffi::String) -> Result<i64> {
        let func = &*FUNC_RELAX_EXECBUILDERF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<i64>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXECBUILDERGET: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecBuilderGet").expect("missing global function")
    });
    pub fn ExecBuilderGet(_0: crate::relax::ExecBuilder) -> Result<tvm_ffi::Module> {
        let func = &*FUNC_RELAX_EXECBUILDERGET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
        typed(_0.into())
    }

    static FUNC_RELAX_DFB_REWRITE_MUTATE_IRMODULE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.dfb_rewrite_mutate_irmodule").expect("missing global function")
    });
    pub fn dfb_rewrite_mutate_irmodule(
        _0: crate::relax::DataflowBlockRewrite,
        _1: crate::ir::IRModule,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_RELAX_DFB_REWRITE_MUTATE_IRMODULE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_DFB_REWRITE_REMOVE_ALL_UNUSED: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.dfb_rewrite_remove_all_unused")
            .expect("missing global function")
    });
    pub fn dfb_rewrite_remove_all_unused(_0: crate::relax::DataflowBlockRewrite) -> Result<()> {
        let func = &*FUNC_RELAX_DFB_REWRITE_REMOVE_ALL_UNUSED;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_RELAX_DFB_REWRITE_REMOVE_UNUSED: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.dfb_rewrite_remove_unused").expect("missing global function")
    });
    pub fn dfb_rewrite_remove_unused(
        _0: crate::relax::DataflowBlockRewrite,
        _1: crate::relax::expr::Var,
        _2: bool,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_DFB_REWRITE_REMOVE_UNUSED;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool) -> Result<()>);
        typed(_0.into(), _1.into(), _2)
    }

    static FUNC_RELAX_DFB_REWRITE_ADD_BINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.dfb_rewrite_add_binding").expect("missing global function")
    });
    pub fn dfb_rewrite_add_binding(
        _0: crate::relax::DataflowBlockRewrite,
        _1: crate::relax::expr::Binding,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_DFB_REWRITE_ADD_BINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_DFB_REWRITE_ADD: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.dfb_rewrite_add").expect("missing global function")
    });
    pub fn dfb_rewrite_add(
        _0: crate::relax::DataflowBlockRewrite,
        _1: crate::ir::RelaxExpr,
        _2: Option<tvm_ffi::String>,
        _3: bool,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_DFB_REWRITE_ADD;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                Option<tvm_ffi::String>,
                bool,
            ) -> Result<()>
        );
        typed(_0.into(), _1.into(), _2, _3)
    }

    static FUNC_RELAX_DFB_REWRITE_REPLACE_ALL_USES: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.dfb_rewrite_replace_all_uses").expect("missing global function")
    });
    pub fn dfb_rewrite_replace_all_uses(
        _0: crate::relax::DataflowBlockRewrite,
        _1: crate::relax::expr::Var,
        _2: crate::relax::expr::Var,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_DFB_REWRITE_REPLACE_ALL_USES;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_RELAX_DATAFLOWBLOCKREWRITE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.DataflowBlockRewrite").expect("missing global function")
    });
    pub fn DataflowBlockRewrite(
        _0: crate::relax::expr::DataflowBlock,
        _1: crate::relax::expr::Function,
    ) -> Result<crate::relax::DataflowBlockRewrite> {
        let func = &*FUNC_RELAX_DATAFLOWBLOCKREWRITE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::DataflowBlockRewrite>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_BLOCKBUILDERCREATE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderCreate").expect("missing global function")
    });
    pub fn BlockBuilderCreate(
        _0: Option<crate::ir::IRModule>,
    ) -> Result<crate::relax::BlockBuilder> {
        let func = &*FUNC_RELAX_BLOCKBUILDERCREATE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(Option<crate::ir::IRModule>) -> Result<crate::relax::BlockBuilder>
        );
        typed(_0)
    }

    static FUNC_RELAX_BLOCKBUILDERBEGINDATAFLOWBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderBeginDataflowBlock")
            .expect("missing global function")
    });
    pub fn BlockBuilderBeginDataflowBlock(_0: crate::relax::BlockBuilder) -> Result<()> {
        let func = &*FUNC_RELAX_BLOCKBUILDERBEGINDATAFLOWBLOCK;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_RELAX_BLOCKBUILDERBEGINBINDINGBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderBeginBindingBlock")
            .expect("missing global function")
    });
    pub fn BlockBuilderBeginBindingBlock(_0: crate::relax::BlockBuilder) -> Result<()> {
        let func = &*FUNC_RELAX_BLOCKBUILDERBEGINBINDINGBLOCK;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_RELAX_BLOCKBUILDERENDBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderEndBlock").expect("missing global function")
    });
    pub fn BlockBuilderEndBlock(
        _0: crate::relax::BlockBuilder,
    ) -> Result<crate::relax::expr::BindingBlock> {
        let func = &*FUNC_RELAX_BLOCKBUILDERENDBLOCK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::BindingBlock>);
        typed(_0.into())
    }

    static FUNC_RELAX_BLOCKBUILDERNORMALIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderNormalize").expect("missing global function")
    });
    pub fn BlockBuilderNormalize(
        _0: crate::relax::BlockBuilder,
        _1: crate::ir::RelaxExpr,
    ) -> Result<crate::ir::RelaxExpr> {
        let func = &*FUNC_RELAX_BLOCKBUILDERNORMALIZE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_BLOCKBUILDEREMIT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderEmit").expect("missing global function")
    });
    pub fn BlockBuilderEmit(
        _0: crate::relax::BlockBuilder,
        _1: crate::ir::RelaxExpr,
        _2: tvm_ffi::String,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_BLOCKBUILDEREMIT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into(), _2)
    }

    static FUNC_RELAX_BLOCKBUILDEREMITMATCHCAST: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderEmitMatchCast").expect("missing global function")
    });
    pub fn BlockBuilderEmitMatchCast(
        _0: crate::relax::BlockBuilder,
        _1: crate::ir::RelaxExpr,
        _2: crate::ir::StructInfo,
        _3: tvm_ffi::String,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_BLOCKBUILDEREMITMATCHCAST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into(), _2.into(), _3)
    }

    static FUNC_RELAX_BLOCKBUILDEREMITOUTPUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderEmitOutput").expect("missing global function")
    });
    pub fn BlockBuilderEmitOutput(
        _0: crate::relax::BlockBuilder,
        _1: crate::ir::RelaxExpr,
        _2: tvm_ffi::String,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_BLOCKBUILDEREMITOUTPUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into(), _2)
    }

    static FUNC_RELAX_BLOCKBUILDEREMITNORMALIZED: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderEmitNormalized").expect("missing global function")
    });
    pub fn BlockBuilderEmitNormalized(
        _0: crate::relax::BlockBuilder,
        _1: crate::relax::expr::Binding,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_BLOCKBUILDEREMITNORMALIZED;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_BLOCKBUILDERGETUNIQUENAME: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderGetUniqueName").expect("missing global function")
    });
    pub fn BlockBuilderGetUniqueName(
        _0: crate::relax::BlockBuilder,
        _1: tvm_ffi::String,
    ) -> Result<tvm_ffi::String> {
        let func = &*FUNC_RELAX_BLOCKBUILDERGETUNIQUENAME;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<tvm_ffi::String>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_BLOCKBUILDERADDFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderAddFunction").expect("missing global function")
    });
    pub fn BlockBuilderAddFunction(
        _0: crate::relax::BlockBuilder,
        _1: crate::ir::BaseFunc,
        _2: tvm_ffi::String,
    ) -> Result<crate::ir::GlobalVar> {
        let func = &*FUNC_RELAX_BLOCKBUILDERADDFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::GlobalVar>);
        typed(_0.into(), _1.into(), _2)
    }

    static FUNC_RELAX_BLOCKBUILDERUPDATEFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderUpdateFunction").expect("missing global function")
    });
    pub fn BlockBuilderUpdateFunction(
        _0: crate::relax::BlockBuilder,
        _1: crate::ir::GlobalVar,
        _2: crate::ir::BaseFunc,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_BLOCKBUILDERUPDATEFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_RELAX_BLOCKBUILDERGETCONTEXTIRMODULE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderGetContextIRModule")
            .expect("missing global function")
    });
    pub fn BlockBuilderGetContextIRModule(
        _0: crate::relax::BlockBuilder,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_RELAX_BLOCKBUILDERGETCONTEXTIRMODULE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into())
    }

    static FUNC_RELAX_BLOCKBUILDERFINALIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderFinalize").expect("missing global function")
    });
    pub fn BlockBuilderFinalize(_0: crate::relax::BlockBuilder) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_RELAX_BLOCKBUILDERFINALIZE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into())
    }

    static FUNC_RELAX_BLOCKBUILDERCURRENTBLOCKISDATAFLOW: LazyLock<Function> =
        LazyLock::new(|| {
            Function::get_global("relax.BlockBuilderCurrentBlockIsDataFlow")
                .expect("missing global function")
        });
    pub fn BlockBuilderCurrentBlockIsDataFlow(_0: crate::relax::BlockBuilder) -> Result<bool> {
        let func = &*FUNC_RELAX_BLOCKBUILDERCURRENTBLOCKISDATAFLOW;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_RELAX_BLOCKBUILDERLOOKUPBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderLookupBinding").expect("missing global function")
    });
    pub fn BlockBuilderLookupBinding(
        _0: crate::relax::BlockBuilder,
        _1: crate::relax::expr::Var,
    ) -> Result<Option<crate::ir::RelaxExpr>> {
        let func = &*FUNC_RELAX_BLOCKBUILDERLOOKUPBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<crate::ir::RelaxExpr>>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_BLOCKBUILDERBEGINSCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderBeginScope").expect("missing global function")
    });
    pub fn BlockBuilderBeginScope(
        _0: crate::relax::BlockBuilder,
        _1: Option<tvm_ffi::Array<crate::relax::expr::Var>>,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_BLOCKBUILDERBEGINSCOPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                Option<tvm_ffi::Array<crate::relax::expr::Var>>,
            ) -> Result<()>
        );
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_BLOCKBUILDERENDSCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BlockBuilderEndScope").expect("missing global function")
    });
    pub fn BlockBuilderEndScope(_0: crate::relax::BlockBuilder) -> Result<()> {
        let func = &*FUNC_RELAX_BLOCKBUILDERENDSCOPE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_RELAX_TETENSOR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.TETensor").expect("missing global function"));
    pub fn TETensor(
        _0: crate::ir::RelaxExpr,
        _1: tvm_ffi::Map<crate::tir::Var, crate::ir::PrimExpr>,
        _2: tvm_ffi::String,
    ) -> Result<crate::te::Tensor> {
        let func = &*FUNC_RELAX_TETENSOR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<crate::tir::Var, crate::ir::PrimExpr>,
                tvm_ffi::String,
            ) -> Result<crate::te::Tensor>
        );
        typed(_0.into(), _1, _2)
    }

    static FUNC_RELAX_GETSHAPEOF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.GetShapeOf").expect("missing global function")
    });
    pub fn GetShapeOf(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
        let func = &*FUNC_RELAX_GETSHAPEOF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
        typed(_0.into())
    }

    static FUNC_RELAX_FUNCWITHATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FuncWithAttr").expect("missing global function")
    });
    pub fn FuncWithAttr(
        _0: crate::ir::BaseFunc,
        _1: tvm_ffi::String,
        _2: tvm_ffi::object::ObjectRef,
    ) -> Result<Option<crate::relax::expr::Function>> {
        let func = &*FUNC_RELAX_FUNCWITHATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<Option<crate::relax::expr::Function>>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_RELAX_FUNCWITHATTRS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FuncWithAttrs").expect("missing global function")
    });
    pub fn FuncWithAttrs(
        _0: crate::ir::BaseFunc,
        _1: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    ) -> Result<Option<crate::relax::expr::Function>> {
        let func = &*FUNC_RELAX_FUNCWITHATTRS;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            ) -> Result<Option<crate::relax::expr::Function>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_FUNCWITHOUTATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FuncWithoutAttr").expect("missing global function")
    });
    pub fn FuncWithoutAttr(
        _0: crate::ir::BaseFunc,
        _1: tvm_ffi::String,
    ) -> Result<Option<crate::relax::expr::Function>> {
        let func = &*FUNC_RELAX_FUNCWITHOUTATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<Option<crate::relax::expr::Function>>);
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_EXTERNFUNC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExternFunc").expect("missing global function")
    });
    pub fn ExternFunc(
        _0: tvm_ffi::String,
        _1: Option<crate::ir::StructInfo>,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::ExternFunc> {
        let func = &*FUNC_RELAX_EXTERNFUNC;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                Option<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::ExternFunc>
        );
        typed(_0, _1, _2.into())
    }

    static FUNC_RELAX_FUNCTIONCREATEEMPTY: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FunctionCreateEmpty").expect("missing global function")
    });
    pub fn FunctionCreateEmpty(
        _0: tvm_ffi::Array<crate::relax::expr::Var>,
        _1: crate::ir::StructInfo,
        _2: bool,
        _3: crate::ir::DictAttrs,
        _4: crate::ir::Span,
    ) -> Result<crate::relax::expr::Function> {
        let func = &*FUNC_RELAX_FUNCTIONCREATEEMPTY;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::relax::expr::Var>,
                tvm_ffi::object::ObjectRef,
                bool,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Function>
        );
        typed(_0, _1.into(), _2, _3.into(), _4.into())
    }

    static FUNC_RELAX_FUNCTION: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.Function").expect("missing global function"));
    pub fn Function(
        _0: tvm_ffi::Array<crate::relax::expr::Var>,
        _1: crate::ir::RelaxExpr,
        _2: Option<crate::ir::StructInfo>,
        _3: bool,
        _4: crate::ir::DictAttrs,
        _5: crate::ir::Span,
    ) -> Result<crate::relax::expr::Function> {
        let func = &*FUNC_RELAX_FUNCTION;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::relax::expr::Var>,
                tvm_ffi::object::ObjectRef,
                Option<crate::ir::StructInfo>,
                bool,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Function>
        );
        typed(_0, _1.into(), _2, _3, _4.into(), _5.into())
    }

    static FUNC_RELAX_SEQEXPR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.SeqExpr").expect("missing global function"));
    pub fn SeqExpr(
        _0: tvm_ffi::Array<crate::relax::expr::BindingBlock>,
        _1: crate::ir::RelaxExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::SeqExpr> {
        let func = &*FUNC_RELAX_SEQEXPR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::relax::expr::BindingBlock>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::SeqExpr>
        );
        typed(_0, _1.into(), _2.into())
    }

    static FUNC_RELAX_DATAFLOWBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.DataflowBlock").expect("missing global function")
    });
    pub fn DataflowBlock(
        _0: tvm_ffi::Array<crate::relax::expr::Binding>,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::DataflowBlock> {
        let func = &*FUNC_RELAX_DATAFLOWBLOCK;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::relax::expr::Binding>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::DataflowBlock>
        );
        typed(_0, _1.into())
    }

    static FUNC_RELAX_BINDINGBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.BindingBlock").expect("missing global function")
    });
    pub fn BindingBlock(
        _0: tvm_ffi::Array<crate::relax::expr::Binding>,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::BindingBlock> {
        let func = &*FUNC_RELAX_BINDINGBLOCK;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::relax::expr::Binding>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::BindingBlock>
        );
        typed(_0, _1.into())
    }

    static FUNC_RELAX_VARBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.VarBinding").expect("missing global function")
    });
    pub fn VarBinding(
        _0: crate::relax::expr::Var,
        _1: crate::ir::RelaxExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::VarBinding> {
        let func = &*FUNC_RELAX_VARBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::VarBinding>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_RELAX_MATCHCAST: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.MatchCast").expect("missing global function"));
    pub fn MatchCast(
        _0: crate::relax::expr::Var,
        _1: crate::ir::RelaxExpr,
        _2: crate::ir::StructInfo,
        _3: crate::ir::Span,
    ) -> Result<crate::relax::expr::MatchCast> {
        let func = &*FUNC_RELAX_MATCHCAST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::MatchCast>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_RELAX_DATATYPEIMM: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.DataTypeImm").expect("missing global function")
    });
    pub fn DataTypeImm(
        _0: tvm_ffi::DLDataType,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::DataTypeImm> {
        let func = &*FUNC_RELAX_DATATYPEIMM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::DataTypeImm>);
        typed(_0, _1.into())
    }

    static FUNC_RELAX_STRINGIMM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.StringImm").expect("missing global function"));
    pub fn StringImm(
        _0: tvm_ffi::String,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::StringImm> {
        let func = &*FUNC_RELAX_STRINGIMM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::StringImm>);
        typed(_0, _1.into())
    }

    static FUNC_RELAX_PRIMVALUE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.PrimValue").expect("missing global function"));
    pub fn PrimValue(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::PrimValue> {
        let func = &*FUNC_RELAX_PRIMVALUE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::PrimValue>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_CONSTANT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.Constant").expect("missing global function"));
    pub fn Constant(
        _0: tvm_ffi::Tensor,
        _1: Option<crate::ir::StructInfo>,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::Constant> {
        let func = &*FUNC_RELAX_CONSTANT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Tensor,
                Option<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Constant>
        );
        typed(_0, _1, _2.into())
    }

    static FUNC_RELAX_DATAFLOWVAR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.DataflowVar").expect("missing global function")
    });
    pub fn DataflowVar(
        _0: tvm_ffi::String,
        _1: Option<crate::ir::StructInfo>,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::DataflowVar> {
        let func = &*FUNC_RELAX_DATAFLOWVAR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                Option<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::DataflowVar>
        );
        typed(_0, _1, _2.into())
    }

    static FUNC_RELAX_DATAFLOWVARFROMID: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.DataflowVarFromId").expect("missing global function")
    });
    pub fn DataflowVarFromId(
        _0: crate::relax::Id,
        _1: Option<crate::ir::StructInfo>,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::DataflowVar> {
        let func = &*FUNC_RELAX_DATAFLOWVARFROMID;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                Option<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::DataflowVar>
        );
        typed(_0.into(), _1, _2.into())
    }

    static FUNC_RELAX_VAR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.Var").expect("missing global function"));
    pub fn Var(
        _0: tvm_ffi::String,
        _1: Option<crate::ir::StructInfo>,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_VAR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                Option<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Var>
        );
        typed(_0, _1, _2.into())
    }

    static FUNC_RELAX_VARFROMID: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.VarFromId").expect("missing global function"));
    pub fn VarFromId(
        _0: crate::relax::Id,
        _1: Option<crate::ir::StructInfo>,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_VARFROMID;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                Option<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Var>
        );
        typed(_0.into(), _1, _2.into())
    }

    static FUNC_RELAX_SHAPEEXPR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.ShapeExpr").expect("missing global function"));
    pub fn ShapeExpr(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::ShapeExpr> {
        let func = &*FUNC_RELAX_SHAPEEXPR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::ShapeExpr>
        );
        typed(_0, _1.into())
    }

    static FUNC_RELAX_TUPLEGETITEM: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.TupleGetItem").expect("missing global function")
    });
    pub fn TupleGetItem(
        _0: crate::ir::RelaxExpr,
        _1: i64,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::expr::TupleGetItem> {
        let func = &*FUNC_RELAX_TUPLEGETITEM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::TupleGetItem>);
        typed(_0.into(), _1, _2.into())
    }

    static FUNC_RELAX_TUPLE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.Tuple").expect("missing global function"));
    pub fn Tuple(
        _0: tvm_ffi::Array<crate::ir::RelaxExpr>,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::expr::Tuple> {
        let func = &*FUNC_RELAX_TUPLE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::RelaxExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Tuple>
        );
        typed(_0, _1.into())
    }

    static FUNC_RELAX_IF: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.If").expect("missing global function"));
    pub fn If(
        _0: crate::ir::RelaxExpr,
        _1: crate::ir::RelaxExpr,
        _2: crate::ir::RelaxExpr,
        _3: crate::ir::Span,
    ) -> Result<crate::relax::expr::If> {
        let func = &*FUNC_RELAX_IF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::If>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_RELAX_CALL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.Call").expect("missing global function"));
    pub fn Call(
        _0: crate::ir::RelaxExpr,
        _1: tvm_ffi::Array<crate::ir::RelaxExpr>,
        _2: crate::ir::Attrs,
        _3: tvm_ffi::Array<crate::ir::StructInfo>,
        _4: crate::ir::Span,
    ) -> Result<crate::relax::expr::Call> {
        let func = &*FUNC_RELAX_CALL;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::RelaxExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::expr::Call>
        );
        typed(_0.into(), _1, _2.into(), _3, _4.into())
    }

    static FUNC_RELAX_MAKEPYEXPRVISITOR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.MakePyExprVisitor").expect("missing global function")
    });
    pub fn MakePyExprVisitor(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_RELAX_MAKEPYEXPRVISITOR;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_RELAX_PYEXPRVISITORVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprVisitorVisitExpr").expect("missing global function")
    });
    pub fn PyExprVisitorVisitExpr(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::ir::RelaxExpr,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_PYEXPRVISITORVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRVISITORVISITBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprVisitorVisitBinding").expect("missing global function")
    });
    pub fn PyExprVisitorVisitBinding(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::relax::expr::Binding,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_PYEXPRVISITORVISITBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRVISITORVISITBINDINGBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprVisitorVisitBindingBlock")
            .expect("missing global function")
    });
    pub fn PyExprVisitorVisitBindingBlock(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::relax::expr::BindingBlock,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_PYEXPRVISITORVISITBINDINGBLOCK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRVISITORVISITVARDEF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprVisitorVisitVarDef").expect("missing global function")
    });
    pub fn PyExprVisitorVisitVarDef(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::relax::expr::Var,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_PYEXPRVISITORVISITVARDEF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRVISITORVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprVisitorVisitExpr").expect("missing global function")
    });
    pub fn ExprVisitorVisitExpr(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::ir::RelaxExpr,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXPRVISITORVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRVISITORVISITBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprVisitorVisitBinding").expect("missing global function")
    });
    pub fn ExprVisitorVisitBinding(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::relax::expr::Binding,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXPRVISITORVISITBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRVISITORVISITBINDINGBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprVisitorVisitBindingBlock").expect("missing global function")
    });
    pub fn ExprVisitorVisitBindingBlock(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::relax::expr::BindingBlock,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXPRVISITORVISITBINDINGBLOCK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRVISITORVISITVARDEF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprVisitorVisitVarDef").expect("missing global function")
    });
    pub fn ExprVisitorVisitVarDef(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::relax::expr::Var,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXPRVISITORVISITVARDEF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRVISITORVISITSPAN: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprVisitorVisitSpan").expect("missing global function")
    });
    pub fn ExprVisitorVisitSpan(
        _0: crate::expr_functor::PyExprVisitor,
        _1: crate::ir::Span,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXPRVISITORVISITSPAN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_MAKEPYEXPRMUTATOR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.MakePyExprMutator").expect("missing global function")
    });
    pub fn MakePyExprMutator(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_RELAX_MAKEPYEXPRMUTATOR;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_RELAX_PYEXPRMUTATORVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorVisitExpr").expect("missing global function")
    });
    pub fn PyExprMutatorVisitExpr(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::ir::RelaxExpr,
    ) -> Result<crate::ir::RelaxExpr> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORVISITBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorVisitBinding").expect("missing global function")
    });
    pub fn PyExprMutatorVisitBinding(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::Binding,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORVISITBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORVISITBINDINGBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorVisitBindingBlock")
            .expect("missing global function")
    });
    pub fn PyExprMutatorVisitBindingBlock(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::BindingBlock,
    ) -> Result<crate::relax::expr::BindingBlock> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORVISITBINDINGBLOCK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::BindingBlock>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORVISITVARDEF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorVisitVarDef").expect("missing global function")
    });
    pub fn PyExprMutatorVisitVarDef(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::Var,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORVISITVARDEF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRMUTATORVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprMutatorVisitExpr").expect("missing global function")
    });
    pub fn ExprMutatorVisitExpr(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::ir::RelaxExpr,
    ) -> Result<crate::ir::RelaxExpr> {
        let func = &*FUNC_RELAX_EXPRMUTATORVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRMUTATORVISITBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprMutatorVisitBinding").expect("missing global function")
    });
    pub fn ExprMutatorVisitBinding(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::Binding,
    ) -> Result<()> {
        let func = &*FUNC_RELAX_EXPRMUTATORVISITBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRMUTATORVISITBINDINGBLOCK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprMutatorVisitBindingBlock").expect("missing global function")
    });
    pub fn ExprMutatorVisitBindingBlock(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::BindingBlock,
    ) -> Result<crate::relax::expr::BindingBlock> {
        let func = &*FUNC_RELAX_EXPRMUTATORVISITBINDINGBLOCK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::BindingBlock>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_EXPRMUTATORVISITVARDEF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExprMutatorVisitVarDef").expect("missing global function")
    });
    pub fn ExprMutatorVisitVarDef(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::Var,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_EXPRMUTATORVISITVARDEF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORVISITEXPRPOSTORDER: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorVisitExprPostOrder")
            .expect("missing global function")
    });
    pub fn PyExprMutatorVisitExprPostOrder(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::ir::RelaxExpr,
    ) -> Result<crate::ir::RelaxExpr> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORVISITEXPRPOSTORDER;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORVISITWITHNEWSCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorVisitWithNewScope")
            .expect("missing global function")
    });
    pub fn PyExprMutatorVisitWithNewScope(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::ir::RelaxExpr,
    ) -> Result<crate::ir::RelaxExpr> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORVISITWITHNEWSCOPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORLOOKUPBINDING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorLookupBinding").expect("missing global function")
    });
    pub fn PyExprMutatorLookupBinding(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::Var,
    ) -> Result<Option<crate::ir::RelaxExpr>> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORLOOKUPBINDING;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<crate::ir::RelaxExpr>>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORWITHSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorWithStructInfo").expect("missing global function")
    });
    pub fn PyExprMutatorWithStructInfo(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::expr::Var,
        _2: crate::ir::StructInfo,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORWITHSTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORSETVARREMAP: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorSetVarRemap").expect("missing global function")
    });
    pub fn PyExprMutatorSetVarRemap(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::Id,
        _2: crate::relax::expr::Var,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORSETVARREMAP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_RELAX_PYEXPRMUTATORGETVARREMAP: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PyExprMutatorGetVarRemap").expect("missing global function")
    });
    pub fn PyExprMutatorGetVarRemap(
        _0: crate::expr_functor::PyExprMutator,
        _1: crate::relax::Id,
    ) -> Result<crate::relax::expr::Var> {
        let func = &*FUNC_RELAX_PYEXPRMUTATORGETVARREMAP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_UPDATESTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.UpdateStructInfo").expect("missing global function")
    });
    pub fn UpdateStructInfo(_0: crate::ir::RelaxExpr, _1: crate::ir::StructInfo) -> Result<()> {
        let func = &*FUNC_RELAX_UPDATESTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_FUNCSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FuncStructInfo").expect("missing global function")
    });
    pub fn FuncStructInfo(
        _0: tvm_ffi::Array<crate::ir::StructInfo>,
        _1: crate::ir::StructInfo,
        _2: bool,
        _3: crate::ir::Span,
    ) -> Result<crate::relax::FuncStructInfo> {
        let func = &*FUNC_RELAX_FUNCSTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
                bool,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::FuncStructInfo>
        );
        typed(_0, _1.into(), _2, _3.into())
    }

    static FUNC_RELAX_FUNCSTRUCTINFOOPAQUEFUNC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FuncStructInfoOpaqueFunc").expect("missing global function")
    });
    pub fn FuncStructInfoOpaqueFunc(
        _0: Option<crate::ir::StructInfo>,
        _1: Option<crate::ir::EnvFunc>,
        _2: bool,
        _3: crate::ir::Span,
    ) -> Result<crate::relax::FuncStructInfo> {
        let func = &*FUNC_RELAX_FUNCSTRUCTINFOOPAQUEFUNC;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                Option<crate::ir::StructInfo>,
                Option<crate::ir::EnvFunc>,
                bool,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::FuncStructInfo>
        );
        typed(_0, _1, _2, _3.into())
    }

    static FUNC_RELAX_TUPLESTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.TupleStructInfo").expect("missing global function")
    });
    pub fn TupleStructInfo(
        _0: tvm_ffi::Array<crate::ir::StructInfo>,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::TupleStructInfo> {
        let func = &*FUNC_RELAX_TUPLESTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::StructInfo>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::TupleStructInfo>
        );
        typed(_0, _1.into())
    }

    static FUNC_RELAX_TENSORSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.TensorStructInfo").expect("missing global function")
    });
    pub fn TensorStructInfo(
        _0: Option<crate::ir::RelaxExpr>,
        _1: Option<tvm_ffi::DLDataType>,
        _2: i64,
        _3: crate::ir::VDevice,
        _4: crate::ir::Span,
    ) -> Result<crate::relax::TensorStructInfo> {
        let func = &*FUNC_RELAX_TENSORSTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                Option<crate::ir::RelaxExpr>,
                Option<tvm_ffi::DLDataType>,
                i64,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::TensorStructInfo>
        );
        typed(_0, _1, _2, _3.into(), _4.into())
    }

    static FUNC_RELAX_SHAPESTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ShapeStructInfo").expect("missing global function")
    });
    pub fn ShapeStructInfo(
        _0: Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
        _1: i64,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::ShapeStructInfo> {
        let func = &*FUNC_RELAX_SHAPESTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
                i64,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::relax::ShapeStructInfo>
        );
        typed(_0, _1, _2.into())
    }

    static FUNC_RELAX_PRIMSTRUCTINFOFROMDTYPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PrimStructInfoFromDtype").expect("missing global function")
    });
    pub fn PrimStructInfoFromDtype(
        _0: tvm_ffi::DLDataType,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::PrimStructInfo> {
        let func = &*FUNC_RELAX_PRIMSTRUCTINFOFROMDTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::relax::PrimStructInfo>);
        typed(_0, _1.into())
    }

    static FUNC_RELAX_PRIMSTRUCTINFOFROMVALUE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PrimStructInfoFromValue").expect("missing global function")
    });
    pub fn PrimStructInfoFromValue(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::Span,
    ) -> Result<crate::relax::PrimStructInfo> {
        let func = &*FUNC_RELAX_PRIMSTRUCTINFOFROMVALUE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::PrimStructInfo>);
        typed(_0.into(), _1.into())
    }

    static FUNC_RELAX_OBJECTSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ObjectStructInfo").expect("missing global function")
    });
    pub fn ObjectStructInfo(_0: crate::ir::Span) -> Result<crate::relax::ObjectStructInfo> {
        let func = &*FUNC_RELAX_OBJECTSTRUCTINFO;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::ObjectStructInfo>);
        typed(_0.into())
    }

    static FUNC_RELAX_PACKEDFUNCTYPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.PackedFuncType").expect("missing global function")
    });
    pub fn PackedFuncType(_0: crate::ir::Span) -> Result<crate::relax::PackedFuncType> {
        let func = &*FUNC_RELAX_PACKEDFUNCTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::PackedFuncType>);
        typed(_0.into())
    }

    static FUNC_RELAX_TENSORTYPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.TensorType").expect("missing global function")
    });
    pub fn TensorType(
        _0: i64,
        _1: tvm_ffi::DLDataType,
        _2: crate::ir::Span,
    ) -> Result<crate::relax::DynTensorType> {
        let func = &*FUNC_RELAX_TENSORTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::relax::DynTensorType>);
        typed(_0, _1, _2.into())
    }

    static FUNC_RELAX_OBJECTTYPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ObjectType").expect("missing global function")
    });
    pub fn ObjectType(_0: crate::ir::Span) -> Result<crate::relax::ObjectType> {
        let func = &*FUNC_RELAX_OBJECTTYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::ObjectType>);
        typed(_0.into())
    }

    static FUNC_RELAX_SHAPETYPE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("relax.ShapeType").expect("missing global function"));
    pub fn ShapeType(_0: i64, _1: crate::ir::Span) -> Result<crate::relax::ShapeType> {
        let func = &*FUNC_RELAX_SHAPETYPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, tvm_ffi::object::ObjectRef) -> Result<crate::relax::ShapeType>);
        typed(_0, _1.into())
    }

    static FUNC_RELAX_FUNCTIONBINDPARAMS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FunctionBindParams").expect("missing global function")
    });
    pub fn FunctionBindParams(
        _0: crate::relax::expr::Function,
        _1: tvm_ffi::Map<tvm_ffi::AnyValue, tvm_ffi::object::ObjectRef>,
    ) -> Result<crate::relax::expr::Function> {
        let func = &*FUNC_RELAX_FUNCTIONBINDPARAMS;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<tvm_ffi::AnyValue, tvm_ffi::object::ObjectRef>,
            ) -> Result<crate::relax::expr::Function>
        );
        typed(_0.into(), _1)
    }

    static FUNC_RELAX_FUNCTIONBINDSYMBOLICVARS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FunctionBindSymbolicVars").expect("missing global function")
    });
    pub fn FunctionBindSymbolicVars(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_RELAX_FUNCTIONBINDSYMBOLICVARS;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_RELAX_FUNCTIONINLINEFUNCTIONS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.FunctionInlineFunctions").expect("missing global function")
    });
    pub fn FunctionInlineFunctions(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_RELAX_FUNCTIONINLINEFUNCTIONS;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_RELAX_COPYWITHNEWVARS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.CopyWithNewVars").expect("missing global function")
    });
    pub fn CopyWithNewVars(
        _0: crate::relax::expr::Function,
    ) -> Result<crate::relax::expr::Function> {
        let func = &*FUNC_RELAX_COPYWITHNEWVARS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Function>);
        typed(_0.into())
    }

    static FUNC_RELAX_EXECUTABLELOADFROMFILE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("relax.ExecutableLoadFromFile").expect("missing global function")
    });
    pub fn ExecutableLoadFromFile(_0: tvm_ffi::String) -> Result<tvm_ffi::Module> {
        let func = &*FUNC_RELAX_EXECUTABLELOADFROMFILE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<tvm_ffi::Module>);
        typed(_0)
    }

    pub mod analysis {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_ANALYSIS_FREE_VARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.free_vars").expect("missing global function")
        });
        pub fn free_vars(
            _0: crate::ir::RelaxExpr,
        ) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_FREE_VARS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::relax::expr::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_BOUND_VARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.bound_vars").expect("missing global function")
        });
        pub fn bound_vars(
            _0: crate::ir::RelaxExpr,
        ) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_BOUND_VARS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::relax::expr::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_ALL_VARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.all_vars").expect("missing global function")
        });
        pub fn all_vars(
            _0: crate::ir::RelaxExpr,
        ) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_ALL_VARS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::relax::expr::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_ALL_GLOBAL_VARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.all_global_vars").expect("missing global function")
        });
        pub fn all_global_vars(
            _0: crate::ir::RelaxExpr,
        ) -> Result<tvm_ffi::Array<crate::ir::GlobalVar>> {
            let func = &*FUNC_RELAX_ANALYSIS_ALL_GLOBAL_VARS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::GlobalVar>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_CONTAINS_IMPURE_CALL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.contains_impure_call")
                .expect("missing global function")
        });
        pub fn contains_impure_call(
            _0: crate::ir::RelaxExpr,
            _1: Option<crate::ir::RelaxExpr>,
        ) -> Result<bool> {
            let func = &*FUNC_RELAX_ANALYSIS_CONTAINS_IMPURE_CALL;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, Option<crate::ir::RelaxExpr>) -> Result<bool>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_ANALYSIS_COMPUTABLE_AT_COMPILE_TIME: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.analysis.computable_at_compile_time")
                    .expect("missing global function")
            });
        pub fn computable_at_compile_time(
            _0: crate::relax::expr::Function,
        ) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_COMPUTABLE_AT_COMPILE_TIME;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::relax::expr::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_DETECT_RECURSION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.detect_recursion")
                .expect("missing global function")
        });
        pub fn detect_recursion(
            _0: crate::ir::IRModule,
        ) -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::ir::GlobalVar>>> {
            let func = &*FUNC_RELAX_ANALYSIS_DETECT_RECURSION;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::ir::GlobalVar>>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_SUGGEST_LAYOUT_TRANSFORMS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.analysis.suggest_layout_transforms")
                    .expect("missing global function")
            });
        pub fn suggest_layout_transforms(
            _0: crate::tir::PrimFunc,
            _1: tvm_ffi::Array<crate::tir::IndexMap>,
        ) -> Result<
            tvm_ffi::Map<
                crate::tir::Block,
                tvm_ffi::Map<tvm_ffi::object::ObjectRef, crate::tir::IndexMap>,
            >,
        > {
            let func = &*FUNC_RELAX_ANALYSIS_SUGGEST_LAYOUT_TRANSFORMS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::tir::IndexMap>,
                ) -> Result<
                    tvm_ffi::Map<
                        crate::tir::Block,
                        tvm_ffi::Map<tvm_ffi::object::ObjectRef, crate::tir::IndexMap>,
                    >,
                >
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_ANALYSIS_STRUCTINFOLCA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.StructInfoLCA").expect("missing global function")
        });
        pub fn StructInfoLCA(
            _0: crate::ir::StructInfo,
            _1: crate::ir::StructInfo,
        ) -> Result<crate::ir::StructInfo> {
            let func = &*FUNC_RELAX_ANALYSIS_STRUCTINFOLCA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::StructInfo>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_ANALYSIS_DERIVECALLRETSTRUCTINFO: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.analysis.DeriveCallRetStructInfo")
                    .expect("missing global function")
            });
        pub fn DeriveCallRetStructInfo(
            _0: crate::relax::FuncStructInfo,
            _1: crate::relax::expr::Call,
            _2: crate::relax::BlockBuilder,
        ) -> Result<crate::ir::StructInfo> {
            let func = &*FUNC_RELAX_ANALYSIS_DERIVECALLRETSTRUCTINFO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::StructInfo>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_RELAX_ANALYSIS_STRUCTINFOBASECHECK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.StructInfoBaseCheck")
                .expect("missing global function")
        });
        pub fn StructInfoBaseCheck(
            _0: crate::ir::StructInfo,
            _1: crate::ir::StructInfo,
        ) -> Result<i64> {
            let func = &*FUNC_RELAX_ANALYSIS_STRUCTINFOBASECHECK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<i64>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_ANALYSIS_ERASETOWELLDEFINED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.EraseToWellDefined")
                .expect("missing global function")
        });
        pub fn EraseToWellDefined(
            _0: crate::ir::StructInfo,
            _1: tvm_ffi::Map<crate::tir::Var, crate::ir::PrimExpr>,
            _2: tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>,
        ) -> Result<crate::ir::StructInfo> {
            let func = &*FUNC_RELAX_ANALYSIS_ERASETOWELLDEFINED;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Map<crate::tir::Var, crate::ir::PrimExpr>,
                    tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>,
                ) -> Result<crate::ir::StructInfo>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_ANALYSIS_GETSTATICTYPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.GetStaticType").expect("missing global function")
        });
        pub fn GetStaticType(_0: crate::ir::StructInfo) -> Result<crate::ir::Type> {
            let func = &*FUNC_RELAX_ANALYSIS_GETSTATICTYPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::Type>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_DEFINEDSYMBOLICVARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.DefinedSymbolicVars")
                .expect("missing global function")
        });
        pub fn DefinedSymbolicVars(
            _0: crate::ir::RelaxExpr,
        ) -> Result<tvm_ffi::Array<crate::tir::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_DEFINEDSYMBOLICVARS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_FREESYMBOLICVARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.FreeSymbolicVars")
                .expect("missing global function")
        });
        pub fn FreeSymbolicVars(
            _0: crate::ir::RelaxExpr,
        ) -> Result<tvm_ffi::Array<crate::tir::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_FREESYMBOLICVARS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_TIRVARSINSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.TIRVarsInStructInfo")
                .expect("missing global function")
        });
        pub fn TIRVarsInStructInfo(
            _0: crate::ir::StructInfo,
        ) -> Result<tvm_ffi::Array<crate::tir::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_TIRVARSINSTRUCTINFO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_DEFINABLETIRVARSINSTRUCTINFO: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.analysis.DefinableTIRVarsInStructInfo")
                    .expect("missing global function")
            });
        pub fn DefinableTIRVarsInStructInfo(
            _0: crate::ir::StructInfo,
        ) -> Result<tvm_ffi::Array<crate::tir::Var>> {
            let func = &*FUNC_RELAX_ANALYSIS_DEFINABLETIRVARSINSTRUCTINFO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::Var>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_COLLECTNONNEGATIVEEXPRESSIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.analysis.CollectNonNegativeExpressions")
                    .expect("missing global function")
            });
        pub fn CollectNonNegativeExpressions(
            _0: crate::ir::StructInfo,
        ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            let func = &*FUNC_RELAX_ANALYSIS_COLLECTNONNEGATIVEEXPRESSIONS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_HAS_RESHAPE_PATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.has_reshape_pattern")
                .expect("missing global function")
        });
        pub fn has_reshape_pattern(_0: crate::tir::PrimFunc) -> Result<bool> {
            let func = &*FUNC_RELAX_ANALYSIS_HAS_RESHAPE_PATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_UDCHAIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.udchain").expect("missing global function")
        });
        pub fn udchain(
            _0: crate::relax::expr::DataflowBlock,
        ) -> Result<tvm_ffi::Map<crate::relax::expr::Var, tvm_ffi::Array<crate::relax::expr::Var>>>
        {
            let func = &*FUNC_RELAX_ANALYSIS_UDCHAIN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<crate::relax::expr::Var, tvm_ffi::Array<crate::relax::expr::Var>>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_GET_VAR2VAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.get_var2val").expect("missing global function")
        });
        pub fn get_var2val(
            _0: crate::relax::expr::Function,
        ) -> Result<tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>> {
            let func = &*FUNC_RELAX_ANALYSIS_GET_VAR2VAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_NAME_TO_BINDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.name_to_binding").expect("missing global function")
        });
        pub fn name_to_binding(
            _0: crate::relax::expr::Function,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::relax::expr::Binding>>>
        {
            let func = &*FUNC_RELAX_ANALYSIS_NAME_TO_BINDING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::relax::expr::Binding>>>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_WELL_FORMED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.well_formed").expect("missing global function")
        });
        pub fn well_formed(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_ANALYSIS_WELL_FORMED;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_ANALYSIS_REMOVE_ALL_UNUSED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.remove_all_unused")
                .expect("missing global function")
        });
        pub fn remove_all_unused(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_ANALYSIS_REMOVE_ALL_UNUSED;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_ANALYSIS_POST_ORDER_VISIT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.analysis.post_order_visit")
                .expect("missing global function")
        });
        pub fn post_order_visit(_0: crate::ir::RelaxExpr, _1: tvm_ffi::Function) -> Result<()> {
            let func = &*FUNC_RELAX_ANALYSIS_POST_ORDER_VISIT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function) -> Result<()>);
            typed(_0.into(), _1)
        }
    }
    pub mod backend {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_BACKEND_REGISTERPATTERNS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.backend.RegisterPatterns").expect("missing global function")
        });
        pub fn RegisterPatterns(
            _0: tvm_ffi::Array<crate::relax::transform::FusionPattern>,
        ) -> Result<()> {
            let func = &*FUNC_RELAX_BACKEND_REGISTERPATTERNS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::Array<crate::relax::transform::FusionPattern>) -> Result<()>
            );
            typed(_0)
        }

        static FUNC_RELAX_BACKEND_REMOVEPATTERNS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.backend.RemovePatterns").expect("missing global function")
        });
        pub fn RemovePatterns(_0: tvm_ffi::Array<tvm_ffi::String>) -> Result<()> {
            let func = &*FUNC_RELAX_BACKEND_REMOVEPATTERNS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::Array<tvm_ffi::String>) -> Result<()>
            );
            typed(_0)
        }

        static FUNC_RELAX_BACKEND_GETPATTERNSWITHPREFIX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.backend.GetPatternsWithPrefix")
                .expect("missing global function")
        });
        pub fn GetPatternsWithPrefix(
            _0: tvm_ffi::String,
        ) -> Result<tvm_ffi::Array<crate::relax::transform::FusionPattern>> {
            let func = &*FUNC_RELAX_BACKEND_GETPATTERNSWITHPREFIX;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<tvm_ffi::Array<crate::relax::transform::FusionPattern>>);
            typed(_0)
        }

        static FUNC_RELAX_BACKEND_GETPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.backend.GetPattern").expect("missing global function")
        });
        pub fn GetPattern(
            _0: tvm_ffi::String,
        ) -> Result<Option<crate::relax::transform::FusionPattern>> {
            let func = &*FUNC_RELAX_BACKEND_GETPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<Option<crate::relax::transform::FusionPattern>>);
            typed(_0)
        }

        static FUNC_RELAX_BACKEND_METASCHEDULEEXTRACTTASK: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.backend.MetaScheduleExtractTask")
                    .expect("missing global function")
            });
        pub fn MetaScheduleExtractTask(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
            _2: tvm_ffi::String,
        ) -> Result<tvm_ffi::Array<crate::meta_schedule::ExtractedTask>> {
            let func = &*FUNC_RELAX_BACKEND_METASCHEDULEEXTRACTTASK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<tvm_ffi::Array<crate::meta_schedule::ExtractedTask>>);
            typed(_0.into(), _1.into(), _2)
        }

        pub mod adreno {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            pub mod transform {
                use std::sync::LazyLock;
                use tvm_ffi::{Any, AnyView, Function, Result};

                static FUNC_RELAX_BACKEND_ADRENO_TRANSFORM_ANNOTATECUSTOMMEMORYSCOPE: LazyLock<
                    Function,
                > = LazyLock::new(|| {
                    Function::get_global("relax.backend.adreno.transform.AnnotateCustomMemoryScope")
                        .expect("missing global function")
                });
                pub fn AnnotateCustomMemoryScope(
                    _0: crate::target::Target,
                ) -> Result<crate::transform::Pass> {
                    let func = &*FUNC_RELAX_BACKEND_ADRENO_TRANSFORM_ANNOTATECUSTOMMEMORYSCOPE;
                    let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::transform::Pass>);
                    typed(_0.into())
                }

                static FUNC_RELAX_BACKEND_ADRENO_TRANSFORM_FOLDVDEVICESCOPECHANGE: LazyLock<
                    Function,
                > = LazyLock::new(|| {
                    Function::get_global("relax.backend.adreno.transform.FoldVDeviceScopeChange")
                        .expect("missing global function")
                });
                pub fn FoldVDeviceScopeChange() -> Result<crate::transform::Pass> {
                    let func = &*FUNC_RELAX_BACKEND_ADRENO_TRANSFORM_FOLDVDEVICESCOPECHANGE;
                    let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
                    typed()
                }
            }
        }
    }
    pub mod contrib {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_CONTRIB_EXTRACT_ARG_IDX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.contrib.extract_arg_idx").expect("missing global function")
        });
        pub fn extract_arg_idx(
            _0: tvm_ffi::String,
            _1: crate::relax::expr::Function,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, crate::ir::IntImm>> {
            let func = &*FUNC_RELAX_CONTRIB_EXTRACT_ARG_IDX;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<tvm_ffi::String, crate::ir::IntImm>>);
            typed(_0, _1.into())
        }
    }
    pub mod distributed {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_DISTRIBUTED_DEVICEMESH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.distributed.DeviceMesh").expect("missing global function")
        });
        pub fn DeviceMesh(
            _0: tvm_ffi::Shape,
            _1: tvm_ffi::Array<crate::ir::IntImm>,
            _2: Option<crate::ir::Range>,
        ) -> Result<crate::relax::distributed::DeviceMesh> {
            let func = &*FUNC_RELAX_DISTRIBUTED_DEVICEMESH;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Shape,
                    tvm_ffi::Array<crate::ir::IntImm>,
                    Option<crate::ir::Range>,
                ) -> Result<crate::relax::distributed::DeviceMesh>
            );
            typed(_0, _1, _2)
        }

        static FUNC_RELAX_DISTRIBUTED_DTENSORSTRUCTINFO: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.distributed.DTensorStructInfo")
                .expect("missing global function")
        });
        pub fn DTensorStructInfo(
            _0: crate::relax::TensorStructInfo,
            _1: crate::relax::distributed::DeviceMesh,
            _2: crate::relax::distributed::Placement,
            _3: crate::ir::Span,
        ) -> Result<crate::relax::DTensorStructInfo> {
            let func = &*FUNC_RELAX_DISTRIBUTED_DTENSORSTRUCTINFO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::DTensorStructInfo>);
            typed(_0.into(), _1.into(), _2.into(), _3.into())
        }

        static FUNC_RELAX_DISTRIBUTED_SHARDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.distributed.Sharding").expect("missing global function")
        });
        pub fn Sharding(_0: i64) -> Result<crate::relax::distributed::PlacementSpec> {
            let func = &*FUNC_RELAX_DISTRIBUTED_SHARDING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64) -> Result<crate::relax::distributed::PlacementSpec>);
            typed(_0)
        }

        static FUNC_RELAX_DISTRIBUTED_REPLICA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.distributed.Replica").expect("missing global function")
        });
        pub fn Replica() -> Result<crate::relax::distributed::PlacementSpec> {
            let func = &*FUNC_RELAX_DISTRIBUTED_REPLICA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::relax::distributed::PlacementSpec>);
            typed()
        }

        static FUNC_RELAX_DISTRIBUTED_PLACEMENTFROMTEXT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.distributed.PlacementFromText")
                .expect("missing global function")
        });
        pub fn PlacementFromText(
            _0: tvm_ffi::String,
        ) -> Result<crate::relax::distributed::Placement> {
            let func = &*FUNC_RELAX_DISTRIBUTED_PLACEMENTFROMTEXT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::relax::distributed::Placement>);
            typed(_0)
        }

        static FUNC_RELAX_DISTRIBUTED_PLACEMENT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.distributed.Placement").expect("missing global function")
        });
        pub fn Placement(
            _0: tvm_ffi::Array<crate::relax::distributed::PlacementSpec>,
        ) -> Result<crate::relax::distributed::Placement> {
            let func = &*FUNC_RELAX_DISTRIBUTED_PLACEMENT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::distributed::PlacementSpec>,
                ) -> Result<crate::relax::distributed::Placement>
            );
            typed(_0)
        }

        pub mod transform {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_DISTRIBUTED_TRANSFORM_LEGALIZEREDISTRIBUTE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.distributed.transform.LegalizeRedistribute")
                        .expect("missing global function")
                });
            pub fn LegalizeRedistribute() -> Result<crate::transform::Pass> {
                let func = &*FUNC_RELAX_DISTRIBUTED_TRANSFORM_LEGALIZEREDISTRIBUTE;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
                typed()
            }

            static FUNC_RELAX_DISTRIBUTED_TRANSFORM_LOWERDISTIR: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.distributed.transform.LowerDistIR")
                        .expect("missing global function")
                });
            pub fn LowerDistIR() -> Result<crate::transform::Pass> {
                let func = &*FUNC_RELAX_DISTRIBUTED_TRANSFORM_LOWERDISTIR;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
                typed()
            }

            static FUNC_RELAX_DISTRIBUTED_TRANSFORM_LOWERGLOBALVIEWTOLOCALVIEW: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.distributed.transform.LowerGlobalViewToLocalView")
                        .expect("missing global function")
                });
            pub fn LowerGlobalViewToLocalView() -> Result<crate::transform::Pass> {
                let func = &*FUNC_RELAX_DISTRIBUTED_TRANSFORM_LOWERGLOBALVIEWTOLOCALVIEW;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
                typed()
            }

            static FUNC_RELAX_DISTRIBUTED_TRANSFORM_PROPAGATESHARDING: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.distributed.transform.PropagateSharding")
                        .expect("missing global function")
                });
            pub fn PropagateSharding() -> Result<crate::transform::Pass> {
                let func = &*FUNC_RELAX_DISTRIBUTED_TRANSFORM_PROPAGATESHARDING;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
                typed()
            }
        }
    }
    pub mod dpl {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_DPL_MATCH_DFB: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.match_dfb").expect("missing global function")
        });
        pub fn match_dfb(
            _0: tvm_ffi::object::ObjectRef,
            _1: crate::relax::expr::DataflowBlock,
        ) -> Result<Option<tvm_ffi::Map<crate::relax::dpl::DFPattern, crate::relax::expr::Var>>>
        {
            let func = &*FUNC_RELAX_DPL_MATCH_DFB;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<tvm_ffi::Map<crate::relax::dpl::DFPattern, crate::relax::expr::Var>>>);
            typed(_0, _1.into())
        }

        static FUNC_RELAX_DPL_REWRITE_BINDINGS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.rewrite_bindings").expect("missing global function")
        });
        pub fn rewrite_bindings(
            _0: tvm_ffi::object::ObjectRef,
            _1: tvm_ffi::Function,
            _2: crate::relax::expr::Function,
        ) -> Result<crate::relax::expr::Function> {
            let func = &*FUNC_RELAX_DPL_REWRITE_BINDINGS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Function>);
            typed(_0, _1, _2.into())
        }

        static FUNC_RELAX_DPL_TUPLEREWRITER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.TupleRewriter").expect("missing global function")
        });
        pub fn TupleRewriter(
            _0: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _1: tvm_ffi::Function,
        ) -> Result<crate::relax::dpl::TupleRewriter> {
            let func = &*FUNC_RELAX_DPL_TUPLEREWRITER;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                    tvm_ffi::Function,
                ) -> Result<crate::relax::dpl::TupleRewriter>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_DPL_ORREWRITER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.OrRewriter").expect("missing global function")
        });
        pub fn OrRewriter(
            _0: crate::relax::dpl::PatternMatchingRewriter,
            _1: crate::relax::dpl::PatternMatchingRewriter,
        ) -> Result<crate::relax::dpl::OrRewriter> {
            let func = &*FUNC_RELAX_DPL_ORREWRITER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::OrRewriter>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_DPL_PATTERNREWRITER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.PatternRewriter").expect("missing global function")
        });
        pub fn PatternRewriter(
            _0: crate::relax::dpl::DFPattern,
            _1: tvm_ffi::Function,
        ) -> Result<crate::relax::dpl::ExprPatternRewriter> {
            let func = &*FUNC_RELAX_DPL_PATTERNREWRITER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function) -> Result<crate::relax::dpl::ExprPatternRewriter>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_DPL_PATTERNMATCHINGREWRITERFROMPATTERN: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.dpl.PatternMatchingRewriterFromPattern")
                    .expect("missing global function")
            });
        pub fn PatternMatchingRewriterFromPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: tvm_ffi::Function,
        ) -> Result<crate::relax::dpl::PatternMatchingRewriter> {
            let func = &*FUNC_RELAX_DPL_PATTERNMATCHINGREWRITERFROMPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function) -> Result<crate::relax::dpl::PatternMatchingRewriter>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_DPL_PATTERNMATCHINGREWRITERFROMMODULE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.dpl.PatternMatchingRewriterFromModule")
                    .expect("missing global function")
            });
        pub fn PatternMatchingRewriterFromModule(
            _0: crate::ir::IRModule,
        ) -> Result<crate::relax::dpl::PatternMatchingRewriter> {
            let func = &*FUNC_RELAX_DPL_PATTERNMATCHINGREWRITERFROMMODULE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::PatternMatchingRewriter>);
            typed(_0.into())
        }

        static FUNC_RELAX_DPL_PATTERNMATCHINGREWRITERAPPLY: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.dpl.PatternMatchingRewriterApply")
                    .expect("missing global function")
            });
        pub fn PatternMatchingRewriterApply(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_DPL_PATTERNMATCHINGREWRITERAPPLY;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_DPL_REWRITE_CALL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.rewrite_call").expect("missing global function")
        });
        pub fn rewrite_call(
            _0: crate::relax::dpl::DFPattern,
            _1: tvm_ffi::Function,
            _2: crate::relax::expr::Function,
        ) -> Result<crate::relax::expr::Function> {
            let func = &*FUNC_RELAX_DPL_REWRITE_CALL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Function>);
            typed(_0.into(), _1, _2.into())
        }

        static FUNC_RELAX_DPL_MATCH_EXPR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.match_expr").expect("missing global function")
        });
        pub fn match_expr(
            _0: crate::relax::dpl::DFPattern,
            _1: crate::ir::RelaxExpr,
            _2: Option<tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>>,
        ) -> Result<bool> {
            let func = &*FUNC_RELAX_DPL_MATCH_EXPR;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>>,
                ) -> Result<bool>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_DPL_EXTRACT_MATCHED_EXPR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.extract_matched_expr").expect("missing global function")
        });
        pub fn extract_matched_expr(
            _0: crate::relax::dpl::DFPattern,
            _1: crate::ir::RelaxExpr,
            _2: Option<tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>>,
        ) -> Result<Option<tvm_ffi::Map<crate::relax::dpl::DFPattern, crate::ir::RelaxExpr>>>
        {
            let func = &*FUNC_RELAX_DPL_EXTRACT_MATCHED_EXPR;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>>,
                ) -> Result<
                    Option<tvm_ffi::Map<crate::relax::dpl::DFPattern, crate::ir::RelaxExpr>>,
                >
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_DPL_DUP_PATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.dup_pattern").expect("missing global function")
        });
        pub fn dup_pattern(
            _0: crate::relax::dpl::DFPattern,
        ) -> Result<crate::relax::dpl::DFPattern> {
            let func = &*FUNC_RELAX_DPL_DUP_PATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::DFPattern>);
            typed(_0.into())
        }

        static FUNC_RELAX_DPL_DUP_SEQ: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.dup_seq").expect("missing global function")
        });
        pub fn dup_seq(_0: crate::relax::dpl::PatternSeq) -> Result<crate::relax::dpl::PatternSeq> {
            let func = &*FUNC_RELAX_DPL_DUP_SEQ;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::PatternSeq>);
            typed(_0.into())
        }

        static FUNC_RELAX_DPL_PATTERNCONTEXT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.PatternContext").expect("missing global function")
        });
        pub fn PatternContext(_0: bool) -> Result<tvm_ffi::object::ObjectRef> {
            let func = &*FUNC_RELAX_DPL_PATTERNCONTEXT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<tvm_ffi::object::ObjectRef>);
            typed(_0)
        }

        static FUNC_RELAX_DPL_CURRENT_CONTEXT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.current_context").expect("missing global function")
        });
        pub fn current_context() -> Result<Option<tvm_ffi::object::ObjectRef>> {
            let func = &*FUNC_RELAX_DPL_CURRENT_CONTEXT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<Option<tvm_ffi::object::ObjectRef>>);
            typed()
        }

        static FUNC_RELAX_DPL_ENTER_CONTEXT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.enter_context").expect("missing global function")
        });
        pub fn enter_context(_0: tvm_ffi::object::ObjectRef) -> Result<()> {
            let func = &*FUNC_RELAX_DPL_ENTER_CONTEXT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0)
        }

        static FUNC_RELAX_DPL_EXIT_CONTEXT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.exit_context").expect("missing global function")
        });
        pub fn exit_context(_0: tvm_ffi::object::ObjectRef) -> Result<()> {
            let func = &*FUNC_RELAX_DPL_EXIT_CONTEXT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0)
        }

        static FUNC_RELAX_DPL_USED_BY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.used_by").expect("missing global function")
        });
        pub fn used_by(
            _0: crate::relax::dpl::PatternSeq,
            _1: crate::relax::dpl::PatternSeq,
            _2: i64,
        ) -> Result<crate::relax::dpl::PatternSeq> {
            let func = &*FUNC_RELAX_DPL_USED_BY;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<crate::relax::dpl::PatternSeq>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_DPL_ONLY_USED_BY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.only_used_by").expect("missing global function")
        });
        pub fn only_used_by(
            _0: crate::relax::dpl::PatternSeq,
            _1: crate::relax::dpl::PatternSeq,
            _2: i64,
        ) -> Result<crate::relax::dpl::PatternSeq> {
            let func = &*FUNC_RELAX_DPL_ONLY_USED_BY;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<crate::relax::dpl::PatternSeq>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_DPL_PATTERNSEQ: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.PatternSeq").expect("missing global function")
        });
        pub fn PatternSeq(
            _0: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _1: bool,
        ) -> Result<crate::relax::dpl::PatternSeq> {
            let func = &*FUNC_RELAX_DPL_PATTERNSEQ;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                    bool,
                ) -> Result<crate::relax::dpl::PatternSeq>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_DPL_ATTRPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.AttrPattern").expect("missing global function")
        });
        pub fn AttrPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: crate::ir::DictAttrs,
        ) -> Result<crate::relax::dpl::AttrPattern> {
            let func = &*FUNC_RELAX_DPL_ATTRPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::AttrPattern>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_DPL_DATATYPEPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.DataTypePattern").expect("missing global function")
        });
        pub fn DataTypePattern(
            _0: crate::relax::dpl::DFPattern,
            _1: tvm_ffi::DLDataType,
        ) -> Result<crate::relax::dpl::DataTypePattern> {
            let func = &*FUNC_RELAX_DPL_DATATYPEPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::relax::dpl::DataTypePattern>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_DPL_SAMESHAPECONSTRAINT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.SameShapeConstraint").expect("missing global function")
        });
        pub fn SameShapeConstraint(
            _0: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
        ) -> Result<crate::relax::dpl::SameShapeConstraint> {
            let func = &*FUNC_RELAX_DPL_SAMESHAPECONSTRAINT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                ) -> Result<crate::relax::dpl::SameShapeConstraint>
            );
            typed(_0)
        }

        static FUNC_RELAX_DPL_SHAPEPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.ShapePattern").expect("missing global function")
        });
        pub fn ShapePattern(
            _0: crate::relax::dpl::DFPattern,
            _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        ) -> Result<crate::relax::dpl::ShapePattern> {
            let func = &*FUNC_RELAX_DPL_SHAPEPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::PrimExpr>,
                ) -> Result<crate::relax::dpl::ShapePattern>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_DPL_STRUCTINFOPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.StructInfoPattern").expect("missing global function")
        });
        pub fn StructInfoPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: crate::ir::StructInfo,
        ) -> Result<crate::relax::dpl::StructInfoPattern> {
            let func = &*FUNC_RELAX_DPL_STRUCTINFOPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::StructInfoPattern>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_DPL_WILDCARDPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.WildcardPattern").expect("missing global function")
        });
        pub fn WildcardPattern() -> Result<crate::relax::dpl::WildcardPattern> {
            let func = &*FUNC_RELAX_DPL_WILDCARDPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::relax::dpl::WildcardPattern>);
            typed()
        }

        static FUNC_RELAX_DPL_NOTPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.NotPattern").expect("missing global function")
        });
        pub fn NotPattern(
            _0: crate::relax::dpl::DFPattern,
        ) -> Result<crate::relax::dpl::NotPattern> {
            let func = &*FUNC_RELAX_DPL_NOTPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::NotPattern>);
            typed(_0.into())
        }

        static FUNC_RELAX_DPL_ORPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.OrPattern").expect("missing global function")
        });
        pub fn OrPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: crate::relax::dpl::DFPattern,
        ) -> Result<crate::relax::dpl::OrPattern> {
            let func = &*FUNC_RELAX_DPL_ORPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::OrPattern>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_DPL_ANDPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.AndPattern").expect("missing global function")
        });
        pub fn AndPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: crate::relax::dpl::DFPattern,
        ) -> Result<crate::relax::dpl::AndPattern> {
            let func = &*FUNC_RELAX_DPL_ANDPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::AndPattern>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_DPL_TUPLEGETITEMPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.TupleGetItemPattern").expect("missing global function")
        });
        pub fn TupleGetItemPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: i64,
        ) -> Result<crate::relax::dpl::TupleGetItemPattern> {
            let func = &*FUNC_RELAX_DPL_TUPLEGETITEMPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::relax::dpl::TupleGetItemPattern>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_DPL_UNORDEREDTUPLEPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.UnorderedTuplePattern")
                .expect("missing global function")
        });
        pub fn UnorderedTuplePattern(
            _0: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
        ) -> Result<crate::relax::dpl::UnorderedTuplePattern> {
            let func = &*FUNC_RELAX_DPL_UNORDEREDTUPLEPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                ) -> Result<crate::relax::dpl::UnorderedTuplePattern>
            );
            typed(_0)
        }

        static FUNC_RELAX_DPL_TUPLEPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.TuplePattern").expect("missing global function")
        });
        pub fn TuplePattern(
            _0: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
        ) -> Result<crate::relax::dpl::TuplePattern> {
            let func = &*FUNC_RELAX_DPL_TUPLEPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                ) -> Result<crate::relax::dpl::TuplePattern>
            );
            typed(_0)
        }

        static FUNC_RELAX_DPL_FUNCTIONPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.FunctionPattern").expect("missing global function")
        });
        pub fn FunctionPattern(
            _0: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _1: crate::relax::dpl::DFPattern,
        ) -> Result<crate::relax::dpl::FunctionPattern> {
            let func = &*FUNC_RELAX_DPL_FUNCTIONPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<crate::relax::dpl::FunctionPattern>
            );
            typed(_0, _1.into())
        }

        static FUNC_RELAX_DPL_PRIMARRPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.PrimArrPattern").expect("missing global function")
        });
        pub fn PrimArrPattern(
            _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        ) -> Result<crate::relax::dpl::PrimArrPattern> {
            let func = &*FUNC_RELAX_DPL_PRIMARRPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::ir::PrimExpr>,
                ) -> Result<crate::relax::dpl::PrimArrPattern>
            );
            typed(_0)
        }

        static FUNC_RELAX_DPL_CALLPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.CallPattern").expect("missing global function")
        });
        pub fn CallPattern(
            _0: crate::relax::dpl::DFPattern,
            _1: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _2: bool,
        ) -> Result<crate::relax::dpl::CallPattern> {
            let func = &*FUNC_RELAX_DPL_CALLPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::relax::dpl::DFPattern>,
                    bool,
                ) -> Result<crate::relax::dpl::CallPattern>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_DPL_CONSTANTPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.ConstantPattern").expect("missing global function")
        });
        pub fn ConstantPattern() -> Result<crate::relax::dpl::ConstantPattern> {
            let func = &*FUNC_RELAX_DPL_CONSTANTPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::relax::dpl::ConstantPattern>);
            typed()
        }

        static FUNC_RELAX_DPL_EXPRPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.ExprPattern").expect("missing global function")
        });
        pub fn ExprPattern(_0: crate::ir::RelaxExpr) -> Result<crate::relax::dpl::ExprPattern> {
            let func = &*FUNC_RELAX_DPL_EXPRPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::dpl::ExprPattern>);
            typed(_0.into())
        }

        static FUNC_RELAX_DPL_GLOBALVARPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.GlobalVarPattern").expect("missing global function")
        });
        pub fn GlobalVarPattern(
            _0: tvm_ffi::String,
        ) -> Result<crate::relax::dpl::GlobalVarPattern> {
            let func = &*FUNC_RELAX_DPL_GLOBALVARPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::relax::dpl::GlobalVarPattern>);
            typed(_0)
        }

        static FUNC_RELAX_DPL_DATAFLOWVARPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.DataflowVarPattern").expect("missing global function")
        });
        pub fn DataflowVarPattern(
            _0: tvm_ffi::String,
        ) -> Result<crate::relax::dpl::DataflowVarPattern> {
            let func = &*FUNC_RELAX_DPL_DATAFLOWVARPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::relax::dpl::DataflowVarPattern>);
            typed(_0)
        }

        static FUNC_RELAX_DPL_VARPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.VarPattern").expect("missing global function")
        });
        pub fn VarPattern(_0: tvm_ffi::String) -> Result<crate::relax::dpl::VarPattern> {
            let func = &*FUNC_RELAX_DPL_VARPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::relax::dpl::VarPattern>);
            typed(_0)
        }

        static FUNC_RELAX_DPL_EXTERNFUNCPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.dpl.ExternFuncPattern").expect("missing global function")
        });
        pub fn ExternFuncPattern(
            _0: tvm_ffi::String,
        ) -> Result<crate::relax::dpl::ExternFuncPattern> {
            let func = &*FUNC_RELAX_DPL_EXTERNFUNCPATTERN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::relax::dpl::ExternFuncPattern>);
            typed(_0)
        }
    }
    pub mod op {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_OP_HINT_ON_DEVICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.hint_on_device").expect("missing global function")
        });
        pub fn hint_on_device(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_OP_HINT_ON_DEVICE;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_OP_TO_VDEVICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.to_vdevice").expect("missing global function")
        });
        pub fn to_vdevice(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::VDevice,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TO_VDEVICE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_SHAPE_TO_TENSOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.shape_to_tensor").expect("missing global function")
        });
        pub fn shape_to_tensor(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SHAPE_TO_TENSOR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_TENSOR_TO_SHAPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.tensor_to_shape").expect("missing global function")
        });
        pub fn tensor_to_shape(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TENSOR_TO_SHAPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SHAPE_OF: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.shape_of").expect("missing global function")
        });
        pub fn shape_of(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SHAPE_OF;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_MAKE_CLOSURE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.make_closure").expect("missing global function")
        });
        pub fn make_closure(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MAKE_CLOSURE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_ASSERT_OP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.assert_op").expect("missing global function")
        });
        pub fn assert_op(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::Array<crate::ir::RelaxExpr>,
            _2: crate::relax::expr::StringImm,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ASSERT_OP;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::RelaxExpr>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2.into())
        }

        static FUNC_RELAX_OP_PRINT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.print").expect("missing global function")
        });
        pub fn print(
            _0: tvm_ffi::Array<crate::ir::RelaxExpr>,
            _1: crate::relax::expr::StringImm,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_PRINT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::ir::RelaxExpr>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0, _1.into())
        }

        static FUNC_RELAX_OP_NULL_VALUE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.null_value").expect("missing global function")
        });
        pub fn null_value() -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_NULL_VALUE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::ir::RelaxExpr>);
            typed()
        }

        static FUNC_RELAX_OP_INVOKE_PURE_CLOSURE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.invoke_pure_closure").expect("missing global function")
        });
        pub fn invoke_pure_closure(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::ir::StructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_INVOKE_PURE_CLOSURE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::StructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_INVOKE_CLOSURE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.invoke_closure").expect("missing global function")
        });
        pub fn invoke_closure(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::ir::StructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_INVOKE_CLOSURE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::StructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_CALL_BUILTIN_WITH_CTX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_builtin_with_ctx").expect("missing global function")
        });
        pub fn call_builtin_with_ctx(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::ir::StructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_BUILTIN_WITH_CTX;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::StructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_CALL_PY_FUNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_py_func").expect("missing global function")
        });
        pub fn call_py_func(
            _0: crate::relax::expr::StringImm,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::relax::TensorStructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_PY_FUNC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::relax::TensorStructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_CALL_DPS_PACKED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_dps_packed").expect("missing global function")
        });
        pub fn call_dps_packed(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::relax::TensorStructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_DPS_PACKED;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::relax::TensorStructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_CALL_TIR_INPLACE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_tir_inplace").expect("missing global function")
        });
        pub fn call_tir_inplace(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::ir::IntImm>,
            _3: tvm_ffi::Array<crate::relax::TensorStructInfo>,
            _4: Option<crate::ir::RelaxExpr>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_TIR_INPLACE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::IntImm>,
                    tvm_ffi::Array<crate::relax::TensorStructInfo>,
                    Option<crate::ir::RelaxExpr>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_RELAX_OP_CALL_TIR_WITH_GRAD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_tir_with_grad").expect("missing global function")
        });
        pub fn call_tir_with_grad(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::relax::TensorStructInfo>,
            _3: tvm_ffi::String,
            _4: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            _5: Option<crate::ir::RelaxExpr>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_TIR_WITH_GRAD;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::relax::TensorStructInfo>,
                    tvm_ffi::String,
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                    Option<crate::ir::RelaxExpr>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2, _3, _4, _5)
        }

        static FUNC_RELAX_OP_CALL_TIR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_tir").expect("missing global function")
        });
        pub fn call_tir(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::Tuple,
            _2: tvm_ffi::Array<crate::relax::TensorStructInfo>,
            _3: Option<crate::ir::RelaxExpr>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_TIR;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::relax::TensorStructInfo>,
                    Option<crate::ir::RelaxExpr>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_RELAX_OP_CALL_INPLACE_PACKED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_inplace_packed").expect("missing global function")
        });
        pub fn call_inplace_packed(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::Array<crate::ir::RelaxExpr>,
            _2: tvm_ffi::Array<crate::ir::IntImm>,
            _3: tvm_ffi::Array<crate::ir::StructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_INPLACE_PACKED;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::RelaxExpr>,
                    tvm_ffi::Array<crate::ir::IntImm>,
                    tvm_ffi::Array<crate::ir::StructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_RELAX_OP_CALL_PURE_PACKED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.call_pure_packed").expect("missing global function")
        });
        pub fn call_pure_packed(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::Array<crate::ir::RelaxExpr>,
            _2: crate::ir::Attrs,
            _3: tvm_ffi::Array<crate::ir::StructInfo>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CALL_PURE_PACKED;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::RelaxExpr>,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::StructInfo>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2.into(), _3)
        }

        static FUNC_RELAX_OP_RIGHT_SHIFT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.right_shift").expect("missing global function")
        });
        pub fn right_shift(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_RIGHT_SHIFT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LEFT_SHIFT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.left_shift").expect("missing global function")
        });
        pub fn left_shift(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LEFT_SHIFT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_BITWISE_XOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.bitwise_xor").expect("missing global function")
        });
        pub fn bitwise_xor(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_BITWISE_XOR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_BITWISE_OR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.bitwise_or").expect("missing global function")
        });
        pub fn bitwise_or(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_BITWISE_OR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_BITWISE_AND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.bitwise_and").expect("missing global function")
        });
        pub fn bitwise_and(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_BITWISE_AND;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LOGICAL_XOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.logical_xor").expect("missing global function")
        });
        pub fn logical_xor(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LOGICAL_XOR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LOGICAL_OR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.logical_or").expect("missing global function")
        });
        pub fn logical_or(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LOGICAL_OR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LOGICAL_AND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.logical_and").expect("missing global function")
        });
        pub fn logical_and(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LOGICAL_AND;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_MAXIMUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.maximum").expect("missing global function")
        });
        pub fn maximum(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MAXIMUM;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_MINIMUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.minimum").expect("missing global function")
        });
        pub fn minimum(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MINIMUM;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_NOT_EQUAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.not_equal").expect("missing global function")
        });
        pub fn not_equal(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_NOT_EQUAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LESS_EQUAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.less_equal").expect("missing global function")
        });
        pub fn less_equal(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LESS_EQUAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LESS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.less").expect("missing global function")
        });
        pub fn less(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LESS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_GREATER_EQUAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.greater_equal").expect("missing global function")
        });
        pub fn greater_equal(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_GREATER_EQUAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_GREATER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.greater").expect("missing global function")
        });
        pub fn greater(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_GREATER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_EQUAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.equal").expect("missing global function")
        });
        pub fn equal(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EQUAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_FLOOR_MOD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.floor_mod").expect("missing global function")
        });
        pub fn floor_mod(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_FLOOR_MOD;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_MOD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.mod").expect("missing global function")
        });
        pub fn mod_(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MOD;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_SUBTRACT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.subtract").expect("missing global function")
        });
        pub fn subtract(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SUBTRACT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_POWER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.power").expect("missing global function")
        });
        pub fn power(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_POWER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_MULTIPLY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.multiply").expect("missing global function")
        });
        pub fn multiply(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MULTIPLY;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_LOG_ADD_EXP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.log_add_exp").expect("missing global function")
        });
        pub fn log_add_exp(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LOG_ADD_EXP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_FLOOR_DIVIDE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.floor_divide").expect("missing global function")
        });
        pub fn floor_divide(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_FLOOR_DIVIDE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_DIVIDE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.divide").expect("missing global function")
        });
        pub fn divide(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_DIVIDE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_ADD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.add").expect("missing global function")
        });
        pub fn add(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ADD;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_TRIL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.tril").expect("missing global function")
        });
        pub fn tril(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TRIL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_TRIU: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.triu").expect("missing global function")
        });
        pub fn triu(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TRIU;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_ARANGE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.arange").expect("missing global function")
        });
        pub fn arange(
            _0: crate::relax::expr::PrimValue,
            _1: crate::relax::expr::PrimValue,
            _2: crate::relax::expr::PrimValue,
            _3: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ARANGE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_RELAX_OP_HAMMING_WINDOW: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.hamming_window").expect("missing global function")
        });
        pub fn hamming_window(
            _0: crate::relax::expr::PrimValue,
            _1: crate::relax::expr::PrimValue,
            _2: crate::relax::expr::PrimValue,
            _3: crate::relax::expr::PrimValue,
            _4: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_HAMMING_WINDOW;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3.into(), _4)
        }

        static FUNC_RELAX_OP_EYE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.eye").expect("missing global function")
        });
        pub fn eye(
            _0: crate::relax::expr::PrimValue,
            _1: crate::relax::expr::PrimValue,
            _2: crate::relax::expr::PrimValue,
            _3: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EYE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_RELAX_OP_EYE_LIKE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.eye_like").expect("missing global function")
        });
        pub fn eye_like(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::PrimValue,
            _2: Option<tvm_ffi::DLDataType>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EYE_LIKE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::DLDataType>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_ZEROS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.zeros").expect("missing global function")
        });
        pub fn zeros(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ZEROS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_ZEROS_LIKE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.zeros_like").expect("missing global function")
        });
        pub fn zeros_like(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::DLDataType>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ZEROS_LIKE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::DLDataType>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_ONES: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.ones").expect("missing global function")
        });
        pub fn ones(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ONES;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_ONES_LIKE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.ones_like").expect("missing global function")
        });
        pub fn ones_like(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::DLDataType>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ONES_LIKE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::DLDataType>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_FULL_LIKE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.full_like").expect("missing global function")
        });
        pub fn full_like(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: Option<tvm_ffi::DLDataType>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_FULL_LIKE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::DLDataType>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_FULL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.full").expect("missing global function")
        });
        pub fn full(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_OP_FULL;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_OP_WRAP_PARAM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.wrap_param").expect("missing global function")
        });
        pub fn wrap_param(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_WRAP_PARAM;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_ASTYPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.astype").expect("missing global function")
        });
        pub fn astype(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ASTYPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_DYNAMIC_STRIDED_SLICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.dynamic_strided_slice").expect("missing global function")
        });
        pub fn dynamic_strided_slice(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_DYNAMIC_STRIDED_SLICE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3.into())
        }

        static FUNC_RELAX_OP_STRIDED_SLICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.strided_slice").expect("missing global function")
        });
        pub fn strided_slice(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: crate::ir::RelaxExpr,
            _4: Option<crate::ir::RelaxExpr>,
            _5: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_STRIDED_SLICE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<crate::ir::RelaxExpr>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2.into(), _3.into(), _4, _5)
        }

        static FUNC_RELAX_OP_TAKE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.take").expect("missing global function")
        });
        pub fn take(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: Option<i64>,
            _3: tvm_ffi::String,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TAKE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<i64>,
                    tvm_ffi::String,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_RELAX_OP_OUTER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.outer").expect("missing global function")
        });
        pub fn outer(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_OUTER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_EINSUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.einsum").expect("missing global function")
        });
        pub fn einsum(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::String,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EINSUM;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_MATMUL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.matmul").expect("missing global function")
        });
        pub fn matmul(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: Option<tvm_ffi::DLDataType>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MATMUL;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::DLDataType>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_ONE_HOT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.one_hot").expect("missing global function")
        });
        pub fn one_hot(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::PrimValue,
            _2: crate::relax::expr::PrimValue,
            _3: i64,
            _4: i64,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ONE_HOT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, i64) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_RELAX_OP_SLICE_SCATTER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.slice_scatter").expect("missing global function")
        });
        pub fn slice_scatter(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: i64,
            _3: crate::relax::expr::PrimValue,
            _4: crate::relax::expr::PrimValue,
            _5: crate::relax::expr::PrimValue,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SLICE_SCATTER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2, _3.into(), _4.into(), _5.into())
        }

        static FUNC_RELAX_OP_SCATTER_ND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.scatter_nd").expect("missing global function")
        });
        pub fn scatter_nd(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: tvm_ffi::String,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SCATTER_ND;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_RELAX_OP_SCATTER_ELEMENTS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.scatter_elements").expect("missing global function")
        });
        pub fn scatter_elements(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: i64,
            _4: tvm_ffi::String,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SCATTER_ELEMENTS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_RELAX_OP_MESHGRID: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.meshgrid").expect("missing global function")
        });
        pub fn meshgrid(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::String>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MESHGRID;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::String>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_INDEX_PUT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.index_put").expect("missing global function")
        });
        pub fn index_put(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_INDEX_PUT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_RELAX_OP_GATHER_ND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.gather_nd").expect("missing global function")
        });
        pub fn gather_nd(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: i64,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_GATHER_ND;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_GATHER_ELEMENTS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.gather_elements").expect("missing global function")
        });
        pub fn gather_elements(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: i64,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_GATHER_ELEMENTS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_RELAX_OP_FLIP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.flip").expect("missing global function")
        });
        pub fn flip(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::IntImm,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_FLIP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_REPEAT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.repeat").expect("missing global function")
        });
        pub fn repeat(
            _0: crate::ir::RelaxExpr,
            _1: i64,
            _2: Option<i64>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_REPEAT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, i64, Option<i64>) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_STACK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.stack").expect("missing global function")
        });
        pub fn stack(
            _0: crate::ir::RelaxExpr,
            _1: Option<crate::ir::IntImm>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_STACK;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<crate::ir::IntImm>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_SPLIT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.split").expect("missing global function")
        });
        pub fn split(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_OP_SPLIT;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_OP_RESHAPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.reshape").expect("missing global function")
        });
        pub fn reshape(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_OP_RESHAPE;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_OP_SQUEEZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.squeeze").expect("missing global function")
        });
        pub fn squeeze(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SQUEEZE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_PERMUTE_DIMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.permute_dims").expect("missing global function")
        });
        pub fn permute_dims(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_PERMUTE_DIMS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_LAYOUT_TRANSFORM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.layout_transform").expect("missing global function")
        });
        pub fn layout_transform(
            _0: crate::ir::RelaxExpr,
            _1: crate::tir::IndexMap,
            _2: Option<crate::relax::expr::PrimValue>,
            _3: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _4: Option<tvm_ffi::Array<crate::ir::IntImm>>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LAYOUT_TRANSFORM;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<crate::relax::expr::PrimValue>,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_RELAX_OP_FLATTEN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.flatten").expect("missing global function")
        });
        pub fn flatten(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_FLATTEN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_TILE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.tile").expect("missing global function")
        });
        pub fn tile(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::Array<crate::ir::IntImm>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TILE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::IntImm>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_EXPAND_DIMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.expand_dims").expect("missing global function")
        });
        pub fn expand_dims(
            _0: crate::ir::RelaxExpr,
            _1: tvm_ffi::Array<crate::ir::IntImm>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EXPAND_DIMS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::IntImm>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_CONCAT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.concat").expect("missing global function")
        });
        pub fn concat(_0: crate::ir::RelaxExpr, _1: Option<i64>) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CONCAT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, Option<i64>) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1)
        }

        static FUNC_RELAX_OP_COLLAPSE_SUM_TO: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.collapse_sum_to").expect("missing global function")
        });
        pub fn collapse_sum_to(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_COLLAPSE_SUM_TO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_COLLAPSE_SUM_LIKE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.collapse_sum_like").expect("missing global function")
        });
        pub fn collapse_sum_like(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_COLLAPSE_SUM_LIKE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_INDEX_TENSOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.index_tensor").expect("missing global function")
        });
        pub fn index_tensor(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_INDEX_TENSOR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_BROADCAST_TO: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.broadcast_to").expect("missing global function")
        });
        pub fn broadcast_to(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_BROADCAST_TO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into())
        }

        static FUNC_RELAX_OP_DEQUANTIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.dequantize").expect("missing global function")
        });
        pub fn dequantize(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: i64,
            _4: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_DEQUANTIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_RELAX_OP_QUANTIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.quantize").expect("missing global function")
        });
        pub fn quantize(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: i64,
            _4: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_QUANTIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_RELAX_OP_MULTINOMIAL_FROM_UNIFORM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.multinomial_from_uniform")
                .expect("missing global function")
        });
        pub fn multinomial_from_uniform(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
            _3: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MULTINOMIAL_FROM_UNIFORM;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_RELAX_OP_ARGMIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.argmin").expect("missing global function")
        });
        pub fn argmin(
            _0: crate::ir::RelaxExpr,
            _1: Option<i64>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ARGMIN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, Option<i64>, bool) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_ARGMAX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.argmax").expect("missing global function")
        });
        pub fn argmax(
            _0: crate::ir::RelaxExpr,
            _1: Option<i64>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ARGMAX;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, Option<i64>, bool) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_WHERE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.where").expect("missing global function")
        });
        pub fn where_(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_WHERE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_RELAX_OP_BUCKETIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.bucketize").expect("missing global function")
        });
        pub fn bucketize(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: bool,
            _3: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_BUCKETIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool, bool) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_RELAX_OP_NONZERO: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.nonzero").expect("missing global function")
        });
        pub fn nonzero(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_NONZERO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_UNIQUE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.unique").expect("missing global function")
        });
        pub fn unique(
            _0: crate::ir::RelaxExpr,
            _1: crate::relax::expr::PrimValue,
            _2: crate::relax::expr::PrimValue,
            _3: crate::relax::expr::PrimValue,
            _4: crate::relax::expr::PrimValue,
            _5: Option<crate::relax::expr::PrimValue>,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_UNIQUE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<crate::relax::expr::PrimValue>,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1.into(), _2.into(), _3.into(), _4.into(), _5)
        }

        static FUNC_RELAX_OP_TOPK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.topk").expect("missing global function")
        });
        pub fn topk(
            _0: crate::ir::RelaxExpr,
            _1: i64,
            _2: i64,
            _3: tvm_ffi::String,
            _4: bool,
            _5: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TOPK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64, tvm_ffi::String, bool, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1, _2, _3, _4, _5)
        }

        static FUNC_RELAX_OP_ARGSORT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.argsort").expect("missing global function")
        });
        pub fn argsort(
            _0: crate::ir::RelaxExpr,
            _1: i64,
            _2: bool,
            _3: tvm_ffi::DLDataType,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ARGSORT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, bool, tvm_ffi::DLDataType) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_RELAX_OP_SORT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sort").expect("missing global function")
        });
        pub fn sort(_0: crate::ir::RelaxExpr, _1: i64, _2: bool) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SORT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, bool) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_VARIANCE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.variance").expect("missing global function")
        });
        pub fn variance(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_VARIANCE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_SUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sum").expect("missing global function")
        });
        pub fn sum(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SUM;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_STD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.std").expect("missing global function")
        });
        pub fn std(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_STD;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_PROD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.prod").expect("missing global function")
        });
        pub fn prod(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_PROD;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_MIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.min").expect("missing global function")
        });
        pub fn min(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MIN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_MEAN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.mean").expect("missing global function")
        });
        pub fn mean(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MEAN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_MAX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.max").expect("missing global function")
        });
        pub fn max(
            _0: crate::ir::RelaxExpr,
            _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _2: bool,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_MAX;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    bool,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_RELAX_OP_CUMSUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.cumsum").expect("missing global function")
        });
        pub fn cumsum(
            _0: crate::ir::RelaxExpr,
            _1: Option<i64>,
            _2: Option<tvm_ffi::DLDataType>,
            _3: crate::ir::IntImm,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CUMSUM;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<i64>,
                    Option<tvm_ffi::DLDataType>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2, _3.into())
        }

        static FUNC_RELAX_OP_CUMPROD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.cumprod").expect("missing global function")
        });
        pub fn cumprod(
            _0: crate::ir::RelaxExpr,
            _1: Option<i64>,
            _2: Option<tvm_ffi::DLDataType>,
            _3: crate::ir::IntImm,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CUMPROD;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<i64>,
                    Option<tvm_ffi::DLDataType>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<crate::ir::RelaxExpr>
            );
            typed(_0.into(), _1, _2, _3.into())
        }

        static FUNC_RELAX_OP_EWISE_FMA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.ewise_fma").expect("missing global function")
        });
        pub fn ewise_fma(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EWISE_FMA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_RELAX_OP_CLIP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.clip").expect("missing global function")
        });
        pub fn clip(
            _0: crate::ir::RelaxExpr,
            _1: crate::ir::RelaxExpr,
            _2: crate::ir::RelaxExpr,
        ) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CLIP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_RELAX_OP_ISNAN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.isnan").expect("missing global function")
        });
        pub fn isnan(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ISNAN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ISINF: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.isinf").expect("missing global function")
        });
        pub fn isinf(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ISINF;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ISFINITE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.isfinite").expect("missing global function")
        });
        pub fn isfinite(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ISFINITE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ERF: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.erf").expect("missing global function")
        });
        pub fn erf(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ERF;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_TRUNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.trunc").expect("missing global function")
        });
        pub fn trunc(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TRUNC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_TANH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.tanh").expect("missing global function")
        });
        pub fn tanh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TANH;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_TAN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.tan").expect("missing global function")
        });
        pub fn tan(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_TAN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SQRT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sqrt").expect("missing global function")
        });
        pub fn sqrt(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SQRT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SQUARE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.square").expect("missing global function")
        });
        pub fn square(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SQUARE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SINH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sinh").expect("missing global function")
        });
        pub fn sinh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SINH;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sin").expect("missing global function")
        });
        pub fn sin(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SIN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SIGN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sign").expect("missing global function")
        });
        pub fn sign(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SIGN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_SIGMOID: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.sigmoid").expect("missing global function")
        });
        pub fn sigmoid(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_SIGMOID;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_RSQRT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.rsqrt").expect("missing global function")
        });
        pub fn rsqrt(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_RSQRT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ROUND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.round").expect("missing global function")
        });
        pub fn round(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ROUND;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_NEGATIVE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.negative").expect("missing global function")
        });
        pub fn negative(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_NEGATIVE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_LOGICAL_NOT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.logical_not").expect("missing global function")
        });
        pub fn logical_not(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LOGICAL_NOT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_LOG: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.log").expect("missing global function")
        });
        pub fn log(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_LOG;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_FLOOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.floor").expect("missing global function")
        });
        pub fn floor(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_FLOOR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_EXP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.exp").expect("missing global function")
        });
        pub fn exp(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_EXP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_COSH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.cosh").expect("missing global function")
        });
        pub fn cosh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_COSH;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_COS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.cos").expect("missing global function")
        });
        pub fn cos(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_COS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_CEIL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.ceil").expect("missing global function")
        });
        pub fn ceil(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_CEIL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_BITWISE_NOT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.bitwise_not").expect("missing global function")
        });
        pub fn bitwise_not(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_BITWISE_NOT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ATANH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.atanh").expect("missing global function")
        });
        pub fn atanh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ATANH;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ATAN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.atan").expect("missing global function")
        });
        pub fn atan(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ATAN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ASINH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.asinh").expect("missing global function")
        });
        pub fn asinh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ASINH;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ASIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.asin").expect("missing global function")
        });
        pub fn asin(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ASIN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ACOSH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.acosh").expect("missing global function")
        });
        pub fn acosh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ACOSH;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ACOS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.acos").expect("missing global function")
        });
        pub fn acos(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ACOS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        static FUNC_RELAX_OP_ABS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.op.abs").expect("missing global function")
        });
        pub fn abs(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
            let func = &*FUNC_RELAX_OP_ABS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
            typed(_0.into())
        }

        pub mod builtin {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_BUILTIN_ALLOC_TENSOR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.builtin.alloc_tensor")
                    .expect("missing global function")
            });
            pub fn alloc_tensor(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::DataTypeImm,
                _2: crate::relax::expr::PrimValue,
                _3: crate::relax::expr::StringImm,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_BUILTIN_ALLOC_TENSOR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into(), _3.into())
            }

            static FUNC_RELAX_OP_BUILTIN_STOP_LIFT_PARAMS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.builtin.stop_lift_params")
                        .expect("missing global function")
                });
            pub fn stop_lift_params(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_BUILTIN_STOP_LIFT_PARAMS;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }
        }
        pub mod ccl {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_CCL_SCATTER_FROM_WORKER0: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.ccl.scatter_from_worker0")
                        .expect("missing global function")
                });
            pub fn scatter_from_worker0(
                _0: crate::ir::RelaxExpr,
                _1: i64,
                _2: i64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_CCL_SCATTER_FROM_WORKER0;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1, _2)
            }

            static FUNC_RELAX_OP_CCL_BROADCAST_FROM_WORKER0: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.ccl.broadcast_from_worker0")
                        .expect("missing global function")
                });
            pub fn broadcast_from_worker0(
                _0: crate::ir::RelaxExpr,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_CCL_BROADCAST_FROM_WORKER0;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_CCL_ALLGATHER: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.ccl.allgather").expect("missing global function")
            });
            pub fn allgather(
                _0: crate::ir::RelaxExpr,
                _1: i64,
                _2: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_CCL_ALLGATHER;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, bool) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1, _2)
            }

            static FUNC_RELAX_OP_CCL_ALLREDUCE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.ccl.allreduce").expect("missing global function")
            });
            pub fn allreduce(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::String,
                _2: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_CCL_ALLREDUCE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1, _2)
            }
        }
        pub mod dist {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_DIST_REDISTRIBUTE_REPLICA_TO_SHARD: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.dist.redistribute_replica_to_shard")
                        .expect("missing global function")
                });
            pub fn redistribute_replica_to_shard(
                _0: crate::ir::RelaxExpr,
                _1: i64,
                _2: i64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_DIST_REDISTRIBUTE_REPLICA_TO_SHARD;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1, _2)
            }

            static FUNC_RELAX_OP_DIST_CALL_TIR_LOCAL_VIEW: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.dist.call_tir_local_view")
                        .expect("missing global function")
                });
            pub fn call_tir_local_view(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::Tuple,
                _2: tvm_ffi::Array<crate::relax::DTensorStructInfo>,
                _3: Option<crate::ir::RelaxExpr>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_DIST_CALL_TIR_LOCAL_VIEW;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::relax::DTensorStructInfo>,
                        Option<crate::ir::RelaxExpr>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_RELAX_OP_DIST_REDISTRIBUTE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.dist.redistribute").expect("missing global function")
            });
            pub fn redistribute(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::distributed::DeviceMesh,
                _2: crate::relax::distributed::Placement,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_DIST_REDISTRIBUTE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into())
            }

            static FUNC_RELAX_OP_DIST_ANNOTATE_SHARDING: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.dist.annotate_sharding")
                    .expect("missing global function")
            });
            pub fn annotate_sharding(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::distributed::DeviceMesh,
                _2: crate::relax::distributed::Placement,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_DIST_ANNOTATE_SHARDING;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into())
            }
        }
        pub mod grad {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_GRAD_TAKE_BACKWARD: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.grad.take_backward")
                    .expect("missing global function")
            });
            pub fn take_backward(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: Option<i64>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_TAKE_BACKWARD;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<i64>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2.into(), _3)
            }

            static FUNC_RELAX_OP_GRAD_AVG_POOL2D_BACKWARD: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.grad.avg_pool2d_backward")
                        .expect("missing global function")
                });
            pub fn avg_pool2d_backward(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: tvm_ffi::Array<crate::ir::IntImm>,
                _6: bool,
                _7: bool,
                _8: tvm_ffi::String,
                _9: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_AVG_POOL2D_BACKWARD;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9)
            }

            static FUNC_RELAX_OP_GRAD_MAX_POOL2D_BACKWARD: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.grad.max_pool2d_backward")
                        .expect("missing global function")
                });
            pub fn max_pool2d_backward(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: tvm_ffi::Array<crate::ir::IntImm>,
                _6: bool,
                _7: bool,
                _8: tvm_ffi::String,
                _9: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_MAX_POOL2D_BACKWARD;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9)
            }

            static FUNC_RELAX_OP_GRAD_NLL_LOSS_BACKWARD: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.grad.nll_loss_backward")
                    .expect("missing global function")
            });
            pub fn nll_loss_backward(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: Option<crate::ir::RelaxExpr>,
                _4: tvm_ffi::String,
                _5: i64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_NLL_LOSS_BACKWARD;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::RelaxExpr>,
                        tvm_ffi::String,
                        i64,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2.into(), _3, _4, _5)
            }

            static FUNC_RELAX_OP_GRAD_END_CHECKPOINT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.grad.end_checkpoint")
                    .expect("missing global function")
            });
            pub fn end_checkpoint(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_END_CHECKPOINT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_GRAD_START_CHECKPOINT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.grad.start_checkpoint")
                    .expect("missing global function")
            });
            pub fn start_checkpoint(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_START_CHECKPOINT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_GRAD_NO_GRAD: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.grad.no_grad").expect("missing global function")
            });
            pub fn no_grad(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_GRAD_NO_GRAD;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }
        }
        pub mod image {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_IMAGE_GRID_SAMPLE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.image.grid_sample").expect("missing global function")
            });
            pub fn grid_sample(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::String,
                _3: tvm_ffi::String,
                _4: tvm_ffi::String,
                _5: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_IMAGE_GRID_SAMPLE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::String, tvm_ffi::String, bool) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2, _3, _4, _5)
            }

            static FUNC_RELAX_OP_IMAGE_RESIZE2D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.image.resize2d").expect("missing global function")
            });
            pub fn resize2d(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::FloatImm>,
                _3: tvm_ffi::String,
                _4: tvm_ffi::String,
                _5: tvm_ffi::String,
                _6: tvm_ffi::String,
                _7: f64,
                _8: i64,
                _9: f64,
                _10: Option<tvm_ffi::DLDataType>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_IMAGE_RESIZE2D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::FloatImm>,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        f64,
                        i64,
                        f64,
                        Option<tvm_ffi::DLDataType>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9, _10)
            }
        }
        pub mod memory {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_MEMORY_ENSURE_ZERO_OFFSET: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.memory.ensure_zero_offset")
                        .expect("missing global function")
                });
            pub fn ensure_zero_offset(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_MEMORY_ENSURE_ZERO_OFFSET;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_MEMORY_VIEW: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.memory.view").expect("missing global function")
            });
            pub fn view(
                _0: crate::ir::RelaxExpr,
                _1: Option<crate::ir::RelaxExpr>,
                _2: Option<crate::ir::RelaxExpr>,
                _3: Option<crate::ir::RelaxExpr>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_MEMORY_VIEW;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::RelaxExpr>,
                        Option<crate::ir::RelaxExpr>,
                        Option<crate::ir::RelaxExpr>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3)
            }

            static FUNC_RELAX_OP_MEMORY_ALLOC_TENSOR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.memory.alloc_tensor")
                    .expect("missing global function")
            });
            pub fn alloc_tensor(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::PrimValue,
                _2: crate::ir::RelaxExpr,
                _3: crate::relax::expr::DataTypeImm,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_MEMORY_ALLOC_TENSOR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into(), _3.into())
            }

            static FUNC_RELAX_OP_MEMORY_ALLOC_STORAGE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.memory.alloc_storage")
                    .expect("missing global function")
            });
            pub fn alloc_storage(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::PrimValue,
                _2: crate::relax::expr::StringImm,
                _3: crate::relax::expr::DataTypeImm,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_MEMORY_ALLOC_STORAGE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into(), _3.into())
            }

            static FUNC_RELAX_OP_MEMORY_KILL_TENSOR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.memory.kill_tensor")
                    .expect("missing global function")
            });
            pub fn kill_tensor(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_MEMORY_KILL_TENSOR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_MEMORY_KILL_STORAGE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.memory.kill_storage")
                    .expect("missing global function")
            });
            pub fn kill_storage(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_MEMORY_KILL_STORAGE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }
        }
        pub mod nn {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_NN_ATTENTION: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.attention").expect("missing global function")
            });
            pub fn attention(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: Option<crate::ir::RelaxExpr>,
                _4: Option<crate::ir::FloatImm>,
                _5: Option<tvm_ffi::String>,
                _6: Option<crate::ir::IntImm>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_ATTENTION;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::RelaxExpr>,
                        Option<crate::ir::FloatImm>,
                        Option<tvm_ffi::String>,
                        Option<crate::ir::IntImm>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2.into(), _3, _4, _5, _6)
            }

            static FUNC_RELAX_OP_NN_ATTENTION_VAR_LEN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.attention_var_len")
                    .expect("missing global function")
            });
            pub fn attention_var_len(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: crate::ir::RelaxExpr,
                _4: crate::ir::RelaxExpr,
                _5: crate::ir::RelaxExpr,
                _6: crate::ir::RelaxExpr,
                _7: Option<crate::ir::FloatImm>,
                _8: Option<tvm_ffi::String>,
                _9: Option<crate::ir::IntImm>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_ATTENTION_VAR_LEN;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::FloatImm>,
                        Option<tvm_ffi::String>,
                        Option<crate::ir::IntImm>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(
                    _0.into(),
                    _1.into(),
                    _2.into(),
                    _3.into(),
                    _4.into(),
                    _5.into(),
                    _6.into(),
                    _7,
                    _8,
                    _9,
                )
            }

            static FUNC_RELAX_OP_NN_CONV2D_TRANSPOSE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.conv2d_transpose")
                    .expect("missing global function")
            });
            pub fn conv2d_transpose(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: tvm_ffi::Array<crate::ir::IntImm>,
                _6: i64,
                _7: tvm_ffi::String,
                _8: tvm_ffi::String,
                _9: Option<tvm_ffi::String>,
                _10: Option<tvm_ffi::DLDataType>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_CONV2D_TRANSPOSE;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        i64,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                        Option<tvm_ffi::DLDataType>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9, _10)
            }

            static FUNC_RELAX_OP_NN_CONV1D_TRANSPOSE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.conv1d_transpose")
                    .expect("missing global function")
            });
            pub fn conv1d_transpose(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: tvm_ffi::Array<crate::ir::IntImm>,
                _6: i64,
                _7: tvm_ffi::String,
                _8: tvm_ffi::String,
                _9: Option<tvm_ffi::String>,
                _10: Option<tvm_ffi::DLDataType>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_CONV1D_TRANSPOSE;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        i64,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                        Option<tvm_ffi::DLDataType>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9, _10)
            }

            static FUNC_RELAX_OP_NN_CONV3D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.conv3d").expect("missing global function")
            });
            pub fn conv3d(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: i64,
                _6: tvm_ffi::String,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
                _9: Option<tvm_ffi::DLDataType>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_CONV3D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        i64,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                        Option<tvm_ffi::DLDataType>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9)
            }

            static FUNC_RELAX_OP_NN_CONV2D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.conv2d").expect("missing global function")
            });
            pub fn conv2d(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: i64,
                _6: tvm_ffi::String,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
                _9: Option<tvm_ffi::DLDataType>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_CONV2D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        i64,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                        Option<tvm_ffi::DLDataType>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9)
            }

            static FUNC_RELAX_OP_NN_CONV1D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.conv1d").expect("missing global function")
            });
            pub fn conv1d(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: i64,
                _6: tvm_ffi::String,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
                _9: Option<tvm_ffi::DLDataType>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_CONV1D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        i64,
                        tvm_ffi::String,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                        Option<tvm_ffi::DLDataType>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4, _5, _6, _7, _8, _9)
            }

            static FUNC_RELAX_OP_NN_NLL_LOSS: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.nll_loss").expect("missing global function")
            });
            pub fn nll_loss(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: Option<crate::ir::RelaxExpr>,
                _3: tvm_ffi::String,
                _4: i64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_NLL_LOSS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::RelaxExpr>,
                        tvm_ffi::String,
                        i64,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3, _4)
            }

            static FUNC_RELAX_OP_NN_CROSS_ENTROPY_WITH_LOGITS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.nn.cross_entropy_with_logits")
                        .expect("missing global function")
                });
            pub fn cross_entropy_with_logits(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_CROSS_ENTROPY_WITH_LOGITS;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into())
            }

            static FUNC_RELAX_OP_NN_RMS_NORM: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.rms_norm").expect("missing global function")
            });
            pub fn rms_norm(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: f64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_RMS_NORM;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        f64,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_RELAX_OP_NN_INSTANCE_NORM: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.instance_norm").expect("missing global function")
            });
            pub fn instance_norm(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: i64,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: f64,
                _6: bool,
                _7: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_INSTANCE_NORM;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        i64,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        f64,
                        bool,
                        bool,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2.into(), _3, _4, _5, _6, _7)
            }

            static FUNC_RELAX_OP_NN_GROUP_NORM: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.group_norm").expect("missing global function")
            });
            pub fn group_norm(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: i64,
                _4: i64,
                _5: tvm_ffi::Array<crate::ir::IntImm>,
                _6: f64,
                _7: bool,
                _8: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_GROUP_NORM;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        i64,
                        i64,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        f64,
                        bool,
                        bool,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2.into(), _3, _4, _5, _6, _7, _8)
            }

            static FUNC_RELAX_OP_NN_LAYER_NORM: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.layer_norm").expect("missing global function")
            });
            pub fn layer_norm(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: f64,
                _5: bool,
                _6: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_LAYER_NORM;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        f64,
                        bool,
                        bool,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1.into(), _2.into(), _3, _4, _5, _6)
            }

            static FUNC_RELAX_OP_NN_BATCH_NORM: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.batch_norm").expect("missing global function")
            });
            pub fn batch_norm(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: crate::ir::RelaxExpr,
                _4: crate::ir::RelaxExpr,
                _5: i64,
                _6: f64,
                _7: bool,
                _8: bool,
                _9: f64,
                _10: bool,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_BATCH_NORM;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, f64, bool, bool, f64, bool) -> Result<crate::ir::RelaxExpr>);
                typed(
                    _0.into(),
                    _1.into(),
                    _2.into(),
                    _3.into(),
                    _4.into(),
                    _5,
                    _6,
                    _7,
                    _8,
                    _9,
                    _10,
                )
            }

            static FUNC_RELAX_OP_NN_PAD: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.pad").expect("missing global function")
            });
            pub fn pad(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::String,
                _3: f64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_PAD;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::String,
                        f64,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3)
            }

            static FUNC_RELAX_OP_NN_PIXEL_SHUFFLE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.pixel_shuffle").expect("missing global function")
            });
            pub fn pixel_shuffle(
                _0: crate::ir::RelaxExpr,
                _1: i64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_PIXEL_SHUFFLE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1)
            }

            static FUNC_RELAX_OP_NN_LOG_SOFTMAX: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.log_softmax").expect("missing global function")
            });
            pub fn log_softmax(_0: crate::ir::RelaxExpr, _1: i64) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_LOG_SOFTMAX;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1)
            }

            static FUNC_RELAX_OP_NN_SOFTMAX: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.softmax").expect("missing global function")
            });
            pub fn softmax(_0: crate::ir::RelaxExpr, _1: i64) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_SOFTMAX;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1)
            }

            static FUNC_RELAX_OP_NN_PRELU: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.prelu").expect("missing global function")
            });
            pub fn prelu(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: i64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_PRELU;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2)
            }

            static FUNC_RELAX_OP_NN_SOFTPLUS: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.softplus").expect("missing global function")
            });
            pub fn softplus(
                _0: crate::ir::RelaxExpr,
                _1: f64,
                _2: f64,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_SOFTPLUS;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, f64, f64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1, _2)
            }

            static FUNC_RELAX_OP_NN_DROPOUT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.dropout").expect("missing global function")
            });
            pub fn dropout(_0: crate::ir::RelaxExpr, _1: f64) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_DROPOUT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, f64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1)
            }

            static FUNC_RELAX_OP_NN_LEAKYRELU: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.leakyrelu").expect("missing global function")
            });
            pub fn leakyrelu(_0: crate::ir::RelaxExpr, _1: f64) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_LEAKYRELU;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, f64) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1)
            }

            static FUNC_RELAX_OP_NN_SILU: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.silu").expect("missing global function")
            });
            pub fn silu(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_SILU;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_NN_SELU: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.selu").expect("missing global function")
            });
            pub fn selu(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_SELU;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_NN_GELU_TANH: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.gelu_tanh").expect("missing global function")
            });
            pub fn gelu_tanh(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_GELU_TANH;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_NN_GELU: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.gelu").expect("missing global function")
            });
            pub fn gelu(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_GELU;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_NN_RELU: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.relu").expect("missing global function")
            });
            pub fn relu(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_RELU;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_NN_ADAPTIVE_AVG_POOL3D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.adaptive_avg_pool3d")
                    .expect("missing global function")
            });
            pub fn adaptive_avg_pool3d(
                _0: crate::ir::RelaxExpr,
                _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
                _2: tvm_ffi::String,
                _3: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_ADAPTIVE_AVG_POOL3D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3)
            }

            static FUNC_RELAX_OP_NN_ADAPTIVE_AVG_POOL2D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.adaptive_avg_pool2d")
                    .expect("missing global function")
            });
            pub fn adaptive_avg_pool2d(
                _0: crate::ir::RelaxExpr,
                _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
                _2: tvm_ffi::String,
                _3: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_ADAPTIVE_AVG_POOL2D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3)
            }

            static FUNC_RELAX_OP_NN_ADAPTIVE_AVG_POOL1D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.adaptive_avg_pool1d")
                    .expect("missing global function")
            });
            pub fn adaptive_avg_pool1d(
                _0: crate::ir::RelaxExpr,
                _1: Option<tvm_ffi::Array<crate::ir::IntImm>>,
                _2: tvm_ffi::String,
                _3: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_ADAPTIVE_AVG_POOL1D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3)
            }

            static FUNC_RELAX_OP_NN_AVG_POOL3D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.avg_pool3d").expect("missing global function")
            });
            pub fn avg_pool3d(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: bool,
                _6: bool,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_AVG_POOL3D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3, _4, _5, _6, _7, _8)
            }

            static FUNC_RELAX_OP_NN_AVG_POOL2D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.avg_pool2d").expect("missing global function")
            });
            pub fn avg_pool2d(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: bool,
                _6: bool,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_AVG_POOL2D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3, _4, _5, _6, _7, _8)
            }

            static FUNC_RELAX_OP_NN_AVG_POOL1D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.avg_pool1d").expect("missing global function")
            });
            pub fn avg_pool1d(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: bool,
                _6: bool,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_AVG_POOL1D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3, _4, _5, _6, _7, _8)
            }

            static FUNC_RELAX_OP_NN_MAX_POOL3D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.max_pool3d").expect("missing global function")
            });
            pub fn max_pool3d(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: bool,
                _6: bool,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_MAX_POOL3D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3, _4, _5, _6, _7, _8)
            }

            static FUNC_RELAX_OP_NN_MAX_POOL2D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.max_pool2d").expect("missing global function")
            });
            pub fn max_pool2d(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: bool,
                _6: bool,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_MAX_POOL2D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3, _4, _5, _6, _7, _8)
            }

            static FUNC_RELAX_OP_NN_MAX_POOL1D: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.nn.max_pool1d").expect("missing global function")
            });
            pub fn max_pool1d(
                _0: crate::ir::RelaxExpr,
                _1: tvm_ffi::Array<crate::ir::IntImm>,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
                _3: tvm_ffi::Array<crate::ir::IntImm>,
                _4: tvm_ffi::Array<crate::ir::IntImm>,
                _5: bool,
                _6: bool,
                _7: tvm_ffi::String,
                _8: Option<tvm_ffi::String>,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_NN_MAX_POOL1D;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        tvm_ffi::Array<crate::ir::IntImm>,
                        bool,
                        bool,
                        tvm_ffi::String,
                        Option<tvm_ffi::String>,
                    ) -> Result<crate::ir::RelaxExpr>
                );
                typed(_0.into(), _1, _2, _3, _4, _5, _6, _7, _8)
            }
        }
        pub mod vision {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_VISION_ALL_CLASS_NON_MAX_SUPPRESSION: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.op.vision.all_class_non_max_suppression")
                        .expect("missing global function")
                });
            pub fn all_class_non_max_suppression(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::RelaxExpr,
                _2: crate::ir::RelaxExpr,
                _3: crate::ir::RelaxExpr,
                _4: crate::ir::RelaxExpr,
                _5: tvm_ffi::String,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_VISION_ALL_CLASS_NON_MAX_SUPPRESSION;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into(), _3.into(), _4.into(), _5)
            }
        }
        pub mod vm {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_OP_VM_ALLOC_STORAGE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.vm.alloc_storage").expect("missing global function")
            });
            pub fn alloc_storage(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::PrimValue,
                _2: crate::relax::expr::DataTypeImm,
                _3: crate::relax::expr::StringImm,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_VM_ALLOC_STORAGE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into(), _3.into())
            }

            static FUNC_RELAX_OP_VM_ALLOC_TENSOR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.vm.alloc_tensor").expect("missing global function")
            });
            pub fn alloc_tensor(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::PrimValue,
                _2: crate::ir::RelaxExpr,
                _3: crate::relax::expr::DataTypeImm,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_VM_ALLOC_TENSOR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into(), _2.into(), _3.into())
            }

            static FUNC_RELAX_OP_VM_KILL_OBJECT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.vm.kill_object").expect("missing global function")
            });
            pub fn kill_object(_0: crate::ir::RelaxExpr) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_VM_KILL_OBJECT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into())
            }

            static FUNC_RELAX_OP_VM_CALL_TIR_DYN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("relax.op.vm.call_tir_dyn").expect("missing global function")
            });
            pub fn call_tir_dyn(
                _0: crate::ir::RelaxExpr,
                _1: crate::relax::expr::Tuple,
            ) -> Result<crate::ir::RelaxExpr> {
                let func = &*FUNC_RELAX_OP_VM_CALL_TIR_DYN;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::RelaxExpr>);
                typed(_0.into(), _1.into())
            }
        }
    }
    pub mod testing {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        pub mod transform {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_RELAX_TESTING_TRANSFORM_APPLYEMPTYCPPMUTATOR: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.testing.transform.ApplyEmptyCppMutator")
                        .expect("missing global function")
                });
            pub fn ApplyEmptyCppMutator() -> Result<crate::transform::Pass> {
                let func = &*FUNC_RELAX_TESTING_TRANSFORM_APPLYEMPTYCPPMUTATOR;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
                typed()
            }

            static FUNC_RELAX_TESTING_TRANSFORM_DATAFLOWLIVENESSANALYSIS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.testing.transform.DataflowLivenessAnalysis")
                        .expect("missing global function")
                });
            pub fn DataflowLivenessAnalysis(
                _0: crate::relax::expr::DataflowBlock,
            ) -> Result<tvm_ffi::Map<crate::relax::expr::Var, tvm_ffi::Array<crate::ir::IntImm>>>
            {
                let func = &*FUNC_RELAX_TESTING_TRANSFORM_DATAFLOWLIVENESSANALYSIS;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<crate::relax::expr::Var, tvm_ffi::Array<crate::ir::IntImm>>>);
                typed(_0.into())
            }

            static FUNC_RELAX_TESTING_TRANSFORM_DATAFLOWALIASANALYSIS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.testing.transform.DataflowAliasAnalysis")
                        .expect("missing global function")
                });
            pub fn DataflowAliasAnalysis(
                _0: crate::relax::expr::DataflowBlock,
                _1: tvm_ffi::Array<crate::relax::expr::Var>,
            ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>> {
                let func = &*FUNC_RELAX_TESTING_TRANSFORM_DATAFLOWALIASANALYSIS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::relax::expr::Var>,
                    ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>>
                );
                typed(_0.into(), _1)
            }

            static FUNC_RELAX_TESTING_TRANSFORM_DATAFLOWINPLACEANALYSIS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.testing.transform.DataflowInplaceAnalysis")
                        .expect("missing global function")
                });
            pub fn DataflowInplaceAnalysis(
                _0: crate::relax::expr::DataflowBlock,
                _1: tvm_ffi::Array<crate::relax::expr::Var>,
                _2: crate::ir::IRModule,
            ) -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::relax::transform::InplaceOpportunity>>>
            {
                let func = &*FUNC_RELAX_TESTING_TRANSFORM_DATAFLOWINPLACEANALYSIS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::relax::expr::Var>,
                        tvm_ffi::object::ObjectRef,
                    ) -> Result<
                        tvm_ffi::Array<tvm_ffi::Array<crate::relax::transform::InplaceOpportunity>>,
                    >
                );
                typed(_0.into(), _1, _2.into())
            }

            static FUNC_RELAX_TESTING_TRANSFORM_SINGLEINPLACECALL: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("relax.testing.transform.SingleInplaceCall")
                        .expect("missing global function")
                });
            pub fn SingleInplaceCall(
                _0: crate::ir::IRModule,
                _1: crate::relax::expr::Call,
                _2: tvm_ffi::Array<crate::ir::IntImm>,
            ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>> {
                let func = &*FUNC_RELAX_TESTING_TRANSFORM_SINGLEINPLACECALL;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::IntImm>,
                    ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>>
                );
                typed(_0.into(), _1.into(), _2)
            }
        }
    }
    pub mod training {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_TRAINING_APPENDLOSS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.training.AppendLoss").expect("missing global function")
        });
        pub fn AppendLoss(
            _0: tvm_ffi::String,
            _1: crate::relax::expr::Function,
            _2: i64,
            _3: Option<tvm_ffi::String>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRAINING_APPENDLOSS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::String,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    Option<tvm_ffi::String>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1.into(), _2, _3)
        }
    }
    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_RELAX_TRANSFORM_LOWERRUNTIMEBUILTIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LowerRuntimeBuiltin")
                .expect("missing global function")
        });
        pub fn LowerRuntimeBuiltin() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_LOWERRUNTIMEBUILTIN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_VMSHAPELOWER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.VMShapeLower").expect("missing global function")
        });
        pub fn VMShapeLower(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_VMSHAPELOWER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_MAKEFUNCTIONPASS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.MakeFunctionPass")
                .expect("missing global function")
        });
        pub fn MakeFunctionPass(
            _0: tvm_ffi::Function,
            _1: crate::transform::PassInfo,
        ) -> Result<crate::relax::FunctionPass> {
            let func = &*FUNC_RELAX_TRANSFORM_MAKEFUNCTIONPASS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function, tvm_ffi::object::ObjectRef) -> Result<crate::relax::FunctionPass>);
            typed(_0, _1.into())
        }

        static FUNC_RELAX_TRANSFORM_MAKEDATAFLOWBLOCKPASS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.MakeDataflowBlockPass")
                    .expect("missing global function")
            });
        pub fn MakeDataflowBlockPass(
            _0: tvm_ffi::Function,
            _1: crate::transform::PassInfo,
        ) -> Result<crate::relax::DataflowBlockPass> {
            let func = &*FUNC_RELAX_TRANSFORM_MAKEDATAFLOWBLOCKPASS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function, tvm_ffi::object::ObjectRef) -> Result<crate::relax::DataflowBlockPass>);
            typed(_0, _1.into())
        }

        static FUNC_RELAX_TRANSFORM_ADJUSTMATMULORDER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.AdjustMatmulOrder")
                .expect("missing global function")
        });
        pub fn AdjustMatmulOrder() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ADJUSTMATMULORDER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_ALLOCATEWORKSPACE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.AllocateWorkspace")
                .expect("missing global function")
        });
        pub fn AllocateWorkspace() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ALLOCATEWORKSPACE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_ALTEROPIMPL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.AlterOpImpl").expect("missing global function")
        });
        pub fn AlterOpImpl(
            _0: tvm_ffi::Map<tvm_ffi::String, crate::tir::PrimFunc>,
            _1: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::tir::IndexMap>>,
            _2: tvm_ffi::Map<
                tvm_ffi::String,
                Option<tvm_ffi::Array<tvm_ffi::Array<crate::ir::IntImm>>>,
            >,
            _3: tvm_ffi::Map<
                tvm_ffi::String,
                Option<tvm_ffi::Array<tvm_ffi::Array<crate::ir::IntImm>>>,
            >,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ALTEROPIMPL;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Map<tvm_ffi::String, crate::tir::PrimFunc>,
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::tir::IndexMap>>,
                    tvm_ffi::Map<
                        tvm_ffi::String,
                        Option<tvm_ffi::Array<tvm_ffi::Array<crate::ir::IntImm>>>,
                    >,
                    tvm_ffi::Map<
                        tvm_ffi::String,
                        Option<tvm_ffi::Array<tvm_ffi::Array<crate::ir::IntImm>>>,
                    >,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1, _2, _3)
        }

        static FUNC_RELAX_TRANSFORM_ANNOTATETIROPPATTERN: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.AnnotateTIROpPattern")
                    .expect("missing global function")
            });
        pub fn AnnotateTIROpPattern() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ANNOTATETIROPPATTERN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_ATTACHATTRLAYOUTFREEBUFFERS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.AttachAttrLayoutFreeBuffers")
                    .expect("missing global function")
            });
        pub fn AttachAttrLayoutFreeBuffers() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ATTACHATTRLAYOUTFREEBUFFERS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_ATTACHGLOBALSYMBOL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.AttachGlobalSymbol")
                .expect("missing global function")
        });
        pub fn AttachGlobalSymbol() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ATTACHGLOBALSYMBOL;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_BINDPARAMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.BindParams").expect("missing global function")
        });
        pub fn BindParams(
            _0: tvm_ffi::String,
            _1: tvm_ffi::Map<tvm_ffi::AnyValue, tvm_ffi::object::ObjectRef>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_BINDPARAMS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::String,
                    tvm_ffi::Map<tvm_ffi::AnyValue, tvm_ffi::object::ObjectRef>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_BINDSYMBOLICVARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.BindSymbolicVars")
                .expect("missing global function")
        });
        pub fn BindSymbolicVars(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_TRANSFORM_BINDSYMBOLICVARS;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_TRANSFORM_BUNDLEMODELPARAMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.BundleModelParams")
                .expect("missing global function")
        });
        pub fn BundleModelParams(_0: Option<tvm_ffi::String>) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_BUNDLEMODELPARAMS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(Option<tvm_ffi::String>) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_CALLTIRREWRITE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.CallTIRRewrite").expect("missing global function")
        });
        pub fn CallTIRRewrite() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_CALLTIRREWRITE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_CANONICALIZEBINDINGS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.CanonicalizeBindings")
                    .expect("missing global function")
            });
        pub fn CanonicalizeBindings() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_CANONICALIZEBINDINGS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_COMBINEPARALLELMATMUL: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.CombineParallelMatmul")
                    .expect("missing global function")
            });
        pub fn CombineParallelMatmul(_0: tvm_ffi::Function) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_COMBINEPARALLELMATMUL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_COMPUTEPRIMVALUE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.ComputePrimValue")
                .expect("missing global function")
        });
        pub fn ComputePrimValue() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_COMPUTEPRIMVALUE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_CONVERTTODATAFLOW: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.ConvertToDataflow")
                .expect("missing global function")
        });
        pub fn ConvertToDataflow(_0: i64) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_CONVERTTODATAFLOW;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(i64) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_CONVERTLAYOUT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.ConvertLayout").expect("missing global function")
        });
        pub fn ConvertLayout(
            _0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<tvm_ffi::String>>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_CONVERTLAYOUT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<tvm_ffi::String>>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_DATAFLOWUSEINPLACECALLS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.DataflowUseInplaceCalls")
                    .expect("missing global function")
            });
        pub fn DataflowUseInplaceCalls() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_DATAFLOWUSEINPLACECALLS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_DEADCODEELIMINATION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.DeadCodeElimination")
                .expect("missing global function")
        });
        pub fn DeadCodeElimination(
            _0: tvm_ffi::Array<tvm_ffi::String>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_DEADCODEELIMINATION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::Array<tvm_ffi::String>) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_DECOMPOSEOPSFORINFERENCE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.DecomposeOpsForInference")
                    .expect("missing global function")
            });
        pub fn DecomposeOpsForInference(
            _0: Option<tvm_ffi::String>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_DECOMPOSEOPSFORINFERENCE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(Option<tvm_ffi::String>) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_DECOMPOSEOPSFORTRAINING: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.DecomposeOpsForTraining")
                    .expect("missing global function")
            });
        pub fn DecomposeOpsForTraining(
            _0: Option<tvm_ffi::String>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_DECOMPOSEOPSFORTRAINING;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(Option<tvm_ffi::String>) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_ELIMINATECOMMONSUBEXPR: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.EliminateCommonSubexpr")
                    .expect("missing global function")
            });
        pub fn EliminateCommonSubexpr(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_ELIMINATECOMMONSUBEXPR;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_EXPANDMATMULOFSUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.ExpandMatmulOfSum")
                .expect("missing global function")
        });
        pub fn ExpandMatmulOfSum() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_EXPANDMATMULOFSUM;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_EXPANDTUPLEARGUMENTS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.ExpandTupleArguments")
                    .expect("missing global function")
            });
        pub fn ExpandTupleArguments() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_EXPANDTUPLEARGUMENTS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_FEWSHOTTUNING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.FewShotTuning").expect("missing global function")
        });
        pub fn FewShotTuning(_0: i64, _1: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_FEWSHOTTUNING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, bool) -> Result<crate::transform::Pass>);
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_FOLDCONSTANT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.FoldConstant").expect("missing global function")
        });
        pub fn FoldConstant() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_FOLDCONSTANT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_FUSIONPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.FusionPattern").expect("missing global function")
        });
        pub fn FusionPattern(
            _0: tvm_ffi::String,
            _1: crate::relax::dpl::DFPattern,
            _2: tvm_ffi::Map<tvm_ffi::String, crate::relax::dpl::DFPattern>,
            _3: Option<tvm_ffi::Function>,
            _4: Option<tvm_ffi::Function>,
        ) -> Result<crate::relax::transform::FusionPattern> {
            let func = &*FUNC_RELAX_TRANSFORM_FUSIONPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::String,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Map<tvm_ffi::String, crate::relax::dpl::DFPattern>,
                    Option<tvm_ffi::Function>,
                    Option<tvm_ffi::Function>,
                ) -> Result<crate::relax::transform::FusionPattern>
            );
            typed(_0, _1.into(), _2, _3, _4)
        }

        static FUNC_RELAX_TRANSFORM_FUSEOPSBYPATTERN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.FuseOpsByPattern")
                .expect("missing global function")
        });
        pub fn FuseOpsByPattern(
            _0: tvm_ffi::Array<crate::relax::transform::FusionPattern>,
            _1: bool,
            _2: bool,
            _3: tvm_ffi::Array<tvm_ffi::String>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_FUSEOPSBYPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::relax::transform::FusionPattern>,
                    bool,
                    bool,
                    tvm_ffi::Array<tvm_ffi::String>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1, _2, _3)
        }

        static FUNC_RELAX_TRANSFORM_FUSEOPS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.FuseOps").expect("missing global function")
        });
        pub fn FuseOps(_0: i64) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_FUSEOPS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(i64) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_FUSETIR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.FuseTIR").expect("missing global function")
        });
        pub fn FuseTIR() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_FUSETIR;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_GRADIENT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.Gradient").expect("missing global function")
        });
        pub fn Gradient(
            _0: tvm_ffi::String,
            _1: Option<tvm_ffi::Array<crate::relax::expr::Var>>,
            _2: i64,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_GRADIENT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::String,
                    Option<tvm_ffi::Array<crate::relax::expr::Var>>,
                    i64,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1, _2)
        }

        static FUNC_RELAX_TRANSFORM_INLINEPRIVATEFUNCTIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.InlinePrivateFunctions")
                    .expect("missing global function")
            });
        pub fn InlinePrivateFunctions() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_INLINEPRIVATEFUNCTIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_KILLAFTERLASTUSE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.KillAfterLastUse")
                .expect("missing global function")
        });
        pub fn KillAfterLastUse() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_KILLAFTERLASTUSE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_LAMBDALIFT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LambdaLift").expect("missing global function")
        });
        pub fn LambdaLift() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_LAMBDALIFT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_LAZYSETOUTPUT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LazySetOutput").expect("missing global function")
        });
        pub fn LazySetOutput() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_LAZYSETOUTPUT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_LAZYGETINPUT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LazyGetInput").expect("missing global function")
        });
        pub fn LazyGetInput() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_LAZYGETINPUT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_LEGALIZEOPS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LegalizeOps").expect("missing global function")
        });
        pub fn LegalizeOps(
            _0: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Function>>,
            _1: Option<tvm_ffi::Array<tvm_ffi::String>>,
            _2: bool,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_LEGALIZEOPS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Function>>,
                    Option<tvm_ffi::Array<tvm_ffi::String>>,
                    bool,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1, _2)
        }

        static FUNC_RELAX_TRANSFORM_LIFTTRANSFORMPARAMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LiftTransformParams")
                .expect("missing global function")
        });
        pub fn LiftTransformParams(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_RELAX_TRANSFORM_LIFTTRANSFORMPARAMS;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_RELAX_TRANSFORM_LOWERALLOCTENSOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.LowerAllocTensor")
                .expect("missing global function")
        });
        pub fn LowerAllocTensor() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_LOWERALLOCTENSOR;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_MERGECOMPOSITEFUNCTIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.MergeCompositeFunctions")
                    .expect("missing global function")
            });
        pub fn MergeCompositeFunctions() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_MERGECOMPOSITEFUNCTIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_METASCHEDULEAPPLYDATABASE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.MetaScheduleApplyDatabase")
                    .expect("missing global function")
            });
        pub fn MetaScheduleApplyDatabase(
            _0: Option<tvm_ffi::String>,
            _1: bool,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_METASCHEDULEAPPLYDATABASE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(Option<tvm_ffi::String>, bool) -> Result<crate::transform::Pass>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_METASCHEDULETUNEIRMOD: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.MetaScheduleTuneIRMod")
                    .expect("missing global function")
            });
        pub fn MetaScheduleTuneIRMod(
            _0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Tensor>,
            _1: tvm_ffi::String,
            _2: crate::ir::IntImm,
            _3: Option<crate::ir::IntImm>,
            _4: Option<tvm_ffi::Array<tvm_ffi::String>>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_METASCHEDULETUNEIRMOD;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Tensor>,
                    tvm_ffi::String,
                    tvm_ffi::object::ObjectRef,
                    Option<crate::ir::IntImm>,
                    Option<tvm_ffi::Array<tvm_ffi::String>>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1, _2.into(), _3, _4)
        }

        static FUNC_RELAX_TRANSFORM_METASCHEDULETUNETIR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.MetaScheduleTuneTIR")
                .expect("missing global function")
        });
        pub fn MetaScheduleTuneTIR(
            _0: tvm_ffi::String,
            _1: crate::ir::IntImm,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_METASCHEDULETUNETIR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::transform::Pass>);
            typed(_0, _1.into())
        }

        static FUNC_RELAX_TRANSFORM_NORMALIZEGLOBALVAR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.NormalizeGlobalVar")
                .expect("missing global function")
        });
        pub fn NormalizeGlobalVar() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_NORMALIZEGLOBALVAR;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_NORMALIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.Normalize").expect("missing global function")
        });
        pub fn Normalize() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_NORMALIZE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REALIZEVDEVICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.RealizeVDevice").expect("missing global function")
        });
        pub fn RealizeVDevice() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REALIZEVDEVICE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REMOVEPURITYCHECKING: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.RemovePurityChecking")
                    .expect("missing global function")
            });
        pub fn RemovePurityChecking() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REMOVEPURITYCHECKING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REMOVEUNUSEDOUTPUTS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.RemoveUnusedOutputs")
                .expect("missing global function")
        });
        pub fn RemoveUnusedOutputs() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REMOVEUNUSEDOUTPUTS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REMOVEUNUSEDPARAMETERS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.RemoveUnusedParameters")
                    .expect("missing global function")
            });
        pub fn RemoveUnusedParameters() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REMOVEUNUSEDPARAMETERS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REORDERPERMUTEDIMSAFTERCONCAT: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.ReorderPermuteDimsAfterConcat")
                    .expect("missing global function")
            });
        pub fn ReorderPermuteDimsAfterConcat() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REORDERPERMUTEDIMSAFTERCONCAT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REORDERTAKEAFTERMATMUL: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.ReorderTakeAfterMatmul")
                    .expect("missing global function")
            });
        pub fn ReorderTakeAfterMatmul() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REORDERTAKEAFTERMATMUL;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REWRITECUDAGRAPH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.RewriteCUDAGraph")
                .expect("missing global function")
        });
        pub fn RewriteCUDAGraph() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REWRITECUDAGRAPH;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_REWRITEDATAFLOWRESHAPE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.RewriteDataflowReshape")
                    .expect("missing global function")
            });
        pub fn RewriteDataflowReshape() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_REWRITEDATAFLOWRESHAPE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_RUNCODEGEN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.RunCodegen").expect("missing global function")
        });
        pub fn RunCodegen(
            _0: Option<
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
            >,
            _1: tvm_ffi::Array<tvm_ffi::String>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_RUNCODEGEN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    Option<
                        tvm_ffi::Map<
                            tvm_ffi::String,
                            tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                        >,
                    >,
                    tvm_ffi::Array<tvm_ffi::String>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_SPECIALIZEPRIMFUNCBASEDONCALLSITE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.SpecializePrimFuncBasedOnCallSite")
                    .expect("missing global function")
            });
        pub fn SpecializePrimFuncBasedOnCallSite() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_SPECIALIZEPRIMFUNCBASEDONCALLSITE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_SPLITCALLTIRBYPATTERN: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.SplitCallTIRByPattern")
                    .expect("missing global function")
            });
        pub fn SplitCallTIRByPattern(
            _0: tvm_ffi::Array<crate::tir::PrimFunc>,
            _1: tvm_ffi::Function,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_SPLITCALLTIRBYPATTERN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::tir::PrimFunc>,
                    tvm_ffi::Function,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_SPLITLAYOUTREWRITEPREPROC: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.SplitLayoutRewritePreproc")
                    .expect("missing global function")
            });
        pub fn SplitLayoutRewritePreproc() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_SPLITLAYOUTREWRITEPREPROC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_STATICPLANBLOCKMEMORY: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.StaticPlanBlockMemory")
                    .expect("missing global function")
            });
        pub fn StaticPlanBlockMemory() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_STATICPLANBLOCKMEMORY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_TOMIXEDPRECISION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.ToMixedPrecision")
                .expect("missing global function")
        });
        pub fn ToMixedPrecision(
            _0: tvm_ffi::DLDataType,
            _1: Option<tvm_ffi::Array<tvm_ffi::String>>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_TOMIXEDPRECISION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::DLDataType,
                    Option<tvm_ffi::Array<tvm_ffi::String>>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_TONONDATAFLOW: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.ToNonDataflow").expect("missing global function")
        });
        pub fn ToNonDataflow() -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_TONONDATAFLOW;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_RELAX_TRANSFORM_TOPOLOGICALSORT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.TopologicalSort")
                .expect("missing global function")
        });
        pub fn TopologicalSort(
            _0: tvm_ffi::String,
            _1: tvm_ffi::String,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_TOPOLOGICALSORT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::String) -> Result<crate::transform::Pass>);
            typed(_0, _1)
        }

        static FUNC_RELAX_TRANSFORM_UPDATEPARAMSTRUCTINFO: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("relax.transform.UpdateParamStructInfo")
                    .expect("missing global function")
            });
        pub fn UpdateParamStructInfo(_0: tvm_ffi::Function) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_UPDATEPARAMSTRUCTINFO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_RELAX_TRANSFORM_UPDATEVDEVICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("relax.transform.UpdateVDevice").expect("missing global function")
        });
        pub fn UpdateVDevice(_0: crate::ir::VDevice, _1: i64) -> Result<crate::transform::Pass> {
            let func = &*FUNC_RELAX_TRANSFORM_UPDATEVDEVICE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::transform::Pass>);
            typed(_0.into(), _1)
        }
    }
}
pub mod script {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_SCRIPT_COMPLETE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("script.Complete").expect("missing global function"));
    pub fn Complete(
        _0: crate::tir::PrimFunc,
        _1: tvm_ffi::Array<crate::tir::Buffer>,
    ) -> Result<crate::tir::PrimFunc> {
        let func = &*FUNC_SCRIPT_COMPLETE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::tir::Buffer>,
            ) -> Result<crate::tir::PrimFunc>
        );
        typed(_0.into(), _1)
    }

    pub mod ir_builder {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERFRAMEENTER: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("script.ir_builder.IRBuilderFrameEnter")
                    .expect("missing global function")
            });
        pub fn IRBuilderFrameEnter(_0: crate::script::ir_builder::IRBuilderFrame) -> Result<()> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERFRAMEENTER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERFRAMEEXIT: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("script.ir_builder.IRBuilderFrameExit")
                    .expect("missing global function")
            });
        pub fn IRBuilderFrameExit(_0: crate::script::ir_builder::IRBuilderFrame) -> Result<()> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERFRAMEEXIT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERFRAMEADDCALLBACK: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("script.ir_builder.IRBuilderFrameAddCallback")
                    .expect("missing global function")
            });
        pub fn IRBuilderFrameAddCallback(
            _0: crate::script::ir_builder::IRBuilderFrame,
            _1: tvm_ffi::Function,
        ) -> Result<()> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERFRAMEADDCALLBACK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function) -> Result<()>);
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.ir_builder.IRBuilder").expect("missing global function")
        });
        pub fn IRBuilder() -> Result<crate::script::ir_builder::IRBuilder> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::IRBuilder>);
            typed()
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERENTER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.ir_builder.IRBuilderEnter")
                .expect("missing global function")
        });
        pub fn IRBuilderEnter(_0: crate::script::ir_builder::IRBuilder) -> Result<()> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERENTER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDEREXIT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.ir_builder.IRBuilderExit")
                .expect("missing global function")
        });
        pub fn IRBuilderExit(_0: crate::script::ir_builder::IRBuilder) -> Result<()> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDEREXIT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERCURRENT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.ir_builder.IRBuilderCurrent")
                .expect("missing global function")
        });
        pub fn IRBuilderCurrent() -> Result<crate::script::ir_builder::IRBuilder> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERCURRENT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::IRBuilder>);
            typed()
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERISINSCOPE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("script.ir_builder.IRBuilderIsInScope")
                    .expect("missing global function")
            });
        pub fn IRBuilderIsInScope() -> Result<bool> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERISINSCOPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<bool>);
            typed()
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERGET: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.ir_builder.IRBuilderGet").expect("missing global function")
        });
        pub fn IRBuilderGet(
            _0: crate::script::ir_builder::IRBuilder,
        ) -> Result<tvm_ffi::object::ObjectRef> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERGET;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::object::ObjectRef>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_IR_BUILDER_IRBUILDERNAME: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.ir_builder.IRBuilderName")
                .expect("missing global function")
        });
        pub fn IRBuilderName(
            _0: tvm_ffi::String,
            _1: tvm_ffi::object::ObjectRef,
        ) -> Result<tvm_ffi::object::ObjectRef> {
            let func = &*FUNC_SCRIPT_IR_BUILDER_IRBUILDERNAME;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::object::ObjectRef>);
            typed(_0, _1)
        }

        pub mod ir {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_SCRIPT_IR_BUILDER_IR_IRMODULE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.ir.IRModule")
                    .expect("missing global function")
            });
            pub fn IRModule() -> Result<crate::script::ir_builder::IRModuleFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_IRMODULE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::IRModuleFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_DECLFUNCTION: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.DeclFunction")
                        .expect("missing global function")
                });
            pub fn DeclFunction(
                _0: tvm_ffi::String,
                _1: crate::ir::BaseFunc,
            ) -> Result<crate::ir::GlobalVar> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_DECLFUNCTION;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::ir::GlobalVar>);
                typed(_0, _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_DEFFUNCTION: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.DefFunction")
                        .expect("missing global function")
                });
            pub fn DefFunction(_0: tvm_ffi::String, _1: crate::ir::BaseFunc) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_DEFFUNCTION;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<()>);
                typed(_0, _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_MODULEATTRS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.ModuleAttrs")
                        .expect("missing global function")
                });
            pub fn ModuleAttrs(
                _0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                _1: bool,
            ) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_MODULEATTRS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>, bool) -> Result<()>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_MODULEGETATTR: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.ModuleGetAttr")
                        .expect("missing global function")
                });
            pub fn ModuleGetAttr(
                _0: tvm_ffi::String,
            ) -> Result<Option<tvm_ffi::object::ObjectRef>> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_MODULEGETATTR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<Option<tvm_ffi::object::ObjectRef>>);
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_MODULESETATTR: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.ModuleSetAttr")
                        .expect("missing global function")
                });
            pub fn ModuleSetAttr(
                _0: tvm_ffi::String,
                _1: Option<tvm_ffi::object::ObjectRef>,
                _2: bool,
            ) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_MODULESETATTR;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::String, Option<tvm_ffi::object::ObjectRef>, bool) -> Result<()>
                );
                typed(_0, _1, _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_MODULEGLOBALINFOS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.ModuleGlobalInfos")
                        .expect("missing global function")
                });
            pub fn ModuleGlobalInfos(
                _0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
            ) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_MODULEGLOBALINFOS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
                    ) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_IR_LOOKUPVDEVICE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.ir.LookupVDevice")
                        .expect("missing global function")
                });
            pub fn LookupVDevice(_0: tvm_ffi::String, _1: i64) -> Result<crate::ir::VDevice> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_IR_LOOKUPVDEVICE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, i64) -> Result<crate::ir::VDevice>);
                typed(_0, _1)
            }
        }
        pub mod relax {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_SCRIPT_IR_BUILDER_RELAX_IF: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.relax.If").expect("missing global function")
            });
            pub fn If(
                _0: crate::ir::RelaxExpr,
            ) -> Result<crate::script::ir_builder::relax::IfFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_IF;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::relax::IfFrame>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_THEN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.relax.Then")
                    .expect("missing global function")
            });
            pub fn Then() -> Result<crate::script::ir_builder::relax::ThenFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_THEN;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::relax::ThenFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_ELSE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.relax.Else")
                    .expect("missing global function")
            });
            pub fn Else() -> Result<crate::script::ir_builder::relax::ElseFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_ELSE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::relax::ElseFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_SEQEXPR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.relax.SeqExpr")
                    .expect("missing global function")
            });
            pub fn SeqExpr() -> Result<crate::script::ir_builder::relax::SeqExprFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_SEQEXPR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::relax::SeqExprFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_EMIT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.relax.Emit")
                    .expect("missing global function")
            });
            pub fn Emit(
                _0: crate::ir::RelaxExpr,
                _1: Option<crate::ir::StructInfo>,
            ) -> Result<crate::relax::expr::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_EMIT;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::StructInfo>,
                    ) -> Result<crate::relax::expr::Var>
                );
                typed(_0.into(), _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_EMITMATCHCAST: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.EmitMatchCast")
                        .expect("missing global function")
                });
            pub fn EmitMatchCast(
                _0: crate::ir::RelaxExpr,
                _1: crate::ir::StructInfo,
            ) -> Result<crate::relax::expr::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_EMITMATCHCAST;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
                typed(_0.into(), _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_EMITVARBINDING: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.EmitVarBinding")
                        .expect("missing global function")
                });
            pub fn EmitVarBinding(
                _0: crate::relax::expr::VarBinding,
            ) -> Result<crate::relax::expr::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_EMITVARBINDING;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_DATAFLOW: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.Dataflow")
                        .expect("missing global function")
                });
            pub fn Dataflow() -> Result<crate::script::ir_builder::relax::BlockFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_DATAFLOW;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::relax::BlockFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_BINDINGBLOCK: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.BindingBlock")
                        .expect("missing global function")
                });
            pub fn BindingBlock() -> Result<crate::script::ir_builder::relax::BlockFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_BINDINGBLOCK;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::relax::BlockFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_DATAFLOWBLOCKOUTPUT: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.DataflowBlockOutput")
                        .expect("missing global function")
                });
            pub fn DataflowBlockOutput(_0: tvm_ffi::Array<crate::relax::expr::Var>) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_DATAFLOWBLOCKOUTPUT;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Array<crate::relax::expr::Var>) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCTION: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.Function")
                        .expect("missing global function")
                });
            pub fn Function(
                _0: crate::ir::IntImm,
                _1: crate::ir::IntImm,
            ) -> Result<crate::script::ir_builder::relax::FunctionFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCTION;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::relax::FunctionFrame>);
                typed(_0.into(), _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_ARG: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.relax.Arg")
                    .expect("missing global function")
            });
            pub fn Arg(
                _0: tvm_ffi::String,
                _1: crate::ir::StructInfo,
            ) -> Result<crate::relax::expr::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_ARG;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::relax::expr::Var>);
                typed(_0, _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCNAME: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.FuncName")
                        .expect("missing global function")
                });
            pub fn FuncName(_0: tvm_ffi::String) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCNAME;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<()>);
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCATTRS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.FuncAttrs")
                        .expect("missing global function")
                });
            pub fn FuncAttrs(_0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCATTRS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCRETSTRUCTINFO: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.FuncRetStructInfo")
                        .expect("missing global function")
                });
            pub fn FuncRetStructInfo(_0: crate::ir::StructInfo) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCRETSTRUCTINFO;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCRETVALUE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.relax.FuncRetValue")
                        .expect("missing global function")
                });
            pub fn FuncRetValue(_0: crate::ir::RelaxExpr) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_FUNCRETVALUE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
                typed(_0.into())
            }

            pub mod distributed {
                use std::sync::LazyLock;
                use tvm_ffi::{Any, AnyView, Function, Result};

                static FUNC_SCRIPT_IR_BUILDER_RELAX_DISTRIBUTED_CALL_TIR_DIST: LazyLock<Function> =
                    LazyLock::new(|| {
                        Function::get_global("script.ir_builder.relax.distributed.call_tir_dist")
                            .expect("missing global function")
                    });
                pub fn call_tir_dist(
                    _0: crate::ir::RelaxExpr,
                    _1: crate::relax::expr::Tuple,
                    _2: tvm_ffi::Array<crate::relax::DTensorStructInfo>,
                    _3: Option<crate::ir::RelaxExpr>,
                ) -> Result<crate::ir::RelaxExpr> {
                    let func = &*FUNC_SCRIPT_IR_BUILDER_RELAX_DISTRIBUTED_CALL_TIR_DIST;
                    let typed = tvm_ffi::into_typed_fn!(
                        func.clone(),
                        Fn(
                            tvm_ffi::object::ObjectRef,
                            tvm_ffi::object::ObjectRef,
                            tvm_ffi::Array<crate::relax::DTensorStructInfo>,
                            Option<crate::ir::RelaxExpr>,
                        ) -> Result<crate::ir::RelaxExpr>
                    );
                    typed(_0.into(), _1.into(), _2, _3)
                }
            }
        }
        pub mod tir {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, Function, Result};

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.BFloat16")
                    .expect("missing global function")
            });
            pub fn BFloat16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8")
                    .expect("missing global function")
            });
            pub fn Float8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float16")
                    .expect("missing global function")
            });
            pub fn Float16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float32")
                    .expect("missing global function")
            });
            pub fn Float32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float64")
                    .expect("missing global function")
            });
            pub fn Float64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8")
                    .expect("missing global function")
            });
            pub fn UInt8(_0: Option<crate::ir::PrimExpr>, _1: bool) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16")
                    .expect("missing global function")
            });
            pub fn UInt16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32")
                    .expect("missing global function")
            });
            pub fn UInt32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64")
                    .expect("missing global function")
            });
            pub fn UInt64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8").expect("missing global function")
            });
            pub fn Int8(_0: Option<crate::ir::PrimExpr>, _1: bool) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16")
                    .expect("missing global function")
            });
            pub fn Int16(_0: Option<crate::ir::PrimExpr>, _1: bool) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32")
                    .expect("missing global function")
            });
            pub fn Int32(_0: Option<crate::ir::PrimExpr>, _1: bool) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64")
                    .expect("missing global function")
            });
            pub fn Int64(_0: Option<crate::ir::PrimExpr>, _1: bool) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8x2")
                    .expect("missing global function")
            });
            pub fn Float8x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8x4")
                    .expect("missing global function")
            });
            pub fn Float8x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8x8")
                    .expect("missing global function")
            });
            pub fn Float8x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8x16")
                    .expect("missing global function")
            });
            pub fn Float8x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8x32")
                    .expect("missing global function")
            });
            pub fn Float8x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float8x64")
                    .expect("missing global function")
            });
            pub fn Float8x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float16x2")
                    .expect("missing global function")
            });
            pub fn Float16x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float16x4")
                    .expect("missing global function")
            });
            pub fn Float16x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float16x8")
                    .expect("missing global function")
            });
            pub fn Float16x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float16x16")
                        .expect("missing global function")
                });
            pub fn Float16x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float16x32")
                        .expect("missing global function")
                });
            pub fn Float16x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float16x64")
                        .expect("missing global function")
                });
            pub fn Float16x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT16X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float32x2")
                    .expect("missing global function")
            });
            pub fn Float32x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float32x4")
                    .expect("missing global function")
            });
            pub fn Float32x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float32x8")
                    .expect("missing global function")
            });
            pub fn Float32x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float32x16")
                        .expect("missing global function")
                });
            pub fn Float32x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float32x32")
                        .expect("missing global function")
                });
            pub fn Float32x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float32x64")
                        .expect("missing global function")
                });
            pub fn Float32x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT32X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float64x2")
                    .expect("missing global function")
            });
            pub fn Float64x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float64x4")
                    .expect("missing global function")
            });
            pub fn Float64x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Float64x8")
                    .expect("missing global function")
            });
            pub fn Float64x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float64x16")
                        .expect("missing global function")
                });
            pub fn Float64x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float64x32")
                        .expect("missing global function")
                });
            pub fn Float64x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float64x64")
                        .expect("missing global function")
                });
            pub fn Float64x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT64X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8x2")
                    .expect("missing global function")
            });
            pub fn UInt8x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8x4")
                    .expect("missing global function")
            });
            pub fn UInt8x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8x8")
                    .expect("missing global function")
            });
            pub fn UInt8x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8x16")
                    .expect("missing global function")
            });
            pub fn UInt8x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8x32")
                    .expect("missing global function")
            });
            pub fn UInt8x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt8x64")
                    .expect("missing global function")
            });
            pub fn UInt8x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT8X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16x2")
                    .expect("missing global function")
            });
            pub fn UInt16x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16x4")
                    .expect("missing global function")
            });
            pub fn UInt16x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16x8")
                    .expect("missing global function")
            });
            pub fn UInt16x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16x16")
                    .expect("missing global function")
            });
            pub fn UInt16x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16x32")
                    .expect("missing global function")
            });
            pub fn UInt16x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt16x64")
                    .expect("missing global function")
            });
            pub fn UInt16x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT16X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32x2")
                    .expect("missing global function")
            });
            pub fn UInt32x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32x4")
                    .expect("missing global function")
            });
            pub fn UInt32x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32x8")
                    .expect("missing global function")
            });
            pub fn UInt32x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32x16")
                    .expect("missing global function")
            });
            pub fn UInt32x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32x32")
                    .expect("missing global function")
            });
            pub fn UInt32x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt32x64")
                    .expect("missing global function")
            });
            pub fn UInt32x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT32X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64x2")
                    .expect("missing global function")
            });
            pub fn UInt64x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64x4")
                    .expect("missing global function")
            });
            pub fn UInt64x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64x8")
                    .expect("missing global function")
            });
            pub fn UInt64x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64x16")
                    .expect("missing global function")
            });
            pub fn UInt64x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64x32")
                    .expect("missing global function")
            });
            pub fn UInt64x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.UInt64x64")
                    .expect("missing global function")
            });
            pub fn UInt64x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UINT64X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8x2")
                    .expect("missing global function")
            });
            pub fn Int8x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8x4")
                    .expect("missing global function")
            });
            pub fn Int8x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8x8")
                    .expect("missing global function")
            });
            pub fn Int8x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8x16")
                    .expect("missing global function")
            });
            pub fn Int8x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8x32")
                    .expect("missing global function")
            });
            pub fn Int8x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT8X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int8x64")
                    .expect("missing global function")
            });
            pub fn Int8x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT8X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16x2")
                    .expect("missing global function")
            });
            pub fn Int16x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16x4")
                    .expect("missing global function")
            });
            pub fn Int16x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16x8")
                    .expect("missing global function")
            });
            pub fn Int16x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16x16")
                    .expect("missing global function")
            });
            pub fn Int16x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16x32")
                    .expect("missing global function")
            });
            pub fn Int16x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT16X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int16x64")
                    .expect("missing global function")
            });
            pub fn Int16x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT16X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32x2")
                    .expect("missing global function")
            });
            pub fn Int32x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32x4")
                    .expect("missing global function")
            });
            pub fn Int32x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32x8")
                    .expect("missing global function")
            });
            pub fn Int32x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32x16")
                    .expect("missing global function")
            });
            pub fn Int32x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32x32")
                    .expect("missing global function")
            });
            pub fn Int32x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT32X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int32x64")
                    .expect("missing global function")
            });
            pub fn Int32x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT32X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64X2: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64x2")
                    .expect("missing global function")
            });
            pub fn Int64x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64X4: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64x4")
                    .expect("missing global function")
            });
            pub fn Int64x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64X8: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64x8")
                    .expect("missing global function")
            });
            pub fn Int64x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64X16: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64x16")
                    .expect("missing global function")
            });
            pub fn Int64x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64X32: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64x32")
                    .expect("missing global function")
            });
            pub fn Int64x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INT64X64: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Int64x64")
                    .expect("missing global function")
            });
            pub fn Int64x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INT64X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BFloat16x2")
                        .expect("missing global function")
                });
            pub fn BFloat16x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BFloat16x4")
                        .expect("missing global function")
                });
            pub fn BFloat16x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BFloat16x8")
                        .expect("missing global function")
                });
            pub fn BFloat16x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BFloat16x16")
                        .expect("missing global function")
                });
            pub fn BFloat16x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BFloat16x32")
                        .expect("missing global function")
                });
            pub fn BFloat16x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BFloat16x64")
                        .expect("missing global function")
                });
            pub fn BFloat16x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BFLOAT16X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4")
                        .expect("missing global function")
                });
            pub fn Float8E3M4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4x2")
                        .expect("missing global function")
                });
            pub fn Float8E3M4x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4x4")
                        .expect("missing global function")
                });
            pub fn Float8E3M4x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4x8")
                        .expect("missing global function")
                });
            pub fn Float8E3M4x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4x16")
                        .expect("missing global function")
                });
            pub fn Float8E3M4x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4x32")
                        .expect("missing global function")
                });
            pub fn Float8E3M4x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E3M4x64")
                        .expect("missing global function")
                });
            pub fn Float8E3M4x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E3M4X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3")
                        .expect("missing global function")
                });
            pub fn Float8E4M3(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3x2")
                        .expect("missing global function")
                });
            pub fn Float8E4M3x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3x4")
                        .expect("missing global function")
                });
            pub fn Float8E4M3x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3x8")
                        .expect("missing global function")
                });
            pub fn Float8E4M3x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3x16")
                        .expect("missing global function")
                });
            pub fn Float8E4M3x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3x32")
                        .expect("missing global function")
                });
            pub fn Float8E4M3x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3x64")
                        .expect("missing global function")
                });
            pub fn Float8E4M3x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZ: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZ")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZ(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZ;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZx2")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZx4")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZx8")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZx16")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZx32")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3B11FNUZx64")
                        .expect("missing global function")
                });
            pub fn Float8E4M3B11FNUZx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3B11FNUZX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FN: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FN")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FN(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FN;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNx2")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNx4")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNx8")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNx16")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNx32")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNx64")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZ: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZ")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZ(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZ;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZx2")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZx4")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZx8")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZx16")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZx32")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E4M3FNUZx64")
                        .expect("missing global function")
                });
            pub fn Float8E4M3FNUZx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E4M3FNUZX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2")
                        .expect("missing global function")
                });
            pub fn Float8E5M2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2x2")
                        .expect("missing global function")
                });
            pub fn Float8E5M2x2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2x4")
                        .expect("missing global function")
                });
            pub fn Float8E5M2x4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2x8")
                        .expect("missing global function")
                });
            pub fn Float8E5M2x8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2x16")
                        .expect("missing global function")
                });
            pub fn Float8E5M2x16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2x32")
                        .expect("missing global function")
                });
            pub fn Float8E5M2x32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2x64")
                        .expect("missing global function")
                });
            pub fn Float8E5M2x64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2X64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZ: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZ")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZ(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZ;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZx2")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZx4")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZx8")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZx16")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZx32")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E5M2FNUZx64")
                        .expect("missing global function")
                });
            pub fn Float8E5M2FNUZx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E5M2FNUZX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNU: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNU")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNU(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNU;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNUx2")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNUx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNUx4")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNUx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNUx8")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNUx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNUx16")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNUx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNUx32")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNUx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float8E8M0FNUx64")
                        .expect("missing global function")
                });
            pub fn Float8E8M0FNUx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT8E8M0FNUX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FN: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FN")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FN(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FN;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FNx2")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FNx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FNx4")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FNx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FNx8")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FNx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FNx16")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FNx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FNx32")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FNx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E2M3FNx64")
                        .expect("missing global function")
                });
            pub fn Float6E2M3FNx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E2M3FNX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FN: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FN")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FN(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FN;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FNx2")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FNx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FNx4")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FNx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FNx8")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FNx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FNx16")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FNx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FNx32")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FNx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float6E3M2FNx64")
                        .expect("missing global function")
                });
            pub fn Float6E3M2FNx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT6E3M2FNX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FN: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FN")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FN(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FN;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX2: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FNx2")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FNx2(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX2;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX4: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FNx4")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FNx4(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX4;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX8: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FNx8")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FNx8(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX8;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX16: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FNx16")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FNx16(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX16;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX32: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FNx32")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FNx32(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX32;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX64: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Float4E2M1FNx64")
                        .expect("missing global function")
                });
            pub fn Float4E2M1FNx64(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FLOAT4E2M1FNX64;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BOOLEAN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Boolean")
                    .expect("missing global function")
            });
            pub fn Boolean(
                _0: Option<crate::ir::PrimExpr>,
                _1: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BOOLEAN;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_HANDLE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Handle")
                    .expect("missing global function")
            });
            pub fn Handle(
                _0: tvm_ffi::DLDataType,
                _1: tvm_ffi::String,
                _2: bool,
                _3: bool,
            ) -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_HANDLE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::String, bool, bool) -> Result<crate::tir::Var>);
                typed(_0, _1, _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_TENSORMAPHANDLE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.TensormapHandle")
                        .expect("missing global function")
                });
            pub fn TensormapHandle() -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_TENSORMAPHANDLE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tir::Var>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_VOID: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Void").expect("missing global function")
            });
            pub fn Void(_0: Option<crate::ir::PrimExpr>, _1: bool) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_VOID;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(Option<crate::ir::PrimExpr>, bool) -> Result<crate::ir::PrimExpr>
                );
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_MIN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.min").expect("missing global function")
            });
            pub fn min(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_MIN;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
                typed(_0.into(), _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_MAX: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.max").expect("missing global function")
            });
            pub fn max(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_MAX;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
                typed(_0.into(), _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BUFFER: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Buffer")
                    .expect("missing global function")
            });
            pub fn Buffer(
                _0: tvm_ffi::Array<crate::ir::PrimExpr>,
                _1: tvm_ffi::DLDataType,
                _2: tvm_ffi::String,
                _3: Option<crate::tir::Var>,
                _4: Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
                _5: Option<crate::ir::PrimExpr>,
                _6: tvm_ffi::String,
                _7: i64,
                _8: i64,
                _9: tvm_ffi::String,
                _10: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            ) -> Result<crate::tir::Buffer> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BUFFER;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::DLDataType,
                        tvm_ffi::String,
                        Option<crate::tir::Var>,
                        Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
                        Option<crate::ir::PrimExpr>,
                        tvm_ffi::String,
                        i64,
                        i64,
                        tvm_ffi::String,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    ) -> Result<crate::tir::Buffer>
                );
                typed(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_PRIMFUNC: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.PrimFunc")
                    .expect("missing global function")
            });
            pub fn PrimFunc(_0: bool) -> Result<crate::script::ir_builder::tir::PrimFuncFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_PRIMFUNC;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::script::ir_builder::tir::PrimFuncFrame>);
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ARG: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Arg").expect("missing global function")
            });
            pub fn Arg(
                _0: tvm_ffi::String,
                _1: tvm_ffi::object::ObjectRef,
            ) -> Result<tvm_ffi::object::ObjectRef> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ARG;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::object::ObjectRef>);
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FUNCNAME: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.FuncName")
                    .expect("missing global function")
            });
            pub fn FuncName(_0: tvm_ffi::String) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FUNCNAME;
                let typed =
                    tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<()>);
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FUNCATTRS: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.FuncAttrs")
                    .expect("missing global function")
            });
            pub fn FuncAttrs(_0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FUNCATTRS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_FUNCRET: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.FuncRet")
                    .expect("missing global function")
            });
            pub fn FuncRet(_0: crate::ir::Type) -> Result<crate::ir::Type> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_FUNCRET;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::Type>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_MATCHBUFFER: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.MatchBuffer")
                        .expect("missing global function")
                });
            pub fn MatchBuffer(
                _0: tvm_ffi::object::ObjectRef,
                _1: tvm_ffi::Array<crate::ir::PrimExpr>,
                _2: tvm_ffi::DLDataType,
                _3: Option<crate::tir::Var>,
                _4: tvm_ffi::Array<crate::ir::PrimExpr>,
                _5: crate::ir::PrimExpr,
                _6: tvm_ffi::String,
                _7: i64,
                _8: i64,
                _9: tvm_ffi::String,
                _10: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            ) -> Result<crate::tir::Buffer> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_MATCHBUFFER;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::DLDataType,
                        Option<crate::tir::Var>,
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::String,
                        i64,
                        i64,
                        tvm_ffi::String,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    ) -> Result<crate::tir::Buffer>
                );
                typed(_0, _1, _2, _3, _4, _5.into(), _6, _7, _8, _9, _10)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BLOCK: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Block")
                    .expect("missing global function")
            });
            pub fn Block(
                _0: tvm_ffi::String,
                _1: bool,
            ) -> Result<crate::script::ir_builder::tir::BlockFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BLOCK;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, bool) -> Result<crate::script::ir_builder::tir::BlockFrame>);
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_INIT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Init").expect("missing global function")
            });
            pub fn Init() -> Result<crate::script::ir_builder::tir::BlockInitFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_INIT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::tir::BlockInitFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_WHERE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Where")
                    .expect("missing global function")
            });
            pub fn Where(_0: crate::ir::PrimExpr) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_WHERE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_READS: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Reads")
                    .expect("missing global function")
            });
            pub fn Reads(_0: tvm_ffi::Array<tvm_ffi::object::ObjectRef>) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_READS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Array<tvm_ffi::object::ObjectRef>) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_WRITES: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Writes")
                    .expect("missing global function")
            });
            pub fn Writes(_0: tvm_ffi::Array<tvm_ffi::object::ObjectRef>) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_WRITES;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Array<tvm_ffi::object::ObjectRef>) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BLOCKATTRS: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BlockAttrs")
                        .expect("missing global function")
                });
            pub fn BlockAttrs(_0: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BLOCKATTRS;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>) -> Result<()>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ALLOCBUFFER: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.AllocBuffer")
                        .expect("missing global function")
                });
            pub fn AllocBuffer(
                _0: tvm_ffi::Array<crate::ir::PrimExpr>,
                _1: tvm_ffi::DLDataType,
                _2: Option<crate::tir::Var>,
                _3: tvm_ffi::Array<crate::ir::PrimExpr>,
                _4: crate::ir::PrimExpr,
                _5: tvm_ffi::String,
                _6: i64,
                _7: i64,
                _8: tvm_ffi::String,
                _9: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            ) -> Result<crate::tir::Buffer> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ALLOCBUFFER;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::DLDataType,
                        Option<crate::tir::Var>,
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::String,
                        i64,
                        i64,
                        tvm_ffi::String,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    ) -> Result<crate::tir::Buffer>
                );
                typed(_0, _1, _2, _3, _4.into(), _5, _6, _7, _8, _9)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_AXISSPATIAL: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.AxisSpatial")
                        .expect("missing global function")
                });
            pub fn AxisSpatial(
                _0: crate::ir::Range,
                _1: crate::ir::PrimExpr,
                _2: tvm_ffi::DLDataType,
            ) -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_AXISSPATIAL;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::tir::Var>);
                typed(_0.into(), _1.into(), _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_AXISREDUCE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.AxisReduce")
                        .expect("missing global function")
                });
            pub fn AxisReduce(
                _0: crate::ir::Range,
                _1: crate::ir::PrimExpr,
                _2: tvm_ffi::DLDataType,
            ) -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_AXISREDUCE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::tir::Var>);
                typed(_0.into(), _1.into(), _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_AXISSCAN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.AxisScan")
                    .expect("missing global function")
            });
            pub fn AxisScan(
                _0: crate::ir::Range,
                _1: crate::ir::PrimExpr,
                _2: tvm_ffi::DLDataType,
            ) -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_AXISSCAN;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::tir::Var>);
                typed(_0.into(), _1.into(), _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_AXISOPAQUE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.AxisOpaque")
                        .expect("missing global function")
                });
            pub fn AxisOpaque(
                _0: crate::ir::Range,
                _1: crate::ir::PrimExpr,
                _2: tvm_ffi::DLDataType,
            ) -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_AXISOPAQUE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::DLDataType) -> Result<crate::tir::Var>);
                typed(_0.into(), _1.into(), _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_AXISREMAP: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.AxisRemap")
                    .expect("missing global function")
            });
            pub fn AxisRemap(
                _0: tvm_ffi::String,
                _1: tvm_ffi::Array<crate::ir::PrimExpr>,
                _2: tvm_ffi::DLDataType,
            ) -> Result<tvm_ffi::Array<crate::tir::Var>> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_AXISREMAP;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::String,
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::DLDataType,
                    ) -> Result<tvm_ffi::Array<crate::tir::Var>>
                );
                typed(_0, _1, _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_SERIAL: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Serial")
                    .expect("missing global function")
            });
            pub fn Serial(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
                _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                _3: Option<crate::ir::PrimExpr>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_SERIAL;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                        Option<crate::ir::PrimExpr>,
                    ) -> Result<crate::script::ir_builder::tir::ForFrame>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_PARALLEL: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Parallel")
                    .expect("missing global function")
            });
            pub fn Parallel(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
                _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                _3: Option<crate::ir::PrimExpr>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_PARALLEL;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                        Option<crate::ir::PrimExpr>,
                    ) -> Result<crate::script::ir_builder::tir::ForFrame>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_VECTORIZED: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.Vectorized")
                        .expect("missing global function")
                });
            pub fn Vectorized(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
                _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                _3: Option<crate::ir::PrimExpr>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_VECTORIZED;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                        Option<crate::ir::PrimExpr>,
                    ) -> Result<crate::script::ir_builder::tir::ForFrame>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_UNROLL: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Unroll")
                    .expect("missing global function")
            });
            pub fn Unroll(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
                _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                _3: Option<crate::ir::PrimExpr>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_UNROLL;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                        Option<crate::ir::PrimExpr>,
                    ) -> Result<crate::script::ir_builder::tir::ForFrame>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_THREADBINDING: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.ThreadBinding")
                        .expect("missing global function")
                });
            pub fn ThreadBinding(
                _0: crate::ir::PrimExpr,
                _1: crate::ir::PrimExpr,
                _2: tvm_ffi::String,
                _3: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_THREADBINDING;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::String,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                    ) -> Result<crate::script::ir_builder::tir::ForFrame>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_GRID: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Grid").expect("missing global function")
            });
            pub fn Grid(
                _0: tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_GRID;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                    ) -> Result<crate::script::ir_builder::tir::ForFrame>
                );
                typed(_0)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ASSERT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Assert")
                    .expect("missing global function")
            });
            pub fn Assert(
                _0: crate::ir::PrimExpr,
                _1: tvm_ffi::String,
            ) -> Result<crate::script::ir_builder::tir::AssertFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ASSERT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::script::ir_builder::tir::AssertFrame>);
                typed(_0.into(), _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_LETSTMT: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.LetStmt")
                    .expect("missing global function")
            });
            pub fn LetStmt(
                _0: crate::ir::PrimExpr,
                _1: Option<crate::ir::Type>,
                _2: Option<crate::tir::Var>,
            ) -> Result<crate::script::ir_builder::tir::LetFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_LETSTMT;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        Option<crate::ir::Type>,
                        Option<crate::tir::Var>,
                    ) -> Result<crate::script::ir_builder::tir::LetFrame>
                );
                typed(_0.into(), _1, _2)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_LEGACYLETSTMT: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.LegacyLetStmt")
                        .expect("missing global function")
                });
            pub fn LegacyLetStmt(
                _0: crate::tir::Var,
                _1: crate::ir::PrimExpr,
            ) -> Result<crate::script::ir_builder::tir::LetFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_LEGACYLETSTMT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::tir::LetFrame>);
                typed(_0.into(), _1.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ALLOCATE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Allocate")
                    .expect("missing global function")
            });
            pub fn Allocate(
                _0: tvm_ffi::Array<crate::ir::PrimExpr>,
                _1: tvm_ffi::DLDataType,
                _2: tvm_ffi::String,
                _3: Option<crate::ir::PrimExpr>,
                _4: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
            ) -> Result<crate::script::ir_builder::tir::AllocateFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ALLOCATE;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::DLDataType,
                        tvm_ffi::String,
                        Option<crate::ir::PrimExpr>,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                    )
                        -> Result<crate::script::ir_builder::tir::AllocateFrame>
                );
                typed(_0, _1, _2, _3, _4)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ALLOCATECONST: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.AllocateConst")
                        .expect("missing global function")
                });
            pub fn AllocateConst(
                _0: tvm_ffi::Tensor,
                _1: tvm_ffi::DLDataType,
                _2: tvm_ffi::Array<crate::ir::PrimExpr>,
                _3: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
            ) -> Result<crate::script::ir_builder::tir::AllocateConstFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ALLOCATECONST;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Tensor,
                        tvm_ffi::DLDataType,
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                    )
                        -> Result<crate::script::ir_builder::tir::AllocateConstFrame>
                );
                typed(_0, _1, _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_REALIZE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Realize")
                    .expect("missing global function")
            });
            pub fn Realize(
                _0: crate::tir::BufferRegion,
                _1: tvm_ffi::String,
                _2: crate::ir::PrimExpr,
            ) -> Result<crate::script::ir_builder::tir::RealizeFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_REALIZE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::tir::RealizeFrame>);
                typed(_0.into(), _1, _2.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ATTR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Attr").expect("missing global function")
            });
            pub fn Attr(
                _0: tvm_ffi::AnyValue,
                _1: tvm_ffi::String,
                _2: crate::ir::PrimExpr,
            ) -> Result<crate::script::ir_builder::tir::AttrFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ATTR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::AnyValue, tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::tir::AttrFrame>);
                typed(_0, _1, _2.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_WHILE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.While")
                    .expect("missing global function")
            });
            pub fn While(
                _0: crate::ir::PrimExpr,
            ) -> Result<crate::script::ir_builder::tir::WhileFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_WHILE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::tir::WhileFrame>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_IF: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.If").expect("missing global function")
            });
            pub fn If(_0: crate::ir::PrimExpr) -> Result<crate::script::ir_builder::tir::IfFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_IF;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::script::ir_builder::tir::IfFrame>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_THEN: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Then").expect("missing global function")
            });
            pub fn Then() -> Result<crate::script::ir_builder::tir::ThenFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_THEN;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::tir::ThenFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ELSE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Else").expect("missing global function")
            });
            pub fn Else() -> Result<crate::script::ir_builder::tir::ElseFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ELSE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::script::ir_builder::tir::ElseFrame>);
                typed()
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_DECLBUFFER: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.DeclBuffer")
                        .expect("missing global function")
                });
            pub fn DeclBuffer(
                _0: tvm_ffi::Array<crate::ir::PrimExpr>,
                _1: tvm_ffi::DLDataType,
                _2: tvm_ffi::String,
                _3: Option<crate::tir::Var>,
                _4: Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
                _5: Option<crate::ir::PrimExpr>,
                _6: tvm_ffi::String,
                _7: i64,
                _8: i64,
                _9: tvm_ffi::String,
                _10: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            ) -> Result<crate::script::ir_builder::tir::DeclBufferFrame> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_DECLBUFFER;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        tvm_ffi::DLDataType,
                        tvm_ffi::String,
                        Option<crate::tir::Var>,
                        Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
                        Option<crate::ir::PrimExpr>,
                        tvm_ffi::String,
                        i64,
                        i64,
                        tvm_ffi::String,
                        Option<tvm_ffi::Array<crate::ir::IntImm>>,
                    )
                        -> Result<crate::script::ir_builder::tir::DeclBufferFrame>
                );
                typed(_0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREAD: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.LaunchThread")
                        .expect("missing global function")
                });
            pub fn LaunchThread(args: &[Any]) -> Result<Any> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREAD;
                let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
                func.call_packed(&views)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_ENVTHREAD: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.EnvThread")
                    .expect("missing global function")
            });
            pub fn EnvThread(
                _0: tvm_ffi::String,
                _1: tvm_ffi::DLDataType,
            ) -> Result<crate::tir::Var> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_ENVTHREAD;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::DLDataType) -> Result<crate::tir::Var>);
                typed(_0, _1)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_BUFFERSTORE: LazyLock<Function> =
                LazyLock::new(|| {
                    Function::get_global("script.ir_builder.tir.BufferStore")
                        .expect("missing global function")
                });
            pub fn BufferStore(
                _0: crate::tir::Buffer,
                _1: crate::ir::PrimExpr,
                _2: tvm_ffi::Array<crate::ir::PrimExpr>,
                _3: Option<crate::ir::PrimExpr>,
            ) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_BUFFERSTORE;
                let typed = tvm_ffi::into_typed_fn!(
                    func.clone(),
                    Fn(
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::object::ObjectRef,
                        tvm_ffi::Array<crate::ir::PrimExpr>,
                        Option<crate::ir::PrimExpr>,
                    ) -> Result<()>
                );
                typed(_0.into(), _1.into(), _2, _3)
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_EVALUATE: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Evaluate")
                    .expect("missing global function")
            });
            pub fn Evaluate(_0: crate::ir::PrimExpr) -> Result<()> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_EVALUATE;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
                typed(_0.into())
            }

            static FUNC_SCRIPT_IR_BUILDER_TIR_PTR: LazyLock<Function> = LazyLock::new(|| {
                Function::get_global("script.ir_builder.tir.Ptr").expect("missing global function")
            });
            pub fn Ptr(
                _0: tvm_ffi::DLDataType,
                _1: tvm_ffi::String,
                _2: bool,
            ) -> Result<crate::ir::PrimExpr> {
                let func = &*FUNC_SCRIPT_IR_BUILDER_TIR_PTR;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::String, bool) -> Result<crate::ir::PrimExpr>);
                typed(_0, _1, _2)
            }
        }
    }
    pub mod printer {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_SCRIPT_PRINTER_DOCSTRINGDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.DocStringDoc").expect("missing global function")
        });
        pub fn DocStringDoc(_0: tvm_ffi::String) -> Result<crate::script::printer::DocStringDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_DOCSTRINGDOC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::script::printer::DocStringDoc>);
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_COMMENTDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.CommentDoc").expect("missing global function")
        });
        pub fn CommentDoc(_0: tvm_ffi::String) -> Result<crate::script::printer::CommentDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_COMMENTDOC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::script::printer::CommentDoc>);
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_CLASSDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ClassDoc").expect("missing global function")
        });
        pub fn ClassDoc(
            _0: crate::script::printer::IdDoc,
            _1: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _2: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::ClassDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_CLASSDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::ClassDoc>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_SCRIPT_PRINTER_FUNCTIONDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.FunctionDoc").expect("missing global function")
        });
        pub fn FunctionDoc(
            _0: crate::script::printer::IdDoc,
            _1: tvm_ffi::Array<crate::script::printer::AssignDoc>,
            _2: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _3: Option<crate::script::printer::ExprDoc>,
            _4: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::FunctionDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_FUNCTIONDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::AssignDoc>,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                    Option<crate::script::printer::ExprDoc>,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::FunctionDoc>
            );
            typed(_0.into(), _1, _2, _3, _4)
        }

        static FUNC_SCRIPT_PRINTER_RETURNDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ReturnDoc").expect("missing global function")
        });
        pub fn ReturnDoc(
            _0: crate::script::printer::ExprDoc,
        ) -> Result<crate::script::printer::ReturnDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_RETURNDOC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::script::printer::ReturnDoc>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_PRINTER_ASSERTDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.AssertDoc").expect("missing global function")
        });
        pub fn AssertDoc(
            _0: crate::script::printer::ExprDoc,
            _1: Option<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::AssertDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_ASSERTDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::AssertDoc>
            );
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_EXPRSTMTDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ExprStmtDoc").expect("missing global function")
        });
        pub fn ExprStmtDoc(
            _0: crate::script::printer::ExprDoc,
        ) -> Result<crate::script::printer::ExprStmtDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_EXPRSTMTDOC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::script::printer::ExprStmtDoc>);
            typed(_0.into())
        }

        static FUNC_SCRIPT_PRINTER_SCOPEDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ScopeDoc").expect("missing global function")
        });
        pub fn ScopeDoc(
            _0: Option<crate::script::printer::ExprDoc>,
            _1: crate::script::printer::ExprDoc,
            _2: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::ScopeDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_SCOPEDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    Option<crate::script::printer::ExprDoc>,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::ScopeDoc>
            );
            typed(_0, _1.into(), _2)
        }

        static FUNC_SCRIPT_PRINTER_FORDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ForDoc").expect("missing global function")
        });
        pub fn ForDoc(
            _0: crate::script::printer::ExprDoc,
            _1: crate::script::printer::ExprDoc,
            _2: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::ForDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_FORDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::ForDoc>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_SCRIPT_PRINTER_WHILEDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.WhileDoc").expect("missing global function")
        });
        pub fn WhileDoc(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::WhileDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_WHILEDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::WhileDoc>
            );
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_IFDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.IfDoc").expect("missing global function")
        });
        pub fn IfDoc(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _2: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::IfDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_IFDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::IfDoc>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_SCRIPT_PRINTER_ASSIGNDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.AssignDoc").expect("missing global function")
        });
        pub fn AssignDoc(
            _0: crate::script::printer::ExprDoc,
            _1: Option<crate::script::printer::ExprDoc>,
            _2: Option<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::AssignDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_ASSIGNDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    Option<crate::script::printer::ExprDoc>,
                    Option<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::AssignDoc>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_SCRIPT_PRINTER_SLICEDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.SliceDoc").expect("missing global function")
        });
        pub fn SliceDoc(
            _0: Option<crate::script::printer::ExprDoc>,
            _1: Option<crate::script::printer::ExprDoc>,
            _2: Option<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::SliceDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_SLICEDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    Option<crate::script::printer::ExprDoc>,
                    Option<crate::script::printer::ExprDoc>,
                    Option<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::SliceDoc>
            );
            typed(_0, _1, _2)
        }

        static FUNC_SCRIPT_PRINTER_DICTDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.DictDoc").expect("missing global function")
        });
        pub fn DictDoc(
            _0: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _1: tvm_ffi::Array<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::DictDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_DICTDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::DictDoc>
            );
            typed(_0, _1)
        }

        static FUNC_SCRIPT_PRINTER_LISTDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ListDoc").expect("missing global function")
        });
        pub fn ListDoc(
            _0: tvm_ffi::Array<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::ListDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LISTDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::ListDoc>
            );
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_TUPLEDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.TupleDoc").expect("missing global function")
        });
        pub fn TupleDoc(
            _0: tvm_ffi::Array<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::TupleDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_TUPLEDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::TupleDoc>
            );
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_LAMBDADOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.LambdaDoc").expect("missing global function")
        });
        pub fn LambdaDoc(
            _0: tvm_ffi::Array<crate::script::printer::IdDoc>,
            _1: crate::script::printer::ExprDoc,
        ) -> Result<crate::script::printer::LambdaDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LAMBDADOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::script::printer::IdDoc>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<crate::script::printer::LambdaDoc>
            );
            typed(_0, _1.into())
        }

        static FUNC_SCRIPT_PRINTER_OPERATIONDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.OperationDoc").expect("missing global function")
        });
        pub fn OperationDoc(
            _0: i64,
            _1: tvm_ffi::Array<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::OperationDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_OPERATIONDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    i64,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::OperationDoc>
            );
            typed(_0, _1)
        }

        static FUNC_SCRIPT_PRINTER_CALLDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.CallDoc").expect("missing global function")
        });
        pub fn CallDoc(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _2: tvm_ffi::Array<tvm_ffi::String>,
            _3: tvm_ffi::Array<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::CallDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_CALLDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                    tvm_ffi::Array<tvm_ffi::String>,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::CallDoc>
            );
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_SCRIPT_PRINTER_INDEXDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.IndexDoc").expect("missing global function")
        });
        pub fn IndexDoc(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::Array<crate::script::printer::Doc>,
        ) -> Result<crate::script::printer::IndexDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_INDEXDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::Doc>,
                ) -> Result<crate::script::printer::IndexDoc>
            );
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_ATTRACCESSDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.AttrAccessDoc").expect("missing global function")
        });
        pub fn AttrAccessDoc(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::String,
        ) -> Result<crate::script::printer::AttrAccessDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_ATTRACCESSDOC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::script::printer::AttrAccessDoc>);
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_IDDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.IdDoc").expect("missing global function")
        });
        pub fn IdDoc(_0: tvm_ffi::String) -> Result<crate::script::printer::IdDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_IDDOC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::script::printer::IdDoc>);
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_STMTBLOCKDOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.StmtBlockDoc").expect("missing global function")
        });
        pub fn StmtBlockDoc(
            _0: tvm_ffi::Array<crate::script::printer::StmtDoc>,
        ) -> Result<crate::script::printer::StmtBlockDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_STMTBLOCKDOC;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Array<crate::script::printer::StmtDoc>,
                ) -> Result<crate::script::printer::StmtBlockDoc>
            );
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_STMTDOCSETCOMMENT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.StmtDocSetComment")
                .expect("missing global function")
        });
        pub fn StmtDocSetComment(
            _0: crate::script::printer::StmtDoc,
            _1: Option<tvm_ffi::String>,
        ) -> Result<()> {
            let func = &*FUNC_SCRIPT_PRINTER_STMTDOCSETCOMMENT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, Option<tvm_ffi::String>) -> Result<()>
            );
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_DOCSETSOURCEPATHS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.DocSetSourcePaths")
                .expect("missing global function")
        });
        pub fn DocSetSourcePaths(
            _0: crate::script::printer::Doc,
            _1: tvm_ffi::Array<crate::ffi::reflection::AccessPath>,
        ) -> Result<()> {
            let func = &*FUNC_SCRIPT_PRINTER_DOCSETSOURCEPATHS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ffi::reflection::AccessPath>,
                ) -> Result<()>
            );
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_EXPRDOCATTR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ExprDocAttr").expect("missing global function")
        });
        pub fn ExprDocAttr(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::String,
        ) -> Result<crate::script::printer::ExprDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_EXPRDOCATTR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::script::printer::ExprDoc>);
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_EXPRDOCINDEX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ExprDocIndex").expect("missing global function")
        });
        pub fn ExprDocIndex(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::Array<crate::script::printer::Doc>,
        ) -> Result<crate::script::printer::ExprDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_EXPRDOCINDEX;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::Doc>,
                ) -> Result<crate::script::printer::ExprDoc>
            );
            typed(_0.into(), _1)
        }

        static FUNC_SCRIPT_PRINTER_EXPRDOCCALL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ExprDocCall").expect("missing global function")
        });
        pub fn ExprDocCall(
            _0: crate::script::printer::ExprDoc,
            _1: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _2: tvm_ffi::Array<tvm_ffi::String>,
            _3: tvm_ffi::Array<crate::script::printer::ExprDoc>,
        ) -> Result<crate::script::printer::ExprDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_EXPRDOCCALL;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                    tvm_ffi::Array<tvm_ffi::String>,
                    tvm_ffi::Array<crate::script::printer::ExprDoc>,
                ) -> Result<crate::script::printer::ExprDoc>
            );
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_SCRIPT_PRINTER_LITERALDOCNONE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.LiteralDocNone").expect("missing global function")
        });
        pub fn LiteralDocNone(
            _0: Option<crate::ffi::reflection::AccessPath>,
        ) -> Result<crate::script::printer::LiteralDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LITERALDOCNONE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    Option<crate::ffi::reflection::AccessPath>,
                ) -> Result<crate::script::printer::LiteralDoc>
            );
            typed(_0)
        }

        static FUNC_SCRIPT_PRINTER_LITERALDOCINT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.LiteralDocInt").expect("missing global function")
        });
        pub fn LiteralDocInt(
            _0: i64,
            _1: Option<crate::ffi::reflection::AccessPath>,
        ) -> Result<crate::script::printer::LiteralDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LITERALDOCINT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    i64,
                    Option<crate::ffi::reflection::AccessPath>,
                ) -> Result<crate::script::printer::LiteralDoc>
            );
            typed(_0, _1)
        }

        static FUNC_SCRIPT_PRINTER_LITERALDOCBOOLEAN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.LiteralDocBoolean")
                .expect("missing global function")
        });
        pub fn LiteralDocBoolean(
            _0: bool,
            _1: Option<crate::ffi::reflection::AccessPath>,
        ) -> Result<crate::script::printer::LiteralDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LITERALDOCBOOLEAN;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    bool,
                    Option<crate::ffi::reflection::AccessPath>,
                ) -> Result<crate::script::printer::LiteralDoc>
            );
            typed(_0, _1)
        }

        static FUNC_SCRIPT_PRINTER_LITERALDOCFLOAT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.LiteralDocFloat").expect("missing global function")
        });
        pub fn LiteralDocFloat(
            _0: f64,
            _1: Option<crate::ffi::reflection::AccessPath>,
        ) -> Result<crate::script::printer::LiteralDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LITERALDOCFLOAT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    f64,
                    Option<crate::ffi::reflection::AccessPath>,
                ) -> Result<crate::script::printer::LiteralDoc>
            );
            typed(_0, _1)
        }

        static FUNC_SCRIPT_PRINTER_LITERALDOCSTR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.LiteralDocStr").expect("missing global function")
        });
        pub fn LiteralDocStr(
            _0: tvm_ffi::String,
            _1: Option<crate::ffi::reflection::AccessPath>,
        ) -> Result<crate::script::printer::LiteralDoc> {
            let func = &*FUNC_SCRIPT_PRINTER_LITERALDOCSTR;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::String,
                    Option<crate::ffi::reflection::AccessPath>,
                ) -> Result<crate::script::printer::LiteralDoc>
            );
            typed(_0, _1)
        }

        static FUNC_SCRIPT_PRINTER_DOCTOPYTHONSCRIPT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.DocToPythonScript")
                .expect("missing global function")
        });
        pub fn DocToPythonScript(
            _0: crate::script::printer::Doc,
            _1: crate::script::PrinterConfig,
        ) -> Result<tvm_ffi::String> {
            let func = &*FUNC_SCRIPT_PRINTER_DOCTOPYTHONSCRIPT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::String>);
            typed(_0.into(), _1.into())
        }

        static FUNC_SCRIPT_PRINTER_REPRPRINTRELAX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("script.printer.ReprPrintRelax").expect("missing global function")
        });
        pub fn ReprPrintRelax(
            _0: tvm_ffi::object::ObjectRef,
            _1: crate::script::PrinterConfig,
        ) -> Result<tvm_ffi::String> {
            let func = &*FUNC_SCRIPT_PRINTER_REPRPRINTRELAX;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::String>);
            typed(_0, _1.into())
        }
    }
}
pub mod target {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_TARGET_BUILD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("target.Build").expect("missing global function"));
    pub fn Build(_0: crate::ir::IRModule, _1: crate::target::Target) -> Result<tvm_ffi::Module> {
        let func = &*FUNC_TARGET_BUILD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TARGET_TARGETTAGLISTTAGS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetTagListTags").expect("missing global function")
    });
    pub fn TargetTagListTags() -> Result<tvm_ffi::Map<tvm_ffi::String, crate::target::Target>> {
        let func = &*FUNC_TARGET_TARGETTAGLISTTAGS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<tvm_ffi::Map<tvm_ffi::String, crate::target::Target>>);
        typed()
    }

    static FUNC_TARGET_TARGETTAGADDTAG: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetTagAddTag").expect("missing global function")
    });
    pub fn TargetTagAddTag(
        _0: tvm_ffi::String,
        _1: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        _2: bool,
    ) -> Result<crate::target::Target> {
        let func = &*FUNC_TARGET_TARGETTAGADDTAG;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                bool,
            ) -> Result<crate::target::Target>
        );
        typed(_0, _1, _2)
    }

    static FUNC_TARGET_TARGET: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("target.Target").expect("missing global function"));
    pub fn Target(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TARGET_TARGET;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TARGET_TARGETENTERSCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetEnterScope").expect("missing global function")
    });
    pub fn TargetEnterScope(_0: crate::target::Target) -> Result<()> {
        let func = &*FUNC_TARGET_TARGETENTERSCOPE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_TARGET_TARGETEXITSCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetExitScope").expect("missing global function")
    });
    pub fn TargetExitScope(_0: crate::target::Target) -> Result<()> {
        let func = &*FUNC_TARGET_TARGETEXITSCOPE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_TARGET_TARGETCURRENT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetCurrent").expect("missing global function")
    });
    pub fn TargetCurrent(_0: bool) -> Result<crate::target::Target> {
        let func = &*FUNC_TARGET_TARGETCURRENT;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::target::Target>);
        typed(_0)
    }

    static FUNC_TARGET_TARGETEXPORT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetExport").expect("missing global function")
    });
    pub fn TargetExport(
        _0: crate::target::Target,
    ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
        let func = &*FUNC_TARGET_TARGETEXPORT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>);
        typed(_0.into())
    }

    static FUNC_TARGET_WITHHOST: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("target.WithHost").expect("missing global function"));
    pub fn WithHost(
        _0: crate::target::Target,
        _1: crate::target::Target,
    ) -> Result<crate::target::Target> {
        let func = &*FUNC_TARGET_WITHHOST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::target::Target>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TARGET_TARGETGETDEVICETYPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetGetDeviceType").expect("missing global function")
    });
    pub fn TargetGetDeviceType(_0: crate::target::Target) -> Result<i64> {
        let func = &*FUNC_TARGET_TARGETGETDEVICETYPE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into())
    }

    static FUNC_TARGET_TARGETGETFEATURE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetGetFeature").expect("missing global function")
    });
    pub fn TargetGetFeature(
        _0: crate::target::Target,
        _1: tvm_ffi::String,
    ) -> Result<tvm_ffi::AnyValue> {
        let func = &*FUNC_TARGET_TARGETGETFEATURE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<tvm_ffi::AnyValue>);
        typed(_0.into(), _1)
    }

    static FUNC_TARGET_TARGETKINDGETATTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.TargetKindGetAttr").expect("missing global function")
    });
    pub fn TargetKindGetAttr(
        _0: crate::target::TargetKind,
        _1: tvm_ffi::String,
    ) -> Result<tvm_ffi::AnyValue> {
        let func = &*FUNC_TARGET_TARGETKINDGETATTR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<tvm_ffi::AnyValue>);
        typed(_0.into(), _1)
    }

    static FUNC_TARGET_LISTTARGETKINDS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.ListTargetKinds").expect("missing global function")
    });
    pub fn ListTargetKinds() -> Result<tvm_ffi::Array<tvm_ffi::String>> {
        let func = &*FUNC_TARGET_LISTTARGETKINDS;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<tvm_ffi::Array<tvm_ffi::String>>);
        typed()
    }

    static FUNC_TARGET_LISTTARGETKINDOPTIONS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.ListTargetKindOptions").expect("missing global function")
    });
    pub fn ListTargetKindOptions(
        _0: crate::target::TargetKind,
    ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::String>> {
        let func = &*FUNC_TARGET_LISTTARGETKINDOPTIONS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::String>>);
        typed(_0.into())
    }

    static FUNC_TARGET_LISTTARGETKINDOPTIONSFROMNAME: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("target.ListTargetKindOptionsFromName")
            .expect("missing global function")
    });
    pub fn ListTargetKindOptionsFromName(
        _0: tvm_ffi::String,
    ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::String>> {
        let func = &*FUNC_TARGET_LISTTARGETKINDOPTIONSFROMNAME;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::String>>);
        typed(_0)
    }

    static FUNC_TARGET_VIRTUALDEVICE_FORDEVICETARGETANDMEMORYSCOPE: LazyLock<Function> =
        LazyLock::new(|| {
            Function::get_global("target.VirtualDevice_ForDeviceTargetAndMemoryScope")
                .expect("missing global function")
        });
    pub fn VirtualDevice_ForDeviceTargetAndMemoryScope(
        _0: tvm_ffi::DLDevice,
        _1: crate::target::Target,
        _2: tvm_ffi::String,
    ) -> Result<crate::target::VirtualDevice> {
        let func = &*FUNC_TARGET_VIRTUALDEVICE_FORDEVICETARGETANDMEMORYSCOPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDevice, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<crate::target::VirtualDevice>);
        typed(_0, _1.into(), _2)
    }

    pub mod build {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_TARGET_BUILD_C: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.c").expect("missing global function")
        });
        pub fn c(_0: crate::ir::IRModule, _1: crate::target::Target) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_C;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_METAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.metal").expect("missing global function")
        });
        pub fn metal(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_METAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_OPENCL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.opencl").expect("missing global function")
        });
        pub fn opencl(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_OPENCL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_WEBGPU: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.webgpu").expect("missing global function")
        });
        pub fn webgpu(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_WEBGPU;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_CUDA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.cuda").expect("missing global function")
        });
        pub fn cuda(_0: crate::ir::IRModule, _1: crate::target::Target) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_CUDA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_TILELANG_C: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.tilelang_c").expect("missing global function")
        });
        pub fn tilelang_c(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_TILELANG_C;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_TILELANG_CPP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.tilelang_cpp").expect("missing global function")
        });
        pub fn tilelang_cpp(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_TILELANG_CPP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_TILELANG_CUDA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("target.build.tilelang_cuda").expect("missing global function")
        });
        pub fn tilelang_cuda(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_TILELANG_CUDA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_TILELANG_CUDA_WITHOUT_COMPILE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("target.build.tilelang_cuda_without_compile")
                    .expect("missing global function")
            });
        pub fn tilelang_cuda_without_compile(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_TILELANG_CUDA_WITHOUT_COMPILE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TARGET_BUILD_TILELANG_CUTEDSL_WITHOUT_COMPILE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("target.build.tilelang_cutedsl_without_compile")
                    .expect("missing global function")
            });
        pub fn tilelang_cutedsl_without_compile(
            _0: crate::ir::IRModule,
            _1: crate::target::Target,
        ) -> Result<tvm_ffi::Module> {
            let func = &*FUNC_TARGET_BUILD_TILELANG_CUTEDSL_WITHOUT_COMPILE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Module>);
            typed(_0.into(), _1.into())
        }
    }
}
pub mod te {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_TE_COMPUTEOP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.ComputeOp").expect("missing global function"));
    pub fn ComputeOp(
        _0: tvm_ffi::String,
        _1: tvm_ffi::String,
        _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
        _3: tvm_ffi::Array<crate::tir::IterVar>,
        _4: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<crate::te::ComputeOp> {
        let func = &*FUNC_TE_COMPUTEOP;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                tvm_ffi::String,
                Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                tvm_ffi::Array<crate::tir::IterVar>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<crate::te::ComputeOp>
        );
        typed(_0, _1, _2, _3, _4)
    }

    static FUNC_TE_CREATEPRIMFUNC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("te.CreatePrimFunc").expect("missing global function")
    });
    pub fn CreatePrimFunc(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TE_CREATEPRIMFUNC;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TE_EXTERNOP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.ExternOp").expect("missing global function"));
    pub fn ExternOp(
        _0: tvm_ffi::String,
        _1: tvm_ffi::String,
        _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
        _3: tvm_ffi::Array<crate::te::Tensor>,
        _4: tvm_ffi::Array<crate::tir::Buffer>,
        _5: tvm_ffi::Array<crate::tir::Buffer>,
        _6: crate::tir::Stmt,
    ) -> Result<crate::te::ExternOp> {
        let func = &*FUNC_TE_EXTERNOP;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                tvm_ffi::String,
                Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                tvm_ffi::Array<crate::te::Tensor>,
                tvm_ffi::Array<crate::tir::Buffer>,
                tvm_ffi::Array<crate::tir::Buffer>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::te::ExternOp>
        );
        typed(_0, _1, _2, _3, _4, _5, _6.into())
    }

    static FUNC_TE_PLACEHOLDER: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.Placeholder").expect("missing global function"));
    pub fn Placeholder(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TE_PLACEHOLDER;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TE_SCANOP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.ScanOp").expect("missing global function"));
    pub fn ScanOp(
        _0: tvm_ffi::String,
        _1: tvm_ffi::String,
        _2: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
        _3: crate::tir::IterVar,
        _4: tvm_ffi::Array<crate::te::Tensor>,
        _5: tvm_ffi::Array<crate::te::Tensor>,
        _6: tvm_ffi::Array<crate::te::Tensor>,
        _7: tvm_ffi::Array<crate::te::Tensor>,
    ) -> Result<crate::te::ScanOp> {
        let func = &*FUNC_TE_SCANOP;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::String,
                tvm_ffi::String,
                Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::te::Tensor>,
                tvm_ffi::Array<crate::te::Tensor>,
                tvm_ffi::Array<crate::te::Tensor>,
                tvm_ffi::Array<crate::te::Tensor>,
            ) -> Result<crate::te::ScanOp>
        );
        typed(_0, _1, _2, _3.into(), _4, _5, _6, _7)
    }

    static FUNC_TE_TENSOR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.Tensor").expect("missing global function"));
    pub fn Tensor(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: tvm_ffi::DLDataType,
        _2: crate::te::Operation,
        _3: i64,
    ) -> Result<crate::te::Tensor> {
        let func = &*FUNC_TE_TENSOR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::DLDataType,
                tvm_ffi::object::ObjectRef,
                i64,
            ) -> Result<crate::te::Tensor>
        );
        typed(_0, _1, _2.into(), _3)
    }

    static FUNC_TE_TENSOREQUAL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.TensorEqual").expect("missing global function"));
    pub fn TensorEqual(_0: crate::te::Tensor, _1: crate::te::Tensor) -> Result<bool> {
        let func = &*FUNC_TE_TENSOREQUAL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TE_TENSORHASH: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.TensorHash").expect("missing global function"));
    pub fn TensorHash(_0: crate::te::Tensor) -> Result<i64> {
        let func = &*FUNC_TE_TENSORHASH;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into())
    }

    static FUNC_TE_OPGETOUTPUT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.OpGetOutput").expect("missing global function"));
    pub fn OpGetOutput(_0: crate::te::Operation, _1: i64) -> Result<crate::te::Tensor> {
        let func = &*FUNC_TE_OPGETOUTPUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::te::Tensor>);
        typed(_0.into(), _1)
    }

    static FUNC_TE_OPNUMOUTPUTS: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("te.OpNumOutputs").expect("missing global function"));
    pub fn OpNumOutputs(_0: crate::te::Operation) -> Result<i64> {
        let func = &*FUNC_TE_OPNUMOUTPUTS;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into())
    }

    static FUNC_TE_OPINPUTTENSORS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("te.OpInputTensors").expect("missing global function")
    });
    pub fn OpInputTensors(_0: crate::te::Operation) -> Result<tvm_ffi::Array<crate::te::Tensor>> {
        let func = &*FUNC_TE_OPINPUTTENSORS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::te::Tensor>>);
        typed(_0.into())
    }
}
pub mod tir {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_TIR_BLOCKDEPENDENCEINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BlockDependenceInfo").expect("missing global function")
    });
    pub fn BlockDependenceInfo(_0: crate::ir::IRModule) -> Result<crate::tir::BlockDependenceInfo> {
        let func = &*FUNC_TIR_BLOCKDEPENDENCEINFO;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockDependenceInfo>);
        typed(_0.into())
    }

    static FUNC_TIR_BLOCKDEPENDENCEINFOGETBLOCKSCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BlockDependenceInfoGetBlockScope")
            .expect("missing global function")
    });
    pub fn BlockDependenceInfoGetBlockScope(
        _0: crate::tir::BlockDependenceInfo,
        _1: crate::tir::StmtSRef,
    ) -> Result<crate::tir::BlockScope> {
        let func = &*FUNC_TIR_BLOCKDEPENDENCEINFOGETBLOCKSCOPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockScope>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_BLOCKDEPENDENCEINFOGETSREF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BlockDependenceInfoGetSRef").expect("missing global function")
    });
    pub fn BlockDependenceInfoGetSRef(
        _0: crate::tir::BlockDependenceInfo,
        _1: crate::tir::Stmt,
    ) -> Result<Option<crate::tir::StmtSRef>> {
        let func = &*FUNC_TIR_BLOCKDEPENDENCEINFOGETSREF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::StmtSRef>>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_STMTSREFSTMT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.StmtSRefStmt").expect("missing global function")
    });
    pub fn StmtSRefStmt(_0: crate::tir::StmtSRef) -> Result<Option<crate::tir::Stmt>> {
        let func = &*FUNC_TIR_STMTSREFSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::Stmt>>);
        typed(_0.into())
    }

    static FUNC_TIR_STMTSREFPARENT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.StmtSRefParent").expect("missing global function")
    });
    pub fn StmtSRefParent(_0: crate::tir::StmtSRef) -> Result<Option<crate::tir::StmtSRef>> {
        let func = &*FUNC_TIR_STMTSREFPARENT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::StmtSRef>>);
        typed(_0.into())
    }

    static FUNC_TIR_STMTSREFROOTMARK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.StmtSRefRootMark").expect("missing global function")
    });
    pub fn StmtSRefRootMark() -> Result<crate::tir::StmtSRef> {
        let func = &*FUNC_TIR_STMTSREFROOTMARK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tir::StmtSRef>);
        typed()
    }

    static FUNC_TIR_STMTSREFINLINEMARK: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.StmtSRefInlineMark").expect("missing global function")
    });
    pub fn StmtSRefInlineMark() -> Result<crate::tir::StmtSRef> {
        let func = &*FUNC_TIR_STMTSREFINLINEMARK;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tir::StmtSRef>);
        typed()
    }

    static FUNC_TIR_BLOCKSCOPEGETDEPSBYSRC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BlockScopeGetDepsBySrc").expect("missing global function")
    });
    pub fn BlockScopeGetDepsBySrc(
        _0: crate::tir::BlockScope,
        _1: crate::tir::StmtSRef,
    ) -> Result<tvm_ffi::Array<crate::tir::Dependency>> {
        let func = &*FUNC_TIR_BLOCKSCOPEGETDEPSBYSRC;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::Dependency>>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_BLOCKSCOPEGETDEPSBYDST: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BlockScopeGetDepsByDst").expect("missing global function")
    });
    pub fn BlockScopeGetDepsByDst(
        _0: crate::tir::BlockScope,
        _1: crate::tir::StmtSRef,
    ) -> Result<tvm_ffi::Array<crate::tir::Dependency>> {
        let func = &*FUNC_TIR_BLOCKSCOPEGETDEPSBYDST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::Dependency>>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_BUFFER: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Buffer").expect("missing global function"));
    pub fn Buffer(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_BUFFER;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_BUFFERACCESSPTR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferAccessPtr").expect("missing global function")
    });
    pub fn BufferAccessPtr(
        _0: crate::tir::Buffer,
        _1: i64,
        _2: tvm_ffi::DLDataType,
        _3: i64,
        _4: crate::ir::PrimExpr,
        _5: Option<crate::ir::PrimExpr>,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_BUFFERACCESSPTR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                i64,
                tvm_ffi::DLDataType,
                i64,
                tvm_ffi::object::ObjectRef,
                Option<crate::ir::PrimExpr>,
            ) -> Result<crate::ir::PrimExpr>
        );
        typed(_0.into(), _1, _2, _3, _4.into(), _5)
    }

    static FUNC_TIR_BUFFERGETFLATTENEDBUFFER: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferGetFlattenedBuffer").expect("missing global function")
    });
    pub fn BufferGetFlattenedBuffer(_0: crate::tir::Buffer) -> Result<crate::tir::Buffer> {
        let func = &*FUNC_TIR_BUFFERGETFLATTENEDBUFFER;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tir::Buffer>);
        typed(_0.into())
    }

    static FUNC_TIR_BUFFEROFFSETOF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferOffsetOf").expect("missing global function")
    });
    pub fn BufferOffsetOf(
        _0: crate::tir::Buffer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_BUFFEROFFSETOF;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_BUFFERVLOAD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.BufferVLoad").expect("missing global function"));
    pub fn BufferVLoad(
        _0: crate::tir::Buffer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: tvm_ffi::DLDataType,
        _3: Option<crate::ir::PrimExpr>,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_BUFFERVLOAD;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::DLDataType,
                Option<crate::ir::PrimExpr>,
            ) -> Result<crate::ir::PrimExpr>
        );
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_TIR_BUFFERVSTORE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferVStore").expect("missing global function")
    });
    pub fn BufferVStore(
        _0: crate::tir::Buffer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: crate::ir::PrimExpr,
        _3: Option<crate::ir::PrimExpr>,
    ) -> Result<crate::tir::Stmt> {
        let func = &*FUNC_TIR_BUFFERVSTORE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                Option<crate::ir::PrimExpr>,
            ) -> Result<crate::tir::Stmt>
        );
        typed(_0.into(), _1, _2.into(), _3)
    }

    static FUNC_TIR_BUFFERSTORAGESCOPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferStorageScope").expect("missing global function")
    });
    pub fn BufferStorageScope(_0: crate::tir::Buffer) -> Result<tvm_ffi::String> {
        let func = &*FUNC_TIR_BUFFERSTORAGESCOPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::String>);
        typed(_0.into())
    }

    static FUNC_TIR_LAYOUT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Layout").expect("missing global function"));
    pub fn Layout(_0: tvm_ffi::String, _1: tvm_ffi::DLDataType) -> Result<crate::tir::Layout> {
        let func = &*FUNC_TIR_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::DLDataType) -> Result<crate::tir::Layout>);
        typed(_0, _1)
    }

    static FUNC_TIR_LAYOUTINDEXOF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.LayoutIndexOf").expect("missing global function")
    });
    pub fn LayoutIndexOf(_0: crate::tir::Layout, _1: tvm_ffi::String) -> Result<i64> {
        let func = &*FUNC_TIR_LAYOUTINDEXOF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<i64>);
        typed(_0.into(), _1)
    }

    static FUNC_TIR_LAYOUTFACTOROF: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.LayoutFactorOf").expect("missing global function")
    });
    pub fn LayoutFactorOf(_0: crate::tir::Layout, _1: tvm_ffi::String) -> Result<i64> {
        let func = &*FUNC_TIR_LAYOUTFACTOROF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<i64>);
        typed(_0.into(), _1)
    }

    static FUNC_TIR_LAYOUTNDIM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.LayoutNdim").expect("missing global function"));
    pub fn LayoutNdim(_0: crate::tir::Layout) -> Result<i64> {
        let func = &*FUNC_TIR_LAYOUTNDIM;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into())
    }

    static FUNC_TIR_LAYOUTGETITEM: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.LayoutGetItem").expect("missing global function")
    });
    pub fn LayoutGetItem(_0: crate::tir::Layout, _1: i64) -> Result<tvm_ffi::String> {
        let func = &*FUNC_TIR_LAYOUTGETITEM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<tvm_ffi::String>);
        typed(_0.into(), _1)
    }

    static FUNC_TIR_BIJECTIVELAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BijectiveLayout").expect("missing global function")
    });
    pub fn BijectiveLayout(
        _0: crate::tir::Layout,
        _1: crate::tir::Layout,
    ) -> Result<crate::tir::BijectiveLayout> {
        let func = &*FUNC_TIR_BIJECTIVELAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BijectiveLayout>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_BIJECTIVELAYOUTFORWARDINDEX: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BijectiveLayoutForwardIndex").expect("missing global function")
    });
    pub fn BijectiveLayoutForwardIndex(
        _0: crate::tir::BijectiveLayout,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_BIJECTIVELAYOUTFORWARDINDEX;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_BIJECTIVELAYOUTBACKWARDINDEX: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BijectiveLayoutBackwardIndex").expect("missing global function")
    });
    pub fn BijectiveLayoutBackwardIndex(
        _0: crate::tir::BijectiveLayout,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_BIJECTIVELAYOUTBACKWARDINDEX;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_BIJECTIVELAYOUTFORWARDSHAPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BijectiveLayoutForwardShape").expect("missing global function")
    });
    pub fn BijectiveLayoutForwardShape(
        _0: crate::tir::BijectiveLayout,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_BIJECTIVELAYOUTFORWARDSHAPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_BIJECTIVELAYOUTBACKWARDSHAPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BijectiveLayoutBackwardShape").expect("missing global function")
    });
    pub fn BijectiveLayoutBackwardShape(
        _0: crate::tir::BijectiveLayout,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_BIJECTIVELAYOUTBACKWARDSHAPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_PRODUCERLOAD: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.ProducerLoad").expect("missing global function")
    });
    pub fn ProducerLoad(
        _0: crate::tir::DataProducer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::ProducerLoad> {
        let func = &*FUNC_TIR_PRODUCERLOAD;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::ProducerLoad>
        );
        typed(_0.into(), _1, _2.into())
    }

    static FUNC_TIR_BUFFERLOAD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.BufferLoad").expect("missing global function"));
    pub fn BufferLoad(
        _0: crate::tir::Buffer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: Option<crate::ir::PrimExpr>,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::BufferLoad> {
        let func = &*FUNC_TIR_BUFFERLOAD;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                Option<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::BufferLoad>
        );
        typed(_0.into(), _1, _2, _3.into())
    }

    static FUNC_TIR_REDUCE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Reduce").expect("missing global function"));
    pub fn Reduce(
        _0: crate::tir::CommReducer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: tvm_ffi::Array<crate::tir::IterVar>,
        _3: crate::ir::PrimExpr,
        _4: i64,
        _5: tvm_ffi::Array<crate::ir::PrimExpr>,
        _6: crate::ir::Span,
    ) -> Result<crate::tir::Reduce> {
        let func = &*FUNC_TIR_REDUCE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Array<crate::tir::IterVar>,
                tvm_ffi::object::ObjectRef,
                i64,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::Reduce>
        );
        typed(_0.into(), _1, _2, _3.into(), _4, _5, _6.into())
    }

    static FUNC_TIR_SHUFFLE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Shuffle").expect("missing global function"));
    pub fn Shuffle(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Shuffle> {
        let func = &*FUNC_TIR_SHUFFLE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::Shuffle>
        );
        typed(_0, _1, _2.into())
    }

    static FUNC_TIR_CALL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Call").expect("missing global function"));
    pub fn Call(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_CALL;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_LET: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Let").expect("missing global function"));
    pub fn Let(
        _0: crate::tir::Var,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::PrimExpr,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::Let> {
        let func = &*FUNC_TIR_LET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Let>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_BROADCAST: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Broadcast").expect("missing global function"));
    pub fn Broadcast(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Broadcast> {
        let func = &*FUNC_TIR_BROADCAST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Broadcast>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_RAMP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Ramp").expect("missing global function"));
    pub fn Ramp(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::PrimExpr,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::Ramp> {
        let func = &*FUNC_TIR_RAMP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Ramp>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_SELECT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Select").expect("missing global function"));
    pub fn Select(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::PrimExpr,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::Select> {
        let func = &*FUNC_TIR_SELECT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Select>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_NOT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Not").expect("missing global function"));
    pub fn Not(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::tir::Not> {
        let func = &*FUNC_TIR_NOT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Not>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_OR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Or").expect("missing global function"));
    pub fn Or(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Or> {
        let func = &*FUNC_TIR_OR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Or>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_AND: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.And").expect("missing global function"));
    pub fn And(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::And> {
        let func = &*FUNC_TIR_AND;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::And>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_GE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.GE").expect("missing global function"));
    pub fn GE(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::GE> {
        let func = &*FUNC_TIR_GE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::GE>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_GT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.GT").expect("missing global function"));
    pub fn GT(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::GT> {
        let func = &*FUNC_TIR_GT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::GT>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_LE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.LE").expect("missing global function"));
    pub fn LE(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::LE> {
        let func = &*FUNC_TIR_LE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::LE>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_LT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.LT").expect("missing global function"));
    pub fn LT(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::LT> {
        let func = &*FUNC_TIR_LT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::LT>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_NE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.NE").expect("missing global function"));
    pub fn NE(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::NE> {
        let func = &*FUNC_TIR_NE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::NE>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_EQ: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.EQ").expect("missing global function"));
    pub fn EQ(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::EQ> {
        let func = &*FUNC_TIR_EQ;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::EQ>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_MAX: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Max").expect("missing global function"));
    pub fn Max(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Max> {
        let func = &*FUNC_TIR_MAX;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Max>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_MIN: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Min").expect("missing global function"));
    pub fn Min(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Min> {
        let func = &*FUNC_TIR_MIN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Min>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_FLOORMOD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.FloorMod").expect("missing global function"));
    pub fn FloorMod(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::FloorMod> {
        let func = &*FUNC_TIR_FLOORMOD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::FloorMod>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_FLOORDIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.FloorDiv").expect("missing global function"));
    pub fn FloorDiv(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::FloorDiv> {
        let func = &*FUNC_TIR_FLOORDIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::FloorDiv>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_MOD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Mod").expect("missing global function"));
    pub fn Mod(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Mod> {
        let func = &*FUNC_TIR_MOD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Mod>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_DIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Div").expect("missing global function"));
    pub fn Div(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Div> {
        let func = &*FUNC_TIR_DIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Div>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_MUL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Mul").expect("missing global function"));
    pub fn Mul(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Mul> {
        let func = &*FUNC_TIR_MUL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Mul>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_SUB: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Sub").expect("missing global function"));
    pub fn Sub(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Sub> {
        let func = &*FUNC_TIR_SUB;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Sub>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_ADD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Add").expect("missing global function"));
    pub fn Add(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Add> {
        let func = &*FUNC_TIR_ADD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Add>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_CAST: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Cast").expect("missing global function"));
    pub fn Cast(
        _0: tvm_ffi::DLDataType,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Cast> {
        let func = &*FUNC_TIR_CAST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Cast>);
        typed(_0, _1.into(), _2.into())
    }

    static FUNC_TIR_STRINGIMM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.StringImm").expect("missing global function"));
    pub fn StringImm(_0: tvm_ffi::String, _1: crate::ir::Span) -> Result<crate::tir::StringImm> {
        let func = &*FUNC_TIR_STRINGIMM;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::tir::StringImm>);
        typed(_0, _1.into())
    }

    static FUNC_TIR_ITERVAR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.IterVar").expect("missing global function"));
    pub fn IterVar(
        _0: crate::ir::Range,
        _1: crate::tir::Var,
        _2: i64,
        _3: tvm_ffi::String,
        _4: crate::ir::Span,
    ) -> Result<crate::tir::IterVar> {
        let func = &*FUNC_TIR_ITERVAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::tir::IterVar>);
        typed(_0.into(), _1.into(), _2, _3, _4.into())
    }

    static FUNC_TIR_SIZEVAR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.SizeVar").expect("missing global function"));
    pub fn SizeVar(
        _0: tvm_ffi::String,
        _1: tvm_ffi::DLDataType,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::SizeVar> {
        let func = &*FUNC_TIR_SIZEVAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::tir::SizeVar>);
        typed(_0, _1, _2.into())
    }

    static FUNC_TIR_VAR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Var").expect("missing global function"));
    pub fn Var(
        _0: tvm_ffi::String,
        _1: tvm_ffi::AnyValue,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::Var> {
        let func = &*FUNC_TIR_VAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::AnyValue, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Var>);
        typed(_0, _1, _2.into())
    }

    static FUNC_TIR_CONVERT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.convert").expect("missing global function"));
    pub fn convert(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_CONVERT;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_COMMREDUCER: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.CommReducer").expect("missing global function"));
    pub fn CommReducer(
        _0: tvm_ffi::Array<crate::tir::Var>,
        _1: tvm_ffi::Array<crate::tir::Var>,
        _2: tvm_ffi::Array<crate::ir::PrimExpr>,
        _3: tvm_ffi::Array<crate::ir::PrimExpr>,
        _4: crate::ir::Span,
    ) -> Result<crate::tir::CommReducer> {
        let func = &*FUNC_TIR_COMMREDUCER;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::Var>,
                tvm_ffi::Array<crate::tir::Var>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::CommReducer>
        );
        typed(_0, _1, _2, _3, _4.into())
    }

    static FUNC_TIR_COMMREDUCERCOMBINE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.CommReducerCombine").expect("missing global function")
    });
    pub fn CommReducerCombine(
        _0: crate::tir::CommReducer,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_COMMREDUCERCOMBINE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1, _2)
    }

    static FUNC_TIR_PRIMFUNC: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.PrimFunc").expect("missing global function"));
    pub fn PrimFunc(
        _0: tvm_ffi::Array<crate::tir::Var>,
        _1: crate::tir::Stmt,
        _2: crate::ir::Type,
        _3: tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
        _4: crate::ir::DictAttrs,
        _5: crate::ir::Span,
    ) -> Result<crate::tir::PrimFunc> {
        let func = &*FUNC_TIR_PRIMFUNC;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::Var>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::PrimFunc>
        );
        typed(_0, _1.into(), _2.into(), _3, _4.into(), _5.into())
    }

    static FUNC_TIR_TENSORINTRIN: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.TensorIntrin").expect("missing global function")
    });
    pub fn TensorIntrin(
        _0: crate::tir::PrimFunc,
        _1: crate::tir::PrimFunc,
    ) -> Result<crate::tir::TensorIntrin> {
        let func = &*FUNC_TIR_TENSORINTRIN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::TensorIntrin>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_TENSORINTRINREGISTER: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.TensorIntrinRegister").expect("missing global function")
    });
    pub fn TensorIntrinRegister(
        _0: tvm_ffi::String,
        _1: crate::tir::TensorIntrin,
        _2: bool,
    ) -> Result<()> {
        let func = &*FUNC_TIR_TENSORINTRINREGISTER;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, tvm_ffi::object::ObjectRef, bool) -> Result<()>);
        typed(_0, _1.into(), _2)
    }

    static FUNC_TIR_TENSORINTRINGET: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.TensorIntrinGet").expect("missing global function")
    });
    pub fn TensorIntrinGet(
        _0: tvm_ffi::String,
        _1: bool,
    ) -> Result<Option<crate::tir::TensorIntrin>> {
        let func = &*FUNC_TIR_TENSORINTRINGET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String, bool) -> Result<Option<crate::tir::TensorIntrin>>);
        typed(_0, _1)
    }

    static FUNC_TIR_INDEXMAP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.IndexMap").expect("missing global function"));
    pub fn IndexMap(
        _0: tvm_ffi::Array<crate::tir::Var>,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: Option<crate::tir::IndexMap>,
    ) -> Result<crate::tir::IndexMap> {
        let func = &*FUNC_TIR_INDEXMAP;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::Var>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                Option<crate::tir::IndexMap>,
            ) -> Result<crate::tir::IndexMap>
        );
        typed(_0, _1, _2)
    }

    static FUNC_TIR_INDEXMAPMAPINDICES: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.IndexMapMapIndices").expect("missing global function")
    });
    pub fn IndexMapMapIndices(
        _0: crate::tir::IndexMap,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_INDEXMAPMAPINDICES;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_INDEXMAPMAPSHAPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.IndexMapMapShape").expect("missing global function")
    });
    pub fn IndexMapMapShape(
        _0: crate::tir::IndexMap,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TIR_INDEXMAPMAPSHAPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_INDEXMAPINVERSE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.IndexMapInverse").expect("missing global function")
    });
    pub fn IndexMapInverse(
        _0: crate::tir::IndexMap,
        _1: tvm_ffi::Array<crate::ir::Range>,
    ) -> Result<crate::tir::IndexMap> {
        let func = &*FUNC_TIR_INDEXMAPINVERSE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::Range>,
            ) -> Result<crate::tir::IndexMap>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_INDEXMAPMAPTENSOR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.IndexMapMapTensor").expect("missing global function")
    });
    pub fn IndexMapMapTensor(
        _0: crate::tir::IndexMap,
        _1: tvm_ffi::Tensor,
    ) -> Result<tvm_ffi::Tensor> {
        let func = &*FUNC_TIR_INDEXMAPMAPTENSOR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Tensor) -> Result<tvm_ffi::Tensor>);
        typed(_0.into(), _1)
    }

    static FUNC_TIR_INDEXMAPNONSURJECTIVEINVERSE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.IndexMapNonSurjectiveInverse").expect("missing global function")
    });
    pub fn IndexMapNonSurjectiveInverse(
        _0: crate::tir::IndexMap,
        _1: tvm_ffi::Array<crate::ir::Range>,
    ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>> {
        let func = &*FUNC_TIR_INDEXMAPNONSURJECTIVEINVERSE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::Range>,
            ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_PYSTMTEXPRMUTATORDEFAULTVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprMutatorDefaultVisitExpr")
            .expect("missing global function")
    });
    pub fn PyStmtExprMutatorDefaultVisitExpr(
        _0: crate::tir::PyStmtExprMutator,
        _1: crate::ir::PrimExpr,
    ) -> Result<()> {
        let func = &*FUNC_TIR_PYSTMTEXPRMUTATORDEFAULTVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRMUTATORDEFAULTVISITSTMT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprMutatorDefaultVisitStmt")
            .expect("missing global function")
    });
    pub fn PyStmtExprMutatorDefaultVisitStmt(
        _0: crate::tir::PyStmtExprMutator,
        _1: crate::tir::Stmt,
    ) -> Result<()> {
        let func = &*FUNC_TIR_PYSTMTEXPRMUTATORDEFAULTVISITSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRMUTATORVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprMutatorVisitExpr").expect("missing global function")
    });
    pub fn PyStmtExprMutatorVisitExpr(
        _0: crate::tir::PyStmtExprMutator,
        _1: crate::ir::PrimExpr,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_PYSTMTEXPRMUTATORVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRMUTATORVISITSTMT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprMutatorVisitStmt").expect("missing global function")
    });
    pub fn PyStmtExprMutatorVisitStmt(
        _0: crate::tir::PyStmtExprMutator,
        _1: crate::tir::Stmt,
    ) -> Result<crate::tir::Stmt> {
        let func = &*FUNC_TIR_PYSTMTEXPRMUTATORVISITSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Stmt>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRVISITORDEFAULTVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprVisitorDefaultVisitExpr")
            .expect("missing global function")
    });
    pub fn PyStmtExprVisitorDefaultVisitExpr(
        _0: crate::tir::PyStmtExprVisitor,
        _1: crate::ir::PrimExpr,
    ) -> Result<()> {
        let func = &*FUNC_TIR_PYSTMTEXPRVISITORDEFAULTVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRVISITORDEFAULTVISITSTMT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprVisitorDefaultVisitStmt")
            .expect("missing global function")
    });
    pub fn PyStmtExprVisitorDefaultVisitStmt(
        _0: crate::tir::PyStmtExprVisitor,
        _1: crate::tir::Stmt,
    ) -> Result<()> {
        let func = &*FUNC_TIR_PYSTMTEXPRVISITORDEFAULTVISITSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRVISITORVISITSTMT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprVisitorVisitStmt").expect("missing global function")
    });
    pub fn PyStmtExprVisitorVisitStmt(
        _0: crate::tir::PyStmtExprVisitor,
        _1: crate::tir::Stmt,
    ) -> Result<()> {
        let func = &*FUNC_TIR_PYSTMTEXPRVISITORVISITSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_PYSTMTEXPRVISITORVISITEXPR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PyStmtExprVisitorVisitExpr").expect("missing global function")
    });
    pub fn PyStmtExprVisitorVisitExpr(
        _0: crate::tir::PyStmtExprVisitor,
        _1: crate::ir::PrimExpr,
    ) -> Result<()> {
        let func = &*FUNC_TIR_PYSTMTEXPRVISITORVISITEXPR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_MAKEPYSTMTEXPRVISITOR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.MakePyStmtExprVisitor").expect("missing global function")
    });
    pub fn MakePyStmtExprVisitor(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_MAKEPYSTMTEXPRVISITOR;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_MAKEPYSTMTEXPRMUTATOR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.MakePyStmtExprMutator").expect("missing global function")
    });
    pub fn MakePyStmtExprMutator(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_MAKEPYSTMTEXPRMUTATOR;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_SPECIALIZE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Specialize").expect("missing global function"));
    pub fn Specialize(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_SPECIALIZE;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_BLOCKREALIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BlockRealize").expect("missing global function")
    });
    pub fn BlockRealize(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: crate::ir::PrimExpr,
        _2: crate::tir::Block,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::BlockRealize> {
        let func = &*FUNC_TIR_BLOCKREALIZE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::BlockRealize>
        );
        typed(_0, _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_BLOCK: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Block").expect("missing global function"));
    pub fn Block(
        _0: tvm_ffi::Array<crate::tir::IterVar>,
        _1: tvm_ffi::Array<crate::tir::BufferRegion>,
        _2: tvm_ffi::Array<crate::tir::BufferRegion>,
        _3: tvm_ffi::String,
        _4: crate::tir::Stmt,
        _5: Option<crate::tir::Stmt>,
        _6: tvm_ffi::Array<crate::tir::Buffer>,
        _7: tvm_ffi::Array<crate::tir::MatchBufferRegion>,
        _8: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        _9: crate::ir::Span,
    ) -> Result<crate::tir::Block> {
        let func = &*FUNC_TIR_BLOCK;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::IterVar>,
                tvm_ffi::Array<crate::tir::BufferRegion>,
                tvm_ffi::Array<crate::tir::BufferRegion>,
                tvm_ffi::String,
                tvm_ffi::object::ObjectRef,
                Option<crate::tir::Stmt>,
                tvm_ffi::Array<crate::tir::Buffer>,
                tvm_ffi::Array<crate::tir::MatchBufferRegion>,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::Block>
        );
        typed(_0, _1, _2, _3, _4.into(), _5, _6, _7, _8, _9.into())
    }

    static FUNC_TIR_MATCHBUFFERREGION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.MatchBufferRegion").expect("missing global function")
    });
    pub fn MatchBufferRegion(
        _0: crate::tir::Buffer,
        _1: crate::tir::BufferRegion,
    ) -> Result<crate::tir::MatchBufferRegion> {
        let func = &*FUNC_TIR_MATCHBUFFERREGION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::MatchBufferRegion>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_BUFFERREGION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferRegion").expect("missing global function")
    });
    pub fn BufferRegion(
        _0: crate::tir::Buffer,
        _1: tvm_ffi::Array<crate::ir::Range>,
    ) -> Result<crate::tir::BufferRegion> {
        let func = &*FUNC_TIR_BUFFERREGION;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::Range>,
            ) -> Result<crate::tir::BufferRegion>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TIR_BUFFERREALIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.BufferRealize").expect("missing global function")
    });
    pub fn BufferRealize(
        _0: crate::tir::Buffer,
        _1: tvm_ffi::Array<crate::ir::Range>,
        _2: crate::ir::PrimExpr,
        _3: crate::tir::Stmt,
        _4: crate::ir::Span,
    ) -> Result<crate::tir::BufferRealize> {
        let func = &*FUNC_TIR_BUFFERREALIZE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::Range>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::BufferRealize>
        );
        typed(_0.into(), _1, _2.into(), _3.into(), _4.into())
    }

    static FUNC_TIR_BUFFERSTORE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.BufferStore").expect("missing global function"));
    pub fn BufferStore(
        _0: crate::tir::Buffer,
        _1: crate::ir::PrimExpr,
        _2: tvm_ffi::Array<crate::ir::PrimExpr>,
        _3: Option<crate::ir::PrimExpr>,
        _4: crate::ir::Span,
    ) -> Result<crate::tir::BufferStore> {
        let func = &*FUNC_TIR_BUFFERSTORE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                Option<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::BufferStore>
        );
        typed(_0.into(), _1.into(), _2, _3, _4.into())
    }

    static FUNC_TIR_EVALUATE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Evaluate").expect("missing global function"));
    pub fn Evaluate(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::tir::Evaluate> {
        let func = &*FUNC_TIR_EVALUATE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::Evaluate>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_IFTHENELSE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.IfThenElse").expect("missing global function"));
    pub fn IfThenElse(
        _0: crate::ir::PrimExpr,
        _1: crate::tir::Stmt,
        _2: crate::tir::Stmt,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::IfThenElse> {
        let func = &*FUNC_TIR_IFTHENELSE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::IfThenElse>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_SEQSTMT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.SeqStmt").expect("missing global function"));
    pub fn SeqStmt(
        _0: tvm_ffi::Array<crate::tir::Stmt>,
        _1: crate::ir::Span,
    ) -> Result<crate::tir::SeqStmt> {
        let func = &*FUNC_TIR_SEQSTMT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::Stmt>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::SeqStmt>
        );
        typed(_0, _1.into())
    }

    static FUNC_TIR_DECLBUFFER: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.DeclBuffer").expect("missing global function"));
    pub fn DeclBuffer(
        _0: crate::tir::Buffer,
        _1: crate::tir::Stmt,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::DeclBuffer> {
        let func = &*FUNC_TIR_DECLBUFFER;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::DeclBuffer>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_ALLOCATECONST: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.AllocateConst").expect("missing global function")
    });
    pub fn AllocateConst(
        _0: crate::tir::Var,
        _1: tvm_ffi::DLDataType,
        _2: tvm_ffi::Array<crate::ir::PrimExpr>,
        _3: tvm_ffi::object::ObjectRef,
        _4: crate::tir::Stmt,
        _5: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
        _6: crate::ir::Span,
    ) -> Result<crate::tir::AllocateConst> {
        let func = &*FUNC_TIR_ALLOCATECONST;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::DLDataType,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::AllocateConst>
        );
        typed(_0.into(), _1, _2, _3, _4.into(), _5, _6.into())
    }

    static FUNC_TIR_ALLOCATE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Allocate").expect("missing global function"));
    pub fn Allocate(
        _0: crate::tir::Var,
        _1: tvm_ffi::DLDataType,
        _2: tvm_ffi::Array<crate::ir::PrimExpr>,
        _3: crate::ir::PrimExpr,
        _4: crate::tir::Stmt,
        _5: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        _6: crate::ir::Span,
    ) -> Result<crate::tir::Allocate> {
        let func = &*FUNC_TIR_ALLOCATE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::DLDataType,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::Allocate>
        );
        typed(_0.into(), _1, _2, _3.into(), _4.into(), _5, _6.into())
    }

    static FUNC_TIR_WHILE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.While").expect("missing global function"));
    pub fn While(
        _0: crate::ir::PrimExpr,
        _1: crate::tir::Stmt,
        _2: crate::ir::Span,
    ) -> Result<crate::tir::While> {
        let func = &*FUNC_TIR_WHILE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::While>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_FOR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.For").expect("missing global function"));
    pub fn For(
        _0: crate::tir::Var,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::PrimExpr,
        _3: i64,
        _4: crate::tir::Stmt,
        _5: Option<crate::tir::IterVar>,
        _6: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
        _7: Option<crate::ir::PrimExpr>,
        _8: crate::ir::Span,
    ) -> Result<crate::tir::For> {
        let func = &*FUNC_TIR_FOR;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                i64,
                tvm_ffi::object::ObjectRef,
                Option<crate::tir::IterVar>,
                Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                Option<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tir::For>
        );
        typed(
            _0.into(),
            _1.into(),
            _2.into(),
            _3,
            _4.into(),
            _5,
            _6,
            _7,
            _8.into(),
        )
    }

    static FUNC_TIR_ASSERTSTMT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.AssertStmt").expect("missing global function"));
    pub fn AssertStmt(
        _0: crate::ir::PrimExpr,
        _1: crate::tir::StringImm,
        _2: crate::tir::Stmt,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::AssertStmt> {
        let func = &*FUNC_TIR_ASSERTSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::AssertStmt>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_ATTRSTMT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.AttrStmt").expect("missing global function"));
    pub fn AttrStmt(
        _0: tvm_ffi::AnyValue,
        _1: tvm_ffi::String,
        _2: crate::ir::PrimExpr,
        _3: crate::tir::Stmt,
        _4: crate::ir::Span,
    ) -> Result<crate::tir::AttrStmt> {
        let func = &*FUNC_TIR_ATTRSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::AnyValue, tvm_ffi::String, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::AttrStmt>);
        typed(_0, _1, _2.into(), _3.into(), _4.into())
    }

    static FUNC_TIR_LETSTMT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.LetStmt").expect("missing global function"));
    pub fn LetStmt(
        _0: crate::tir::Var,
        _1: crate::ir::PrimExpr,
        _2: crate::tir::Stmt,
        _3: crate::ir::Span,
    ) -> Result<crate::tir::LetStmt> {
        let func = &*FUNC_TIR_LETSTMT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::LetStmt>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_IRTRANSFORM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.IRTransform").expect("missing global function"));
    pub fn IRTransform(
        _0: crate::tir::Stmt,
        _1: tvm_ffi::Function,
        _2: tvm_ffi::Function,
        _3: Option<tvm_ffi::Array<tvm_ffi::String>>,
    ) -> Result<crate::tir::Stmt> {
        let func = &*FUNC_TIR_IRTRANSFORM;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Function,
                tvm_ffi::Function,
                Option<tvm_ffi::Array<tvm_ffi::String>>,
            ) -> Result<crate::tir::Stmt>
        );
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_TIR_POSTORDERVISIT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PostOrderVisit").expect("missing global function")
    });
    pub fn PostOrderVisit(_0: tvm_ffi::object::ObjectRef, _1: tvm_ffi::Function) -> Result<()> {
        let func = &*FUNC_TIR_POSTORDERVISIT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function) -> Result<()>);
        typed(_0, _1)
    }

    static FUNC_TIR_PREORDERVISIT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.PreOrderVisit").expect("missing global function")
    });
    pub fn PreOrderVisit(_0: tvm_ffi::object::ObjectRef, _1: tvm_ffi::Function) -> Result<()> {
        let func = &*FUNC_TIR_PREORDERVISIT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Function) -> Result<()>);
        typed(_0, _1)
    }

    static FUNC_TIR_SUBSTITUTE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.Substitute").expect("missing global function"));
    pub fn Substitute(
        _0: tvm_ffi::object::ObjectRef,
        _1: tvm_ffi::Map<crate::tir::Var, crate::ir::PrimExpr>,
    ) -> Result<tvm_ffi::object::ObjectRef> {
        let func = &*FUNC_TIR_SUBSTITUTE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<crate::tir::Var, crate::ir::PrimExpr>,
            ) -> Result<tvm_ffi::object::ObjectRef>
        );
        typed(_0, _1)
    }

    static FUNC_TIR_BITWISE_NOT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.bitwise_not").expect("missing global function"));
    pub fn bitwise_not(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_BITWISE_NOT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR__OPIFTHENELSE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir._OpIfThenElse").expect("missing global function")
    });
    pub fn _OpIfThenElse(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::PrimExpr,
        _3: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPIFTHENELSE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into(), _3.into())
    }

    static FUNC_TIR_CONST_TRUE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.const_true").expect("missing global function"));
    pub fn const_true(_0: tvm_ffi::DLDataType, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_CONST_TRUE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0, _1.into())
    }

    static FUNC_TIR__OPADD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpAdd").expect("missing global function"));
    pub fn _OpAdd(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPADD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPSUB: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpSub").expect("missing global function"));
    pub fn _OpSub(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPSUB;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPMUL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpMul").expect("missing global function"));
    pub fn _OpMul(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPMUL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPDIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpDiv").expect("missing global function"));
    pub fn _OpDiv(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPDIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPMOD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpMod").expect("missing global function"));
    pub fn _OpMod(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPMOD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPINDEXDIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpIndexDiv").expect("missing global function"));
    pub fn _OpIndexDiv(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPINDEXDIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPINDEXMOD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpIndexMod").expect("missing global function"));
    pub fn _OpIndexMod(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPINDEXMOD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPFLOORDIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpFloorDiv").expect("missing global function"));
    pub fn _OpFloorDiv(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPFLOORDIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPLOGADDEXP: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir._OpLogAddExp").expect("missing global function")
    });
    pub fn _OpLogAddExp(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPLOGADDEXP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPFLOORMOD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpFloorMod").expect("missing global function"));
    pub fn _OpFloorMod(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPFLOORMOD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPTRUNCDIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpTruncDiv").expect("missing global function"));
    pub fn _OpTruncDiv(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPTRUNCDIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPTRUNCMOD: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpTruncMod").expect("missing global function"));
    pub fn _OpTruncMod(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPTRUNCMOD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPCEILDIV: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpCeilDiv").expect("missing global function"));
    pub fn _OpCeilDiv(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPCEILDIV;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPPOW: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpPow").expect("missing global function"));
    pub fn _OpPow(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPPOW;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPMIN: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpMin").expect("missing global function"));
    pub fn _OpMin(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPMIN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPMAX: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpMax").expect("missing global function"));
    pub fn _OpMax(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPMAX;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPEQ: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpEQ").expect("missing global function"));
    pub fn _OpEQ(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPEQ;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPNE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpNE").expect("missing global function"));
    pub fn _OpNE(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPNE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPLT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpLT").expect("missing global function"));
    pub fn _OpLT(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPLT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPLE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpLE").expect("missing global function"));
    pub fn _OpLE(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPLE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPGT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpGT").expect("missing global function"));
    pub fn _OpGT(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPGT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPGE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpGE").expect("missing global function"));
    pub fn _OpGE(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPGE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPAND: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpAnd").expect("missing global function"));
    pub fn _OpAnd(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPAND;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR__OPOR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._OpOr").expect("missing global function"));
    pub fn _OpOr(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__OPOR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into(), _2.into())
    }

    static FUNC_TIR_BITWISE_AND: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.bitwise_and").expect("missing global function"));
    pub fn bitwise_and(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_BITWISE_AND;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_BITWISE_OR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.bitwise_or").expect("missing global function"));
    pub fn bitwise_or(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_BITWISE_OR;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_BITWISE_XOR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.bitwise_xor").expect("missing global function"));
    pub fn bitwise_xor(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_BITWISE_XOR;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_LEFT_SHIFT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.left_shift").expect("missing global function"));
    pub fn left_shift(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_LEFT_SHIFT;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_RIGHT_SHIFT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.right_shift").expect("missing global function"));
    pub fn right_shift(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TIR_RIGHT_SHIFT;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TIR_RET: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.ret").expect("missing global function"));
    pub fn ret(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_RET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_THREAD_RETURN: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.thread_return").expect("missing global function")
    });
    pub fn thread_return(_0: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_THREAD_RETURN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into())
    }

    static FUNC_TIR_CONTINUE_LOOP: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tir.continue_loop").expect("missing global function")
    });
    pub fn continue_loop(_0: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_CONTINUE_LOOP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into())
    }

    static FUNC_TIR_BREAK_LOOP: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.break_loop").expect("missing global function"));
    pub fn break_loop(_0: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_BREAK_LOOP;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into())
    }

    static FUNC_TIR_MIN_VALUE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.min_value").expect("missing global function"));
    pub fn min_value(_0: tvm_ffi::DLDataType, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_MIN_VALUE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0, _1.into())
    }

    static FUNC_TIR_MAX_VALUE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.max_value").expect("missing global function"));
    pub fn max_value(_0: tvm_ffi::DLDataType, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_MAX_VALUE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0, _1.into())
    }

    static FUNC_TIR_INFINITY: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.infinity").expect("missing global function"));
    pub fn infinity(_0: tvm_ffi::DLDataType, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_INFINITY;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0, _1.into())
    }

    static FUNC_TIR_ABS: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.abs").expect("missing global function"));
    pub fn abs(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_ABS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_LIKELY: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.likely").expect("missing global function"));
    pub fn likely(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_LIKELY;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_ISNAN: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.isnan").expect("missing global function"));
    pub fn isnan(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_ISNAN;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_ISFINITE: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.isfinite").expect("missing global function"));
    pub fn isfinite(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_ISFINITE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_ISINF: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.isinf").expect("missing global function"));
    pub fn isinf(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_ISINF;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_FLOOR: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.floor").expect("missing global function"));
    pub fn floor(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_FLOOR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_CEIL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.ceil").expect("missing global function"));
    pub fn ceil(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_CEIL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_ROUND: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.round").expect("missing global function"));
    pub fn round(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_ROUND;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_NEARBYINT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.nearbyint").expect("missing global function"));
    pub fn nearbyint(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_NEARBYINT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR_TRUNC: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.trunc").expect("missing global function"));
    pub fn trunc(_0: crate::ir::PrimExpr, _1: crate::ir::Span) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_TRUNC;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TIR__CAST: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir._cast").expect("missing global function"));
    pub fn _cast(
        _0: tvm_ffi::DLDataType,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR__CAST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0, _1.into(), _2.into())
    }

    static FUNC_TIR_REINTERPRET: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.reinterpret").expect("missing global function"));
    pub fn reinterpret(
        _0: tvm_ffi::DLDataType,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::Span,
    ) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TIR_REINTERPRET;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::DLDataType, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0, _1.into(), _2.into())
    }

    static FUNC_TIR_RENEWDEFS: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tir.RenewDefs").expect("missing global function"));
    pub fn RenewDefs(_0: crate::tir::PrimFunc) -> Result<crate::tir::PrimFunc> {
        let func = &*FUNC_TIR_RENEWDEFS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tir::PrimFunc>);
        typed(_0.into())
    }

    pub mod analysis {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_TIR_ANALYSIS_GETBLOCKACCESSREGION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.GetBlockAccessRegion")
                .expect("missing global function")
        });
        pub fn GetBlockAccessRegion(
            _0: crate::tir::Block,
            _1: tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
        ) -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::tir::BufferRegion>>> {
            let func = &*FUNC_TIR_ANALYSIS_GETBLOCKACCESSREGION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
                )
                    -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::tir::BufferRegion>>>
            );
            typed(_0.into(), _1)
        }

        static FUNC_TIR_ANALYSIS_GETBLOCKREADWRITEREGION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.analysis.GetBlockReadWriteRegion")
                    .expect("missing global function")
            });
        pub fn GetBlockReadWriteRegion(
            _0: crate::tir::Block,
            _1: tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
        ) -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::tir::BufferRegion>>> {
            let func = &*FUNC_TIR_ANALYSIS_GETBLOCKREADWRITEREGION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
                )
                    -> Result<tvm_ffi::Array<tvm_ffi::Array<crate::tir::BufferRegion>>>
            );
            typed(_0.into(), _1)
        }

        static FUNC_TIR_ANALYSIS_DETECT_BUFFER_ACCESS_LCA: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.analysis.detect_buffer_access_lca")
                    .expect("missing global function")
            });
        pub fn detect_buffer_access_lca(
            _0: crate::tir::PrimFunc,
        ) -> Result<tvm_ffi::Map<crate::tir::Buffer, Option<crate::tir::Stmt>>> {
            let func = &*FUNC_TIR_ANALYSIS_DETECT_BUFFER_ACCESS_LCA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<crate::tir::Buffer, Option<crate::tir::Stmt>>>);
            typed(_0.into())
        }

        static FUNC_TIR_ANALYSIS_DETECT_BUFFER_VAR_ACCESS_LCA: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.analysis.detect_buffer_var_access_lca")
                    .expect("missing global function")
            });
        pub fn detect_buffer_var_access_lca(
            _0: crate::tir::PrimFunc,
        ) -> Result<tvm_ffi::Map<crate::tir::Var, Option<crate::tir::Stmt>>> {
            let func = &*FUNC_TIR_ANALYSIS_DETECT_BUFFER_VAR_ACCESS_LCA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<crate::tir::Var, Option<crate::tir::Stmt>>>);
            typed(_0.into())
        }

        static FUNC_TIR_ANALYSIS_GET_VTCM_COMPACTION_PASSES: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.analysis.get_vtcm_compaction_passes")
                    .expect("missing global function")
            });
        pub fn get_vtcm_compaction_passes() -> Result<tvm_ffi::Array<crate::transform::Pass>> {
            let func = &*FUNC_TIR_ANALYSIS_GET_VTCM_COMPACTION_PASSES;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<tvm_ffi::Array<crate::transform::Pass>>);
            typed()
        }

        static FUNC_TIR_ANALYSIS_CALCULATE_ALLOCATED_BYTES: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.analysis.calculate_allocated_bytes")
                    .expect("missing global function")
            });
        pub fn calculate_allocated_bytes(
            _0: tvm_ffi::object::ObjectRef,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Map<tvm_ffi::String, crate::ir::IntImm>>>
        {
            let func = &*FUNC_TIR_ANALYSIS_CALCULATE_ALLOCATED_BYTES;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Map<tvm_ffi::String, crate::ir::IntImm>>>);
            typed(_0)
        }

        static FUNC_TIR_ANALYSIS_EXPR_DEEP_EQUAL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.expr_deep_equal").expect("missing global function")
        });
        pub fn expr_deep_equal(_0: crate::ir::PrimExpr, _1: crate::ir::PrimExpr) -> Result<bool> {
            let func = &*FUNC_TIR_ANALYSIS_EXPR_DEEP_EQUAL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_ANALYSIS_ESTIMATETIRFLOPS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.EstimateTIRFlops").expect("missing global function")
        });
        pub fn EstimateTIRFlops(_0: tvm_ffi::object::ObjectRef) -> Result<f64> {
            let func = &*FUNC_TIR_ANALYSIS_ESTIMATETIRFLOPS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<f64>);
            typed(_0)
        }

        static FUNC_TIR_ANALYSIS__IDENTIFY_MEMCPY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis._identify_memcpy").expect("missing global function")
        });
        pub fn _identify_memcpy(
            _0: crate::tir::Stmt,
        ) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>> {
            let func = &*FUNC_TIR_ANALYSIS__IDENTIFY_MEMCPY;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<tvm_ffi::object::ObjectRef>>);
            typed(_0.into())
        }

        static FUNC_TIR_ANALYSIS_IS_PURE_FUNCTION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.is_pure_function").expect("missing global function")
        });
        pub fn is_pure_function(_0: crate::tir::PrimFunc, _1: bool) -> Result<bool> {
            let func = &*FUNC_TIR_ANALYSIS_IS_PURE_FUNCTION;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, bool) -> Result<bool>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_ANALYSIS_OOBCHECKER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.OOBChecker").expect("missing global function")
        });
        pub fn OOBChecker() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_ANALYSIS_OOBCHECKER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_ANALYSIS_FIND_ANCHOR_BLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.find_anchor_block").expect("missing global function")
        });
        pub fn find_anchor_block(_0: crate::ir::IRModule) -> Result<Option<crate::tir::Block>> {
            let func = &*FUNC_TIR_ANALYSIS_FIND_ANCHOR_BLOCK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::Block>>);
            typed(_0.into())
        }

        static FUNC_TIR_ANALYSIS_UNDEFINEDVARS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.UndefinedVars").expect("missing global function")
        });
        pub fn UndefinedVars(args: &[Any]) -> Result<Any> {
            let func = &*FUNC_TIR_ANALYSIS_UNDEFINEDVARS;
            let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
            func.call_packed(&views)
        }

        static FUNC_TIR_ANALYSIS_VERIFY_GPU_CODE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.verify_gpu_code").expect("missing global function")
        });
        pub fn verify_gpu_code(
            _0: crate::tir::PrimFunc,
            _1: tvm_ffi::Map<tvm_ffi::String, crate::ir::PrimExpr>,
        ) -> Result<bool> {
            let func = &*FUNC_TIR_ANALYSIS_VERIFY_GPU_CODE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Map<tvm_ffi::String, crate::ir::PrimExpr>,
                ) -> Result<bool>
            );
            typed(_0.into(), _1)
        }

        static FUNC_TIR_ANALYSIS_VERIFY_MEMORY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.verify_memory").expect("missing global function")
        });
        pub fn verify_memory(_0: crate::tir::PrimFunc) -> Result<bool> {
            let func = &*FUNC_TIR_ANALYSIS_VERIFY_MEMORY;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into())
        }

        static FUNC_TIR_ANALYSIS_VERIFY_SSA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.verify_ssa").expect("missing global function")
        });
        pub fn verify_ssa(_0: crate::tir::PrimFunc) -> Result<bool> {
            let func = &*FUNC_TIR_ANALYSIS_VERIFY_SSA;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into())
        }

        static FUNC_TIR_ANALYSIS_VERIFYWELLFORMED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.analysis.VerifyWellFormed").expect("missing global function")
        });
        pub fn VerifyWellFormed(_0: tvm_ffi::object::ObjectRef, _1: bool) -> Result<bool> {
            let func = &*FUNC_TIR_ANALYSIS_VERIFYWELLFORMED;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, bool) -> Result<bool>);
            typed(_0, _1)
        }
    }
    pub mod schedule {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_TIR_SCHEDULE_ISTRIVIALBINDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.IsTrivialBinding").expect("missing global function")
        });
        pub fn IsTrivialBinding(_0: crate::tir::Schedule, _1: crate::tir::BlockRV) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_ISTRIVIALBINDING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_GETBLOCKREALIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.GetBlockRealize").expect("missing global function")
        });
        pub fn GetBlockRealize(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<crate::tir::BlockRealize> {
            let func = &*FUNC_TIR_SCHEDULE_GETBLOCKREALIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockRealize>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_ISREDUCTIONBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.IsReductionBlock").expect("missing global function")
        });
        pub fn IsReductionBlock(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::BlockRV,
        ) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_ISREDUCTIONBLOCK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_GETAUTOTENSORIZEMAPPINGINFO: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.GetAutoTensorizeMappingInfo")
                    .expect("missing global function")
            });
        pub fn GetAutoTensorizeMappingInfo(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::PrimFunc,
        ) -> Result<Option<crate::tir::schedule::AutoTensorizeMappingInfo>> {
            let func = &*FUNC_TIR_SCHEDULE_GETAUTOTENSORIZEMAPPINGINFO;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::schedule::AutoTensorizeMappingInfo>>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_HASBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.HasBlock").expect("missing global function")
        });
        pub fn HasBlock(_0: crate::tir::Schedule, _1: tvm_ffi::String) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_HASBLOCK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<bool>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_ISOUTPUTBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.IsOutputBlock").expect("missing global function")
        });
        pub fn IsOutputBlock(_0: crate::tir::Schedule, _1: crate::tir::BlockRV) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_ISOUTPUTBLOCK;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_GETLOOPITERTYPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.GetLoopIterType").expect("missing global function")
        });
        pub fn GetLoopIterType(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
        ) -> Result<tvm_ffi::String> {
            let func = &*FUNC_TIR_SCHEDULE_GETLOOPITERTYPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::String>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_HASIFTHENELSE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.HasIfThenElse").expect("missing global function")
        });
        pub fn HasIfThenElse(_0: crate::tir::Stmt) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_HASIFTHENELSE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_ISSPATIALPRIMFUNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.IsSpatialPrimFunc").expect("missing global function")
        });
        pub fn IsSpatialPrimFunc(_0: crate::tir::PrimFunc) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_ISSPATIALPRIMFUNC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_GETTENSORIZELOOPMAPPING: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.GetTensorizeLoopMapping")
                    .expect("missing global function")
            });
        pub fn GetTensorizeLoopMapping(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::PrimFunc,
            _3: bool,
        ) -> Result<Option<crate::tir::schedule::TensorizeInfo>> {
            let func = &*FUNC_TIR_SCHEDULE_GETTENSORIZELOOPMAPPING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool) -> Result<Option<crate::tir::schedule::TensorizeInfo>>);
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_TIR_SCHEDULE_SUGGESTINDEXMAP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.SuggestIndexMap").expect("missing global function")
        });
        pub fn SuggestIndexMap(
            _0: crate::tir::Buffer,
            _1: tvm_ffi::Array<crate::ir::PrimExpr>,
            _2: tvm_ffi::Array<crate::tir::For>,
            _3: crate::ir::PrimExpr,
        ) -> Result<Option<crate::tir::IndexMap>> {
            let func = &*FUNC_TIR_SCHEDULE_SUGGESTINDEXMAP;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::PrimExpr>,
                    tvm_ffi::Array<crate::tir::For>,
                    tvm_ffi::object::ObjectRef,
                ) -> Result<Option<crate::tir::IndexMap>>
            );
            typed(_0.into(), _1, _2, _3.into())
        }

        static FUNC_TIR_SCHEDULE_INSTRUCTIONKINDGET: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.InstructionKindGet")
                .expect("missing global function")
        });
        pub fn InstructionKindGet(_0: tvm_ffi::String) -> Result<crate::tir::InstructionKind> {
            let func = &*FUNC_TIR_SCHEDULE_INSTRUCTIONKINDGET;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::tir::InstructionKind>);
            typed(_0)
        }

        static FUNC_TIR_SCHEDULE_INSTRUCTION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.Instruction").expect("missing global function")
        });
        pub fn Instruction(
            _0: crate::tir::InstructionKind,
            _1: tvm_ffi::Array<tvm_ffi::AnyValue>,
            _2: tvm_ffi::Array<tvm_ffi::AnyValue>,
            _3: tvm_ffi::Array<tvm_ffi::AnyValue>,
        ) -> Result<crate::tir::Instruction> {
            let func = &*FUNC_TIR_SCHEDULE_INSTRUCTION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<tvm_ffi::AnyValue>,
                    tvm_ffi::Array<tvm_ffi::AnyValue>,
                    tvm_ffi::Array<tvm_ffi::AnyValue>,
                ) -> Result<crate::tir::Instruction>
            );
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_TIR_SCHEDULE_CANDECOMPOSEPADDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.CanDecomposePadding")
                .expect("missing global function")
        });
        pub fn CanDecomposePadding(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::LoopRV,
        ) -> Result<bool> {
            let func = &*FUNC_TIR_SCHEDULE_CANDECOMPOSEPADDING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_REGISTERREDUCER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.RegisterReducer").expect("missing global function")
        });
        pub fn RegisterReducer(
            _0: i64,
            _1: tvm_ffi::Function,
            _2: tvm_ffi::Function,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_REGISTERREDUCER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, tvm_ffi::Function, tvm_ffi::Function) -> Result<()>);
            typed(_0, _1, _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEANNOTATEBUFFERACCESS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleAnnotateBufferAccess")
                    .expect("missing global function")
            });
        pub fn ScheduleAnnotateBufferAccess(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: i64,
            _4: crate::tir::IndexMap,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEANNOTATEBUFFERACCESS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, i64, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into(), _2, _3, _4.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEANNOTATE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleAnnotate").expect("missing global function")
        });
        pub fn ScheduleAnnotate(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
            _2: tvm_ffi::String,
            _3: tvm_ffi::AnyValue,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEANNOTATE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String, tvm_ffi::AnyValue) -> Result<()>);
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEUNANNOTATE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleUnannotate")
                .expect("missing global function")
        });
        pub fn ScheduleUnannotate(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
            _2: tvm_ffi::String,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEUNANNOTATE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<()>);
            typed(_0.into(), _1, _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEBLOCKIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleBlockize").expect("missing global function")
        });
        pub fn ScheduleBlockize(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
            _2: bool,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEBLOCKIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1, _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULETENSORIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleTensorize").expect("missing global function")
        });
        pub fn ScheduleTensorize(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
            _2: tvm_ffi::String,
            _3: bool,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULETENSORIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<()>);
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGET: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGet").expect("missing global function")
        });
        pub fn ScheduleGet(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
        ) -> Result<tvm_ffi::object::ObjectRef> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGET;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::object::ObjectRef>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETSREF: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetSRef").expect("missing global function")
        });
        pub fn ScheduleGetSRef(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
        ) -> Result<Option<tvm_ffi::object::ObjectRef>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETSREF;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<tvm_ffi::object::ObjectRef>>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREMOVERV: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleRemoveRV").expect("missing global function")
        });
        pub fn ScheduleRemoveRV(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREMOVERV;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_BLOCKRV: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.BlockRV").expect("missing global function")
        });
        pub fn BlockRV() -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_BLOCKRV;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tir::BlockRV>);
            typed()
        }

        static FUNC_TIR_SCHEDULE_LOOPRV: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.LoopRV").expect("missing global function")
        });
        pub fn LoopRV() -> Result<crate::tir::LoopRV> {
            let func = &*FUNC_TIR_SCHEDULE_LOOPRV;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tir::LoopRV>);
            typed()
        }

        static FUNC_TIR_SCHEDULE_CONCRETESCHEDULE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ConcreteSchedule").expect("missing global function")
        });
        pub fn ConcreteSchedule(
            _0: crate::ir::IRModule,
            _1: i64,
            _2: i64,
            _3: i64,
            _4: bool,
        ) -> Result<crate::tir::Schedule> {
            let func = &*FUNC_TIR_SCHEDULE_CONCRETESCHEDULE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64, i64, bool) -> Result<crate::tir::Schedule>);
            typed(_0.into(), _1, _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_TRACEDSCHEDULE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TracedSchedule").expect("missing global function")
        });
        pub fn TracedSchedule(
            _0: crate::ir::IRModule,
            _1: i64,
            _2: i64,
            _3: i64,
            _4: bool,
        ) -> Result<crate::tir::Schedule> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEDSCHEDULE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64, i64, bool) -> Result<crate::tir::Schedule>);
            typed(_0.into(), _1, _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULETRANSFORMLAYOUT: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleTransformLayout")
                    .expect("missing global function")
            });
        pub fn ScheduleTransformLayout(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: i64,
            _4: crate::tir::IndexMap,
            _5: Option<crate::tir::IndexMap>,
            _6: bool,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULETRANSFORMLAYOUT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    i64,
                    tvm_ffi::object::ObjectRef,
                    Option<crate::tir::IndexMap>,
                    bool,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2, _3, _4.into(), _5, _6)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULETRANSFORMBLOCKLAYOUT: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleTransformBlockLayout")
                    .expect("missing global function")
            });
        pub fn ScheduleTransformBlockLayout(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::IndexMap,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULETRANSFORMBLOCKLAYOUT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESETAXISSEPARATOR: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleSetAxisSeparator")
                    .expect("missing global function")
            });
        pub fn ScheduleSetAxisSeparator(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: i64,
            _4: tvm_ffi::Array<crate::ir::IntImm>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESETAXISSEPARATOR;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    i64,
                    tvm_ffi::Array<crate::ir::IntImm>,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESTORAGEALIGN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleStorageAlign")
                .expect("missing global function")
        });
        pub fn ScheduleStorageAlign(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: i64,
            _4: i64,
            _5: i64,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESTORAGEALIGN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, i64, i64, i64) -> Result<()>);
            typed(_0.into(), _1.into(), _2, _3, _4, _5)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESETSCOPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleSetScope").expect("missing global function")
        });
        pub fn ScheduleSetScope(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESETSCOPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<()>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEUNSAFESETDTYPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleUnsafeSetDType")
                .expect("missing global function")
        });
        pub fn ScheduleUnsafeSetDType(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEUNSAFESETDTYPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<()>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREADAT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleReadAt").expect("missing global function")
        });
        pub fn ScheduleReadAt(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: crate::tir::BlockRV,
            _3: i64,
            _4: tvm_ffi::String,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREADAT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEWRITEAT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleWriteAt").expect("missing global function")
        });
        pub fn ScheduleWriteAt(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: crate::tir::BlockRV,
            _3: i64,
            _4: tvm_ffi::String,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEWRITEAT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECACHEREAD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleCacheRead").expect("missing global function")
        });
        pub fn ScheduleCacheRead(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
            _4: tvm_ffi::Array<crate::tir::BlockRV>,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECACHEREAD;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    tvm_ffi::String,
                    tvm_ffi::Array<crate::tir::BlockRV>,
                ) -> Result<crate::tir::BlockRV>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECACHEWRITE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleCacheWrite")
                .expect("missing global function")
        });
        pub fn ScheduleCacheWrite(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
            _4: tvm_ffi::Array<crate::tir::BlockRV>,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECACHEWRITE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    tvm_ffi::String,
                    tvm_ffi::Array<crate::tir::BlockRV>,
                ) -> Result<crate::tir::BlockRV>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREINDEXCACHEREAD: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleReindexCacheRead")
                    .expect("missing global function")
            });
        pub fn ScheduleReindexCacheRead(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
            _4: crate::tir::IndexMap,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREINDEXCACHEREAD;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2, _3, _4.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREINDEXCACHEWRITE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleReindexCacheWrite")
                    .expect("missing global function")
            });
        pub fn ScheduleReindexCacheWrite(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
            _4: crate::tir::IndexMap,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREINDEXCACHEWRITE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2, _3, _4.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECACHEINPLACE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleCacheInplace")
                .expect("missing global function")
        });
        pub fn ScheduleCacheInplace(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: tvm_ffi::String,
        ) -> Result<tvm_ffi::Array<crate::tir::BlockRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECACHEINPLACE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, tvm_ffi::String) -> Result<tvm_ffi::Array<crate::tir::BlockRV>>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECACHEINDEX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleCacheIndex")
                .expect("missing global function")
        });
        pub fn ScheduleCacheIndex(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: tvm_ffi::String,
            _3: i64,
        ) -> Result<tvm_ffi::Array<crate::tir::BlockRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECACHEINDEX;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String, i64) -> Result<tvm_ffi::Array<crate::tir::BlockRV>>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREINDEX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleReIndex").expect("missing global function")
        });
        pub fn ScheduleReIndex(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
            _3: i64,
            _4: bool,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREINDEX;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64, i64, bool) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEPARALLEL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleParallel").expect("missing global function")
        });
        pub fn ScheduleParallel(_0: crate::tir::Schedule, _1: crate::tir::LoopRV) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEPARALLEL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEVECTORIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleVectorize").expect("missing global function")
        });
        pub fn ScheduleVectorize(_0: crate::tir::Schedule, _1: crate::tir::LoopRV) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEVECTORIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEBIND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleBind").expect("missing global function")
        });
        pub fn ScheduleBind(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: tvm_ffi::String,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEBIND;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<()>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEUNROLL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleUnroll").expect("missing global function")
        });
        pub fn ScheduleUnroll(_0: crate::tir::Schedule, _1: crate::tir::LoopRV) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEUNROLL;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEMERGE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleMerge").expect("missing global function")
        });
        pub fn ScheduleMerge(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::Array<crate::tir::LoopRV>,
        ) -> Result<crate::tir::LoopRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEMERGE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::tir::LoopRV>,
                ) -> Result<crate::tir::LoopRV>
            );
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEFUSE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleFuse").expect("missing global function")
        });
        pub fn ScheduleFuse(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::Array<crate::tir::LoopRV>,
            _2: bool,
        ) -> Result<crate::tir::LoopRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEFUSE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::tir::LoopRV>,
                    bool,
                ) -> Result<crate::tir::LoopRV>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESPLIT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleSplit").expect("missing global function")
        });
        pub fn ScheduleSplit(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: tvm_ffi::Array<Option<crate::ir::PrimExpr>>,
            _3: bool,
            _4: bool,
        ) -> Result<tvm_ffi::Array<crate::tir::LoopRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESPLIT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<Option<crate::ir::PrimExpr>>,
                    bool,
                    bool,
                ) -> Result<tvm_ffi::Array<crate::tir::LoopRV>>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULELOOPPARTITION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleLoopPartition")
                .expect("missing global function")
        });
        pub fn ScheduleLoopPartition(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: tvm_ffi::Array<Option<crate::ir::PrimExpr>>,
            _3: bool,
        ) -> Result<tvm_ffi::Array<crate::tir::LoopRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULELOOPPARTITION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<Option<crate::ir::PrimExpr>>,
                    bool,
                ) -> Result<tvm_ffi::Array<crate::tir::LoopRV>>
            );
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREORDER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleReorder").expect("missing global function")
        });
        pub fn ScheduleReorder(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::Array<crate::tir::LoopRV>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREORDER;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(tvm_ffi::object::ObjectRef, tvm_ffi::Array<crate::tir::LoopRV>) -> Result<()>
            );
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREORDERBLOCKITERVAR: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleReorderBlockIterVar")
                    .expect("missing global function")
            });
        pub fn ScheduleReorderBlockIterVar(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: tvm_ffi::Array<crate::ir::IntImm>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREORDERBLOCKITERVAR;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::IntImm>,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEADDUNITLOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleAddUnitLoop")
                .expect("missing global function")
        });
        pub fn ScheduleAddUnitLoop(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
        ) -> Result<crate::tir::LoopRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEADDUNITLOOP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::LoopRV>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetBlock").expect("missing global function")
        });
        pub fn ScheduleGetBlock(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::String,
            _2: Option<tvm_ffi::String>,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETBLOCK;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::String,
                    Option<tvm_ffi::String>,
                ) -> Result<crate::tir::BlockRV>
            );
            typed(_0.into(), _1, _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETLOOPS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetLoops").expect("missing global function")
        });
        pub fn ScheduleGetLoops(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<tvm_ffi::Array<crate::tir::LoopRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETLOOPS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::LoopRV>>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETCHILDBLOCKS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetChildBlocks")
                .expect("missing global function")
        });
        pub fn ScheduleGetChildBlocks(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::object::ObjectRef,
        ) -> Result<tvm_ffi::Array<crate::tir::BlockRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETCHILDBLOCKS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::BlockRV>>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETPRODUCERS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetProducers")
                .expect("missing global function")
        });
        pub fn ScheduleGetProducers(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<tvm_ffi::Array<crate::tir::BlockRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETPRODUCERS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::BlockRV>>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETCONSUMERS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetConsumers")
                .expect("missing global function")
        });
        pub fn ScheduleGetConsumers(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<tvm_ffi::Array<crate::tir::BlockRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETCONSUMERS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::BlockRV>>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETOUTPUTBLOCKS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleGetOutputBlocks")
                    .expect("missing global function")
            });
        pub fn ScheduleGetOutputBlocks(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<tvm_ffi::Array<crate::tir::BlockRV>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETOUTPUTBLOCKS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::tir::BlockRV>>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETMOD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetMod").expect("missing global function")
        });
        pub fn ScheduleGetMod(_0: crate::tir::Schedule) -> Result<crate::ir::IRModule> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETMOD;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETSTATE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetState").expect("missing global function")
        });
        pub fn ScheduleGetState(_0: crate::tir::Schedule) -> Result<crate::tir::ScheduleState> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETSTATE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tir::ScheduleState>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETTRACE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleGetTrace").expect("missing global function")
        });
        pub fn ScheduleGetTrace(_0: crate::tir::Schedule) -> Result<Option<crate::tir::Trace>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETTRACE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::Trace>>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEGETFUNCWORKINGON: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleGetFuncWorkingOn")
                    .expect("missing global function")
            });
        pub fn ScheduleGetFuncWorkingOn(
            _0: crate::tir::Schedule,
        ) -> Result<Option<crate::ir::GlobalVar>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEGETFUNCWORKINGON;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<crate::ir::GlobalVar>>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECOPY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleCopy").expect("missing global function")
        });
        pub fn ScheduleCopy(_0: crate::tir::Schedule) -> Result<crate::tir::Schedule> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECOPY;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tir::Schedule>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESEED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleSeed").expect("missing global function")
        });
        pub fn ScheduleSeed(_0: crate::tir::Schedule, _1: i64) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESEED;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<()>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEFORKSEED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleForkSeed").expect("missing global function")
        });
        pub fn ScheduleForkSeed(_0: crate::tir::Schedule) -> Result<i64> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEFORKSEED;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEWORKON: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleWorkOn").expect("missing global function")
        });
        pub fn ScheduleWorkOn(_0: crate::tir::Schedule, _1: tvm_ffi::String) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEWORKON;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String) -> Result<()>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESAMPLECATEGORICAL: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleSampleCategorical")
                    .expect("missing global function")
            });
        pub fn ScheduleSampleCategorical(
            _0: crate::tir::Schedule,
            _1: tvm_ffi::Array<crate::ir::IntImm>,
            _2: tvm_ffi::Array<crate::ir::FloatImm>,
            _3: Option<crate::ir::IntImm>,
        ) -> Result<crate::ir::PrimExpr> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESAMPLECATEGORICAL;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::IntImm>,
                    tvm_ffi::Array<crate::ir::FloatImm>,
                    Option<crate::ir::IntImm>,
                ) -> Result<crate::ir::PrimExpr>
            );
            typed(_0.into(), _1, _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESAMPLEPERFECTTILE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleSamplePerfectTile")
                    .expect("missing global function")
            });
        pub fn ScheduleSamplePerfectTile(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: i64,
            _3: i64,
            _4: Option<tvm_ffi::Array<crate::ir::IntImm>>,
        ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESAMPLEPERFECTTILE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    i64,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
            );
            typed(_0.into(), _1.into(), _2, _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESAMPLEPARTITIONEDTILE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleSamplePartitionedTile")
                    .expect("missing global function")
            });
        pub fn ScheduleSamplePartitionedTile(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: i64,
            _3: i64,
            _4: i64,
            _5: Option<tvm_ffi::Array<crate::ir::IntImm>>,
        ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESAMPLEPARTITIONEDTILE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    i64,
                    i64,
                    i64,
                    Option<tvm_ffi::Array<crate::ir::IntImm>>,
                ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>
            );
            typed(_0.into(), _1.into(), _2, _3, _4, _5)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESAMPLECOMPUTELOCATION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleSampleComputeLocation")
                    .expect("missing global function")
            });
        pub fn ScheduleSampleComputeLocation(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: Option<crate::ir::IntImm>,
        ) -> Result<crate::tir::LoopRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESAMPLECOMPUTELOCATION;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<crate::ir::IntImm>,
                ) -> Result<crate::tir::LoopRV>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECOMPUTEAT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleComputeAt").expect("missing global function")
        });
        pub fn ScheduleComputeAt(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::LoopRV,
            _3: bool,
            _4: i64,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECOMPUTEAT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool, i64) -> Result<()>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREVERSECOMPUTEAT: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleReverseComputeAt")
                    .expect("missing global function")
            });
        pub fn ScheduleReverseComputeAt(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::LoopRV,
            _3: bool,
            _4: i64,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREVERSECOMPUTEAT;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool, i64) -> Result<()>);
            typed(_0.into(), _1.into(), _2.into(), _3, _4)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULECOMPUTEINLINE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleComputeInline")
                .expect("missing global function")
        });
        pub fn ScheduleComputeInline(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULECOMPUTEINLINE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEREVERSECOMPUTEINLINE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleReverseComputeInline")
                    .expect("missing global function")
            });
        pub fn ScheduleReverseComputeInline(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEREVERSECOMPUTEINLINE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEFUSEREDUCTIONEPILOGUE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleFuseReductionEpilogue")
                    .expect("missing global function")
            });
        pub fn ScheduleFuseReductionEpilogue(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::BlockRV,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEFUSEREDUCTIONEPILOGUE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEDECOMPOSEREDUCTION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleDecomposeReduction")
                    .expect("missing global function")
            });
        pub fn ScheduleDecomposeReduction(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::LoopRV,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEDECOMPOSEREDUCTION;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULERFACTOR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleRFactor").expect("missing global function")
        });
        pub fn ScheduleRFactor(
            _0: crate::tir::Schedule,
            _1: crate::tir::LoopRV,
            _2: i64,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULERFACTOR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEDECOMPOSEPADDING: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleDecomposePadding")
                    .expect("missing global function")
            });
        pub fn ScheduleDecomposePadding(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: crate::tir::LoopRV,
        ) -> Result<crate::tir::BlockRV> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEDECOMPOSEPADDING;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockRV>);
            typed(_0.into(), _1.into(), _2.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEPADEINSUM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.SchedulePadEinsum").expect("missing global function")
        });
        pub fn SchedulePadEinsum(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: tvm_ffi::Array<crate::ir::IntImm>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEPADEINSUM;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Array<crate::ir::IntImm>,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEROLLINGBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleRollingBuffer")
                .expect("missing global function")
        });
        pub fn ScheduleRollingBuffer(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: i64,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEROLLINGBUFFER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, i64) -> Result<()>);
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEENTERPOSTPROC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleEnterPostproc")
                .expect("missing global function")
        });
        pub fn ScheduleEnterPostproc(_0: crate::tir::Schedule) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEENTERPOSTPROC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULEUNSAFEHIDEBUFFERACCESS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleUnsafeHideBufferAccess")
                    .expect("missing global function")
            });
        pub fn ScheduleUnsafeHideBufferAccess(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: tvm_ffi::String,
            _3: tvm_ffi::Array<crate::ir::IntImm>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULEUNSAFEHIDEBUFFERACCESS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::String,
                    tvm_ffi::Array<crate::ir::IntImm>,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESTATE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleState").expect("missing global function")
        });
        pub fn ScheduleState(
            _0: crate::ir::IRModule,
            _1: i64,
            _2: bool,
        ) -> Result<crate::tir::ScheduleState> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESTATE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, bool) -> Result<crate::tir::ScheduleState>);
            typed(_0.into(), _1, _2)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESTATEGETBLOCKSCOPE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleStateGetBlockScope")
                    .expect("missing global function")
            });
        pub fn ScheduleStateGetBlockScope(
            _0: crate::tir::ScheduleState,
            _1: crate::tir::StmtSRef,
        ) -> Result<crate::tir::BlockScope> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESTATEGETBLOCKSCOPE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::tir::BlockScope>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESTATEREPLACE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleStateReplace")
                .expect("missing global function")
        });
        pub fn ScheduleStateReplace(
            _0: crate::tir::ScheduleState,
            _1: crate::tir::StmtSRef,
            _2: crate::tir::Stmt,
            _3: tvm_ffi::Map<crate::tir::Block, crate::tir::Block>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESTATEREPLACE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::Map<crate::tir::Block, crate::tir::Block>,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2.into(), _3)
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESTATEGETSREF: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.ScheduleStateGetSRef")
                .expect("missing global function")
        });
        pub fn ScheduleStateGetSRef(
            _0: crate::tir::ScheduleState,
            _1: crate::tir::Stmt,
        ) -> Result<Option<crate::tir::StmtSRef>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESTATEGETSREF;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::StmtSRef>>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_SCHEDULESTATEGETCACHEDFLAGS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.ScheduleStateGetCachedFlags")
                    .expect("missing global function")
            });
        pub fn ScheduleStateGetCachedFlags(
            _0: crate::tir::ScheduleState,
            _1: crate::tir::StmtSRef,
        ) -> Result<tvm_ffi::Array<crate::ir::IntImm>> {
            let func = &*FUNC_TIR_SCHEDULE_SCHEDULESTATEGETCACHEDFLAGS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::IntImm>>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_TRACE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.Trace").expect("missing global function")
        });
        pub fn Trace(
            _0: Option<tvm_ffi::Array<crate::tir::Instruction>>,
            _1: Option<tvm_ffi::Map<crate::tir::Instruction, tvm_ffi::AnyValue>>,
        ) -> Result<crate::tir::Trace> {
            let func = &*FUNC_TIR_SCHEDULE_TRACE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    Option<tvm_ffi::Array<crate::tir::Instruction>>,
                    Option<tvm_ffi::Map<crate::tir::Instruction, tvm_ffi::AnyValue>>,
                ) -> Result<crate::tir::Trace>
            );
            typed(_0, _1)
        }

        static FUNC_TIR_SCHEDULE_TRACEGETDECISION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceGetDecision").expect("missing global function")
        });
        pub fn TraceGetDecision(
            _0: crate::tir::Trace,
            _1: crate::tir::Instruction,
        ) -> Result<tvm_ffi::AnyValue> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEGETDECISION;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::AnyValue>);
            typed(_0.into(), _1.into())
        }

        static FUNC_TIR_SCHEDULE_TRACEAPPEND: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceAppend").expect("missing global function")
        });
        pub fn TraceAppend(
            _0: crate::tir::Trace,
            _1: crate::tir::Instruction,
            _2: Option<tvm_ffi::object::ObjectRef>,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEAPPEND;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::object::ObjectRef,
                    tvm_ffi::object::ObjectRef,
                    Option<tvm_ffi::object::ObjectRef>,
                ) -> Result<()>
            );
            typed(_0.into(), _1.into(), _2)
        }

        static FUNC_TIR_SCHEDULE_TRACEPOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TracePop").expect("missing global function")
        });
        pub fn TracePop(_0: crate::tir::Trace) -> Result<Option<crate::tir::Instruction>> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEPOP;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<crate::tir::Instruction>>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_TRACEAPPLYTOSCHEDULE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceApplyToSchedule")
                .expect("missing global function")
        });
        pub fn TraceApplyToSchedule(
            _0: crate::tir::Trace,
            _1: crate::tir::Schedule,
            _2: bool,
            _3: tvm_ffi::Function,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEAPPLYTOSCHEDULE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, bool, tvm_ffi::Function) -> Result<()>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_TRACEASJSON: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceAsJSON").expect("missing global function")
        });
        pub fn TraceAsJSON(_0: crate::tir::Trace, _1: bool) -> Result<tvm_ffi::object::ObjectRef> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEASJSON;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, bool) -> Result<tvm_ffi::object::ObjectRef>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_TRACEASPYTHON: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceAsPython").expect("missing global function")
        });
        pub fn TraceAsPython(
            _0: crate::tir::Trace,
            _1: bool,
        ) -> Result<tvm_ffi::Array<tvm_ffi::String>> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEASPYTHON;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, bool) -> Result<tvm_ffi::Array<tvm_ffi::String>>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_TRACEWITHDECISION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceWithDecision").expect("missing global function")
        });
        pub fn TraceWithDecision(
            _0: crate::tir::Trace,
            _1: crate::tir::Instruction,
            _2: tvm_ffi::AnyValue,
            _3: bool,
        ) -> Result<crate::tir::Trace> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEWITHDECISION;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::AnyValue, bool) -> Result<crate::tir::Trace>);
            typed(_0.into(), _1.into(), _2, _3)
        }

        static FUNC_TIR_SCHEDULE_TRACESIMPLIFIED: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TraceSimplified").expect("missing global function")
        });
        pub fn TraceSimplified(_0: crate::tir::Trace, _1: bool) -> Result<crate::tir::Trace> {
            let func = &*FUNC_TIR_SCHEDULE_TRACESIMPLIFIED;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, bool) -> Result<crate::tir::Trace>);
            typed(_0.into(), _1)
        }

        static FUNC_TIR_SCHEDULE_TRACEAPPLYJSONTOSCHEDULE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.schedule.TraceApplyJSONToSchedule")
                    .expect("missing global function")
            });
        pub fn TraceApplyJSONToSchedule(
            _0: tvm_ffi::object::ObjectRef,
            _1: crate::tir::Schedule,
        ) -> Result<()> {
            let func = &*FUNC_TIR_SCHEDULE_TRACEAPPLYJSONTOSCHEDULE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<()>);
            typed(_0, _1.into())
        }

        static FUNC_TIR_SCHEDULE_NORMALIZEPRIMFUNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.NormalizePrimFunc").expect("missing global function")
        });
        pub fn NormalizePrimFunc(
            _0: crate::tir::Schedule,
        ) -> Result<Option<tvm_ffi::object::ObjectRef>> {
            let func = &*FUNC_TIR_SCHEDULE_NORMALIZEPRIMFUNC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<Option<tvm_ffi::object::ObjectRef>>);
            typed(_0.into())
        }

        static FUNC_TIR_SCHEDULE_TILEWITHTENSORINTRIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.schedule.TileWithTensorIntrin")
                .expect("missing global function")
        });
        pub fn TileWithTensorIntrin(
            _0: crate::tir::Schedule,
            _1: crate::tir::BlockRV,
            _2: tvm_ffi::String,
            _3: bool,
        ) -> Result<Option<crate::tir::LoopRV>> {
            let func = &*FUNC_TIR_SCHEDULE_TILEWITHTENSORINTRIN;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<Option<crate::tir::LoopRV>>);
            typed(_0.into(), _1.into(), _2, _3)
        }
    }
    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_TIR_TRANSFORM_VERIFYVTCMLIMIT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.VerifyVTCMLimit").expect("missing global function")
        });
        pub fn VerifyVTCMLimit(
            _0: Option<crate::target::Target>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_VERIFYVTCMLIMIT;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(Option<crate::target::Target>) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_VERIFYGPUCODE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.VerifyGPUCode").expect("missing global function")
        });
        pub fn VerifyGPUCode(
            _0: tvm_ffi::Map<tvm_ffi::String, crate::ir::PrimExpr>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_VERIFYGPUCODE;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Map<tvm_ffi::String, crate::ir::PrimExpr>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_VERIFYMEMORY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.VerifyMemory").expect("missing global function")
        });
        pub fn VerifyMemory() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_VERIFYMEMORY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_VERIFYSSA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.VerifySSA").expect("missing global function")
        });
        pub fn VerifySSA() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_VERIFYSSA;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_CREATEPRIMFUNCPASS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.CreatePrimFuncPass")
                .expect("missing global function")
        });
        pub fn CreatePrimFuncPass(
            _0: tvm_ffi::Function,
            _1: crate::transform::PassInfo,
        ) -> Result<crate::tir::PrimFuncPass> {
            let func = &*FUNC_TIR_TRANSFORM_CREATEPRIMFUNCPASS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function, tvm_ffi::object::ObjectRef) -> Result<crate::tir::PrimFuncPass>);
            typed(_0, _1.into())
        }

        static FUNC_TIR_TRANSFORM_ANNOTATEDEVICEREGIONS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.AnnotateDeviceRegions")
                .expect("missing global function")
        });
        pub fn AnnotateDeviceRegions() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_ANNOTATEDEVICEREGIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_ANNOTATEIRREGULARLOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.AnnotateIrregularLoop")
                .expect("missing global function")
        });
        pub fn AnnotateIrregularLoop() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_ANNOTATEIRREGULARLOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_BINDTARGET: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.BindTarget").expect("missing global function")
        });
        pub fn BindTarget(_0: crate::target::Target) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_BINDTARGET;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::transform::Pass>);
            typed(_0.into())
        }

        static FUNC_TIR_TRANSFORM_INSTRUMENTBOUNDCHECKERS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.InstrumentBoundCheckers")
                    .expect("missing global function")
            });
        pub fn InstrumentBoundCheckers() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INSTRUMENTBOUNDCHECKERS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_CANONICALIZELOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.CanonicalizeLoop").expect("missing global function")
        });
        pub fn CanonicalizeLoop() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_CANONICALIZELOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_COMBINECONTEXTCALL: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.CombineContextCall")
                .expect("missing global function")
        });
        pub fn CombineContextCall() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_COMBINECONTEXTCALL;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_COMMONSUBEXPRELIMTIR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.CommonSubexprElimTIR")
                .expect("missing global function")
        });
        pub fn CommonSubexprElimTIR(_0: bool, _1: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_COMMONSUBEXPRELIMTIR;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(bool, bool) -> Result<crate::transform::Pass>);
            typed(_0, _1)
        }

        static FUNC_TIR_TRANSFORM_COMPACTBUFFERALLOCATION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.CompactBufferAllocation")
                    .expect("missing global function")
            });
        pub fn CompactBufferAllocation(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_COMPACTBUFFERALLOCATION;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_CONVERTBLOCKSTOOPAQUE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.ConvertBlocksToOpaque")
                .expect("missing global function")
        });
        pub fn ConvertBlocksToOpaque() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_CONVERTBLOCKSTOOPAQUE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_CONVERTFORLOOPSTOSERIAL: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.ConvertForLoopsToSerial")
                    .expect("missing global function")
            });
        pub fn ConvertForLoopsToSerial() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_CONVERTFORLOOPSTOSERIAL;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_DECORATEDEVICESCOPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.DecorateDeviceScope")
                .expect("missing global function")
        });
        pub fn DecorateDeviceScope() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_DECORATEDEVICESCOPE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_DEFAULTGPUSCHEDULE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.DefaultGPUSchedule")
                .expect("missing global function")
        });
        pub fn DefaultGPUSchedule() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_DEFAULTGPUSCHEDULE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_EXTRACTPRIMFUNCCONSTANTS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.ExtractPrimFuncConstants")
                    .expect("missing global function")
            });
        pub fn ExtractPrimFuncConstants() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_EXTRACTPRIMFUNCCONSTANTS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_FLATTENBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.FlattenBuffer").expect("missing global function")
        });
        pub fn FlattenBuffer() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_FLATTENBUFFER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_FORCENARROWINDEXTOINT32: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.ForceNarrowIndexToInt32")
                    .expect("missing global function")
            });
        pub fn ForceNarrowIndexToInt32() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_FORCENARROWINDEXTOINT32;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_HOISTIFTHENELSEBASIC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.HoistIfThenElseBasic")
                .expect("missing global function")
        });
        pub fn HoistIfThenElseBasic() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_HOISTIFTHENELSEBASIC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_HOISTIFTHENELSE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.HoistIfThenElse").expect("missing global function")
        });
        pub fn HoistIfThenElse() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_HOISTIFTHENELSE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_HOISTEXPRESSION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.HoistExpression").expect("missing global function")
        });
        pub fn HoistExpression() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_HOISTEXPRESSION;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INJECTDOUBLEBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InjectDoubleBuffer")
                .expect("missing global function")
        });
        pub fn InjectDoubleBuffer() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTDOUBLEBUFFER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INJECTPERMUTEDLAYOUT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InjectPermutedLayout")
                .expect("missing global function")
        });
        pub fn InjectPermutedLayout() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTPERMUTEDLAYOUT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INJECTPTXASYNCCOPY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InjectPTXAsyncCopy")
                .expect("missing global function")
        });
        pub fn InjectPTXAsyncCopy() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTPTXASYNCCOPY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INJECTPTXLDG32: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InjectPTXLDG32").expect("missing global function")
        });
        pub fn InjectPTXLDG32(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTPTXLDG32;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_INJECTROLLINGBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InjectRollingBuffer")
                .expect("missing global function")
        });
        pub fn InjectRollingBuffer() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTROLLINGBUFFER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INJECTSOFTWAREPIPELINE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.InjectSoftwarePipeline")
                    .expect("missing global function")
            });
        pub fn InjectSoftwarePipeline() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTSOFTWAREPIPELINE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INJECTVIRTUALTHREAD: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InjectVirtualThread")
                .expect("missing global function")
        });
        pub fn InjectVirtualThread() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INJECTVIRTUALTHREAD;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INLINEPRIVATEFUNCTIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.InlinePrivateFunctions")
                    .expect("missing global function")
            });
        pub fn InlinePrivateFunctions() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INLINEPRIVATEFUNCTIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_CONVERTSSA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.ConvertSSA").expect("missing global function")
        });
        pub fn ConvertSSA() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_CONVERTSSA;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LIFTTHREADBINDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LiftThreadBinding")
                .expect("missing global function")
        });
        pub fn LiftThreadBinding() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LIFTTHREADBINDING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOOPPARTITION: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LoopPartition").expect("missing global function")
        });
        pub fn LoopPartition() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOOPPARTITION;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERASYNCDMA: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerAsyncDMA").expect("missing global function")
        });
        pub fn LowerAsyncDMA() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERASYNCDMA;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERCROSSTHREADREDUCTION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.LowerCrossThreadReduction")
                    .expect("missing global function")
            });
        pub fn LowerCrossThreadReduction() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERCROSSTHREADREDUCTION;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERCUSTOMDATATYPES: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerCustomDatatypes")
                .expect("missing global function")
        });
        pub fn LowerCustomDatatypes() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERCUSTOMDATATYPES;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERDEVICEKERNELLAUNCH: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.LowerDeviceKernelLaunch")
                    .expect("missing global function")
            });
        pub fn LowerDeviceKernelLaunch() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERDEVICEKERNELLAUNCH;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERDEVICESTORAGEACCESSINFO: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.LowerDeviceStorageAccessInfo")
                    .expect("missing global function")
            });
        pub fn LowerDeviceStorageAccessInfo() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERDEVICESTORAGEACCESSINFO;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERINITBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerInitBlock").expect("missing global function")
        });
        pub fn LowerInitBlock() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERINITBLOCK;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERINTRIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerIntrin").expect("missing global function")
        });
        pub fn LowerIntrin() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERINTRIN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERMATCHBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerMatchBuffer").expect("missing global function")
        });
        pub fn LowerMatchBuffer() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERMATCHBUFFER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWEROPAQUEBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerOpaqueBlock").expect("missing global function")
        });
        pub fn LowerOpaqueBlock() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWEROPAQUEBLOCK;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERTHREADALLREDUCE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerThreadAllreduce")
                .expect("missing global function")
        });
        pub fn LowerThreadAllreduce() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERTHREADALLREDUCE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERTVMBUILTIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerTVMBuiltin").expect("missing global function")
        });
        pub fn LowerTVMBuiltin() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERTVMBUILTIN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERVTCMALLOC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerVtcmAlloc").expect("missing global function")
        });
        pub fn LowerVtcmAlloc() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERVTCMALLOC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERWARPMEMORY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerWarpMemory").expect("missing global function")
        });
        pub fn LowerWarpMemory() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERWARPMEMORY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_MAKEPACKEDAPI: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.MakePackedAPI").expect("missing global function")
        });
        pub fn MakePackedAPI() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_MAKEPACKEDAPI;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_MAKEUNPACKEDAPI: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.MakeUnpackedAPI").expect("missing global function")
        });
        pub fn MakeUnpackedAPI() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_MAKEUNPACKEDAPI;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_MANIFESTSHAREDMEMORYLOCALSTAGE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.ManifestSharedMemoryLocalStage")
                    .expect("missing global function")
            });
        pub fn ManifestSharedMemoryLocalStage() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_MANIFESTSHAREDMEMORYLOCALSTAGE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_LOWERAUTOCOPY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.LowerAutoCopy").expect("missing global function")
        });
        pub fn LowerAutoCopy() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_LOWERAUTOCOPY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_MERGESHAREDMEMORYALLOCATIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.MergeSharedMemoryAllocations")
                    .expect("missing global function")
            });
        pub fn MergeSharedMemoryAllocations() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_MERGESHAREDMEMORYALLOCATIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_NARROWDATATYPE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.NarrowDataType").expect("missing global function")
        });
        pub fn NarrowDataType(_0: i64) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_NARROWDATATYPE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(i64) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_PLANANDUPDATEBUFFERALLOCATIONLOCATION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.PlanAndUpdateBufferAllocationLocation")
                    .expect("missing global function")
            });
        pub fn PlanAndUpdateBufferAllocationLocation() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_PLANANDUPDATEBUFFERALLOCATIONLOCATION;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_ANNOTATEENTRYFUNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.AnnotateEntryFunc")
                .expect("missing global function")
        });
        pub fn AnnotateEntryFunc() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_ANNOTATEENTRYFUNC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_FILTER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.Filter").expect("missing global function")
        });
        pub fn Filter(_0: tvm_ffi::Function) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_FILTER;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_INSTRUMENTPROFILEINTRINSICS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.InstrumentProfileIntrinsics")
                    .expect("missing global function")
            });
        pub fn InstrumentProfileIntrinsics() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INSTRUMENTPROFILEINTRINSICS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_REDUCEBRANCHINGTHROUGHOVERCOMPUTE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.ReduceBranchingThroughOvercompute")
                    .expect("missing global function")
            });
        pub fn ReduceBranchingThroughOvercompute() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REDUCEBRANCHINGTHROUGHOVERCOMPUTE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_REMAPTHREADAXIS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.RemapThreadAxis").expect("missing global function")
        });
        pub fn RemapThreadAxis(
            _0: tvm_ffi::Map<tvm_ffi::String, crate::tir::IterVar>,
        ) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REMAPTHREADAXIS;
            let typed = tvm_ffi::into_typed_fn!(
                func.clone(),
                Fn(
                    tvm_ffi::Map<tvm_ffi::String, crate::tir::IterVar>,
                ) -> Result<crate::transform::Pass>
            );
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_REMOVEASSUME: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.RemoveAssume").expect("missing global function")
        });
        pub fn RemoveAssume() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REMOVEASSUME;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_REMOVENOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.RemoveNoOp").expect("missing global function")
        });
        pub fn RemoveNoOp() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REMOVENOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_REMOVESTOREUNDEF: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.RemoveStoreUndef").expect("missing global function")
        });
        pub fn RemoveStoreUndef() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REMOVESTOREUNDEF;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_REMOVEWEIGHTLAYOUTREWRITEBLOCK: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.RemoveWeightLayoutRewriteBlock")
                    .expect("missing global function")
            });
        pub fn RemoveWeightLayoutRewriteBlock(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REMOVEWEIGHTLAYOUTREWRITEBLOCK;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_RENORMALIZESPLITPATTERN: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.RenormalizeSplitPattern")
                    .expect("missing global function")
            });
        pub fn RenormalizeSplitPattern() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_RENORMALIZESPLITPATTERN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_REWRITEUNSAFESELECT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.RewriteUnsafeSelect")
                .expect("missing global function")
        });
        pub fn RewriteUnsafeSelect() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_REWRITEUNSAFESELECT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_SIMPLIFY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.Simplify").expect("missing global function")
        });
        pub fn Simplify() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_SIMPLIFY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_SKIPASSERT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.SkipAssert").expect("missing global function")
        });
        pub fn SkipAssert() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_SKIPASSERT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_SPLITHOSTDEVICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.SplitHostDevice").expect("missing global function")
        });
        pub fn SplitHostDevice() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_SPLITHOSTDEVICE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_POINTERVALUETYPEREWRITE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.PointerValueTypeRewrite")
                    .expect("missing global function")
            });
        pub fn PointerValueTypeRewrite() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_POINTERVALUETYPEREWRITE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_STORAGEREWRITE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.StorageRewrite").expect("missing global function")
        });
        pub fn StorageRewrite() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_STORAGEREWRITE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_INFERFRAGMENT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.InferFragment").expect("missing global function")
        });
        pub fn InferFragment() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_INFERFRAGMENT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_THREADSYNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.ThreadSync").expect("missing global function")
        });
        pub fn ThreadSync(_0: tvm_ffi::String) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_THREADSYNC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_TRANSFORMMMABUFFERLAYOUT: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.TransformMmaBufferLayout")
                    .expect("missing global function")
            });
        pub fn TransformMmaBufferLayout() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_TRANSFORMMMABUFFERLAYOUT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_UNIFYTHREADBINDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.UnifyThreadBinding")
                .expect("missing global function")
        });
        pub fn UnifyThreadBinding() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_UNIFYTHREADBINDING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_UNROLLLOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.UnrollLoop").expect("missing global function")
        });
        pub fn UnrollLoop() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_UNROLLLOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_FP8COMPUTELEGALIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.FP8ComputeLegalize")
                .expect("missing global function")
        });
        pub fn FP8ComputeLegalize(_0: tvm_ffi::String) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_FP8COMPUTELEGALIZE;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TIR_TRANSFORM_FP8STORAGELEGALIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.FP8StorageLegalize")
                .expect("missing global function")
        });
        pub fn FP8StorageLegalize() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_FP8STORAGELEGALIZE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_BF16STORAGELEGALIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.BF16StorageLegalize")
                .expect("missing global function")
        });
        pub fn BF16StorageLegalize() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_BF16STORAGELEGALIZE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_BF16COMPUTELEGALIZE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.BF16ComputeLegalize")
                .expect("missing global function")
        });
        pub fn BF16ComputeLegalize() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_BF16COMPUTELEGALIZE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_USEASSUMETOREDUCEBRANCHES: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tir.transform.UseAssumeToReduceBranches")
                    .expect("missing global function")
            });
        pub fn UseAssumeToReduceBranches() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_USEASSUMETOREDUCEBRANCHES;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TIR_TRANSFORM_VECTORIZELOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tir.transform.VectorizeLoop").expect("missing global function")
        });
        pub fn VectorizeLoop(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TIR_TRANSFORM_VECTORIZELOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }
    }
}
pub mod tl {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_TL_WARPSPECIALIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.WarpSpecialize").expect("missing global function")
    });
    pub fn WarpSpecialize(
        _0: tvm_ffi::Array<crate::ir::IntImm>,
        _1: crate::ir::PrimExpr,
        _2: i64,
    ) -> Result<crate::tl::WarpSpecializeFrame> {
        let func = &*FUNC_TL_WARPSPECIALIZE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::IntImm>,
                tvm_ffi::object::ObjectRef,
                i64,
            ) -> Result<crate::tl::WarpSpecializeFrame>
        );
        typed(_0, _1.into(), _2)
    }

    static FUNC_TL_SIDEEFFECT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.SideEffect").expect("missing global function"));
    pub fn SideEffect(_0: crate::ir::PrimExpr) -> Result<i64> {
        let func = &*FUNC_TL_SIDEEFFECT;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into())
    }

    static FUNC_TL_PARALLEL: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.Parallel").expect("missing global function"));
    pub fn Parallel(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    ) -> Result<crate::script::ir_builder::tir::ForFrame> {
        let func = &*FUNC_TL_PARALLEL;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame>
        );
        typed(_0, _1)
    }

    static FUNC_TL_PIPELINED: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.Pipelined").expect("missing global function"));
    pub fn Pipelined(
        _0: crate::ir::PrimExpr,
        _1: crate::ir::PrimExpr,
        _2: i64,
        _3: tvm_ffi::Array<crate::ir::PrimExpr>,
        _4: tvm_ffi::Array<crate::ir::PrimExpr>,
        _5: tvm_ffi::Array<tvm_ffi::Array<crate::ir::PrimExpr>>,
        _6: tvm_ffi::Array<tvm_ffi::Array<crate::ir::PrimExpr>>,
    ) -> Result<crate::script::ir_builder::tir::ForFrame> {
        let func = &*FUNC_TL_PIPELINED;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                i64,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::Array<tvm_ffi::Array<crate::ir::PrimExpr>>,
                tvm_ffi::Array<tvm_ffi::Array<crate::ir::PrimExpr>>,
            ) -> Result<crate::script::ir_builder::tir::ForFrame>
        );
        typed(_0.into(), _1.into(), _2, _3, _4, _5, _6)
    }

    static FUNC_TL_PERSISTENT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.Persistent").expect("missing global function"));
    pub fn Persistent(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: crate::ir::PrimExpr,
        _2: crate::ir::PrimExpr,
        _3: crate::ir::PrimExpr,
    ) -> Result<crate::script::ir_builder::tir::ForFrame> {
        let func = &*FUNC_TL_PERSISTENT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::script::ir_builder::tir::ForFrame>
        );
        typed(_0, _1.into(), _2.into(), _3.into())
    }

    static FUNC_TL_KERNELLAUNCH: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.KernelLaunch").expect("missing global function"));
    pub fn KernelLaunch(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
        _2: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    ) -> Result<crate::tl::KernelLaunchFrame> {
        let func = &*FUNC_TL_KERNELLAUNCH;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            ) -> Result<crate::tl::KernelLaunchFrame>
        );
        typed(_0, _1, _2)
    }

    static FUNC_TL_LAYOUT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.Layout").expect("missing global function"));
    pub fn Layout(
        _0: tvm_ffi::Array<crate::tir::IterVar>,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::IterVar>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<crate::tl::Layout>
        );
        typed(_0, _1)
    }

    static FUNC_TL_LAYOUT_INPUT_SHAPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_input_shape").expect("missing global function")
    });
    pub fn Layout_input_shape(
        _0: crate::tl::Layout,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TL_LAYOUT_INPUT_SHAPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>);
        typed(_0.into())
    }

    static FUNC_TL_LAYOUT_OUTPUT_SHAPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_output_shape").expect("missing global function")
    });
    pub fn Layout_output_shape(
        _0: crate::tl::Layout,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TL_LAYOUT_OUTPUT_SHAPE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>);
        typed(_0.into())
    }

    static FUNC_TL_LAYOUT_INVERSE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_inverse").expect("missing global function")
    });
    pub fn Layout_inverse(_0: crate::tl::Layout) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_LAYOUT_INVERSE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tl::Layout>);
        typed(_0.into())
    }

    static FUNC_TL_LAYOUT_RESHAPE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_reshape").expect("missing global function")
    });
    pub fn Layout_reshape(
        _0: crate::tl::Layout,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: crate::ir::PrimExpr,
        _3: crate::ir::PrimExpr,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_LAYOUT_RESHAPE;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tl::Layout>
        );
        typed(_0.into(), _1, _2.into(), _3.into())
    }

    static FUNC_TL_LAYOUT_INDEX: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.Layout_index").expect("missing global function"));
    pub fn Layout_index(_0: crate::tl::Layout) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TL_LAYOUT_INDEX;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>);
        typed(_0.into())
    }

    static FUNC_TL_LAYOUT_FORWARD_VARS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_forward_vars").expect("missing global function")
    });
    pub fn Layout_forward_vars(
        _0: crate::tl::Layout,
    ) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
        let func = &*FUNC_TL_LAYOUT_FORWARD_VARS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>>);
        typed(_0.into())
    }

    static FUNC_TL_LAYOUT_REPEAT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_repeat").expect("missing global function")
    });
    pub fn Layout_repeat(_0: crate::tl::Layout, _1: i64, _2: i64) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_LAYOUT_REPEAT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64) -> Result<crate::tl::Layout>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_TL_LAYOUT_EXPAND: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_expand").expect("missing global function")
    });
    pub fn Layout_expand(
        _0: crate::tl::Layout,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_LAYOUT_EXPAND;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
            ) -> Result<crate::tl::Layout>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TL_LAYOUT_IS_EQUAL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Layout_is_equal").expect("missing global function")
    });
    pub fn Layout_is_equal(_0: crate::tl::Layout, _1: crate::tl::Layout) -> Result<bool> {
        let func = &*FUNC_TL_LAYOUT_IS_EQUAL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TL_FRAGMENT: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.Fragment").expect("missing global function"));
    pub fn Fragment(
        _0: tvm_ffi::Array<crate::tir::IterVar>,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: crate::ir::PrimExpr,
        _3: crate::tir::IterVar,
    ) -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_FRAGMENT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::tir::IterVar>,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tl::Fragment>
        );
        typed(_0, _1, _2.into(), _3.into())
    }

    static FUNC_TL_FRAGMENT_IS_EQUAL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Fragment_is_equal").expect("missing global function")
    });
    pub fn Fragment_is_equal(_0: crate::tl::Fragment, _1: crate::tl::Fragment) -> Result<bool> {
        let func = &*FUNC_TL_FRAGMENT_IS_EQUAL;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into(), _1.into())
    }

    static FUNC_TL_FRAGMENT_THREAD_SIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Fragment_thread_size").expect("missing global function")
    });
    pub fn Fragment_thread_size(_0: crate::tl::Fragment) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TL_FRAGMENT_THREAD_SIZE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into())
    }

    static FUNC_TL_FRAGMENT_THREAD: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Fragment_thread").expect("missing global function")
    });
    pub fn Fragment_thread(_0: crate::tl::Fragment) -> Result<crate::ir::PrimExpr> {
        let func = &*FUNC_TL_FRAGMENT_THREAD;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::ir::PrimExpr>);
        typed(_0.into())
    }

    static FUNC_TL_FRAGMENT_REPEAT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Fragment_repeat").expect("missing global function")
    });
    pub fn Fragment_repeat(
        _0: crate::tl::Fragment,
        _1: tvm_ffi::Array<crate::ir::PrimExpr>,
        _2: bool,
        _3: bool,
    ) -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_FRAGMENT_REPEAT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::ir::PrimExpr>,
                bool,
                bool,
            ) -> Result<crate::tl::Fragment>
        );
        typed(_0.into(), _1, _2, _3)
    }

    static FUNC_TL_FRAGMENT_REPLICATE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Fragment_replicate").expect("missing global function")
    });
    pub fn Fragment_replicate(_0: crate::tl::Fragment, _1: i64) -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_FRAGMENT_REPLICATE;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64) -> Result<crate::tl::Fragment>);
        typed(_0.into(), _1)
    }

    static FUNC_TL_FRAGMENT_CONDENSE_REP_VAR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.Fragment_condense_rep_var").expect("missing global function")
    });
    pub fn Fragment_condense_rep_var(_0: crate::tl::Fragment) -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_FRAGMENT_CONDENSE_REP_VAR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tl::Fragment>);
        typed(_0.into())
    }

    static FUNC_TL_MAKE_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_swizzled_layout").expect("missing global function")
    });
    pub fn make_swizzled_layout(
        _0: i64,
        _1: i64,
        _2: i64,
        _3: bool,
        _4: bool,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, i64, i64, bool, bool) -> Result<crate::tl::Layout>);
        typed(_0, _1, _2, _3, _4)
    }

    static FUNC_TL_MAKE_VOLTA_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_volta_swizzled_layout").expect("missing global function")
    });
    pub fn make_volta_swizzled_layout(
        _0: i64,
        _1: i64,
        _2: bool,
        _3: bool,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_VOLTA_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, i64, bool, bool) -> Result<crate::tl::Layout>);
        typed(_0, _1, _2, _3)
    }

    static FUNC_TL_MAKE_WGMMA_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_wgmma_swizzled_layout").expect("missing global function")
    });
    pub fn make_wgmma_swizzled_layout(
        _0: i64,
        _1: i64,
        _2: i64,
        _3: i64,
        _4: bool,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_WGMMA_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, i64, i64, i64, bool) -> Result<crate::tl::Layout>);
        typed(_0, _1, _2, _3, _4)
    }

    static FUNC_TL_MAKE_TCGEN05MMA_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_tcgen05mma_swizzled_layout").expect("missing global function")
    });
    pub fn make_tcgen05mma_swizzled_layout(
        _0: i64,
        _1: i64,
        _2: i64,
        _3: i64,
        _4: bool,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_TCGEN05MMA_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, i64, i64, i64, bool) -> Result<crate::tl::Layout>);
        typed(_0, _1, _2, _3, _4)
    }

    static FUNC_TL_MAKE_FULL_BANK_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_full_bank_swizzled_layout").expect("missing global function")
    });
    pub fn make_full_bank_swizzled_layout(_0: crate::tir::Buffer) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_FULL_BANK_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tl::Layout>);
        typed(_0.into())
    }

    static FUNC_TL_MAKE_HALF_BANK_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_half_bank_swizzled_layout").expect("missing global function")
    });
    pub fn make_half_bank_swizzled_layout(_0: crate::tir::Buffer) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_HALF_BANK_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tl::Layout>);
        typed(_0.into())
    }

    static FUNC_TL_MAKE_QUARTER_BANK_SWIZZLED_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_quarter_bank_swizzled_layout")
            .expect("missing global function")
    });
    pub fn make_quarter_bank_swizzled_layout(_0: crate::tir::Buffer) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_QUARTER_BANK_SWIZZLED_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<crate::tl::Layout>);
        typed(_0.into())
    }

    static FUNC_TL_MAKE_LINEAR_LAYOUT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_linear_layout").expect("missing global function")
    });
    pub fn make_linear_layout(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
    ) -> Result<crate::tl::Layout> {
        let func = &*FUNC_TL_MAKE_LINEAR_LAYOUT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(tvm_ffi::Array<crate::ir::PrimExpr>) -> Result<crate::tl::Layout>
        );
        typed(_0)
    }

    static FUNC_TL_MAKE_GEMM_FRAGMENT_8X8: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_gemm_fragment_8x8").expect("missing global function")
    });
    pub fn make_gemm_fragment_8x8() -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_MAKE_GEMM_FRAGMENT_8X8;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tl::Fragment>);
        typed()
    }

    static FUNC_TL_MAKE_GEMM_FRAGMENT_8X8_TRANSPOSED: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.make_gemm_fragment_8x8_transposed")
            .expect("missing global function")
    });
    pub fn make_gemm_fragment_8x8_transposed() -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_MAKE_GEMM_FRAGMENT_8X8_TRANSPOSED;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::tl::Fragment>);
        typed()
    }

    static FUNC_TL_MAKE_FULLY_REPLICATED_LAYOUT_FRAGMENT: LazyLock<Function> =
        LazyLock::new(|| {
            Function::get_global("tl.make_fully_replicated_layout_fragment")
                .expect("missing global function")
        });
    pub fn make_fully_replicated_layout_fragment(
        _0: tvm_ffi::Array<crate::ir::PrimExpr>,
        _1: crate::ir::PrimExpr,
    ) -> Result<crate::tl::Fragment> {
        let func = &*FUNC_TL_MAKE_FULLY_REPLICATED_LAYOUT_FRAGMENT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::Array<crate::ir::PrimExpr>,
                tvm_ffi::object::ObjectRef,
            ) -> Result<crate::tl::Fragment>
        );
        typed(_0, _1.into())
    }

    static FUNC_TL_GEMMWARPPOLICYCOMPUTEWARPPARTITION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.GemmWarpPolicyComputeWarpPartition")
            .expect("missing global function")
    });
    pub fn GemmWarpPolicyComputeWarpPartition(
        _0: crate::tl::GemmWarpPolicy,
        _1: i64,
        _2: i64,
        _3: i64,
        _4: crate::target::Target,
        _5: i64,
    ) -> Result<()> {
        let func = &*FUNC_TL_GEMMWARPPOLICYCOMPUTEWARPPARTITION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64, i64, tvm_ffi::object::ObjectRef, i64) -> Result<()>);
        typed(_0.into(), _1, _2, _3, _4.into(), _5)
    }

    static FUNC_TL_GET_TCGEN5_MMA_META: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.get_tcgen5_mma_meta").expect("missing global function")
    });
    pub fn get_tcgen5_mma_meta(
        _0: i64,
        _1: i64,
        _2: i64,
        _3: tvm_ffi::DLDataType,
        _4: tvm_ffi::DLDataType,
        _5: bool,
    ) -> Result<tvm_ffi::Array<crate::ir::IntImm>> {
        let func = &*FUNC_TL_GET_TCGEN5_MMA_META;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, i64, i64, tvm_ffi::DLDataType, tvm_ffi::DLDataType, bool) -> Result<tvm_ffi::Array<crate::ir::IntImm>>);
        typed(_0, _1, _2, _3, _4, _5)
    }

    static FUNC_TL_GET_TCGEN5_INSTR_DESC: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.get_tcgen5_instr_desc").expect("missing global function")
    });
    pub fn get_tcgen5_instr_desc(
        _0: i64,
        _1: i64,
        _2: i64,
        _3: tvm_ffi::DLDataType,
        _4: tvm_ffi::DLDataType,
        _5: bool,
        _6: bool,
        _7: i64,
        _8: i64,
    ) -> Result<crate::ir::IntImm> {
        let func = &*FUNC_TL_GET_TCGEN5_INSTR_DESC;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(i64, i64, i64, tvm_ffi::DLDataType, tvm_ffi::DLDataType, bool, bool, i64, i64) -> Result<crate::ir::IntImm>);
        typed(_0, _1, _2, _3, _4, _5, _6, _7, _8)
    }

    static FUNC_TL_GEMMPYGEMMINST: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.GemmPyGemmInst").expect("missing global function")
    });
    pub fn GemmPyGemmInst(
        _0: crate::tl::GemmPy,
        _1: i64,
        _2: crate::target::Target,
    ) -> Result<i64> {
        let func = &*FUNC_TL_GEMMPYGEMMINST;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into(), _1, _2.into())
    }

    static FUNC_TL_GEMMSPWARPPOLICYCOMPUTEWARPPARTITION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.GemmSPWarpPolicyComputeWarpPartition")
            .expect("missing global function")
    });
    pub fn GemmSPWarpPolicyComputeWarpPartition(
        _0: crate::tl::GemmSPWarpPolicy,
        _1: i64,
        _2: i64,
        _3: i64,
        _4: crate::target::Target,
        _5: i64,
        _6: i64,
    ) -> Result<()> {
        let func = &*FUNC_TL_GEMMSPWARPPOLICYCOMPUTEWARPPARTITION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, i64, i64, i64, tvm_ffi::object::ObjectRef, i64, i64) -> Result<()>);
        typed(_0.into(), _1, _2, _3, _4.into(), _5, _6)
    }

    static FUNC_TL_TARGETISCUDA: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.TargetIsCuda").expect("missing global function"));
    pub fn TargetIsCuda(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISCUDA;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISROCM: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.TargetIsRocm").expect("missing global function"));
    pub fn TargetIsRocm(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISROCM;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISMETAL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsMetal").expect("missing global function")
    });
    pub fn TargetIsMetal(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISMETAL;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISVOLTA: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsVolta").expect("missing global function")
    });
    pub fn TargetIsVolta(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISVOLTA;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISTURING: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsTuring").expect("missing global function")
    });
    pub fn TargetIsTuring(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISTURING;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISAMPERE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsAmpere").expect("missing global function")
    });
    pub fn TargetIsAmpere(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISAMPERE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISHOPPER: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsHopper").expect("missing global function")
    });
    pub fn TargetIsHopper(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISHOPPER;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISSM120: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsSM120").expect("missing global function")
    });
    pub fn TargetIsSM120(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISSM120;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISCDNA: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.TargetIsCDNA").expect("missing global function"));
    pub fn TargetIsCDNA(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISCDNA;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISRDNA: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("tl.TargetIsRDNA").expect("missing global function"));
    pub fn TargetIsRDNA(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISRDNA;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETISGFX950: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetIsGfx950").expect("missing global function")
    });
    pub fn TargetIsGfx950(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETISGFX950;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETHASASYNCCOPY: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetHasAsyncCopy").expect("missing global function")
    });
    pub fn TargetHasAsyncCopy(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETHASASYNCCOPY;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETHASLDMATRIX: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetHasLdmatrix").expect("missing global function")
    });
    pub fn TargetHasLdmatrix(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETHASLDMATRIX;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETHASSTMATRIX: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetHasStmatrix").expect("missing global function")
    });
    pub fn TargetHasStmatrix(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETHASSTMATRIX;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETHASBULKCOPY: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetHasBulkCopy").expect("missing global function")
    });
    pub fn TargetHasBulkCopy(_0: crate::target::Target) -> Result<bool> {
        let func = &*FUNC_TL_TARGETHASBULKCOPY;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<bool>);
        typed(_0.into())
    }

    static FUNC_TL_TARGETGETWARPSIZE: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("tl.TargetGetWarpSize").expect("missing global function")
    });
    pub fn TargetGetWarpSize(_0: crate::target::Target) -> Result<i64> {
        let func = &*FUNC_TL_TARGETGETWARPSIZE;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<i64>);
        typed(_0.into())
    }

    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, Function, Result};

        static FUNC_TL_TRANSFORM_ALIGNDYNAMICSHAREDMEMORYALLOCATIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.AlignDynamicSharedMemoryAllocations")
                    .expect("missing global function")
            });
        pub fn AlignDynamicSharedMemoryAllocations(_0: i64) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_ALIGNDYNAMICSHAREDMEMORYALLOCATIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(i64) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TL_TRANSFORM_ANNOTATEDEVICEREGIONS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.AnnotateDeviceRegions")
                .expect("missing global function")
        });
        pub fn AnnotateDeviceRegions() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_ANNOTATEDEVICEREGIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_ANNOTATEREADONLYPARAMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.AnnotateReadOnlyParams")
                .expect("missing global function")
        });
        pub fn AnnotateReadOnlyParams() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_ANNOTATEREADONLYPARAMS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_ANNOTATEWARPGROUPREGALLOC: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.AnnotateWarpGroupRegAlloc")
                    .expect("missing global function")
            });
        pub fn AnnotateWarpGroupRegAlloc() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_ANNOTATEWARPGROUPREGALLOC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_CLUSTERPLANNING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.ClusterPlanning").expect("missing global function")
        });
        pub fn ClusterPlanning() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_CLUSTERPLANNING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_CONFIGINDEXBITWIDTH: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.ConfigIndexBitwidth")
                .expect("missing global function")
        });
        pub fn ConfigIndexBitwidth() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_CONFIGINDEXBITWIDTH;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_ELIMINATESTORAGESYNCFORMBARRIER: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.EliminateStorageSyncForMBarrier")
                    .expect("missing global function")
            });
        pub fn EliminateStorageSyncForMBarrier() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_ELIMINATESTORAGESYNCFORMBARRIER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_FLATTENBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.FlattenBuffer").expect("missing global function")
        });
        pub fn FlattenBuffer() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_FLATTENBUFFER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LETINLINE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LetInline").expect("missing global function")
        });
        pub fn LetInline() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LETINLINE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_FUSEMBARRIERARRIVEEXPECTTX: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.FuseMBarrierArriveExpectTx")
                    .expect("missing global function")
            });
        pub fn FuseMBarrierArriveExpectTx() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_FUSEMBARRIERARRIVEEXPECTTX;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_HOISTGLOBALBUFFERALLOCATIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.HoistGlobalBufferAllocations")
                    .expect("missing global function")
            });
        pub fn HoistGlobalBufferAllocations() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_HOISTGLOBALBUFFERALLOCATIONS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_HOISTNONRESTRICTPARAMS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.HoistNonRestrictParams")
                .expect("missing global function")
        });
        pub fn HoistNonRestrictParams() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_HOISTNONRESTRICTPARAMS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_IFSTMTBINDING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.IfStmtBinding").expect("missing global function")
        });
        pub fn IfStmtBinding() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_IFSTMTBINDING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_INJECTASSUMES: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.InjectAssumes").expect("missing global function")
        });
        pub fn InjectAssumes() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_INJECTASSUMES;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_INJECTFENCEPROXY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.InjectFenceProxy").expect("missing global function")
        });
        pub fn InjectFenceProxy() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_INJECTFENCEPROXY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_INJECTSOFTWAREPIPELINE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.InjectSoftwarePipeline")
                .expect("missing global function")
        });
        pub fn InjectSoftwarePipeline() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_INJECTSOFTWAREPIPELINE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LAYOUTINFERENCE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LayoutInference").expect("missing global function")
        });
        pub fn LayoutInference() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LAYOUTINFERENCE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LAYOUTREDUCER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LayoutReducer").expect("missing global function")
        });
        pub fn LayoutReducer() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LAYOUTREDUCER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LEGALIZENEGATIVEINDEX: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LegalizeNegativeIndex")
                .expect("missing global function")
        });
        pub fn LegalizeNegativeIndex() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LEGALIZENEGATIVEINDEX;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LEGALIZESAFEMEMORYACCESS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.LegalizeSafeMemoryAccess")
                    .expect("missing global function")
            });
        pub fn LegalizeSafeMemoryAccess() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LEGALIZESAFEMEMORYACCESS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LEGALIZEVECTORIZEDLOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LegalizeVectorizedLoop")
                .expect("missing global function")
        });
        pub fn LegalizeVectorizedLoop() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LEGALIZEVECTORIZEDLOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOOPUNSWITCHING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LoopUnswitching").expect("missing global function")
        });
        pub fn LoopUnswitching() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOOPUNSWITCHING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERACCESSPTR: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerAccessPtr").expect("missing global function")
        });
        pub fn LowerAccessPtr() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERACCESSPTR;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERBLACKWELL2SM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerBlackwell2SM").expect("missing global function")
        });
        pub fn LowerBlackwell2SM() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERBLACKWELL2SM;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERDEVICEKERNELLAUNCH: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.LowerDeviceKernelLaunch")
                    .expect("missing global function")
            });
        pub fn LowerDeviceKernelLaunch() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERDEVICEKERNELLAUNCH;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERDEVICESTORAGEACCESSINFO: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.LowerDeviceStorageAccessInfo")
                    .expect("missing global function")
            });
        pub fn LowerDeviceStorageAccessInfo() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERDEVICESTORAGEACCESSINFO;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERHOPPERINTRIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerHopperIntrin").expect("missing global function")
        });
        pub fn LowerHopperIntrin() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERHOPPERINTRIN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERINTRIN: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerIntrin").expect("missing global function")
        });
        pub fn LowerIntrin() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERINTRIN;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERL2PERSISTENT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerL2Persistent").expect("missing global function")
        });
        pub fn LowerL2Persistent() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERL2PERSISTENT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERLDGSTG: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerLDGSTG").expect("missing global function")
        });
        pub fn LowerLDGSTG() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERLDGSTG;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWEROPAQUEBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerOpaqueBlock").expect("missing global function")
        });
        pub fn LowerOpaqueBlock() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWEROPAQUEBLOCK;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_MARKCUDASYNCCALLS: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.MarkCudaSyncCalls").expect("missing global function")
        });
        pub fn MarkCudaSyncCalls(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_MARKCUDASYNCCALLS;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TL_TRANSFORM_LOWERPTXASYNCCOPY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerPTXAsyncCopy").expect("missing global function")
        });
        pub fn LowerPTXAsyncCopy() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERPTXASYNCCOPY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERSHAREDBARRIER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerSharedBarrier")
                .expect("missing global function")
        });
        pub fn LowerSharedBarrier() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERSHAREDBARRIER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERSHAREDTMEM: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerSharedTmem").expect("missing global function")
        });
        pub fn LowerSharedTmem() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERSHAREDTMEM;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERTHREADALLREDUCE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerThreadAllreduce")
                .expect("missing global function")
        });
        pub fn LowerThreadAllreduce() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERTHREADALLREDUCE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_LOWERTILEOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.LowerTileOp").expect("missing global function")
        });
        pub fn LowerTileOp() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_LOWERTILEOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_MAKEPACKEDAPI: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.MakePackedAPI").expect("missing global function")
        });
        pub fn MakePackedAPI() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_MAKEPACKEDAPI;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_MERGEIFSTMT: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.MergeIfStmt").expect("missing global function")
        });
        pub fn MergeIfStmt() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_MERGEIFSTMT;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_MERGESHAREDMEMORYALLOCATIONS: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.MergeSharedMemoryAllocations")
                    .expect("missing global function")
            });
        pub fn MergeSharedMemoryAllocations(_0: bool, _1: i64) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_MERGESHAREDMEMORYALLOCATIONS;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(bool, i64) -> Result<crate::transform::Pass>);
            typed(_0, _1)
        }

        static FUNC_TL_TRANSFORM_MULTIVERSIONBUFFER: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.MultiVersionBuffer")
                .expect("missing global function")
        });
        pub fn MultiVersionBuffer(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_MULTIVERSIONBUFFER;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TL_TRANSFORM_OPTIMIZECPASYNCSYNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.OptimizeCPAsyncSync")
                .expect("missing global function")
        });
        pub fn OptimizeCPAsyncSync() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_OPTIMIZECPASYNCSYNC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_PERSISTTHREADBLOCK: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.PersistThreadblock")
                .expect("missing global function")
        });
        pub fn PersistThreadblock() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_PERSISTTHREADBLOCK;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_PIPELINEPLANNING: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.PipelinePlanning").expect("missing global function")
        });
        pub fn PipelinePlanning() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_PIPELINEPLANNING;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_PLANANDUPDATEBUFFERALLOCATIONLOCATION: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.PlanAndUpdateBufferAllocationLocation")
                    .expect("missing global function")
            });
        pub fn PlanAndUpdateBufferAllocationLocation() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_PLANANDUPDATEBUFFERALLOCATIONLOCATION;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_PRODUCERCONSUMERWARPSPECIALIZED: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.ProducerConsumerWarpSpecialized")
                    .expect("missing global function")
            });
        pub fn ProducerConsumerWarpSpecialized() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_PRODUCERCONSUMERWARPSPECIALIZED;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_SIMPLIFY: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.Simplify").expect("missing global function")
        });
        pub fn Simplify(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_SIMPLIFY;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TL_TRANSFORM_SPLITHOSTDEVICE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.SplitHostDevice").expect("missing global function")
        });
        pub fn SplitHostDevice() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_SPLITHOSTDEVICE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_POINTERVALUETYPEREWRITE: LazyLock<Function> =
            LazyLock::new(|| {
                Function::get_global("tl.transform.PointerValueTypeRewrite")
                    .expect("missing global function")
            });
        pub fn PointerValueTypeRewrite() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_POINTERVALUETYPEREWRITE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_STORAGEREWRITE: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.StorageRewrite").expect("missing global function")
        });
        pub fn StorageRewrite() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_STORAGEREWRITE;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_THREADSYNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.ThreadSync").expect("missing global function")
        });
        pub fn ThreadSync(_0: tvm_ffi::String) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_THREADSYNC;
            let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TL_TRANSFORM_UNROLLLOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.UnrollLoop").expect("missing global function")
        });
        pub fn UnrollLoop() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_UNROLLLOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_VECTORIZELOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.VectorizeLoop").expect("missing global function")
        });
        pub fn VectorizeLoop(_0: bool) -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_VECTORIZELOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn(bool) -> Result<crate::transform::Pass>);
            typed(_0)
        }

        static FUNC_TL_TRANSFORM_VERIFYPARALLELLOOP: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.VerifyParallelLoop")
                .expect("missing global function")
        });
        pub fn VerifyParallelLoop() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_VERIFYPARALLELLOOP;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }

        static FUNC_TL_TRANSFORM_REWRITEWGMMASYNC: LazyLock<Function> = LazyLock::new(|| {
            Function::get_global("tl.transform.RewriteWgmmaSync").expect("missing global function")
        });
        pub fn RewriteWgmmaSync() -> Result<crate::transform::Pass> {
            let func = &*FUNC_TL_TRANSFORM_REWRITEWGMMASYNC;
            let typed =
                tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::Pass>);
            typed()
        }
    }
}
pub mod transform {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, Function, Result};

    static FUNC_TRANSFORM_APPLYPASSTOFUNCTION: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.ApplyPassToFunction").expect("missing global function")
    });
    pub fn ApplyPassToFunction(
        _0: crate::transform::Pass,
        _1: tvm_ffi::String,
        _2: bool,
    ) -> Result<crate::transform::Pass> {
        let func = &*FUNC_TRANSFORM_APPLYPASSTOFUNCTION;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::String, bool) -> Result<crate::transform::Pass>);
        typed(_0.into(), _1, _2)
    }

    static FUNC_TRANSFORM_REPLACEGLOBALVARS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.ReplaceGlobalVars").expect("missing global function")
    });
    pub fn ReplaceGlobalVars(
        _0: crate::ir::IRModule,
        _1: tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::GlobalVar>,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_TRANSFORM_REPLACEGLOBALVARS;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::GlobalVar>,
            ) -> Result<crate::ir::IRModule>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TRANSFORM_PASSCONTEXT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.PassContext").expect("missing global function")
    });
    pub fn PassContext(
        _0: i64,
        _1: tvm_ffi::Array<tvm_ffi::String>,
        _2: tvm_ffi::Array<tvm_ffi::String>,
        _3: tvm_ffi::Array<crate::instrument::PassInstrument>,
        _4: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    ) -> Result<crate::transform::PassContext> {
        let func = &*FUNC_TRANSFORM_PASSCONTEXT;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                i64,
                tvm_ffi::Array<tvm_ffi::String>,
                tvm_ffi::Array<tvm_ffi::String>,
                tvm_ffi::Array<crate::instrument::PassInstrument>,
                Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
            ) -> Result<crate::transform::PassContext>
        );
        typed(_0, _1, _2, _3, _4)
    }

    static FUNC_TRANSFORM_SEQUENTIAL: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.Sequential").expect("missing global function")
    });
    pub fn Sequential(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TRANSFORM_SEQUENTIAL;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TRANSFORM_PASSINFO: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.PassInfo").expect("missing global function")
    });
    pub fn PassInfo(
        _0: i64,
        _1: tvm_ffi::String,
        _2: tvm_ffi::Array<tvm_ffi::String>,
        _3: bool,
    ) -> Result<crate::transform::PassInfo> {
        let func = &*FUNC_TRANSFORM_PASSINFO;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                i64,
                tvm_ffi::String,
                tvm_ffi::Array<tvm_ffi::String>,
                bool,
            ) -> Result<crate::transform::PassInfo>
        );
        typed(_0, _1, _2, _3)
    }

    static FUNC_TRANSFORM_INFO: LazyLock<Function> =
        LazyLock::new(|| Function::get_global("transform.Info").expect("missing global function"));
    pub fn Info(args: &[Any]) -> Result<Any> {
        let func = &*FUNC_TRANSFORM_INFO;
        let views: Vec<AnyView<'_>> = args.iter().map(AnyView::from).collect();
        func.call_packed(&views)
    }

    static FUNC_TRANSFORM_GETCURRENTPASSCONTEXT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.GetCurrentPassContext").expect("missing global function")
    });
    pub fn GetCurrentPassContext() -> Result<crate::transform::PassContext> {
        let func = &*FUNC_TRANSFORM_GETCURRENTPASSCONTEXT;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::transform::PassContext>);
        typed()
    }

    static FUNC_TRANSFORM_ENTERPASSCONTEXT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.EnterPassContext").expect("missing global function")
    });
    pub fn EnterPassContext(_0: crate::transform::PassContext) -> Result<()> {
        let func = &*FUNC_TRANSFORM_ENTERPASSCONTEXT;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_TRANSFORM_EXITPASSCONTEXT: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.ExitPassContext").expect("missing global function")
    });
    pub fn ExitPassContext(_0: crate::transform::PassContext) -> Result<()> {
        let func = &*FUNC_TRANSFORM_EXITPASSCONTEXT;
        let typed =
            tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef) -> Result<()>);
        typed(_0.into())
    }

    static FUNC_TRANSFORM_OVERRIDEINSTRUMENTS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.OverrideInstruments").expect("missing global function")
    });
    pub fn OverrideInstruments(
        _0: crate::transform::PassContext,
        _1: tvm_ffi::Array<crate::instrument::PassInstrument>,
    ) -> Result<()> {
        let func = &*FUNC_TRANSFORM_OVERRIDEINSTRUMENTS;
        let typed = tvm_ffi::into_typed_fn!(
            func.clone(),
            Fn(
                tvm_ffi::object::ObjectRef,
                tvm_ffi::Array<crate::instrument::PassInstrument>,
            ) -> Result<()>
        );
        typed(_0.into(), _1)
    }

    static FUNC_TRANSFORM_PRINTIR: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.PrintIR").expect("missing global function")
    });
    pub fn PrintIR(_0: tvm_ffi::String) -> Result<crate::transform::Pass> {
        let func = &*FUNC_TRANSFORM_PRINTIR;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::String) -> Result<crate::transform::Pass>);
        typed(_0)
    }

    static FUNC_TRANSFORM_LISTCONFIGS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.ListConfigs").expect("missing global function")
    });
    pub fn ListConfigs()
    -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Map<tvm_ffi::String, tvm_ffi::String>>> {
        let func = &*FUNC_TRANSFORM_LISTCONFIGS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Map<tvm_ffi::String, tvm_ffi::String>>>);
        typed()
    }

    static FUNC_TRANSFORM_MAKEMODULEPASS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.MakeModulePass").expect("missing global function")
    });
    pub fn MakeModulePass(
        _0: tvm_ffi::Function,
        _1: crate::transform::PassInfo,
    ) -> Result<crate::transform::ModulePass> {
        let func = &*FUNC_TRANSFORM_MAKEMODULEPASS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::Function, tvm_ffi::object::ObjectRef) -> Result<crate::transform::ModulePass>);
        typed(_0, _1.into())
    }

    static FUNC_TRANSFORM_RUNPASS: LazyLock<Function> = LazyLock::new(|| {
        Function::get_global("transform.RunPass").expect("missing global function")
    });
    pub fn RunPass(
        _0: crate::transform::Pass,
        _1: crate::ObjectRValueRef,
    ) -> Result<crate::ir::IRModule> {
        let func = &*FUNC_TRANSFORM_RUNPASS;
        let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn(tvm_ffi::object::ObjectRef, tvm_ffi::object::ObjectRef) -> Result<crate::ir::IRModule>);
        typed(_0.into(), _1.into())
    }
}
