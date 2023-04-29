use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerBasis<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerBasis<'a> {
    pub fn from_outer(outer: &Basis) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn inverse(&self) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                594669093i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn transposed(&self) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("transposed");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                594669093i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn orthonormalized(&self) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("orthonormalized");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                594669093i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn determinant(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("determinant");
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
    pub fn rotated(&self, axis: Vector3, angle: f64) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("rotated");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1998708965i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&axis),
                <f64 as sys::GodotFfi>::sys_const(&angle),
            ];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn scaled(&self, scale: Vector3) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("scaled");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3934786792i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_scale(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("get_scale");
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
    pub fn get_euler(&self, order: i64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
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
    pub fn tdotx(&self, with: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("tdotx");
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
    pub fn tdoty(&self, with: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("tdoty");
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
    pub fn tdotz(&self, with: Vector3) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("tdotz");
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
    pub fn slerp(&self, to: Basis, weight: f64) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("slerp");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3118673011i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Basis as sys::GodotFfi>::sys_const(&to),
                <f64 as sys::GodotFfi>::sys_const(&weight),
            ];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_equal_approx(&self, b: Basis) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3165333982i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Basis as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
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
    pub fn get_rotation_quaternion(&self) -> Quaternion {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("get_rotation_quaternion");
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
    pub fn looking_at(target: Vector3, up: Vector3) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("looking_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                419916660i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&target),
                <Vector3 as sys::GodotFfi>::sys_const(&up),
            ];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn from_scale(scale: Vector3) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("from_scale");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3703240166i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn from_euler(euler: Vector3, order: i64) -> Basis {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_BASIS;
            let __method_name = StringName::from("from_euler");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2802321791i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Vector3 as sys::GodotFfi>::sys_const(&euler),
                <i64 as sys::GodotFfi>::sys_const(&order),
            ];
            let __args_ptr = __args.as_ptr();
            <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
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
