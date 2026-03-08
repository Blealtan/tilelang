#![allow(unused_imports)]
#![allow(non_snake_case, nonstandard_style)]

use std::sync::LazyLock;
use tvm_ffi::{Any, AnyView, ObjectArc, Result};

tvm_ffi::define_object_wrapper!(ObjectRValueRef, "ObjectRValueRef");

impl ObjectRValueRef {}

pub mod ffi {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    pub mod reflection {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        tvm_ffi::define_object_wrapper!(AccessPath, "ffi.reflection.AccessPath");

        static FIELD_FFI_REFLECTION_ACCESSPATH__PARENT: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::object::ObjectRef>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ffi.reflection.AccessPath", "parent")
                .expect("missing field")
        });
        static FIELD_FFI_REFLECTION_ACCESSPATH__STEP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ffi.reflection.AccessPath", "step")
                .expect("missing field")
        });
        static FIELD_FFI_REFLECTION_ACCESSPATH__DEPTH: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ffi.reflection.AccessPath", "depth")
                .expect("missing field")
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
            pub fn parent(&self) -> Result<Option<tvm_ffi::object::ObjectRef>> {
                FIELD_FFI_REFLECTION_ACCESSPATH__PARENT.get(self.as_object_ref())
            }
            pub fn step(&self) -> Result<tvm_ffi::Any> {
                FIELD_FFI_REFLECTION_ACCESSPATH__STEP.get_any(self.as_object_ref())
            }
            pub fn depth(&self) -> Result<i64> {
                FIELD_FFI_REFLECTION_ACCESSPATH__DEPTH.get(self.as_object_ref())
            }
            pub fn _root() -> Result<crate::ffi::reflection::AccessPath> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ROOT;
                let typed = tvm_ffi::into_typed_fn!(func.clone(), Fn() -> Result<crate::ffi::reflection::AccessPath>);
                typed()
            }
            pub fn _extend(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___EXTEND;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _attr(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ATTR;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _array_item(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ARRAY_ITEM;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _map_item(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___MAP_ITEM;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _attr_missing(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ATTR_MISSING;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _array_item_missing(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___ARRAY_ITEM_MISSING;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _map_item_missing(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___MAP_ITEM_MISSING;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _is_prefix_of(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___IS_PREFIX_OF;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _to_steps(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___TO_STEPS;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
            pub fn _path_equal(&self, args: &[Any]) -> Result<Any> {
                let func = &*METHOD_FFI_REFLECTION_ACCESSPATH___PATH_EQUAL;
                let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
                views.push(AnyView::from(self.as_object_ref()));
                views.extend(args.iter().map(AnyView::from));
                func.call_packed(&views)
            }
        }
    }
}
pub mod ir {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.BaseExpr"]
    pub struct BaseExprObj {
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(PrimExpr, "ir.PrimExpr");

    static FIELD_IR_PRIMEXPR__DTYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.PrimExpr", "dtype").expect("missing field")
    });
    impl PrimExpr {
        pub fn dtype(&self) -> Result<tvm_ffi::DLDataType> {
            FIELD_IR_PRIMEXPR__DTYPE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(IntImm, "ir.IntImm");

    static FIELD_IR_INTIMM__VALUE: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ir.IntImm", "value").expect("missing field")
        });
    impl IntImm {
        pub fn value(&self) -> Result<i64> {
            FIELD_IR_INTIMM__VALUE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(FloatImm, "ir.FloatImm");

    static FIELD_IR_FLOATIMM__VALUE: LazyLock<tvm_ffi::object_wrapper::FieldGetter<f64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ir.FloatImm", "value")
                .expect("missing field")
        });
    impl FloatImm {
        pub fn value(&self) -> Result<f64> {
            FIELD_IR_FLOATIMM__VALUE.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.RelaxExpr"]
    pub struct RelaxExprObj {
        parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseExprObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
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

    tvm_ffi::define_object_wrapper!(Op, "ir.Op");

    static FIELD_IR_OP__NAME: LazyLock<tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "name").expect("missing field")
        });
    static FIELD_IR_OP__OP_TYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::FuncType>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "op_type").expect("missing field")
    });
    static FIELD_IR_OP__DESCRIPTION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "description").expect("missing field")
    });
    static FIELD_IR_OP__ARGUMENTS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::AttrFieldInfo>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "arguments").expect("missing field")
    });
    static FIELD_IR_OP__ATTRS_TYPE_KEY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "attrs_type_key").expect("missing field")
    });
    static FIELD_IR_OP__NUM_INPUTS: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "num_inputs").expect("missing field")
        });
    static FIELD_IR_OP__SUPPORT_LEVEL: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ir.Op", "support_level")
                .expect("missing field")
        });
    impl Op {
        pub fn name(&self) -> Result<tvm_ffi::String> {
            FIELD_IR_OP__NAME.get(self.as_object_ref())
        }
        pub fn op_type(&self) -> Result<crate::ir::FuncType> {
            FIELD_IR_OP__OP_TYPE.get(self.as_object_ref())
        }
        pub fn description(&self) -> Result<tvm_ffi::String> {
            FIELD_IR_OP__DESCRIPTION.get(self.as_object_ref())
        }
        pub fn arguments(&self) -> Result<tvm_ffi::Array<crate::ir::AttrFieldInfo>> {
            FIELD_IR_OP__ARGUMENTS.get(self.as_object_ref())
        }
        pub fn attrs_type_key(&self) -> Result<tvm_ffi::String> {
            FIELD_IR_OP__ATTRS_TYPE_KEY.get(self.as_object_ref())
        }
        pub fn num_inputs(&self) -> Result<i64> {
            FIELD_IR_OP__NUM_INPUTS.get(self.as_object_ref())
        }
        pub fn support_level(&self) -> Result<i64> {
            FIELD_IR_OP__SUPPORT_LEVEL.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(IntSet, "ir.IntSet");

    impl IntSet {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.Type"]
    pub struct TypeObj {
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(PrimType, "ir.PrimType");

    static FIELD_IR_PRIMTYPE__DTYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.PrimType", "dtype").expect("missing field")
    });
    impl PrimType {
        pub fn dtype(&self) -> Result<tvm_ffi::DLDataType> {
            FIELD_IR_PRIMTYPE__DTYPE.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.PointerType"]
    pub struct PointerTypeObj {
        parent: crate::_tvm_ffi_stubgen_detail::types::ir::TypeObj,
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

    tvm_ffi::define_object_wrapper!(TupleType, "ir.TupleType");

    static FIELD_IR_TUPLETYPE__FIELDS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Type>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.TupleType", "fields").expect("missing field")
    });
    static FIELD_IR_TUPLETYPE__SPAN: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Span>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.TupleType", "span").expect("missing field")
    });
    impl TupleType {
        pub fn fields(&self) -> Result<tvm_ffi::Array<crate::ir::Type>> {
            FIELD_IR_TUPLETYPE__FIELDS.get(self.as_object_ref())
        }
        pub fn span(&self) -> Result<crate::ir::Span> {
            FIELD_IR_TUPLETYPE__SPAN.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(FuncType, "ir.FuncType");

    static FIELD_IR_FUNCTYPE__ARG_TYPES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Type>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.FuncType", "arg_types")
            .expect("missing field")
    });
    static FIELD_IR_FUNCTYPE__RET_TYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Type>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.FuncType", "ret_type").expect("missing field")
    });
    static FIELD_IR_FUNCTYPE__SPAN: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Span>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.FuncType", "span").expect("missing field")
    });
    impl FuncType {
        pub fn arg_types(&self) -> Result<tvm_ffi::Array<crate::ir::Type>> {
            FIELD_IR_FUNCTYPE__ARG_TYPES.get(self.as_object_ref())
        }
        pub fn ret_type(&self) -> Result<crate::ir::Type> {
            FIELD_IR_FUNCTYPE__RET_TYPE.get(self.as_object_ref())
        }
        pub fn span(&self) -> Result<crate::ir::Span> {
            FIELD_IR_FUNCTYPE__SPAN.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(TensorMapType, "ir.TensorMapType");

    static FIELD_IR_TENSORMAPTYPE__SPAN: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Span>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.TensorMapType", "span")
            .expect("missing field")
    });
    impl TensorMapType {
        pub fn span(&self) -> Result<crate::ir::Span> {
            FIELD_IR_TENSORMAPTYPE__SPAN.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Attrs, "ir.Attrs");

    impl Attrs {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.AttrFieldInfo"]
    pub struct AttrFieldInfoObj {
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(DictAttrs, "ir.DictAttrs");

    static FIELD_IR_DICTATTRS____DICT__: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.DictAttrs", "__dict__")
            .expect("missing field")
    });
    impl DictAttrs {
        pub fn __dict__(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
            FIELD_IR_DICTATTRS____DICT__.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.EnvFunc"]
    pub struct EnvFuncObj {
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(VDevice, "ir.VDevice");

    static FIELD_IR_VDEVICE__TARGET: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::target::Target>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.VDevice", "target").expect("missing field")
    });
    static FIELD_IR_VDEVICE__VDEVICE_ID: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("ir.VDevice", "vdevice_id")
                .expect("missing field")
        });
    static FIELD_IR_VDEVICE__MEMORY_SCOPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.VDevice", "memory_scope")
            .expect("missing field")
    });
    impl VDevice {
        pub fn target(&self) -> Result<crate::target::Target> {
            FIELD_IR_VDEVICE__TARGET.get(self.as_object_ref())
        }
        pub fn vdevice_id(&self) -> Result<i64> {
            FIELD_IR_VDEVICE__VDEVICE_ID.get(self.as_object_ref())
        }
        pub fn memory_scope(&self) -> Result<tvm_ffi::String> {
            FIELD_IR_VDEVICE__MEMORY_SCOPE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(DummyGlobalInfo, "ir.DummyGlobalInfo");

    impl DummyGlobalInfo {}

    tvm_ffi::define_object_wrapper!(GlobalVarSupply, "ir.GlobalVarSupply");

    impl GlobalVarSupply {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.IRModule"]
    pub struct IRModuleObj {
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(NameSupply, "ir.NameSupply");

    impl NameSupply {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.SourceName"]
    pub struct SourceNameObj {
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::ir::SpanObj,
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

    tvm_ffi::define_object_wrapper!(Source, "ir.Source");

    static FIELD_IR_SOURCE__SOURCE_NAME: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::SourceName>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.Source", "source_name")
            .expect("missing field")
    });
    static FIELD_IR_SOURCE__SOURCE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("ir.Source", "source").expect("missing field")
    });
    impl Source {
        pub fn source_name(&self) -> Result<crate::ir::SourceName> {
            FIELD_IR_SOURCE__SOURCE_NAME.get(self.as_object_ref())
        }
        pub fn source(&self) -> Result<tvm_ffi::String> {
            FIELD_IR_SOURCE__SOURCE.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "ir.SourceMap"]
    pub struct SourceMapObj {
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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
pub mod relax {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    tvm_ffi::define_object_wrapper!(DTensorStructInfo, "relax.DTensorStructInfo");

    static FIELD_RELAX_DTENSORSTRUCTINFO__DEVICE_MESH: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.DTensorStructInfo", "device_mesh")
            .expect("missing field")
    });
    static FIELD_RELAX_DTENSORSTRUCTINFO__PLACEMENT: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.DTensorStructInfo", "placement")
            .expect("missing field")
    });
    static FIELD_RELAX_DTENSORSTRUCTINFO__TENSOR_SINFO: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("relax.DTensorStructInfo", "tensor_sinfo")
            .expect("missing field")
    });
    impl DTensorStructInfo {
        pub fn device_mesh(&self) -> Result<tvm_ffi::Any> {
            FIELD_RELAX_DTENSORSTRUCTINFO__DEVICE_MESH.get_any(self.as_object_ref())
        }
        pub fn placement(&self) -> Result<tvm_ffi::Any> {
            FIELD_RELAX_DTENSORSTRUCTINFO__PLACEMENT.get_any(self.as_object_ref())
        }
        pub fn tensor_sinfo(&self) -> Result<tvm_ffi::Any> {
            FIELD_RELAX_DTENSORSTRUCTINFO__TENSOR_SINFO.get_any(self.as_object_ref())
        }
    }

    pub mod expr {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "relax.expr.Tuple"]
        pub struct TupleObj {
            parent: crate::_tvm_ffi_stubgen_detail::types::ir::RelaxExprObj,
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

        tvm_ffi::define_object_wrapper!(Var, "relax.expr.Var");

        static FIELD_RELAX_EXPR_VAR__VID: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.expr.Var", "vid")
                .expect("missing field")
        });
        impl Var {
            pub fn vid(&self) -> Result<tvm_ffi::Any> {
                FIELD_RELAX_EXPR_VAR__VID.get_any(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(VarBinding, "relax.expr.VarBinding");

        static FIELD_RELAX_EXPR_VARBINDING__VALUE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::ir::RelaxExpr>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("relax.expr.VarBinding", "value")
                .expect("missing field")
        });
        impl VarBinding {
            pub fn value(&self) -> Result<crate::ir::RelaxExpr> {
                FIELD_RELAX_EXPR_VARBINDING__VALUE.get(self.as_object_ref())
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
        parent: tvm_ffi::object::Object,
        binding_names: tvm_ffi::Array<tvm_ffi::String>,
        show_meta: bool,
        ir_prefix: tvm_ffi::String,
        tir_prefix: tvm_ffi::String,
        relax_prefix: tvm_ffi::String,
        module_alias: tvm_ffi::String,
        buffer_dtype: tvm_ffi::DLDataType,
        int_dtype: tvm_ffi::DLDataType,
        float_dtype: tvm_ffi::DLDataType,
        verbose_expr: bool,
        indent_spaces: i32,
        print_line_numbers: bool,
        num_context_lines: i32,
        syntax_sugar: bool,
        show_object_address: bool,
        show_all_struct_info: bool,
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

        tvm_ffi::define_object_wrapper!(IRBuilderFrame, "script.ir_builder.IRBuilderFrame");

        impl IRBuilderFrame {}

        #[repr(C)]
        #[derive(tvm_ffi::derive::Object)]
        #[type_key = "script.ir_builder.IRBuilder"]
        pub struct IRBuilderObj {
            parent: tvm_ffi::object::Object,
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

        tvm_ffi::define_object_wrapper!(IRModuleFrame, "script.ir_builder.IRModuleFrame");

        static FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__GLOBAL_VARS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<
                tvm_ffi::Map<tvm_ffi::String, crate::ir::GlobalVar>,
            >,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.ir_builder.IRModuleFrame",
                "global_vars",
            )
            .expect("missing field")
        });
        static FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__FUNCTIONS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<
                tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>,
            >,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.ir_builder.IRModuleFrame",
                "functions",
            )
            .expect("missing field")
        });
        static FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__ATTRS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.IRModuleFrame", "attrs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__GLOBAL_INFOS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<
                tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>,
            >,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.ir_builder.IRModuleFrame",
                "global_infos",
            )
            .expect("missing field")
        });
        impl IRModuleFrame {
            pub fn global_vars(
                &self,
            ) -> Result<tvm_ffi::Map<tvm_ffi::String, crate::ir::GlobalVar>> {
                FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__GLOBAL_VARS.get(self.as_object_ref())
            }
            pub fn functions(
                &self,
            ) -> Result<tvm_ffi::Map<crate::ir::GlobalVar, crate::ir::BaseFunc>> {
                FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__FUNCTIONS.get(self.as_object_ref())
            }
            pub fn attrs(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
                FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__ATTRS.get(self.as_object_ref())
            }
            pub fn global_infos(
                &self,
            ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::Array<crate::ir::GlobalInfo>>>
            {
                FIELD_SCRIPT_IR_BUILDER_IRMODULEFRAME__GLOBAL_INFOS.get(self.as_object_ref())
            }
        }

        pub mod relax {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, ObjectArc, Result};

            tvm_ffi::define_object_wrapper!(RelaxFrame, "script.ir_builder.relax.RelaxFrame");

            impl RelaxFrame {}

            tvm_ffi::define_object_wrapper!(SeqExprFrame, "script.ir_builder.relax.SeqExprFrame");

            static FIELD_SCRIPT_IR_BUILDER_RELAX_SEQEXPRFRAME__BINDING_BLOCKS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.SeqExprFrame",
                    "binding_blocks",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_SEQEXPRFRAME__OUTPUT: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::RelaxExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.SeqExprFrame",
                    "output",
                )
                .expect("missing field")
            });
            impl SeqExprFrame {
                pub fn binding_blocks(&self) -> Result<tvm_ffi::Any> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_SEQEXPRFRAME__BINDING_BLOCKS
                        .get_any(self.as_object_ref())
                }
                pub fn output(&self) -> Result<Option<crate::ir::RelaxExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_SEQEXPRFRAME__OUTPUT.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(FunctionFrame, "script.ir_builder.relax.FunctionFrame");

            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__NAME: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "name",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__PARAMS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::relax::expr::Var>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "params",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__RET_STRUCT_INFO: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::StructInfo>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "ret_struct_info",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__IS_PURE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::IntImm>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "is_pure",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__ATTRS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "attrs",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__BINDING_BLOCKS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "binding_blocks",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__OUTPUT: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::RelaxExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.FunctionFrame",
                    "output",
                )
                .expect("missing field")
            });
            impl FunctionFrame {
                pub fn name(&self) -> Result<Option<tvm_ffi::String>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__NAME.get(self.as_object_ref())
                }
                pub fn params(&self) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__PARAMS.get(self.as_object_ref())
                }
                pub fn ret_struct_info(&self) -> Result<Option<crate::ir::StructInfo>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__RET_STRUCT_INFO
                        .get(self.as_object_ref())
                }
                pub fn is_pure(&self) -> Result<Option<crate::ir::IntImm>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__IS_PURE.get(self.as_object_ref())
                }
                pub fn attrs(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__ATTRS.get(self.as_object_ref())
                }
                pub fn binding_blocks(&self) -> Result<tvm_ffi::Any> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__BINDING_BLOCKS
                        .get_any(self.as_object_ref())
                }
                pub fn output(&self) -> Result<Option<crate::ir::RelaxExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_FUNCTIONFRAME__OUTPUT.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(BlockFrame, "script.ir_builder.relax.BlockFrame");

            static FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__IS_DATAFLOW: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<bool>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.BlockFrame",
                    "is_dataflow",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__EMITTED_VARS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::relax::expr::Var>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.BlockFrame",
                    "emitted_vars",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__OUTPUT_VARS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::relax::expr::Var>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.BlockFrame",
                    "output_vars",
                )
                .expect("missing field")
            });
            impl BlockFrame {
                pub fn is_dataflow(&self) -> Result<bool> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__IS_DATAFLOW.get(self.as_object_ref())
                }
                pub fn emitted_vars(&self) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__EMITTED_VARS.get(self.as_object_ref())
                }
                pub fn output_vars(&self) -> Result<tvm_ffi::Array<crate::relax::expr::Var>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_BLOCKFRAME__OUTPUT_VARS.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(IfFrame, "script.ir_builder.relax.IfFrame");

            static FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__CONDITION: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::RelaxExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.IfFrame",
                    "condition",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__THEN_EXPR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::RelaxExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.IfFrame",
                    "then_expr",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__ELSE_EXPR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::RelaxExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.IfFrame",
                    "else_expr",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__VAR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::relax::expr::Var>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.relax.IfFrame", "var")
                    .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__VAR_NAME: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.relax.IfFrame",
                    "var_name",
                )
                .expect("missing field")
            });
            impl IfFrame {
                pub fn condition(&self) -> Result<crate::ir::RelaxExpr> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__CONDITION.get(self.as_object_ref())
                }
                pub fn then_expr(&self) -> Result<Option<crate::ir::RelaxExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__THEN_EXPR.get(self.as_object_ref())
                }
                pub fn else_expr(&self) -> Result<Option<crate::ir::RelaxExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__ELSE_EXPR.get(self.as_object_ref())
                }
                pub fn var(&self) -> Result<crate::relax::expr::Var> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__VAR.get(self.as_object_ref())
                }
                pub fn var_name(&self) -> Result<tvm_ffi::String> {
                    FIELD_SCRIPT_IR_BUILDER_RELAX_IFFRAME__VAR_NAME.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(ThenFrame, "script.ir_builder.relax.ThenFrame");

            impl ThenFrame {}

            tvm_ffi::define_object_wrapper!(ElseFrame, "script.ir_builder.relax.ElseFrame");

            impl ElseFrame {}
        }
        pub mod tir {
            use std::sync::LazyLock;
            use tvm_ffi::{Any, AnyView, ObjectArc, Result};

            tvm_ffi::define_object_wrapper!(TIRFrame, "script.ir_builder.tir.TIRFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_TIRFRAME__STMTS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Stmt>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.TIRFrame", "stmts")
                    .expect("missing field")
            });
            impl TIRFrame {
                pub fn stmts(&self) -> Result<tvm_ffi::Array<crate::tir::Stmt>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_TIRFRAME__STMTS.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(PrimFuncFrame, "script.ir_builder.tir.PrimFuncFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__NAME: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "name",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ARGS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Var>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "args",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__IS_PRIVATE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<bool>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "is_private",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__RET_TYPE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::Type>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "ret_type",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__BUFFER_MAP: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "buffer_map",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ATTRS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "attrs",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ENV_THREADS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Map<crate::tir::Var, crate::tir::IterVar>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "env_threads",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ROOT_ALLOC_BUFFERS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Buffer>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.PrimFuncFrame",
                    "root_alloc_buffers",
                )
                .expect("missing field")
            });
            impl PrimFuncFrame {
                pub fn name(&self) -> Result<Option<tvm_ffi::String>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__NAME.get(self.as_object_ref())
                }
                pub fn args(&self) -> Result<tvm_ffi::Array<crate::tir::Var>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ARGS.get(self.as_object_ref())
                }
                pub fn is_private(&self) -> Result<bool> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__IS_PRIVATE.get(self.as_object_ref())
                }
                pub fn ret_type(&self) -> Result<Option<crate::ir::Type>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__RET_TYPE.get(self.as_object_ref())
                }
                pub fn buffer_map(
                    &self,
                ) -> Result<tvm_ffi::Map<crate::tir::Var, crate::tir::Buffer>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__BUFFER_MAP.get(self.as_object_ref())
                }
                pub fn attrs(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ATTRS.get(self.as_object_ref())
                }
                pub fn env_threads(
                    &self,
                ) -> Result<tvm_ffi::Map<crate::tir::Var, crate::tir::IterVar>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ENV_THREADS.get(self.as_object_ref())
                }
                pub fn root_alloc_buffers(&self) -> Result<tvm_ffi::Array<crate::tir::Buffer>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_PRIMFUNCFRAME__ROOT_ALLOC_BUFFERS
                        .get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(BlockFrame, "script.ir_builder.tir.BlockFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__NAME: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "name",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ITER_VARS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::IterVar>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "iter_vars",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__READS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    Option<tvm_ffi::Array<crate::tir::BufferRegion>>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "reads",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__WRITES: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    Option<tvm_ffi::Array<crate::tir::BufferRegion>>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "writes",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__INIT: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::tir::Stmt>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "init",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ALLOC_BUFFERS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Buffer>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "alloc_buffers",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__MATCH_BUFFERS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::MatchBufferRegion>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "match_buffers",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ANNOTATIONS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "annotations",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ITER_VALUES: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "iter_values",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__PREDICATE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::PrimExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "predicate",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__NO_REALIZE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<bool>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.BlockFrame",
                    "no_realize",
                )
                .expect("missing field")
            });
            impl BlockFrame {
                pub fn name(&self) -> Result<tvm_ffi::String> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__NAME.get(self.as_object_ref())
                }
                pub fn iter_vars(&self) -> Result<tvm_ffi::Array<crate::tir::IterVar>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ITER_VARS.get(self.as_object_ref())
                }
                pub fn reads(&self) -> Result<Option<tvm_ffi::Array<crate::tir::BufferRegion>>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__READS.get(self.as_object_ref())
                }
                pub fn writes(&self) -> Result<Option<tvm_ffi::Array<crate::tir::BufferRegion>>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__WRITES.get(self.as_object_ref())
                }
                pub fn init(&self) -> Result<Option<crate::tir::Stmt>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__INIT.get(self.as_object_ref())
                }
                pub fn alloc_buffers(&self) -> Result<tvm_ffi::Array<crate::tir::Buffer>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ALLOC_BUFFERS.get(self.as_object_ref())
                }
                pub fn match_buffers(
                    &self,
                ) -> Result<tvm_ffi::Array<crate::tir::MatchBufferRegion>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__MATCH_BUFFERS.get(self.as_object_ref())
                }
                pub fn annotations(
                    &self,
                ) -> Result<Option<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>>
                {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ANNOTATIONS.get(self.as_object_ref())
                }
                pub fn iter_values(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__ITER_VALUES.get(self.as_object_ref())
                }
                pub fn predicate(&self) -> Result<Option<crate::ir::PrimExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__PREDICATE.get(self.as_object_ref())
                }
                pub fn no_realize(&self) -> Result<bool> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_BLOCKFRAME__NO_REALIZE.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(BlockInitFrame, "script.ir_builder.tir.BlockInitFrame");

            impl BlockInitFrame {}

            tvm_ffi::define_object_wrapper!(ForFrame, "script.ir_builder.tir.ForFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_FORFRAME__VARS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::Var>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.ForFrame", "vars")
                    .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_FORFRAME__DOMS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.ForFrame", "doms")
                    .expect("missing field")
            });
            impl ForFrame {
                pub fn vars(&self) -> Result<tvm_ffi::Array<crate::tir::Var>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_FORFRAME__VARS.get(self.as_object_ref())
                }
                pub fn doms(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_FORFRAME__DOMS.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(AssertFrame, "script.ir_builder.tir.AssertFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_ASSERTFRAME__CONDITION: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AssertFrame",
                    "condition",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ASSERTFRAME__MESSAGE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AssertFrame",
                    "message",
                )
                .expect("missing field")
            });
            impl AssertFrame {
                pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ASSERTFRAME__CONDITION.get(self.as_object_ref())
                }
                pub fn message(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ASSERTFRAME__MESSAGE.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(LetFrame, "script.ir_builder.tir.LetFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_LETFRAME__VAR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::Var>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.LetFrame", "var")
                    .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_LETFRAME__VALUE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.LetFrame", "value")
                    .expect("missing field")
            });
            impl LetFrame {
                pub fn var(&self) -> Result<crate::tir::Var> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_LETFRAME__VAR.get(self.as_object_ref())
                }
                pub fn value(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_LETFRAME__VALUE.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(
                LaunchThreadFrame,
                "script.ir_builder.tir.LaunchThreadFrame"
            );

            static FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__EXTENT: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.LaunchThreadFrame",
                    "extent",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__ATTR_KEY: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.LaunchThreadFrame",
                    "attr_key",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__ITER_VAR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::IterVar>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.LaunchThreadFrame",
                    "iter_var",
                )
                .expect("missing field")
            });
            impl LaunchThreadFrame {
                pub fn extent(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__EXTENT.get(self.as_object_ref())
                }
                pub fn attr_key(&self) -> Result<tvm_ffi::String> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__ATTR_KEY
                        .get(self.as_object_ref())
                }
                pub fn iter_var(&self) -> Result<crate::tir::IterVar> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_LAUNCHTHREADFRAME__ITER_VAR
                        .get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(RealizeFrame, "script.ir_builder.tir.RealizeFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__BUFFER_SLICE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.RealizeFrame",
                    "buffer_slice",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__STORAGE_SCOPE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.RealizeFrame",
                    "storage_scope",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__CONDITION: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.RealizeFrame",
                    "condition",
                )
                .expect("missing field")
            });
            impl RealizeFrame {
                pub fn buffer_slice(&self) -> Result<crate::tir::BufferRegion> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__BUFFER_SLICE.get(self.as_object_ref())
                }
                pub fn storage_scope(&self) -> Result<tvm_ffi::String> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__STORAGE_SCOPE
                        .get(self.as_object_ref())
                }
                pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_REALIZEFRAME__CONDITION.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(AllocateFrame, "script.ir_builder.tir.AllocateFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__EXTENTS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "extents",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__DTYPE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "dtype",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__STORAGE_SCOPE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "storage_scope",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__CONDITION: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "condition",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__ANNOTATIONS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "annotations",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__BUFFER_VAR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::Var>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateFrame",
                    "buffer_var",
                )
                .expect("missing field")
            });
            impl AllocateFrame {
                pub fn extents(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__EXTENTS.get(self.as_object_ref())
                }
                pub fn dtype(&self) -> Result<tvm_ffi::DLDataType> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__DTYPE.get(self.as_object_ref())
                }
                pub fn storage_scope(&self) -> Result<tvm_ffi::String> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__STORAGE_SCOPE
                        .get(self.as_object_ref())
                }
                pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__CONDITION.get(self.as_object_ref())
                }
                pub fn annotations(
                    &self,
                ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__ANNOTATIONS.get(self.as_object_ref())
                }
                pub fn buffer_var(&self) -> Result<crate::tir::Var> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATEFRAME__BUFFER_VAR.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(
                AllocateConstFrame,
                "script.ir_builder.tir.AllocateConstFrame"
            );

            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__DTYPE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::DLDataType>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateConstFrame",
                    "dtype",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__EXTENTS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateConstFrame",
                    "extents",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__DATA: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Tensor>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateConstFrame",
                    "data",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__BUFFER_VAR: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::Var>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateConstFrame",
                    "buffer_var",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__ANNOTATIONS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<
                    tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>,
                >,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AllocateConstFrame",
                    "annotations",
                )
                .expect("missing field")
            });
            impl AllocateConstFrame {
                pub fn dtype(&self) -> Result<tvm_ffi::DLDataType> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__DTYPE.get(self.as_object_ref())
                }
                pub fn extents(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__EXTENTS
                        .get(self.as_object_ref())
                }
                pub fn data(&self) -> Result<tvm_ffi::Tensor> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__DATA.get(self.as_object_ref())
                }
                pub fn buffer_var(&self) -> Result<crate::tir::Var> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__BUFFER_VAR
                        .get(self.as_object_ref())
                }
                pub fn annotations(
                    &self,
                ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ALLOCATECONSTFRAME__ANNOTATIONS
                        .get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(AttrFrame, "script.ir_builder.tir.AttrFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__NODE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::AnyValue>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new("script.ir_builder.tir.AttrFrame", "node")
                    .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__ATTR_KEY: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AttrFrame",
                    "attr_key",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__VALUE: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.AttrFrame",
                    "value",
                )
                .expect("missing field")
            });
            impl AttrFrame {
                pub fn node(&self) -> Result<tvm_ffi::AnyValue> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__NODE.get(self.as_object_ref())
                }
                pub fn attr_key(&self) -> Result<tvm_ffi::String> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__ATTR_KEY.get(self.as_object_ref())
                }
                pub fn value(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_ATTRFRAME__VALUE.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(WhileFrame, "script.ir_builder.tir.WhileFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_WHILEFRAME__CONDITION: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.WhileFrame",
                    "condition",
                )
                .expect("missing field")
            });
            impl WhileFrame {
                pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_WHILEFRAME__CONDITION.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(IfFrame, "script.ir_builder.tir.IfFrame");

            static FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__CONDITION: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.IfFrame",
                    "condition",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__THEN_STMTS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::tir::Stmt>>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.IfFrame",
                    "then_stmts",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__ELSE_STMTS: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::Array<crate::tir::Stmt>>>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.IfFrame",
                    "else_stmts",
                )
                .expect("missing field")
            });
            impl IfFrame {
                pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__CONDITION.get(self.as_object_ref())
                }
                pub fn then_stmts(&self) -> Result<Option<tvm_ffi::Array<crate::tir::Stmt>>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__THEN_STMTS.get(self.as_object_ref())
                }
                pub fn else_stmts(&self) -> Result<Option<tvm_ffi::Array<crate::tir::Stmt>>> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_IFFRAME__ELSE_STMTS.get(self.as_object_ref())
                }
            }

            tvm_ffi::define_object_wrapper!(ThenFrame, "script.ir_builder.tir.ThenFrame");

            impl ThenFrame {}

            tvm_ffi::define_object_wrapper!(ElseFrame, "script.ir_builder.tir.ElseFrame");

            impl ElseFrame {}

            tvm_ffi::define_object_wrapper!(
                DeclBufferFrame,
                "script.ir_builder.tir.DeclBufferFrame"
            );

            static FIELD_SCRIPT_IR_BUILDER_TIR_DECLBUFFERFRAME__BUFFER: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.DeclBufferFrame",
                    "buffer",
                )
                .expect("missing field")
            });
            static FIELD_SCRIPT_IR_BUILDER_TIR_DECLBUFFERFRAME__ALLOCATED: LazyLock<
                tvm_ffi::object_wrapper::FieldGetter<bool>,
            > = LazyLock::new(|| {
                tvm_ffi::object_wrapper::FieldGetter::new(
                    "script.ir_builder.tir.DeclBufferFrame",
                    "allocated",
                )
                .expect("missing field")
            });
            impl DeclBufferFrame {
                pub fn buffer(&self) -> Result<crate::tir::Buffer> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_DECLBUFFERFRAME__BUFFER.get(self.as_object_ref())
                }
                pub fn allocated(&self) -> Result<bool> {
                    FIELD_SCRIPT_IR_BUILDER_TIR_DECLBUFFERFRAME__ALLOCATED.get(self.as_object_ref())
                }
            }
        }
    }
    pub mod printer {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        tvm_ffi::define_object_wrapper!(Doc, "script.printer.Doc");

        static FIELD_SCRIPT_PRINTER_DOC__SOURCE_PATHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<
                tvm_ffi::Array<crate::ffi::reflection::AccessPath>,
            >,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.Doc", "source_paths")
                .expect("missing field")
        });
        impl Doc {
            pub fn source_paths(
                &self,
            ) -> Result<tvm_ffi::Array<crate::ffi::reflection::AccessPath>> {
                FIELD_SCRIPT_PRINTER_DOC__SOURCE_PATHS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ExprDoc, "script.printer.ExprDoc");

        impl ExprDoc {}

        tvm_ffi::define_object_wrapper!(StmtDoc, "script.printer.StmtDoc");

        static FIELD_SCRIPT_PRINTER_STMTDOC__COMMENT: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::String>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.StmtDoc", "comment")
                .expect("missing field")
        });
        impl StmtDoc {
            pub fn comment(&self) -> Result<Option<tvm_ffi::String>> {
                FIELD_SCRIPT_PRINTER_STMTDOC__COMMENT.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(StmtBlockDoc, "script.printer.StmtBlockDoc");

        static FIELD_SCRIPT_PRINTER_STMTBLOCKDOC__STMTS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.StmtBlockDoc", "stmts")
                .expect("missing field")
        });
        impl StmtBlockDoc {
            pub fn stmts(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_STMTBLOCKDOC__STMTS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(LiteralDoc, "script.printer.LiteralDoc");

        static FIELD_SCRIPT_PRINTER_LITERALDOC__VALUE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::AnyValue>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.LiteralDoc", "value")
                .expect("missing field")
        });
        impl LiteralDoc {
            pub fn value(&self) -> Result<tvm_ffi::AnyValue> {
                FIELD_SCRIPT_PRINTER_LITERALDOC__VALUE.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(IdDoc, "script.printer.IdDoc");

        static FIELD_SCRIPT_PRINTER_IDDOC__NAME: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IdDoc", "name")
                .expect("missing field")
        });
        impl IdDoc {
            pub fn name(&self) -> Result<tvm_ffi::String> {
                FIELD_SCRIPT_PRINTER_IDDOC__NAME.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(AttrAccessDoc, "script.printer.AttrAccessDoc");

        static FIELD_SCRIPT_PRINTER_ATTRACCESSDOC__VALUE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AttrAccessDoc", "value")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_ATTRACCESSDOC__NAME: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AttrAccessDoc", "name")
                .expect("missing field")
        });
        impl AttrAccessDoc {
            pub fn value(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_ATTRACCESSDOC__VALUE.get(self.as_object_ref())
            }
            pub fn name(&self) -> Result<tvm_ffi::String> {
                FIELD_SCRIPT_PRINTER_ATTRACCESSDOC__NAME.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(IndexDoc, "script.printer.IndexDoc");

        static FIELD_SCRIPT_PRINTER_INDEXDOC__VALUE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IndexDoc", "value")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_INDEXDOC__INDICES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::Doc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IndexDoc", "indices")
                .expect("missing field")
        });
        impl IndexDoc {
            pub fn value(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_INDEXDOC__VALUE.get(self.as_object_ref())
            }
            pub fn indices(&self) -> Result<tvm_ffi::Array<crate::script::printer::Doc>> {
                FIELD_SCRIPT_PRINTER_INDEXDOC__INDICES.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(CallDoc, "script.printer.CallDoc");

        static FIELD_SCRIPT_PRINTER_CALLDOC__CALLEE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.CallDoc", "callee")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_CALLDOC__ARGS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.CallDoc", "args")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_CALLDOC__KWARGS_KEYS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<tvm_ffi::String>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.CallDoc", "kwargs_keys")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_CALLDOC__KWARGS_VALUES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.CallDoc", "kwargs_values")
                .expect("missing field")
        });
        impl CallDoc {
            pub fn callee(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_CALLDOC__CALLEE.get(self.as_object_ref())
            }
            pub fn args(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_CALLDOC__ARGS.get(self.as_object_ref())
            }
            pub fn kwargs_keys(&self) -> Result<tvm_ffi::Array<tvm_ffi::String>> {
                FIELD_SCRIPT_PRINTER_CALLDOC__KWARGS_KEYS.get(self.as_object_ref())
            }
            pub fn kwargs_values(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_CALLDOC__KWARGS_VALUES.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(OperationDoc, "script.printer.OperationDoc");

        static FIELD_SCRIPT_PRINTER_OPERATIONDOC__KIND: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.OperationDoc", "kind")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_OPERATIONDOC__OPERANDS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.OperationDoc", "operands")
                .expect("missing field")
        });
        impl OperationDoc {
            pub fn kind(&self) -> Result<i64> {
                FIELD_SCRIPT_PRINTER_OPERATIONDOC__KIND.get(self.as_object_ref())
            }
            pub fn operands(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_OPERATIONDOC__OPERANDS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(LambdaDoc, "script.printer.LambdaDoc");

        static FIELD_SCRIPT_PRINTER_LAMBDADOC__ARGS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::IdDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.LambdaDoc", "args")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_LAMBDADOC__BODY: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.LambdaDoc", "body")
                .expect("missing field")
        });
        impl LambdaDoc {
            pub fn args(&self) -> Result<tvm_ffi::Array<crate::script::printer::IdDoc>> {
                FIELD_SCRIPT_PRINTER_LAMBDADOC__ARGS.get(self.as_object_ref())
            }
            pub fn body(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_LAMBDADOC__BODY.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(TupleDoc, "script.printer.TupleDoc");

        static FIELD_SCRIPT_PRINTER_TUPLEDOC__ELEMENTS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.TupleDoc", "elements")
                .expect("missing field")
        });
        impl TupleDoc {
            pub fn elements(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_TUPLEDOC__ELEMENTS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ListDoc, "script.printer.ListDoc");

        static FIELD_SCRIPT_PRINTER_LISTDOC__ELEMENTS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ListDoc", "elements")
                .expect("missing field")
        });
        impl ListDoc {
            pub fn elements(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_LISTDOC__ELEMENTS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(DictDoc, "script.printer.DictDoc");

        static FIELD_SCRIPT_PRINTER_DICTDOC__KEYS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.DictDoc", "keys")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_DICTDOC__VALUES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.DictDoc", "values")
                .expect("missing field")
        });
        impl DictDoc {
            pub fn keys(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_DICTDOC__KEYS.get(self.as_object_ref())
            }
            pub fn values(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_DICTDOC__VALUES.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(SliceDoc, "script.printer.SliceDoc");

        static FIELD_SCRIPT_PRINTER_SLICEDOC__START: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.SliceDoc", "start")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_SLICEDOC__STOP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.SliceDoc", "stop")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_SLICEDOC__STEP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.SliceDoc", "step")
                .expect("missing field")
        });
        impl SliceDoc {
            pub fn start(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_SLICEDOC__START.get(self.as_object_ref())
            }
            pub fn stop(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_SLICEDOC__STOP.get(self.as_object_ref())
            }
            pub fn step(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_SLICEDOC__STEP.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(AssignDoc, "script.printer.AssignDoc");

        static FIELD_SCRIPT_PRINTER_ASSIGNDOC__LHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssignDoc", "lhs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_ASSIGNDOC__RHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssignDoc", "rhs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_ASSIGNDOC__ANNOTATION: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssignDoc", "annotation")
                .expect("missing field")
        });
        impl AssignDoc {
            pub fn lhs(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_ASSIGNDOC__LHS.get(self.as_object_ref())
            }
            pub fn rhs(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_ASSIGNDOC__RHS.get(self.as_object_ref())
            }
            pub fn annotation(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_ASSIGNDOC__ANNOTATION.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(IfDoc, "script.printer.IfDoc");

        static FIELD_SCRIPT_PRINTER_IFDOC__PREDICATE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IfDoc", "predicate")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_IFDOC__THEN_BRANCH: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IfDoc", "then_branch")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_IFDOC__ELSE_BRANCH: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IfDoc", "else_branch")
                .expect("missing field")
        });
        impl IfDoc {
            pub fn predicate(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_IFDOC__PREDICATE.get(self.as_object_ref())
            }
            pub fn then_branch(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_IFDOC__THEN_BRANCH.get(self.as_object_ref())
            }
            pub fn else_branch(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_IFDOC__ELSE_BRANCH.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(WhileDoc, "script.printer.WhileDoc");

        static FIELD_SCRIPT_PRINTER_WHILEDOC__PREDICATE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.WhileDoc", "predicate")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_WHILEDOC__BODY: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.WhileDoc", "body")
                .expect("missing field")
        });
        impl WhileDoc {
            pub fn predicate(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_WHILEDOC__PREDICATE.get(self.as_object_ref())
            }
            pub fn body(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_WHILEDOC__BODY.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ForDoc, "script.printer.ForDoc");

        static FIELD_SCRIPT_PRINTER_FORDOC__LHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ForDoc", "lhs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_FORDOC__RHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ForDoc", "rhs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_FORDOC__BODY: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ForDoc", "body")
                .expect("missing field")
        });
        impl ForDoc {
            pub fn lhs(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_FORDOC__LHS.get(self.as_object_ref())
            }
            pub fn rhs(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_FORDOC__RHS.get(self.as_object_ref())
            }
            pub fn body(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_FORDOC__BODY.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ScopeDoc, "script.printer.ScopeDoc");

        static FIELD_SCRIPT_PRINTER_SCOPEDOC__LHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ScopeDoc", "lhs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_SCOPEDOC__RHS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ScopeDoc", "rhs")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_SCOPEDOC__BODY: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ScopeDoc", "body")
                .expect("missing field")
        });
        impl ScopeDoc {
            pub fn lhs(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_SCOPEDOC__LHS.get(self.as_object_ref())
            }
            pub fn rhs(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_SCOPEDOC__RHS.get(self.as_object_ref())
            }
            pub fn body(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_SCOPEDOC__BODY.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ExprStmtDoc, "script.printer.ExprStmtDoc");

        static FIELD_SCRIPT_PRINTER_EXPRSTMTDOC__EXPR: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ExprStmtDoc", "expr")
                .expect("missing field")
        });
        impl ExprStmtDoc {
            pub fn expr(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_EXPRSTMTDOC__EXPR.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(AssertDoc, "script.printer.AssertDoc");

        static FIELD_SCRIPT_PRINTER_ASSERTDOC__TEST: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssertDoc", "test")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_ASSERTDOC__MSG: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.AssertDoc", "msg")
                .expect("missing field")
        });
        impl AssertDoc {
            pub fn test(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_ASSERTDOC__TEST.get(self.as_object_ref())
            }
            pub fn msg(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_ASSERTDOC__MSG.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ReturnDoc, "script.printer.ReturnDoc");

        static FIELD_SCRIPT_PRINTER_RETURNDOC__VALUE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::ExprDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ReturnDoc", "value")
                .expect("missing field")
        });
        impl ReturnDoc {
            pub fn value(&self) -> Result<crate::script::printer::ExprDoc> {
                FIELD_SCRIPT_PRINTER_RETURNDOC__VALUE.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(FunctionDoc, "script.printer.FunctionDoc");

        static FIELD_SCRIPT_PRINTER_FUNCTIONDOC__NAME: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::IdDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.FunctionDoc", "name")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_FUNCTIONDOC__ARGS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::AssignDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.FunctionDoc", "args")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_FUNCTIONDOC__DECORATORS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.FunctionDoc", "decorators")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_FUNCTIONDOC__RETURN_TYPE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<Option<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.FunctionDoc", "return_type")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_FUNCTIONDOC__BODY: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.FunctionDoc", "body")
                .expect("missing field")
        });
        impl FunctionDoc {
            pub fn name(&self) -> Result<crate::script::printer::IdDoc> {
                FIELD_SCRIPT_PRINTER_FUNCTIONDOC__NAME.get(self.as_object_ref())
            }
            pub fn args(&self) -> Result<tvm_ffi::Array<crate::script::printer::AssignDoc>> {
                FIELD_SCRIPT_PRINTER_FUNCTIONDOC__ARGS.get(self.as_object_ref())
            }
            pub fn decorators(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_FUNCTIONDOC__DECORATORS.get(self.as_object_ref())
            }
            pub fn return_type(&self) -> Result<Option<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_FUNCTIONDOC__RETURN_TYPE.get(self.as_object_ref())
            }
            pub fn body(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_FUNCTIONDOC__BODY.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(ClassDoc, "script.printer.ClassDoc");

        static FIELD_SCRIPT_PRINTER_CLASSDOC__NAME: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<crate::script::printer::IdDoc>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ClassDoc", "name")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_CLASSDOC__DECORATORS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::ExprDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ClassDoc", "decorators")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_CLASSDOC__BODY: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.ClassDoc", "body")
                .expect("missing field")
        });
        impl ClassDoc {
            pub fn name(&self) -> Result<crate::script::printer::IdDoc> {
                FIELD_SCRIPT_PRINTER_CLASSDOC__NAME.get(self.as_object_ref())
            }
            pub fn decorators(&self) -> Result<tvm_ffi::Array<crate::script::printer::ExprDoc>> {
                FIELD_SCRIPT_PRINTER_CLASSDOC__DECORATORS.get(self.as_object_ref())
            }
            pub fn body(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_CLASSDOC__BODY.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(CommentDoc, "script.printer.CommentDoc");

        impl CommentDoc {}

        tvm_ffi::define_object_wrapper!(DocStringDoc, "script.printer.DocStringDoc");

        impl DocStringDoc {}

        tvm_ffi::define_object_wrapper!(Frame, "script.printer.Frame");

        static FIELD_SCRIPT_PRINTER_FRAME__STMTS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::StmtDoc>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.Frame", "stmts")
                .expect("missing field")
        });
        impl Frame {
            pub fn stmts(&self) -> Result<tvm_ffi::Array<crate::script::printer::StmtDoc>> {
                FIELD_SCRIPT_PRINTER_FRAME__STMTS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(IRDocsifier, "script.printer.IRDocsifier");

        static FIELD_SCRIPT_PRINTER_IRDOCSIFIER__FRAMES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::script::printer::Frame>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.IRDocsifier", "frames")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_IRDOCSIFIER__DISPATCH_TOKENS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<tvm_ffi::String>>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.printer.IRDocsifier",
                "dispatch_tokens",
            )
            .expect("missing field")
        });
        impl IRDocsifier {
            pub fn frames(&self) -> Result<tvm_ffi::Array<crate::script::printer::Frame>> {
                FIELD_SCRIPT_PRINTER_IRDOCSIFIER__FRAMES.get(self.as_object_ref())
            }
            pub fn dispatch_tokens(&self) -> Result<tvm_ffi::Array<tvm_ffi::String>> {
                FIELD_SCRIPT_PRINTER_IRDOCSIFIER__DISPATCH_TOKENS.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(RelaxFrame, "script.printer.RelaxFrame");

        static FIELD_SCRIPT_PRINTER_RELAXFRAME__IS_FUNC: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.RelaxFrame", "is_func")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_RELAXFRAME__MODULE_ALIAS_PRINTED: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.printer.RelaxFrame",
                "module_alias_printed",
            )
            .expect("missing field")
        });
        impl RelaxFrame {
            pub fn is_func(&self) -> Result<bool> {
                FIELD_SCRIPT_PRINTER_RELAXFRAME__IS_FUNC.get(self.as_object_ref())
            }
            pub fn module_alias_printed(&self) -> Result<bool> {
                FIELD_SCRIPT_PRINTER_RELAXFRAME__MODULE_ALIAS_PRINTED.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(TIRFrame, "script.printer.TIRFrame");

        static FIELD_SCRIPT_PRINTER_TIRFRAME__TIR: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::object::ObjectRef>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("script.printer.TIRFrame", "tir")
                .expect("missing field")
        });
        static FIELD_SCRIPT_PRINTER_TIRFRAME__ALLOW_CONCISE_SCOPING: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "script.printer.TIRFrame",
                "allow_concise_scoping",
            )
            .expect("missing field")
        });
        impl TIRFrame {
            pub fn tir(&self) -> Result<tvm_ffi::object::ObjectRef> {
                FIELD_SCRIPT_PRINTER_TIRFRAME__TIR.get(self.as_object_ref())
            }
            pub fn allow_concise_scoping(&self) -> Result<bool> {
                FIELD_SCRIPT_PRINTER_TIRFRAME__ALLOW_CONCISE_SCOPING.get(self.as_object_ref())
            }
        }
    }
}
pub mod target {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    tvm_ffi::define_object_wrapper!(Target, "target.Target");

