use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerStringName<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerStringName<'a> {
    pub fn from_outer(outer: &StringName) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn casecmp_to(&self, to: GodotString) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("casecmp_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2920860731i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn nocasecmp_to(&self, to: GodotString) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("nocasecmp_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2920860731i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn naturalnocasecmp_to(&self, to: GodotString) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("naturalnocasecmp_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2920860731i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn length(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("length");
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
    pub fn substr(&self, from: i64, len: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("substr");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                787537301i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&from),
                <i64 as sys::GodotFfi>::sys_const(&len),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_slice(&self, delimiter: GodotString, slice: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_slice");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3535100402i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&delimiter),
                <i64 as sys::GodotFfi>::sys_const(&slice),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_slicec(&self, delimiter: i64, slice: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_slicec");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                787537301i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&delimiter),
                <i64 as sys::GodotFfi>::sys_const(&slice),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_slice_count(&self, delimiter: GodotString) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_slice_count");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2920860731i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&delimiter)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn find(&self, what: GodotString, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("find");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1760645412i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn count(&self, what: GodotString, from: i64, to: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("count");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2343087891i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
                <i64 as sys::GodotFfi>::sys_const(&to),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn countn(&self, what: GodotString, from: i64, to: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("countn");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2343087891i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
                <i64 as sys::GodotFfi>::sys_const(&to),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn findn(&self, what: GodotString, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("findn");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1760645412i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rfind(&self, what: GodotString, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("rfind");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1760645412i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rfindn(&self, what: GodotString, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("rfindn");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1760645412i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn match_(&self, expr: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("match");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&expr)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn matchn(&self, expr: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("matchn");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&expr)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn begins_with(&self, text: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("begins_with");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn ends_with(&self, text: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("ends_with");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_subsequence_of(&self, text: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_subsequence_of");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_subsequence_ofn(&self, text: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_subsequence_ofn");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bigrams(&self) -> PackedStringArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("bigrams");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                747180633i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn similarity(&self, text: GodotString) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("similarity");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2697460964i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn format(&self, values: Variant, placeholder: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("format");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3212199029i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Variant as sys::GodotFfi>::sys_const(&values),
                <GodotString as sys::GodotFfi>::sys_const(&placeholder),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn replace(&self, what: GodotString, forwhat: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("replace");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1340436205i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <GodotString as sys::GodotFfi>::sys_const(&forwhat),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn replacen(&self, what: GodotString, forwhat: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("replacen");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1340436205i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&what),
                <GodotString as sys::GodotFfi>::sys_const(&forwhat),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn repeat(&self, count: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("repeat");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2162347432i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&count)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn insert(&self, position: i64, what: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("insert");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                248737229i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&position),
                <GodotString as sys::GodotFfi>::sys_const(&what),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn capitalize(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("capitalize");
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
    pub fn to_camel_case(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_camel_case");
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
    pub fn to_pascal_case(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_pascal_case");
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
    pub fn to_snake_case(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_snake_case");
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
    pub fn split(
        &self,
        delimiter: GodotString,
        allow_empty: bool,
        maxsplit: i64,
    ) -> PackedStringArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("split");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1252735785i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&delimiter),
                <bool as sys::GodotFfi>::sys_const(&allow_empty),
                <i64 as sys::GodotFfi>::sys_const(&maxsplit),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rsplit(
        &self,
        delimiter: GodotString,
        allow_empty: bool,
        maxsplit: i64,
    ) -> PackedStringArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("rsplit");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1252735785i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&delimiter),
                <bool as sys::GodotFfi>::sys_const(&allow_empty),
                <i64 as sys::GodotFfi>::sys_const(&maxsplit),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn split_floats(&self, delimiter: GodotString, allow_empty: bool) -> PackedFloat64Array {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("split_floats");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2092079095i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&delimiter),
                <bool as sys::GodotFfi>::sys_const(&allow_empty),
            ];
            let __args_ptr = __args.as_ptr();
            <PackedFloat64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn join(&self, parts: PackedStringArray) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("join");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3595973238i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<PackedStringArray as sys::GodotFfi>::sys_const(&parts)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_upper(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_upper");
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
    pub fn to_lower(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_lower");
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
    pub fn left(&self, length: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("left");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2162347432i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&length)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn right(&self, length: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("right");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2162347432i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&length)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn strip_edges(&self, left: bool, right: bool) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("strip_edges");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                907855311i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <bool as sys::GodotFfi>::sys_const(&left),
                <bool as sys::GodotFfi>::sys_const(&right),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn strip_escapes(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("strip_escapes");
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
    pub fn lstrip(&self, chars: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("lstrip");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3134094431i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&chars)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rstrip(&self, chars: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("rstrip");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3134094431i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&chars)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_extension(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_extension");
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
    pub fn get_basename(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_basename");
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
    pub fn path_join(&self, file: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("path_join");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3134094431i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&file)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn unicode_at(&self, at: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("unicode_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&at)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn indent(&self, prefix: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("indent");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3134094431i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&prefix)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn dedent(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("dedent");
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
    pub fn md5_text(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("md5_text");
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
    pub fn sha1_text(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("sha1_text");
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
    pub fn sha256_text(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("sha256_text");
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
    pub fn md5_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("md5_buffer");
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
    pub fn sha1_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("sha1_buffer");
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
    pub fn sha256_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("sha256_buffer");
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
    pub fn is_empty(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
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
    pub fn contains(&self, what: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("contains");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2566493496i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&what)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_absolute_path(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_absolute_path");
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
    pub fn is_relative_path(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_relative_path");
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
    pub fn simplify_path(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("simplify_path");
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
    pub fn get_base_dir(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_base_dir");
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
    pub fn get_file(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("get_file");
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
    pub fn xml_escape(&self, escape_quotes: bool) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("xml_escape");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3429816538i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<bool as sys::GodotFfi>::sys_const(&escape_quotes)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn xml_unescape(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("xml_unescape");
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
    pub fn uri_encode(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("uri_encode");
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
    pub fn uri_decode(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("uri_decode");
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
    pub fn c_escape(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("c_escape");
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
    pub fn c_unescape(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("c_unescape");
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
    pub fn json_escape(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("json_escape");
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
    pub fn validate_node_name(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("validate_node_name");
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
    pub fn validate_filename(&self) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("validate_filename");
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
    pub fn is_valid_identifier(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_identifier");
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
    pub fn is_valid_int(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_int");
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
    pub fn is_valid_float(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_float");
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
    pub fn is_valid_hex_number(&self, with_prefix: bool) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_hex_number");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                593672999i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<bool as sys::GodotFfi>::sys_const(&with_prefix)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_valid_html_color(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_html_color");
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
    pub fn is_valid_ip_address(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_ip_address");
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
    pub fn is_valid_filename(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("is_valid_filename");
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
    pub fn to_int(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_int");
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
    pub fn to_float(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_float");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                466405837i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn hex_to_int(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("hex_to_int");
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
    pub fn bin_to_int(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("bin_to_int");
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
    pub fn lpad(&self, min_length: i64, character: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("lpad");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                248737229i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&min_length),
                <GodotString as sys::GodotFfi>::sys_const(&character),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rpad(&self, min_length: i64, character: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("rpad");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                248737229i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&min_length),
                <GodotString as sys::GodotFfi>::sys_const(&character),
            ];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn pad_decimals(&self, digits: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("pad_decimals");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2162347432i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&digits)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn pad_zeros(&self, digits: i64) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("pad_zeros");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2162347432i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&digits)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn trim_prefix(&self, prefix: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("trim_prefix");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3134094431i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&prefix)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn trim_suffix(&self, suffix: GodotString) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("trim_suffix");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3134094431i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&suffix)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn to_ascii_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_ascii_buffer");
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
    pub fn to_utf8_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_utf8_buffer");
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
    pub fn to_utf16_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_utf16_buffer");
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
    pub fn to_utf32_buffer(&self) -> PackedByteArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
            let __method_name = StringName::from("to_utf32_buffer");
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
    pub fn hash(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_STRING_NAME;
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
}
