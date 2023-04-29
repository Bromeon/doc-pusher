use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerCallable<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerCallable<'a> {
    pub fn from_outer(outer: &Callable) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn callv(&self, arguments: VariantArray) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("callv");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                413578926i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<VariantArray as sys::GodotFfi>::sys_const(&arguments)];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_null(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("is_null");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_custom(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("is_custom");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_standard(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("is_standard");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_valid(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("is_valid");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_object(&self) -> Option<Gd<Object>> {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("get_object");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4008621732i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Gd<Object>>::from_sys_init_opt(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_object_id(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("get_object_id");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3173160232i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_method(&self) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("get_method");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1825232092i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_bound_arguments_count(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("get_bound_arguments_count");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3173160232i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_bound_arguments(&self) -> VariantArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("get_bound_arguments");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4144163970i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn hash(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("hash");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3173160232i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bindv(&mut self, arguments: VariantArray) -> Callable {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("bindv");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3564560322i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<VariantArray as sys::GodotFfi>::sys_const(&arguments)];
            let __args_ptr = __args.as_ptr();
            <Callable as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn unbind(&self, argcount: i64) -> Callable {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("unbind");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                755001590i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&argcount)];
            let __args_ptr = __args.as_ptr();
            <Callable as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn call(&self, varargs: &[Variant]) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("call");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3643564216i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __explicit_args = [];
            let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __args.extend(__explicit_args.iter().map(Variant::sys_const));
            __args.extend(varargs.iter().map(Variant::sys_const));
            let __args_ptr = __args.as_ptr();
            let variant = Variant::from_sys_init_default(|return_ptr| {
                let mut __err = sys::default_call_error();
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
                if __err.error != sys::GDEXTENSION_CALL_OK {
                    let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                    __arg_types.extend(varargs.iter().map(Variant::get_type));
                    let __vararg_str = varargs
                        .iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    sys::panic_call_error(&__err, &format!("call(; {__vararg_str})"), &__arg_types);
                }
            });
            variant
        }
    }
    pub fn call_deferred(&self, varargs: &[Variant]) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("call_deferred");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3286317445i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __explicit_args = [];
            let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __args.extend(__explicit_args.iter().map(Variant::sys_const));
            __args.extend(varargs.iter().map(Variant::sys_const));
            let __args_ptr = __args.as_ptr();
            let mut __err = sys::default_call_error();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            if __err.error != sys::GDEXTENSION_CALL_OK {
                let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __arg_types.extend(varargs.iter().map(Variant::get_type));
                let __vararg_str = varargs
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                sys::panic_call_error(
                    &__err,
                    &format!("call_deferred(; {__vararg_str})"),
                    &__arg_types,
                );
            }
        }
    }
    pub fn rpc(&self, varargs: &[Variant]) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("rpc");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3286317445i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __explicit_args = [];
            let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __args.extend(__explicit_args.iter().map(Variant::sys_const));
            __args.extend(varargs.iter().map(Variant::sys_const));
            let __args_ptr = __args.as_ptr();
            let mut __err = sys::default_call_error();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            if __err.error != sys::GDEXTENSION_CALL_OK {
                let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __arg_types.extend(varargs.iter().map(Variant::get_type));
                let __vararg_str = varargs
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                sys::panic_call_error(&__err, &format!("rpc(; {__vararg_str})"), &__arg_types);
            }
        }
    }
    pub fn rpc_id(&self, peer_id: i64, varargs: &[Variant]) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("rpc_id");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2270047679i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __explicit_args = [<i64 as ToVariant>::to_variant(&peer_id)];
            let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __args.extend(__explicit_args.iter().map(Variant::sys_const));
            __args.extend(varargs.iter().map(Variant::sys_const));
            let __args_ptr = __args.as_ptr();
            let mut __err = sys::default_call_error();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            if __err.error != sys::GDEXTENSION_CALL_OK {
                let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __arg_types.extend(varargs.iter().map(Variant::get_type));
                let __vararg_str = varargs
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                sys::panic_call_error(
                    &__err,
                    &format!("rpc_id({peer_id:?}; {__vararg_str})"),
                    &__arg_types,
                );
            }
        }
    }
    pub fn bind(&self, varargs: &[Variant]) -> Callable {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_CALLABLE;
            let __method_name = StringName::from("bind");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3224143119i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __explicit_args = [];
            let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __args.extend(__explicit_args.iter().map(Variant::sys_const));
            __args.extend(varargs.iter().map(Variant::sys_const));
            let __args_ptr = __args.as_ptr();
            let variant = Variant::from_sys_init_default(|return_ptr| {
                let mut __err = sys::default_call_error();
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
                if __err.error != sys::GDEXTENSION_CALL_OK {
                    let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                    __arg_types.extend(varargs.iter().map(Variant::get_type));
                    let __vararg_str = varargs
                        .iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    sys::panic_call_error(&__err, &format!("bind(; {__vararg_str})"), &__arg_types);
                }
            });
            variant.to()
        }
    }
}
