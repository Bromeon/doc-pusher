#![doc = "Sidecar module for class [`Tween`][crate::engine::Tween].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Tween` enums](https://docs.godotengine.org/en/stable/classes/class_tween.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Tween.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`tween`][crate::engine::tween]: sidecar module with related enum/flag types\n* [`TweenVirtual`][crate::engine::TweenVirtual]: virtual methods\n\n\nSee also [Godot docs for `Tween`](https://docs.godotengine.org/en/stable/classes/class_tween.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Tween {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Tween`][crate::engine::Tween].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Tween` methods](https://docs.godotengine.org/en/stable/classes/class_tween.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait TweenVirtual:
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
    }
    impl Tween {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Tween");
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
        pub fn tween_property(
            &mut self,
            object: Gd<Object>,
            property: NodePath,
            final_val: Variant,
            duration: f64,
        ) -> Option<Gd<PropertyTweener>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("tween_property");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4049770449i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "tween_property" , 4049770449i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Object> as AsArg>::as_arg_ptr(&object),
                    <NodePath as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&final_val),
                    <f64 as sys::GodotFfi>::sys_const(&duration),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<PropertyTweener>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tween_interval(&mut self, time: f64) -> Option<Gd<IntervalTweener>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("tween_interval");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    413360199i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "tween_interval" , 413360199i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&time)];
                let __args_ptr = __args.as_ptr();
                <Gd<IntervalTweener>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tween_callback(&mut self, callback: Callable) -> Option<Gd<CallbackTweener>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("tween_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1540176488i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "tween_callback" , 1540176488i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Callable as sys::GodotFfi>::sys_const(&callback)];
                let __args_ptr = __args.as_ptr();
                <Gd<CallbackTweener>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tween_method(
            &mut self,
            method: Callable,
            from: Variant,
            to: Variant,
            duration: f64,
        ) -> Option<Gd<MethodTweener>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("tween_method");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2337877153i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "tween_method" , 2337877153i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&method),
                    <Variant as sys::GodotFfi>::sys_const(&from),
                    <Variant as sys::GodotFfi>::sys_const(&to),
                    <f64 as sys::GodotFfi>::sys_const(&duration),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<MethodTweener>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn custom_step(&mut self, delta: f64) -> bool {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("custom_step");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    330693286i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "custom_step" , 330693286i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&delta)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn stop(&mut self) {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("stop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "stop" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pause(&mut self) {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("pause");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "pause" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn play(&mut self) {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("play");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "play" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn kill(&mut self) {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("kill");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "kill" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_total_elapsed_time(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("get_total_elapsed_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "get_total_elapsed_time" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_running(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("is_running");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "is_running" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_valid(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("is_valid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "is_valid" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bind_node(&mut self, node: Gd<Node>) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("bind_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2946786331i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "bind_node" , 2946786331i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&node)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_process_mode(&mut self, mode: tween::TweenProcessMode) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_process_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    855258840i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_process_mode" , 855258840i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tween::TweenProcessMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_pause_mode(&mut self, mode: tween::TweenPauseMode) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_pause_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3363368837i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_pause_mode" , 3363368837i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tween::TweenPauseMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_parallel(&mut self, parallel: bool) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_parallel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1942052223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_parallel" , 1942052223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&parallel)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_loops(&mut self, loops: i64) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_loops");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2670836414i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_loops" , 2670836414i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&loops)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_speed_scale(&mut self, speed: f64) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_speed_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3961971106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_speed_scale" , 3961971106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&speed)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_trans(&mut self, trans: tween::TransitionType) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_trans");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3965963875i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_trans" , 3965963875i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tween::TransitionType as sys::GodotFfi>::sys_const(&trans)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_ease(&mut self, ease: tween::EaseType) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("set_ease");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1208117252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "set_ease" , 1208117252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tween::EaseType as sys::GodotFfi>::sys_const(&ease)];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn parallel(&mut self) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("parallel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3426978995i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "parallel" , 3426978995i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn chain(&mut self) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("chain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3426978995i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "chain" , 3426978995i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn interpolate_value(
            initial_value: Variant,
            delta_value: Variant,
            elapsed_time: f64,
            duration: f64,
            trans_type: tween::TransitionType,
            ease_type: tween::EaseType,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("Tween");
                let __method_name = StringName::from("interpolate_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3452526450i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Tween" , "interpolate_value" , 3452526450i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Variant as sys::GodotFfi>::sys_const(&initial_value),
                    <Variant as sys::GodotFfi>::sys_const(&delta_value),
                    <f64 as sys::GodotFfi>::sys_const(&elapsed_time),
                    <f64 as sys::GodotFfi>::sys_const(&duration),
                    <tween::TransitionType as sys::GodotFfi>::sys_const(&trans_type),
                    <tween::EaseType as sys::GodotFfi>::sys_const(&ease_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Tween {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Tween";
    }
    impl crate::obj::EngineClass for Tween {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for Tween {}
    impl crate::obj::Inherits<crate::engine::Object> for Tween {}
    impl std::ops::Deref for Tween {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Tween {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Tween {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Tween> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TweenProcessMode {
    ord: i32,
}
impl TweenProcessMode {
    pub const TWEEN_PROCESS_PHYSICS: Self = Self { ord: 0 };
    pub const TWEEN_PROCESS_IDLE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for TweenProcessMode {
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
unsafe impl sys::GodotFfi for TweenProcessMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TweenPauseMode {
    ord: i32,
}
impl TweenPauseMode {
    pub const TWEEN_PAUSE_BOUND: Self = Self { ord: 0 };
    pub const TWEEN_PAUSE_STOP: Self = Self { ord: 1 };
    pub const TWEEN_PAUSE_PROCESS: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for TweenPauseMode {
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
unsafe impl sys::GodotFfi for TweenPauseMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TransitionType {
    ord: i32,
}
impl TransitionType {
    pub const TRANS_LINEAR: Self = Self { ord: 0 };
    pub const TRANS_SINE: Self = Self { ord: 1 };
    pub const TRANS_QUINT: Self = Self { ord: 2 };
    pub const TRANS_QUART: Self = Self { ord: 3 };
    pub const TRANS_QUAD: Self = Self { ord: 4 };
    pub const TRANS_EXPO: Self = Self { ord: 5 };
    pub const TRANS_ELASTIC: Self = Self { ord: 6 };
    pub const TRANS_CUBIC: Self = Self { ord: 7 };
    pub const TRANS_CIRC: Self = Self { ord: 8 };
    pub const TRANS_BOUNCE: Self = Self { ord: 9 };
    pub const TRANS_BACK: Self = Self { ord: 10 };
}
impl crate::obj::EngineEnum for TransitionType {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32
            | ord @ 8i32
            | ord @ 9i32
            | ord @ 10i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TransitionType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EaseType {
    ord: i32,
}
impl EaseType {
    pub const EASE_IN: Self = Self { ord: 0 };
    pub const EASE_OUT: Self = Self { ord: 1 };
    pub const EASE_IN_OUT: Self = Self { ord: 2 };
    pub const EASE_OUT_IN: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for EaseType {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for EaseType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
