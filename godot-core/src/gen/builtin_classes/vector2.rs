use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerVector2<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerVector2<'a> {
    pub fn from_outer(outer: &Vector2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn angle(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("angle");
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
    pub fn angle_to(&self, to: Vector2) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("angle_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3819070308i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn angle_to_point(&self, to: Vector2) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("angle_to_point");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3819070308i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn direction_to(&self, to: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("direction_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn distance_to(&self, to: Vector2) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("distance_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3819070308i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn distance_squared_to(&self, to: Vector2) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("distance_squared_to");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3819070308i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn length(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
    pub fn limit_length(&self, length: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("limit_length");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2544004089i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&length)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn normalized(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("normalized");
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
    pub fn is_normalized(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
    pub fn is_equal_approx(&self, to: Vector2) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3190634762i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_zero_approx(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
    pub fn posmod(&self, mod_: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("posmod");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2544004089i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&mod_)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn posmodv(&self, modv: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("posmodv");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&modv)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn project(&self, b: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("project");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn lerp(&self, to: Vector2, weight: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("lerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4250033116i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slerp(&self, to: Vector2, weight: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("slerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4250033116i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn cubic_interpolate(
        &self,
        b: Vector2,
        pre_a: Vector2,
        post_b: Vector2,
        weight: f64,
    ) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("cubic_interpolate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                193522989i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&b),
                <Vector2 as sys::GodotFfi>::sys_const(&pre_a),
                <Vector2 as sys::GodotFfi>::sys_const(&post_b),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn cubic_interpolate_in_time(
        &self,
        b: Vector2,
        pre_a: Vector2,
        post_b: Vector2,
        weight: f64,
        b_t: f64,
        pre_a_t: f64,
        post_b_t: f64,
    ) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("cubic_interpolate_in_time");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1957055074i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&b),
                <Vector2 as sys::GodotFfi>::sys_const(&pre_a),
                <Vector2 as sys::GodotFfi>::sys_const(&post_b),
                <f64 as sys::GodotFfi>::sys_const(&weight),
                <f64 as sys::GodotFfi>::sys_const(&b_t),
                <f64 as sys::GodotFfi>::sys_const(&pre_a_t),
                <f64 as sys::GodotFfi>::sys_const(&post_b_t),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bezier_interpolate(
        &self,
        control_1: Vector2,
        control_2: Vector2,
        end: Vector2,
        t: f64,
    ) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("bezier_interpolate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                193522989i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&control_1),
                <Vector2 as sys::GodotFfi>::sys_const(&control_2),
                <Vector2 as sys::GodotFfi>::sys_const(&end),
                <f64 as sys::GodotFfi>::sys_const(&t),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bezier_derivative(
        &self,
        control_1: Vector2,
        control_2: Vector2,
        end: Vector2,
        t: f64,
    ) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("bezier_derivative");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                193522989i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&control_1),
                <Vector2 as sys::GodotFfi>::sys_const(&control_2),
                <Vector2 as sys::GodotFfi>::sys_const(&end),
                <f64 as sys::GodotFfi>::sys_const(&t),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn max_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
    pub fn move_toward(&self, to: Vector2, delta: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("move_toward");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4250033116i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&delta),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rotated(&self, angle: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("rotated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2544004089i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn orthogonal(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("orthogonal");
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
    pub fn floor(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("floor");
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
    pub fn ceil(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("ceil");
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
    pub fn round(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("round");
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
    pub fn aspect(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
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
    pub fn dot(&self, with: Vector2) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("dot");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3819070308i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slide(&self, n: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("slide");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&n)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bounce(&self, n: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("bounce");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&n)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn reflect(&self, n: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("reflect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&n)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn cross(&self, with: Vector2) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("cross");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3819070308i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn abs(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("abs");
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
    pub fn sign(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("sign");
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
    pub fn clamp(&self, min: Vector2, max: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("clamp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                318031021i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector2 as sys::GodotFfi>::sys_const(&min),
                <Vector2 as sys::GodotFfi>::sys_const(&max),
            ];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn snapped(&self, step: Vector2) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("snapped");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2026743667i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&step)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn from_angle(angle: f64) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_VECTOR2;
            let __method_name = StringName::from("from_angle");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                889263119i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
            let __args_ptr = __args.as_ptr();
            <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
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
