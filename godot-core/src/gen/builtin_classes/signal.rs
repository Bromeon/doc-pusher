use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerSignal<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerSignal<'a> {
    pub fn from_outer(outer: &Signal) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn is_null(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
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
    pub fn get_object(&self) -> Option<Gd<Object>> {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
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
    pub fn get_name(&self) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
            let __method_name = StringName::from("get_name");
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
    pub fn connect(&mut self, callable: Callable, flags: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
            let __method_name = StringName::from("connect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                979702392i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Callable as sys::GodotFfi>::sys_const(&callable),
                <i64 as sys::GodotFfi>::sys_const(&flags),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn disconnect(&mut self, callable: Callable) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
            let __method_name = StringName::from("disconnect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3470848906i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&callable)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn is_connected(&self, callable: Callable) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
            let __method_name = StringName::from("is_connected");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4129521963i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&callable)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_connections(&self) -> VariantArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
            let __method_name = StringName::from("get_connections");
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
    pub fn emit(&self, varargs: &[Variant]) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_SIGNAL;
            let __method_name = StringName::from("emit");
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
                sys::panic_call_error(&__err, &format!("emit(; {__vararg_str})"), &__arg_types);
            }
        }
    }
}