    static FIELD_TARGET_TARGET__KIND: LazyLock<tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("target.Target", "kind")
                .expect("missing field")
        });
    static FIELD_TARGET_TARGET__TAG: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.Target", "tag").expect("missing field")
    });
    static FIELD_TARGET_TARGET__KEYS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<tvm_ffi::String>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.Target", "keys").expect("missing field")
    });
    static FIELD_TARGET_TARGET__ATTRS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.Target", "attrs").expect("missing field")
    });
    static FIELD_TARGET_TARGET__FEATURES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.Target", "features")
            .expect("missing field")
    });
    static FIELD_TARGET_TARGET__HOST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<Option<tvm_ffi::object::ObjectRef>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("target.Target", "host").expect("missing field")
    });
    impl Target {
        pub fn kind(&self) -> Result<tvm_ffi::Any> {
            FIELD_TARGET_TARGET__KIND.get_any(self.as_object_ref())
        }
        pub fn tag(&self) -> Result<tvm_ffi::String> {
            FIELD_TARGET_TARGET__TAG.get(self.as_object_ref())
        }
        pub fn keys(&self) -> Result<tvm_ffi::Array<tvm_ffi::String>> {
            FIELD_TARGET_TARGET__KEYS.get(self.as_object_ref())
        }
        pub fn attrs(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
            FIELD_TARGET_TARGET__ATTRS.get(self.as_object_ref())
        }
        pub fn features(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
            FIELD_TARGET_TARGET__FEATURES.get(self.as_object_ref())
        }
        pub fn host(&self) -> Result<Option<tvm_ffi::object::ObjectRef>> {
            FIELD_TARGET_TARGET__HOST.get(self.as_object_ref())
        }
    }
}
pub mod tir {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    tvm_ffi::define_object_wrapper!(Var, "tir.Var");

