use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerPackedByteArray<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerPackedByteArray<'a> {
    pub fn from_outer(outer: &PackedByteArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
    pub fn append_array(&mut self, array: PackedByteArray) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("append_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                791097111i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&array)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn remove_at(&mut self, index: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
    pub fn slice(&self, begin: i64, end: i64) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("slice");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2278869132i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&begin),
                <i64 as sys::GodotFfi>::sys_const(&end),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn sort(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
    pub fn duplicate(&mut self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("duplicate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                851781288i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn find(&self, value: i64, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
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
    pub fn get_string_from_ascii(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("get_string_from_ascii");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3942272618i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_string_from_utf8(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("get_string_from_utf8");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3942272618i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_string_from_utf16(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("get_string_from_utf16");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3942272618i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_string_from_utf32(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("get_string_from_utf32");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3942272618i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn hex_encode(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("hex_encode");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3942272618i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn compress(&self, compression_mode: i64) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("compress");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1845905913i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&compression_mode)];
            let __args_ptr = __args.as_ptr();
            <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decompress(&self, buffer_size: i64, compression_mode: i64) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decompress");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2278869132i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&buffer_size),
                <i64 as sys::GodotFfi>::sys_const(&compression_mode),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decompress_dynamic(
        &self,
        max_output_size: i64,
        compression_mode: i64,
    ) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decompress_dynamic");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2278869132i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&max_output_size),
                <i64 as sys::GodotFfi>::sys_const(&compression_mode),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_u8(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_u8");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_s8(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_s8");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_u16(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_u16");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_s16(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_s16");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_u32(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_u32");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_s32(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_s32");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_u64(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_u64");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_s64(&self, byte_offset: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_s64");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_half(&self, byte_offset: i64) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_half");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1401583798i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_float(&self, byte_offset: i64) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_float");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1401583798i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_double(&self, byte_offset: i64) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_double");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1401583798i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&byte_offset)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn has_encoded_var(&self, byte_offset: i64, allow_objects: bool) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("has_encoded_var");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2914632957i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <bool as sys::GodotFfi>::sys_const(&allow_objects),
            ];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_var(&self, byte_offset: i64, allow_objects: bool) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_var");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1740420038i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <bool as sys::GodotFfi>::sys_const(&allow_objects),
            ];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn decode_var_size(&self, byte_offset: i64, allow_objects: bool) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("decode_var_size");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                954237325i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <bool as sys::GodotFfi>::sys_const(&allow_objects),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_int32_array(&self) -> PackedInt32Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("to_int32_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3158844420i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_int64_array(&self) -> PackedInt64Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("to_int64_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1961294120i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_float32_array(&self) -> PackedFloat32Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("to_float32_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3575107827i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedFloat32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_float64_array(&self) -> PackedFloat64Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("to_float64_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1627308337i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedFloat64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn encode_u8(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_u8");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_s8(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_s8");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_u16(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_u16");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_s16(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_s16");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_u32(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_u32");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_s32(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_s32");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_u64(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_u64");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_s64(&mut self, byte_offset: i64, value: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_s64");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3638975848i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <i64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_half(&mut self, byte_offset: i64, value: f64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_half");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1113000516i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <f64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_float(&mut self, byte_offset: i64, value: f64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_float");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1113000516i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <f64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_double(&mut self, byte_offset: i64, value: f64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_double");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1113000516i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <f64 as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn encode_var(&mut self, byte_offset: i64, value: Variant, allow_objects: bool) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY;
            let __method_name = StringName::from("encode_var");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2604460497i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&byte_offset),
                <Variant as sys::GodotFfi>::sys_const(&value),
                <bool as sys::GodotFfi>::sys_const(&allow_objects),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
