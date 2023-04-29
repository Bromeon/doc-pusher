#![doc = "Sidecar module for class [`CanvasItemMaterial`][crate::engine::CanvasItemMaterial].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItemMaterial` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `CanvasItemMaterial.`\n\nInherits [`Material`][crate::engine::Material].\n\nRelated symbols:\n\n* [`canvas_item_material`][crate::engine::canvas_item_material]: sidecar module with related enum/flag types\n* [`CanvasItemMaterialVirtual`][crate::engine::CanvasItemMaterialVirtual]: virtual methods\n\n\nSee also [Godot docs for `CanvasItemMaterial`](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct CanvasItemMaterial {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`CanvasItemMaterial`][crate::engine::CanvasItemMaterial].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CanvasItemMaterial` methods](https://docs.godotengine.org/en/stable/classes/class_canvasitemmaterial.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait CanvasItemMaterialVirtual:
        crate::obj::GodotClass + crate::private::You_forgot_the_attribute__godot_api
    {
        #[doc(hidden)]
        fn register_class(builder: &mut crate::builder::ClassBuilder<Self>) {
            unimplemented!()
        }
        #[doc = r" Godot constructor, accepting an injected `base` object."]
        #[doc = r""]
        #[doc = r" `base` refers to the base instance of the class, which can either be stored in a `#[base]` field or discarded."]
        #[doc = r" This method returns a fully-constructed instance, which will then be moved into a [`Gd<T>`][crate::obj::Gd] pointer."]
        #[doc = r""]
        #[doc = r" If the class has a `#[class(init)]` attribute, this method will be auto-generated and must not be overridden."]
        fn init(base: crate::obj::Base<Self::Base>) -> Self {
            unimplemented!()
        }
        #[doc = r" String representation of the Godot instance."]
        #[doc = r""]
        #[doc = r" Override this method to define how the instance is represented as a string."]
        #[doc = r" Used by `impl Display for Gd<T>`, as well as `str()` and `print()` in GDScript."]
        fn to_string(&self) -> crate::builtin::GodotString {
            unimplemented!()
        }
        #[doc = r" Called when the object receives a Godot notification."]
        #[doc = r""]
        #[doc = r" The type of notification can be identified through `what`. The enum is designed to hold all possible `NOTIFICATION_*`"]
        #[doc = r" constants that the current class can handle. However, this is not validated in Godot, so an enum variant `Unknown` exists"]
        #[doc = r" to represent integers out of known constants (mistakes or future additions)."]
        #[doc = r""]
        #[doc = r" This method is named `_notification` in Godot, but `on_notification` in Rust. To _send_ notifications, use the"]
        #[doc = r" [`Object::notify`][crate::engine::Object::notify] method."]
        #[doc = r""]
        #[doc = r" See also in Godot docs:"]
        #[doc = r" * [`Object::_notification`](https://docs.godotengine.org/en/stable/classes/class_object.html#class-object-method-notification)."]
        #[doc = r" * [Notifications tutorial](https://docs.godotengine.org/en/stable/tutorials/best_practices/godot_notifications.html)."]
        fn on_notification(&mut self, what: ObjectNotification) {
            unimplemented!()
        }
        fn get_shader_rid(&self) -> Rid {
            unimplemented!()
        }
        fn get_shader_mode(&self) -> shader::Mode {
            unimplemented!()
        }
        fn can_do_next_pass(&self) -> bool {
            unimplemented!()
        }
        fn can_use_render_priority(&self) -> bool {
            unimplemented!()
        }
    }
    impl CanvasItemMaterial {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __object_ptr =
                    sys::interface_fn!(classdb_construct_object)(__class_name.string_sys());
                Gd::from_obj_sys(__object_ptr)
            }
        }
        #[doc = r" ⚠️ Sends a Godot notification to all classes inherited by the object."]
        #[doc = r""]
        #[doc = r" Triggers calls to `on_notification()`, and depending on the notification, also to Godot's lifecycle callbacks such as `ready()`."]
        #[doc = r""]
        #[doc = r" Starts from the highest ancestor (the `Object` class) and goes down the hierarchy."]
        #[doc = r" See also [Godot docs for `Object::notification()`](https://docs.godotengine.org/en/latest/classes/class_object.html#id3)."]
        #[doc = r""]
        #[doc = r" # Panics"]
        #[doc = r""]
        #[doc = r" If you call this method on a user-defined object while holding a `GdRef` or `GdMut` guard on the instance, you will encounter"]
        #[doc = r" a panic. The reason is that the receiving virtual method `on_notification()` acquires a `GdMut` lock dynamically, which must"]
        #[doc = r" be exclusive."]
        pub fn notify(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ObjectNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn set_blend_mode(&mut self, blend_mode: canvas_item_material::BlendMode) {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("set_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1786054936i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "set_blend_mode" , 1786054936i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<canvas_item_material::BlendMode as sys::GodotFfi>::sys_const(&blend_mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_mode(&self) -> canvas_item_material::BlendMode {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("get_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3318684035i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "get_blend_mode" , 3318684035i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item_material::BlendMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_light_mode(&mut self, light_mode: canvas_item_material::LightMode) {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("set_light_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    628074070i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "set_light_mode" , 628074070i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<canvas_item_material::LightMode as sys::GodotFfi>::sys_const(&light_mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_light_mode(&self) -> canvas_item_material::LightMode {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("get_light_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3863292382i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "get_light_mode" , 3863292382i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item_material::LightMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_particles_animation(&mut self, particles_anim: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("set_particles_animation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "set_particles_animation" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&particles_anim)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_animation(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("get_particles_animation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "get_particles_animation" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i64) {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("set_particles_anim_h_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "set_particles_anim_h_frames" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&frames)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_anim_h_frames(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("get_particles_anim_h_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "get_particles_anim_h_frames" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i64) {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("set_particles_anim_v_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "set_particles_anim_v_frames" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&frames)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_anim_v_frames(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("get_particles_anim_v_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "get_particles_anim_v_frames" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("set_particles_anim_loop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "set_particles_anim_loop" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&loop_)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_anim_loop(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItemMaterial");
                let __method_name = StringName::from("get_particles_anim_loop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItemMaterial" , "get_particles_anim_loop" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for CanvasItemMaterial {
        type Base = crate::engine::Material;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "CanvasItemMaterial";
    }
    impl crate::obj::EngineClass for CanvasItemMaterial {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Material> for CanvasItemMaterial {}
    impl crate::obj::Inherits<crate::engine::Resource> for CanvasItemMaterial {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for CanvasItemMaterial {}
    impl crate::obj::Inherits<crate::engine::Object> for CanvasItemMaterial {}
    impl std::ops::Deref for CanvasItemMaterial {
        type Target = crate::engine::Material;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for CanvasItemMaterial {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_CanvasItemMaterial {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::CanvasItemMaterial> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Material> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlendMode {
    ord: i32,
}
impl BlendMode {
    pub const BLEND_MODE_MIX: Self = Self { ord: 0 };
    pub const BLEND_MODE_ADD: Self = Self { ord: 1 };
    pub const BLEND_MODE_SUB: Self = Self { ord: 2 };
    pub const BLEND_MODE_MUL: Self = Self { ord: 3 };
    pub const BLEND_MODE_PREMULT_ALPHA: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for BlendMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for BlendMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightMode {
    ord: i32,
}
impl LightMode {
    pub const LIGHT_MODE_NORMAL: Self = Self { ord: 0 };
    pub const LIGHT_MODE_UNSHADED: Self = Self { ord: 1 };
    pub const LIGHT_MODE_LIGHT_ONLY: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LightMode {
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
unsafe impl sys::GodotFfi for LightMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
