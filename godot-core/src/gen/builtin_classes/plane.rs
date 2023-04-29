use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerPlane<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerPlane<'a> {
    pub fn from_outer(outer: &Plane) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn normalized(&self) -> Plane {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("normalized");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1051796340i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Plane as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_center(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("get_center");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1776574132i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_equal_approx(&self, to_plane: Plane) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1150170233i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Plane as sys::GodotFfi>::sys_const(&to_plane)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
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
    pub fn is_point_over(&self, point: Vector3) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("is_point_over");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1749054343i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&point)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn distance_to(&self, point: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("distance_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1047977935i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&point)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn has_point(&self, point: Vector3, tolerance: f64) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("has_point");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1258189072i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&point),
                <f64 as sys::GodotFfi>::sys_const(&tolerance),
            ];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn project(&self, point: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("project");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&point)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersect_3(&self, b: Plane, c: Plane) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("intersect_3");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2012052692i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Plane as sys::GodotFfi>::sys_const(&b),
                <Plane as sys::GodotFfi>::sys_const(&c),
            ];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("intersects_ray");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2048133369i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&from),
                <Vector3 as sys::GodotFfi>::sys_const(&dir),
            ];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PLANE;
            let __method_name = StringName::from("intersects_segment");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2048133369i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&from),
                <Vector3 as sys::GodotFfi>::sys_const(&to),
            ];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