    static FIELD_TIR_VAR__NAME: LazyLock<tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Var", "name").expect("missing field")
        });
    static FIELD_TIR_VAR__TYPE_ANNOTATION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Type>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Var", "type_annotation")
            .expect("missing field")
    });
    impl Var {
        pub fn name(&self) -> Result<tvm_ffi::String> {
            FIELD_TIR_VAR__NAME.get(self.as_object_ref())
        }
        pub fn type_annotation(&self) -> Result<crate::ir::Type> {
            FIELD_TIR_VAR__TYPE_ANNOTATION.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(SizeVar, "tir.SizeVar");

    impl SizeVar {}

    tvm_ffi::define_object_wrapper!(BufferLoad, "tir.BufferLoad");

    static FIELD_TIR_BUFFERLOAD__BUFFER: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.BufferLoad", "buffer")
            .expect("missing field")
    });
    static FIELD_TIR_BUFFERLOAD__INDICES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.BufferLoad", "indices")
            .expect("missing field")
    });
    static FIELD_TIR_BUFFERLOAD__PREDICATE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.BufferLoad", "predicate")
            .expect("missing field")
    });
    impl BufferLoad {
        pub fn buffer(&self) -> Result<crate::tir::Buffer> {
            FIELD_TIR_BUFFERLOAD__BUFFER.get(self.as_object_ref())
        }
        pub fn indices(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_BUFFERLOAD__INDICES.get(self.as_object_ref())
        }
        pub fn predicate(&self) -> Result<Option<crate::ir::PrimExpr>> {
            FIELD_TIR_BUFFERLOAD__PREDICATE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(ProducerLoad, "tir.ProducerLoad");

    static FIELD_TIR_PRODUCERLOAD__PRODUCER: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::DataProducer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.ProducerLoad", "producer")
            .expect("missing field")
    });
    static FIELD_TIR_PRODUCERLOAD__INDICES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.ProducerLoad", "indices")
            .expect("missing field")
    });
    impl ProducerLoad {
        pub fn producer(&self) -> Result<crate::tir::DataProducer> {
            FIELD_TIR_PRODUCERLOAD__PRODUCER.get(self.as_object_ref())
        }
        pub fn indices(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_PRODUCERLOAD__INDICES.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Not, "tir.Not");

    static FIELD_TIR_NOT__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Not", "a").expect("missing field")
        });
    impl Not {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_NOT__A.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(StringImm, "tir.StringImm");

    static FIELD_TIR_STRINGIMM__VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.StringImm", "value").expect("missing field")
    });
    impl StringImm {
        pub fn value(&self) -> Result<tvm_ffi::String> {
            FIELD_TIR_STRINGIMM__VALUE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Cast, "tir.Cast");

    static FIELD_TIR_CAST__VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Cast", "value").expect("missing field")
    });
    impl Cast {
        pub fn value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_CAST__VALUE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Select, "tir.Select");

    static FIELD_TIR_SELECT__CONDITION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Select", "condition").expect("missing field")
    });
    static FIELD_TIR_SELECT__TRUE_VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Select", "true_value")
            .expect("missing field")
    });
    static FIELD_TIR_SELECT__FALSE_VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Select", "false_value")
            .expect("missing field")
    });
    impl Select {
        pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_SELECT__CONDITION.get(self.as_object_ref())
        }
        pub fn true_value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_SELECT__TRUE_VALUE.get(self.as_object_ref())
        }
        pub fn false_value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_SELECT__FALSE_VALUE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Ramp, "tir.Ramp");

    static FIELD_TIR_RAMP__BASE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Ramp", "base").expect("missing field")
    });
    static FIELD_TIR_RAMP__STRIDE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Ramp", "stride").expect("missing field")
    });
    static FIELD_TIR_RAMP__LANES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Ramp", "lanes").expect("missing field")
    });
    impl Ramp {
        pub fn base(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_RAMP__BASE.get(self.as_object_ref())
        }
        pub fn stride(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_RAMP__STRIDE.get(self.as_object_ref())
        }
        pub fn lanes(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_RAMP__LANES.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Broadcast, "tir.Broadcast");

    static FIELD_TIR_BROADCAST__VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Broadcast", "value").expect("missing field")
    });
    static FIELD_TIR_BROADCAST__LANES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Broadcast", "lanes").expect("missing field")
    });
    impl Broadcast {
        pub fn value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_BROADCAST__VALUE.get(self.as_object_ref())
        }
        pub fn lanes(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_BROADCAST__LANES.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Shuffle, "tir.Shuffle");

    static FIELD_TIR_SHUFFLE__VECTORS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Shuffle", "vectors").expect("missing field")
    });
    static FIELD_TIR_SHUFFLE__INDICES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Shuffle", "indices").expect("missing field")
    });
    impl Shuffle {
        pub fn vectors(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_SHUFFLE__VECTORS.get(self.as_object_ref())
        }
        pub fn indices(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_SHUFFLE__INDICES.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Let, "tir.Let");

    static FIELD_TIR_LET__VAR: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Var>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Let", "var").expect("missing field")
        });
    static FIELD_TIR_LET__VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Let", "value").expect("missing field")
    });
    static FIELD_TIR_LET__BODY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Let", "body").expect("missing field")
    });
    impl Let {
        pub fn var(&self) -> Result<crate::tir::Var> {
            FIELD_TIR_LET__VAR.get(self.as_object_ref())
        }
        pub fn value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_LET__VALUE.get(self.as_object_ref())
        }
        pub fn body(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_LET__BODY.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Call, "tir.Call");

    static FIELD_TIR_CALL__OP: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::RelaxExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Call", "op").expect("missing field")
    });
    static FIELD_TIR_CALL__ARGS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Call", "args").expect("missing field")
    });
    static FIELD_TIR_CALL__ANNOTATIONS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Call", "annotations").expect("missing field")
    });
    impl Call {
        pub fn op(&self) -> Result<crate::ir::RelaxExpr> {
            FIELD_TIR_CALL__OP.get(self.as_object_ref())
        }
        pub fn args(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_CALL__ARGS.get(self.as_object_ref())
        }
        pub fn annotations(
            &self,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>> {
            FIELD_TIR_CALL__ANNOTATIONS.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Reduce, "tir.Reduce");

    static FIELD_TIR_REDUCE__COMBINER: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::CommReducer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Reduce", "combiner").expect("missing field")
    });
    static FIELD_TIR_REDUCE__SOURCE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Reduce", "source").expect("missing field")
    });
    static FIELD_TIR_REDUCE__INIT: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Reduce", "init").expect("missing field")
    });
    static FIELD_TIR_REDUCE__AXIS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::tir::IterVar>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Reduce", "axis").expect("missing field")
    });
    static FIELD_TIR_REDUCE__CONDITION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Reduce", "condition").expect("missing field")
    });
    static FIELD_TIR_REDUCE__VALUE_INDEX: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Reduce", "value_index")
                .expect("missing field")
        });
    impl Reduce {
        pub fn combiner(&self) -> Result<crate::tir::CommReducer> {
            FIELD_TIR_REDUCE__COMBINER.get(self.as_object_ref())
        }
        pub fn source(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_REDUCE__SOURCE.get(self.as_object_ref())
        }
        pub fn init(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TIR_REDUCE__INIT.get(self.as_object_ref())
        }
        pub fn axis(&self) -> Result<tvm_ffi::Array<crate::tir::IterVar>> {
            FIELD_TIR_REDUCE__AXIS.get(self.as_object_ref())
        }
        pub fn condition(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_REDUCE__CONDITION.get(self.as_object_ref())
        }
        pub fn value_index(&self) -> Result<i64> {
            FIELD_TIR_REDUCE__VALUE_INDEX.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Div, "tir.Div");

    static FIELD_TIR_DIV__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Div", "a").expect("missing field")
        });
    static FIELD_TIR_DIV__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Div", "b").expect("missing field")
        });
    impl Div {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_DIV__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_DIV__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Add, "tir.Add");

    static FIELD_TIR_ADD__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Add", "a").expect("missing field")
        });
    static FIELD_TIR_ADD__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Add", "b").expect("missing field")
        });
    impl Add {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_ADD__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_ADD__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Sub, "tir.Sub");

    static FIELD_TIR_SUB__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Sub", "a").expect("missing field")
        });
    static FIELD_TIR_SUB__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Sub", "b").expect("missing field")
        });
    impl Sub {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_SUB__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_SUB__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Mul, "tir.Mul");

    static FIELD_TIR_MUL__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Mul", "a").expect("missing field")
        });
    static FIELD_TIR_MUL__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Mul", "b").expect("missing field")
        });
    impl Mul {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MUL__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MUL__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(FloorDiv, "tir.FloorDiv");

    static FIELD_TIR_FLOORDIV__A: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.FloorDiv", "a").expect("missing field")
    });
    static FIELD_TIR_FLOORDIV__B: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.FloorDiv", "b").expect("missing field")
    });
    impl FloorDiv {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_FLOORDIV__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_FLOORDIV__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(FloorMod, "tir.FloorMod");

    static FIELD_TIR_FLOORMOD__A: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.FloorMod", "a").expect("missing field")
    });
    static FIELD_TIR_FLOORMOD__B: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.FloorMod", "b").expect("missing field")
    });
    impl FloorMod {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_FLOORMOD__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_FLOORMOD__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(LT, "tir.LT");

    static FIELD_TIR_LT__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.LT", "a").expect("missing field")
        });
    static FIELD_TIR_LT__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.LT", "b").expect("missing field")
        });
    impl LT {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_LT__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_LT__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(LE, "tir.LE");

    static FIELD_TIR_LE__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.LE", "a").expect("missing field")
        });
    static FIELD_TIR_LE__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.LE", "b").expect("missing field")
        });
    impl LE {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_LE__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_LE__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(EQ, "tir.EQ");

    static FIELD_TIR_EQ__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.EQ", "a").expect("missing field")
        });
    static FIELD_TIR_EQ__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.EQ", "b").expect("missing field")
        });
    impl EQ {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_EQ__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_EQ__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(NE, "tir.NE");

    static FIELD_TIR_NE__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.NE", "a").expect("missing field")
        });
    static FIELD_TIR_NE__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.NE", "b").expect("missing field")
        });
    impl NE {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_NE__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_NE__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GT, "tir.GT");

    static FIELD_TIR_GT__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.GT", "a").expect("missing field")
        });
    static FIELD_TIR_GT__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.GT", "b").expect("missing field")
        });
    impl GT {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_GT__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_GT__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GE, "tir.GE");

    static FIELD_TIR_GE__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.GE", "a").expect("missing field")
        });
    static FIELD_TIR_GE__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.GE", "b").expect("missing field")
        });
    impl GE {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_GE__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_GE__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(And, "tir.And");

    static FIELD_TIR_AND__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.And", "a").expect("missing field")
        });
    static FIELD_TIR_AND__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.And", "b").expect("missing field")
        });
    impl And {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_AND__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_AND__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Or, "tir.Or");

    static FIELD_TIR_OR__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Or", "a").expect("missing field")
        });
    static FIELD_TIR_OR__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Or", "b").expect("missing field")
        });
    impl Or {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_OR__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_OR__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Mod, "tir.Mod");

    static FIELD_TIR_MOD__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Mod", "a").expect("missing field")
        });
    static FIELD_TIR_MOD__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Mod", "b").expect("missing field")
        });
    impl Mod {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MOD__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MOD__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Min, "tir.Min");

    static FIELD_TIR_MIN__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Min", "a").expect("missing field")
        });
    static FIELD_TIR_MIN__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Min", "b").expect("missing field")
        });
    impl Min {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MIN__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MIN__B.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Max, "tir.Max");

    static FIELD_TIR_MAX__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Max", "a").expect("missing field")
        });
    static FIELD_TIR_MAX__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Max", "b").expect("missing field")
        });
    impl Max {
        pub fn a(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MAX__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TIR_MAX__B.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Buffer"]
    pub struct BufferObj {
        parent: tvm_ffi::object::Object,
        data: crate::tir::Var,
        dtype: tvm_ffi::DLDataType,
        shape: tvm_ffi::Array<crate::ir::PrimExpr>,
        axis_separators: tvm_ffi::Array<crate::ir::IntImm>,
        strides: tvm_ffi::Array<crate::ir::PrimExpr>,
        elem_offset: crate::ir::PrimExpr,
        name: tvm_ffi::String,
        data_alignment: i32,
        offset_factor: i32,
        buffer_type: i32,
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

    tvm_ffi::define_object_wrapper!(IterVar, "tir.IterVar");

    static FIELD_TIR_ITERVAR__DOM: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::Range>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.IterVar", "dom").expect("missing field")
    });
    static FIELD_TIR_ITERVAR__VAR: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Var>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.IterVar", "var").expect("missing field")
        });
    static FIELD_TIR_ITERVAR__ITER_TYPE: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.IterVar", "iter_type")
                .expect("missing field")
        });
    static FIELD_TIR_ITERVAR__THREAD_TAG: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.IterVar", "thread_tag")
            .expect("missing field")
    });
    impl IterVar {
        pub fn dom(&self) -> Result<crate::ir::Range> {
            FIELD_TIR_ITERVAR__DOM.get(self.as_object_ref())
        }
        pub fn var(&self) -> Result<crate::tir::Var> {
            FIELD_TIR_ITERVAR__VAR.get(self.as_object_ref())
        }
        pub fn iter_type(&self) -> Result<i64> {
            FIELD_TIR_ITERVAR__ITER_TYPE.get(self.as_object_ref())
        }
        pub fn thread_tag(&self) -> Result<tvm_ffi::String> {
            FIELD_TIR_ITERVAR__THREAD_TAG.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Stmt"]
    pub struct StmtObj {
        parent: tvm_ffi::object::Object,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        loop_var: crate::tir::Var,
        min: crate::ir::PrimExpr,
        extent: crate::ir::PrimExpr,
        kind: i32,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer_var: crate::tir::Var,
        dtype: tvm_ffi::DLDataType,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
        buffer_var: crate::tir::Var,
        data: Option<tvm_ffi::Tensor>,
        irmod_storage_idx: Option<crate::ir::IntImm>,
        dtype: tvm_ffi::DLDataType,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::tir::StmtObj,
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

    tvm_ffi::define_object_wrapper!(BufferRegion, "tir.BufferRegion");

    static FIELD_TIR_BUFFERREGION__BUFFER: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.BufferRegion", "buffer")
            .expect("missing field")
    });
    static FIELD_TIR_BUFFERREGION__REGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.BufferRegion", "region")
            .expect("missing field")
    });
    impl BufferRegion {
        pub fn buffer(&self) -> Result<crate::tir::Buffer> {
            FIELD_TIR_BUFFERREGION__BUFFER.get(self.as_object_ref())
        }
        pub fn region(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TIR_BUFFERREGION__REGION.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.MatchBufferRegion"]
    pub struct MatchBufferRegionObj {
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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
        parent: crate::_tvm_ffi_stubgen_detail::types::ir::BaseFuncObj,
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

    tvm_ffi::define_object_wrapper!(BlockDependenceInfo, "tir.BlockDependenceInfo");

    impl BlockDependenceInfo {}

    tvm_ffi::define_object_wrapper!(StmtSRef, "tir.StmtSRef");

    static FIELD_TIR_STMTSREF__SEQ_INDEX: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.StmtSRef", "seq_index")
                .expect("missing field")
        });
    impl StmtSRef {
        pub fn seq_index(&self) -> Result<i64> {
            FIELD_TIR_STMTSREF__SEQ_INDEX.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Dependency, "tir.Dependency");

    static FIELD_TIR_DEPENDENCY__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::StmtSRef>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Dependency", "src").expect("missing field")
    });
    static FIELD_TIR_DEPENDENCY__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::StmtSRef>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.Dependency", "dst").expect("missing field")
    });
    static FIELD_TIR_DEPENDENCY__KIND: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tir.Dependency", "kind")
                .expect("missing field")
        });
    impl Dependency {
        pub fn src(&self) -> Result<crate::tir::StmtSRef> {
            FIELD_TIR_DEPENDENCY__SRC.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::StmtSRef> {
            FIELD_TIR_DEPENDENCY__DST.get(self.as_object_ref())
        }
        pub fn kind(&self) -> Result<i64> {
            FIELD_TIR_DEPENDENCY__KIND.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(BlockScope, "tir.BlockScope");

    impl BlockScope {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Layout"]
    pub struct LayoutObj {
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(PyStmtExprVisitor, "tir.PyStmtExprVisitor");

    impl PyStmtExprVisitor {}

    tvm_ffi::define_object_wrapper!(PyStmtExprMutator, "tir.PyStmtExprMutator");

    impl PyStmtExprMutator {}

    tvm_ffi::define_object_wrapper!(PrimFuncPass, "tir.PrimFuncPass");

    static FIELD_TIR_PRIMFUNCPASS__PASS_INFO: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::transform::PassInfo>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.PrimFuncPass", "pass_info")
            .expect("missing field")
    });
    impl PrimFuncPass {
        pub fn pass_info(&self) -> Result<crate::transform::PassInfo> {
            FIELD_TIR_PRIMFUNCPASS__PASS_INFO.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(InstructionKind, "tir.InstructionKind");

    static FIELD_TIR_INSTRUCTIONKIND__NAME: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::String>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.InstructionKind", "name")
            .expect("missing field")
    });
    static FIELD_TIR_INSTRUCTIONKIND___IS_PURE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<bool>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.InstructionKind", "_is_pure")
            .expect("missing field")
    });
    impl InstructionKind {
        pub fn name(&self) -> Result<tvm_ffi::String> {
            FIELD_TIR_INSTRUCTIONKIND__NAME.get(self.as_object_ref())
        }
        pub fn _is_pure(&self) -> Result<bool> {
            FIELD_TIR_INSTRUCTIONKIND___IS_PURE.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Instruction"]
    pub struct InstructionObj {
        parent: tvm_ffi::object::Object,
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

    tvm_ffi::define_object_wrapper!(Schedule, "tir.Schedule");

    impl Schedule {}

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.BlockRV"]
    pub struct BlockRVObj {
        parent: tvm_ffi::object::Object,
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
        parent: tvm_ffi::object::Object,
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::ObjectRef, Clone)]
    pub struct LoopRV {
        data: tvm_ffi::object::ObjectArc<LoopRVObj>,
    }

    tvm_ffi::impl_object_hierarchy!(LoopRV: tvm_ffi::object::ObjectRef);

    impl LoopRV {}

    impl LoopRV {}

    tvm_ffi::define_object_wrapper!(ScheduleState, "tir.ScheduleState");

    static FIELD_TIR_SCHEDULESTATE__MOD: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::IRModule>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.ScheduleState", "mod")
            .expect("missing field")
    });
    static FIELD_TIR_SCHEDULESTATE__DEBUG_MASK: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i64>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.ScheduleState", "debug_mask")
            .expect("missing field")
    });
    static FIELD_TIR_SCHEDULESTATE__ENABLE_CHECK: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<bool>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tir.ScheduleState", "enable_check")
            .expect("missing field")
    });
    impl ScheduleState {
        pub fn mod_(&self) -> Result<crate::ir::IRModule> {
            FIELD_TIR_SCHEDULESTATE__MOD.get(self.as_object_ref())
        }
        pub fn debug_mask(&self) -> Result<i64> {
            FIELD_TIR_SCHEDULESTATE__DEBUG_MASK.get(self.as_object_ref())
        }
        pub fn enable_check(&self) -> Result<bool> {
            FIELD_TIR_SCHEDULESTATE__ENABLE_CHECK.get(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "tir.Trace"]
    pub struct TraceObj {
        parent: tvm_ffi::object::Object,
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
            parent: tvm_ffi::object::Object,
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
            parent: tvm_ffi::object::Object,
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

        tvm_ffi::define_object_wrapper!(
            HoistExpressionConfig,
            "tir.transform.HoistExpressionConfig"
        );

        static FIELD_TIR_TRANSFORM_HOISTEXPRESSIONCONFIG__HOISTED_CONDITIONALS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.HoistExpressionConfig",
                "hoisted_conditionals",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_HOISTEXPRESSIONCONFIG__HOISTED_LET_BINDINGS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.HoistExpressionConfig",
                "hoisted_let_bindings",
            )
            .expect("missing field")
        });
        impl HoistExpressionConfig {
            pub fn hoisted_conditionals(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_HOISTEXPRESSIONCONFIG__HOISTED_CONDITIONALS
                    .get(self.as_object_ref())
            }
            pub fn hoisted_let_bindings(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_HOISTEXPRESSIONCONFIG__HOISTED_LET_BINDINGS
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(
            HoistIfThenElseConfig,
            "tir.transform.HoistIfThenElseConfig"
        );

        static FIELD_TIR_TRANSFORM_HOISTIFTHENELSECONFIG__SUPPORT_BLOCK_SCOPE_HOISTING: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.HoistIfThenElseConfig",
                "support_block_scope_hoisting",
            )
            .expect("missing field")
        });
        impl HoistIfThenElseConfig {
            pub fn support_block_scope_hoisting(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_HOISTIFTHENELSECONFIG__SUPPORT_BLOCK_SCOPE_HOISTING
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(
            InjectDoubleBufferConfig,
            "tir.transform.InjectDoubleBufferConfig"
        );

        static FIELD_TIR_TRANSFORM_INJECTDOUBLEBUFFERCONFIG__SPLIT_LOOP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.InjectDoubleBufferConfig",
                "split_loop",
            )
            .expect("missing field")
        });
        impl InjectDoubleBufferConfig {
            pub fn split_loop(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_INJECTDOUBLEBUFFERCONFIG__SPLIT_LOOP.get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(LoopPartitionConfig, "tir.transform.LoopPartitionConfig");

        static FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__PARTITION_CONST_LOOP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.LoopPartitionConfig",
                "partition_const_loop",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__NO_UNROLL_LOOP_WITH_EXTENT_ONE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.LoopPartitionConfig",
                "no_unroll_loop_with_extent_one",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__UNROLL_LOOP_WITH_PARTITION_HINT_NO_INTERVAL: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> = LazyLock::new(|| tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.LoopPartitionConfig", "unroll_loop_with_partition_hint_no_interval").expect("missing field"));
        impl LoopPartitionConfig {
            pub fn partition_const_loop(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__PARTITION_CONST_LOOP
                    .get(self.as_object_ref())
            }
            pub fn no_unroll_loop_with_extent_one(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__NO_UNROLL_LOOP_WITH_EXTENT_ONE
                    .get(self.as_object_ref())
            }
            pub fn unroll_loop_with_partition_hint_no_interval(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_LOOPPARTITIONCONFIG__UNROLL_LOOP_WITH_PARTITION_HINT_NO_INTERVAL
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(
            ReduceBranchingThroughOvercomputeConfig,
            "tir.transform.ReduceBranchingThroughOvercomputeConfig"
        );

        static FIELD_TIR_TRANSFORM_REDUCEBRANCHINGTHROUGHOVERCOMPUTECONFIG__USE_DATAFLOW_ANALYSIS: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> = LazyLock::new(|| tvm_ffi::object_wrapper::FieldGetter::new("tir.transform.ReduceBranchingThroughOvercomputeConfig", "use_dataflow_analysis").expect("missing field"));
        impl ReduceBranchingThroughOvercomputeConfig {
            pub fn use_dataflow_analysis(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_REDUCEBRANCHINGTHROUGHOVERCOMPUTECONFIG__USE_DATAFLOW_ANALYSIS
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(RemoveNoOpConfig, "tir.transform.RemoveNoOpConfig");

        static FIELD_TIR_TRANSFORM_REMOVENOOPCONFIG__USE_DATAFLOW_ANALYSIS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.RemoveNoOpConfig",
                "use_dataflow_analysis",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_REMOVENOOPCONFIG__MAX_SIMPLIFICATION_STEPS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.RemoveNoOpConfig",
                "max_simplification_steps",
            )
            .expect("missing field")
        });
        impl RemoveNoOpConfig {
            pub fn use_dataflow_analysis(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_REMOVENOOPCONFIG__USE_DATAFLOW_ANALYSIS
                    .get(self.as_object_ref())
            }
            pub fn max_simplification_steps(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_REMOVENOOPCONFIG__MAX_SIMPLIFICATION_STEPS
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(SimplifyConfig, "tir.transform.SimplifyConfig");

        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.SimplifyConfig",
                "transitively_prove_inequalities",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.SimplifyConfig",
                "propagate_knowns_to_prove_conditional",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS:
            LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.SimplifyConfig",
                "propagate_knowns_to_simplify_expressions",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.SimplifyConfig",
                "convert_boolean_to_and_of_ors",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.SimplifyConfig",
                "apply_constraints_to_boolean_branches",
            )
            .expect("missing field")
        });
        impl SimplifyConfig {
            pub fn transitively_prove_inequalities(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES
                    .get(self.as_object_ref())
            }
            pub fn propagate_knowns_to_prove_conditional(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL
                    .get(self.as_object_ref())
            }
            pub fn propagate_knowns_to_simplify_expressions(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS
                    .get(self.as_object_ref())
            }
            pub fn convert_boolean_to_and_of_ors(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS
                    .get(self.as_object_ref())
            }
            pub fn apply_constraints_to_boolean_branches(&self) -> Result<bool> {
                FIELD_TIR_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(UnrollLoopConfig, "tir.transform.UnrollLoopConfig");

        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "auto_max_step",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "auto_max_depth",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_EXTENT: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "auto_max_extent",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__EXPLICIT_UNROLL: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "explicit_unroll",
            )
            .expect("missing field")
        });
        static FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__UNROLL_LOCAL_ACCESS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tir.transform.UnrollLoopConfig",
                "unroll_local_access",
            )
            .expect("missing field")
        });
        impl UnrollLoopConfig {
            pub fn auto_max_step(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP.get(self.as_object_ref())
            }
            pub fn auto_max_depth(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH.get(self.as_object_ref())
            }
            pub fn auto_max_extent(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_EXTENT.get(self.as_object_ref())
            }
            pub fn explicit_unroll(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__EXPLICIT_UNROLL.get(self.as_object_ref())
            }
            pub fn unroll_local_access(&self) -> Result<i64> {
                FIELD_TIR_TRANSFORM_UNROLLLOOPCONFIG__UNROLL_LOCAL_ACCESS.get(self.as_object_ref())
            }
        }
    }
}
pub mod tl {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    tvm_ffi::define_object_wrapper!(KernelLaunchFrame, "tl.KernelLaunchFrame");

