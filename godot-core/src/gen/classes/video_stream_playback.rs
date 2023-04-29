#![doc = "Sidecar module for class [`VideoStreamPlayback`][crate::engine::VideoStreamPlayback].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VideoStreamPlayback` enums](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `VideoStreamPlayback.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`VideoStreamPlaybackVirtual`][crate::engine::VideoStreamPlaybackVirtual]: virtual methods\n\n\nSee also [Godot docs for `VideoStreamPlayback`](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct VideoStreamPlayback {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`VideoStreamPlayback`][crate::engine::VideoStreamPlayback].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VideoStreamPlayback` methods](https://docs.godotengine.org/en/stable/classes/class_videostreamplayback.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait VideoStreamPlaybackVirtual:
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
        fn stop(&mut self) {
            unimplemented!()
        }
        fn play(&mut self) {
            unimplemented!()
        }
        fn is_playing(&self) -> bool {
            unimplemented!()
        }
        fn set_paused(&mut self, paused: bool) {
            unimplemented!()
        }
        fn is_paused(&self) -> bool {
            unimplemented!()
        }
        fn get_length(&self) -> f64 {
            unimplemented!()
        }
        fn get_playback_position(&self) -> f64 {
            unimplemented!()
        }
        fn seek(&mut self, time: f64) {
            unimplemented!()
        }
        fn set_audio_track(&mut self, idx: i64) {
            unimplemented!()
        }
        fn get_texture(&self) -> Option<Gd<Texture2D>> {
            unimplemented!()
        }
        fn update(&mut self, delta: f64) {
            unimplemented!()
        }
        fn get_channels(&self) -> i64 {
            unimplemented!()
        }
        fn get_mix_rate(&self) -> i64 {
            unimplemented!()
        }
    }
    impl VideoStreamPlayback {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("VideoStreamPlayback");
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
        pub fn mix_audio(
            &mut self,
            num_frames: i64,
            buffer: PackedFloat32Array,
            offset: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("VideoStreamPlayback");
                let __method_name = StringName::from("mix_audio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1369271885i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VideoStreamPlayback" , "mix_audio" , 1369271885i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&num_frames),
                    <PackedFloat32Array as sys::GodotFfi>::sys_const(&buffer),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for VideoStreamPlayback {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "VideoStreamPlayback";
    }
    impl crate::obj::EngineClass for VideoStreamPlayback {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for VideoStreamPlayback {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for VideoStreamPlayback {}
    impl crate::obj::Inherits<crate::engine::Object> for VideoStreamPlayback {}
    impl std::ops::Deref for VideoStreamPlayback {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for VideoStreamPlayback {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_VideoStreamPlayback {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::VideoStreamPlayback> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
