#![doc = "Sidecar module for class [`XrInterfaceExtension`][crate::engine::XrInterfaceExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `XRInterfaceExtension` enums](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `XRInterfaceExtension.`\n\nInherits [`XrInterface`][crate::engine::XrInterface].\n\nRelated symbols:\n\n* [`XrInterfaceExtensionVirtual`][crate::engine::XrInterfaceExtensionVirtual]: virtual methods\n\n\nSee also [Godot docs for `XRInterfaceExtension`](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct XrInterfaceExtension {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`XrInterfaceExtension`][crate::engine::XrInterfaceExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `XRInterfaceExtension` methods](https://docs.godotengine.org/en/stable/classes/class_xrinterfaceextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait XrInterfaceExtensionVirtual:
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
        fn get_name(&self) -> StringName {
            unimplemented!()
        }
        fn get_capabilities(&self) -> i64 {
            unimplemented!()
        }
        fn is_initialized(&self) -> bool {
            unimplemented!()
        }
        fn initialize(&mut self) -> bool {
            unimplemented!()
        }
        fn uninitialize(&mut self) {
            unimplemented!()
        }
        fn supports_play_area_mode(&self, mode: xr_interface::PlayAreaMode) -> bool {
            unimplemented!()
        }
        fn get_play_area_mode(&self) -> xr_interface::PlayAreaMode {
            unimplemented!()
        }
        fn set_play_area_mode(&self, mode: xr_interface::PlayAreaMode) -> bool {
            unimplemented!()
        }
        fn get_play_area(&self) -> PackedVector3Array {
            unimplemented!()
        }
        fn get_render_target_size(&mut self) -> Vector2 {
            unimplemented!()
        }
        fn get_view_count(&mut self) -> i64 {
            unimplemented!()
        }
        fn get_camera_transform(&mut self) -> Transform3D {
            unimplemented!()
        }
        fn get_transform_for_view(&mut self, view: i64, cam_transform: Transform3D) -> Transform3D {
            unimplemented!()
        }
        fn get_projection_for_view(
            &mut self,
            view: i64,
            aspect: f64,
            z_near: f64,
            z_far: f64,
        ) -> PackedFloat64Array {
            unimplemented!()
        }
        fn get_vrs_texture(&mut self) -> Rid {
            unimplemented!()
        }
        fn process(&mut self) {
            unimplemented!()
        }
        fn pre_render(&mut self) {
            unimplemented!()
        }
        fn pre_draw_viewport(&mut self, render_target: Rid) -> bool {
            unimplemented!()
        }
        fn post_draw_viewport(&mut self, render_target: Rid, screen_rect: Rect2) {
            unimplemented!()
        }
        fn end_frame(&mut self) {
            unimplemented!()
        }
        fn get_suggested_tracker_names(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn get_suggested_pose_names(&self, tracker_name: StringName) -> PackedStringArray {
            unimplemented!()
        }
        fn get_tracking_status(&self) -> xr_interface::TrackingStatus {
            unimplemented!()
        }
        fn trigger_haptic_pulse(
            &mut self,
            action_name: GodotString,
            tracker_name: StringName,
            frequency: f64,
            amplitude: f64,
            duration_sec: f64,
            delay_sec: f64,
        ) {
            unimplemented!()
        }
        fn get_anchor_detection_is_enabled(&self) -> bool {
            unimplemented!()
        }
        fn set_anchor_detection_is_enabled(&mut self, enabled: bool) {
            unimplemented!()
        }
        fn get_camera_feed_id(&self) -> i64 {
            unimplemented!()
        }
        fn get_color_texture(&mut self) -> Rid {
            unimplemented!()
        }
        fn get_depth_texture(&mut self) -> Rid {
            unimplemented!()
        }
        fn get_velocity_texture(&mut self) -> Rid {
            unimplemented!()
        }
    }
    impl XrInterfaceExtension {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("XRInterfaceExtension");
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
        pub fn get_color_texture(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("XRInterfaceExtension");
                let __method_name = StringName::from("get_color_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "XRInterfaceExtension" , "get_color_texture" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_depth_texture(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("XRInterfaceExtension");
                let __method_name = StringName::from("get_depth_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "XRInterfaceExtension" , "get_depth_texture" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_velocity_texture(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("XRInterfaceExtension");
                let __method_name = StringName::from("get_velocity_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "XRInterfaceExtension" , "get_velocity_texture" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_blit(
            &mut self,
            render_target: Rid,
            src_rect: Rect2,
            dst_rect: Rect2i,
            use_layer: bool,
            layer: i64,
            apply_lens_distortion: bool,
            eye_center: Vector2,
            k1: f64,
            k2: f64,
            upscale: f64,
            aspect_ratio: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("XRInterfaceExtension");
                let __method_name = StringName::from("add_blit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    258596971i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "XRInterfaceExtension" , "add_blit" , 258596971i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&render_target),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Rect2i as sys::GodotFfi>::sys_const(&dst_rect),
                    <bool as sys::GodotFfi>::sys_const(&use_layer),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                    <bool as sys::GodotFfi>::sys_const(&apply_lens_distortion),
                    <Vector2 as sys::GodotFfi>::sys_const(&eye_center),
                    <f64 as sys::GodotFfi>::sys_const(&k1),
                    <f64 as sys::GodotFfi>::sys_const(&k2),
                    <f64 as sys::GodotFfi>::sys_const(&upscale),
                    <f64 as sys::GodotFfi>::sys_const(&aspect_ratio),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_render_target_texture(&mut self, render_target: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("XRInterfaceExtension");
                let __method_name = StringName::from("get_render_target_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    41030802i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "XRInterfaceExtension" , "get_render_target_texture" , 41030802i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&render_target)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for XrInterfaceExtension {
        type Base = crate::engine::XrInterface;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "XRInterfaceExtension";
    }
    impl crate::obj::EngineClass for XrInterfaceExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::XrInterface> for XrInterfaceExtension {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for XrInterfaceExtension {}
    impl crate::obj::Inherits<crate::engine::Object> for XrInterfaceExtension {}
    impl std::ops::Deref for XrInterfaceExtension {
        type Target = crate::engine::XrInterface;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for XrInterfaceExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_XrInterfaceExtension {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::XrInterfaceExtension> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::XrInterface> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
