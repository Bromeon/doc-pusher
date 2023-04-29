use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerColor<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerColor<'a> {
    pub fn from_outer(outer: &Color) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn to_argb32(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_argb32");
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
    pub fn to_abgr32(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_abgr32");
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
    pub fn to_rgba32(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_rgba32");
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
    pub fn to_argb64(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_argb64");
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
    pub fn to_abgr64(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_abgr64");
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
    pub fn to_rgba64(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_rgba64");
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
    pub fn to_html(&self, with_alpha: bool) -> GodotString {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("to_html");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3429816538i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<bool as sys::GodotFfi>::sys_const(&with_alpha)];
            let __args_ptr = __args.as_ptr();
            <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn clamp(&self, min: Color, max: Color) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("clamp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                105651410i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Color as sys::GodotFfi>::sys_const(&min),
                <Color as sys::GodotFfi>::sys_const(&max),
            ];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn inverted(&self) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("inverted");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3334027602i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn lerp(&self, to: Color, weight: f64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("lerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                402949615i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Color as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn lightened(&self, amount: f64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("lightened");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1466039168i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&amount)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn darkened(&self, amount: f64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("darkened");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1466039168i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&amount)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn blend(&self, over: Color) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("blend");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3803690977i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&over)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_luminance(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("get_luminance");
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
    pub fn srgb_to_linear(&self) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("srgb_to_linear");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3334027602i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn linear_to_srgb(&self) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("linear_to_srgb");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3334027602i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_equal_approx(&self, to: Color) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3167426256i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Color as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn hex(hex: i64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("hex");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                351421375i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&hex)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn hex64(hex: i64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("hex64");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                351421375i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&hex)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn html(rgba: GodotString) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("html");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2500054655i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&rgba)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn html_is_valid(color: GodotString) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("html_is_valid");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2942997125i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<GodotString as sys::GodotFfi>::sys_const(&color)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn from_string(str: GodotString, default: Color) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("from_string");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3755044230i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <GodotString as sys::GodotFfi>::sys_const(&str),
                <Color as sys::GodotFfi>::sys_const(&default),
            ];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn from_hsv(h: f64, s: f64, v: f64, alpha: f64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("from_hsv");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1573799446i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&h),
                <f64 as sys::GodotFfi>::sys_const(&s),
                <f64 as sys::GodotFfi>::sys_const(&v),
                <f64 as sys::GodotFfi>::sys_const(&alpha),
            ];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn from_ok_hsl(h: f64, s: f64, l: f64, alpha: f64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("from_ok_hsl");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1573799446i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&h),
                <f64 as sys::GodotFfi>::sys_const(&s),
                <f64 as sys::GodotFfi>::sys_const(&l),
                <f64 as sys::GodotFfi>::sys_const(&alpha),
            ];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn from_rgbe9995(rgbe: i64) -> Color {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_COLOR;
            let __method_name = StringName::from("from_rgbe9995");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                351421375i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&rgbe)];
            let __args_ptr = __args.as_ptr();
            <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
}
