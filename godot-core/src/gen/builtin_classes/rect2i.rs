use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerRect2i<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerRect2i<'a> {
    pub fn from_outer(outer: &Rect2i) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn get_center(&self) -> Vector2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("get_center");
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
    pub fn get_area(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("get_area");
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
    pub fn has_area(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
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
    pub fn has_point(&self, point: Vector2i) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("has_point");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                328189994i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2i as sys::GodotFfi>::sys_const(&point)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersects(&self, b: Rect2i) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("intersects");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3434691493i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2i as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn encloses(&self, b: Rect2i) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("encloses");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3434691493i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2i as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn intersection(&self, b: Rect2i) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("intersection");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                717431873i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2i as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn merge(&self, b: Rect2i) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("merge");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                717431873i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2i as sys::GodotFfi>::sys_const(&b)];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn expand(&self, to: Vector2i) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("expand");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1355196872i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2i as sys::GodotFfi>::sys_const(&to)];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow(&self, amount: i64) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("grow");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1578070074i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&amount)];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow_side(&self, side: i64, amount: i64) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("grow_side");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3191154199i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&side),
                <i64 as sys::GodotFfi>::sys_const(&amount),
            ];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn grow_individual(&self, left: i64, top: i64, right: i64, bottom: i64) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("grow_individual");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1893743416i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&left),
                <i64 as sys::GodotFfi>::sys_const(&top),
                <i64 as sys::GodotFfi>::sys_const(&right),
                <i64 as sys::GodotFfi>::sys_const(&bottom),
            ];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn abs(&self) -> Rect2i {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_RECT2I;
            let __method_name = StringName::from("abs");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1469025700i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
