use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerNodePath<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerNodePath<'a> {
    pub fn from_outer(outer: &NodePath) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn is_absolute(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("is_absolute");
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
    pub fn get_name_count(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_name_count");
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
    pub fn get_name(&self, idx: i64) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_name");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2948586938i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
            let __args_ptr = __args.as_ptr();
            <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_subname_count(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_subname_count");
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
    pub fn hash(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
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
    pub fn get_subname(&self, idx: i64) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_subname");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2948586938i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
            let __args_ptr = __args.as_ptr();
            <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_concatenated_names(&self) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_concatenated_names");
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
    pub fn get_concatenated_subnames(&self) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_concatenated_subnames");
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
    pub fn get_as_property_path(&self) -> NodePath {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("get_as_property_path");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1598598043i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_empty(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_NODE_PATH;
            let __method_name = StringName::from("is_empty");
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
}