    static FIELD_TL_KERNELLAUNCHFRAME__FRAMES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.KernelLaunchFrame", "frames")
            .expect("missing field")
    });
    impl KernelLaunchFrame {
        pub fn frames(&self) -> Result<tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame>> {
            FIELD_TL_KERNELLAUNCHFRAME__FRAMES.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(WarpSpecializeFrame, "tl.WarpSpecializeFrame");

    static FIELD_TL_WARPSPECIALIZEFRAME__FRAMES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.WarpSpecializeFrame", "frames")
            .expect("missing field")
    });
    impl WarpSpecializeFrame {
        pub fn frames(&self) -> Result<tvm_ffi::Array<crate::script::ir_builder::tir::TIRFrame>> {
            FIELD_TL_WARPSPECIALIZEFRAME__FRAMES.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Layout, "tl.Layout");

    static FIELD_TL_LAYOUT__INPUT_SIZE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Layout", "input_size").expect("missing field")
    });
    static FIELD_TL_LAYOUT__FORWARD_INDEX: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Layout", "forward_index")
            .expect("missing field")
    });
    static METHOD_TL_LAYOUT___DEBUGOUTPUT: LazyLock<tvm_ffi::Function> = LazyLock::new(|| {
        tvm_ffi::object_wrapper::resolve_type_method("tl.Layout", "_DebugOutput")
            .expect("missing type method")
    });
    impl Layout {
        pub fn input_size(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TL_LAYOUT__INPUT_SIZE.get(self.as_object_ref())
        }
        pub fn forward_index(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TL_LAYOUT__FORWARD_INDEX.get(self.as_object_ref())
        }
        pub fn _DebugOutput(&self, args: &[Any]) -> Result<Any> {
            let func = &*METHOD_TL_LAYOUT___DEBUGOUTPUT;
            let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
            views.push(AnyView::from(self.as_object_ref()));
            views.extend(args.iter().map(AnyView::from));
            func.call_packed(&views)
        }
    }

    tvm_ffi::define_object_wrapper!(Fragment, "tl.Fragment");

    static FIELD_TL_FRAGMENT__FORWARD_THREAD: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Fragment", "forward_thread")
            .expect("missing field")
    });
    static FIELD_TL_FRAGMENT__REPLICATE_SIZE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Fragment", "replicate_size")
            .expect("missing field")
    });
    static METHOD_TL_FRAGMENT___DEBUGOUTPUT: LazyLock<tvm_ffi::Function> = LazyLock::new(|| {
        tvm_ffi::object_wrapper::resolve_type_method("tl.Fragment", "_DebugOutput")
            .expect("missing type method")
    });
    impl Fragment {
        pub fn forward_thread(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_FRAGMENT__FORWARD_THREAD.get(self.as_object_ref())
        }
        pub fn replicate_size(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_FRAGMENT__REPLICATE_SIZE.get(self.as_object_ref())
        }
        pub fn _DebugOutput(&self, args: &[Any]) -> Result<Any> {
            let func = &*METHOD_TL_FRAGMENT___DEBUGOUTPUT;
            let mut views: Vec<AnyView<'_>> = Vec::with_capacity(args.len() + 1);
            views.push(AnyView::from(self.as_object_ref()));
            views.extend(args.iter().map(AnyView::from));
            func.call_packed(&views)
        }
    }

    tvm_ffi::define_object_wrapper!(TileOperator, "tl.TileOperator");

    impl TileOperator {}

    tvm_ffi::define_object_wrapper!(AtomicAdd, "tl.AtomicAdd");

    static FIELD_TL_ATOMICADD__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "src").expect("missing field")
    });
    static FIELD_TL_ATOMICADD__SRC_VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "src_value")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICADD__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "dst").expect("missing field")
    });
    static FIELD_TL_ATOMICADD__SRC_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "src_range")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICADD__DST_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "dst_range")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICADD__ANNOTATIONS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicAdd", "annotations")
            .expect("missing field")
    });
    impl AtomicAdd {
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_ATOMICADD__SRC.get(self.as_object_ref())
        }
        pub fn src_value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_ATOMICADD__SRC_VALUE.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_ATOMICADD__DST.get(self.as_object_ref())
        }
        pub fn src_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_ATOMICADD__SRC_RANGE.get(self.as_object_ref())
        }
        pub fn dst_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_ATOMICADD__DST_RANGE.get(self.as_object_ref())
        }
        pub fn annotations(
            &self,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>> {
            FIELD_TL_ATOMICADD__ANNOTATIONS.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(AtomicMax, "tl.AtomicMax");

    static FIELD_TL_ATOMICMAX__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "src").expect("missing field")
    });
    static FIELD_TL_ATOMICMAX__SRC_VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "src_value")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICMAX__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "dst").expect("missing field")
    });
    static FIELD_TL_ATOMICMAX__SRC_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "src_range")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICMAX__DST_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "dst_range")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICMAX__ANNOTATIONS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMax", "annotations")
            .expect("missing field")
    });
    impl AtomicMax {
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_ATOMICMAX__SRC.get(self.as_object_ref())
        }
        pub fn src_value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_ATOMICMAX__SRC_VALUE.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_ATOMICMAX__DST.get(self.as_object_ref())
        }
        pub fn src_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_ATOMICMAX__SRC_RANGE.get(self.as_object_ref())
        }
        pub fn dst_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_ATOMICMAX__DST_RANGE.get(self.as_object_ref())
        }
        pub fn annotations(
            &self,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>> {
            FIELD_TL_ATOMICMAX__ANNOTATIONS.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(AtomicMin, "tl.AtomicMin");

    static FIELD_TL_ATOMICMIN__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "src").expect("missing field")
    });
    static FIELD_TL_ATOMICMIN__SRC_VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "src_value")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICMIN__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "dst").expect("missing field")
    });
    static FIELD_TL_ATOMICMIN__SRC_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "src_range")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICMIN__DST_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "dst_range")
            .expect("missing field")
    });
    static FIELD_TL_ATOMICMIN__ANNOTATIONS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.AtomicMin", "annotations")
            .expect("missing field")
    });
    impl AtomicMin {
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_ATOMICMIN__SRC.get(self.as_object_ref())
        }
        pub fn src_value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_ATOMICMIN__SRC_VALUE.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_ATOMICMIN__DST.get(self.as_object_ref())
        }
        pub fn src_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_ATOMICMIN__SRC_RANGE.get(self.as_object_ref())
        }
        pub fn dst_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_ATOMICMIN__DST_RANGE.get(self.as_object_ref())
        }
        pub fn annotations(
            &self,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>> {
            FIELD_TL_ATOMICMIN__ANNOTATIONS.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Copy, "tl.Copy");

    static FIELD_TL_COPY__SRC: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Copy", "src").expect("missing field")
        });
    static FIELD_TL_COPY__DST: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Copy", "dst").expect("missing field")
        });
    static FIELD_TL_COPY__SRC_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Copy", "src_range").expect("missing field")
    });
    static FIELD_TL_COPY__DST_RANGE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Copy", "dst_range").expect("missing field")
    });
    static FIELD_TL_COPY__ANNOTATIONS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<
            tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>,
        >,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Copy", "annotations").expect("missing field")
    });
    impl Copy {
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_COPY__SRC.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_COPY__DST.get(self.as_object_ref())
        }
        pub fn src_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_COPY__SRC_RANGE.get(self.as_object_ref())
        }
        pub fn dst_range(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_COPY__DST_RANGE.get(self.as_object_ref())
        }
        pub fn annotations(
            &self,
        ) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::object::ObjectRef>> {
            FIELD_TL_COPY__ANNOTATIONS.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Conv2DIm2Col, "tl.Conv2DIm2Col");

    static FIELD_TL_CONV2DIM2COL__SRCREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "srcRegion")
            .expect("missing field")
    });
    static FIELD_TL_CONV2DIM2COL__DSTREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "dstRegion")
            .expect("missing field")
    });
    static FIELD_TL_CONV2DIM2COL__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "src").expect("missing field")
    });
    static FIELD_TL_CONV2DIM2COL__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "dst").expect("missing field")
    });
    static FIELD_TL_CONV2DIM2COL__STRIDE: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "stride")
                .expect("missing field")
        });
    static FIELD_TL_CONV2DIM2COL__PADDING: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "padding")
                .expect("missing field")
        });
    static FIELD_TL_CONV2DIM2COL__DILATION: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "dilation")
                .expect("missing field")
        });
    static FIELD_TL_CONV2DIM2COL__KERNEL: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "kernel")
                .expect("missing field")
        });
    static FIELD_TL_CONV2DIM2COL__EVICTION_POLICY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i64>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Conv2DIm2Col", "eviction_policy")
            .expect("missing field")
    });
    impl Conv2DIm2Col {
        pub fn srcRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_CONV2DIM2COL__SRCREGION.get(self.as_object_ref())
        }
        pub fn dstRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_CONV2DIM2COL__DSTREGION.get(self.as_object_ref())
        }
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_CONV2DIM2COL__SRC.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_CONV2DIM2COL__DST.get(self.as_object_ref())
        }
        pub fn stride(&self) -> Result<i64> {
            FIELD_TL_CONV2DIM2COL__STRIDE.get(self.as_object_ref())
        }
        pub fn padding(&self) -> Result<i64> {
            FIELD_TL_CONV2DIM2COL__PADDING.get(self.as_object_ref())
        }
        pub fn dilation(&self) -> Result<i64> {
            FIELD_TL_CONV2DIM2COL__DILATION.get(self.as_object_ref())
        }
        pub fn kernel(&self) -> Result<i64> {
            FIELD_TL_CONV2DIM2COL__KERNEL.get(self.as_object_ref())
        }
        pub fn eviction_policy(&self) -> Result<i64> {
            FIELD_TL_CONV2DIM2COL__EVICTION_POLICY.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Fill, "tl.Fill");

    static FIELD_TL_FILL__DST: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Fill", "dst").expect("missing field")
        });
    static FIELD_TL_FILL__VALUE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Fill", "value").expect("missing field")
    });
    static FIELD_TL_FILL__REGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Fill", "region").expect("missing field")
    });
    impl Fill {
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_FILL__DST.get(self.as_object_ref())
        }
        pub fn value(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_FILL__VALUE.get(self.as_object_ref())
        }
        pub fn region(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_FILL__REGION.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(FinalizeReducerOp, "tl.FinalizeReducerOp");

    static FIELD_TL_FINALIZEREDUCEROP__REDUCER: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.FinalizeReducerOp", "reducer")
            .expect("missing field")
    });
    static FIELD_TL_FINALIZEREDUCEROP__OP: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.FinalizeReducerOp", "op")
                .expect("missing field")
        });
    impl FinalizeReducerOp {
        pub fn reducer(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_FINALIZEREDUCEROP__REDUCER.get(self.as_object_ref())
        }
        pub fn op(&self) -> Result<i64> {
            FIELD_TL_FINALIZEREDUCEROP__OP.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(Gemm, "tl.Gemm");

    static FIELD_TL_GEMM__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "a").expect("missing field")
        });
    static FIELD_TL_GEMM__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "b").expect("missing field")
        });
    static FIELD_TL_GEMM__C: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "c").expect("missing field")
        });
    static FIELD_TL_GEMM__AREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "aRegion").expect("missing field")
    });
    static FIELD_TL_GEMM__BREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "bRegion").expect("missing field")
    });
    static FIELD_TL_GEMM__CREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "cRegion").expect("missing field")
    });
    static FIELD_TL_GEMM__TRANSA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "transA").expect("missing field")
        });
    static FIELD_TL_GEMM__TRANSB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "transB").expect("missing field")
        });
    static FIELD_TL_GEMM__M: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "m").expect("missing field")
        });
    static FIELD_TL_GEMM__N: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "n").expect("missing field")
        });
    static FIELD_TL_GEMM__K: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "k").expect("missing field")
        });
    static FIELD_TL_GEMM__STRIDEA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "strideA").expect("missing field")
        });
    static FIELD_TL_GEMM__STRIDEB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "strideB").expect("missing field")
        });
    static FIELD_TL_GEMM__OFFSETA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "offsetA").expect("missing field")
        });
    static FIELD_TL_GEMM__OFFSETB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "offsetB").expect("missing field")
        });
    static FIELD_TL_GEMM__CLEARACCUM: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "clearAccum").expect("missing field")
    });
    static FIELD_TL_GEMM__KPACK: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "kPack").expect("missing field")
        });
    static FIELD_TL_GEMM__WGWAIT: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "wgWait").expect("missing field")
        });
    static FIELD_TL_GEMM__POLICY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tl::GemmWarpPolicy>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.Gemm", "policy").expect("missing field")
    });
    impl Gemm {
        pub fn a(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMM__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMM__B.get(self.as_object_ref())
        }
        pub fn c(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMM__C.get(self.as_object_ref())
        }
        pub fn aRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMM__AREGION.get(self.as_object_ref())
        }
        pub fn bRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMM__BREGION.get(self.as_object_ref())
        }
        pub fn cRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMM__CREGION.get(self.as_object_ref())
        }
        pub fn transA(&self) -> Result<bool> {
            FIELD_TL_GEMM__TRANSA.get(self.as_object_ref())
        }
        pub fn transB(&self) -> Result<bool> {
            FIELD_TL_GEMM__TRANSB.get(self.as_object_ref())
        }
        pub fn m(&self) -> Result<i64> {
            FIELD_TL_GEMM__M.get(self.as_object_ref())
        }
        pub fn n(&self) -> Result<i64> {
            FIELD_TL_GEMM__N.get(self.as_object_ref())
        }
        pub fn k(&self) -> Result<i64> {
            FIELD_TL_GEMM__K.get(self.as_object_ref())
        }
        pub fn strideA(&self) -> Result<i64> {
            FIELD_TL_GEMM__STRIDEA.get(self.as_object_ref())
        }
        pub fn strideB(&self) -> Result<i64> {
            FIELD_TL_GEMM__STRIDEB.get(self.as_object_ref())
        }
        pub fn offsetA(&self) -> Result<i64> {
            FIELD_TL_GEMM__OFFSETA.get(self.as_object_ref())
        }
        pub fn offsetB(&self) -> Result<i64> {
            FIELD_TL_GEMM__OFFSETB.get(self.as_object_ref())
        }
        pub fn clearAccum(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_GEMM__CLEARACCUM.get(self.as_object_ref())
        }
        pub fn kPack(&self) -> Result<i64> {
            FIELD_TL_GEMM__KPACK.get(self.as_object_ref())
        }
        pub fn wgWait(&self) -> Result<i64> {
            FIELD_TL_GEMM__WGWAIT.get(self.as_object_ref())
        }
        pub fn policy(&self) -> Result<crate::tl::GemmWarpPolicy> {
            FIELD_TL_GEMM__POLICY.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GemmWarpPolicy, "tl.GemmWarpPolicy");

    static FIELD_TL_GEMMWARPPOLICY__POLICY_TYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i64>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmWarpPolicy", "policy_type")
            .expect("missing field")
    });
    static FIELD_TL_GEMMWARPPOLICY__M_WARP: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmWarpPolicy", "m_warp")
                .expect("missing field")
        });
    static FIELD_TL_GEMMWARPPOLICY__N_WARP: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmWarpPolicy", "n_warp")
                .expect("missing field")
        });
    impl GemmWarpPolicy {
        pub fn policy_type(&self) -> Result<i64> {
            FIELD_TL_GEMMWARPPOLICY__POLICY_TYPE.get(self.as_object_ref())
        }
        pub fn m_warp(&self) -> Result<i64> {
            FIELD_TL_GEMMWARPPOLICY__M_WARP.get(self.as_object_ref())
        }
        pub fn n_warp(&self) -> Result<i64> {
            FIELD_TL_GEMMWARPPOLICY__N_WARP.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GemmPy, "tl.GemmPy");

    static FIELD_TL_GEMMPY__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "a").expect("missing field")
        });
    static FIELD_TL_GEMMPY__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "b").expect("missing field")
        });
    static FIELD_TL_GEMMPY__C: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "c").expect("missing field")
        });
    static FIELD_TL_GEMMPY__AREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "aRegion").expect("missing field")
    });
    static FIELD_TL_GEMMPY__BREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "bRegion").expect("missing field")
    });
    static FIELD_TL_GEMMPY__CREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "cRegion").expect("missing field")
    });
    static FIELD_TL_GEMMPY__TRANSA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "transA").expect("missing field")
        });
    static FIELD_TL_GEMMPY__TRANSB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "transB").expect("missing field")
        });
    static FIELD_TL_GEMMPY__M: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "m").expect("missing field")
        });
    static FIELD_TL_GEMMPY__N: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "n").expect("missing field")
        });
    static FIELD_TL_GEMMPY__K: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "k").expect("missing field")
        });
    static FIELD_TL_GEMMPY__STRIDEA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "strideA")
                .expect("missing field")
        });
    static FIELD_TL_GEMMPY__STRIDEB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "strideB")
                .expect("missing field")
        });
    static FIELD_TL_GEMMPY__OFFSETA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "offsetA")
                .expect("missing field")
        });
    static FIELD_TL_GEMMPY__OFFSETB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "offsetB")
                .expect("missing field")
        });
    static FIELD_TL_GEMMPY__CLEARACCUM: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "clearAccum").expect("missing field")
    });
    static FIELD_TL_GEMMPY__MBARREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "mbarRegion").expect("missing field")
    });
    static FIELD_TL_GEMMPY__MBAR: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "mbar").expect("missing field")
    });
    static FIELD_TL_GEMMPY__CCOORDS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "cCoords").expect("missing field")
    });
    static FIELD_TL_GEMMPY__KPACK: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "kPack").expect("missing field")
        });
    static FIELD_TL_GEMMPY__WGWAIT: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "wgWait").expect("missing field")
        });
    static FIELD_TL_GEMMPY__POLICY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tl::GemmWarpPolicy>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmPy", "policy").expect("missing field")
    });
    impl GemmPy {
        pub fn a(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMPY__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMPY__B.get(self.as_object_ref())
        }
        pub fn c(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMPY__C.get(self.as_object_ref())
        }
        pub fn aRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMPY__AREGION.get(self.as_object_ref())
        }
        pub fn bRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMPY__BREGION.get(self.as_object_ref())
        }
        pub fn cRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMPY__CREGION.get(self.as_object_ref())
        }
        pub fn transA(&self) -> Result<bool> {
            FIELD_TL_GEMMPY__TRANSA.get(self.as_object_ref())
        }
        pub fn transB(&self) -> Result<bool> {
            FIELD_TL_GEMMPY__TRANSB.get(self.as_object_ref())
        }
        pub fn m(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__M.get(self.as_object_ref())
        }
        pub fn n(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__N.get(self.as_object_ref())
        }
        pub fn k(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__K.get(self.as_object_ref())
        }
        pub fn strideA(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__STRIDEA.get(self.as_object_ref())
        }
        pub fn strideB(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__STRIDEB.get(self.as_object_ref())
        }
        pub fn offsetA(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__OFFSETA.get(self.as_object_ref())
        }
        pub fn offsetB(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__OFFSETB.get(self.as_object_ref())
        }
        pub fn clearAccum(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_GEMMPY__CLEARACCUM.get(self.as_object_ref())
        }
        pub fn mbarRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMPY__MBARREGION.get(self.as_object_ref())
        }
        pub fn mbar(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMPY__MBAR.get(self.as_object_ref())
        }
        pub fn cCoords(&self) -> Result<tvm_ffi::Array<crate::ir::PrimExpr>> {
            FIELD_TL_GEMMPY__CCOORDS.get(self.as_object_ref())
        }
        pub fn kPack(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__KPACK.get(self.as_object_ref())
        }
        pub fn wgWait(&self) -> Result<i64> {
            FIELD_TL_GEMMPY__WGWAIT.get(self.as_object_ref())
        }
        pub fn policy(&self) -> Result<crate::tl::GemmWarpPolicy> {
            FIELD_TL_GEMMPY__POLICY.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GemmSP, "tl.GemmSP");

    static FIELD_TL_GEMMSP__POLICY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tl::GemmSPWarpPolicy>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "policy").expect("missing field")
    });
    static FIELD_TL_GEMMSP__AREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "aRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSP__BREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "bRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSP__CREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "cRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSP__EREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "eRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSP__A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "a").expect("missing field")
        });
    static FIELD_TL_GEMMSP__B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "b").expect("missing field")
        });
    static FIELD_TL_GEMMSP__C: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "c").expect("missing field")
        });
    static FIELD_TL_GEMMSP__E: LazyLock<tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "e").expect("missing field")
        });
    static FIELD_TL_GEMMSP__TRANSA: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "transA").expect("missing field")
        });
    static FIELD_TL_GEMMSP__TRANSB: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "transB").expect("missing field")
        });
    static FIELD_TL_GEMMSP__M: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "m").expect("missing field")
        });
    static FIELD_TL_GEMMSP__N: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "n").expect("missing field")
        });
    static FIELD_TL_GEMMSP__K: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "k").expect("missing field")
        });
    static FIELD_TL_GEMMSP__CLEARACCUM: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "clearAccum")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSP__KPACK: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "kPack").expect("missing field")
        });
    static FIELD_TL_GEMMSP__WGWAIT: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSP", "wgWait").expect("missing field")
        });
    impl GemmSP {
        pub fn policy(&self) -> Result<crate::tl::GemmSPWarpPolicy> {
            FIELD_TL_GEMMSP__POLICY.get(self.as_object_ref())
        }
        pub fn aRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSP__AREGION.get(self.as_object_ref())
        }
        pub fn bRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSP__BREGION.get(self.as_object_ref())
        }
        pub fn cRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSP__CREGION.get(self.as_object_ref())
        }
        pub fn eRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSP__EREGION.get(self.as_object_ref())
        }
        pub fn a(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSP__A.get(self.as_object_ref())
        }
        pub fn b(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSP__B.get(self.as_object_ref())
        }
        pub fn c(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSP__C.get(self.as_object_ref())
        }
        pub fn e(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSP__E.get(self.as_object_ref())
        }
        pub fn transA(&self) -> Result<bool> {
            FIELD_TL_GEMMSP__TRANSA.get(self.as_object_ref())
        }
        pub fn transB(&self) -> Result<bool> {
            FIELD_TL_GEMMSP__TRANSB.get(self.as_object_ref())
        }
        pub fn m(&self) -> Result<i64> {
            FIELD_TL_GEMMSP__M.get(self.as_object_ref())
        }
        pub fn n(&self) -> Result<i64> {
            FIELD_TL_GEMMSP__N.get(self.as_object_ref())
        }
        pub fn k(&self) -> Result<i64> {
            FIELD_TL_GEMMSP__K.get(self.as_object_ref())
        }
        pub fn clearAccum(&self) -> Result<bool> {
            FIELD_TL_GEMMSP__CLEARACCUM.get(self.as_object_ref())
        }
        pub fn kPack(&self) -> Result<i64> {
            FIELD_TL_GEMMSP__KPACK.get(self.as_object_ref())
        }
        pub fn wgWait(&self) -> Result<i64> {
            FIELD_TL_GEMMSP__WGWAIT.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GemmSPWarpPolicy, "tl.GemmSPWarpPolicy");

    static FIELD_TL_GEMMSPWARPPOLICY__POLICY_TYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i64>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPWarpPolicy", "policy_type")
            .expect("missing field")
    });
    static FIELD_TL_GEMMSPWARPPOLICY__M_WARP: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPWarpPolicy", "m_warp")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPWARPPOLICY__N_WARP: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPWarpPolicy", "n_warp")
                .expect("missing field")
        });
    impl GemmSPWarpPolicy {
        pub fn policy_type(&self) -> Result<i64> {
            FIELD_TL_GEMMSPWARPPOLICY__POLICY_TYPE.get(self.as_object_ref())
        }
        pub fn m_warp(&self) -> Result<i64> {
            FIELD_TL_GEMMSPWARPPOLICY__M_WARP.get(self.as_object_ref())
        }
        pub fn n_warp(&self) -> Result<i64> {
            FIELD_TL_GEMMSPWARPPOLICY__N_WARP.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(GemmSPPy, "tl.GemmSPPy");

    static FIELD_TL_GEMMSPPY__A: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "A").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__E: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "E").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__B: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "B").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__C: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "C").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__AREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "aRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__EREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "eRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__BREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "bRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__CREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "cRegion").expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__TRANS_A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "trans_A")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__TRANS_B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "trans_B")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__TRANS_E: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "trans_E")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__M: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "M").expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__N: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "N").expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__K: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "K").expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__STRIDE_A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "stride_A")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__STRIDE_B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "stride_B")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__OFFSET_A: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "offset_A")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__OFFSET_B: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "offset_B")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__CLEAR_ACCUM: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::ir::PrimExpr>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "clear_accum")
            .expect("missing field")
    });
    static FIELD_TL_GEMMSPPY__KPACK: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "kPack")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__WG_WAIT: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "wg_wait")
                .expect("missing field")
        });
    static FIELD_TL_GEMMSPPY__POLICY: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tl::GemmWarpPolicy>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.GemmSPPy", "policy").expect("missing field")
    });
    impl GemmSPPy {
        pub fn A(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSPPY__A.get(self.as_object_ref())
        }
        pub fn E(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSPPY__E.get(self.as_object_ref())
        }
        pub fn B(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSPPY__B.get(self.as_object_ref())
        }
        pub fn C(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_GEMMSPPY__C.get(self.as_object_ref())
        }
        pub fn aRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSPPY__AREGION.get(self.as_object_ref())
        }
        pub fn eRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSPPY__EREGION.get(self.as_object_ref())
        }
        pub fn bRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSPPY__BREGION.get(self.as_object_ref())
        }
        pub fn cRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_GEMMSPPY__CREGION.get(self.as_object_ref())
        }
        pub fn trans_A(&self) -> Result<bool> {
            FIELD_TL_GEMMSPPY__TRANS_A.get(self.as_object_ref())
        }
        pub fn trans_B(&self) -> Result<bool> {
            FIELD_TL_GEMMSPPY__TRANS_B.get(self.as_object_ref())
        }
        pub fn trans_E(&self) -> Result<bool> {
            FIELD_TL_GEMMSPPY__TRANS_E.get(self.as_object_ref())
        }
        pub fn M(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__M.get(self.as_object_ref())
        }
        pub fn N(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__N.get(self.as_object_ref())
        }
        pub fn K(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__K.get(self.as_object_ref())
        }
        pub fn stride_A(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__STRIDE_A.get(self.as_object_ref())
        }
        pub fn stride_B(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__STRIDE_B.get(self.as_object_ref())
        }
        pub fn offset_A(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__OFFSET_A.get(self.as_object_ref())
        }
        pub fn offset_B(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__OFFSET_B.get(self.as_object_ref())
        }
        pub fn clear_accum(&self) -> Result<crate::ir::PrimExpr> {
            FIELD_TL_GEMMSPPY__CLEAR_ACCUM.get(self.as_object_ref())
        }
        pub fn kPack(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__KPACK.get(self.as_object_ref())
        }
        pub fn wg_wait(&self) -> Result<i64> {
            FIELD_TL_GEMMSPPY__WG_WAIT.get(self.as_object_ref())
        }
        pub fn policy(&self) -> Result<crate::tl::GemmWarpPolicy> {
            FIELD_TL_GEMMSPPY__POLICY.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(ParallelOp, "tl.ParallelOp");

    static FIELD_TL_PARALLELOP__ROOT: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::For>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ParallelOp", "root").expect("missing field")
    });
    static FIELD_TL_PARALLELOP__LOOP_LAYOUT: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tl::Fragment>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ParallelOp", "loop_layout")
            .expect("missing field")
    });
    static FIELD_TL_PARALLELOP__PREDICATE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<Option<crate::ir::PrimExpr>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ParallelOp", "predicate")
            .expect("missing field")
    });
    impl ParallelOp {
        pub fn root(&self) -> Result<crate::tir::For> {
            FIELD_TL_PARALLELOP__ROOT.get(self.as_object_ref())
        }
        pub fn loop_layout(&self) -> Result<crate::tl::Fragment> {
            FIELD_TL_PARALLELOP__LOOP_LAYOUT.get(self.as_object_ref())
        }
        pub fn predicate(&self) -> Result<Option<crate::ir::PrimExpr>> {
            FIELD_TL_PARALLELOP__PREDICATE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(ReduceOp, "tl.ReduceOp");

    static FIELD_TL_REDUCEOP__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "src").expect("missing field")
    });
    static FIELD_TL_REDUCEOP__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "dst").expect("missing field")
    });
    static FIELD_TL_REDUCEOP__SRCREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "srcRegion")
            .expect("missing field")
    });
    static FIELD_TL_REDUCEOP__DSTREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "dstRegion")
            .expect("missing field")
    });
    static FIELD_TL_REDUCEOP__DIM: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "dim").expect("missing field")
        });
    static FIELD_TL_REDUCEOP__TYPE: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tl::ReduceType>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "type").expect("missing field")
    });
    static FIELD_TL_REDUCEOP__CLEAR: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceOp", "clear")
                .expect("missing field")
        });
    impl ReduceOp {
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_REDUCEOP__SRC.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_REDUCEOP__DST.get(self.as_object_ref())
        }
        pub fn srcRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_REDUCEOP__SRCREGION.get(self.as_object_ref())
        }
        pub fn dstRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_REDUCEOP__DSTREGION.get(self.as_object_ref())
        }
        pub fn dim(&self) -> Result<i64> {
            FIELD_TL_REDUCEOP__DIM.get(self.as_object_ref())
        }
        pub fn type_(&self) -> Result<crate::tl::ReduceType> {
            FIELD_TL_REDUCEOP__TYPE.get(self.as_object_ref())
        }
        pub fn clear(&self) -> Result<bool> {
            FIELD_TL_REDUCEOP__CLEAR.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(CumSumOp, "tl.CumSumOp");

    static FIELD_TL_CUMSUMOP__SRC: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "src").expect("missing field")
    });
    static FIELD_TL_CUMSUMOP__DST: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "dst").expect("missing field")
    });
    static FIELD_TL_CUMSUMOP__SRCREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "srcRegion")
            .expect("missing field")
    });
    static FIELD_TL_CUMSUMOP__DSTREGION: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::BufferRegion>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "dstRegion")
            .expect("missing field")
    });
    static FIELD_TL_CUMSUMOP__DIM: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "dim").expect("missing field")
        });
    static FIELD_TL_CUMSUMOP__REVERSE: LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.CumSumOp", "reverse")
                .expect("missing field")
        });
    impl CumSumOp {
        pub fn src(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_CUMSUMOP__SRC.get(self.as_object_ref())
        }
        pub fn dst(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_CUMSUMOP__DST.get(self.as_object_ref())
        }
        pub fn srcRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_CUMSUMOP__SRCREGION.get(self.as_object_ref())
        }
        pub fn dstRegion(&self) -> Result<crate::tir::BufferRegion> {
            FIELD_TL_CUMSUMOP__DSTREGION.get(self.as_object_ref())
        }
        pub fn dim(&self) -> Result<i64> {
            FIELD_TL_CUMSUMOP__DIM.get(self.as_object_ref())
        }
        pub fn reverse(&self) -> Result<bool> {
            FIELD_TL_CUMSUMOP__REVERSE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(ReduceType, "tl.ReduceType");

    static FIELD_TL_REDUCETYPE__TYPE: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.ReduceType", "type")
                .expect("missing field")
        });
    impl ReduceType {
        pub fn type_(&self) -> Result<i64> {
            FIELD_TL_REDUCETYPE__TYPE.get(self.as_object_ref())
        }
    }

    tvm_ffi::define_object_wrapper!(RegionOp, "tl.RegionOp");

    static FIELD_TL_REGIONOP__BUFFER: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<crate::tir::Buffer>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.RegionOp", "buffer").expect("missing field")
    });
    static FIELD_TL_REGIONOP__RANGES: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<crate::ir::Range>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("tl.RegionOp", "ranges").expect("missing field")
    });
    static FIELD_TL_REGIONOP__ACCESS_MASK: LazyLock<tvm_ffi::object_wrapper::FieldGetter<i64>> =
        LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new("tl.RegionOp", "access_mask")
                .expect("missing field")
        });
    impl RegionOp {
        pub fn buffer(&self) -> Result<crate::tir::Buffer> {
            FIELD_TL_REGIONOP__BUFFER.get(self.as_object_ref())
        }
        pub fn ranges(&self) -> Result<tvm_ffi::Array<crate::ir::Range>> {
            FIELD_TL_REGIONOP__RANGES.get(self.as_object_ref())
        }
        pub fn access_mask(&self) -> Result<i64> {
            FIELD_TL_REGIONOP__ACCESS_MASK.get(self.as_object_ref())
        }
    }

    pub mod transform {
        use std::sync::LazyLock;
        use tvm_ffi::{Any, AnyView, ObjectArc, Result};

        tvm_ffi::define_object_wrapper!(SimplifyConfig, "tl.transform.SimplifyConfig");

        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "transitively_prove_inequalities",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "propagate_knowns_to_prove_conditional",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS:
            LazyLock<tvm_ffi::object_wrapper::FieldGetter<bool>> = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "propagate_knowns_to_simplify_expressions",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "convert_boolean_to_and_of_ors",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "apply_constraints_to_boolean_branches",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__ENABLE_SIMPLIFY_LET_INLINE: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<bool>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.SimplifyConfig",
                "enable_simplify_let_inline",
            )
            .expect("missing field")
        });
        impl SimplifyConfig {
            pub fn transitively_prove_inequalities(&self) -> Result<bool> {
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__TRANSITIVELY_PROVE_INEQUALITIES
                    .get(self.as_object_ref())
            }
            pub fn propagate_knowns_to_prove_conditional(&self) -> Result<bool> {
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_PROVE_CONDITIONAL
                    .get(self.as_object_ref())
            }
            pub fn propagate_knowns_to_simplify_expressions(&self) -> Result<bool> {
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__PROPAGATE_KNOWNS_TO_SIMPLIFY_EXPRESSIONS
                    .get(self.as_object_ref())
            }
            pub fn convert_boolean_to_and_of_ors(&self) -> Result<bool> {
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__CONVERT_BOOLEAN_TO_AND_OF_ORS
                    .get(self.as_object_ref())
            }
            pub fn apply_constraints_to_boolean_branches(&self) -> Result<bool> {
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__APPLY_CONSTRAINTS_TO_BOOLEAN_BRANCHES
                    .get(self.as_object_ref())
            }
            pub fn enable_simplify_let_inline(&self) -> Result<bool> {
                FIELD_TL_TRANSFORM_SIMPLIFYCONFIG__ENABLE_SIMPLIFY_LET_INLINE
                    .get(self.as_object_ref())
            }
        }

        tvm_ffi::define_object_wrapper!(UnrollLoopConfig, "tl.transform.UnrollLoopConfig");

        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "auto_max_step",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "auto_max_depth",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_EXTENT: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "auto_max_extent",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__EXPLICIT_UNROLL: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "explicit_unroll",
            )
            .expect("missing field")
        });
        static FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__UNROLL_LOCAL_ACCESS: LazyLock<
            tvm_ffi::object_wrapper::FieldGetter<i64>,
        > = LazyLock::new(|| {
            tvm_ffi::object_wrapper::FieldGetter::new(
                "tl.transform.UnrollLoopConfig",
                "unroll_local_access",
            )
            .expect("missing field")
        });
        impl UnrollLoopConfig {
            pub fn auto_max_step(&self) -> Result<i64> {
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_STEP.get(self.as_object_ref())
            }
            pub fn auto_max_depth(&self) -> Result<i64> {
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_DEPTH.get(self.as_object_ref())
            }
            pub fn auto_max_extent(&self) -> Result<i64> {
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__AUTO_MAX_EXTENT.get(self.as_object_ref())
            }
            pub fn explicit_unroll(&self) -> Result<i64> {
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__EXPLICIT_UNROLL.get(self.as_object_ref())
            }
            pub fn unroll_local_access(&self) -> Result<i64> {
                FIELD_TL_TRANSFORM_UNROLLLOOPCONFIG__UNROLL_LOCAL_ACCESS.get(self.as_object_ref())
            }
        }
    }
}
pub mod transform {
    use std::sync::LazyLock;
    use tvm_ffi::{Any, AnyView, ObjectArc, Result};

