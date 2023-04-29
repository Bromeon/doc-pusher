use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerPackedColorArray<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerPackedColorArray<'a> {
    pub fn from_outer(outer: &PackedColorArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
    pub fn set(&mut self, index: i64, value: Color) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("set");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1444096570i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&index),
                <Color as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn push_back(&mut self, value: Color) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("push_back");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1007858200i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn append(&mut self, value: Color) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("append");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1007858200i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn append_array(&mut self, array: PackedColorArray) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("append_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                798822497i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<PackedColorArray as sys::GodotFfi>::sys_const(&array)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn remove_at(&mut self, index: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
    pub fn insert(&mut self, at_index: i64, value: Color) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("insert");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                785289703i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&at_index),
                <Color as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn fill(&mut self, value: Color) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("fill");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3730314301i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn resize(&mut self, new_size: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
    pub fn has(&self, value: Color) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("has");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3167426256i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn reverse(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
    pub fn slice(&self, begin: i64, end: i64) -> PackedColorArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("slice");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2451797139i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&begin),
                <i64 as sys::GodotFfi>::sys_const(&end),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedColorArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_byte_array(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
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
    pub fn bsearch(&mut self, value: Color, before: bool) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("bsearch");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                314143821i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Color as sys::GodotFfi>::sys_const(&value),
                <bool as sys::GodotFfi>::sys_const(&before),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn duplicate(&mut self) -> PackedColorArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("duplicate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1011903421i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedColorArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn find(&self, value: Color, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("find");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3156095363i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Color as sys::GodotFfi>::sys_const(&value),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rfind(&self, value: Color, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("rfind");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3156095363i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Color as sys::GodotFfi>::sys_const(&value),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn count(&self, value: Color) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY;
            let __method_name = StringName::from("count");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1682108616i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
