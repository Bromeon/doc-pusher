use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerRect2<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerRect2<'a> {
    pub fn from_outer(outer: &Rect2) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn get_center(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("get_center");
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
    pub fn get_area(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("get_area");
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
    pub fn has_area(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("has_area");
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
    pub fn has_point(&self, point: Vector2) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("has_point");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3190634762i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&point)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_equal_approx(&self, rect: Rect2) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("is_equal_approx");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1908192260i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2 as sys::GodotFfi>::sys_const(&rect)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_finite(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
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
    pub fn intersects(&self, b: Rect2, include_borders: bool) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("intersects");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                819294880i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Rect2 as sys::GodotFfi>::sys_const(&b),
                <bool as sys::GodotFfi>::sys_const(&include_borders),
            ];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn encloses(&self, b: Rect2) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("encloses");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1908192260i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2 as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersection(&self, b: Rect2) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("intersection");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2282977743i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2 as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn merge(&self, b: Rect2) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("merge");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2282977743i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2 as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn expand(&self, to: Vector2) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("expand");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                293272265i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow(&self, amount: f64) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("grow");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                39664498i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&amount)];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow_side(&self, side: i64, amount: f64) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("grow_side");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4177736158i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&side),
                <f64 as sys::GodotFfi>::sys_const(&amount),
            ];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow_individual(&self, left: f64, top: f64, right: f64, bottom: f64) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("grow_individual");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3203390369i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&left),
                <f64 as sys::GodotFfi>::sys_const(&top),
                <f64 as sys::GodotFfi>::sys_const(&right),
                <f64 as sys::GodotFfi>::sys_const(&bottom),
            ];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn abs(&self) -> Rect2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2;
            let __method_name = StringName::from("abs");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3107653634i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
