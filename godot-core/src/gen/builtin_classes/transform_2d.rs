use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerTransform2D<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerTransform2D<'a> {
    pub fn from_outer(outer: &Transform2D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn inverse(&self) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1420440541i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn affine_inverse(&self) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("affine_inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1420440541i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_rotation(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("get_rotation");
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
    pub fn get_origin(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("get_origin");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2428350749i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_scale(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("get_scale");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2428350749i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_skew(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("get_skew");
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
    pub fn orthonormalized(&self) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("orthonormalized");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1420440541i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rotated(&self, angle: f64) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("rotated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                729597514i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rotated_local(&self, angle: f64) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("rotated_local");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                729597514i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn scaled(&self, scale: Vector2) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("scaled");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1446323263i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&scale)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn scaled_local(&self, scale: Vector2) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("scaled_local");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1446323263i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&scale)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn translated(&self, offset: Vector2) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("translated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1446323263i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&offset)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn translated_local(&self, offset: Vector2) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("translated_local");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1446323263i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&offset)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn basis_xform(&self, v: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("basis_xform");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&v)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn basis_xform_inv(&self, v: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("basis_xform_inv");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&v)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn interpolate_with(&self, xform: Transform2D, weight: f64) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("interpolate_with");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                359399686i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Transform2D as sys::GodotFfi>::sys_const(&xform),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_equal_approx(&self, xform: Transform2D) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3837431929i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Transform2D as sys::GodotFfi>::sys_const(&xform)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("is_finite");
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
    pub fn looking_at(&self, target: Vector2) -> Transform2D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D;
            let __method_name = StringName::from("looking_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1446323263i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&target)];
            let __args_ptr = __args.as_ptr();
            <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
