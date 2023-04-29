use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerTransform3D<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerTransform3D<'a> {
    pub fn from_outer(outer: &Transform3D) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn inverse(&self) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3816817146i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn affine_inverse(&self) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("affine_inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3816817146i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn orthonormalized(&self) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("orthonormalized");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3816817146i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("rotated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1563203923i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&axis),
                <f64 as sys::GodotFfi>::sys_const(&angle),
            ];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rotated_local(&self, axis: Vector3, angle: f64) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("rotated_local");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1563203923i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&axis),
                <f64 as sys::GodotFfi>::sys_const(&angle),
            ];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn scaled(&self, scale: Vector3) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("scaled");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1405596198i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn scaled_local(&self, scale: Vector3) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("scaled_local");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1405596198i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn translated(&self, offset: Vector3) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("translated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1405596198i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn translated_local(&self, offset: Vector3) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("translated_local");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1405596198i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn looking_at(&self, target: Vector3, up: Vector3) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("looking_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                806929180i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&target),
                <Vector3 as sys::GodotFfi>::sys_const(&up),
            ];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn interpolate_with(&self, xform: Transform3D, weight: f64) -> Transform3D {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("interpolate_with");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1786453358i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Transform3D as sys::GodotFfi>::sys_const(&xform),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_equal_approx(&self, xform: Transform3D) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                696001652i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Transform3D as sys::GodotFfi>::sys_const(&xform)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D;
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
}
