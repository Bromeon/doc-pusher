use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerVector3<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerVector3<'a> {
    pub fn from_outer(outer: &Vector3) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn min_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
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
    pub fn max_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
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
    pub fn angle_to(&self, to: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("angle_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1047977935i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn signed_angle_to(&self, to: Vector3, axis: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("signed_angle_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2781412522i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&to),
                <Vector3 as sys::GodotFfi>::sys_const(&axis),
            ];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn direction_to(&self, to: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("direction_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn distance_to(&self, to: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("distance_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1047977935i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn distance_squared_to(&self, to: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("distance_squared_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1047977935i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn length(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
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
    pub fn limit_length(&self, length: f64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("limit_length");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                514930144i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&length)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn normalized(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("normalized");
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
    pub fn is_normalized(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
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
    pub fn is_equal_approx(&self, to: Vector3) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1749054343i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_zero_approx(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("is_zero_approx");
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
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
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
    pub fn inverse(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("inverse");
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
    pub fn clamp(&self, min: Vector3, max: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("clamp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4145107892i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&min),
                <Vector3 as sys::GodotFfi>::sys_const(&max),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn snapped(&self, step: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("snapped");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&step)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rotated(&self, axis: Vector3, angle: f64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("rotated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1682608829i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&axis),
                <f64 as sys::GodotFfi>::sys_const(&angle),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn lerp(&self, to: Vector3, weight: f64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("lerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1682608829i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slerp(&self, to: Vector3, weight: f64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("slerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1682608829i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn cubic_interpolate(
        &self,
        b: Vector3,
        pre_a: Vector3,
        post_b: Vector3,
        weight: f64,
    ) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("cubic_interpolate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2597922253i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&b),
                <Vector3 as sys::GodotFfi>::sys_const(&pre_a),
                <Vector3 as sys::GodotFfi>::sys_const(&post_b),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn cubic_interpolate_in_time(
        &self,
        b: Vector3,
        pre_a: Vector3,
        post_b: Vector3,
        weight: f64,
        b_t: f64,
        pre_a_t: f64,
        post_b_t: f64,
    ) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("cubic_interpolate_in_time");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3256682901i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&b),
                <Vector3 as sys::GodotFfi>::sys_const(&pre_a),
                <Vector3 as sys::GodotFfi>::sys_const(&post_b),
                <f64 as sys::GodotFfi>::sys_const(&weight),
                <f64 as sys::GodotFfi>::sys_const(&b_t),
                <f64 as sys::GodotFfi>::sys_const(&pre_a_t),
                <f64 as sys::GodotFfi>::sys_const(&post_b_t),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bezier_interpolate(
        &self,
        control_1: Vector3,
        control_2: Vector3,
        end: Vector3,
        t: f64,
    ) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("bezier_interpolate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2597922253i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&control_1),
                <Vector3 as sys::GodotFfi>::sys_const(&control_2),
                <Vector3 as sys::GodotFfi>::sys_const(&end),
                <f64 as sys::GodotFfi>::sys_const(&t),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bezier_derivative(
        &self,
        control_1: Vector3,
        control_2: Vector3,
        end: Vector3,
        t: f64,
    ) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("bezier_derivative");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2597922253i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&control_1),
                <Vector3 as sys::GodotFfi>::sys_const(&control_2),
                <Vector3 as sys::GodotFfi>::sys_const(&end),
                <f64 as sys::GodotFfi>::sys_const(&t),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn move_toward(&self, to: Vector3, delta: f64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("move_toward");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1682608829i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&delta),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn dot(&self, with: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("dot");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1047977935i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn cross(&self, with: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("cross");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn outer(&self, with: Vector3) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("outer");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3934786792i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn abs(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("abs");
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
    pub fn floor(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("floor");
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
    pub fn ceil(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("ceil");
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
    pub fn round(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("round");
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
    pub fn posmod(&self, mod_: f64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("posmod");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                514930144i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&mod_)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn posmodv(&self, modv: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("posmodv");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&modv)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn project(&self, b: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("project");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slide(&self, n: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("slide");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&n)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bounce(&self, n: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("bounce");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&n)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn reflect(&self, n: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("reflect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&n)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn sign(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("sign");
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
    pub fn octahedron_encode(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("octahedron_encode");
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
    pub fn octahedron_decode(uv: Vector2) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR3;
            let __method_name = StringName::from("octahedron_decode");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3991820552i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&uv)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Axis {
    ord: i32,
}
impl Axis {
    pub const AXIS_X: Self = Self { ord: 0 };
    pub const AXIS_Y: Self = Self { ord: 1 };
    pub const AXIS_Z: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for Axis {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self { ord }),
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
