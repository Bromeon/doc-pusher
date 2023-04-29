#![doc = "Sidecar module for class [`AnimationNodeTransition`][crate::engine::AnimationNodeTransition].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeTransition` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodetransition.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `AnimationNodeTransition.`\n\nInherits [`AnimationNodeSync`][crate::engine::AnimationNodeSync].\n\nRelated symbols:\n\n* [`AnimationNodeTransitionVirtual`][crate::engine::AnimationNodeTransitionVirtual]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeTransition`](https://docs.godotengine.org/en/stable/classes/class_animationnodetransition.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct AnimationNodeTransition {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`AnimationNodeTransition`][crate::engine::AnimationNodeTransition].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeTransition` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodetransition.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait AnimationNodeTransitionVirtual:
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
    impl AnimationNodeTransition {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
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
        pub fn set_input_count(&mut self, input_count: i64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("set_input_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "set_input_count" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&input_count)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_input_as_auto_advance(&mut self, input: i64, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("set_input_as_auto_advance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "set_input_as_auto_advance" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&input),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_input_set_as_auto_advance(&self, input: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("is_input_set_as_auto_advance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "is_input_set_as_auto_advance" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&input)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_input_reset(&mut self, input: i64, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("set_input_reset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "set_input_reset" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&input),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_input_reset(&self, input: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("is_input_reset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "is_input_reset" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&input)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_xfade_time(&mut self, time: f64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("set_xfade_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "set_xfade_time" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&time)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_xfade_time(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("get_xfade_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "get_xfade_time" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_xfade_curve(&mut self, curve: Gd<Curve>) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("set_xfade_curve");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    270443179i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "set_xfade_curve" , 270443179i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Curve> as AsArg>::as_arg_ptr(&curve)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_xfade_curve(&self) -> Option<Gd<Curve>> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("get_xfade_curve");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2460114913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "get_xfade_curve" , 2460114913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Curve>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_allow_transition_to_self(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("set_allow_transition_to_self");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "set_allow_transition_to_self" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_allow_transition_to_self(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeTransition");
                let __method_name = StringName::from("is_allow_transition_to_self");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeTransition" , "is_allow_transition_to_self" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for AnimationNodeTransition {
        type Base = crate::engine::AnimationNodeSync;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "AnimationNodeTransition";
    }
    impl crate::obj::EngineClass for AnimationNodeTransition {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::AnimationNodeSync> for AnimationNodeTransition {}
    impl crate::obj::Inherits<crate::engine::AnimationNode> for AnimationNodeTransition {}
    impl crate::obj::Inherits<crate::engine::Resource> for AnimationNodeTransition {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for AnimationNodeTransition {}
    impl crate::obj::Inherits<crate::engine::Object> for AnimationNodeTransition {}
    impl std::ops::Deref for AnimationNodeTransition {
        type Target = crate::engine::AnimationNodeSync;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for AnimationNodeTransition {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_AnimationNodeTransition {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::AnimationNodeTransition> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationNodeSync> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
