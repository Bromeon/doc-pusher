use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerPackedInt64Array<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerPackedInt64Array<'a> {
    pub fn from_outer(outer: &PackedInt64Array) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("size");
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
    pub fn is_empty(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
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
    pub fn set(&mut self, index: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("set");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&index),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn push_back(&mut self, value: i64) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("push_back");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                694024632i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn append(&mut self, value: i64) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("append");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                694024632i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn append_array(&mut self, array: PackedInt64Array) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("append_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2090311302i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<PackedInt64Array as sys::GodotFfi>::sys_const(&array)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn remove_at(&mut self, index: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("remove_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2823966027i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn insert(&mut self, at_index: i64, value: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("insert");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1487112728i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&at_index),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn fill(&mut self, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("fill");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2823966027i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn resize(&mut self, new_size: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("resize");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                848867239i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&new_size)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn clear(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("clear");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn has(&self, value: i64) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("has");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                931488181i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn reverse(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("reverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn slice(&self, begin: i64, end: i64) -> PackedInt64Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("slice");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1726550804i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&begin),
                <i64 as sys::GodotFfi>::sys_const(&end),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_byte_array(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("to_byte_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                247621236i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn sort(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("sort");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn bsearch(&mut self, value: i64, before: bool) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("bsearch");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3380005890i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&value),
                <bool as sys::GodotFfi>::sys_const(&before),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn duplicate(&mut self) -> PackedInt64Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("duplicate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2376370016i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn find(&self, value: i64, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("find");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2984303840i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&value),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rfind(&self, value: i64, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("rfind");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2984303840i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&value),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn count(&self, value: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY;
            let __method_name = StringName::from("count");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
