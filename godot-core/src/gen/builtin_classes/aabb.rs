use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerAabb<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerAabb<'a> {
    pub fn from_outer(outer: &Aabb) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn abs(&self) -> Aabb {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("abs");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1576868580i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_center(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
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
    pub fn get_volume(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_volume");
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
    pub fn has_volume(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("has_volume");
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
    pub fn has_surface(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("has_surface");
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
    pub fn has_point(&self, point: Vector3) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("has_point");
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
    pub fn is_equal_approx(&self, aabb: Aabb) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                299946684i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Aabb as sys::GodotFfi>::sys_const(&aabb)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
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
    pub fn intersects(&self, with: Aabb) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("intersects");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                299946684i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Aabb as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn encloses(&self, with: Aabb) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("encloses");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                299946684i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Aabb as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersects_plane(&self, plane: Plane) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("intersects_plane");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1150170233i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Plane as sys::GodotFfi>::sys_const(&plane)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersection(&self, with: Aabb) -> Aabb {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("intersection");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1271470306i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Aabb as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn merge(&self, with: Aabb) -> Aabb {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("merge");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1271470306i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Aabb as sys::GodotFfi>::sys_const(&with)];
            let __args_ptr = __args.as_ptr();
            <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn expand(&self, to_point: Vector3) -> Aabb {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("expand");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2851643018i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&to_point)];
            let __args_ptr = __args.as_ptr();
            <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow(&self, by: f64) -> Aabb {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("grow");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                239217291i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&by)];
            let __args_ptr = __args.as_ptr();
            <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_support(&self, dir: Vector3) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_support");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2923479887i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector3 as sys::GodotFfi>::sys_const(&dir)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_longest_axis(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_longest_axis");
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
    pub fn get_longest_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_longest_axis_index");
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
    pub fn get_longest_axis_size(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_longest_axis_size");
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
    pub fn get_shortest_axis(&self) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_shortest_axis");
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
    pub fn get_shortest_axis_index(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_shortest_axis_index");
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
    pub fn get_shortest_axis_size(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_shortest_axis_size");
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
    pub fn get_endpoint(&self, idx: i64) -> Vector3 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
            let __method_name = StringName::from("get_endpoint");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1394941017i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
            let __args_ptr = __args.as_ptr();
            <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersects_segment(&self, from: Vector3, to: Vector3) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
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
    pub fn intersects_ray(&self, from: Vector3, dir: Vector3) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_AABB;
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
}
