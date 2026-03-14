#![allow(unused_imports)]
#![allow(non_snake_case, nonstandard_style)]

use std::sync::LazyLock;
use tvm_ffi::{Any, AnyView, ObjectArc, Result};

tvm_ffi::define_object_wrapper!(ObjectRValueRef, "ObjectRValueRef");

impl ObjectRValueRef {}

pub mod expr_functor {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "expr_functor.PyExprMutator"]
    pub struct PyExprMutatorObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 304],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PyExprMutator {
        data: tvm_ffi::object::ObjectArc<PyExprMutatorObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PyExprMutator: tvm_ffi::object::ObjectRef);

    impl PyExprMutator {}

    static FIELD_EXPR_FUNCTOR_PYEXPRMUTATOR__BUILDER_: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("expr_functor.PyExprMutator", "builder_")
            .expect("non-layout field builder_ must be registered in TVM reflection")
    });
    impl PyExprMutator {
        pub fn get_builder_(&self) -> tvm_ffi::Any {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_EXPR_FUNCTOR_PYEXPRMUTATOR__BUILDER_
                .get_any(&__obj)
                .expect("non-layout field builder_ should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "expr_functor.PyExprVisitor"]
    pub struct PyExprVisitorObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 240],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PyExprVisitor {
        data: tvm_ffi::object::ObjectArc<PyExprVisitorObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PyExprVisitor: tvm_ffi::object::ObjectRef);

    impl PyExprVisitor {}

    impl PyExprVisitor {}
}
pub mod ffi {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    pub mod reflection {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "ffi.reflection.AccessPath"]
        pub struct AccessPathObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            parent: Option<tvm_ffi::object::ObjectRef>,
            _gap0: [u8; 8],
            depth: i32,
            _gap1: [u8; 4],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AccessPath {
            data: tvm_ffi::object::ObjectArc<AccessPathObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AccessPath: tvm_ffi::object::ObjectRef);

        impl AccessPath {
            pub fn get_parent(&self) -> Option<tvm_ffi::object::ObjectRef> {
                self.data.parent.clone()
            }
            pub fn get_depth(&self) -> i32 {
                self.data.depth
            }
        }

        static FIELD_FFI_REFLECTION_ACCESSPATH__STEP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ffi.reflection.AccessPath", "step")
                .expect("non-layout field step must be registered in TVM reflection")
        });
        static METHOD_FFI_REFLECTION_ACCESSPATH____FFI_SHALLOW_COPY__: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "__ffi_shallow_copy__",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___ROOT: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method("ffi.reflection.AccessPath", "_root")
                    .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___EXTEND: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method("ffi.reflection.AccessPath", "_extend")
                    .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___ATTR: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method("ffi.reflection.AccessPath", "_attr")
                    .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___ARRAY_ITEM: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_array_item",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___MAP_ITEM: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_map_item",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___ATTR_MISSING: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_attr_missing",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___ARRAY_ITEM_MISSING: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_array_item_missing",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___MAP_ITEM_MISSING: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_map_item_missing",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___IS_PREFIX_OF: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_is_prefix_of",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___TO_STEPS: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_to_steps",
                )
                .expect("missing type method")
            });
        static METHOD_FFI_REFLECTION_ACCESSPATH___PATH_EQUAL: LazyLock<tvm_ffi::Function> =
            LazyLock::new(|| {
                tvm_ffi::object_wrapper::resolve_type_method(
                    "ffi.reflection.AccessPath",
                    "_path_equal",
                )
                .expect("missing type method")
            });
        impl AccessPath {
            pub fn __ffi_shallow_copy__(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH____FFI_SHALLOW_COPY__;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _root() -> Result<crate::ffi::reflection::AccessPath> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ROOT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::ffi::reflection::AccessPath>);
                typed()
            }
            pub fn _extend(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___EXTEND;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _attr(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ATTR;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _array_item(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ARRAY_ITEM;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _map_item(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___MAP_ITEM;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _attr_missing(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ATTR_MISSING;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _array_item_missing(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ARRAY_ITEM_MISSING;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _map_item_missing(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___MAP_ITEM_MISSING;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _is_prefix_of(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___IS_PREFIX_OF;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _to_steps(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___TO_STEPS;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _path_equal(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___PATH_EQUAL;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn get_step(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_FFI_REFLECTION_ACCESSPATH__STEP
                    .get_any(&__obj)
                    .expect("non-layout field step should be accessible")
            }
        }
    }
}
pub mod instrument {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "instrument.PassInstrument"]
    pub struct PassInstrumentObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PassInstrument {
        data: tvm_ffi::object::ObjectArc<PassInstrumentObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PassInstrument: tvm_ffi::object::ObjectRef);

    impl PassInstrument {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
    }

    impl PassInstrument {}
}
pub mod ir {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.BaseExpr"]
    pub struct BaseExprObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BaseExpr {
        data: tvm_ffi::object::ObjectArc<BaseExprObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BaseExpr: tvm_ffi::object::ObjectRef);

    impl BaseExpr {
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl BaseExpr {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.PrimExpr"]
    pub struct PrimExprObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseExprObj,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PrimExpr {
        data: tvm_ffi::object::ObjectArc<PrimExprObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PrimExpr: crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl PrimExpr {
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
    }

    impl PrimExpr {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.IntImm"]
    pub struct IntImmObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        value: i64,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct IntImm {
        data: tvm_ffi::object::ObjectArc<IntImmObj>,
    }

    tvm_ffi::impl_object_hierarchy!(IntImm: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl IntImm {
        pub fn get_value(&self) -> i64 {
            self.data.value
        }
    }

    impl IntImm {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.FloatImm"]
    pub struct FloatImmObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        value: f64,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FloatImm {
        data: tvm_ffi::object::ObjectArc<FloatImmObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FloatImm: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl FloatImm {
        pub fn get_value(&self) -> f64 {
            self.data.value
        }
    }

    impl FloatImm {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.RelaxExpr"]
    pub struct RelaxExprObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseExprObj,
        struct_info_: Option<tvm_ffi::object::ObjectRef>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct RelaxExpr {
        data: tvm_ffi::object::ObjectArc<RelaxExprObj>,
    }

    tvm_ffi::impl_object_hierarchy!(RelaxExpr: crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl RelaxExpr {
        pub fn get_struct_info_(&self) -> Option<tvm_ffi::object::ObjectRef> {
            self.data.struct_info_.clone()
        }
    }

    impl RelaxExpr {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.BaseFunc"]
    pub struct BaseFuncObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
        attrs: crate::ir::DictAttrs,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BaseFunc {
        data: tvm_ffi::object::ObjectArc<BaseFuncObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BaseFunc: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl BaseFunc {
        pub fn get_attrs(&self) -> crate::ir::DictAttrs {
            self.data.attrs.clone()
        }
    }

    impl BaseFunc {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.GlobalVar"]
    pub struct GlobalVarObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
        name_hint: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GlobalVar {
        data: tvm_ffi::object::ObjectArc<GlobalVarObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GlobalVar: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl GlobalVar {
        pub fn get_name_hint(&self) -> tvm_ffi::String {
            self.data.name_hint.clone()
        }
    }

    impl GlobalVar {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Op"]
    pub struct OpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
        name: tvm_ffi::String,
        op_type: crate::ir::FuncType,
        description: tvm_ffi::String,
        arguments: tvm_ffi::Array<crate::ir::AttrFieldInfo>,
        attrs_type_key: tvm_ffi::String,
        _gap0: [u8; 4],
        num_inputs: i32,
        support_level: i32,
        _gap1: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Op {
        data: tvm_ffi::object::ObjectArc<OpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Op: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Op {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_op_type(&self) -> crate::ir::FuncType {
            self.data.op_type.clone()
        }
        pub fn get_description(&self) -> tvm_ffi::String {
            self.data.description.clone()
        }
        pub fn get_arguments(&self) -> tvm_ffi::Array<crate::ir::AttrFieldInfo> {
            self.data.arguments.clone()
        }
        pub fn get_attrs_type_key(&self) -> tvm_ffi::String {
            self.data.attrs_type_key.clone()
        }
        pub fn get_num_inputs(&self) -> i32 {
            self.data.num_inputs
        }
        pub fn get_support_level(&self) -> i32 {
            self.data.support_level
        }
    }

    impl Op {}

    tvm_ffi::define_object_wrapper!(IntSet, "ir.IntSet");

    impl IntSet {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Type"]
    pub struct TypeObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Type {
        data: tvm_ffi::object::ObjectArc<TypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Type: tvm_ffi::object::ObjectRef);

    impl Type {
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl Type {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.PrimType"]
    pub struct PrimTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PrimType {
        data: tvm_ffi::object::ObjectArc<PrimTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PrimType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl PrimType {
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
    }

    impl PrimType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.PointerType"]
    pub struct PointerTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
        element_type: crate::ir::Type,
        storage_scope: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PointerType {
        data: tvm_ffi::object::ObjectArc<PointerTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PointerType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl PointerType {
        pub fn get_element_type(&self) -> crate::ir::Type {
            self.data.element_type.clone()
        }
        pub fn get_storage_scope(&self) -> tvm_ffi::String {
            self.data.storage_scope.clone()
        }
    }

    impl PointerType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.TupleType"]
    pub struct TupleTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
        fields: tvm_ffi::Array<crate::ir::Type>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TupleType {
        data: tvm_ffi::object::ObjectArc<TupleTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TupleType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl TupleType {
        pub fn get_fields(&self) -> tvm_ffi::Array<crate::ir::Type> {
            self.data.fields.clone()
        }
    }

    static FIELD_IR_TUPLETYPE__SPAN: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Span>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.TupleType", "span")
            .expect("non-layout field span must be registered in TVM reflection")
    });
    impl TupleType {
        pub fn get_span(&self) -> crate::ir::Span {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_IR_TUPLETYPE__SPAN
                .get(&__obj)
                .expect("non-layout field span should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.FuncType"]
    pub struct FuncTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
        arg_types: tvm_ffi::Array<crate::ir::Type>,
        ret_type: crate::ir::Type,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FuncType {
        data: tvm_ffi::object::ObjectArc<FuncTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FuncType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl FuncType {
        pub fn get_arg_types(&self) -> tvm_ffi::Array<crate::ir::Type> {
            self.data.arg_types.clone()
        }
        pub fn get_ret_type(&self) -> crate::ir::Type {
            self.data.ret_type.clone()
        }
    }

    static FIELD_IR_FUNCTYPE__SPAN: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Span>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.FuncType", "span")
            .expect("non-layout field span must be registered in TVM reflection")
    });
    impl FuncType {
        pub fn get_span(&self) -> crate::ir::Span {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_IR_FUNCTYPE__SPAN
                .get(&__obj)
                .expect("non-layout field span should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.TensorMapType"]
    pub struct TensorMapTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TensorMapType {
        data: tvm_ffi::object::ObjectArc<TensorMapTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TensorMapType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl TensorMapType {}

    static FIELD_IR_TENSORMAPTYPE__SPAN: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Span>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.TensorMapType", "span")
            .expect("non-layout field span must be registered in TVM reflection")
    });
    impl TensorMapType {
        pub fn get_span(&self) -> crate::ir::Span {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_IR_TENSORMAPTYPE__SPAN
                .get(&__obj)
                .expect("non-layout field span should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Attrs"]
    pub struct AttrsObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Attrs {
        data: tvm_ffi::object::ObjectArc<AttrsObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Attrs: tvm_ffi::object::ObjectRef);

    impl Attrs {}

    impl Attrs {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.AttrFieldInfo"]
    pub struct AttrFieldInfoObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        type_info: tvm_ffi::String,
        description: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AttrFieldInfo {
        data: tvm_ffi::object::ObjectArc<AttrFieldInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AttrFieldInfo: tvm_ffi::object::ObjectRef);

    impl AttrFieldInfo {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_type_info(&self) -> tvm_ffi::String {
            self.data.type_info.clone()
        }
        pub fn get_description(&self) -> tvm_ffi::String {
            self.data.description.clone()
        }
    }

    impl AttrFieldInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.DictAttrs"]
    pub struct DictAttrsObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DictAttrs {
        data: tvm_ffi::object::ObjectArc<DictAttrsObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DictAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

    impl DictAttrs {}

    static FIELD_IR_DICTATTRS____DICT__: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.DictAttrs", "__dict__")
            .expect("non-layout field __dict__ must be registered in TVM reflection")
    });
    impl DictAttrs {
        pub fn get___dict__(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_IR_DICTATTRS____DICT__
                .get(&__obj)
                .expect("non-layout field __dict__ should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.EnvFunc"]
    pub struct EnvFuncObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        func: tvm_ffi::Function,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct EnvFunc {
        data: tvm_ffi::object::ObjectArc<EnvFuncObj>,
    }

    tvm_ffi::impl_object_hierarchy!(EnvFunc: tvm_ffi::object::ObjectRef);

    impl EnvFunc {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_func(&self) -> tvm_ffi::Function {
            self.data.func.clone()
        }
    }

    impl EnvFunc {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Range"]
    pub struct RangeObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        min: crate::ir::PrimExpr,
        extent: crate::ir::PrimExpr,
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Range {
        data: tvm_ffi::object::ObjectArc<RangeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Range: tvm_ffi::object::ObjectRef);

    impl Range {
        pub fn get_min(&self) -> crate::ir::PrimExpr {
            self.data.min.clone()
        }
        pub fn get_extent(&self) -> crate::ir::PrimExpr {
            self.data.extent.clone()
        }
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl Range {}

    tvm_ffi::define_object_wrapper!(GlobalInfo, "ir.GlobalInfo");

    impl GlobalInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.VDevice"]
    pub struct VDeviceObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        target: crate::target::Target,
        vdevice_id: i32,
        _gap0: [u8; 4],
        memory_scope: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct VDevice {
        data: tvm_ffi::object::ObjectArc<VDeviceObj>,
    }

    tvm_ffi::impl_object_hierarchy!(VDevice: tvm_ffi::object::ObjectRef);

    impl VDevice {
        pub fn get_target(&self) -> crate::target::Target {
            self.data.target.clone()
        }
        pub fn get_vdevice_id(&self) -> i32 {
            self.data.vdevice_id
        }
        pub fn get_memory_scope(&self) -> tvm_ffi::String {
            self.data.memory_scope.clone()
        }
    }

    impl VDevice {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.DummyGlobalInfo"]
    pub struct DummyGlobalInfoObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DummyGlobalInfo {
        data: tvm_ffi::object::ObjectArc<DummyGlobalInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DummyGlobalInfo: tvm_ffi::object::ObjectRef);

    impl DummyGlobalInfo {}

    impl DummyGlobalInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.GlobalVarSupply"]
    pub struct GlobalVarSupplyObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 64],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GlobalVarSupply {
        data: tvm_ffi::object::ObjectArc<GlobalVarSupplyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GlobalVarSupply: tvm_ffi::object::ObjectRef);

    impl GlobalVarSupply {}

    impl GlobalVarSupply {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.IRModule"]
    pub struct IRModuleObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        functions: tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
        source_map: crate::ir::SourceMap,
        attrs: crate::ir::DictAttrs,
        global_infos: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
        global_var_map_: tvm_ffi::Map<tvm_ffi::String, crate::ir::GlobalVar>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct IRModule {
        data: tvm_ffi::object::ObjectArc<IRModuleObj>,
    }

    tvm_ffi::impl_object_hierarchy!(IRModule: tvm_ffi::object::ObjectRef);

    impl IRModule {
        pub fn get_functions(&self) -> tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc> {
            self.data.functions.clone()
        }
        pub fn get_source_map(&self) -> crate::ir::SourceMap {
            self.data.source_map.clone()
        }
        pub fn get_attrs(&self) -> crate::ir::DictAttrs {
            self.data.attrs.clone()
        }
        pub fn get_global_infos(
            &self,
        ) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>> {
            self.data.global_infos.clone()
        }
        pub fn get_global_var_map_(&self) -> tvm_ffi::Map<tvm_ffi::String, crate::ir::GlobalVar> {
            self.data.global_var_map_.clone()
        }
    }

    impl IRModule {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.NameSupply"]
    pub struct NameSupplyObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 88],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct NameSupply {
        data: tvm_ffi::object::ObjectArc<NameSupplyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(NameSupply: tvm_ffi::object::ObjectRef);

    impl NameSupply {}

    impl NameSupply {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.SourceName"]
    pub struct SourceNameObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct SourceName {
        data: tvm_ffi::object::ObjectArc<SourceNameObj>,
    }

    tvm_ffi::impl_object_hierarchy!(SourceName: tvm_ffi::object::ObjectRef);

    impl SourceName {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
    }

    impl SourceName {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Span"]
    pub struct SpanObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        source_name: crate::ir::SourceName,
        line: i32,
        column: i32,
        end_line: i32,
        end_column: i32,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Span {
        data: tvm_ffi::object::ObjectArc<SpanObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Span: tvm_ffi::object::ObjectRef);

    impl Span {
        pub fn get_source_name(&self) -> crate::ir::SourceName {
            self.data.source_name.clone()
        }
        pub fn get_line(&self) -> i32 {
            self.data.line
        }
        pub fn get_column(&self) -> i32 {
            self.data.column
        }
        pub fn get_end_line(&self) -> i32 {
            self.data.end_line
        }
        pub fn get_end_column(&self) -> i32 {
            self.data.end_column
        }
    }

    impl Span {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.SequentialSpan"]
    pub struct SequentialSpanObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::SpanObj,
        spans: tvm_ffi::Array<crate::ir::Span>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct SequentialSpan {
        data: tvm_ffi::object::ObjectArc<SequentialSpanObj>,
    }

    tvm_ffi::impl_object_hierarchy!(SequentialSpan: crate::ir::Span, tvm_ffi::object::ObjectRef);

    impl SequentialSpan {
        pub fn get_spans(&self) -> tvm_ffi::Array<crate::ir::Span> {
            self.data.spans.clone()
        }
    }

    impl SequentialSpan {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Source"]
    pub struct SourceObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        source_name: crate::ir::SourceName,
        source: tvm_ffi::String,
        _gap0: [u8; 24],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Source {
        data: tvm_ffi::object::ObjectArc<SourceObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Source: tvm_ffi::object::ObjectRef);

    impl Source {
        pub fn get_source_name(&self) -> crate::ir::SourceName {
            self.data.source_name.clone()
        }
        pub fn get_source(&self) -> tvm_ffi::String {
            self.data.source.clone()
        }
    }

    impl Source {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.SourceMap"]
    pub struct SourceMapObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        source_map: tvm_ffi::Map<crate::ir::SourceName, crate::ir::Source>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct SourceMap {
        data: tvm_ffi::object::ObjectArc<SourceMapObj>,
    }

    tvm_ffi::impl_object_hierarchy!(SourceMap: tvm_ffi::object::ObjectRef);

    impl SourceMap {
        pub fn get_source_map(&self) -> tvm_ffi::Map<crate::ir::SourceName, crate::ir::Source> {
            self.data.source_map.clone()
        }
    }

    impl SourceMap {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.StructInfo"]
    pub struct StructInfoObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct StructInfo {
        data: tvm_ffi::object::ObjectArc<StructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(StructInfo: tvm_ffi::object::ObjectRef);

    impl StructInfo {
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl StructInfo {}

    tvm_ffi::define_object_wrapper!(PrimExprConvertible, "ir.PrimExprConvertible");

    impl PrimExprConvertible {}
}
pub mod meta_schedule {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "meta_schedule.ExtractedTask"]
    pub struct ExtractedTaskObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        task_name: tvm_ffi::String,
        mod_: crate::ir::IRModule,
        target: crate::target::Target,
        dispatched: tvm_ffi::Array<crate::ir::IRModule>,
        weight: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ExtractedTask {
        data: tvm_ffi::object::ObjectArc<ExtractedTaskObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ExtractedTask: tvm_ffi::object::ObjectRef);

    impl ExtractedTask {
        pub fn get_task_name(&self) -> tvm_ffi::String {
            self.data.task_name.clone()
        }
        pub fn get_mod_(&self) -> crate::ir::IRModule {
            self.data.mod_.clone()
        }
        pub fn get_target(&self) -> crate::target::Target {
            self.data.target.clone()
        }
        pub fn get_dispatched(&self) -> tvm_ffi::Array<crate::ir::IRModule> {
            self.data.dispatched.clone()
        }
        pub fn get_weight(&self) -> i32 {
            self.data.weight
        }
    }

    impl ExtractedTask {}
}
pub mod relax {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.ShapeType"]
    pub struct ShapeTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
        ndim: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ShapeType {
        data: tvm_ffi::object::ObjectArc<ShapeTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ShapeType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl ShapeType {
        pub fn get_ndim(&self) -> i32 {
            self.data.ndim
        }
    }

    impl ShapeType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.DynTensorType"]
    pub struct DynTensorTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
        ndim: i32,
        dtype: tvm_ffi::DLDataType,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DynTensorType {
        data: tvm_ffi::object::ObjectArc<DynTensorTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DynTensorType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl DynTensorType {
        pub fn get_ndim(&self) -> i32 {
            self.data.ndim
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
    }

    impl DynTensorType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.ObjectType"]
    pub struct ObjectTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ObjectType {
        data: tvm_ffi::object::ObjectArc<ObjectTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ObjectType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl ObjectType {}

    impl ObjectType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.PackedFuncType"]
    pub struct PackedFuncTypeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PackedFuncType {
        data: tvm_ffi::object::ObjectArc<PackedFuncTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PackedFuncType: crate::ir::Type, tvm_ffi::object::ObjectRef);

    impl PackedFuncType {}

    impl PackedFuncType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.ExecBuilder"]
    pub struct ExecBuilderObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 64],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ExecBuilder {
        data: tvm_ffi::object::ObjectArc<ExecBuilderObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ExecBuilder: tvm_ffi::object::ObjectRef);

    impl ExecBuilder {}

    impl ExecBuilder {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.DTensorStructInfo"]
    pub struct DTensorStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
        tensor_sinfo: crate::relax::TensorStructInfo,
        device_mesh: crate::relax::distributed::DeviceMesh,
        placement: crate::relax::distributed::Placement,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DTensorStructInfo {
        data: tvm_ffi::object::ObjectArc<DTensorStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DTensorStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl DTensorStructInfo {
        pub fn get_tensor_sinfo(&self) -> crate::relax::TensorStructInfo {
            self.data.tensor_sinfo.clone()
        }
        pub fn get_device_mesh(&self) -> crate::relax::distributed::DeviceMesh {
            self.data.device_mesh.clone()
        }
        pub fn get_placement(&self) -> crate::relax::distributed::Placement {
            self.data.placement.clone()
        }
    }

    impl DTensorStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.ObjectStructInfo"]
    pub struct ObjectStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ObjectStructInfo {
        data: tvm_ffi::object::ObjectArc<ObjectStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ObjectStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl ObjectStructInfo {}

    impl ObjectStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.PrimStructInfo"]
    pub struct PrimStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
        value: Option<crate::ir::PrimExpr>,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PrimStructInfo {
        data: tvm_ffi::object::ObjectArc<PrimStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PrimStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl PrimStructInfo {
        pub fn get_value(&self) -> Option<crate::ir::PrimExpr> {
            self.data.value.clone()
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
    }

    impl PrimStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.ShapeStructInfo"]
    pub struct ShapeStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
        values: Option<tvm_ffi::Array<crate::ir::PrimExpr>>,
        ndim: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ShapeStructInfo {
        data: tvm_ffi::object::ObjectArc<ShapeStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ShapeStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl ShapeStructInfo {
        pub fn get_values(&self) -> Option<tvm_ffi::Array<crate::ir::PrimExpr>> {
            self.data.values.clone()
        }
        pub fn get_ndim(&self) -> i32 {
            self.data.ndim
        }
    }

    impl ShapeStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.TensorStructInfo"]
    pub struct TensorStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
        shape: Option<crate::ir::RelaxExpr>,
        vdevice: Option<crate::ir::VDevice>,
        dtype: tvm_ffi::DLDataType,
        ndim: i32,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TensorStructInfo {
        data: tvm_ffi::object::ObjectArc<TensorStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TensorStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl TensorStructInfo {
        pub fn get_shape(&self) -> Option<crate::ir::RelaxExpr> {
            self.data.shape.clone()
        }
        pub fn get_vdevice(&self) -> Option<crate::ir::VDevice> {
            self.data.vdevice.clone()
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
        pub fn get_ndim(&self) -> i32 {
            self.data.ndim
        }
    }

    impl TensorStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.TupleStructInfo"]
    pub struct TupleStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
        fields: tvm_ffi::Array<crate::ir::StructInfo>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TupleStructInfo {
        data: tvm_ffi::object::ObjectArc<TupleStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TupleStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl TupleStructInfo {
        pub fn get_fields(&self) -> tvm_ffi::Array<crate::ir::StructInfo> {
            self.data.fields.clone()
        }
    }

    impl TupleStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.FuncStructInfo"]
    pub struct FuncStructInfoObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::StructInfoObj,
        params: Option<tvm_ffi::Array<crate::ir::StructInfo>>,
        ret: crate::ir::StructInfo,
        derive_func: Option<crate::ir::EnvFunc>,
        purity: bool,
        _gap0: [u8; 7],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FuncStructInfo {
        data: tvm_ffi::object::ObjectArc<FuncStructInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FuncStructInfo: crate::ir::StructInfo, tvm_ffi::object::ObjectRef);

    impl FuncStructInfo {
        pub fn get_params(&self) -> Option<tvm_ffi::Array<crate::ir::StructInfo>> {
            self.data.params.clone()
        }
        pub fn get_ret(&self) -> crate::ir::StructInfo {
            self.data.ret.clone()
        }
        pub fn get_derive_func(&self) -> Option<crate::ir::EnvFunc> {
            self.data.derive_func.clone()
        }
        pub fn get_purity(&self) -> bool {
            self.data.purity
        }
    }

    impl FuncStructInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.DataflowBlockRewrite"]
    pub struct DataflowBlockRewriteObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        dfb: crate::relax::expr::DataflowBlock,
        root_fn: Option<crate::relax::expr::Function>,
        _gap0: [u8; 32],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DataflowBlockRewrite {
        data: tvm_ffi::object::ObjectArc<DataflowBlockRewriteObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DataflowBlockRewrite: tvm_ffi::object::ObjectRef);

    impl DataflowBlockRewrite {
        pub fn get_dfb(&self) -> crate::relax::expr::DataflowBlock {
            self.data.dfb.clone()
        }
        pub fn get_root_fn(&self) -> Option<crate::relax::expr::Function> {
            self.data.root_fn.clone()
        }
    }

    impl DataflowBlockRewrite {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.BlockBuilder"]
    pub struct BlockBuilderObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BlockBuilder {
        data: tvm_ffi::object::ObjectArc<BlockBuilderObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BlockBuilder: tvm_ffi::object::ObjectRef);

    impl BlockBuilder {}

    impl BlockBuilder {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.TEPlaceholderOp"]
    pub struct TEPlaceholderOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::te::PlaceholderOpObj,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TEPlaceholderOp {
        data: tvm_ffi::object::ObjectArc<TEPlaceholderOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TEPlaceholderOp: crate::te::PlaceholderOp, crate::te::Operation, tvm_ffi::object::ObjectRef);

    impl TEPlaceholderOp {}

    static FIELD_RELAX_TEPLACEHOLDEROP__NAME: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.TEPlaceholderOp", "name")
            .expect("non-layout field name must be registered in TVM reflection")
    });
    static FIELD_RELAX_TEPLACEHOLDEROP__TAG: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.TEPlaceholderOp", "tag")
            .expect("non-layout field tag must be registered in TVM reflection")
    });
    static FIELD_RELAX_TEPLACEHOLDEROP__ATTRS: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.TEPlaceholderOp", "attrs")
            .expect("non-layout field attrs must be registered in TVM reflection")
    });
    static FIELD_RELAX_TEPLACEHOLDEROP__VALUE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::RelaxExpr>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.TEPlaceholderOp", "value")
            .expect("non-layout field value must be registered in TVM reflection")
    });
    static FIELD_RELAX_TEPLACEHOLDEROP__SHAPE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.TEPlaceholderOp", "shape")
            .expect("non-layout field shape must be registered in TVM reflection")
    });
    static FIELD_RELAX_TEPLACEHOLDEROP__DTYPE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.TEPlaceholderOp", "dtype")
            .expect("non-layout field dtype must be registered in TVM reflection")
    });
    impl TEPlaceholderOp {
        pub fn get_name(&self) -> tvm_ffi::String {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_RELAX_TEPLACEHOLDEROP__NAME
                .get(&__obj)
                .expect("non-layout field name should be accessible")
        }
        pub fn get_tag(&self) -> tvm_ffi::String {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_RELAX_TEPLACEHOLDEROP__TAG
                .get(&__obj)
                .expect("non-layout field tag should be accessible")
        }
        pub fn get_attrs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_RELAX_TEPLACEHOLDEROP__ATTRS
                .get(&__obj)
                .expect("non-layout field attrs should be accessible")
        }
        pub fn get_value(&self) -> crate::ir::RelaxExpr {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_RELAX_TEPLACEHOLDEROP__VALUE
                .get(&__obj)
                .expect("non-layout field value should be accessible")
        }
        pub fn get_shape(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_RELAX_TEPLACEHOLDEROP__SHAPE
                .get(&__obj)
                .expect("non-layout field shape should be accessible")
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_RELAX_TEPLACEHOLDEROP__DTYPE
                .get(&__obj)
                .expect("non-layout field dtype should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.Id"]
    pub struct IdObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name_hint: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Id {
        data: tvm_ffi::object::ObjectArc<IdObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Id: tvm_ffi::object::ObjectRef);

    impl Id {
        pub fn get_name_hint(&self) -> tvm_ffi::String {
            self.data.name_hint.clone()
        }
    }

    impl Id {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.MatchResult"]
    pub struct MatchResultObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        pattern: crate::tir::PrimFunc,
        symbol_values: tvm_ffi::Array<crate::ir::PrimExpr>,
        matched_buffers: tvm_ffi::Array<crate::tir::Buffer>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct MatchResult {
        data: tvm_ffi::object::ObjectArc<MatchResultObj>,
    }

    tvm_ffi::impl_object_hierarchy!(MatchResult: tvm_ffi::object::ObjectRef);

    impl MatchResult {
        pub fn get_pattern(&self) -> crate::tir::PrimFunc {
            self.data.pattern.clone()
        }
        pub fn get_symbol_values(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.symbol_values.clone()
        }
        pub fn get_matched_buffers(&self) -> tvm_ffi::Array<crate::tir::Buffer> {
            self.data.matched_buffers.clone()
        }
    }

    impl MatchResult {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.FunctionPass"]
    pub struct FunctionPassObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        pass_info: crate::transform::PassInfo,
        _gap0: [u8; 40],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FunctionPass {
        data: tvm_ffi::object::ObjectArc<FunctionPassObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FunctionPass: tvm_ffi::object::ObjectRef);

    impl FunctionPass {
        pub fn get_pass_info(&self) -> crate::transform::PassInfo {
            self.data.pass_info.clone()
        }
    }

    impl FunctionPass {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "relax.DataflowBlockPass"]
    pub struct DataflowBlockPassObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        pass_info: crate::transform::PassInfo,
        _gap0: [u8; 40],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DataflowBlockPass {
        data: tvm_ffi::object::ObjectArc<DataflowBlockPassObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DataflowBlockPass: tvm_ffi::object::ObjectRef);

    impl DataflowBlockPass {
        pub fn get_pass_info(&self) -> crate::transform::PassInfo {
            self.data.pass_info.clone()
        }
    }

    impl DataflowBlockPass {}

    pub mod attrs {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AllReduceAttrs"]
        pub struct AllReduceAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            in_group: bool,
            _gap1: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AllReduceAttrs {
            data: tvm_ffi::object::ObjectArc<AllReduceAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AllReduceAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AllReduceAttrs {
            pub fn get_in_group(&self) -> bool {
                self.data.in_group
            }
        }

        static FIELD_RELAX_ATTRS_ALLREDUCEATTRS__OP_TYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.AllReduceAttrs", "op_type")
                .expect("non-layout field op_type must be registered in TVM reflection")
        });
        impl AllReduceAttrs {
            pub fn get_op_type(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ALLREDUCEATTRS__OP_TYPE
                    .get(&__obj)
                    .expect("non-layout field op_type should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AllGatherAttrs"]
        pub struct AllGatherAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AllGatherAttrs {
            data: tvm_ffi::object::ObjectArc<AllGatherAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AllGatherAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AllGatherAttrs {}

        static FIELD_RELAX_ATTRS_ALLGATHERATTRS__NUM_WORKERS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.AllGatherAttrs", "num_workers")
                .expect("non-layout field num_workers must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_ALLGATHERATTRS__IN_GROUP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.AllGatherAttrs", "in_group")
                .expect("non-layout field in_group must be registered in TVM reflection")
        });
        impl AllGatherAttrs {
            pub fn get_num_workers(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ALLGATHERATTRS__NUM_WORKERS
                    .get(&__obj)
                    .expect("non-layout field num_workers should be accessible")
            }
            pub fn get_in_group(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ALLGATHERATTRS__IN_GROUP
                    .get(&__obj)
                    .expect("non-layout field in_group should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ScatterCollectiveAttrs"]
        pub struct ScatterCollectiveAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ScatterCollectiveAttrs {
            data: tvm_ffi::object::ObjectArc<ScatterCollectiveAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ScatterCollectiveAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ScatterCollectiveAttrs {}

        static FIELD_RELAX_ATTRS_SCATTERCOLLECTIVEATTRS__NUM_WORKERS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.ScatterCollectiveAttrs",
                "num_workers",
            )
            .expect("non-layout field num_workers must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_SCATTERCOLLECTIVEATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ScatterCollectiveAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl ScatterCollectiveAttrs {
            pub fn get_num_workers(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SCATTERCOLLECTIVEATTRS__NUM_WORKERS
                    .get(&__obj)
                    .expect("non-layout field num_workers should be accessible")
            }
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SCATTERCOLLECTIVEATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.DistributionAttrs"]
        pub struct DistributionAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            placement: crate::relax::distributed::Placement,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DistributionAttrs {
            data: tvm_ffi::object::ObjectArc<DistributionAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DistributionAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl DistributionAttrs {
            pub fn get_placement(&self) -> crate::relax::distributed::Placement {
                self.data.placement.clone()
            }
        }

        static FIELD_RELAX_ATTRS_DISTRIBUTIONATTRS__DEVICE_MESH: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::relax::distributed::DeviceMesh>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.DistributionAttrs",
                "device_mesh",
            )
            .expect("non-layout field device_mesh must be registered in TVM reflection")
        });
        impl DistributionAttrs {
            pub fn get_device_mesh(&self) -> crate::relax::distributed::DeviceMesh {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_DISTRIBUTIONATTRS__DEVICE_MESH
                    .get(&__obj)
                    .expect("non-layout field device_mesh should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.GridSampleAttrs"]
        pub struct GridSampleAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            layout: tvm_ffi::String,
            padding_mode: tvm_ffi::String,
            align_corners: bool,
            _gap1: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct GridSampleAttrs {
            data: tvm_ffi::object::ObjectArc<GridSampleAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(GridSampleAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl GridSampleAttrs {
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_padding_mode(&self) -> tvm_ffi::String {
                self.data.padding_mode.clone()
            }
            pub fn get_align_corners(&self) -> bool {
                self.data.align_corners
            }
        }

        static FIELD_RELAX_ATTRS_GRIDSAMPLEATTRS__METHOD: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.GridSampleAttrs", "method")
                .expect("non-layout field method must be registered in TVM reflection")
        });
        impl GridSampleAttrs {
            pub fn get_method(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_GRIDSAMPLEATTRS__METHOD
                    .get(&__obj)
                    .expect("non-layout field method should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Resize2DAttrs"]
        pub struct Resize2DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            layout: tvm_ffi::String,
            method: tvm_ffi::String,
            coordinate_transformation_mode: tvm_ffi::String,
            rounding_method: tvm_ffi::String,
            cubic_alpha: f64,
            cubic_exclude: i32,
            _gap0: [u8; 4],
            extrapolation_value: f64,
            out_dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Resize2DAttrs {
            data: tvm_ffi::object::ObjectArc<Resize2DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Resize2DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Resize2DAttrs {
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_method(&self) -> tvm_ffi::String {
                self.data.method.clone()
            }
            pub fn get_coordinate_transformation_mode(&self) -> tvm_ffi::String {
                self.data.coordinate_transformation_mode.clone()
            }
            pub fn get_rounding_method(&self) -> tvm_ffi::String {
                self.data.rounding_method.clone()
            }
            pub fn get_cubic_alpha(&self) -> f64 {
                self.data.cubic_alpha
            }
            pub fn get_cubic_exclude(&self) -> i32 {
                self.data.cubic_exclude
            }
            pub fn get_extrapolation_value(&self) -> f64 {
                self.data.extrapolation_value
            }
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.out_dtype
            }
        }

        static FIELD_RELAX_ATTRS_RESIZE2DATTRS__ROI: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::FloatImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Resize2DAttrs", "roi")
                .expect("non-layout field roi must be registered in TVM reflection")
        });
        impl Resize2DAttrs {
            pub fn get_roi(&self) -> tvm_ffi::Array<crate::ir::FloatImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_RESIZE2DATTRS__ROI
                    .get(&__obj)
                    .expect("non-layout field roi should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AttentionAttrs"]
        pub struct AttentionAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            causal_mask: Option<tvm_ffi::String>,
            window_size: Option<crate::ir::IntImm>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AttentionAttrs {
            data: tvm_ffi::object::ObjectArc<AttentionAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AttentionAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AttentionAttrs {
            pub fn get_causal_mask(&self) -> Option<tvm_ffi::String> {
                self.data.causal_mask.clone()
            }
            pub fn get_window_size(&self) -> Option<crate::ir::IntImm> {
                self.data.window_size.clone()
            }
        }

        static FIELD_RELAX_ATTRS_ATTENTIONATTRS__SCALE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::FloatImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.AttentionAttrs", "scale")
                .expect("non-layout field scale must be registered in TVM reflection")
        });
        impl AttentionAttrs {
            pub fn get_scale(&self) -> Option<crate::ir::FloatImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ATTENTIONATTRS__SCALE
                    .get(&__obj)
                    .expect("non-layout field scale should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Conv1DAttrs"]
        pub struct Conv1DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            groups: i32,
            _gap0: [u8; 4],
            data_layout: tvm_ffi::String,
            kernel_layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            out_dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Conv1DAttrs {
            data: tvm_ffi::object::ObjectArc<Conv1DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Conv1DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Conv1DAttrs {
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_groups(&self) -> i32 {
                self.data.groups
            }
            pub fn get_data_layout(&self) -> tvm_ffi::String {
                self.data.data_layout.clone()
            }
            pub fn get_kernel_layout(&self) -> tvm_ffi::String {
                self.data.kernel_layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.out_dtype
            }
        }

        static FIELD_RELAX_ATTRS_CONV1DATTRS__STRIDES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Conv1DAttrs", "strides")
                .expect("non-layout field strides must be registered in TVM reflection")
        });
        impl Conv1DAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CONV1DATTRS__STRIDES
                    .get(&__obj)
                    .expect("non-layout field strides should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Conv2DAttrs"]
        pub struct Conv2DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            groups: i32,
            _gap0: [u8; 4],
            data_layout: tvm_ffi::String,
            kernel_layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            out_dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Conv2DAttrs {
            data: tvm_ffi::object::ObjectArc<Conv2DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Conv2DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Conv2DAttrs {
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_groups(&self) -> i32 {
                self.data.groups
            }
            pub fn get_data_layout(&self) -> tvm_ffi::String {
                self.data.data_layout.clone()
            }
            pub fn get_kernel_layout(&self) -> tvm_ffi::String {
                self.data.kernel_layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.out_dtype
            }
        }

        static FIELD_RELAX_ATTRS_CONV2DATTRS__STRIDES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Conv2DAttrs", "strides")
                .expect("non-layout field strides must be registered in TVM reflection")
        });
        impl Conv2DAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CONV2DATTRS__STRIDES
                    .get(&__obj)
                    .expect("non-layout field strides should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Conv3DAttrs"]
        pub struct Conv3DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            groups: i32,
            _gap0: [u8; 4],
            data_layout: tvm_ffi::String,
            kernel_layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            out_dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Conv3DAttrs {
            data: tvm_ffi::object::ObjectArc<Conv3DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Conv3DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Conv3DAttrs {
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_groups(&self) -> i32 {
                self.data.groups
            }
            pub fn get_data_layout(&self) -> tvm_ffi::String {
                self.data.data_layout.clone()
            }
            pub fn get_kernel_layout(&self) -> tvm_ffi::String {
                self.data.kernel_layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.out_dtype
            }
        }

        static FIELD_RELAX_ATTRS_CONV3DATTRS__STRIDES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Conv3DAttrs", "strides")
                .expect("non-layout field strides must be registered in TVM reflection")
        });
        impl Conv3DAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CONV3DATTRS__STRIDES
                    .get(&__obj)
                    .expect("non-layout field strides should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Conv1DTransposeAttrs"]
        pub struct Conv1DTransposeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            output_padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            groups: i32,
            _gap0: [u8; 4],
            data_layout: tvm_ffi::String,
            kernel_layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            out_dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Conv1DTransposeAttrs {
            data: tvm_ffi::object::ObjectArc<Conv1DTransposeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Conv1DTransposeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Conv1DTransposeAttrs {
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_output_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.output_padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_groups(&self) -> i32 {
                self.data.groups
            }
            pub fn get_data_layout(&self) -> tvm_ffi::String {
                self.data.data_layout.clone()
            }
            pub fn get_kernel_layout(&self) -> tvm_ffi::String {
                self.data.kernel_layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.out_dtype
            }
        }

        static FIELD_RELAX_ATTRS_CONV1DTRANSPOSEATTRS__STRIDES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Conv1DTransposeAttrs", "strides")
                .expect("non-layout field strides must be registered in TVM reflection")
        });
        impl Conv1DTransposeAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CONV1DTRANSPOSEATTRS__STRIDES
                    .get(&__obj)
                    .expect("non-layout field strides should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Conv2DTransposeAttrs"]
        pub struct Conv2DTransposeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            output_padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            groups: i32,
            _gap0: [u8; 4],
            data_layout: tvm_ffi::String,
            kernel_layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            out_dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Conv2DTransposeAttrs {
            data: tvm_ffi::object::ObjectArc<Conv2DTransposeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Conv2DTransposeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Conv2DTransposeAttrs {
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_output_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.output_padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_groups(&self) -> i32 {
                self.data.groups
            }
            pub fn get_data_layout(&self) -> tvm_ffi::String {
                self.data.data_layout.clone()
            }
            pub fn get_kernel_layout(&self) -> tvm_ffi::String {
                self.data.kernel_layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.out_dtype
            }
        }

        static FIELD_RELAX_ATTRS_CONV2DTRANSPOSEATTRS__STRIDES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Conv2DTransposeAttrs", "strides")
                .expect("non-layout field strides must be registered in TVM reflection")
        });
        impl Conv2DTransposeAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CONV2DTRANSPOSEATTRS__STRIDES
                    .get(&__obj)
                    .expect("non-layout field strides should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.SoftmaxAttrs"]
        pub struct SoftmaxAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SoftmaxAttrs {
            data: tvm_ffi::object::ObjectArc<SoftmaxAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SoftmaxAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SoftmaxAttrs {}

        static FIELD_RELAX_ATTRS_SOFTMAXATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.SoftmaxAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl SoftmaxAttrs {
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SOFTMAXATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.LeakyReluAttrs"]
        pub struct LeakyReluAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LeakyReluAttrs {
            data: tvm_ffi::object::ObjectArc<LeakyReluAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LeakyReluAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl LeakyReluAttrs {}

        static FIELD_RELAX_ATTRS_LEAKYRELUATTRS__ALPHA: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<f64>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.LeakyReluAttrs", "alpha")
                .expect("non-layout field alpha must be registered in TVM reflection")
        });
        impl LeakyReluAttrs {
            pub fn get_alpha(&self) -> f64 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_LEAKYRELUATTRS__ALPHA
                    .get(&__obj)
                    .expect("non-layout field alpha should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.SoftplusAttrs"]
        pub struct SoftplusAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            threshold: f64,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SoftplusAttrs {
            data: tvm_ffi::object::ObjectArc<SoftplusAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SoftplusAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SoftplusAttrs {
            pub fn get_threshold(&self) -> f64 {
                self.data.threshold
            }
        }

        static FIELD_RELAX_ATTRS_SOFTPLUSATTRS__BETA: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<f64>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.SoftplusAttrs", "beta")
                .expect("non-layout field beta must be registered in TVM reflection")
        });
        impl SoftplusAttrs {
            pub fn get_beta(&self) -> f64 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SOFTPLUSATTRS__BETA
                    .get(&__obj)
                    .expect("non-layout field beta should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.PReluAttrs"]
        pub struct PReluAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PReluAttrs {
            data: tvm_ffi::object::ObjectArc<PReluAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PReluAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl PReluAttrs {}

        static FIELD_RELAX_ATTRS_PRELUATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.PReluAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl PReluAttrs {
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_PRELUATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.BatchNormAttrs"]
        pub struct BatchNormAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            epsilon: f64,
            center: bool,
            scale: bool,
            _gap0: [u8; 6],
            momentum: f64,
            training: bool,
            _gap1: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct BatchNormAttrs {
            data: tvm_ffi::object::ObjectArc<BatchNormAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(BatchNormAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl BatchNormAttrs {
            pub fn get_epsilon(&self) -> f64 {
                self.data.epsilon
            }
            pub fn get_center(&self) -> bool {
                self.data.center
            }
            pub fn get_scale(&self) -> bool {
                self.data.scale
            }
            pub fn get_momentum(&self) -> f64 {
                self.data.momentum
            }
            pub fn get_training(&self) -> bool {
                self.data.training
            }
        }

        static FIELD_RELAX_ATTRS_BATCHNORMATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.BatchNormAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl BatchNormAttrs {
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_BATCHNORMATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.LayerNormAttrs"]
        pub struct LayerNormAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            epsilon: f64,
            center: bool,
            scale: bool,
            _gap0: [u8; 14],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LayerNormAttrs {
            data: tvm_ffi::object::ObjectArc<LayerNormAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LayerNormAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl LayerNormAttrs {
            pub fn get_epsilon(&self) -> f64 {
                self.data.epsilon
            }
            pub fn get_center(&self) -> bool {
                self.data.center
            }
            pub fn get_scale(&self) -> bool {
                self.data.scale
            }
        }

        static FIELD_RELAX_ATTRS_LAYERNORMATTRS__AXES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.LayerNormAttrs", "axes")
                .expect("non-layout field axes must be registered in TVM reflection")
        });
        impl LayerNormAttrs {
            pub fn get_axes(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_LAYERNORMATTRS__AXES
                    .get(&__obj)
                    .expect("non-layout field axes should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.GroupNormAttrs"]
        pub struct GroupNormAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            axes: tvm_ffi::Array<crate::ir::IntImm>,
            epsilon: f64,
            center: bool,
            scale: bool,
            _gap0: [u8; 14],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct GroupNormAttrs {
            data: tvm_ffi::object::ObjectArc<GroupNormAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(GroupNormAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl GroupNormAttrs {
            pub fn get_axes(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.axes.clone()
            }
            pub fn get_epsilon(&self) -> f64 {
                self.data.epsilon
            }
            pub fn get_center(&self) -> bool {
                self.data.center
            }
            pub fn get_scale(&self) -> bool {
                self.data.scale
            }
        }

        static FIELD_RELAX_ATTRS_GROUPNORMATTRS__NUM_GROUPS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.GroupNormAttrs", "num_groups")
                .expect("non-layout field num_groups must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_GROUPNORMATTRS__CHANNEL_AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.GroupNormAttrs", "channel_axis")
                .expect("non-layout field channel_axis must be registered in TVM reflection")
        });
        impl GroupNormAttrs {
            pub fn get_num_groups(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_GROUPNORMATTRS__NUM_GROUPS
                    .get(&__obj)
                    .expect("non-layout field num_groups should be accessible")
            }
            pub fn get_channel_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_GROUPNORMATTRS__CHANNEL_AXIS
                    .get(&__obj)
                    .expect("non-layout field channel_axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.InstanceNormAttrs"]
        pub struct InstanceNormAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            axes: tvm_ffi::Array<crate::ir::IntImm>,
            epsilon: f64,
            center: bool,
            scale: bool,
            _gap0: [u8; 14],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct InstanceNormAttrs {
            data: tvm_ffi::object::ObjectArc<InstanceNormAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(InstanceNormAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl InstanceNormAttrs {
            pub fn get_axes(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.axes.clone()
            }
            pub fn get_epsilon(&self) -> f64 {
                self.data.epsilon
            }
            pub fn get_center(&self) -> bool {
                self.data.center
            }
            pub fn get_scale(&self) -> bool {
                self.data.scale
            }
        }

        static FIELD_RELAX_ATTRS_INSTANCENORMATTRS__CHANNEL_AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.InstanceNormAttrs",
                "channel_axis",
            )
            .expect("non-layout field channel_axis must be registered in TVM reflection")
        });
        impl InstanceNormAttrs {
            pub fn get_channel_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_INSTANCENORMATTRS__CHANNEL_AXIS
                    .get(&__obj)
                    .expect("non-layout field channel_axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.RMSNormAttrs"]
        pub struct RMSNormAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            epsilon: f64,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct RMSNormAttrs {
            data: tvm_ffi::object::ObjectArc<RMSNormAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(RMSNormAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl RMSNormAttrs {
            pub fn get_epsilon(&self) -> f64 {
                self.data.epsilon
            }
        }

        static FIELD_RELAX_ATTRS_RMSNORMATTRS__AXES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.RMSNormAttrs", "axes")
                .expect("non-layout field axes must be registered in TVM reflection")
        });
        impl RMSNormAttrs {
            pub fn get_axes(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_RMSNORMATTRS__AXES
                    .get(&__obj)
                    .expect("non-layout field axes should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.NLLLossAttrs"]
        pub struct NLLLossAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            ignore_index: i32,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct NLLLossAttrs {
            data: tvm_ffi::object::ObjectArc<NLLLossAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(NLLLossAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl NLLLossAttrs {
            pub fn get_ignore_index(&self) -> i32 {
                self.data.ignore_index
            }
        }

        static FIELD_RELAX_ATTRS_NLLLOSSATTRS__REDUCTION: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.NLLLossAttrs", "reduction")
                .expect("non-layout field reduction must be registered in TVM reflection")
        });
        impl NLLLossAttrs {
            pub fn get_reduction(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_NLLLOSSATTRS__REDUCTION
                    .get(&__obj)
                    .expect("non-layout field reduction should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.DropoutAttrs"]
        pub struct DropoutAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DropoutAttrs {
            data: tvm_ffi::object::ObjectArc<DropoutAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DropoutAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl DropoutAttrs {}

        static FIELD_RELAX_ATTRS_DROPOUTATTRS__RATE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<f64>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.DropoutAttrs", "rate")
                .expect("non-layout field rate must be registered in TVM reflection")
        });
        impl DropoutAttrs {
            pub fn get_rate(&self) -> f64 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_DROPOUTATTRS__RATE
                    .get(&__obj)
                    .expect("non-layout field rate should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.PadAttrs"]
        pub struct PadAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            pad_value: f64,
            pad_mode: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PadAttrs {
            data: tvm_ffi::object::ObjectArc<PadAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PadAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl PadAttrs {
            pub fn get_pad_value(&self) -> f64 {
                self.data.pad_value
            }
            pub fn get_pad_mode(&self) -> tvm_ffi::String {
                self.data.pad_mode.clone()
            }
        }

        static FIELD_RELAX_ATTRS_PADATTRS__PAD_WIDTH: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.PadAttrs", "pad_width")
                .expect("non-layout field pad_width must be registered in TVM reflection")
        });
        impl PadAttrs {
            pub fn get_pad_width(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_PADATTRS__PAD_WIDTH
                    .get(&__obj)
                    .expect("non-layout field pad_width should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.PixelShuffleAttrs"]
        pub struct PixelShuffleAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PixelShuffleAttrs {
            data: tvm_ffi::object::ObjectArc<PixelShuffleAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PixelShuffleAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl PixelShuffleAttrs {}

        static FIELD_RELAX_ATTRS_PIXELSHUFFLEATTRS__UPSCALE_FACTOR: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.PixelShuffleAttrs",
                "upscale_factor",
            )
            .expect("non-layout field upscale_factor must be registered in TVM reflection")
        });
        impl PixelShuffleAttrs {
            pub fn get_upscale_factor(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_PIXELSHUFFLEATTRS__UPSCALE_FACTOR
                    .get(&__obj)
                    .expect("non-layout field upscale_factor should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Pool1DAttrs"]
        pub struct Pool1DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            strides: tvm_ffi::Array<crate::ir::IntImm>,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            ceil_mode: bool,
            count_include_pad: bool,
            _gap0: [u8; 6],
            layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            _gap1: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Pool1DAttrs {
            data: tvm_ffi::object::ObjectArc<Pool1DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Pool1DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Pool1DAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.strides.clone()
            }
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_ceil_mode(&self) -> bool {
                self.data.ceil_mode
            }
            pub fn get_count_include_pad(&self) -> bool {
                self.data.count_include_pad
            }
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
        }

        static FIELD_RELAX_ATTRS_POOL1DATTRS__POOL_SIZE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Pool1DAttrs", "pool_size")
                .expect("non-layout field pool_size must be registered in TVM reflection")
        });
        impl Pool1DAttrs {
            pub fn get_pool_size(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_POOL1DATTRS__POOL_SIZE
                    .get(&__obj)
                    .expect("non-layout field pool_size should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Pool2DAttrs"]
        pub struct Pool2DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            strides: tvm_ffi::Array<crate::ir::IntImm>,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            ceil_mode: bool,
            count_include_pad: bool,
            _gap0: [u8; 6],
            layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            _gap1: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Pool2DAttrs {
            data: tvm_ffi::object::ObjectArc<Pool2DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Pool2DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Pool2DAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.strides.clone()
            }
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_ceil_mode(&self) -> bool {
                self.data.ceil_mode
            }
            pub fn get_count_include_pad(&self) -> bool {
                self.data.count_include_pad
            }
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
        }

        static FIELD_RELAX_ATTRS_POOL2DATTRS__POOL_SIZE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Pool2DAttrs", "pool_size")
                .expect("non-layout field pool_size must be registered in TVM reflection")
        });
        impl Pool2DAttrs {
            pub fn get_pool_size(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_POOL2DATTRS__POOL_SIZE
                    .get(&__obj)
                    .expect("non-layout field pool_size should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.Pool3DAttrs"]
        pub struct Pool3DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            strides: tvm_ffi::Array<crate::ir::IntImm>,
            padding: tvm_ffi::Array<crate::ir::IntImm>,
            dilation: tvm_ffi::Array<crate::ir::IntImm>,
            ceil_mode: bool,
            count_include_pad: bool,
            _gap0: [u8; 6],
            layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            _gap1: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Pool3DAttrs {
            data: tvm_ffi::object::ObjectArc<Pool3DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Pool3DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl Pool3DAttrs {
            pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.strides.clone()
            }
            pub fn get_padding(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.padding.clone()
            }
            pub fn get_dilation(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.dilation.clone()
            }
            pub fn get_ceil_mode(&self) -> bool {
                self.data.ceil_mode
            }
            pub fn get_count_include_pad(&self) -> bool {
                self.data.count_include_pad
            }
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
        }

        static FIELD_RELAX_ATTRS_POOL3DATTRS__POOL_SIZE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.Pool3DAttrs", "pool_size")
                .expect("non-layout field pool_size must be registered in TVM reflection")
        });
        impl Pool3DAttrs {
            pub fn get_pool_size(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_POOL3DATTRS__POOL_SIZE
                    .get(&__obj)
                    .expect("non-layout field pool_size should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AdaptivePool1DAttrs"]
        pub struct AdaptivePool1DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AdaptivePool1DAttrs {
            data: tvm_ffi::object::ObjectArc<AdaptivePool1DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AdaptivePool1DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AdaptivePool1DAttrs {
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
        }

        static FIELD_RELAX_ATTRS_ADAPTIVEPOOL1DATTRS__OUTPUT_SIZE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::ir::IntImm>>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.AdaptivePool1DAttrs",
                "output_size",
            )
            .expect("non-layout field output_size must be registered in TVM reflection")
        });
        impl AdaptivePool1DAttrs {
            pub fn get_output_size(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ADAPTIVEPOOL1DATTRS__OUTPUT_SIZE
                    .get(&__obj)
                    .expect("non-layout field output_size should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AdaptivePool2DAttrs"]
        pub struct AdaptivePool2DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AdaptivePool2DAttrs {
            data: tvm_ffi::object::ObjectArc<AdaptivePool2DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AdaptivePool2DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AdaptivePool2DAttrs {
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
        }

        static FIELD_RELAX_ATTRS_ADAPTIVEPOOL2DATTRS__OUTPUT_SIZE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::ir::IntImm>>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.AdaptivePool2DAttrs",
                "output_size",
            )
            .expect("non-layout field output_size must be registered in TVM reflection")
        });
        impl AdaptivePool2DAttrs {
            pub fn get_output_size(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ADAPTIVEPOOL2DATTRS__OUTPUT_SIZE
                    .get(&__obj)
                    .expect("non-layout field output_size should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AdaptivePool3DAttrs"]
        pub struct AdaptivePool3DAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            layout: tvm_ffi::String,
            out_layout: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AdaptivePool3DAttrs {
            data: tvm_ffi::object::ObjectArc<AdaptivePool3DAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AdaptivePool3DAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AdaptivePool3DAttrs {
            pub fn get_layout(&self) -> tvm_ffi::String {
                self.data.layout.clone()
            }
            pub fn get_out_layout(&self) -> tvm_ffi::String {
                self.data.out_layout.clone()
            }
        }

        static FIELD_RELAX_ATTRS_ADAPTIVEPOOL3DATTRS__OUTPUT_SIZE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::ir::IntImm>>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.AdaptivePool3DAttrs",
                "output_size",
            )
            .expect("non-layout field output_size must be registered in TVM reflection")
        });
        impl AdaptivePool3DAttrs {
            pub fn get_output_size(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ADAPTIVEPOOL3DATTRS__OUTPUT_SIZE
                    .get(&__obj)
                    .expect("non-layout field output_size should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.CallTIRWithGradAttrs"]
        pub struct CallTIRWithGradAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            te_grad_kwargs: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            _gap1: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct CallTIRWithGradAttrs {
            data: tvm_ffi::object::ObjectArc<CallTIRWithGradAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(CallTIRWithGradAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl CallTIRWithGradAttrs {
            pub fn get_te_grad_kwargs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
                self.data.te_grad_kwargs.clone()
            }
        }

        static FIELD_RELAX_ATTRS_CALLTIRWITHGRADATTRS__TE_GRAD_NAME: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.CallTIRWithGradAttrs",
                "te_grad_name",
            )
            .expect("non-layout field te_grad_name must be registered in TVM reflection")
        });
        impl CallTIRWithGradAttrs {
            pub fn get_te_grad_name(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CALLTIRWITHGRADATTRS__TE_GRAD_NAME
                    .get(&__obj)
                    .expect("non-layout field te_grad_name should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.CallTIRInplaceAttrs"]
        pub struct CallTIRInplaceAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct CallTIRInplaceAttrs {
            data: tvm_ffi::object::ObjectArc<CallTIRInplaceAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(CallTIRInplaceAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl CallTIRInplaceAttrs {}

        static FIELD_RELAX_ATTRS_CALLTIRINPLACEATTRS__INPLACE_INDICES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.CallTIRInplaceAttrs",
                "inplace_indices",
            )
            .expect("non-layout field inplace_indices must be registered in TVM reflection")
        });
        impl CallTIRInplaceAttrs {
            pub fn get_inplace_indices(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CALLTIRINPLACEATTRS__INPLACE_INDICES
                    .get(&__obj)
                    .expect("non-layout field inplace_indices should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.CallInplacePackedAttrs"]
        pub struct CallInplacePackedAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct CallInplacePackedAttrs {
            data: tvm_ffi::object::ObjectArc<CallInplacePackedAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(CallInplacePackedAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl CallInplacePackedAttrs {}

        static FIELD_RELAX_ATTRS_CALLINPLACEPACKEDATTRS__INPLACE_INDICES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.CallInplacePackedAttrs",
                "inplace_indices",
            )
            .expect("non-layout field inplace_indices must be registered in TVM reflection")
        });
        impl CallInplacePackedAttrs {
            pub fn get_inplace_indices(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CALLINPLACEPACKEDATTRS__INPLACE_INDICES
                    .get(&__obj)
                    .expect("non-layout field inplace_indices should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ToVDeviceAttrs"]
        pub struct ToVDeviceAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ToVDeviceAttrs {
            data: tvm_ffi::object::ObjectArc<ToVDeviceAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ToVDeviceAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ToVDeviceAttrs {}

        static FIELD_RELAX_ATTRS_TOVDEVICEATTRS__DST_VDEVICE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::ir::VDevice>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ToVDeviceAttrs", "dst_vdevice")
                .expect("non-layout field dst_vdevice must be registered in TVM reflection")
        });
        impl ToVDeviceAttrs {
            pub fn get_dst_vdevice(&self) -> crate::ir::VDevice {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_TOVDEVICEATTRS__DST_VDEVICE
                    .get(&__obj)
                    .expect("non-layout field dst_vdevice should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.HintOnDeviceAttrs"]
        pub struct HintOnDeviceAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            memory_scope: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct HintOnDeviceAttrs {
            data: tvm_ffi::object::ObjectArc<HintOnDeviceAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(HintOnDeviceAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl HintOnDeviceAttrs {
            pub fn get_memory_scope(&self) -> tvm_ffi::String {
                self.data.memory_scope.clone()
            }
        }

        static FIELD_RELAX_ATTRS_HINTONDEVICEATTRS__DEVICE_TYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.HintOnDeviceAttrs",
                "device_type",
            )
            .expect("non-layout field device_type must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_HINTONDEVICEATTRS__INDEX: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.HintOnDeviceAttrs", "index")
                .expect("non-layout field index must be registered in TVM reflection")
        });
        impl HintOnDeviceAttrs {
            pub fn get_device_type(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_HINTONDEVICEATTRS__DEVICE_TYPE
                    .get(&__obj)
                    .expect("non-layout field device_type should be accessible")
            }
            pub fn get_index(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_HINTONDEVICEATTRS__INDEX
                    .get(&__obj)
                    .expect("non-layout field index should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.InitAttrs"]
        pub struct InitAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct InitAttrs {
            data: tvm_ffi::object::ObjectArc<InitAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(InitAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl InitAttrs {}

        static FIELD_RELAX_ATTRS_INITATTRS__DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.InitAttrs", "dtype")
                .expect("non-layout field dtype must be registered in TVM reflection")
        });
        impl InitAttrs {
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_INITATTRS__DTYPE
                    .get(&__obj)
                    .expect("non-layout field dtype should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.TriluAttrs"]
        pub struct TriluAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TriluAttrs {
            data: tvm_ffi::object::ObjectArc<TriluAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TriluAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl TriluAttrs {}

        static FIELD_RELAX_ATTRS_TRILUATTRS__K: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.TriluAttrs", "k")
                .expect("non-layout field k must be registered in TVM reflection")
        });
        impl TriluAttrs {
            pub fn get_k(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_TRILUATTRS__K
                    .get(&__obj)
                    .expect("non-layout field k should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AstypeAttrs"]
        pub struct AstypeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AstypeAttrs {
            data: tvm_ffi::object::ObjectArc<AstypeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AstypeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AstypeAttrs {}

        static FIELD_RELAX_ATTRS_ASTYPEATTRS__DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.AstypeAttrs", "dtype")
                .expect("non-layout field dtype must be registered in TVM reflection")
        });
        impl AstypeAttrs {
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ASTYPEATTRS__DTYPE
                    .get(&__obj)
                    .expect("non-layout field dtype should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.WrapParamAttrs"]
        pub struct WrapParamAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct WrapParamAttrs {
            data: tvm_ffi::object::ObjectArc<WrapParamAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(WrapParamAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl WrapParamAttrs {}

        static FIELD_RELAX_ATTRS_WRAPPARAMATTRS__DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.WrapParamAttrs", "dtype")
                .expect("non-layout field dtype must be registered in TVM reflection")
        });
        impl WrapParamAttrs {
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_WRAPPARAMATTRS__DTYPE
                    .get(&__obj)
                    .expect("non-layout field dtype should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.TakeAttrs"]
        pub struct TakeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            mode: tvm_ffi::String,
            _gap1: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TakeAttrs {
            data: tvm_ffi::object::ObjectArc<TakeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TakeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl TakeAttrs {
            pub fn get_mode(&self) -> tvm_ffi::String {
                self.data.mode.clone()
            }
        }

        static FIELD_RELAX_ATTRS_TAKEATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.TakeAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl TakeAttrs {
            pub fn get_axis(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_TAKEATTRS__AXIS
                    .get_any(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.StridedSliceAttrs"]
        pub struct StridedSliceAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StridedSliceAttrs {
            data: tvm_ffi::object::ObjectArc<StridedSliceAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StridedSliceAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl StridedSliceAttrs {}

        static FIELD_RELAX_ATTRS_STRIDEDSLICEATTRS__ASSUME_INBOUND: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.StridedSliceAttrs",
                "assume_inbound",
            )
            .expect("non-layout field assume_inbound must be registered in TVM reflection")
        });
        impl StridedSliceAttrs {
            pub fn get_assume_inbound(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_STRIDEDSLICEATTRS__ASSUME_INBOUND
                    .get(&__obj)
                    .expect("non-layout field assume_inbound should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.MatmulAttrs"]
        pub struct MatmulAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct MatmulAttrs {
            data: tvm_ffi::object::ObjectArc<MatmulAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(MatmulAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl MatmulAttrs {}

        static FIELD_RELAX_ATTRS_MATMULATTRS__OUT_DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.MatmulAttrs", "out_dtype")
                .expect("non-layout field out_dtype must be registered in TVM reflection")
        });
        impl MatmulAttrs {
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_MATMULATTRS__OUT_DTYPE
                    .get(&__obj)
                    .expect("non-layout field out_dtype should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.EinsumAttrs"]
        pub struct EinsumAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct EinsumAttrs {
            data: tvm_ffi::object::ObjectArc<EinsumAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(EinsumAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl EinsumAttrs {}

        static FIELD_RELAX_ATTRS_EINSUMATTRS__SUBSCRIPTS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.EinsumAttrs", "subscripts")
                .expect("non-layout field subscripts must be registered in TVM reflection")
        });
        impl EinsumAttrs {
            pub fn get_subscripts(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_EINSUMATTRS__SUBSCRIPTS
                    .get(&__obj)
                    .expect("non-layout field subscripts should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ConcatAttrs"]
        pub struct ConcatAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ConcatAttrs {
            data: tvm_ffi::object::ObjectArc<ConcatAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ConcatAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ConcatAttrs {}

        static FIELD_RELAX_ATTRS_CONCATATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ConcatAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl ConcatAttrs {
            pub fn get_axis(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_CONCATATTRS__AXIS
                    .get_any(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ExpandDimsAttrs"]
        pub struct ExpandDimsAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExpandDimsAttrs {
            data: tvm_ffi::object::ObjectArc<ExpandDimsAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExpandDimsAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ExpandDimsAttrs {}

        static FIELD_RELAX_ATTRS_EXPANDDIMSATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ExpandDimsAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl ExpandDimsAttrs {
            pub fn get_axis(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_EXPANDDIMSATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.LayoutTransformAttrs"]
        pub struct LayoutTransformAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            pad_value: Option<crate::relax::expr::PrimValue>,
            axis_separators: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            input_axis_separators: Option<tvm_ffi::Array<crate::ir::IntImm>>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LayoutTransformAttrs {
            data: tvm_ffi::object::ObjectArc<LayoutTransformAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LayoutTransformAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl LayoutTransformAttrs {
            pub fn get_pad_value(&self) -> Option<crate::relax::expr::PrimValue> {
                self.data.pad_value.clone()
            }
            pub fn get_axis_separators(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                self.data.axis_separators.clone()
            }
            pub fn get_input_axis_separators(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                self.data.input_axis_separators.clone()
            }
        }

        static FIELD_RELAX_ATTRS_LAYOUTTRANSFORMATTRS__INDEX_MAP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::tir::IndexMap>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.LayoutTransformAttrs",
                "index_map",
            )
            .expect("non-layout field index_map must be registered in TVM reflection")
        });
        impl LayoutTransformAttrs {
            pub fn get_index_map(&self) -> crate::tir::IndexMap {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_LAYOUTTRANSFORMATTRS__INDEX_MAP
                    .get(&__obj)
                    .expect("non-layout field index_map should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.PermuteDimsAttrs"]
        pub struct PermuteDimsAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PermuteDimsAttrs {
            data: tvm_ffi::object::ObjectArc<PermuteDimsAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PermuteDimsAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl PermuteDimsAttrs {}

        static FIELD_RELAX_ATTRS_PERMUTEDIMSATTRS__AXES: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::ir::IntImm>>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.PermuteDimsAttrs", "axes")
                .expect("non-layout field axes must be registered in TVM reflection")
        });
        impl PermuteDimsAttrs {
            pub fn get_axes(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_PERMUTEDIMSATTRS__AXES
                    .get(&__obj)
                    .expect("non-layout field axes should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.SplitAttrs"]
        pub struct SplitAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            axis: i32,
            _gap0: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SplitAttrs {
            data: tvm_ffi::object::ObjectArc<SplitAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SplitAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SplitAttrs {
            pub fn get_axis(&self) -> i32 {
                self.data.axis
            }
        }

        static FIELD_RELAX_ATTRS_SPLITATTRS__INDICES_OR_SECTIONS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::object::ObjectRef>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.SplitAttrs",
                "indices_or_sections",
            )
            .expect("non-layout field indices_or_sections must be registered in TVM reflection")
        });
        impl SplitAttrs {
            pub fn get_indices_or_sections(&self) -> tvm_ffi::object::ObjectRef {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SPLITATTRS__INDICES_OR_SECTIONS
                    .get(&__obj)
                    .expect("non-layout field indices_or_sections should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.SqueezeAttrs"]
        pub struct SqueezeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SqueezeAttrs {
            data: tvm_ffi::object::ObjectArc<SqueezeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SqueezeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SqueezeAttrs {}

        static FIELD_RELAX_ATTRS_SQUEEZEATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::ir::IntImm>>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.SqueezeAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl SqueezeAttrs {
            pub fn get_axis(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SQUEEZEATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.StackAttrs"]
        pub struct StackAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StackAttrs {
            data: tvm_ffi::object::ObjectArc<StackAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StackAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl StackAttrs {}

        static FIELD_RELAX_ATTRS_STACKATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.StackAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl StackAttrs {
            pub fn get_axis(&self) -> Option<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_STACKATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.RepeatAttrs"]
        pub struct RepeatAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 24],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct RepeatAttrs {
            data: tvm_ffi::object::ObjectArc<RepeatAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(RepeatAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl RepeatAttrs {}

        static FIELD_RELAX_ATTRS_REPEATATTRS__REPEATS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.RepeatAttrs", "repeats")
                .expect("non-layout field repeats must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_REPEATATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.RepeatAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl RepeatAttrs {
            pub fn get_repeats(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_REPEATATTRS__REPEATS
                    .get(&__obj)
                    .expect("non-layout field repeats should be accessible")
            }
            pub fn get_axis(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_REPEATATTRS__AXIS
                    .get_any(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.TileAttrs"]
        pub struct TileAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TileAttrs {
            data: tvm_ffi::object::ObjectArc<TileAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TileAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl TileAttrs {}

        static FIELD_RELAX_ATTRS_TILEATTRS__REPEATS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::IntImm>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.TileAttrs", "repeats")
                .expect("non-layout field repeats must be registered in TVM reflection")
        });
        impl TileAttrs {
            pub fn get_repeats(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_TILEATTRS__REPEATS
                    .get(&__obj)
                    .expect("non-layout field repeats should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.FlipAttrs"]
        pub struct FlipAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct FlipAttrs {
            data: tvm_ffi::object::ObjectArc<FlipAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(FlipAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl FlipAttrs {}

        static FIELD_RELAX_ATTRS_FLIPATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::ir::IntImm>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.FlipAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl FlipAttrs {
            pub fn get_axis(&self) -> crate::ir::IntImm {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_FLIPATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.GatherElementsAttrs"]
        pub struct GatherElementsAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct GatherElementsAttrs {
            data: tvm_ffi::object::ObjectArc<GatherElementsAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(GatherElementsAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl GatherElementsAttrs {}

        static FIELD_RELAX_ATTRS_GATHERELEMENTSATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::ir::IntImm>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.GatherElementsAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl GatherElementsAttrs {
            pub fn get_axis(&self) -> crate::ir::IntImm {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_GATHERELEMENTSATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.GatherNDAttrs"]
        pub struct GatherNDAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct GatherNDAttrs {
            data: tvm_ffi::object::ObjectArc<GatherNDAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(GatherNDAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl GatherNDAttrs {}

        static FIELD_RELAX_ATTRS_GATHERNDATTRS__BATCH_DIMS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::ir::IntImm>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.GatherNDAttrs", "batch_dims")
                .expect("non-layout field batch_dims must be registered in TVM reflection")
        });
        impl GatherNDAttrs {
            pub fn get_batch_dims(&self) -> crate::ir::IntImm {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_GATHERNDATTRS__BATCH_DIMS
                    .get(&__obj)
                    .expect("non-layout field batch_dims should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.IndexPutAttrs"]
        pub struct IndexPutAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IndexPutAttrs {
            data: tvm_ffi::object::ObjectArc<IndexPutAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IndexPutAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl IndexPutAttrs {}

        static FIELD_RELAX_ATTRS_INDEXPUTATTRS__ACCUMULATE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.IndexPutAttrs", "accumulate")
                .expect("non-layout field accumulate must be registered in TVM reflection")
        });
        impl IndexPutAttrs {
            pub fn get_accumulate(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_INDEXPUTATTRS__ACCUMULATE
                    .get(&__obj)
                    .expect("non-layout field accumulate should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.MeshgridAttrs"]
        pub struct MeshgridAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct MeshgridAttrs {
            data: tvm_ffi::object::ObjectArc<MeshgridAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(MeshgridAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl MeshgridAttrs {}

        static FIELD_RELAX_ATTRS_MESHGRIDATTRS__INDEXING: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.MeshgridAttrs", "indexing")
                .expect("non-layout field indexing must be registered in TVM reflection")
        });
        impl MeshgridAttrs {
            pub fn get_indexing(&self) -> Option<tvm_ffi::String> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_MESHGRIDATTRS__INDEXING
                    .get(&__obj)
                    .expect("non-layout field indexing should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ScatterElementsAttrs"]
        pub struct ScatterElementsAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            reduction: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ScatterElementsAttrs {
            data: tvm_ffi::object::ObjectArc<ScatterElementsAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ScatterElementsAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ScatterElementsAttrs {
            pub fn get_reduction(&self) -> tvm_ffi::String {
                self.data.reduction.clone()
            }
        }

        static FIELD_RELAX_ATTRS_SCATTERELEMENTSATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::ir::IntImm>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ScatterElementsAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl ScatterElementsAttrs {
            pub fn get_axis(&self) -> crate::ir::IntImm {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SCATTERELEMENTSATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ScatterNDAttrs"]
        pub struct ScatterNDAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ScatterNDAttrs {
            data: tvm_ffi::object::ObjectArc<ScatterNDAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ScatterNDAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ScatterNDAttrs {}

        static FIELD_RELAX_ATTRS_SCATTERNDATTRS__REDUCTION: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ScatterNDAttrs", "reduction")
                .expect("non-layout field reduction must be registered in TVM reflection")
        });
        impl ScatterNDAttrs {
            pub fn get_reduction(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SCATTERNDATTRS__REDUCTION
                    .get(&__obj)
                    .expect("non-layout field reduction should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.SliceScatterAttrs"]
        pub struct SliceScatterAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SliceScatterAttrs {
            data: tvm_ffi::object::ObjectArc<SliceScatterAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SliceScatterAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SliceScatterAttrs {}

        static FIELD_RELAX_ATTRS_SLICESCATTERATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.SliceScatterAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl SliceScatterAttrs {
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SLICESCATTERATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.OneHotAttrs"]
        pub struct OneHotAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct OneHotAttrs {
            data: tvm_ffi::object::ObjectArc<OneHotAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(OneHotAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl OneHotAttrs {}

        static FIELD_RELAX_ATTRS_ONEHOTATTRS__DEPTH: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.OneHotAttrs", "depth")
                .expect("non-layout field depth must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_ONEHOTATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.OneHotAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl OneHotAttrs {
            pub fn get_depth(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ONEHOTATTRS__DEPTH
                    .get(&__obj)
                    .expect("non-layout field depth should be accessible")
            }
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ONEHOTATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.QuantizeAttrs"]
        pub struct QuantizeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct QuantizeAttrs {
            data: tvm_ffi::object::ObjectArc<QuantizeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(QuantizeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl QuantizeAttrs {}

        static FIELD_RELAX_ATTRS_QUANTIZEATTRS__OUT_DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.QuantizeAttrs", "out_dtype")
                .expect("non-layout field out_dtype must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_QUANTIZEATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.QuantizeAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl QuantizeAttrs {
            pub fn get_out_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_QUANTIZEATTRS__OUT_DTYPE
                    .get(&__obj)
                    .expect("non-layout field out_dtype should be accessible")
            }
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_QUANTIZEATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.MultinomialFromUniformAttrs"]
        pub struct MultinomialFromUniformAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct MultinomialFromUniformAttrs {
            data: tvm_ffi::object::ObjectArc<MultinomialFromUniformAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(MultinomialFromUniformAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl MultinomialFromUniformAttrs {}

        static FIELD_RELAX_ATTRS_MULTINOMIALFROMUNIFORMATTRS__DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.attrs.MultinomialFromUniformAttrs",
                "dtype",
            )
            .expect("non-layout field dtype must be registered in TVM reflection")
        });
        impl MultinomialFromUniformAttrs {
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_MULTINOMIALFROMUNIFORMATTRS__DTYPE
                    .get(&__obj)
                    .expect("non-layout field dtype should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ArgmaxArgminAttrs"]
        pub struct ArgmaxArgminAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            keepdims: bool,
            _gap1: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ArgmaxArgminAttrs {
            data: tvm_ffi::object::ObjectArc<ArgmaxArgminAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ArgmaxArgminAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ArgmaxArgminAttrs {
            pub fn get_keepdims(&self) -> bool {
                self.data.keepdims
            }
        }

        static FIELD_RELAX_ATTRS_ARGMAXARGMINATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ArgmaxArgminAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl ArgmaxArgminAttrs {
            pub fn get_axis(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ARGMAXARGMINATTRS__AXIS
                    .get_any(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.BucketizeAttrs"]
        pub struct BucketizeAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct BucketizeAttrs {
            data: tvm_ffi::object::ObjectArc<BucketizeAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(BucketizeAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl BucketizeAttrs {}

        static FIELD_RELAX_ATTRS_BUCKETIZEATTRS__OUT_INT32: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.BucketizeAttrs", "out_int32")
                .expect("non-layout field out_int32 must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_BUCKETIZEATTRS__RIGHT: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.BucketizeAttrs", "right")
                .expect("non-layout field right must be registered in TVM reflection")
        });
        impl BucketizeAttrs {
            pub fn get_out_int32(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_BUCKETIZEATTRS__OUT_INT32
                    .get(&__obj)
                    .expect("non-layout field out_int32 should be accessible")
            }
            pub fn get_right(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_BUCKETIZEATTRS__RIGHT
                    .get(&__obj)
                    .expect("non-layout field right should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.SortAttrs"]
        pub struct SortAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SortAttrs {
            data: tvm_ffi::object::ObjectArc<SortAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SortAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SortAttrs {}

        static FIELD_RELAX_ATTRS_SORTATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.SortAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_SORTATTRS__DESCENDING: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.SortAttrs", "descending")
                .expect("non-layout field descending must be registered in TVM reflection")
        });
        impl SortAttrs {
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SORTATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
            pub fn get_descending(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SORTATTRS__DESCENDING
                    .get(&__obj)
                    .expect("non-layout field descending should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ArgsortAttrs"]
        pub struct ArgsortAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ArgsortAttrs {
            data: tvm_ffi::object::ObjectArc<ArgsortAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ArgsortAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ArgsortAttrs {}

        static FIELD_RELAX_ATTRS_ARGSORTATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ArgsortAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_ARGSORTATTRS__DESCENDING: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ArgsortAttrs", "descending")
                .expect("non-layout field descending must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_ARGSORTATTRS__DTYPE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ArgsortAttrs", "dtype")
                .expect("non-layout field dtype must be registered in TVM reflection")
        });
        impl ArgsortAttrs {
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ARGSORTATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
            pub fn get_descending(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ARGSORTATTRS__DESCENDING
                    .get(&__obj)
                    .expect("non-layout field descending should be accessible")
            }
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ARGSORTATTRS__DTYPE
                    .get(&__obj)
                    .expect("non-layout field dtype should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.TopKAttrs"]
        pub struct TopKAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            largest: bool,
            _gap0: [u8; 7],
            ret_type: tvm_ffi::String,
            dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TopKAttrs {
            data: tvm_ffi::object::ObjectArc<TopKAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TopKAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl TopKAttrs {
            pub fn get_largest(&self) -> bool {
                self.data.largest
            }
            pub fn get_ret_type(&self) -> tvm_ffi::String {
                self.data.ret_type.clone()
            }
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.dtype
            }
        }

        static FIELD_RELAX_ATTRS_TOPKATTRS__K: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.TopKAttrs", "k")
                .expect("non-layout field k must be registered in TVM reflection")
        });
        static FIELD_RELAX_ATTRS_TOPKATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.TopKAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl TopKAttrs {
            pub fn get_k(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_TOPKATTRS__K
                    .get(&__obj)
                    .expect("non-layout field k should be accessible")
            }
            pub fn get_axis(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_TOPKATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.StatisticalAttrs"]
        pub struct StatisticalAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            keepdims: bool,
            _gap0: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StatisticalAttrs {
            data: tvm_ffi::object::ObjectArc<StatisticalAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StatisticalAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl StatisticalAttrs {
            pub fn get_keepdims(&self) -> bool {
                self.data.keepdims
            }
        }

        static FIELD_RELAX_ATTRS_STATISTICALATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::ir::IntImm>>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.StatisticalAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl StatisticalAttrs {
            pub fn get_axis(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_STATISTICALATTRS__AXIS
                    .get(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.ScanopAttrs"]
        pub struct ScanopAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
            dtype: tvm_ffi::DLDataType,
            _gap1: [u8; 4],
            exclusive: crate::ir::IntImm,
            _gap2: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ScanopAttrs {
            data: tvm_ffi::object::ObjectArc<ScanopAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ScanopAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ScanopAttrs {
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.dtype
            }
            pub fn get_exclusive(&self) -> crate::ir::IntImm {
                self.data.exclusive.clone()
            }
        }

        static FIELD_RELAX_ATTRS_SCANOPATTRS__AXIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.attrs.ScanopAttrs", "axis")
                .expect("non-layout field axis must be registered in TVM reflection")
        });
        impl ScanopAttrs {
            pub fn get_axis(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_SCANOPATTRS__AXIS
                    .get_any(&__obj)
                    .expect("non-layout field axis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.attrs.AllClassNonMaximumSuppressionAttrs"]
        pub struct AllClassNonMaximumSuppressionAttrsObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AllClassNonMaximumSuppressionAttrs {
            data: tvm_ffi::object::ObjectArc<AllClassNonMaximumSuppressionAttrsObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AllClassNonMaximumSuppressionAttrs: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl AllClassNonMaximumSuppressionAttrs {}

        static FIELD_RELAX_ATTRS_ALLCLASSNONMAXIMUMSUPPRESSIONATTRS__OUTPUT_FORMAT:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "relax.attrs.AllClassNonMaximumSuppressionAttrs",
                    "output_format",
                )
                .expect("non-layout field output_format must be registered in TVM reflection")
            });
        impl AllClassNonMaximumSuppressionAttrs {
            pub fn get_output_format(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_ATTRS_ALLCLASSNONMAXIMUMSUPPRESSIONATTRS__OUTPUT_FORMAT
                    .get(&__obj)
                    .expect("non-layout field output_format should be accessible")
            }
        }
    }
    pub mod distributed {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.distributed.DeviceMesh"]
        pub struct DeviceMeshObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            shape: tvm_ffi::Shape,
            device_ids: tvm_ffi::Array<crate::ir::IntImm>,
            device_range: Option<crate::ir::Range>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DeviceMesh {
            data: tvm_ffi::object::ObjectArc<DeviceMeshObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DeviceMesh: tvm_ffi::object::ObjectRef);

        impl DeviceMesh {
            pub fn get_shape(&self) -> tvm_ffi::Shape {
                self.data.shape.clone()
            }
            pub fn get_device_ids(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.device_ids.clone()
            }
            pub fn get_device_range(&self) -> Option<crate::ir::Range> {
                self.data.device_range.clone()
            }
        }

        impl DeviceMesh {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.distributed.Placement"]
        pub struct PlacementObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            dim_specs: tvm_ffi::Array<crate::relax::distributed::PlacementSpec>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Placement {
            data: tvm_ffi::object::ObjectArc<PlacementObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Placement: tvm_ffi::object::ObjectRef);

        impl Placement {
            pub fn get_dim_specs(
                &self,
            ) -> tvm_ffi::Array<crate::relax::distributed::PlacementSpec> {
                self.data.dim_specs.clone()
            }
        }

        impl Placement {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.distributed.PlacementSpec"]
        pub struct PlacementSpecObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            axis: i32,
            kind: i32,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PlacementSpec {
            data: tvm_ffi::object::ObjectArc<PlacementSpecObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PlacementSpec: tvm_ffi::object::ObjectRef);

        impl PlacementSpec {
            pub fn get_axis(&self) -> i32 {
                self.data.axis
            }
            pub fn get_kind(&self) -> i32 {
                self.data.kind
            }
        }

        impl PlacementSpec {}
    }
    pub mod dpl {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        tvm_ffi::define_object_wrapper!(
            PatternMatchingRewriter,
            "relax.dpl.PatternMatchingRewriter"
        );

        impl PatternMatchingRewriter {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.PatternContextRewriter"]
        pub struct PatternContextRewriterObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            pattern: tvm_ffi::object::ObjectRef,
            rewriter_func: tvm_ffi::Function,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PatternContextRewriter {
            data: tvm_ffi::object::ObjectArc<PatternContextRewriterObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PatternContextRewriter: tvm_ffi::object::ObjectRef);

        impl PatternContextRewriter {
            pub fn get_pattern(&self) -> tvm_ffi::object::ObjectRef {
                self.data.pattern.clone()
            }
            pub fn get_rewriter_func(&self) -> tvm_ffi::Function {
                self.data.rewriter_func.clone()
            }
        }

        impl PatternContextRewriter {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.ExprPatternRewriter"]
        pub struct ExprPatternRewriterObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            pattern: crate::relax::dpl::DFPattern,
            func: tvm_ffi::Function,
            _gap0: [u8; 24],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExprPatternRewriter {
            data: tvm_ffi::object::ObjectArc<ExprPatternRewriterObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExprPatternRewriter: tvm_ffi::object::ObjectRef);

        impl ExprPatternRewriter {
            pub fn get_pattern(&self) -> crate::relax::dpl::DFPattern {
                self.data.pattern.clone()
            }
            pub fn get_func(&self) -> tvm_ffi::Function {
                self.data.func.clone()
            }
        }

        impl ExprPatternRewriter {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.OrRewriter"]
        pub struct OrRewriterObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            lhs: crate::relax::dpl::PatternMatchingRewriter,
            rhs: crate::relax::dpl::PatternMatchingRewriter,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct OrRewriter {
            data: tvm_ffi::object::ObjectArc<OrRewriterObj>,
        }

        tvm_ffi::impl_object_hierarchy!(OrRewriter: tvm_ffi::object::ObjectRef);

        impl OrRewriter {
            pub fn get_lhs(&self) -> crate::relax::dpl::PatternMatchingRewriter {
                self.data.lhs.clone()
            }
            pub fn get_rhs(&self) -> crate::relax::dpl::PatternMatchingRewriter {
                self.data.rhs.clone()
            }
        }

        impl OrRewriter {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.TupleRewriter"]
        pub struct TupleRewriterObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            patterns: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            func: tvm_ffi::Function,
            _gap0: [u8; 24],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TupleRewriter {
            data: tvm_ffi::object::ObjectArc<TupleRewriterObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TupleRewriter: tvm_ffi::object::ObjectRef);

        impl TupleRewriter {
            pub fn get_patterns(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.patterns.clone()
            }
            pub fn get_func(&self) -> tvm_ffi::Function {
                self.data.func.clone()
            }
        }

        impl TupleRewriter {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.PatternSeq"]
        pub struct PatternSeqObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            patterns: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _gap0: [u8; 24],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PatternSeq {
            data: tvm_ffi::object::ObjectArc<PatternSeqObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PatternSeq: tvm_ffi::object::ObjectRef);

        impl PatternSeq {
            pub fn get_patterns(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.patterns.clone()
            }
        }

        impl PatternSeq {}

        tvm_ffi::define_object_wrapper!(DFPattern, "relax.dpl.DFPattern");

        impl DFPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.ExprPattern"]
        pub struct ExprPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            expr: crate::ir::RelaxExpr,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExprPattern {
            data: tvm_ffi::object::ObjectArc<ExprPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExprPattern: tvm_ffi::object::ObjectRef);

        impl ExprPattern {
            pub fn get_expr(&self) -> crate::ir::RelaxExpr {
                self.data.expr.clone()
            }
        }

        impl ExprPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.VarPattern"]
        pub struct VarPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            name: tvm_ffi::String,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct VarPattern {
            data: tvm_ffi::object::ObjectArc<VarPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(VarPattern: tvm_ffi::object::ObjectRef);

        impl VarPattern {
            pub fn get_name(&self) -> tvm_ffi::String {
                self.data.name.clone()
            }
        }

        impl VarPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.DataflowVarPattern"]
        pub struct DataflowVarPatternObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::relax::dpl::VarPatternObj,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DataflowVarPattern {
            data: tvm_ffi::object::ObjectArc<DataflowVarPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DataflowVarPattern: crate::relax::dpl::VarPattern, tvm_ffi::object::ObjectRef);

        impl DataflowVarPattern {}

        impl DataflowVarPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.CallPattern"]
        pub struct CallPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            op: crate::relax::dpl::DFPattern,
            args: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct CallPattern {
            data: tvm_ffi::object::ObjectArc<CallPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(CallPattern: tvm_ffi::object::ObjectRef);

        impl CallPattern {
            pub fn get_op(&self) -> crate::relax::dpl::DFPattern {
                self.data.op.clone()
            }
            pub fn get_args(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.args.clone()
            }
        }

        impl CallPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.PrimArrPattern"]
        pub struct PrimArrPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            fields: tvm_ffi::Array<crate::ir::PrimExpr>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PrimArrPattern {
            data: tvm_ffi::object::ObjectArc<PrimArrPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PrimArrPattern: tvm_ffi::object::ObjectRef);

        impl PrimArrPattern {
            pub fn get_fields(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
                self.data.fields.clone()
            }
        }

        impl PrimArrPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.FunctionPattern"]
        pub struct FunctionPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            params: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            body: crate::relax::dpl::DFPattern,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct FunctionPattern {
            data: tvm_ffi::object::ObjectArc<FunctionPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(FunctionPattern: tvm_ffi::object::ObjectRef);

        impl FunctionPattern {
            pub fn get_params(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.params.clone()
            }
            pub fn get_body(&self) -> crate::relax::dpl::DFPattern {
                self.data.body.clone()
            }
        }

        impl FunctionPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.TuplePattern"]
        pub struct TuplePatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            fields: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TuplePattern {
            data: tvm_ffi::object::ObjectArc<TuplePatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TuplePattern: tvm_ffi::object::ObjectRef);

        impl TuplePattern {
            pub fn get_fields(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.fields.clone()
            }
        }

        impl TuplePattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.UnorderedTuplePattern"]
        pub struct UnorderedTuplePatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            fields: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct UnorderedTuplePattern {
            data: tvm_ffi::object::ObjectArc<UnorderedTuplePatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(UnorderedTuplePattern: tvm_ffi::object::ObjectRef);

        impl UnorderedTuplePattern {
            pub fn get_fields(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.fields.clone()
            }
        }

        impl UnorderedTuplePattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.TupleGetItemPattern"]
        pub struct TupleGetItemPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            tuple: crate::relax::dpl::DFPattern,
            index: i32,
            _gap0: [u8; 4],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TupleGetItemPattern {
            data: tvm_ffi::object::ObjectArc<TupleGetItemPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TupleGetItemPattern: tvm_ffi::object::ObjectRef);

        impl TupleGetItemPattern {
            pub fn get_tuple(&self) -> crate::relax::dpl::DFPattern {
                self.data.tuple.clone()
            }
            pub fn get_index(&self) -> i32 {
                self.data.index
            }
        }

        impl TupleGetItemPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.AndPattern"]
        pub struct AndPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            left: crate::relax::dpl::DFPattern,
            right: crate::relax::dpl::DFPattern,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AndPattern {
            data: tvm_ffi::object::ObjectArc<AndPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AndPattern: tvm_ffi::object::ObjectRef);

        impl AndPattern {
            pub fn get_left(&self) -> crate::relax::dpl::DFPattern {
                self.data.left.clone()
            }
            pub fn get_right(&self) -> crate::relax::dpl::DFPattern {
                self.data.right.clone()
            }
        }

        impl AndPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.OrPattern"]
        pub struct OrPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            left: crate::relax::dpl::DFPattern,
            right: crate::relax::dpl::DFPattern,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct OrPattern {
            data: tvm_ffi::object::ObjectArc<OrPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(OrPattern: tvm_ffi::object::ObjectRef);

        impl OrPattern {
            pub fn get_left(&self) -> crate::relax::dpl::DFPattern {
                self.data.left.clone()
            }
            pub fn get_right(&self) -> crate::relax::dpl::DFPattern {
                self.data.right.clone()
            }
        }

        impl OrPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.NotPattern"]
        pub struct NotPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            reject: crate::relax::dpl::DFPattern,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct NotPattern {
            data: tvm_ffi::object::ObjectArc<NotPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(NotPattern: tvm_ffi::object::ObjectRef);

        impl NotPattern {
            pub fn get_reject(&self) -> crate::relax::dpl::DFPattern {
                self.data.reject.clone()
            }
        }

        impl NotPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.WildcardPattern"]
        pub struct WildcardPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct WildcardPattern {
            data: tvm_ffi::object::ObjectArc<WildcardPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(WildcardPattern: tvm_ffi::object::ObjectRef);

        impl WildcardPattern {}

        impl WildcardPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.StructInfoPattern"]
        pub struct StructInfoPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            pattern: crate::relax::dpl::DFPattern,
            struct_info: crate::ir::StructInfo,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StructInfoPattern {
            data: tvm_ffi::object::ObjectArc<StructInfoPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StructInfoPattern: tvm_ffi::object::ObjectRef);

        impl StructInfoPattern {
            pub fn get_pattern(&self) -> crate::relax::dpl::DFPattern {
                self.data.pattern.clone()
            }
            pub fn get_struct_info(&self) -> crate::ir::StructInfo {
                self.data.struct_info.clone()
            }
        }

        impl StructInfoPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.ShapePattern"]
        pub struct ShapePatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            pattern: crate::relax::dpl::DFPattern,
            shape: tvm_ffi::Array<crate::ir::PrimExpr>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ShapePattern {
            data: tvm_ffi::object::ObjectArc<ShapePatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ShapePattern: tvm_ffi::object::ObjectRef);

        impl ShapePattern {
            pub fn get_pattern(&self) -> crate::relax::dpl::DFPattern {
                self.data.pattern.clone()
            }
            pub fn get_shape(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
                self.data.shape.clone()
            }
        }

        impl ShapePattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.DataTypePattern"]
        pub struct DataTypePatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            pattern: crate::relax::dpl::DFPattern,
            dtype: tvm_ffi::DLDataType,
            _gap0: [u8; 4],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DataTypePattern {
            data: tvm_ffi::object::ObjectArc<DataTypePatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DataTypePattern: tvm_ffi::object::ObjectRef);

        impl DataTypePattern {
            pub fn get_pattern(&self) -> crate::relax::dpl::DFPattern {
                self.data.pattern.clone()
            }
            pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                self.data.dtype
            }
        }

        impl DataTypePattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.AttrPattern"]
        pub struct AttrPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            pattern: crate::relax::dpl::DFPattern,
            attrs: crate::ir::DictAttrs,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AttrPattern {
            data: tvm_ffi::object::ObjectArc<AttrPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AttrPattern: tvm_ffi::object::ObjectRef);

        impl AttrPattern {
            pub fn get_pattern(&self) -> crate::relax::dpl::DFPattern {
                self.data.pattern.clone()
            }
            pub fn get_attrs(&self) -> crate::ir::DictAttrs {
                self.data.attrs.clone()
            }
        }

        impl AttrPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.ExternFuncPattern"]
        pub struct ExternFuncPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            global_symbol: tvm_ffi::String,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExternFuncPattern {
            data: tvm_ffi::object::ObjectArc<ExternFuncPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExternFuncPattern: tvm_ffi::object::ObjectRef);

        impl ExternFuncPattern {
            pub fn get_global_symbol(&self) -> tvm_ffi::String {
                self.data.global_symbol.clone()
            }
        }

        impl ExternFuncPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.ConstantPattern"]
        pub struct ConstantPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ConstantPattern {
            data: tvm_ffi::object::ObjectArc<ConstantPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ConstantPattern: tvm_ffi::object::ObjectRef);

        impl ConstantPattern {}

        impl ConstantPattern {}

        tvm_ffi::define_object_wrapper!(GlobalVarPattern, "relax.dpl.GlobalVarPattern");

        impl GlobalVarPattern {}

        tvm_ffi::define_object_wrapper!(DFConstraint, "relax.dpl.DFConstraint");

        impl DFConstraint {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.dpl.SameShapeConstraint"]
        pub struct SameShapeConstraintObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            args: tvm_ffi::Array<crate::relax::dpl::DFPattern>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SameShapeConstraint {
            data: tvm_ffi::object::ObjectArc<SameShapeConstraintObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SameShapeConstraint: tvm_ffi::object::ObjectRef);

        impl SameShapeConstraint {
            pub fn get_args(&self) -> tvm_ffi::Array<crate::relax::dpl::DFPattern> {
                self.data.args.clone()
            }
        }

        impl SameShapeConstraint {}
    }
    pub mod expr {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Function"]
        pub struct FunctionObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseFuncObj,
            params: tvm_ffi::Array<crate::relax::expr::Var>,
            body: crate::relax::expr::SeqExpr,
            ret_struct_info: crate::ir::StructInfo,
            is_pure: bool,
            _gap0: [u8; 7],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Function {
            data: tvm_ffi::object::ObjectArc<FunctionObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Function: crate::ir::BaseFunc, crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl Function {
            pub fn get_params(&self) -> tvm_ffi::Array<crate::relax::expr::Var> {
                self.data.params.clone()
            }
            pub fn get_body(&self) -> crate::relax::expr::SeqExpr {
                self.data.body.clone()
            }
            pub fn get_ret_struct_info(&self) -> crate::ir::StructInfo {
                self.data.ret_struct_info.clone()
            }
            pub fn get_is_pure(&self) -> bool {
                self.data.is_pure
            }
        }

        impl Function {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.ExternFunc"]
        pub struct ExternFuncObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseFuncObj,
            global_symbol: tvm_ffi::String,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExternFunc {
            data: tvm_ffi::object::ObjectArc<ExternFuncObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExternFunc: crate::ir::BaseFunc, crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl ExternFunc {
            pub fn get_global_symbol(&self) -> tvm_ffi::String {
                self.data.global_symbol.clone()
            }
        }

        impl ExternFunc {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Call"]
        pub struct CallObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
            op: crate::ir::RelaxExpr,
            args: tvm_ffi::Array<crate::ir::RelaxExpr>,
            attrs: crate::ir::Attrs,
            sinfo_args: tvm_ffi::Array<crate::ir::StructInfo>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Call {
            data: tvm_ffi::object::ObjectArc<CallObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Call: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl Call {
            pub fn get_op(&self) -> crate::ir::RelaxExpr {
                self.data.op.clone()
            }
            pub fn get_args(&self) -> tvm_ffi::Array<crate::ir::RelaxExpr> {
                self.data.args.clone()
            }
            pub fn get_attrs(&self) -> crate::ir::Attrs {
                self.data.attrs.clone()
            }
            pub fn get_sinfo_args(&self) -> tvm_ffi::Array<crate::ir::StructInfo> {
                self.data.sinfo_args.clone()
            }
        }

        impl Call {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Tuple"]
        pub struct TupleObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
            fields: tvm_ffi::Array<crate::ir::RelaxExpr>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Tuple {
            data: tvm_ffi::object::ObjectArc<TupleObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Tuple: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl Tuple {
            pub fn get_fields(&self) -> tvm_ffi::Array<crate::ir::RelaxExpr> {
                self.data.fields.clone()
            }
        }

        impl Tuple {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.TupleGetItem"]
        pub struct TupleGetItemObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
            tuple_value: crate::ir::RelaxExpr,
            index: i32,
            _gap0: [u8; 4],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TupleGetItem {
            data: tvm_ffi::object::ObjectArc<TupleGetItemObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TupleGetItem: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl TupleGetItem {
            pub fn get_tuple_value(&self) -> crate::ir::RelaxExpr {
                self.data.tuple_value.clone()
            }
            pub fn get_index(&self) -> i32 {
                self.data.index
            }
        }

        impl TupleGetItem {}

        tvm_ffi::define_object_wrapper!(LeafExpr, "relax.expr.LeafExpr");

        impl LeafExpr {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.ShapeExpr"]
        pub struct ShapeExprObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            values: tvm_ffi::Array<crate::ir::PrimExpr>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ShapeExpr {
            data: tvm_ffi::object::ObjectArc<ShapeExprObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ShapeExpr: tvm_ffi::object::ObjectRef);

        impl ShapeExpr {
            pub fn get_values(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
                self.data.values.clone()
            }
        }

        impl ShapeExpr {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Var"]
        pub struct VarObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            vid: crate::relax::Id,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Var {
            data: tvm_ffi::object::ObjectArc<VarObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Var: tvm_ffi::object::ObjectRef);

        impl Var {
            pub fn get_vid(&self) -> crate::relax::Id {
                self.data.vid.clone()
            }
        }

        impl Var {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.DataflowVar"]
        pub struct DataflowVarObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::relax::expr::VarObj,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DataflowVar {
            data: tvm_ffi::object::ObjectArc<DataflowVarObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DataflowVar: crate::relax::expr::Var, tvm_ffi::object::ObjectRef);

        impl DataflowVar {}

        impl DataflowVar {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Constant"]
        pub struct ConstantObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            data: tvm_ffi::Tensor,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Constant {
            data: tvm_ffi::object::ObjectArc<ConstantObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Constant: tvm_ffi::object::ObjectRef);

        impl Constant {
            pub fn get_data(&self) -> tvm_ffi::Tensor {
                self.data.data.clone()
            }
        }

        impl Constant {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.PrimValue"]
        pub struct PrimValueObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            value: crate::ir::PrimExpr,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PrimValue {
            data: tvm_ffi::object::ObjectArc<PrimValueObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PrimValue: tvm_ffi::object::ObjectRef);

        impl PrimValue {
            pub fn get_value(&self) -> crate::ir::PrimExpr {
                self.data.value.clone()
            }
        }

        impl PrimValue {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.StringImm"]
        pub struct StringImmObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            value: tvm_ffi::String,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StringImm {
            data: tvm_ffi::object::ObjectArc<StringImmObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StringImm: tvm_ffi::object::ObjectRef);

        impl StringImm {
            pub fn get_value(&self) -> tvm_ffi::String {
                self.data.value.clone()
            }
        }

        impl StringImm {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.DataTypeImm"]
        pub struct DataTypeImmObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            value: tvm_ffi::DLDataType,
            _gap1: [u8; 4],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DataTypeImm {
            data: tvm_ffi::object::ObjectArc<DataTypeImmObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DataTypeImm: tvm_ffi::object::ObjectRef);

        impl DataTypeImm {
            pub fn get_value(&self) -> tvm_ffi::DLDataType {
                self.data.value
            }
        }

        impl DataTypeImm {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.SeqExpr"]
        pub struct SeqExprObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
            blocks: tvm_ffi::Array<crate::relax::expr::BindingBlock>,
            body: crate::ir::RelaxExpr,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SeqExpr {
            data: tvm_ffi::object::ObjectArc<SeqExprObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SeqExpr: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl SeqExpr {
            pub fn get_blocks(&self) -> tvm_ffi::Array<crate::relax::expr::BindingBlock> {
                self.data.blocks.clone()
            }
            pub fn get_body(&self) -> crate::ir::RelaxExpr {
                self.data.body.clone()
            }
        }

        impl SeqExpr {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.If"]
        pub struct IfObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
            cond: crate::ir::RelaxExpr,
            true_branch: crate::relax::expr::SeqExpr,
            false_branch: crate::relax::expr::SeqExpr,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct If {
            data: tvm_ffi::object::ObjectArc<IfObj>,
        }

        tvm_ffi::impl_object_hierarchy!(If: crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

        impl If {
            pub fn get_cond(&self) -> crate::ir::RelaxExpr {
                self.data.cond.clone()
            }
            pub fn get_true_branch(&self) -> crate::relax::expr::SeqExpr {
                self.data.true_branch.clone()
            }
            pub fn get_false_branch(&self) -> crate::relax::expr::SeqExpr {
                self.data.false_branch.clone()
            }
        }

        impl If {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Binding"]
        pub struct BindingObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            span: crate::ir::Span,
            var: crate::relax::expr::Var,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Binding {
            data: tvm_ffi::object::ObjectArc<BindingObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Binding: tvm_ffi::object::ObjectRef);

        impl Binding {
            pub fn get_span(&self) -> crate::ir::Span {
                self.data.span.clone()
            }
            pub fn get_var(&self) -> crate::relax::expr::Var {
                self.data.var.clone()
            }
        }

        impl Binding {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.MatchCast"]
        pub struct MatchCastObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::relax::expr::BindingObj,
            value: crate::ir::RelaxExpr,
            struct_info: crate::ir::StructInfo,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct MatchCast {
            data: tvm_ffi::object::ObjectArc<MatchCastObj>,
        }

        tvm_ffi::impl_object_hierarchy!(MatchCast: crate::relax::expr::Binding, tvm_ffi::object::ObjectRef);

        impl MatchCast {
            pub fn get_value(&self) -> crate::ir::RelaxExpr {
                self.data.value.clone()
            }
            pub fn get_struct_info(&self) -> crate::ir::StructInfo {
                self.data.struct_info.clone()
            }
        }

        impl MatchCast {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.VarBinding"]
        pub struct VarBindingObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::relax::expr::BindingObj,
            value: crate::ir::RelaxExpr,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct VarBinding {
            data: tvm_ffi::object::ObjectArc<VarBindingObj>,
        }

        tvm_ffi::impl_object_hierarchy!(VarBinding: crate::relax::expr::Binding, tvm_ffi::object::ObjectRef);

        impl VarBinding {
            pub fn get_value(&self) -> crate::ir::RelaxExpr {
                self.data.value.clone()
            }
        }

        impl VarBinding {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.BindingBlock"]
        pub struct BindingBlockObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            bindings: tvm_ffi::Array<crate::relax::expr::Binding>,
            span: crate::ir::Span,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct BindingBlock {
            data: tvm_ffi::object::ObjectArc<BindingBlockObj>,
        }

        tvm_ffi::impl_object_hierarchy!(BindingBlock: tvm_ffi::object::ObjectRef);

        impl BindingBlock {
            pub fn get_bindings(&self) -> tvm_ffi::Array<crate::relax::expr::Binding> {
                self.data.bindings.clone()
            }
            pub fn get_span(&self) -> crate::ir::Span {
                self.data.span.clone()
            }
        }

        impl BindingBlock {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.DataflowBlock"]
        pub struct DataflowBlockObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::relax::expr::BindingBlockObj,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DataflowBlock {
            data: tvm_ffi::object::ObjectArc<DataflowBlockObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DataflowBlock: crate::relax::expr::BindingBlock, tvm_ffi::object::ObjectRef);

        impl DataflowBlock {}

        impl DataflowBlock {}
    }
    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.transform.InplaceOpportunity"]
        pub struct InplaceOpportunityObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            binding_idx: crate::ir::IntImm,
            arg_idxs: tvm_ffi::Array<crate::ir::IntImm>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct InplaceOpportunity {
            data: tvm_ffi::object::ObjectArc<InplaceOpportunityObj>,
        }

        tvm_ffi::impl_object_hierarchy!(InplaceOpportunity: tvm_ffi::object::ObjectRef);

        impl InplaceOpportunity {
            pub fn get_binding_idx(&self) -> crate::ir::IntImm {
                self.data.binding_idx.clone()
            }
            pub fn get_arg_idxs(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
                self.data.arg_idxs.clone()
            }
        }

        impl InplaceOpportunity {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.transform.FusionPattern"]
        pub struct FusionPatternObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            name: tvm_ffi::String,
            pattern: crate::relax::dpl::DFPattern,
            annotation_patterns: tvm_ffi::Map<tvm_ffi::String, crate::relax::dpl::DFPattern>,
            check: Option<tvm_ffi::Function>,
            attrs_getter: Option<tvm_ffi::Function>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct FusionPattern {
            data: tvm_ffi::object::ObjectArc<FusionPatternObj>,
        }

        tvm_ffi::impl_object_hierarchy!(FusionPattern: tvm_ffi::object::ObjectRef);

        impl FusionPattern {
            pub fn get_name(&self) -> tvm_ffi::String {
                self.data.name.clone()
            }
            pub fn get_pattern(&self) -> crate::relax::dpl::DFPattern {
                self.data.pattern.clone()
            }
            pub fn get_annotation_patterns(
                &self,
            ) -> tvm_ffi::Map<tvm_ffi::String, crate::relax::dpl::DFPattern> {
                self.data.annotation_patterns.clone()
            }
            pub fn get_check(&self) -> Option<tvm_ffi::Function> {
                self.data.check.clone()
            }
            pub fn get_attrs_getter(&self) -> Option<tvm_ffi::Function> {
                self.data.attrs_getter.clone()
            }
        }

        impl FusionPattern {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.transform.PatternCheckContext"]
        pub struct PatternCheckContextObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            matched_expr: crate::ir::RelaxExpr,
            annotated_expr: tvm_ffi::Map<tvm_ffi::String, crate::ir::RelaxExpr>,
            matched_bindings: tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr>,
            var_usages:
                tvm_ffi::Map<crate::relax::expr::Var, tvm_ffi::Array<crate::relax::expr::Var>>,
            value_to_bound_var: tvm_ffi::Map<crate::ir::RelaxExpr, crate::relax::expr::Var>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct PatternCheckContext {
            data: tvm_ffi::object::ObjectArc<PatternCheckContextObj>,
        }

        tvm_ffi::impl_object_hierarchy!(PatternCheckContext: tvm_ffi::object::ObjectRef);

        impl PatternCheckContext {
            pub fn get_matched_expr(&self) -> crate::ir::RelaxExpr {
                self.data.matched_expr.clone()
            }
            pub fn get_annotated_expr(
                &self,
            ) -> tvm_ffi::Map<tvm_ffi::String, crate::ir::RelaxExpr> {
                self.data.annotated_expr.clone()
            }
            pub fn get_matched_bindings(
                &self,
            ) -> tvm_ffi::Map<crate::relax::expr::Var, crate::ir::RelaxExpr> {
                self.data.matched_bindings.clone()
            }
            pub fn get_var_usages(
                &self,
            ) -> tvm_ffi::Map<crate::relax::expr::Var, tvm_ffi::Array<crate::relax::expr::Var>>
            {
                self.data.var_usages.clone()
            }
            pub fn get_value_to_bound_var(
                &self,
            ) -> tvm_ffi::Map<crate::ir::RelaxExpr, crate::relax::expr::Var> {
                self.data.value_to_bound_var.clone()
            }
        }

        impl PatternCheckContext {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.transform.LayoutDecision"]
        pub struct LayoutDecisionObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            layout: crate::tir::Layout,
            is_unknown_dim: bool,
            _gap0: [u8; 7],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LayoutDecision {
            data: tvm_ffi::object::ObjectArc<LayoutDecisionObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LayoutDecision: tvm_ffi::object::ObjectRef);

        impl LayoutDecision {
            pub fn get_layout(&self) -> crate::tir::Layout {
                self.data.layout.clone()
            }
            pub fn get_is_unknown_dim(&self) -> bool {
                self.data.is_unknown_dim
            }
        }

        impl LayoutDecision {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.transform.InferLayoutOutput"]
        pub struct InferLayoutOutputObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 16],
            new_attrs: crate::ir::Attrs,
            new_args: tvm_ffi::Map<crate::ir::IntImm, crate::ir::RelaxExpr>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct InferLayoutOutput {
            data: tvm_ffi::object::ObjectArc<InferLayoutOutputObj>,
        }

        tvm_ffi::impl_object_hierarchy!(InferLayoutOutput: tvm_ffi::object::ObjectRef);

        impl InferLayoutOutput {
            pub fn get_new_attrs(&self) -> crate::ir::Attrs {
                self.data.new_attrs.clone()
            }
            pub fn get_new_args(&self) -> tvm_ffi::Map<crate::ir::IntImm, crate::ir::RelaxExpr> {
                self.data.new_args.clone()
            }
        }

        static FIELD_RELAX_TRANSFORM_INFERLAYOUTOUTPUT__INPUT_LAYOUTS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.transform.InferLayoutOutput",
                "input_layouts",
            )
            .expect("non-layout field input_layouts must be registered in TVM reflection")
        });
        static FIELD_RELAX_TRANSFORM_INFERLAYOUTOUTPUT__OUTPUT_LAYOUTS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "relax.transform.InferLayoutOutput",
                "output_layouts",
            )
            .expect("non-layout field output_layouts must be registered in TVM reflection")
        });
        impl InferLayoutOutput {
            pub fn get_input_layouts(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_TRANSFORM_INFERLAYOUTOUTPUT__INPUT_LAYOUTS
                    .get_any(&__obj)
                    .expect("non-layout field input_layouts should be accessible")
            }
            pub fn get_output_layouts(&self) -> tvm_ffi::Any {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_RELAX_TRANSFORM_INFERLAYOUTOUTPUT__OUTPUT_LAYOUTS
                    .get_any(&__obj)
                    .expect("non-layout field output_layouts should be accessible")
            }
        }
    }
}
pub mod script {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "script.PrinterConfig"]
    pub struct PrinterConfigObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        binding_names: tvm_ffi::Array<tvm_ffi::String>,
        show_meta: bool,
        _gap0: [u8; 7],
        ir_prefix: tvm_ffi::String,
        tir_prefix: tvm_ffi::String,
        relax_prefix: tvm_ffi::String,
        module_alias: tvm_ffi::String,
        buffer_dtype: tvm_ffi::DLDataType,
        int_dtype: tvm_ffi::DLDataType,
        float_dtype: tvm_ffi::DLDataType,
        verbose_expr: bool,
        _gap1: [u8; 3],
        indent_spaces: i32,
        print_line_numbers: bool,
        _gap2: [u8; 3],
        num_context_lines: i32,
        syntax_sugar: bool,
        show_object_address: bool,
        show_all_struct_info: bool,
        _gap3: [u8; 1],
        path_to_underline: tvm_ffi::Array<crate::ffi::reflection::AccessPath>,
        path_to_annotate: tvm_ffi::Map<crate::ffi::reflection::AccessPath, tvm_ffi::String>,
        obj_to_underline: tvm_ffi::Array<tvm_ffi::object::ObjectRef>,
        obj_to_annotate: tvm_ffi::Map<tvm_ffi::object::ObjectRef, tvm_ffi::String>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PrinterConfig {
        data: tvm_ffi::object::ObjectArc<PrinterConfigObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PrinterConfig: tvm_ffi::object::ObjectRef);

    impl PrinterConfig {
        pub fn get_binding_names(&self) -> tvm_ffi::Array<tvm_ffi::String> {
            self.data.binding_names.clone()
        }
        pub fn get_show_meta(&self) -> bool {
            self.data.show_meta
        }
        pub fn get_ir_prefix(&self) -> tvm_ffi::String {
            self.data.ir_prefix.clone()
        }
        pub fn get_tir_prefix(&self) -> tvm_ffi::String {
            self.data.tir_prefix.clone()
        }
        pub fn get_relax_prefix(&self) -> tvm_ffi::String {
            self.data.relax_prefix.clone()
        }
        pub fn get_module_alias(&self) -> tvm_ffi::String {
            self.data.module_alias.clone()
        }
        pub fn get_buffer_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.buffer_dtype
        }
        pub fn get_int_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.int_dtype
        }
        pub fn get_float_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.float_dtype
        }
        pub fn get_verbose_expr(&self) -> bool {
            self.data.verbose_expr
        }
        pub fn get_indent_spaces(&self) -> i32 {
            self.data.indent_spaces
        }
        pub fn get_print_line_numbers(&self) -> bool {
            self.data.print_line_numbers
        }
        pub fn get_num_context_lines(&self) -> i32 {
            self.data.num_context_lines
        }
        pub fn get_syntax_sugar(&self) -> bool {
            self.data.syntax_sugar
        }
        pub fn get_show_object_address(&self) -> bool {
            self.data.show_object_address
        }
        pub fn get_show_all_struct_info(&self) -> bool {
            self.data.show_all_struct_info
        }
        pub fn get_path_to_underline(&self) -> tvm_ffi::Array<crate::ffi::reflection::AccessPath> {
            self.data.path_to_underline.clone()
        }
        pub fn get_path_to_annotate(
            &self,
        ) -> tvm_ffi::Map<crate::ffi::reflection::AccessPath, tvm_ffi::String> {
            self.data.path_to_annotate.clone()
        }
        pub fn get_obj_to_underline(&self) -> tvm_ffi::Array<tvm_ffi::object::ObjectRef> {
            self.data.obj_to_underline.clone()
        }
        pub fn get_obj_to_annotate(
            &self,
        ) -> tvm_ffi::Map<tvm_ffi::object::ObjectRef, tvm_ffi::String> {
            self.data.obj_to_annotate.clone()
        }
    }

    impl PrinterConfig {}

    pub mod ir_builder {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.ir_builder.IRBuilderFrame"]
        pub struct IRBuilderFrameObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 32],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IRBuilderFrame {
            data: tvm_ffi::object::ObjectArc<IRBuilderFrameObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IRBuilderFrame: tvm_ffi::object::ObjectRef);

        impl IRBuilderFrame {}

        impl IRBuilderFrame {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.ir_builder.IRBuilder"]
        pub struct IRBuilderObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            frames: tvm_ffi::Array<crate::script::ir_builder::IRBuilderFrame>,
            result: Option<tvm_ffi::object::ObjectRef>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IRBuilder {
            data: tvm_ffi::object::ObjectArc<IRBuilderObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IRBuilder: tvm_ffi::object::ObjectRef);

        impl IRBuilder {
            pub fn get_frames(&self) -> tvm_ffi::Array<crate::script::ir_builder::IRBuilderFrame> {
                self.data.frames.clone()
            }
            pub fn get_result(&self) -> Option<tvm_ffi::object::ObjectRef> {
                self.data.result.clone()
            }
        }

        impl IRBuilder {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.ir_builder.IRModuleFrame"]
        pub struct IRModuleFrameObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::IRBuilderFrameObj,
            functions: tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
            attrs: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
            global_infos: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IRModuleFrame {
            data: tvm_ffi::object::ObjectArc<IRModuleFrameObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IRModuleFrame: crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

        impl IRModuleFrame {
            pub fn get_functions(&self) -> tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc> {
                self.data.functions.clone()
            }
            pub fn get_attrs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
                self.data.attrs.clone()
            }
            pub fn get_global_infos(
                &self,
            ) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>> {
                self.data.global_infos.clone()
            }
        }

        static FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__GLOBAL_VARS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<
                tvm_ffi::Map<tvm_ffi::String, crate::ir::GlobalVar>,
            >,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.ir_builder.IRModuleFrame",
                "global_vars",
            )
            .expect("non-layout field global_vars must be registered in TVM reflection")
        });
        impl IRModuleFrame {
            pub fn get_global_vars(&self) -> tvm_ffi::Map<tvm_ffi::String, crate::ir::GlobalVar> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__GLOBAL_VARS
                    .get(&__obj)
                    .expect("non-layout field global_vars should be accessible")
            }
        }

        pub mod relax {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, ObjectArc, Result};

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.RelaxFrame"]
            pub struct RelaxFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::IRBuilderFrameObj,
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct RelaxFrame {
                data: tvm_ffi::object::ObjectArc<RelaxFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(RelaxFrame: crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl RelaxFrame {}

            impl RelaxFrame {}

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.SeqExprFrame"]
            pub struct SeqExprFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::relax::RelaxFrameObj,
                output: Option<crate::ir::RelaxExpr>,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct SeqExprFrame {
                data: tvm_ffi::object::ObjectArc<SeqExprFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(SeqExprFrame: crate::script::ir_builder::relax::RelaxFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl SeqExprFrame {
                pub fn get_output(&self) -> Option<crate::ir::RelaxExpr> {
                    self.data.output.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_RELAX_SEQEXPRFRAME__BINDING_BLOCKS: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Array<crate::relax::expr::BindingBlock>,
                >,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.SeqExprFrame",
                    "binding_blocks",
                )
                .expect("non-layout field binding_blocks must be registered in TVM reflection")
            });
            impl SeqExprFrame {
                pub fn get_binding_blocks(
                    &self,
                ) -> tvm_ffi::Array<crate::relax::expr::BindingBlock> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_RELAX_SEQEXPRFRAME__BINDING_BLOCKS
                        .get(&__obj)
                        .expect("non-layout field binding_blocks should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.FunctionFrame"]
            pub struct FunctionFrameObj {
                __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::relax::SeqExprFrameObj,
                _gap0: [u8; 8],
                params: tvm_ffi::Array<crate::relax::expr::Var>,
                ret_struct_info: Option<crate::ir::StructInfo>,
                is_pure: Option<crate::ir::IntImm>,
                _gap1: [u8; 8],
                attrs: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                _gap2: [u8; 16],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct FunctionFrame {
                data: tvm_ffi::object::ObjectArc<FunctionFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(FunctionFrame: crate::script::ir_builder::relax::SeqExprFrame, crate::script::ir_builder::relax::RelaxFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl FunctionFrame {
                pub fn get_params(&self) -> tvm_ffi::Array<crate::relax::expr::Var> {
                    self.data.params.clone()
                }
                pub fn get_ret_struct_info(&self) -> Option<crate::ir::StructInfo> {
                    self.data.ret_struct_info.clone()
                }
                pub fn get_is_pure(&self) -> Option<crate::ir::IntImm> {
                    self.data.is_pure.clone()
                }
                pub fn get_attrs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
                    self.data.attrs.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__NAME: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "name",
                )
                .expect("non-layout field name must be registered in TVM reflection")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__BINDING_BLOCKS:
                std::sync::LazyLock<
                    tvm_ffi::object_wrapper::FieldGetter<
                        tvm_ffi::Array<crate::relax::expr::BindingBlock>,
                    >,
                > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "binding_blocks",
                )
                .expect("non-layout field binding_blocks must be registered in TVM reflection")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__OUTPUT: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::RelaxExpr>>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "output",
                )
                .expect("non-layout field output must be registered in TVM reflection")
            });
            impl FunctionFrame {
                pub fn get_name(&self) -> Option<tvm_ffi::String> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__NAME
                        .get(&__obj)
                        .expect("non-layout field name should be accessible")
                }
                pub fn get_binding_blocks(
                    &self,
                ) -> tvm_ffi::Array<crate::relax::expr::BindingBlock> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__BINDING_BLOCKS
                        .get(&__obj)
                        .expect("non-layout field binding_blocks should be accessible")
                }
                pub fn get_output(&self) -> Option<crate::ir::RelaxExpr> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__OUTPUT
                        .get(&__obj)
                        .expect("non-layout field output should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.BlockFrame"]
            pub struct BlockFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::relax::RelaxFrameObj,
                emitted_vars: tvm_ffi::Array<crate::relax::expr::Var>,
                _gap0: [u8; 8],
                output_vars: tvm_ffi::Array<crate::relax::expr::Var>,
                _gap1: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct BlockFrame {
                data: tvm_ffi::object::ObjectArc<BlockFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(BlockFrame: crate::script::ir_builder::relax::RelaxFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl BlockFrame {
                pub fn get_emitted_vars(&self) -> tvm_ffi::Array<crate::relax::expr::Var> {
                    self.data.emitted_vars.clone()
                }
                pub fn get_output_vars(&self) -> tvm_ffi::Array<crate::relax::expr::Var> {
                    self.data.output_vars.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__IS_DATAFLOW: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<bool>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.BlockFrame",
                    "is_dataflow",
                )
                .expect("non-layout field is_dataflow must be registered in TVM reflection")
            });
            impl BlockFrame {
                pub fn get_is_dataflow(&self) -> bool {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__IS_DATAFLOW
                        .get(&__obj)
                        .expect("non-layout field is_dataflow should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.IfFrame"]
            pub struct IfFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::relax::RelaxFrameObj,
                then_expr: Option<crate::ir::RelaxExpr>,
                else_expr: Option<crate::ir::RelaxExpr>,
                var: crate::relax::expr::Var,
                var_name: tvm_ffi::String,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct IfFrame {
                data: tvm_ffi::object::ObjectArc<IfFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(IfFrame: crate::script::ir_builder::relax::RelaxFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl IfFrame {
                pub fn get_then_expr(&self) -> Option<crate::ir::RelaxExpr> {
                    self.data.then_expr.clone()
                }
                pub fn get_else_expr(&self) -> Option<crate::ir::RelaxExpr> {
                    self.data.else_expr.clone()
                }
                pub fn get_var(&self) -> crate::relax::expr::Var {
                    self.data.var.clone()
                }
                pub fn get_var_name(&self) -> tvm_ffi::String {
                    self.data.var_name.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__CONDITION: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::RelaxExpr>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.IfFrame",
                    "condition",
                )
                .expect("non-layout field condition must be registered in TVM reflection")
            });
            impl IfFrame {
                pub fn get_condition(&self) -> crate::ir::RelaxExpr {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__CONDITION
                        .get(&__obj)
                        .expect("non-layout field condition should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.ThenFrame"]
            pub struct ThenFrameObj {
                __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::relax::SeqExprFrameObj,
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct ThenFrame {
                data: tvm_ffi::object::ObjectArc<ThenFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(ThenFrame: crate::script::ir_builder::relax::SeqExprFrame, crate::script::ir_builder::relax::RelaxFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl ThenFrame {}

            impl ThenFrame {}

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.relax.ElseFrame"]
            pub struct ElseFrameObj {
                __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::relax::SeqExprFrameObj,
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct ElseFrame {
                data: tvm_ffi::object::ObjectArc<ElseFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(ElseFrame: crate::script::ir_builder::relax::SeqExprFrame, crate::script::ir_builder::relax::RelaxFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl ElseFrame {}

            impl ElseFrame {}
        }
        pub mod tir {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, ObjectArc, Result};

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.TIRFrame"]
            pub struct TIRFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::IRBuilderFrameObj,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct TIRFrame {
                data: tvm_ffi::object::ObjectArc<TIRFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(TIRFrame: crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl TIRFrame {}

            static FIELD_SCRIPT_IR_BUILDER_TIR_TIRFRAME__STMTS: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Stmt>>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.TIRFrame", "stmts")
                    .expect("non-layout field stmts must be registered in TVM reflection")
            });
            impl TIRFrame {
                pub fn get_stmts(&self) -> tvm_ffi::Array<crate::tir::Stmt> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_TIRFRAME__STMTS
                        .get(&__obj)
                        .expect("non-layout field stmts should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.PrimFuncFrame"]
            pub struct PrimFuncFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                _gap0: [u8; 8],
                args: tvm_ffi::Array<crate::tir::Var>,
                is_private: bool,
                _gap1: [u8; 7],
                ret_type: Option<crate::ir::Type>,
                buffer_map: tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
                attrs: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                env_threads: tvm_ffi::Map<crate::tir::Var, crate::tir::IterVar>,
                root_alloc_buffers: tvm_ffi::Array<crate::tir::Buffer>,
                _gap2: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct PrimFuncFrame {
                data: tvm_ffi::object::ObjectArc<PrimFuncFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(PrimFuncFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl PrimFuncFrame {
                pub fn get_args(&self) -> tvm_ffi::Array<crate::tir::Var> {
                    self.data.args.clone()
                }
                pub fn get_is_private(&self) -> bool {
                    self.data.is_private
                }
                pub fn get_ret_type(&self) -> Option<crate::ir::Type> {
                    self.data.ret_type.clone()
                }
                pub fn get_buffer_map(&self) -> tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer> {
                    self.data.buffer_map.clone()
                }
                pub fn get_attrs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
                    self.data.attrs.clone()
                }
                pub fn get_env_threads(
                    &self,
                ) -> tvm_ffi::Map<crate::tir::Var, crate::tir::IterVar> {
                    self.data.env_threads.clone()
                }
                pub fn get_root_alloc_buffers(&self) -> tvm_ffi::Array<crate::tir::Buffer> {
                    self.data.root_alloc_buffers.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__NAME: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "name",
                )
                .expect("non-layout field name must be registered in TVM reflection")
            });
            impl PrimFuncFrame {
                pub fn get_name(&self) -> Option<tvm_ffi::String> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__NAME
                        .get(&__obj)
                        .expect("non-layout field name should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.BlockFrame"]
            pub struct BlockFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                _gap0: [u8; 8],
                iter_vars: tvm_ffi::Array<crate::tir::IterVar>,
                reads: Option<tvm_ffi::Array<crate::tir::BufferRegion>>,
                writes: Option<tvm_ffi::Array<crate::tir::BufferRegion>>,
                init: Option<crate::tir::Stmt>,
                alloc_buffers: tvm_ffi::Array<crate::tir::Buffer>,
                match_buffers: tvm_ffi::Array<crate::tir::MatchBufferRegion>,
                annotations: Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                iter_values: tvm_ffi::Array<crate::ir::PrimExpr>,
                predicate: Option<crate::ir::PrimExpr>,
                no_realize: bool,
                _gap1: [u8; 15],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct BlockFrame {
                data: tvm_ffi::object::ObjectArc<BlockFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(BlockFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl BlockFrame {
                pub fn get_iter_vars(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
                    self.data.iter_vars.clone()
                }
                pub fn get_reads(&self) -> Option<tvm_ffi::Array<crate::tir::BufferRegion>> {
                    self.data.reads.clone()
                }
                pub fn get_writes(&self) -> Option<tvm_ffi::Array<crate::tir::BufferRegion>> {
                    self.data.writes.clone()
                }
                pub fn get_init(&self) -> Option<crate::tir::Stmt> {
                    self.data.init.clone()
                }
                pub fn get_alloc_buffers(&self) -> tvm_ffi::Array<crate::tir::Buffer> {
                    self.data.alloc_buffers.clone()
                }
                pub fn get_match_buffers(&self) -> tvm_ffi::Array<crate::tir::MatchBufferRegion> {
                    self.data.match_buffers.clone()
                }
                pub fn get_annotations(
                    &self,
                ) -> Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
                    self.data.annotations.clone()
                }
                pub fn get_iter_values(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
                    self.data.iter_values.clone()
                }
                pub fn get_predicate(&self) -> Option<crate::ir::PrimExpr> {
                    self.data.predicate.clone()
                }
                pub fn get_no_realize(&self) -> bool {
                    self.data.no_realize
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__NAME: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "name",
                )
                .expect("non-layout field name must be registered in TVM reflection")
            });
            impl BlockFrame {
                pub fn get_name(&self) -> tvm_ffi::String {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__NAME
                        .get(&__obj)
                        .expect("non-layout field name should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.BlockInitFrame"]
            pub struct BlockInitFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct BlockInitFrame {
                data: tvm_ffi::object::ObjectArc<BlockInitFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(BlockInitFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl BlockInitFrame {}

            impl BlockInitFrame {}

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.ForFrame"]
            pub struct ForFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                doms: tvm_ffi::Array<crate::ir::Range>,
                _gap0: [u8; 24],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct ForFrame {
                data: tvm_ffi::object::ObjectArc<ForFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(ForFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl ForFrame {
                pub fn get_doms(&self) -> tvm_ffi::Array<crate::ir::Range> {
                    self.data.doms.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_FORFRAME__VARS: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Var>>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.ForFrame", "vars")
                    .expect("non-layout field vars must be registered in TVM reflection")
            });
            impl ForFrame {
                pub fn get_vars(&self) -> tvm_ffi::Array<crate::tir::Var> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_FORFRAME__VARS
                        .get(&__obj)
                        .expect("non-layout field vars should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.AssertFrame"]
            pub struct AssertFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                message: crate::ir::PrimExpr,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct AssertFrame {
                data: tvm_ffi::object::ObjectArc<AssertFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(AssertFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl AssertFrame {
                pub fn get_message(&self) -> crate::ir::PrimExpr {
                    self.data.message.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_ASSERTFRAME__CONDITION: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AssertFrame",
                    "condition",
                )
                .expect("non-layout field condition must be registered in TVM reflection")
            });
            impl AssertFrame {
                pub fn get_condition(&self) -> crate::ir::PrimExpr {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_ASSERTFRAME__CONDITION
                        .get(&__obj)
                        .expect("non-layout field condition should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.LetFrame"]
            pub struct LetFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                value: crate::ir::PrimExpr,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct LetFrame {
                data: tvm_ffi::object::ObjectArc<LetFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(LetFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl LetFrame {
                pub fn get_value(&self) -> crate::ir::PrimExpr {
                    self.data.value.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_LETFRAME__VAR: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::Var>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.LetFrame", "var")
                    .expect("non-layout field var must be registered in TVM reflection")
            });
            impl LetFrame {
                pub fn get_var(&self) -> crate::tir::Var {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_LETFRAME__VAR
                        .get(&__obj)
                        .expect("non-layout field var should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.LaunchThreadFrame"]
            pub struct LaunchThreadFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                attr_key: tvm_ffi::String,
                iter_var: crate::tir::IterVar,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct LaunchThreadFrame {
                data: tvm_ffi::object::ObjectArc<LaunchThreadFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(LaunchThreadFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl LaunchThreadFrame {
                pub fn get_attr_key(&self) -> tvm_ffi::String {
                    self.data.attr_key.clone()
                }
                pub fn get_iter_var(&self) -> crate::tir::IterVar {
                    self.data.iter_var.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__EXTENT: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.LaunchThreadFrame",
                    "extent",
                )
                .expect("non-layout field extent must be registered in TVM reflection")
            });
            impl LaunchThreadFrame {
                pub fn get_extent(&self) -> crate::ir::PrimExpr {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__EXTENT
                        .get(&__obj)
                        .expect("non-layout field extent should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.RealizeFrame"]
            pub struct RealizeFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                storage_scope: tvm_ffi::String,
                condition: crate::ir::PrimExpr,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct RealizeFrame {
                data: tvm_ffi::object::ObjectArc<RealizeFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(RealizeFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl RealizeFrame {
                pub fn get_storage_scope(&self) -> tvm_ffi::String {
                    self.data.storage_scope.clone()
                }
                pub fn get_condition(&self) -> crate::ir::PrimExpr {
                    self.data.condition.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__BUFFER_SLICE: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.RealizeFrame",
                    "buffer_slice",
                )
                .expect("non-layout field buffer_slice must be registered in TVM reflection")
            });
            impl RealizeFrame {
                pub fn get_buffer_slice(&self) -> crate::tir::BufferRegion {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__BUFFER_SLICE
                        .get(&__obj)
                        .expect("non-layout field buffer_slice should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.AllocateFrame"]
            pub struct AllocateFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                dtype: tvm_ffi::DLDataType,
                _gap0: [u8; 4],
                storage_scope: tvm_ffi::String,
                condition: crate::ir::PrimExpr,
                annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                buffer_var: crate::tir::Var,
                _gap1: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct AllocateFrame {
                data: tvm_ffi::object::ObjectArc<AllocateFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(AllocateFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl AllocateFrame {
                pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                    self.data.dtype
                }
                pub fn get_storage_scope(&self) -> tvm_ffi::String {
                    self.data.storage_scope.clone()
                }
                pub fn get_condition(&self) -> crate::ir::PrimExpr {
                    self.data.condition.clone()
                }
                pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
                    self.data.annotations.clone()
                }
                pub fn get_buffer_var(&self) -> crate::tir::Var {
                    self.data.buffer_var.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__EXTENTS: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "extents",
                )
                .expect("non-layout field extents must be registered in TVM reflection")
            });
            impl AllocateFrame {
                pub fn get_extents(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__EXTENTS
                        .get(&__obj)
                        .expect("non-layout field extents should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.AllocateConstFrame"]
            pub struct AllocateConstFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                extents: tvm_ffi::Array<crate::ir::PrimExpr>,
                data: tvm_ffi::Tensor,
                buffer_var: crate::tir::Var,
                annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct AllocateConstFrame {
                data: tvm_ffi::object::ObjectArc<AllocateConstFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(AllocateConstFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl AllocateConstFrame {
                pub fn get_extents(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
                    self.data.extents.clone()
                }
                pub fn get_data(&self) -> tvm_ffi::Tensor {
                    self.data.data.clone()
                }
                pub fn get_buffer_var(&self) -> crate::tir::Var {
                    self.data.buffer_var.clone()
                }
                pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
                    self.data.annotations.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__DTYPE: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateConstFrame",
                    "dtype",
                )
                .expect("non-layout field dtype must be registered in TVM reflection")
            });
            impl AllocateConstFrame {
                pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__DTYPE
                        .get(&__obj)
                        .expect("non-layout field dtype should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.AttrFrame"]
            pub struct AttrFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                _gap0: [u8; 8],
                attr_key: tvm_ffi::String,
                value: crate::ir::PrimExpr,
                _gap1: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct AttrFrame {
                data: tvm_ffi::object::ObjectArc<AttrFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(AttrFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl AttrFrame {
                pub fn get_attr_key(&self) -> tvm_ffi::String {
                    self.data.attr_key.clone()
                }
                pub fn get_value(&self) -> crate::ir::PrimExpr {
                    self.data.value.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__NODE: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::AnyValue>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.AttrFrame", "node")
                    .expect("non-layout field node must be registered in TVM reflection")
            });
            impl AttrFrame {
                pub fn get_node(&self) -> tvm_ffi::AnyValue {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__NODE
                        .get(&__obj)
                        .expect("non-layout field node should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.WhileFrame"]
            pub struct WhileFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct WhileFrame {
                data: tvm_ffi::object::ObjectArc<WhileFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(WhileFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl WhileFrame {}

            static FIELD_SCRIPT_IR_BUILDER_TIR_WHILEFRAME__CONDITION: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.WhileFrame",
                    "condition",
                )
                .expect("non-layout field condition must be registered in TVM reflection")
            });
            impl WhileFrame {
                pub fn get_condition(&self) -> crate::ir::PrimExpr {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_WHILEFRAME__CONDITION
                        .get(&__obj)
                        .expect("non-layout field condition should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.IfFrame"]
            pub struct IfFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                then_stmts: Option<tvm_ffi::Array<crate::tir::Stmt>>,
                else_stmts: Option<tvm_ffi::Array<crate::tir::Stmt>>,
                _gap0: [u8; 8],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct IfFrame {
                data: tvm_ffi::object::ObjectArc<IfFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(IfFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl IfFrame {
                pub fn get_then_stmts(&self) -> Option<tvm_ffi::Array<crate::tir::Stmt>> {
                    self.data.then_stmts.clone()
                }
                pub fn get_else_stmts(&self) -> Option<tvm_ffi::Array<crate::tir::Stmt>> {
                    self.data.else_stmts.clone()
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__CONDITION: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.IfFrame",
                    "condition",
                )
                .expect("non-layout field condition must be registered in TVM reflection")
            });
            impl IfFrame {
                pub fn get_condition(&self) -> crate::ir::PrimExpr {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__CONDITION
                        .get(&__obj)
                        .expect("non-layout field condition should be accessible")
                }
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.ThenFrame"]
            pub struct ThenFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct ThenFrame {
                data: tvm_ffi::object::ObjectArc<ThenFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(ThenFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl ThenFrame {}

            impl ThenFrame {}

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.ElseFrame"]
            pub struct ElseFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct ElseFrame {
                data: tvm_ffi::object::ObjectArc<ElseFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(ElseFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl ElseFrame {}

            impl ElseFrame {}

            #[repr(C)]
            #[derive(tvm_ffi::derive::Object)]
            #[type_key = "script.ir_builder.tir.DeclBufferFrame"]
            pub struct DeclBufferFrameObj {
                __tvm_ffi_object_parent:
                    crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
                allocated: bool,
                _gap0: [u8; 15],
            }

            #[repr(C)]
            #[derive(tvm_ffi::derive::ObjectRef, Clone)]
            pub struct DeclBufferFrame {
                data: tvm_ffi::object::ObjectArc<DeclBufferFrameObj>,
            }

            tvm_ffi::impl_object_hierarchy!(DeclBufferFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

            impl DeclBufferFrame {
                pub fn get_allocated(&self) -> bool {
                    self.data.allocated
                }
            }

            static FIELD_SCRIPT_IR_BUILDER_TIR_DECLBUFFERFRAME__BUFFER: std::sync::LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
            > = std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.DeclBufferFrame",
                    "buffer",
                )
                .expect("non-layout field buffer must be registered in TVM reflection")
            });
            impl DeclBufferFrame {
                pub fn get_buffer(&self) -> crate::tir::Buffer {
                    let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                    FIELD_SCRIPT_IR_BUILDER_TIR_DECLBUFFERFRAME__BUFFER
                        .get(&__obj)
                        .expect("non-layout field buffer should be accessible")
                }
            }
        }
    }
    pub mod printer {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.Doc"]
        pub struct DocObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            source_paths: tvm_ffi::Array<crate::ffi::reflection::AccessPath>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Doc {
            data: tvm_ffi::object::ObjectArc<DocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Doc: tvm_ffi::object::ObjectRef);

        impl Doc {
            pub fn get_source_paths(&self) -> tvm_ffi::Array<crate::ffi::reflection::AccessPath> {
                self.data.source_paths.clone()
            }
        }

        impl Doc {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ExprDoc"]
        pub struct ExprDocObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::printer::DocObj,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExprDoc {
            data: tvm_ffi::object::ObjectArc<ExprDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExprDoc: crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ExprDoc {}

        impl ExprDoc {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.StmtDoc"]
        pub struct StmtDocObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::printer::DocObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StmtDoc {
            data: tvm_ffi::object::ObjectArc<StmtDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StmtDoc: crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl StmtDoc {}

        static FIELD_SCRIPT_PRINTER_STMTDOC__COMMENT: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.StmtDoc", "comment")
                .expect("non-layout field comment must be registered in TVM reflection")
        });
        impl StmtDoc {
            pub fn get_comment(&self) -> Option<tvm_ffi::String> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_STMTDOC__COMMENT
                    .get(&__obj)
                    .expect("non-layout field comment should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.StmtBlockDoc"]
        pub struct StmtBlockDocObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::printer::DocObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct StmtBlockDoc {
            data: tvm_ffi::object::ObjectArc<StmtBlockDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(StmtBlockDoc: crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl StmtBlockDoc {}

        static FIELD_SCRIPT_PRINTER_STMTBLOCKDOC__STMTS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.StmtBlockDoc", "stmts")
                .expect("non-layout field stmts must be registered in TVM reflection")
        });
        impl StmtBlockDoc {
            pub fn get_stmts(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_STMTBLOCKDOC__STMTS
                    .get(&__obj)
                    .expect("non-layout field stmts should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.LiteralDoc"]
        pub struct LiteralDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LiteralDoc {
            data: tvm_ffi::object::ObjectArc<LiteralDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LiteralDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl LiteralDoc {}

        static FIELD_SCRIPT_PRINTER_LITERALDOC__VALUE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::AnyValue>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.LiteralDoc", "value")
                .expect("non-layout field value must be registered in TVM reflection")
        });
        impl LiteralDoc {
            pub fn get_value(&self) -> tvm_ffi::AnyValue {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_LITERALDOC__VALUE
                    .get(&__obj)
                    .expect("non-layout field value should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.IdDoc"]
        pub struct IdDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IdDoc {
            data: tvm_ffi::object::ObjectArc<IdDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IdDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl IdDoc {}

        static FIELD_SCRIPT_PRINTER_IDDOC__NAME: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IdDoc", "name")
                .expect("non-layout field name must be registered in TVM reflection")
        });
        impl IdDoc {
            pub fn get_name(&self) -> tvm_ffi::String {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_IDDOC__NAME
                    .get(&__obj)
                    .expect("non-layout field name should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.AttrAccessDoc"]
        pub struct AttrAccessDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            name: tvm_ffi::String,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AttrAccessDoc {
            data: tvm_ffi::object::ObjectArc<AttrAccessDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AttrAccessDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl AttrAccessDoc {
            pub fn get_name(&self) -> tvm_ffi::String {
                self.data.name.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_ATTRACCESSDOC__VALUE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AttrAccessDoc", "value")
                .expect("non-layout field value must be registered in TVM reflection")
        });
        impl AttrAccessDoc {
            pub fn get_value(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_ATTRACCESSDOC__VALUE
                    .get(&__obj)
                    .expect("non-layout field value should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.IndexDoc"]
        pub struct IndexDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            indices: tvm_ffi::Array<crate::script::printer::Doc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IndexDoc {
            data: tvm_ffi::object::ObjectArc<IndexDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IndexDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl IndexDoc {
            pub fn get_indices(&self) -> tvm_ffi::Array<crate::script::printer::Doc> {
                self.data.indices.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_INDEXDOC__VALUE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IndexDoc", "value")
                .expect("non-layout field value must be registered in TVM reflection")
        });
        impl IndexDoc {
            pub fn get_value(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_INDEXDOC__VALUE
                    .get(&__obj)
                    .expect("non-layout field value should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.CallDoc"]
        pub struct CallDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            args: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            kwargs_keys: tvm_ffi::Array<tvm_ffi::String>,
            kwargs_values: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct CallDoc {
            data: tvm_ffi::object::ObjectArc<CallDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(CallDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl CallDoc {
            pub fn get_args(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                self.data.args.clone()
            }
            pub fn get_kwargs_keys(&self) -> tvm_ffi::Array<tvm_ffi::String> {
                self.data.kwargs_keys.clone()
            }
            pub fn get_kwargs_values(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                self.data.kwargs_values.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_CALLDOC__CALLEE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.CallDoc", "callee")
                .expect("non-layout field callee must be registered in TVM reflection")
        });
        impl CallDoc {
            pub fn get_callee(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_CALLDOC__CALLEE
                    .get(&__obj)
                    .expect("non-layout field callee should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.OperationDoc"]
        pub struct OperationDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            operands: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct OperationDoc {
            data: tvm_ffi::object::ObjectArc<OperationDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(OperationDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl OperationDoc {
            pub fn get_operands(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                self.data.operands.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_OPERATIONDOC__KIND: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.OperationDoc", "kind")
                .expect("non-layout field kind must be registered in TVM reflection")
        });
        impl OperationDoc {
            pub fn get_kind(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_OPERATIONDOC__KIND
                    .get(&__obj)
                    .expect("non-layout field kind should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.LambdaDoc"]
        pub struct LambdaDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            body: crate::script::printer::ExprDoc,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LambdaDoc {
            data: tvm_ffi::object::ObjectArc<LambdaDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LambdaDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl LambdaDoc {
            pub fn get_body(&self) -> crate::script::printer::ExprDoc {
                self.data.body.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_LAMBDADOC__ARGS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::IdDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.LambdaDoc", "args")
                .expect("non-layout field args must be registered in TVM reflection")
        });
        impl LambdaDoc {
            pub fn get_args(&self) -> tvm_ffi::Array<crate::script::printer::IdDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_LAMBDADOC__ARGS
                    .get(&__obj)
                    .expect("non-layout field args should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.TupleDoc"]
        pub struct TupleDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TupleDoc {
            data: tvm_ffi::object::ObjectArc<TupleDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TupleDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl TupleDoc {}

        static FIELD_SCRIPT_PRINTER_TUPLEDOC__ELEMENTS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.TupleDoc", "elements")
                .expect("non-layout field elements must be registered in TVM reflection")
        });
        impl TupleDoc {
            pub fn get_elements(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_TUPLEDOC__ELEMENTS
                    .get(&__obj)
                    .expect("non-layout field elements should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ListDoc"]
        pub struct ListDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ListDoc {
            data: tvm_ffi::object::ObjectArc<ListDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ListDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ListDoc {}

        static FIELD_SCRIPT_PRINTER_LISTDOC__ELEMENTS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ListDoc", "elements")
                .expect("non-layout field elements must be registered in TVM reflection")
        });
        impl ListDoc {
            pub fn get_elements(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_LISTDOC__ELEMENTS
                    .get(&__obj)
                    .expect("non-layout field elements should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.DictDoc"]
        pub struct DictDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::ExprDocObj,
            values: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DictDoc {
            data: tvm_ffi::object::ObjectArc<DictDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DictDoc: crate::script::printer::ExprDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl DictDoc {
            pub fn get_values(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                self.data.values.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_DICTDOC__KEYS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.DictDoc", "keys")
                .expect("non-layout field keys must be registered in TVM reflection")
        });
        impl DictDoc {
            pub fn get_keys(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_DICTDOC__KEYS
                    .get(&__obj)
                    .expect("non-layout field keys should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.SliceDoc"]
        pub struct SliceDocObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::script::printer::DocObj,
            stop: Option<crate::script::printer::ExprDoc>,
            step: Option<crate::script::printer::ExprDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SliceDoc {
            data: tvm_ffi::object::ObjectArc<SliceDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SliceDoc: crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl SliceDoc {
            pub fn get_stop(&self) -> Option<crate::script::printer::ExprDoc> {
                self.data.stop.clone()
            }
            pub fn get_step(&self) -> Option<crate::script::printer::ExprDoc> {
                self.data.step.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_SLICEDOC__START: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.SliceDoc", "start")
                .expect("non-layout field start must be registered in TVM reflection")
        });
        impl SliceDoc {
            pub fn get_start(&self) -> Option<crate::script::printer::ExprDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_SLICEDOC__START
                    .get(&__obj)
                    .expect("non-layout field start should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.AssignDoc"]
        pub struct AssignDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            rhs: Option<crate::script::printer::ExprDoc>,
            annotation: Option<crate::script::printer::ExprDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AssignDoc {
            data: tvm_ffi::object::ObjectArc<AssignDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AssignDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl AssignDoc {
            pub fn get_rhs(&self) -> Option<crate::script::printer::ExprDoc> {
                self.data.rhs.clone()
            }
            pub fn get_annotation(&self) -> Option<crate::script::printer::ExprDoc> {
                self.data.annotation.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_ASSIGNDOC__LHS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssignDoc", "lhs")
                .expect("non-layout field lhs must be registered in TVM reflection")
        });
        impl AssignDoc {
            pub fn get_lhs(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_ASSIGNDOC__LHS
                    .get(&__obj)
                    .expect("non-layout field lhs should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.IfDoc"]
        pub struct IfDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            then_branch: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            else_branch: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IfDoc {
            data: tvm_ffi::object::ObjectArc<IfDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IfDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl IfDoc {
            pub fn get_then_branch(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.then_branch.clone()
            }
            pub fn get_else_branch(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.else_branch.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_IFDOC__PREDICATE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IfDoc", "predicate")
                .expect("non-layout field predicate must be registered in TVM reflection")
        });
        impl IfDoc {
            pub fn get_predicate(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_IFDOC__PREDICATE
                    .get(&__obj)
                    .expect("non-layout field predicate should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.WhileDoc"]
        pub struct WhileDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            body: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct WhileDoc {
            data: tvm_ffi::object::ObjectArc<WhileDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(WhileDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl WhileDoc {
            pub fn get_body(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.body.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_WHILEDOC__PREDICATE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.WhileDoc", "predicate")
                .expect("non-layout field predicate must be registered in TVM reflection")
        });
        impl WhileDoc {
            pub fn get_predicate(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_WHILEDOC__PREDICATE
                    .get(&__obj)
                    .expect("non-layout field predicate should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ForDoc"]
        pub struct ForDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            rhs: crate::script::printer::ExprDoc,
            body: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ForDoc {
            data: tvm_ffi::object::ObjectArc<ForDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ForDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ForDoc {
            pub fn get_rhs(&self) -> crate::script::printer::ExprDoc {
                self.data.rhs.clone()
            }
            pub fn get_body(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.body.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_FORDOC__LHS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ForDoc", "lhs")
                .expect("non-layout field lhs must be registered in TVM reflection")
        });
        impl ForDoc {
            pub fn get_lhs(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_FORDOC__LHS
                    .get(&__obj)
                    .expect("non-layout field lhs should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ScopeDoc"]
        pub struct ScopeDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            rhs: crate::script::printer::ExprDoc,
            body: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ScopeDoc {
            data: tvm_ffi::object::ObjectArc<ScopeDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ScopeDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ScopeDoc {
            pub fn get_rhs(&self) -> crate::script::printer::ExprDoc {
                self.data.rhs.clone()
            }
            pub fn get_body(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.body.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_SCOPEDOC__LHS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ScopeDoc", "lhs")
                .expect("non-layout field lhs must be registered in TVM reflection")
        });
        impl ScopeDoc {
            pub fn get_lhs(&self) -> Option<crate::script::printer::ExprDoc> {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_SCOPEDOC__LHS
                    .get(&__obj)
                    .expect("non-layout field lhs should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ExprStmtDoc"]
        pub struct ExprStmtDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ExprStmtDoc {
            data: tvm_ffi::object::ObjectArc<ExprStmtDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ExprStmtDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ExprStmtDoc {}

        static FIELD_SCRIPT_PRINTER_EXPRSTMTDOC__EXPR: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ExprStmtDoc", "expr")
                .expect("non-layout field expr must be registered in TVM reflection")
        });
        impl ExprStmtDoc {
            pub fn get_expr(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_EXPRSTMTDOC__EXPR
                    .get(&__obj)
                    .expect("non-layout field expr should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.AssertDoc"]
        pub struct AssertDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            msg: Option<crate::script::printer::ExprDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AssertDoc {
            data: tvm_ffi::object::ObjectArc<AssertDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AssertDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl AssertDoc {
            pub fn get_msg(&self) -> Option<crate::script::printer::ExprDoc> {
                self.data.msg.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_ASSERTDOC__TEST: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssertDoc", "test")
                .expect("non-layout field test must be registered in TVM reflection")
        });
        impl AssertDoc {
            pub fn get_test(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_ASSERTDOC__TEST
                    .get(&__obj)
                    .expect("non-layout field test should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ReturnDoc"]
        pub struct ReturnDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ReturnDoc {
            data: tvm_ffi::object::ObjectArc<ReturnDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ReturnDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ReturnDoc {}

        static FIELD_SCRIPT_PRINTER_RETURNDOC__VALUE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ReturnDoc", "value")
                .expect("non-layout field value must be registered in TVM reflection")
        });
        impl ReturnDoc {
            pub fn get_value(&self) -> crate::script::printer::ExprDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_RETURNDOC__VALUE
                    .get(&__obj)
                    .expect("non-layout field value should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.FunctionDoc"]
        pub struct FunctionDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            args: tvm_ffi::Array<crate::script::printer::AssignDoc>,
            decorators: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            return_type: Option<crate::script::printer::ExprDoc>,
            body: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct FunctionDoc {
            data: tvm_ffi::object::ObjectArc<FunctionDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(FunctionDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl FunctionDoc {
            pub fn get_args(&self) -> tvm_ffi::Array<crate::script::printer::AssignDoc> {
                self.data.args.clone()
            }
            pub fn get_decorators(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                self.data.decorators.clone()
            }
            pub fn get_return_type(&self) -> Option<crate::script::printer::ExprDoc> {
                self.data.return_type.clone()
            }
            pub fn get_body(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.body.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_FUNCTIONDOC__NAME: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::IdDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.FunctionDoc", "name")
                .expect("non-layout field name must be registered in TVM reflection")
        });
        impl FunctionDoc {
            pub fn get_name(&self) -> crate::script::printer::IdDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_FUNCTIONDOC__NAME
                    .get(&__obj)
                    .expect("non-layout field name should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.ClassDoc"]
        pub struct ClassDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
            decorators: tvm_ffi::Array<crate::script::printer::ExprDoc>,
            body: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ClassDoc {
            data: tvm_ffi::object::ObjectArc<ClassDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ClassDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl ClassDoc {
            pub fn get_decorators(&self) -> tvm_ffi::Array<crate::script::printer::ExprDoc> {
                self.data.decorators.clone()
            }
            pub fn get_body(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.body.clone()
            }
        }

        static FIELD_SCRIPT_PRINTER_CLASSDOC__NAME: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::IdDoc>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ClassDoc", "name")
                .expect("non-layout field name must be registered in TVM reflection")
        });
        impl ClassDoc {
            pub fn get_name(&self) -> crate::script::printer::IdDoc {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_CLASSDOC__NAME
                    .get(&__obj)
                    .expect("non-layout field name should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.CommentDoc"]
        pub struct CommentDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct CommentDoc {
            data: tvm_ffi::object::ObjectArc<CommentDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(CommentDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl CommentDoc {}

        impl CommentDoc {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.DocStringDoc"]
        pub struct DocStringDocObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::StmtDocObj,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct DocStringDoc {
            data: tvm_ffi::object::ObjectArc<DocStringDocObj>,
        }

        tvm_ffi::impl_object_hierarchy!(DocStringDoc: crate::script::printer::StmtDoc, crate::script::printer::Doc, tvm_ffi::object::ObjectRef);

        impl DocStringDoc {}

        impl DocStringDoc {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.Frame"]
        pub struct FrameObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            stmts: tvm_ffi::Array<crate::script::printer::StmtDoc>,
            _gap0: [u8; 40],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct Frame {
            data: tvm_ffi::object::ObjectArc<FrameObj>,
        }

        tvm_ffi::impl_object_hierarchy!(Frame: tvm_ffi::object::ObjectRef);

        impl Frame {
            pub fn get_stmts(&self) -> tvm_ffi::Array<crate::script::printer::StmtDoc> {
                self.data.stmts.clone()
            }
        }

        impl Frame {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.IRDocsifier"]
        pub struct IRDocsifierObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            _gap0: [u8; 8],
            frames: tvm_ffi::Array<crate::script::printer::Frame>,
            dispatch_tokens: tvm_ffi::Array<tvm_ffi::String>,
            _gap1: [u8; 336],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct IRDocsifier {
            data: tvm_ffi::object::ObjectArc<IRDocsifierObj>,
        }

        tvm_ffi::impl_object_hierarchy!(IRDocsifier: tvm_ffi::object::ObjectRef);

        impl IRDocsifier {
            pub fn get_frames(&self) -> tvm_ffi::Array<crate::script::printer::Frame> {
                self.data.frames.clone()
            }
            pub fn get_dispatch_tokens(&self) -> tvm_ffi::Array<tvm_ffi::String> {
                self.data.dispatch_tokens.clone()
            }
        }

        impl IRDocsifier {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.RelaxFrame"]
        pub struct RelaxFrameObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::FrameObj,
            _gap0: [u8; 16],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct RelaxFrame {
            data: tvm_ffi::object::ObjectArc<RelaxFrameObj>,
        }

        tvm_ffi::impl_object_hierarchy!(RelaxFrame: crate::script::printer::Frame, tvm_ffi::object::ObjectRef);

        impl RelaxFrame {}

        static FIELD_SCRIPT_PRINTER_RELAXFRAME__IS_FUNC: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.RelaxFrame", "is_func")
                .expect("non-layout field is_func must be registered in TVM reflection")
        });
        static FIELD_SCRIPT_PRINTER_RELAXFRAME__MODULE_ALIAS_PRINTED: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.printer.RelaxFrame",
                "module_alias_printed",
            )
            .expect("non-layout field module_alias_printed must be registered in TVM reflection")
        });
        impl RelaxFrame {
            pub fn get_is_func(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_RELAXFRAME__IS_FUNC
                    .get(&__obj)
                    .expect("non-layout field is_func should be accessible")
            }
            pub fn get_module_alias_printed(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_RELAXFRAME__MODULE_ALIAS_PRINTED
                    .get(&__obj)
                    .expect("non-layout field module_alias_printed should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.printer.TIRFrame"]
        pub struct TIRFrameObj {
            __tvm_ffi_object_parent:
                crate::_tvm_ffi_stubgen_detail::types::script::printer::FrameObj,
            allow_concise_scoping: bool,
            _gap0: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TIRFrame {
            data: tvm_ffi::object::ObjectArc<TIRFrameObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TIRFrame: crate::script::printer::Frame, tvm_ffi::object::ObjectRef);

        impl TIRFrame {
            pub fn get_allow_concise_scoping(&self) -> bool {
                self.data.allow_concise_scoping
            }
        }

        static FIELD_SCRIPT_PRINTER_TIRFRAME__TIR: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::object::ObjectRef>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.TIRFrame", "tir")
                .expect("non-layout field tir must be registered in TVM reflection")
        });
        impl TIRFrame {
            pub fn get_tir(&self) -> tvm_ffi::object::ObjectRef {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_SCRIPT_PRINTER_TIRFRAME__TIR
                    .get(&__obj)
                    .expect("non-layout field tir should be accessible")
            }
        }
    }
}
pub mod target {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "target.Target"]
    pub struct TargetObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        kind: crate::target::TargetKind,
        host: Option<tvm_ffi::object::ObjectRef>,
        tag: tvm_ffi::String,
        keys: tvm_ffi::Array<tvm_ffi::String>,
        attrs: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        features: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        _gap0: [u8; 32],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Target {
        data: tvm_ffi::object::ObjectArc<TargetObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Target: tvm_ffi::object::ObjectRef);

    impl Target {
        pub fn get_kind(&self) -> crate::target::TargetKind {
            self.data.kind.clone()
        }
        pub fn get_host(&self) -> Option<tvm_ffi::object::ObjectRef> {
            self.data.host.clone()
        }
        pub fn get_tag(&self) -> tvm_ffi::String {
            self.data.tag.clone()
        }
        pub fn get_keys(&self) -> tvm_ffi::Array<tvm_ffi::String> {
            self.data.keys.clone()
        }
        pub fn get_attrs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.attrs.clone()
        }
        pub fn get_features(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.features.clone()
        }
    }

    impl Target {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "target.TargetTag"]
    pub struct TargetTagObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        config: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TargetTag {
        data: tvm_ffi::object::ObjectArc<TargetTagObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TargetTag: tvm_ffi::object::ObjectRef);

    impl TargetTag {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_config(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.config.clone()
        }
    }

    impl TargetTag {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "target.MemoryInfo"]
    pub struct MemoryInfoObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        unit_bits: i64,
        max_num_bits: i64,
        max_simd_bits: i64,
        head_address: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct MemoryInfo {
        data: tvm_ffi::object::ObjectArc<MemoryInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(MemoryInfo: tvm_ffi::object::ObjectRef);

    impl MemoryInfo {
        pub fn get_unit_bits(&self) -> i64 {
            self.data.unit_bits
        }
        pub fn get_max_num_bits(&self) -> i64 {
            self.data.max_num_bits
        }
        pub fn get_max_simd_bits(&self) -> i64 {
            self.data.max_simd_bits
        }
        pub fn get_head_address(&self) -> crate::ir::PrimExpr {
            self.data.head_address.clone()
        }
    }

    impl MemoryInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "target.TargetKind"]
    pub struct TargetKindObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        default_device_type: i32,
        _gap0: [u8; 4],
        default_keys: tvm_ffi::Array<tvm_ffi::String>,
        _gap1: [u8; 136],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TargetKind {
        data: tvm_ffi::object::ObjectArc<TargetKindObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TargetKind: tvm_ffi::object::ObjectRef);

    impl TargetKind {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_default_device_type(&self) -> i32 {
            self.data.default_device_type
        }
        pub fn get_default_keys(&self) -> tvm_ffi::Array<tvm_ffi::String> {
            self.data.default_keys.clone()
        }
    }

    impl TargetKind {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "target.VirtualDevice"]
    pub struct VirtualDeviceObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
        target: crate::target::Target,
        memory_scope: tvm_ffi::String,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct VirtualDevice {
        data: tvm_ffi::object::ObjectArc<VirtualDeviceObj>,
    }

    tvm_ffi::impl_object_hierarchy!(VirtualDevice: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

    impl VirtualDevice {
        pub fn get_target(&self) -> crate::target::Target {
            self.data.target.clone()
        }
        pub fn get_memory_scope(&self) -> tvm_ffi::String {
            self.data.memory_scope.clone()
        }
    }

    static FIELD_TARGET_VIRTUALDEVICE__DEVICE_TYPE_INT: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i32>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.VirtualDevice", "device_type_int")
            .expect("non-layout field device_type_int must be registered in TVM reflection")
    });
    static FIELD_TARGET_VIRTUALDEVICE__VIRTUAL_DEVICE_ID: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i32>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.VirtualDevice", "virtual_device_id")
            .expect("non-layout field virtual_device_id must be registered in TVM reflection")
    });
    impl VirtualDevice {
        pub fn get_device_type_int(&self) -> i32 {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TARGET_VIRTUALDEVICE__DEVICE_TYPE_INT
                .get(&__obj)
                .expect("non-layout field device_type_int should be accessible")
        }
        pub fn get_virtual_device_id(&self) -> i32 {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TARGET_VIRTUALDEVICE__VIRTUAL_DEVICE_ID
                .get(&__obj)
                .expect("non-layout field virtual_device_id should be accessible")
        }
    }
}
pub mod te {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.Operation"]
    pub struct OperationObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        tag: tvm_ffi::String,
        attrs: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Operation {
        data: tvm_ffi::object::ObjectArc<OperationObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Operation: tvm_ffi::object::ObjectRef);

    impl Operation {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_tag(&self) -> tvm_ffi::String {
            self.data.tag.clone()
        }
        pub fn get_attrs(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.attrs.clone()
        }
    }

    impl Operation {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.PlaceholderOp"]
    pub struct PlaceholderOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::te::OperationObj,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 12],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PlaceholderOp {
        data: tvm_ffi::object::ObjectArc<PlaceholderOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PlaceholderOp: crate::te::Operation, tvm_ffi::object::ObjectRef);

    impl PlaceholderOp {
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
    }

    static FIELD_TE_PLACEHOLDEROP__SHAPE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("te.PlaceholderOp", "shape")
            .expect("non-layout field shape must be registered in TVM reflection")
    });
    impl PlaceholderOp {
        pub fn get_shape(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TE_PLACEHOLDEROP__SHAPE
                .get(&__obj)
                .expect("non-layout field shape should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.BaseComputeOp"]
    pub struct BaseComputeOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::te::OperationObj,
        reduce_axis: tvm_ffi::Array<crate::tir::IterVar>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BaseComputeOp {
        data: tvm_ffi::object::ObjectArc<BaseComputeOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BaseComputeOp: crate::te::Operation, tvm_ffi::object::ObjectRef);

    impl BaseComputeOp {
        pub fn get_reduce_axis(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
            self.data.reduce_axis.clone()
        }
    }

    static FIELD_TE_BASECOMPUTEOP__AXIS: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::IterVar>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("te.BaseComputeOp", "axis")
            .expect("non-layout field axis must be registered in TVM reflection")
    });
    impl BaseComputeOp {
        pub fn get_axis(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TE_BASECOMPUTEOP__AXIS
                .get(&__obj)
                .expect("non-layout field axis should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.ComputeOp"]
    pub struct ComputeOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::te::BaseComputeOpObj,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ComputeOp {
        data: tvm_ffi::object::ObjectArc<ComputeOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ComputeOp: crate::te::BaseComputeOp, crate::te::Operation, tvm_ffi::object::ObjectRef);

    impl ComputeOp {}

    static FIELD_TE_COMPUTEOP__BODY: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("te.ComputeOp", "body")
            .expect("non-layout field body must be registered in TVM reflection")
    });
    impl ComputeOp {
        pub fn get_body(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TE_COMPUTEOP__BODY
                .get(&__obj)
                .expect("non-layout field body should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.ExternOp"]
    pub struct ExternOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::te::OperationObj,
        input_placeholders: tvm_ffi::Array<crate::tir::Buffer>,
        output_placeholders: tvm_ffi::Array<crate::tir::Buffer>,
        body: crate::tir::Stmt,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ExternOp {
        data: tvm_ffi::object::ObjectArc<ExternOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ExternOp: crate::te::Operation, tvm_ffi::object::ObjectRef);

    impl ExternOp {
        pub fn get_input_placeholders(&self) -> tvm_ffi::Array<crate::tir::Buffer> {
            self.data.input_placeholders.clone()
        }
        pub fn get_output_placeholders(&self) -> tvm_ffi::Array<crate::tir::Buffer> {
            self.data.output_placeholders.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    static FIELD_TE_EXTERNOP__INPUTS: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::te::Tensor>>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("te.ExternOp", "inputs")
            .expect("non-layout field inputs must be registered in TVM reflection")
    });
    impl ExternOp {
        pub fn get_inputs(&self) -> tvm_ffi::Array<crate::te::Tensor> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TE_EXTERNOP__INPUTS
                .get(&__obj)
                .expect("non-layout field inputs should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.ScanOp"]
    pub struct ScanOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::te::OperationObj,
        init: tvm_ffi::Array<crate::te::Tensor>,
        update: tvm_ffi::Array<crate::te::Tensor>,
        state_placeholder: tvm_ffi::Array<crate::te::Tensor>,
        inputs: tvm_ffi::Array<crate::te::Tensor>,
        spatial_axis_: tvm_ffi::Array<crate::tir::IterVar>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ScanOp {
        data: tvm_ffi::object::ObjectArc<ScanOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ScanOp: crate::te::Operation, tvm_ffi::object::ObjectRef);

    impl ScanOp {
        pub fn get_init(&self) -> tvm_ffi::Array<crate::te::Tensor> {
            self.data.init.clone()
        }
        pub fn get_update(&self) -> tvm_ffi::Array<crate::te::Tensor> {
            self.data.update.clone()
        }
        pub fn get_state_placeholder(&self) -> tvm_ffi::Array<crate::te::Tensor> {
            self.data.state_placeholder.clone()
        }
        pub fn get_inputs(&self) -> tvm_ffi::Array<crate::te::Tensor> {
            self.data.inputs.clone()
        }
        pub fn get_spatial_axis_(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
            self.data.spatial_axis_.clone()
        }
    }

    static FIELD_TE_SCANOP__SCAN_AXIS: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::IterVar>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("te.ScanOp", "scan_axis")
            .expect("non-layout field scan_axis must be registered in TVM reflection")
    });
    impl ScanOp {
        pub fn get_scan_axis(&self) -> crate::tir::IterVar {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TE_SCANOP__SCAN_AXIS
                .get(&__obj)
                .expect("non-layout field scan_axis should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "te.Tensor"]
    pub struct TensorObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        shape: tvm_ffi::Array<crate::ir::PrimExpr>,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
        op: crate::te::Operation,
        value_index: i32,
        _gap1: [u8; 12],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Tensor {
        data: tvm_ffi::object::ObjectArc<TensorObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Tensor: tvm_ffi::object::ObjectRef);

    impl Tensor {
        pub fn get_shape(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.shape.clone()
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
        pub fn get_op(&self) -> crate::te::Operation {
            self.data.op.clone()
        }
        pub fn get_value_index(&self) -> i32 {
            self.data.value_index
        }
    }

    impl Tensor {}
}
pub mod tir {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Var"]
    pub struct VarObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        name: tvm_ffi::String,
        type_annotation: crate::ir::Type,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Var {
        data: tvm_ffi::object::ObjectArc<VarObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Var: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Var {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_type_annotation(&self) -> crate::ir::Type {
            self.data.type_annotation.clone()
        }
    }

    impl Var {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.SizeVar"]
    pub struct SizeVarObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::VarObj,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct SizeVar {
        data: tvm_ffi::object::ObjectArc<SizeVarObj>,
    }

    tvm_ffi::impl_object_hierarchy!(SizeVar: crate::tir::Var, crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl SizeVar {}

    impl SizeVar {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BufferLoad"]
    pub struct BufferLoadObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        buffer: crate::tir::Buffer,
        indices: tvm_ffi::Array<crate::ir::PrimExpr>,
        predicate: Option<crate::ir::PrimExpr>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BufferLoad {
        data: tvm_ffi::object::ObjectArc<BufferLoadObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BufferLoad: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl BufferLoad {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            self.data.buffer.clone()
        }
        pub fn get_indices(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.indices.clone()
        }
        pub fn get_predicate(&self) -> Option<crate::ir::PrimExpr> {
            self.data.predicate.clone()
        }
    }

    impl BufferLoad {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.ProducerLoad"]
    pub struct ProducerLoadObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        producer: crate::tir::DataProducer,
        indices: tvm_ffi::Array<crate::ir::PrimExpr>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ProducerLoad {
        data: tvm_ffi::object::ObjectArc<ProducerLoadObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ProducerLoad: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl ProducerLoad {
        pub fn get_producer(&self) -> crate::tir::DataProducer {
            self.data.producer.clone()
        }
        pub fn get_indices(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.indices.clone()
        }
    }

    impl ProducerLoad {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Not"]
    pub struct NotObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Not {
        data: tvm_ffi::object::ObjectArc<NotObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Not: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Not {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
    }

    impl Not {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.StringImm"]
    pub struct StringImmObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        value: tvm_ffi::String,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct StringImm {
        data: tvm_ffi::object::ObjectArc<StringImmObj>,
    }

    tvm_ffi::impl_object_hierarchy!(StringImm: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl StringImm {
        pub fn get_value(&self) -> tvm_ffi::String {
            self.data.value.clone()
        }
    }

    impl StringImm {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Cast"]
    pub struct CastObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        value: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Cast {
        data: tvm_ffi::object::ObjectArc<CastObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Cast: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Cast {
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
    }

    impl Cast {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Select"]
    pub struct SelectObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        condition: crate::ir::PrimExpr,
        true_value: crate::ir::PrimExpr,
        false_value: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Select {
        data: tvm_ffi::object::ObjectArc<SelectObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Select: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Select {
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_true_value(&self) -> crate::ir::PrimExpr {
            self.data.true_value.clone()
        }
        pub fn get_false_value(&self) -> crate::ir::PrimExpr {
            self.data.false_value.clone()
        }
    }

    impl Select {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Ramp"]
    pub struct RampObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        base: crate::ir::PrimExpr,
        stride: crate::ir::PrimExpr,
        lanes: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Ramp {
        data: tvm_ffi::object::ObjectArc<RampObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Ramp: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Ramp {
        pub fn get_base(&self) -> crate::ir::PrimExpr {
            self.data.base.clone()
        }
        pub fn get_stride(&self) -> crate::ir::PrimExpr {
            self.data.stride.clone()
        }
        pub fn get_lanes(&self) -> crate::ir::PrimExpr {
            self.data.lanes.clone()
        }
    }

    impl Ramp {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Broadcast"]
    pub struct BroadcastObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        value: crate::ir::PrimExpr,
        lanes: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Broadcast {
        data: tvm_ffi::object::ObjectArc<BroadcastObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Broadcast: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Broadcast {
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
        pub fn get_lanes(&self) -> crate::ir::PrimExpr {
            self.data.lanes.clone()
        }
    }

    impl Broadcast {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Shuffle"]
    pub struct ShuffleObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        vectors: tvm_ffi::Array<crate::ir::PrimExpr>,
        indices: tvm_ffi::Array<crate::ir::PrimExpr>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Shuffle {
        data: tvm_ffi::object::ObjectArc<ShuffleObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Shuffle: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Shuffle {
        pub fn get_vectors(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.vectors.clone()
        }
        pub fn get_indices(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.indices.clone()
        }
    }

    impl Shuffle {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Let"]
    pub struct LetObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        var: crate::tir::Var,
        value: crate::ir::PrimExpr,
        body: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Let {
        data: tvm_ffi::object::ObjectArc<LetObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Let: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Let {
        pub fn get_var(&self) -> crate::tir::Var {
            self.data.var.clone()
        }
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
        pub fn get_body(&self) -> crate::ir::PrimExpr {
            self.data.body.clone()
        }
    }

    impl Let {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Call"]
    pub struct CallObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        op: crate::ir::RelaxExpr,
        args: tvm_ffi::Array<crate::ir::PrimExpr>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Call {
        data: tvm_ffi::object::ObjectArc<CallObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Call: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Call {
        pub fn get_op(&self) -> crate::ir::RelaxExpr {
            self.data.op.clone()
        }
        pub fn get_args(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.args.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef> {
            self.data.annotations.clone()
        }
    }

    impl Call {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Reduce"]
    pub struct ReduceObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        combiner: crate::tir::CommReducer,
        source: tvm_ffi::Array<crate::ir::PrimExpr>,
        init: tvm_ffi::Array<crate::ir::PrimExpr>,
        axis: tvm_ffi::Array<crate::tir::IterVar>,
        condition: crate::ir::PrimExpr,
        value_index: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Reduce {
        data: tvm_ffi::object::ObjectArc<ReduceObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Reduce: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Reduce {
        pub fn get_combiner(&self) -> crate::tir::CommReducer {
            self.data.combiner.clone()
        }
        pub fn get_source(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.source.clone()
        }
        pub fn get_init(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.init.clone()
        }
        pub fn get_axis(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
            self.data.axis.clone()
        }
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_value_index(&self) -> i32 {
            self.data.value_index
        }
    }

    impl Reduce {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Div"]
    pub struct DivObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Div {
        data: tvm_ffi::object::ObjectArc<DivObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Div: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Div {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Div {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Add"]
    pub struct AddObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Add {
        data: tvm_ffi::object::ObjectArc<AddObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Add: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Add {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Add {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Sub"]
    pub struct SubObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Sub {
        data: tvm_ffi::object::ObjectArc<SubObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Sub: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Sub {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Sub {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Mul"]
    pub struct MulObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Mul {
        data: tvm_ffi::object::ObjectArc<MulObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Mul: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Mul {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Mul {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.FloorDiv"]
    pub struct FloorDivObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FloorDiv {
        data: tvm_ffi::object::ObjectArc<FloorDivObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FloorDiv: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl FloorDiv {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl FloorDiv {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.FloorMod"]
    pub struct FloorModObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FloorMod {
        data: tvm_ffi::object::ObjectArc<FloorModObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FloorMod: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl FloorMod {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl FloorMod {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.LT"]
    pub struct LTObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct LT {
        data: tvm_ffi::object::ObjectArc<LTObj>,
    }

    tvm_ffi::impl_object_hierarchy!(LT: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl LT {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl LT {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.LE"]
    pub struct LEObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct LE {
        data: tvm_ffi::object::ObjectArc<LEObj>,
    }

    tvm_ffi::impl_object_hierarchy!(LE: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl LE {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl LE {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.EQ"]
    pub struct EQObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct EQ {
        data: tvm_ffi::object::ObjectArc<EQObj>,
    }

    tvm_ffi::impl_object_hierarchy!(EQ: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl EQ {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl EQ {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.NE"]
    pub struct NEObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct NE {
        data: tvm_ffi::object::ObjectArc<NEObj>,
    }

    tvm_ffi::impl_object_hierarchy!(NE: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl NE {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl NE {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.GT"]
    pub struct GTObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GT {
        data: tvm_ffi::object::ObjectArc<GTObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GT: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl GT {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl GT {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.GE"]
    pub struct GEObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GE {
        data: tvm_ffi::object::ObjectArc<GEObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GE: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl GE {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl GE {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.And"]
    pub struct AndObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct And {
        data: tvm_ffi::object::ObjectArc<AndObj>,
    }

    tvm_ffi::impl_object_hierarchy!(And: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl And {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl And {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Or"]
    pub struct OrObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Or {
        data: tvm_ffi::object::ObjectArc<OrObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Or: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Or {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Or {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Mod"]
    pub struct ModObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Mod {
        data: tvm_ffi::object::ObjectArc<ModObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Mod: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Mod {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Mod {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Min"]
    pub struct MinObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Min {
        data: tvm_ffi::object::ObjectArc<MinObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Min: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Min {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Min {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Max"]
    pub struct MaxObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::PrimExprObj,
        a: crate::ir::PrimExpr,
        b: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Max {
        data: tvm_ffi::object::ObjectArc<MaxObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Max: crate::ir::PrimExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl Max {
        pub fn get_a(&self) -> crate::ir::PrimExpr {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::ir::PrimExpr {
            self.data.b.clone()
        }
    }

    impl Max {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Buffer"]
    pub struct BufferObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        data: crate::tir::Var,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
        shape: tvm_ffi::Array<crate::ir::PrimExpr>,
        axis_separators: tvm_ffi::Array<crate::ir::IntImm>,
        strides: tvm_ffi::Array<crate::ir::PrimExpr>,
        elem_offset: crate::ir::PrimExpr,
        name: tvm_ffi::String,
        data_alignment: i32,
        offset_factor: i32,
        buffer_type: i32,
        _gap1: [u8; 4],
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Buffer {
        data: tvm_ffi::object::ObjectArc<BufferObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Buffer: tvm_ffi::object::ObjectRef);

    impl Buffer {
        pub fn get_data(&self) -> crate::tir::Var {
            self.data.data.clone()
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
        pub fn get_shape(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.shape.clone()
        }
        pub fn get_axis_separators(&self) -> tvm_ffi::Array<crate::ir::IntImm> {
            self.data.axis_separators.clone()
        }
        pub fn get_strides(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.strides.clone()
        }
        pub fn get_elem_offset(&self) -> crate::ir::PrimExpr {
            self.data.elem_offset.clone()
        }
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_data_alignment(&self) -> i32 {
            self.data.data_alignment
        }
        pub fn get_offset_factor(&self) -> i32 {
            self.data.offset_factor
        }
        pub fn get_buffer_type(&self) -> i32 {
            self.data.buffer_type
        }
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl Buffer {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.IterVar"]
    pub struct IterVarObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        dom: crate::ir::Range,
        var: crate::tir::Var,
        iter_type: i32,
        _gap0: [u8; 4],
        thread_tag: tvm_ffi::String,
        _gap1: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct IterVar {
        data: tvm_ffi::object::ObjectArc<IterVarObj>,
    }

    tvm_ffi::impl_object_hierarchy!(IterVar: tvm_ffi::object::ObjectRef);

    impl IterVar {
        pub fn get_dom(&self) -> crate::ir::Range {
            self.data.dom.clone()
        }
        pub fn get_var(&self) -> crate::tir::Var {
            self.data.var.clone()
        }
        pub fn get_iter_type(&self) -> i32 {
            self.data.iter_type
        }
        pub fn get_thread_tag(&self) -> tvm_ffi::String {
            self.data.thread_tag.clone()
        }
    }

    impl IterVar {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Stmt"]
    pub struct StmtObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Stmt {
        data: tvm_ffi::object::ObjectArc<StmtObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Stmt: tvm_ffi::object::ObjectRef);

    impl Stmt {
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl Stmt {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BlockRealize"]
    pub struct BlockRealizeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        iter_values: tvm_ffi::Array<crate::ir::PrimExpr>,
        predicate: crate::ir::PrimExpr,
        block: crate::tir::Block,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BlockRealize {
        data: tvm_ffi::object::ObjectArc<BlockRealizeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BlockRealize: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl BlockRealize {
        pub fn get_iter_values(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.iter_values.clone()
        }
        pub fn get_predicate(&self) -> crate::ir::PrimExpr {
            self.data.predicate.clone()
        }
        pub fn get_block(&self) -> crate::tir::Block {
            self.data.block.clone()
        }
    }

    impl BlockRealize {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Block"]
    pub struct BlockObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        iter_vars: tvm_ffi::Array<crate::tir::IterVar>,
        reads: tvm_ffi::Array<crate::tir::BufferRegion>,
        writes: tvm_ffi::Array<crate::tir::BufferRegion>,
        name_hint: tvm_ffi::String,
        alloc_buffers: tvm_ffi::Array<crate::tir::Buffer>,
        match_buffers: tvm_ffi::Array<crate::tir::MatchBufferRegion>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        init: Option<crate::tir::Stmt>,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Block {
        data: tvm_ffi::object::ObjectArc<BlockObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Block: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl Block {
        pub fn get_iter_vars(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
            self.data.iter_vars.clone()
        }
        pub fn get_reads(&self) -> tvm_ffi::Array<crate::tir::BufferRegion> {
            self.data.reads.clone()
        }
        pub fn get_writes(&self) -> tvm_ffi::Array<crate::tir::BufferRegion> {
            self.data.writes.clone()
        }
        pub fn get_name_hint(&self) -> tvm_ffi::String {
            self.data.name_hint.clone()
        }
        pub fn get_alloc_buffers(&self) -> tvm_ffi::Array<crate::tir::Buffer> {
            self.data.alloc_buffers.clone()
        }
        pub fn get_match_buffers(&self) -> tvm_ffi::Array<crate::tir::MatchBufferRegion> {
            self.data.match_buffers.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.annotations.clone()
        }
        pub fn get_init(&self) -> Option<crate::tir::Stmt> {
            self.data.init.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl Block {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BufferStore"]
    pub struct BufferStoreObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer: crate::tir::Buffer,
        value: crate::ir::PrimExpr,
        indices: tvm_ffi::Array<crate::ir::PrimExpr>,
        predicate: Option<crate::ir::PrimExpr>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BufferStore {
        data: tvm_ffi::object::ObjectArc<BufferStoreObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BufferStore: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl BufferStore {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            self.data.buffer.clone()
        }
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
        pub fn get_indices(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.indices.clone()
        }
        pub fn get_predicate(&self) -> Option<crate::ir::PrimExpr> {
            self.data.predicate.clone()
        }
    }

    impl BufferStore {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.For"]
    pub struct ForObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        loop_var: crate::tir::Var,
        min: crate::ir::PrimExpr,
        extent: crate::ir::PrimExpr,
        kind: i32,
        _gap0: [u8; 4],
        body: crate::tir::Stmt,
        thread_binding: Option<crate::tir::IterVar>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        step: Option<crate::ir::PrimExpr>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct For {
        data: tvm_ffi::object::ObjectArc<ForObj>,
    }

    tvm_ffi::impl_object_hierarchy!(For: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl For {
        pub fn get_loop_var(&self) -> crate::tir::Var {
            self.data.loop_var.clone()
        }
        pub fn get_min(&self) -> crate::ir::PrimExpr {
            self.data.min.clone()
        }
        pub fn get_extent(&self) -> crate::ir::PrimExpr {
            self.data.extent.clone()
        }
        pub fn get_kind(&self) -> i32 {
            self.data.kind
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
        pub fn get_thread_binding(&self) -> Option<crate::tir::IterVar> {
            self.data.thread_binding.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.annotations.clone()
        }
        pub fn get_step(&self) -> Option<crate::ir::PrimExpr> {
            self.data.step.clone()
        }
    }

    impl For {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Evaluate"]
    pub struct EvaluateObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        value: crate::ir::PrimExpr,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Evaluate {
        data: tvm_ffi::object::ObjectArc<EvaluateObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Evaluate: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl Evaluate {
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
    }

    impl Evaluate {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.LetStmt"]
    pub struct LetStmtObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        var: crate::tir::Var,
        value: crate::ir::PrimExpr,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct LetStmt {
        data: tvm_ffi::object::ObjectArc<LetStmtObj>,
    }

    tvm_ffi::impl_object_hierarchy!(LetStmt: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl LetStmt {
        pub fn get_var(&self) -> crate::tir::Var {
            self.data.var.clone()
        }
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl LetStmt {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.AssertStmt"]
    pub struct AssertStmtObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        condition: crate::ir::PrimExpr,
        message: crate::ir::PrimExpr,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AssertStmt {
        data: tvm_ffi::object::ObjectArc<AssertStmtObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AssertStmt: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl AssertStmt {
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_message(&self) -> crate::ir::PrimExpr {
            self.data.message.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl AssertStmt {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.While"]
    pub struct WhileObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        condition: crate::ir::PrimExpr,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct While {
        data: tvm_ffi::object::ObjectArc<WhileObj>,
    }

    tvm_ffi::impl_object_hierarchy!(While: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl While {
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl While {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.DeclBuffer"]
    pub struct DeclBufferObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer: crate::tir::Buffer,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct DeclBuffer {
        data: tvm_ffi::object::ObjectArc<DeclBufferObj>,
    }

    tvm_ffi::impl_object_hierarchy!(DeclBuffer: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl DeclBuffer {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            self.data.buffer.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl DeclBuffer {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.IfThenElse"]
    pub struct IfThenElseObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        condition: crate::ir::PrimExpr,
        then_case: crate::tir::Stmt,
        else_case: Option<crate::tir::Stmt>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct IfThenElse {
        data: tvm_ffi::object::ObjectArc<IfThenElseObj>,
    }

    tvm_ffi::impl_object_hierarchy!(IfThenElse: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl IfThenElse {
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_then_case(&self) -> crate::tir::Stmt {
            self.data.then_case.clone()
        }
        pub fn get_else_case(&self) -> Option<crate::tir::Stmt> {
            self.data.else_case.clone()
        }
    }

    impl IfThenElse {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.SeqStmt"]
    pub struct SeqStmtObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        seq: tvm_ffi::Array<crate::tir::Stmt>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct SeqStmt {
        data: tvm_ffi::object::ObjectArc<SeqStmtObj>,
    }

    tvm_ffi::impl_object_hierarchy!(SeqStmt: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl SeqStmt {
        pub fn get_seq(&self) -> tvm_ffi::Array<crate::tir::Stmt> {
            self.data.seq.clone()
        }
    }

    impl SeqStmt {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Allocate"]
    pub struct AllocateObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer_var: crate::tir::Var,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
        extents: tvm_ffi::Array<crate::ir::PrimExpr>,
        condition: crate::ir::PrimExpr,
        body: crate::tir::Stmt,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Allocate {
        data: tvm_ffi::object::ObjectArc<AllocateObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Allocate: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl Allocate {
        pub fn get_buffer_var(&self) -> crate::tir::Var {
            self.data.buffer_var.clone()
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
        pub fn get_extents(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.extents.clone()
        }
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.annotations.clone()
        }
    }

    impl Allocate {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.AllocateConst"]
    pub struct AllocateConstObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer_var: crate::tir::Var,
        data: Option<tvm_ffi::Tensor>,
        irmod_storage_idx: Option<crate::ir::IntImm>,
        dtype: tvm_ffi::DLDataType,
        _gap0: [u8; 4],
        extents: tvm_ffi::Array<crate::ir::PrimExpr>,
        body: crate::tir::Stmt,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AllocateConst {
        data: tvm_ffi::object::ObjectArc<AllocateConstObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AllocateConst: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl AllocateConst {
        pub fn get_buffer_var(&self) -> crate::tir::Var {
            self.data.buffer_var.clone()
        }
        pub fn get_data(&self) -> Option<tvm_ffi::Tensor> {
            self.data.data.clone()
        }
        pub fn get_irmod_storage_idx(&self) -> Option<crate::ir::IntImm> {
            self.data.irmod_storage_idx.clone()
        }
        pub fn get_dtype(&self) -> tvm_ffi::DLDataType {
            self.data.dtype
        }
        pub fn get_extents(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.extents.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.annotations.clone()
        }
    }

    impl AllocateConst {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BufferRealize"]
    pub struct BufferRealizeObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer: crate::tir::Buffer,
        bounds: tvm_ffi::Array<crate::ir::Range>,
        condition: crate::ir::PrimExpr,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BufferRealize {
        data: tvm_ffi::object::ObjectArc<BufferRealizeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BufferRealize: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl BufferRealize {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            self.data.buffer.clone()
        }
        pub fn get_bounds(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.bounds.clone()
        }
        pub fn get_condition(&self) -> crate::ir::PrimExpr {
            self.data.condition.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl BufferRealize {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.AttrStmt"]
    pub struct AttrStmtObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        node: tvm_ffi::AnyValue,
        attr_key: tvm_ffi::String,
        value: crate::ir::PrimExpr,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AttrStmt {
        data: tvm_ffi::object::ObjectArc<AttrStmtObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AttrStmt: crate::tir::Stmt, tvm_ffi::object::ObjectRef);

    impl AttrStmt {
        pub fn get_node(&self) -> tvm_ffi::AnyValue {
            self.data.node.clone()
        }
        pub fn get_attr_key(&self) -> tvm_ffi::String {
            self.data.attr_key.clone()
        }
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl AttrStmt {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BufferRegion"]
    pub struct BufferRegionObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        buffer: crate::tir::Buffer,
        region: tvm_ffi::Array<crate::ir::Range>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BufferRegion {
        data: tvm_ffi::object::ObjectArc<BufferRegionObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BufferRegion: tvm_ffi::object::ObjectRef);

    impl BufferRegion {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            self.data.buffer.clone()
        }
        pub fn get_region(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.region.clone()
        }
    }

    impl BufferRegion {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.MatchBufferRegion"]
    pub struct MatchBufferRegionObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        buffer: crate::tir::Buffer,
        source: crate::tir::BufferRegion,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct MatchBufferRegion {
        data: tvm_ffi::object::ObjectArc<MatchBufferRegionObj>,
    }

    tvm_ffi::impl_object_hierarchy!(MatchBufferRegion: tvm_ffi::object::ObjectRef);

    impl MatchBufferRegion {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            self.data.buffer.clone()
        }
        pub fn get_source(&self) -> crate::tir::BufferRegion {
            self.data.source.clone()
        }
    }

    impl MatchBufferRegion {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.CommReducer"]
    pub struct CommReducerObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        lhs: tvm_ffi::Array<crate::tir::Var>,
        rhs: tvm_ffi::Array<crate::tir::Var>,
        result: tvm_ffi::Array<crate::ir::PrimExpr>,
        identity_element: tvm_ffi::Array<crate::ir::PrimExpr>,
        span: crate::ir::Span,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct CommReducer {
        data: tvm_ffi::object::ObjectArc<CommReducerObj>,
    }

    tvm_ffi::impl_object_hierarchy!(CommReducer: tvm_ffi::object::ObjectRef);

    impl CommReducer {
        pub fn get_lhs(&self) -> tvm_ffi::Array<crate::tir::Var> {
            self.data.lhs.clone()
        }
        pub fn get_rhs(&self) -> tvm_ffi::Array<crate::tir::Var> {
            self.data.rhs.clone()
        }
        pub fn get_result(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.result.clone()
        }
        pub fn get_identity_element(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.identity_element.clone()
        }
        pub fn get_span(&self) -> crate::ir::Span {
            self.data.span.clone()
        }
    }

    impl CommReducer {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.IndexMap"]
    pub struct IndexMapObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        initial_indices: tvm_ffi::Array<crate::tir::Var>,
        final_indices: tvm_ffi::Array<crate::ir::PrimExpr>,
        inverse_index_map: Option<tvm_ffi::object::ObjectRef>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct IndexMap {
        data: tvm_ffi::object::ObjectArc<IndexMapObj>,
    }

    tvm_ffi::impl_object_hierarchy!(IndexMap: tvm_ffi::object::ObjectRef);

    impl IndexMap {
        pub fn get_initial_indices(&self) -> tvm_ffi::Array<crate::tir::Var> {
            self.data.initial_indices.clone()
        }
        pub fn get_final_indices(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.final_indices.clone()
        }
        pub fn get_inverse_index_map(&self) -> Option<tvm_ffi::object::ObjectRef> {
            self.data.inverse_index_map.clone()
        }
    }

    impl IndexMap {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.PrimFunc"]
    pub struct PrimFuncObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseFuncObj,
        params: tvm_ffi::Array<crate::tir::Var>,
        ret_type: crate::ir::Type,
        buffer_map: tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
        body: crate::tir::Stmt,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PrimFunc {
        data: tvm_ffi::object::ObjectArc<PrimFuncObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PrimFunc: crate::ir::BaseFunc, crate::ir::RelaxExpr, crate::ir::BaseExpr, tvm_ffi::object::ObjectRef);

    impl PrimFunc {
        pub fn get_params(&self) -> tvm_ffi::Array<crate::tir::Var> {
            self.data.params.clone()
        }
        pub fn get_ret_type(&self) -> crate::ir::Type {
            self.data.ret_type.clone()
        }
        pub fn get_buffer_map(&self) -> tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer> {
            self.data.buffer_map.clone()
        }
        pub fn get_body(&self) -> crate::tir::Stmt {
            self.data.body.clone()
        }
    }

    impl PrimFunc {}

    tvm_ffi::define_object_wrapper!(DataProducer, "tir.DataProducer");

    impl DataProducer {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BlockDependenceInfo"]
    pub struct BlockDependenceInfoObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 112],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BlockDependenceInfo {
        data: tvm_ffi::object::ObjectArc<BlockDependenceInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BlockDependenceInfo: tvm_ffi::object::ObjectRef);

    impl BlockDependenceInfo {}

    impl BlockDependenceInfo {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.StmtSRef"]
    pub struct StmtSRefObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 16],
        seq_index: i64,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct StmtSRef {
        data: tvm_ffi::object::ObjectArc<StmtSRefObj>,
    }

    tvm_ffi::impl_object_hierarchy!(StmtSRef: tvm_ffi::object::ObjectRef);

    impl StmtSRef {
        pub fn get_seq_index(&self) -> i64 {
            self.data.seq_index
        }
    }

    impl StmtSRef {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Dependency"]
    pub struct DependencyObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        src: crate::tir::StmtSRef,
        dst: crate::tir::StmtSRef,
        kind: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Dependency {
        data: tvm_ffi::object::ObjectArc<DependencyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Dependency: tvm_ffi::object::ObjectRef);

    impl Dependency {
        pub fn get_src(&self) -> crate::tir::StmtSRef {
            self.data.src.clone()
        }
        pub fn get_dst(&self) -> crate::tir::StmtSRef {
            self.data.dst.clone()
        }
        pub fn get_kind(&self) -> i32 {
            self.data.kind
        }
    }

    impl Dependency {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BlockScope"]
    pub struct BlockScopeObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 168],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BlockScope {
        data: tvm_ffi::object::ObjectArc<BlockScopeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BlockScope: tvm_ffi::object::ObjectRef);

    impl BlockScope {}

    impl BlockScope {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Layout"]
    pub struct LayoutObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        axes: tvm_ffi::Array<crate::tir::IterVar>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Layout {
        data: tvm_ffi::object::ObjectArc<LayoutObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Layout: tvm_ffi::object::ObjectRef);

    impl Layout {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_axes(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
            self.data.axes.clone()
        }
    }

    impl Layout {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BijectiveLayout"]
    pub struct BijectiveLayoutObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        index_forward_rule: tvm_ffi::Array<crate::ir::PrimExpr>,
        index_backward_rule: tvm_ffi::Array<crate::ir::PrimExpr>,
        shape_forward_rule: tvm_ffi::Array<crate::ir::PrimExpr>,
        shape_backward_rule: tvm_ffi::Array<crate::ir::PrimExpr>,
        src_layout: crate::tir::Layout,
        dst_layout: crate::tir::Layout,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BijectiveLayout {
        data: tvm_ffi::object::ObjectArc<BijectiveLayoutObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BijectiveLayout: tvm_ffi::object::ObjectRef);

    impl BijectiveLayout {
        pub fn get_index_forward_rule(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.index_forward_rule.clone()
        }
        pub fn get_index_backward_rule(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.index_backward_rule.clone()
        }
        pub fn get_shape_forward_rule(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.shape_forward_rule.clone()
        }
        pub fn get_shape_backward_rule(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.shape_backward_rule.clone()
        }
        pub fn get_src_layout(&self) -> crate::tir::Layout {
            self.data.src_layout.clone()
        }
        pub fn get_dst_layout(&self) -> crate::tir::Layout {
            self.data.dst_layout.clone()
        }
    }

    impl BijectiveLayout {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.TensorIntrin"]
    pub struct TensorIntrinObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        desc: crate::tir::PrimFunc,
        impl_: crate::tir::PrimFunc,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TensorIntrin {
        data: tvm_ffi::object::ObjectArc<TensorIntrinObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TensorIntrin: tvm_ffi::object::ObjectRef);

    impl TensorIntrin {
        pub fn get_desc(&self) -> crate::tir::PrimFunc {
            self.data.desc.clone()
        }
        pub fn get_impl_(&self) -> crate::tir::PrimFunc {
            self.data.impl_.clone()
        }
    }

    impl TensorIntrin {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.PyStmtExprVisitor"]
    pub struct PyStmtExprVisitorObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 416],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PyStmtExprVisitor {
        data: tvm_ffi::object::ObjectArc<PyStmtExprVisitorObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PyStmtExprVisitor: tvm_ffi::object::ObjectRef);

    impl PyStmtExprVisitor {}

    impl PyStmtExprVisitor {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.PyStmtExprMutator"]
    pub struct PyStmtExprMutatorObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 424],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PyStmtExprMutator {
        data: tvm_ffi::object::ObjectArc<PyStmtExprMutatorObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PyStmtExprMutator: tvm_ffi::object::ObjectRef);

    impl PyStmtExprMutator {}

    impl PyStmtExprMutator {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.PrimFuncPass"]
    pub struct PrimFuncPassObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        pass_info: crate::transform::PassInfo,
        _gap0: [u8; 40],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PrimFuncPass {
        data: tvm_ffi::object::ObjectArc<PrimFuncPassObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PrimFuncPass: tvm_ffi::object::ObjectRef);

    impl PrimFuncPass {
        pub fn get_pass_info(&self) -> crate::transform::PassInfo {
            self.data.pass_info.clone()
        }
    }

    impl PrimFuncPass {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.InstructionKind"]
    pub struct InstructionKindObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        name: tvm_ffi::String,
        _is_pure: bool,
        _gap0: [u8; 39],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct InstructionKind {
        data: tvm_ffi::object::ObjectArc<InstructionKindObj>,
    }

    tvm_ffi::impl_object_hierarchy!(InstructionKind: tvm_ffi::object::ObjectRef);

    impl InstructionKind {
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get__is_pure(&self) -> bool {
            self.data._is_pure
        }
    }

    impl InstructionKind {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Instruction"]
    pub struct InstructionObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        kind: crate::tir::InstructionKind,
        inputs: tvm_ffi::Array<tvm_ffi::AnyValue>,
        attrs: tvm_ffi::Array<tvm_ffi::AnyValue>,
        outputs: tvm_ffi::Array<tvm_ffi::AnyValue>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Instruction {
        data: tvm_ffi::object::ObjectArc<InstructionObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Instruction: tvm_ffi::object::ObjectRef);

    impl Instruction {
        pub fn get_kind(&self) -> crate::tir::InstructionKind {
            self.data.kind.clone()
        }
        pub fn get_inputs(&self) -> tvm_ffi::Array<tvm_ffi::AnyValue> {
            self.data.inputs.clone()
        }
        pub fn get_attrs(&self) -> tvm_ffi::Array<tvm_ffi::AnyValue> {
            self.data.attrs.clone()
        }
        pub fn get_outputs(&self) -> tvm_ffi::Array<tvm_ffi::AnyValue> {
            self.data.outputs.clone()
        }
    }

    impl Instruction {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Schedule"]
    pub struct ScheduleObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Schedule {
        data: tvm_ffi::object::ObjectArc<ScheduleObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Schedule: tvm_ffi::object::ObjectRef);

    impl Schedule {}

    impl Schedule {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BlockRV"]
    pub struct BlockRVObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct BlockRV {
        data: tvm_ffi::object::ObjectArc<BlockRVObj>,
    }

    tvm_ffi::impl_object_hierarchy!(BlockRV: tvm_ffi::object::ObjectRef);

    impl BlockRV {}

    impl BlockRV {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.LoopRV"]
    pub struct LoopRVObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct LoopRV {
        data: tvm_ffi::object::ObjectArc<LoopRVObj>,
    }

    tvm_ffi::impl_object_hierarchy!(LoopRV: tvm_ffi::object::ObjectRef);

    impl LoopRV {}

    impl LoopRV {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.ScheduleState"]
    pub struct ScheduleStateObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        mod_: crate::ir::IRModule,
        _gap0: [u8; 112],
        debug_mask: i32,
        enable_check: bool,
        _gap1: [u8; 3],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ScheduleState {
        data: tvm_ffi::object::ObjectArc<ScheduleStateObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ScheduleState: tvm_ffi::object::ObjectRef);

    impl ScheduleState {
        pub fn get_mod_(&self) -> crate::ir::IRModule {
            self.data.mod_.clone()
        }
        pub fn get_debug_mask(&self) -> i32 {
            self.data.debug_mask
        }
        pub fn get_enable_check(&self) -> bool {
            self.data.enable_check
        }
    }

    impl ScheduleState {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Trace"]
    pub struct TraceObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        insts: tvm_ffi::Array<crate::tir::Instruction>,
        decisions: tvm_ffi::Map<crate::tir::Instruction, tvm_ffi::AnyValue>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Trace {
        data: tvm_ffi::object::ObjectArc<TraceObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Trace: tvm_ffi::object::ObjectRef);

    impl Trace {
        pub fn get_insts(&self) -> tvm_ffi::Array<crate::tir::Instruction> {
            self.data.insts.clone()
        }
        pub fn get_decisions(&self) -> tvm_ffi::Map<crate::tir::Instruction, tvm_ffi::AnyValue> {
            self.data.decisions.clone()
        }
    }

    impl Trace {}

    pub mod schedule {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.schedule.TensorizeInfo"]
        pub struct TensorizeInfoObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            loop_map: tvm_ffi::Map<crate::tir::StmtSRef, crate::tir::For>,
            desc_loop_indexer: tvm_ffi::Map<crate::tir::For, crate::ir::IntImm>,
            block_iter_paddings: Option<tvm_ffi::Array<crate::ir::IntImm>>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct TensorizeInfo {
            data: tvm_ffi::object::ObjectArc<TensorizeInfoObj>,
        }

        tvm_ffi::impl_object_hierarchy!(TensorizeInfo: tvm_ffi::object::ObjectRef);

        impl TensorizeInfo {
            pub fn get_loop_map(&self) -> tvm_ffi::Map<crate::tir::StmtSRef, crate::tir::For> {
                self.data.loop_map.clone()
            }
            pub fn get_desc_loop_indexer(
                &self,
            ) -> tvm_ffi::Map<crate::tir::For, crate::ir::IntImm> {
                self.data.desc_loop_indexer.clone()
            }
            pub fn get_block_iter_paddings(&self) -> Option<tvm_ffi::Array<crate::ir::IntImm>> {
                self.data.block_iter_paddings.clone()
            }
        }

        impl TensorizeInfo {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.schedule.AutoTensorizeMappingInfo"]
        pub struct AutoTensorizeMappingInfoObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            mappings: tvm_ffi::Array<crate::tir::IndexMap>,
            lhs_buffer_map: tvm_ffi::Map<crate::tir::Buffer, crate::tir::Buffer>,
            rhs_buffer_indices:
                tvm_ffi::Map<crate::tir::Buffer, tvm_ffi::Array<crate::ir::PrimExpr>>,
            lhs_iters: tvm_ffi::Array<crate::tir::IterVar>,
            rhs_iters: tvm_ffi::Array<crate::tir::IterVar>,
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct AutoTensorizeMappingInfo {
            data: tvm_ffi::object::ObjectArc<AutoTensorizeMappingInfoObj>,
        }

        tvm_ffi::impl_object_hierarchy!(AutoTensorizeMappingInfo: tvm_ffi::object::ObjectRef);

        impl AutoTensorizeMappingInfo {
            pub fn get_mappings(&self) -> tvm_ffi::Array<crate::tir::IndexMap> {
                self.data.mappings.clone()
            }
            pub fn get_lhs_buffer_map(
                &self,
            ) -> tvm_ffi::Map<crate::tir::Buffer, crate::tir::Buffer> {
                self.data.lhs_buffer_map.clone()
            }
            pub fn get_rhs_buffer_indices(
                &self,
            ) -> tvm_ffi::Map<crate::tir::Buffer, tvm_ffi::Array<crate::ir::PrimExpr>> {
                self.data.rhs_buffer_indices.clone()
            }
            pub fn get_lhs_iters(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
                self.data.lhs_iters.clone()
            }
            pub fn get_rhs_iters(&self) -> tvm_ffi::Array<crate::tir::IterVar> {
                self.data.rhs_iters.clone()
            }
        }

        impl AutoTensorizeMappingInfo {}
    }
    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.HoistExpressionConfig"]
        pub struct HoistExpressionConfigObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            hoisted_conditionals: i32,
            hoisted_let_bindings: i32,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct HoistExpressionConfig {
            data: tvm_ffi::object::ObjectArc<HoistExpressionConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(HoistExpressionConfig: tvm_ffi::object::ObjectRef);

        impl HoistExpressionConfig {
            pub fn get_hoisted_conditionals(&self) -> i32 {
                self.data.hoisted_conditionals
            }
            pub fn get_hoisted_let_bindings(&self) -> i32 {
                self.data.hoisted_let_bindings
            }
        }

        impl HoistExpressionConfig {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.HoistIfThenElseConfig"]
        pub struct HoistIfThenElseConfigObj {
            __tvm_ffi_object_parent: tvm_ffi::object::Object,
            support_block_scope_hoisting: bool,
            _gap0: [u8; 15],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct HoistIfThenElseConfig {
            data: tvm_ffi::object::ObjectArc<HoistIfThenElseConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(HoistIfThenElseConfig: tvm_ffi::object::ObjectRef);

        impl HoistIfThenElseConfig {
            pub fn get_support_block_scope_hoisting(&self) -> bool {
                self.data.support_block_scope_hoisting
            }
        }

        impl HoistIfThenElseConfig {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.InjectDoubleBufferConfig"]
        pub struct InjectDoubleBufferConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct InjectDoubleBufferConfig {
            data: tvm_ffi::object::ObjectArc<InjectDoubleBufferConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(InjectDoubleBufferConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl InjectDoubleBufferConfig {}

        static FIELD_TIR_TRANSFORM_INJECTDOUBLEBUFFERCONFIG__SPLIT_LOOP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.InjectDoubleBufferConfig",
                "split_loop",
            )
            .expect("non-layout field split_loop must be registered in TVM reflection")
        });
        impl InjectDoubleBufferConfig {
            pub fn get_split_loop(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_INJECTDOUBLEBUFFERCONFIG__SPLIT_LOOP
                    .get(&__obj)
                    .expect("non-layout field split_loop should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.LoopPartitionConfig"]
        pub struct LoopPartitionConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct LoopPartitionConfig {
            data: tvm_ffi::object::ObjectArc<LoopPartitionConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(LoopPartitionConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl LoopPartitionConfig {}

        static FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__PARTITION_CONST_LOOP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.LoopPartitionConfig",
                "partition_const_loop",
            )
            .expect("non-layout field partition_const_loop must be registered in TVM reflection")
        });
        static FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__NO_UNROLL_LOOP_WITH_EXTENT_ONE:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.LoopPartitionConfig", "no_unroll_loop_with_extent_one")
                .expect("non-layout field no_unroll_loop_with_extent_one must be registered in TVM reflection")
            });
        static FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__UNROLL_LOOP_WITH_PARTITION_HINT_NO_INTERVAL: std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.LoopPartitionConfig", "unroll_loop_with_partition_hint_no_interval")
                .expect("non-layout field unroll_loop_with_partition_hint_no_interval must be registered in TVM reflection")
        });
        impl LoopPartitionConfig {
            pub fn get_partition_const_loop(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__PARTITION_CONST_LOOP
                    .get(&__obj)
                    .expect("non-layout field partition_const_loop should be accessible")
            }
            pub fn get_no_unroll_loop_with_extent_one(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__NO_UNROLL_LOOP_WITH_EXTENT_ONE
                    .get(&__obj)
                    .expect("non-layout field no_unroll_loop_with_extent_one should be accessible")
            }
            pub fn get_unroll_loop_with_partition_hint_no_interval(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__UNROLL_LOOP_WITH_PARTITION_HINT_NO_INTERVAL.get(&__obj).expect("non-layout field unroll_loop_with_partition_hint_no_interval should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.ReduceBranchingThroughOvercomputeConfig"]
        pub struct ReduceBranchingThroughOvercomputeConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct ReduceBranchingThroughOvercomputeConfig {
            data: tvm_ffi::object::ObjectArc<ReduceBranchingThroughOvercomputeConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(ReduceBranchingThroughOvercomputeConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl ReduceBranchingThroughOvercomputeConfig {}

        static FIELD_TIR_TRANSFORM_REDUCEBRANCHINGTHROUGHOVERCOMPUTECONFIG__USE_DATAFLOW_ANALYSIS: std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.ReduceBranchingThroughOvercomputeConfig", "use_dataflow_analysis")
                .expect("non-layout field use_dataflow_analysis must be registered in TVM reflection")
        });
        impl ReduceBranchingThroughOvercomputeConfig {
            pub fn get_use_dataflow_analysis(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_REDUCEBRANCHINGTHROUGHOVERCOMPUTECONFIG__USE_DATAFLOW_ANALYSIS
                    .get(&__obj)
                    .expect("non-layout field use_dataflow_analysis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.RemoveNoOpConfig"]
        pub struct RemoveNoOpConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            max_simplification_steps: i64,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct RemoveNoOpConfig {
            data: tvm_ffi::object::ObjectArc<RemoveNoOpConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(RemoveNoOpConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl RemoveNoOpConfig {
            pub fn get_max_simplification_steps(&self) -> i64 {
                self.data.max_simplification_steps
            }
        }

        static FIELD_TIR_TRANSFORM_REMOVENOOPCONFIG__USE_DATAFLOW_ANALYSIS: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.RemoveNoOpConfig",
                "use_dataflow_analysis",
            )
            .expect("non-layout field use_dataflow_analysis must be registered in TVM reflection")
        });
        impl RemoveNoOpConfig {
            pub fn get_use_dataflow_analysis(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_REMOVENOOPCONFIG__USE_DATAFLOW_ANALYSIS
                    .get(&__obj)
                    .expect("non-layout field use_dataflow_analysis should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.SimplifyConfig"]
        pub struct SimplifyConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SimplifyConfig {
            data: tvm_ffi::object::ObjectArc<SimplifyConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SimplifyConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SimplifyConfig {}

        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.SimplifyConfig", "transitively_prove_inequalities")
                .expect("non-layout field transitively_prove_inequalities must be registered in TVM reflection")
            });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.SimplifyConfig", "propagate_knowns_to_prove_conditional")
                .expect("non-layout field propagate_knowns_to_prove_conditional must be registered in TVM reflection")
            });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.SimplifyConfig", "propagate_knowns_to_simplify_expressions")
                .expect("non-layout field propagate_knowns_to_simplify_expressions must be registered in TVM reflection")
            });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.SimplifyConfig", "convert_boolean_to_and_of_ors")
                .expect("non-layout field convert_boolean_to_and_of_ors must be registered in TVM reflection")
            });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.SimplifyConfig", "apply_constraints_to_boolean_branches")
                .expect("non-layout field apply_constraints_to_boolean_branches must be registered in TVM reflection")
            });
        impl SimplifyConfig {
            pub fn get_transitively_prove_inequalities(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES
                    .get(&__obj)
                    .expect("non-layout field transitively_prove_inequalities should be accessible")
            }
            pub fn get_propagate_knowns_to_prove_conditional(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL.get(&__obj).expect("non-layout field propagate_knowns_to_prove_conditional should be accessible")
            }
            pub fn get_propagate_knowns_to_simplify_expressions(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS.get(&__obj).expect("non-layout field propagate_knowns_to_simplify_expressions should be accessible")
            }
            pub fn get_convert_boolean_to_and_of_ors(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS
                    .get(&__obj)
                    .expect("non-layout field convert_boolean_to_and_of_ors should be accessible")
            }
            pub fn get_apply_constraints_to_boolean_branches(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES.get(&__obj).expect("non-layout field apply_constraints_to_boolean_branches should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tir.transform.UnrollLoopConfig"]
        pub struct UnrollLoopConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            auto_max_extent: i32,
            explicit_unroll: i32,
            unroll_local_access: i32,
            _gap0: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct UnrollLoopConfig {
            data: tvm_ffi::object::ObjectArc<UnrollLoopConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(UnrollLoopConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl UnrollLoopConfig {
            pub fn get_auto_max_extent(&self) -> i32 {
                self.data.auto_max_extent
            }
            pub fn get_explicit_unroll(&self) -> i32 {
                self.data.explicit_unroll
            }
            pub fn get_unroll_local_access(&self) -> i32 {
                self.data.unroll_local_access
            }
        }

        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "auto_max_step",
            )
            .expect("non-layout field auto_max_step must be registered in TVM reflection")
        });
        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "auto_max_depth",
            )
            .expect("non-layout field auto_max_depth must be registered in TVM reflection")
        });
        impl UnrollLoopConfig {
            pub fn get_auto_max_step(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP
                    .get(&__obj)
                    .expect("non-layout field auto_max_step should be accessible")
            }
            pub fn get_auto_max_depth(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH
                    .get(&__obj)
                    .expect("non-layout field auto_max_depth should be accessible")
            }
        }
    }
}
pub mod tl {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.KernelLaunchFrame"]
    pub struct KernelLaunchFrameObj {
        __tvm_ffi_object_parent:
            crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct KernelLaunchFrame {
        data: tvm_ffi::object::ObjectArc<KernelLaunchFrameObj>,
    }

    tvm_ffi::impl_object_hierarchy!(KernelLaunchFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

    impl KernelLaunchFrame {}

    static FIELD_TL_KERNELLAUNCHFRAME__FRAMES: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame>,
        >,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.KernelLaunchFrame", "frames")
            .expect("non-layout field frames must be registered in TVM reflection")
    });
    impl KernelLaunchFrame {
        pub fn get_frames(&self) -> tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_KERNELLAUNCHFRAME__FRAMES
                .get(&__obj)
                .expect("non-layout field frames should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.WarpSpecializeFrame"]
    pub struct WarpSpecializeFrameObj {
        __tvm_ffi_object_parent:
            crate::_tvm_ffi_stubgen_detail::types::script::ir_builder::tir::TIRFrameObj,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct WarpSpecializeFrame {
        data: tvm_ffi::object::ObjectArc<WarpSpecializeFrameObj>,
    }

    tvm_ffi::impl_object_hierarchy!(WarpSpecializeFrame: crate::script::ir_builder::tir::TIRFrame, crate::script::ir_builder::IRBuilderFrame, tvm_ffi::object::ObjectRef);

    impl WarpSpecializeFrame {}

    static FIELD_TL_WARPSPECIALIZEFRAME__FRAMES: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame>,
        >,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.WarpSpecializeFrame", "frames")
            .expect("non-layout field frames must be registered in TVM reflection")
    });
    impl WarpSpecializeFrame {
        pub fn get_frames(&self) -> tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame> {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_WARPSPECIALIZEFRAME__FRAMES
                .get(&__obj)
                .expect("non-layout field frames should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.Layout"]
    pub struct LayoutObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        forward_index: tvm_ffi::Array<crate::ir::PrimExpr>,
        input_size: tvm_ffi::Array<crate::ir::PrimExpr>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Layout {
        data: tvm_ffi::object::ObjectArc<LayoutObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Layout: tvm_ffi::object::ObjectRef);

    impl Layout {
        pub fn get_forward_index(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.forward_index.clone()
        }
        pub fn get_input_size(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.input_size.clone()
        }
    }

    static METHOD_TL_LAYOUT___DEBUGOUTPUT: LazyLock<tvm_ffi::Function> = LazyLock::new(|| {
        tvm_ffi::object_wrapper::resolve_type_method("tl.Layout", "_DebugOutput")
            .expect("missing type method")
    });
    impl Layout {
        pub fn _DebugOutput(&self, args: &[Any]) -> Result<Any> {
            let func = &*METHOD_TL_LAYOUT___DEBUGOUTPUT;
            let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
            views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
            views.extend(args.iter().map(AnyView::from));
            func.call_packed(&views)
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.Fragment"]
    pub struct FragmentObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::LayoutObj,
        forward_thread: crate::ir::PrimExpr,
        replicate_size: crate::ir::PrimExpr,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Fragment {
        data: tvm_ffi::object::ObjectArc<FragmentObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Fragment: crate::tl::Layout, tvm_ffi::object::ObjectRef);

    impl Fragment {
        pub fn get_forward_thread(&self) -> crate::ir::PrimExpr {
            self.data.forward_thread.clone()
        }
        pub fn get_replicate_size(&self) -> crate::ir::PrimExpr {
            self.data.replicate_size.clone()
        }
    }

    static METHOD_TL_FRAGMENT___DEBUGOUTPUT: LazyLock<tvm_ffi::Function> = LazyLock::new(|| {
        tvm_ffi::object_wrapper::resolve_type_method("tl.Fragment", "_DebugOutput")
            .expect("missing type method")
    });
    impl Fragment {
        pub fn _DebugOutput(&self, args: &[Any]) -> Result<Any> {
            let func = &*METHOD_TL_FRAGMENT___DEBUGOUTPUT;
            let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
            views.push(AnyView::from(self as &tvm_ffi::object::ObjectRef));
            views.extend(args.iter().map(AnyView::from));
            func.call_packed(&views)
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.TileOperator"]
    pub struct TileOperatorObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct TileOperator {
        data: tvm_ffi::object::ObjectArc<TileOperatorObj>,
    }

    tvm_ffi::impl_object_hierarchy!(TileOperator: tvm_ffi::object::ObjectRef);

    impl TileOperator {}

    impl TileOperator {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.AtomicAdd"]
    pub struct AtomicAddObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        src: crate::tir::Buffer,
        dst: crate::tir::Buffer,
        src_range: tvm_ffi::Array<crate::ir::Range>,
        dst_range: tvm_ffi::Array<crate::ir::Range>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        _gap0: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AtomicAdd {
        data: tvm_ffi::object::ObjectArc<AtomicAddObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AtomicAdd: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl AtomicAdd {
        pub fn get_src(&self) -> crate::tir::Buffer {
            self.data.src.clone()
        }
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_src_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.src_range.clone()
        }
        pub fn get_dst_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.dst_range.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef> {
            self.data.annotations.clone()
        }
    }

    static FIELD_TL_ATOMICADD__SRC_VALUE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "src_value")
            .expect("non-layout field src_value must be registered in TVM reflection")
    });
    impl AtomicAdd {
        pub fn get_src_value(&self) -> crate::ir::PrimExpr {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_ATOMICADD__SRC_VALUE
                .get(&__obj)
                .expect("non-layout field src_value should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.AtomicMax"]
    pub struct AtomicMaxObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        src: crate::tir::Buffer,
        dst: crate::tir::Buffer,
        src_range: tvm_ffi::Array<crate::ir::Range>,
        dst_range: tvm_ffi::Array<crate::ir::Range>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        _gap0: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AtomicMax {
        data: tvm_ffi::object::ObjectArc<AtomicMaxObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AtomicMax: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl AtomicMax {
        pub fn get_src(&self) -> crate::tir::Buffer {
            self.data.src.clone()
        }
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_src_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.src_range.clone()
        }
        pub fn get_dst_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.dst_range.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef> {
            self.data.annotations.clone()
        }
    }

    static FIELD_TL_ATOMICMAX__SRC_VALUE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "src_value")
            .expect("non-layout field src_value must be registered in TVM reflection")
    });
    impl AtomicMax {
        pub fn get_src_value(&self) -> crate::ir::PrimExpr {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_ATOMICMAX__SRC_VALUE
                .get(&__obj)
                .expect("non-layout field src_value should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.AtomicMin"]
    pub struct AtomicMinObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        src: crate::tir::Buffer,
        dst: crate::tir::Buffer,
        src_range: tvm_ffi::Array<crate::ir::Range>,
        dst_range: tvm_ffi::Array<crate::ir::Range>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        _gap0: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct AtomicMin {
        data: tvm_ffi::object::ObjectArc<AtomicMinObj>,
    }

    tvm_ffi::impl_object_hierarchy!(AtomicMin: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl AtomicMin {
        pub fn get_src(&self) -> crate::tir::Buffer {
            self.data.src.clone()
        }
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_src_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.src_range.clone()
        }
        pub fn get_dst_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.dst_range.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef> {
            self.data.annotations.clone()
        }
    }

    static FIELD_TL_ATOMICMIN__SRC_VALUE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "src_value")
            .expect("non-layout field src_value must be registered in TVM reflection")
    });
    impl AtomicMin {
        pub fn get_src_value(&self) -> crate::ir::PrimExpr {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_ATOMICMIN__SRC_VALUE
                .get(&__obj)
                .expect("non-layout field src_value should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.Copy"]
    pub struct CopyObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        dst: crate::tir::Buffer,
        src_range: tvm_ffi::Array<crate::ir::Range>,
        dst_range: tvm_ffi::Array<crate::ir::Range>,
        annotations: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        _gap0: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Copy {
        data: tvm_ffi::object::ObjectArc<CopyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Copy: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl Copy {
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_src_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.src_range.clone()
        }
        pub fn get_dst_range(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.dst_range.clone()
        }
        pub fn get_annotations(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef> {
            self.data.annotations.clone()
        }
    }

    static FIELD_TL_COPY__SRC: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Copy", "src")
            .expect("non-layout field src must be registered in TVM reflection")
    });
    impl Copy {
        pub fn get_src(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_COPY__SRC
                .get(&__obj)
                .expect("non-layout field src should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.Conv2DIm2Col"]
    pub struct Conv2DIm2ColObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        dstRegion: crate::tir::BufferRegion,
        src: crate::tir::Buffer,
        dst: crate::tir::Buffer,
        stride: i32,
        padding: i32,
        dilation: i32,
        kernel: i32,
        eviction_policy: i32,
        _gap0: [u8; 28],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Conv2DIm2Col {
        data: tvm_ffi::object::ObjectArc<Conv2DIm2ColObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Conv2DIm2Col: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl Conv2DIm2Col {
        pub fn get_dstRegion(&self) -> crate::tir::BufferRegion {
            self.data.dstRegion.clone()
        }
        pub fn get_src(&self) -> crate::tir::Buffer {
            self.data.src.clone()
        }
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_stride(&self) -> i32 {
            self.data.stride
        }
        pub fn get_padding(&self) -> i32 {
            self.data.padding
        }
        pub fn get_dilation(&self) -> i32 {
            self.data.dilation
        }
        pub fn get_kernel(&self) -> i32 {
            self.data.kernel
        }
        pub fn get_eviction_policy(&self) -> i32 {
            self.data.eviction_policy
        }
    }

    static FIELD_TL_CONV2DIM2COL__SRCREGION: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "srcRegion")
            .expect("non-layout field srcRegion must be registered in TVM reflection")
    });
    impl Conv2DIm2Col {
        pub fn get_srcRegion(&self) -> crate::tir::BufferRegion {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_CONV2DIM2COL__SRCREGION
                .get(&__obj)
                .expect("non-layout field srcRegion should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.Fill"]
    pub struct FillObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        value: crate::ir::PrimExpr,
        region: tvm_ffi::Array<crate::ir::Range>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Fill {
        data: tvm_ffi::object::ObjectArc<FillObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Fill: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl Fill {
        pub fn get_value(&self) -> crate::ir::PrimExpr {
            self.data.value.clone()
        }
        pub fn get_region(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.region.clone()
        }
    }

    static FIELD_TL_FILL__DST: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Fill", "dst")
            .expect("non-layout field dst must be registered in TVM reflection")
    });
    impl Fill {
        pub fn get_dst(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_FILL__DST
                .get(&__obj)
                .expect("non-layout field dst should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.FinalizeReducerOp"]
    pub struct FinalizeReducerOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        op: i32,
        _gap0: [u8; 12],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct FinalizeReducerOp {
        data: tvm_ffi::object::ObjectArc<FinalizeReducerOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(FinalizeReducerOp: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl FinalizeReducerOp {
        pub fn get_op(&self) -> i32 {
            self.data.op
        }
    }

    static FIELD_TL_FINALIZEREDUCEROP__REDUCER: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.FinalizeReducerOp", "reducer")
            .expect("non-layout field reducer must be registered in TVM reflection")
    });
    impl FinalizeReducerOp {
        pub fn get_reducer(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_FINALIZEREDUCEROP__REDUCER
                .get(&__obj)
                .expect("non-layout field reducer should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.Gemm"]
    pub struct GemmObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        b: crate::tir::Buffer,
        c: crate::tir::Buffer,
        aRegion: crate::tir::BufferRegion,
        bRegion: crate::tir::BufferRegion,
        cRegion: crate::tir::BufferRegion,
        transA: bool,
        transB: bool,
        _gap0: [u8; 2],
        m: i32,
        n: i32,
        k: i32,
        strideA: i32,
        strideB: i32,
        offsetA: i32,
        offsetB: i32,
        clearAccum: crate::ir::PrimExpr,
        kPack: i32,
        wgWait: i32,
        _gap1: [u8; 32],
        policy: crate::tl::GemmWarpPolicy,
        _gap2: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Gemm {
        data: tvm_ffi::object::ObjectArc<GemmObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Gemm: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl Gemm {
        pub fn get_b(&self) -> crate::tir::Buffer {
            self.data.b.clone()
        }
        pub fn get_c(&self) -> crate::tir::Buffer {
            self.data.c.clone()
        }
        pub fn get_aRegion(&self) -> crate::tir::BufferRegion {
            self.data.aRegion.clone()
        }
        pub fn get_bRegion(&self) -> crate::tir::BufferRegion {
            self.data.bRegion.clone()
        }
        pub fn get_cRegion(&self) -> crate::tir::BufferRegion {
            self.data.cRegion.clone()
        }
        pub fn get_transA(&self) -> bool {
            self.data.transA
        }
        pub fn get_transB(&self) -> bool {
            self.data.transB
        }
        pub fn get_m(&self) -> i32 {
            self.data.m
        }
        pub fn get_n(&self) -> i32 {
            self.data.n
        }
        pub fn get_k(&self) -> i32 {
            self.data.k
        }
        pub fn get_strideA(&self) -> i32 {
            self.data.strideA
        }
        pub fn get_strideB(&self) -> i32 {
            self.data.strideB
        }
        pub fn get_offsetA(&self) -> i32 {
            self.data.offsetA
        }
        pub fn get_offsetB(&self) -> i32 {
            self.data.offsetB
        }
        pub fn get_clearAccum(&self) -> crate::ir::PrimExpr {
            self.data.clearAccum.clone()
        }
        pub fn get_kPack(&self) -> i32 {
            self.data.kPack
        }
        pub fn get_wgWait(&self) -> i32 {
            self.data.wgWait
        }
        pub fn get_policy(&self) -> crate::tl::GemmWarpPolicy {
            self.data.policy.clone()
        }
    }

    static FIELD_TL_GEMM__A: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "a")
            .expect("non-layout field a must be registered in TVM reflection")
    });
    impl Gemm {
        pub fn get_a(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMM__A
                .get(&__obj)
                .expect("non-layout field a should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.GemmWarpPolicy"]
    pub struct GemmWarpPolicyObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        m_warp: i32,
        n_warp: i32,
        policy_type: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GemmWarpPolicy {
        data: tvm_ffi::object::ObjectArc<GemmWarpPolicyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GemmWarpPolicy: tvm_ffi::object::ObjectRef);

    impl GemmWarpPolicy {
        pub fn get_m_warp(&self) -> i32 {
            self.data.m_warp
        }
        pub fn get_n_warp(&self) -> i32 {
            self.data.n_warp
        }
        pub fn get_policy_type(&self) -> i32 {
            self.data.policy_type
        }
    }

    impl GemmWarpPolicy {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.GemmPy"]
    pub struct GemmPyObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        b: crate::tir::Buffer,
        c: crate::tir::Buffer,
        aRegion: crate::tir::BufferRegion,
        bRegion: crate::tir::BufferRegion,
        cRegion: crate::tir::BufferRegion,
        transA: bool,
        transB: bool,
        _gap0: [u8; 2],
        m: i32,
        n: i32,
        k: i32,
        strideA: i32,
        strideB: i32,
        offsetA: i32,
        offsetB: i32,
        clearAccum: crate::ir::PrimExpr,
        mbarRegion: crate::tir::BufferRegion,
        mbar: crate::tir::Buffer,
        cCoords: tvm_ffi::Array<crate::ir::PrimExpr>,
        kPack: i32,
        wgWait: i32,
        policy: crate::tl::GemmWarpPolicy,
        _gap1: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GemmPy {
        data: tvm_ffi::object::ObjectArc<GemmPyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GemmPy: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl GemmPy {
        pub fn get_b(&self) -> crate::tir::Buffer {
            self.data.b.clone()
        }
        pub fn get_c(&self) -> crate::tir::Buffer {
            self.data.c.clone()
        }
        pub fn get_aRegion(&self) -> crate::tir::BufferRegion {
            self.data.aRegion.clone()
        }
        pub fn get_bRegion(&self) -> crate::tir::BufferRegion {
            self.data.bRegion.clone()
        }
        pub fn get_cRegion(&self) -> crate::tir::BufferRegion {
            self.data.cRegion.clone()
        }
        pub fn get_transA(&self) -> bool {
            self.data.transA
        }
        pub fn get_transB(&self) -> bool {
            self.data.transB
        }
        pub fn get_m(&self) -> i32 {
            self.data.m
        }
        pub fn get_n(&self) -> i32 {
            self.data.n
        }
        pub fn get_k(&self) -> i32 {
            self.data.k
        }
        pub fn get_strideA(&self) -> i32 {
            self.data.strideA
        }
        pub fn get_strideB(&self) -> i32 {
            self.data.strideB
        }
        pub fn get_offsetA(&self) -> i32 {
            self.data.offsetA
        }
        pub fn get_offsetB(&self) -> i32 {
            self.data.offsetB
        }
        pub fn get_clearAccum(&self) -> crate::ir::PrimExpr {
            self.data.clearAccum.clone()
        }
        pub fn get_mbarRegion(&self) -> crate::tir::BufferRegion {
            self.data.mbarRegion.clone()
        }
        pub fn get_mbar(&self) -> crate::tir::Buffer {
            self.data.mbar.clone()
        }
        pub fn get_cCoords(&self) -> tvm_ffi::Array<crate::ir::PrimExpr> {
            self.data.cCoords.clone()
        }
        pub fn get_kPack(&self) -> i32 {
            self.data.kPack
        }
        pub fn get_wgWait(&self) -> i32 {
            self.data.wgWait
        }
        pub fn get_policy(&self) -> crate::tl::GemmWarpPolicy {
            self.data.policy.clone()
        }
    }

    static FIELD_TL_GEMMPY__A: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "a")
            .expect("non-layout field a must be registered in TVM reflection")
    });
    impl GemmPy {
        pub fn get_a(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMMPY__A
                .get(&__obj)
                .expect("non-layout field a should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.GemmSP"]
    pub struct GemmSPObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        bRegion: crate::tir::BufferRegion,
        cRegion: crate::tir::BufferRegion,
        eRegion: crate::tir::BufferRegion,
        a: crate::tir::Buffer,
        b: crate::tir::Buffer,
        c: crate::tir::Buffer,
        e: crate::tir::Buffer,
        transA: bool,
        transB: bool,
        _gap0: [u8; 2],
        m: i32,
        n: i32,
        k: i32,
        clearAccum: bool,
        _gap1: [u8; 3],
        kPack: i32,
        wgWait: i32,
        _gap2: [u8; 4],
        policy: crate::tl::GemmSPWarpPolicy,
        _gap3: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GemmSP {
        data: tvm_ffi::object::ObjectArc<GemmSPObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GemmSP: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl GemmSP {
        pub fn get_bRegion(&self) -> crate::tir::BufferRegion {
            self.data.bRegion.clone()
        }
        pub fn get_cRegion(&self) -> crate::tir::BufferRegion {
            self.data.cRegion.clone()
        }
        pub fn get_eRegion(&self) -> crate::tir::BufferRegion {
            self.data.eRegion.clone()
        }
        pub fn get_a(&self) -> crate::tir::Buffer {
            self.data.a.clone()
        }
        pub fn get_b(&self) -> crate::tir::Buffer {
            self.data.b.clone()
        }
        pub fn get_c(&self) -> crate::tir::Buffer {
            self.data.c.clone()
        }
        pub fn get_e(&self) -> crate::tir::Buffer {
            self.data.e.clone()
        }
        pub fn get_transA(&self) -> bool {
            self.data.transA
        }
        pub fn get_transB(&self) -> bool {
            self.data.transB
        }
        pub fn get_m(&self) -> i32 {
            self.data.m
        }
        pub fn get_n(&self) -> i32 {
            self.data.n
        }
        pub fn get_k(&self) -> i32 {
            self.data.k
        }
        pub fn get_clearAccum(&self) -> bool {
            self.data.clearAccum
        }
        pub fn get_kPack(&self) -> i32 {
            self.data.kPack
        }
        pub fn get_wgWait(&self) -> i32 {
            self.data.wgWait
        }
        pub fn get_policy(&self) -> crate::tl::GemmSPWarpPolicy {
            self.data.policy.clone()
        }
    }

    static FIELD_TL_GEMMSP__AREGION: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "aRegion")
            .expect("non-layout field aRegion must be registered in TVM reflection")
    });
    impl GemmSP {
        pub fn get_aRegion(&self) -> crate::tir::BufferRegion {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMMSP__AREGION
                .get(&__obj)
                .expect("non-layout field aRegion should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.GemmSPWarpPolicy"]
    pub struct GemmSPWarpPolicyObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::GemmWarpPolicyObj,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GemmSPWarpPolicy {
        data: tvm_ffi::object::ObjectArc<GemmSPWarpPolicyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GemmSPWarpPolicy: crate::tl::GemmWarpPolicy, tvm_ffi::object::ObjectRef);

    impl GemmSPWarpPolicy {}

    static FIELD_TL_GEMMSPWARPPOLICY__POLICY_TYPE: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i32>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPWarpPolicy", "policy_type")
            .expect("non-layout field policy_type must be registered in TVM reflection")
    });
    static FIELD_TL_GEMMSPWARPPOLICY__M_WARP: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i32>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPWarpPolicy", "m_warp")
            .expect("non-layout field m_warp must be registered in TVM reflection")
    });
    static FIELD_TL_GEMMSPWARPPOLICY__N_WARP: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i32>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPWarpPolicy", "n_warp")
            .expect("non-layout field n_warp must be registered in TVM reflection")
    });
    impl GemmSPWarpPolicy {
        pub fn get_policy_type(&self) -> i32 {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMMSPWARPPOLICY__POLICY_TYPE
                .get(&__obj)
                .expect("non-layout field policy_type should be accessible")
        }
        pub fn get_m_warp(&self) -> i32 {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMMSPWARPPOLICY__M_WARP
                .get(&__obj)
                .expect("non-layout field m_warp should be accessible")
        }
        pub fn get_n_warp(&self) -> i32 {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMMSPWARPPOLICY__N_WARP
                .get(&__obj)
                .expect("non-layout field n_warp should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.GemmSPPy"]
    pub struct GemmSPPyObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        E: crate::tir::Buffer,
        B: crate::tir::Buffer,
        C: crate::tir::Buffer,
        aRegion: crate::tir::BufferRegion,
        eRegion: crate::tir::BufferRegion,
        bRegion: crate::tir::BufferRegion,
        cRegion: crate::tir::BufferRegion,
        trans_A: bool,
        trans_B: bool,
        trans_E: bool,
        _gap0: [u8; 1],
        M: i32,
        N: i32,
        K: i32,
        stride_A: i32,
        stride_B: i32,
        offset_A: i32,
        offset_B: i32,
        clear_accum: crate::ir::PrimExpr,
        kPack: i32,
        wg_wait: i32,
        policy: crate::tl::GemmWarpPolicy,
        _gap1: [u8; 16],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct GemmSPPy {
        data: tvm_ffi::object::ObjectArc<GemmSPPyObj>,
    }

    tvm_ffi::impl_object_hierarchy!(GemmSPPy: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl GemmSPPy {
        pub fn get_E(&self) -> crate::tir::Buffer {
            self.data.E.clone()
        }
        pub fn get_B(&self) -> crate::tir::Buffer {
            self.data.B.clone()
        }
        pub fn get_C(&self) -> crate::tir::Buffer {
            self.data.C.clone()
        }
        pub fn get_aRegion(&self) -> crate::tir::BufferRegion {
            self.data.aRegion.clone()
        }
        pub fn get_eRegion(&self) -> crate::tir::BufferRegion {
            self.data.eRegion.clone()
        }
        pub fn get_bRegion(&self) -> crate::tir::BufferRegion {
            self.data.bRegion.clone()
        }
        pub fn get_cRegion(&self) -> crate::tir::BufferRegion {
            self.data.cRegion.clone()
        }
        pub fn get_trans_A(&self) -> bool {
            self.data.trans_A
        }
        pub fn get_trans_B(&self) -> bool {
            self.data.trans_B
        }
        pub fn get_trans_E(&self) -> bool {
            self.data.trans_E
        }
        pub fn get_M(&self) -> i32 {
            self.data.M
        }
        pub fn get_N(&self) -> i32 {
            self.data.N
        }
        pub fn get_K(&self) -> i32 {
            self.data.K
        }
        pub fn get_stride_A(&self) -> i32 {
            self.data.stride_A
        }
        pub fn get_stride_B(&self) -> i32 {
            self.data.stride_B
        }
        pub fn get_offset_A(&self) -> i32 {
            self.data.offset_A
        }
        pub fn get_offset_B(&self) -> i32 {
            self.data.offset_B
        }
        pub fn get_clear_accum(&self) -> crate::ir::PrimExpr {
            self.data.clear_accum.clone()
        }
        pub fn get_kPack(&self) -> i32 {
            self.data.kPack
        }
        pub fn get_wg_wait(&self) -> i32 {
            self.data.wg_wait
        }
        pub fn get_policy(&self) -> crate::tl::GemmWarpPolicy {
            self.data.policy.clone()
        }
    }

    static FIELD_TL_GEMMSPPY__A: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "A")
            .expect("non-layout field A must be registered in TVM reflection")
    });
    impl GemmSPPy {
        pub fn get_A(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_GEMMSPPY__A
                .get(&__obj)
                .expect("non-layout field A should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.ParallelOp"]
    pub struct ParallelOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        loop_layout: crate::tl::Fragment,
        _gap0: [u8; 8],
        predicate: Option<crate::ir::PrimExpr>,
        _gap1: [u8; 240],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ParallelOp {
        data: tvm_ffi::object::ObjectArc<ParallelOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ParallelOp: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl ParallelOp {
        pub fn get_loop_layout(&self) -> crate::tl::Fragment {
            self.data.loop_layout.clone()
        }
        pub fn get_predicate(&self) -> Option<crate::ir::PrimExpr> {
            self.data.predicate.clone()
        }
    }

    static FIELD_TL_PARALLELOP__ROOT: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::For>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ParallelOp", "root")
            .expect("non-layout field root must be registered in TVM reflection")
    });
    impl ParallelOp {
        pub fn get_root(&self) -> crate::tir::For {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_PARALLELOP__ROOT
                .get(&__obj)
                .expect("non-layout field root should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.ReduceOp"]
    pub struct ReduceOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        dst: crate::tir::Buffer,
        srcRegion: crate::tir::BufferRegion,
        dstRegion: crate::tir::BufferRegion,
        dim: i32,
        _gap0: [u8; 4],
        type_: crate::tl::ReduceType,
        clear: bool,
        _gap1: [u8; 15],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ReduceOp {
        data: tvm_ffi::object::ObjectArc<ReduceOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ReduceOp: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl ReduceOp {
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_srcRegion(&self) -> crate::tir::BufferRegion {
            self.data.srcRegion.clone()
        }
        pub fn get_dstRegion(&self) -> crate::tir::BufferRegion {
            self.data.dstRegion.clone()
        }
        pub fn get_dim(&self) -> i32 {
            self.data.dim
        }
        pub fn get_type_(&self) -> crate::tl::ReduceType {
            self.data.type_.clone()
        }
        pub fn get_clear(&self) -> bool {
            self.data.clear
        }
    }

    static FIELD_TL_REDUCEOP__SRC: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "src")
            .expect("non-layout field src must be registered in TVM reflection")
    });
    impl ReduceOp {
        pub fn get_src(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_REDUCEOP__SRC
                .get(&__obj)
                .expect("non-layout field src should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.CumSumOp"]
    pub struct CumSumOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        dst: crate::tir::Buffer,
        srcRegion: crate::tir::BufferRegion,
        dstRegion: crate::tir::BufferRegion,
        dim: i32,
        reverse: bool,
        _gap0: [u8; 11],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct CumSumOp {
        data: tvm_ffi::object::ObjectArc<CumSumOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(CumSumOp: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl CumSumOp {
        pub fn get_dst(&self) -> crate::tir::Buffer {
            self.data.dst.clone()
        }
        pub fn get_srcRegion(&self) -> crate::tir::BufferRegion {
            self.data.srcRegion.clone()
        }
        pub fn get_dstRegion(&self) -> crate::tir::BufferRegion {
            self.data.dstRegion.clone()
        }
        pub fn get_dim(&self) -> i32 {
            self.data.dim
        }
        pub fn get_reverse(&self) -> bool {
            self.data.reverse
        }
    }

    static FIELD_TL_CUMSUMOP__SRC: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "src")
            .expect("non-layout field src must be registered in TVM reflection")
    });
    impl CumSumOp {
        pub fn get_src(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_CUMSUMOP__SRC
                .get(&__obj)
                .expect("non-layout field src should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.ReduceType"]
    pub struct ReduceTypeObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        type_: i32,
        _gap0: [u8; 4],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ReduceType {
        data: tvm_ffi::object::ObjectArc<ReduceTypeObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ReduceType: tvm_ffi::object::ObjectRef);

    impl ReduceType {
        pub fn get_type_(&self) -> i32 {
            self.data.type_
        }
    }

    impl ReduceType {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tl.RegionOp"]
    pub struct RegionOpObj {
        __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::tl::TileOperatorObj,
        ranges: tvm_ffi::Array<crate::ir::Range>,
        access_mask: i32,
        _gap0: [u8; 12],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct RegionOp {
        data: tvm_ffi::object::ObjectArc<RegionOpObj>,
    }

    tvm_ffi::impl_object_hierarchy!(RegionOp: crate::tl::TileOperator, tvm_ffi::object::ObjectRef);

    impl RegionOp {
        pub fn get_ranges(&self) -> tvm_ffi::Array<crate::ir::Range> {
            self.data.ranges.clone()
        }
        pub fn get_access_mask(&self) -> i32 {
            self.data.access_mask
        }
    }

    static FIELD_TL_REGIONOP__BUFFER: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.RegionOp", "buffer")
            .expect("non-layout field buffer must be registered in TVM reflection")
    });
    impl RegionOp {
        pub fn get_buffer(&self) -> crate::tir::Buffer {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TL_REGIONOP__BUFFER
                .get(&__obj)
                .expect("non-layout field buffer should be accessible")
        }
    }

    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tl.transform.SimplifyConfig"]
        pub struct SimplifyConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            _gap0: [u8; 8],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct SimplifyConfig {
            data: tvm_ffi::object::ObjectArc<SimplifyConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(SimplifyConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl SimplifyConfig {}

        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tl.transform.SimplifyConfig", "transitively_prove_inequalities")
                .expect("non-layout field transitively_prove_inequalities must be registered in TVM reflection")
            });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tl.transform.SimplifyConfig", "propagate_knowns_to_prove_conditional")
                .expect("non-layout field propagate_knowns_to_prove_conditional must be registered in TVM reflection")
            });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tl.transform.SimplifyConfig", "propagate_knowns_to_simplify_expressions")
                .expect("non-layout field propagate_knowns_to_simplify_expressions must be registered in TVM reflection")
            });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tl.transform.SimplifyConfig", "convert_boolean_to_and_of_ors")
                .expect("non-layout field convert_boolean_to_and_of_ors must be registered in TVM reflection")
            });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES:
            std::sync::LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
            std::sync::LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("tl.transform.SimplifyConfig", "apply_constraints_to_boolean_branches")
                .expect("non-layout field apply_constraints_to_boolean_branches must be registered in TVM reflection")
            });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__ENABLE_SIMPLIFY_LET_INLINE: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "enable_simplify_let_inline",
            )
            .expect(
                "non-layout field enable_simplify_let_inline must be registered in TVM reflection",
            )
        });
        impl SimplifyConfig {
            pub fn get_transitively_prove_inequalities(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES
                    .get(&__obj)
                    .expect("non-layout field transitively_prove_inequalities should be accessible")
            }
            pub fn get_propagate_knowns_to_prove_conditional(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL.get(&__obj).expect("non-layout field propagate_knowns_to_prove_conditional should be accessible")
            }
            pub fn get_propagate_knowns_to_simplify_expressions(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS.get(&__obj).expect("non-layout field propagate_knowns_to_simplify_expressions should be accessible")
            }
            pub fn get_convert_boolean_to_and_of_ors(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS
                    .get(&__obj)
                    .expect("non-layout field convert_boolean_to_and_of_ors should be accessible")
            }
            pub fn get_apply_constraints_to_boolean_branches(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES.get(&__obj).expect("non-layout field apply_constraints_to_boolean_branches should be accessible")
            }
            pub fn get_enable_simplify_let_inline(&self) -> bool {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__ENABLE_SIMPLIFY_LET_INLINE
                    .get(&__obj)
                    .expect("non-layout field enable_simplify_let_inline should be accessible")
            }
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "tl.transform.UnrollLoopConfig"]
        pub struct UnrollLoopConfigObj {
            __tvm_ffi_object_parent: crate::_tvm_ffi_stubgen_detail::types::ir::AttrsObj,
            auto_max_extent: i32,
            explicit_unroll: i32,
            unroll_local_access: i32,
            _gap0: [u8; 12],
        }

        #[repr(C)]
        #[derive(tvm_ffi::derive::ObjectRef, Clone)]
        pub struct UnrollLoopConfig {
            data: tvm_ffi::object::ObjectArc<UnrollLoopConfigObj>,
        }

        tvm_ffi::impl_object_hierarchy!(UnrollLoopConfig: crate::ir::Attrs, tvm_ffi::object::ObjectRef);

        impl UnrollLoopConfig {
            pub fn get_auto_max_extent(&self) -> i32 {
                self.data.auto_max_extent
            }
            pub fn get_explicit_unroll(&self) -> i32 {
                self.data.explicit_unroll
            }
            pub fn get_unroll_local_access(&self) -> i32 {
                self.data.unroll_local_access
            }
        }

        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "auto_max_step",
            )
            .expect("non-layout field auto_max_step must be registered in TVM reflection")
        });
        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH: std::sync::LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i32>,
        > = std::sync::LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "auto_max_depth",
            )
            .expect("non-layout field auto_max_depth must be registered in TVM reflection")
        });
        impl UnrollLoopConfig {
            pub fn get_auto_max_step(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP
                    .get(&__obj)
                    .expect("non-layout field auto_max_step should be accessible")
            }
            pub fn get_auto_max_depth(&self) -> i32 {
                let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH
                    .get(&__obj)
                    .expect("non-layout field auto_max_depth should be accessible")
            }
        }
    }
}
pub mod transform {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "transform.PassContext"]
    pub struct PassContextObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        opt_level: i32,
        _gap0: [u8; 4],
        required_pass: tvm_ffi::Array<tvm_ffi::String>,
        disabled_pass: tvm_ffi::Array<tvm_ffi::String>,
        _gap1: [u8; 8],
        config: tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
        instruments: tvm_ffi::Array<crate::instrument::PassInstrument>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PassContext {
        data: tvm_ffi::object::ObjectArc<PassContextObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PassContext: tvm_ffi::object::ObjectRef);

    impl PassContext {
        pub fn get_opt_level(&self) -> i32 {
            self.data.opt_level
        }
        pub fn get_required_pass(&self) -> tvm_ffi::Array<tvm_ffi::String> {
            self.data.required_pass.clone()
        }
        pub fn get_disabled_pass(&self) -> tvm_ffi::Array<tvm_ffi::String> {
            self.data.disabled_pass.clone()
        }
        pub fn get_config(&self) -> tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue> {
            self.data.config.clone()
        }
        pub fn get_instruments(&self) -> tvm_ffi::Array<crate::instrument::PassInstrument> {
            self.data.instruments.clone()
        }
    }

    static FIELD_TRANSFORM_PASSCONTEXT__DIAG_CTX: std::sync::LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = std::sync::LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "diag_ctx")
            .expect("non-layout field diag_ctx must be registered in TVM reflection")
    });
    impl PassContext {
        pub fn get_diag_ctx(&self) -> tvm_ffi::Any {
            let __obj: tvm_ffi::object::ObjectRef = self.clone().into();
            FIELD_TRANSFORM_PASSCONTEXT__DIAG_CTX
                .get_any(&__obj)
                .expect("non-layout field diag_ctx should be accessible")
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "transform.PassInfo"]
    pub struct PassInfoObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        opt_level: i32,
        _gap0: [u8; 4],
        name: tvm_ffi::String,
        traceable: bool,
        _gap1: [u8; 7],
        required: tvm_ffi::Array<tvm_ffi::String>,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct PassInfo {
        data: tvm_ffi::object::ObjectArc<PassInfoObj>,
    }

    tvm_ffi::impl_object_hierarchy!(PassInfo: tvm_ffi::object::ObjectRef);

    impl PassInfo {
        pub fn get_opt_level(&self) -> i32 {
            self.data.opt_level
        }
        pub fn get_name(&self) -> tvm_ffi::String {
            self.data.name.clone()
        }
        pub fn get_traceable(&self) -> bool {
            self.data.traceable
        }
        pub fn get_required(&self) -> tvm_ffi::Array<tvm_ffi::String> {
            self.data.required.clone()
        }
    }

    impl PassInfo {}

    tvm_ffi::define_object_wrapper!(Pass, "transform.Pass");

    impl Pass {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "transform.Sequential"]
    pub struct SequentialObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        pass_info: crate::transform::PassInfo,
        passes: tvm_ffi::Array<crate::transform::Pass>,
        _gap0: [u8; 8],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct Sequential {
        data: tvm_ffi::object::ObjectArc<SequentialObj>,
    }

    tvm_ffi::impl_object_hierarchy!(Sequential: tvm_ffi::object::ObjectRef);

    impl Sequential {
        pub fn get_pass_info(&self) -> crate::transform::PassInfo {
            self.data.pass_info.clone()
        }
        pub fn get_passes(&self) -> tvm_ffi::Array<crate::transform::Pass> {
            self.data.passes.clone()
        }
    }

    impl Sequential {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "transform.ModulePass"]
    pub struct ModulePassObj {
        __tvm_ffi_object_parent: tvm_ffi::object::Object,
        pass_info: crate::transform::PassInfo,
        _gap0: [u8; 40],
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct ModulePass {
        data: tvm_ffi::object::ObjectArc<ModulePassObj>,
    }

    tvm_ffi::impl_object_hierarchy!(ModulePass: tvm_ffi::object::ObjectRef);

    impl ModulePass {
        pub fn get_pass_info(&self) -> crate::transform::PassInfo {
            self.data.pass_info.clone()
        }
    }

    impl ModulePass {}
}
