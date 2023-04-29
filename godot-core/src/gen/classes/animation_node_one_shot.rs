#![doc = "Sidecar module for class [`AnimationNodeOneShot`][crate::engine::AnimationNodeOneShot].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeOneShot` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeoneshot.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `AnimationNodeOneShot.`\n\nInherits [`AnimationNodeSync`][crate::engine::AnimationNodeSync].\n\nRelated symbols:\n\n* [`animation_node_one_shot`][crate::engine::animation_node_one_shot]: sidecar module with related enum/flag types\n* [`AnimationNodeOneShotVirtual`][crate::engine::AnimationNodeOneShotVirtual]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeOneShot`](https://docs.godotengine.org/en/stable/classes/class_animationnodeoneshot.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct AnimationNodeOneShot {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`AnimationNodeOneShot`][crate::engine::AnimationNodeOneShot].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeOneShot` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeoneshot.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait AnimationNodeOneShotVirtual:
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
        fn get_child_nodes(&self) -> Dictionary {
            unimplemented!()
        }
        fn get_parameter_list(&self) -> VariantArray {
            unimplemented!()
        }
        fn get_child_by_name(&self, name: StringName) -> Option<Gd<AnimationNode>> {
            unimplemented!()
        }
        fn get_parameter_default_value(&self, parameter: StringName) -> Variant {
            unimplemented!()
        }
        fn is_parameter_read_only(&self, parameter: StringName) -> bool {
            unimplemented!()
        }
        fn process(&self, time: f64, seek: bool, is_external_seeking: bool) -> f64 {
            unimplemented!()
        }
        fn get_caption(&self) -> GodotString {
            unimplemented!()
        }
        fn has_filter(&self) -> bool {
            unimplemented!()
        }
    }
    impl AnimationNodeOneShot {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
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
        pub fn set_fadein_time(&mut self, time: f64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("set_fadein_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "set_fadein_time" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&time)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fadein_time(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("get_fadein_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "get_fadein_time" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fadeout_time(&mut self, time: f64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("set_fadeout_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "set_fadeout_time" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&time)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fadeout_time(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("get_fadeout_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "get_fadeout_time" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_autorestart(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("set_autorestart");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "set_autorestart" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_autorestart(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("has_autorestart");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "has_autorestart" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_autorestart_delay(&mut self, enable: f64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("set_autorestart_delay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "set_autorestart_delay" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_autorestart_delay(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("get_autorestart_delay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "get_autorestart_delay" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_autorestart_random_delay(&mut self, enable: f64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("set_autorestart_random_delay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "set_autorestart_random_delay" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_autorestart_random_delay(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("get_autorestart_random_delay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "get_autorestart_random_delay" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_mix_mode(&mut self, mode: animation_node_one_shot::MixMode) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("set_mix_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1018899799i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "set_mix_mode" , 1018899799i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<animation_node_one_shot::MixMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_mix_mode(&self) -> animation_node_one_shot::MixMode {
            unsafe {
                let __class_name = StringName::from("AnimationNodeOneShot");
                let __method_name = StringName::from("get_mix_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3076550526i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeOneShot" , "get_mix_mode" , 3076550526i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <animation_node_one_shot::MixMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
    }
    impl crate::obj::GodotClass for AnimationNodeOneShot {
        type Base = crate::engine::AnimationNodeSync;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "AnimationNodeOneShot";
    }
    impl crate::obj::EngineClass for AnimationNodeOneShot {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::AnimationNodeSync> for AnimationNodeOneShot {}
    impl crate::obj::Inherits<crate::engine::AnimationNode> for AnimationNodeOneShot {}
    impl crate::obj::Inherits<crate::engine::Resource> for AnimationNodeOneShot {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for AnimationNodeOneShot {}
    impl crate::obj::Inherits<crate::engine::Object> for AnimationNodeOneShot {}
    impl std::ops::Deref for AnimationNodeOneShot {
        type Target = crate::engine::AnimationNodeSync;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for AnimationNodeOneShot {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_AnimationNodeOneShot {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::AnimationNodeOneShot> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationNodeSync> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OneShotRequest {
    ord: i32,
}
impl OneShotRequest {
    pub const ONE_SHOT_REQUEST_NONE: Self = Self { ord: 0 };
    pub const ONE_SHOT_REQUEST_FIRE: Self = Self { ord: 1 };
    pub const ONE_SHOT_REQUEST_ABORT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for OneShotRequest {
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
unsafe impl sys::GodotFfi for OneShotRequest {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MixMode {
    ord: i32,
}
impl MixMode {
    pub const MIX_MODE_BLEND: Self = Self { ord: 0 };
    pub const MIX_MODE_ADD: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for MixMode {
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
unsafe impl sys::GodotFfi for MixMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
