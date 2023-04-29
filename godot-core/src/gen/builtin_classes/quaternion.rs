use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerQuaternion<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerQuaternion<'a> {
    pub fn from_outer(outer: &Quaternion) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn length(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
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
    pub fn length_squared(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("length_squared");
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
    pub fn normalized(&self) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("normalized");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4274879941i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_normalized(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("is_normalized");
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
    pub fn is_equal_approx(&self, to: Quaternion) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1682156903i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Quaternion as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
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
    pub fn inverse(&self) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4274879941i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn log(&self) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("log");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4274879941i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn exp(&self) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("exp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4274879941i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn angle_to(&self, to: Quaternion) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("angle_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3244682419i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Quaternion as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn dot(&self, with: Quaternion) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("dot");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3244682419i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Quaternion as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slerp(&self, to: Quaternion, weight: f64) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("slerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1773590316i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Quaternion as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slerpni(&self, to: Quaternion, weight: f64) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("slerpni");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1773590316i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Quaternion as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn spherical_cubic_interpolate(
        &self,
        b: Quaternion,
        pre_a: Quaternion,
        post_b: Quaternion,
        weight: f64,
    ) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("spherical_cubic_interpolate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2150967576i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Quaternion as sys::GodotFfi>::sys_const(&b),
                <Quaternion as sys::GodotFfi>::sys_const(&pre_a),
                <Quaternion as sys::GodotFfi>::sys_const(&post_b),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn spherical_cubic_interpolate_in_time(
        &self,
        b: Quaternion,
        pre_a: Quaternion,
        post_b: Quaternion,
        weight: f64,
        b_t: f64,
        pre_a_t: f64,
        post_b_t: f64,
    ) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("spherical_cubic_interpolate_in_time");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1436023539i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Quaternion as sys::GodotFfi>::sys_const(&b),
                <Quaternion as sys::GodotFfi>::sys_const(&pre_a),
                <Quaternion as sys::GodotFfi>::sys_const(&post_b),
                <f64 as sys::GodotFfi>::sys_const(&weight),
                <f64 as sys::GodotFfi>::sys_const(&b_t),
                <f64 as sys::GodotFfi>::sys_const(&pre_a_t),
                <f64 as sys::GodotFfi>::sys_const(&post_b_t),
            ];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_euler(&self, order: i64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("get_euler");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1394941017i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&order)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn from_euler(euler: Vector3) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("from_euler");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4053467903i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&euler)];
            let __args_ptr = __args.as_ptr();
            <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn get_axis(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("get_axis");
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
    pub fn get_angle(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_QUATERNION;
            let __method_name = StringName::from("get_angle");
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
}
