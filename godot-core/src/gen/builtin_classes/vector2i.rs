use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerVector2i<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerVector2i<'a> {
    pub fn from_outer(outer: &Vector2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn aspect(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("aspect");
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
    pub fn max_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("max_axis_index");
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
    pub fn min_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("min_axis_index");
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
    pub fn length(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("length");
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
    pub fn length_squared(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("length_squared");
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
    pub fn sign(&self) -> Vector2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("sign");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3444277866i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn abs(&self) -> Vector2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("abs");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3444277866i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn clamp(&self, min: Vector2i, max: Vector2i) -> Vector2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("clamp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                186568249i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2i as sys::GodotFfi>::sys_const(&min),
                <Vector2i as sys::GodotFfi>::sys_const(&max),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn snapped(&self, step: Vector2i) -> Vector2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2I;
            let __method_name = StringName::from("snapped");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1735278196i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2i as sys::GodotFfi>::sys_const(&step)];
            let __args_ptr = __args.as_ptr();
            <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Axis {
    ord: i32,
}
impl Axis {
    pub const AXIS_X: Self = Self { ord: 0 };
    pub const AXIS_Y: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for Axis {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Axis {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
