use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerProjection<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerProjection<'a> {
    pub fn from_outer(outer: &Projection) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn create_depth_correction(flip_y: bool) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_depth_correction");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1228516048i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<bool as sys::GodotFfi>::sys_const(&flip_y)];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_light_atlas_rect(rect: Rect2) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_light_atlas_rect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2654950662i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Rect2 as sys::GodotFfi>::sys_const(&rect)];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_perspective(
        fovy: f64,
        aspect: f64,
        z_near: f64,
        z_far: f64,
        flip_fov: bool,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_perspective");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                390915442i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&fovy),
                <f64 as sys::GodotFfi>::sys_const(&aspect),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
                <bool as sys::GodotFfi>::sys_const(&flip_fov),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_perspective_hmd(
        fovy: f64,
        aspect: f64,
        z_near: f64,
        z_far: f64,
        flip_fov: bool,
        eye: i64,
        intraocular_dist: f64,
        convergence_dist: f64,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_perspective_hmd");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2857674800i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&fovy),
                <f64 as sys::GodotFfi>::sys_const(&aspect),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
                <bool as sys::GodotFfi>::sys_const(&flip_fov),
                <i64 as sys::GodotFfi>::sys_const(&eye),
                <f64 as sys::GodotFfi>::sys_const(&intraocular_dist),
                <f64 as sys::GodotFfi>::sys_const(&convergence_dist),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_for_hmd(
        eye: i64,
        aspect: f64,
        intraocular_dist: f64,
        display_width: f64,
        display_to_lens: f64,
        oversample: f64,
        z_near: f64,
        z_far: f64,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_for_hmd");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4184144994i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&eye),
                <f64 as sys::GodotFfi>::sys_const(&aspect),
                <f64 as sys::GodotFfi>::sys_const(&intraocular_dist),
                <f64 as sys::GodotFfi>::sys_const(&display_width),
                <f64 as sys::GodotFfi>::sys_const(&display_to_lens),
                <f64 as sys::GodotFfi>::sys_const(&oversample),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_orthogonal(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        z_near: f64,
        z_far: f64,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_orthogonal");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3707929169i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&left),
                <f64 as sys::GodotFfi>::sys_const(&right),
                <f64 as sys::GodotFfi>::sys_const(&bottom),
                <f64 as sys::GodotFfi>::sys_const(&top),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_orthogonal_aspect(
        size: f64,
        aspect: f64,
        z_near: f64,
        z_far: f64,
        flip_fov: bool,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_orthogonal_aspect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                390915442i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&size),
                <f64 as sys::GodotFfi>::sys_const(&aspect),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
                <bool as sys::GodotFfi>::sys_const(&flip_fov),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_frustum(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        z_near: f64,
        z_far: f64,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_frustum");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3707929169i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&left),
                <f64 as sys::GodotFfi>::sys_const(&right),
                <f64 as sys::GodotFfi>::sys_const(&bottom),
                <f64 as sys::GodotFfi>::sys_const(&top),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_frustum_aspect(
        size: f64,
        aspect: f64,
        offset: Vector2,
        z_near: f64,
        z_far: f64,
        flip_fov: bool,
    ) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_frustum_aspect");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1535076251i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&size),
                <f64 as sys::GodotFfi>::sys_const(&aspect),
                <Vector2 as sys::GodotFfi>::sys_const(&offset),
                <f64 as sys::GodotFfi>::sys_const(&z_near),
                <f64 as sys::GodotFfi>::sys_const(&z_far),
                <bool as sys::GodotFfi>::sys_const(&flip_fov),
            ];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn create_fit_aabb(aabb: Aabb) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("create_fit_aabb");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2264694907i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Aabb as sys::GodotFfi>::sys_const(&aabb)];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn determinant(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
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
    pub fn perspective_znear_adjusted(&self, new_znear: f64) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("perspective_znear_adjusted");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3584785443i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<f64 as sys::GodotFfi>::sys_const(&new_znear)];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_projection_plane(&self, plane: i64) -> Plane {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_projection_plane");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1551184160i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&plane)];
            let __args_ptr = __args.as_ptr();
            <Plane as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn flipped_y(&self) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("flipped_y");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4212530932i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn jitter_offseted(&self, offset: Vector2) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("jitter_offseted");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2448438599i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Vector2 as sys::GodotFfi>::sys_const(&offset)];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_fovy(fovx: f64, aspect: f64) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_fovy");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3514207532i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <f64 as sys::GodotFfi>::sys_const(&fovx),
                <f64 as sys::GodotFfi>::sys_const(&aspect),
            ];
            let __args_ptr = __args.as_ptr();
            <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(
                    std::ptr::null_mut(),
                    __args_ptr,
                    return_ptr,
                    __args.len() as i32,
                );
            })
        }
    }
    pub fn get_z_far(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_z_far");
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
    pub fn get_z_near(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_z_near");
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
    pub fn get_aspect(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_aspect");
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
    pub fn get_fov(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_fov");
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
    pub fn is_orthogonal(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("is_orthogonal");
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
    pub fn get_viewport_half_extents(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_viewport_half_extents");
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
    pub fn get_far_plane_half_extents(&self) -> Vector2 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_far_plane_half_extents");
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
    pub fn inverse(&self) -> Projection {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("inverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4212530932i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Projection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_pixels_per_meter(&self, for_pixel_width: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_pixels_per_meter");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4103005248i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&for_pixel_width)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_lod_multiplier(&self) -> f64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_PROJECTION;
            let __method_name = StringName::from("get_lod_multiplier");
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Planes {
    ord: i32,
}
impl Planes {
    pub const PLANE_NEAR: Self = Self { ord: 0 };
    pub const PLANE_FAR: Self = Self { ord: 1 };
    pub const PLANE_LEFT: Self = Self { ord: 2 };
    pub const PLANE_TOP: Self = Self { ord: 3 };
    pub const PLANE_RIGHT: Self = Self { ord: 4 };
    pub const PLANE_BOTTOM: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for Planes {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Planes {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