    tvm_ffi::define_object_wrapper!(Pass, "transform.Pass");

    impl Pass {}

    tvm_ffi::define_object_wrapper!(PassContext, "transform.PassContext");

    static FIELD_TRANSFORM_PASSCONTEXT__OPT_LEVEL: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<i64>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "opt_level")
            .expect("missing field")
    });
    static FIELD_TRANSFORM_PASSCONTEXT__REQUIRED_PASS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<tvm_ffi::String>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "required_pass")
            .expect("missing field")
    });
    static FIELD_TRANSFORM_PASSCONTEXT__DISABLED_PASS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Array<tvm_ffi::String>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "disabled_pass")
            .expect("missing field")
    });
    static FIELD_TRANSFORM_PASSCONTEXT__INSTRUMENTS: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "instruments")
            .expect("missing field")
    });
    static FIELD_TRANSFORM_PASSCONTEXT__CONFIG: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "config")
            .expect("missing field")
    });
    static FIELD_TRANSFORM_PASSCONTEXT__DIAG_CTX: LazyLock<
        tvm_ffi::object_wrapper::FieldGetter<tvm_ffi::Any>,
    > = LazyLock::new(|| {
        tvm_ffi::object_wrapper::FieldGetter::new("transform.PassContext", "diag_ctx")
            .expect("missing field")
    });
    impl PassContext {
        pub fn opt_level(&self) -> Result<i64> {
            FIELD_TRANSFORM_PASSCONTEXT__OPT_LEVEL.get(self.as_object_ref())
        }
        pub fn required_pass(&self) -> Result<tvm_ffi::Array<tvm_ffi::String>> {
            FIELD_TRANSFORM_PASSCONTEXT__REQUIRED_PASS.get(self.as_object_ref())
        }
        pub fn disabled_pass(&self) -> Result<tvm_ffi::Array<tvm_ffi::String>> {
            FIELD_TRANSFORM_PASSCONTEXT__DISABLED_PASS.get(self.as_object_ref())
        }
        pub fn instruments(&self) -> Result<tvm_ffi::Any> {
            FIELD_TRANSFORM_PASSCONTEXT__INSTRUMENTS.get_any(self.as_object_ref())
        }
        pub fn config(&self) -> Result<tvm_ffi::Map<tvm_ffi::String, tvm_ffi::AnyValue>> {
            FIELD_TRANSFORM_PASSCONTEXT__CONFIG.get(self.as_object_ref())
        }
        pub fn diag_ctx(&self) -> Result<tvm_ffi::Any> {
            FIELD_TRANSFORM_PASSCONTEXT__DIAG_CTX.get_any(self.as_object_ref())
        }
    }

    #[repr(C)]
    #[derive(tvm_ffi::derive::Object)]
    #[type_key = "transform.PassInfo"]
    pub struct PassInfoObj {
        parent: tvm_ffi::object::Object,
        opt_level: i32,
        name: tvm_ffi::String,
        traceable: bool,
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
}
