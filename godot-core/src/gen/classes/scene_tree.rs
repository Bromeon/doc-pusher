#![doc = "Sidecar module for class [`SceneTree`][crate::engine::SceneTree].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SceneTree` enums](https://docs.godotengine.org/en/stable/classes/class_scenetree.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `SceneTree.`\n\nInherits [`MainLoop`][crate::engine::MainLoop].\n\nRelated symbols:\n\n* [`scene_tree`][crate::engine::scene_tree]: sidecar module with related enum/flag types\n* [`SceneTreeVirtual`][crate::engine::SceneTreeVirtual]: virtual methods\n\n\nSee also [Godot docs for `SceneTree`](https://docs.godotengine.org/en/stable/classes/class_scenetree.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SceneTree {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`SceneTree`][crate::engine::SceneTree].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SceneTree` methods](https://docs.godotengine.org/en/stable/classes/class_scenetree.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait SceneTreeVirtual:
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
        fn on_notification(&mut self, what: MainLoopNotification) {
            unimplemented!()
        }
        fn initialize(&mut self) {
            unimplemented!()
        }
        fn physics_process(&mut self, delta: f64) -> bool {
            unimplemented!()
        }
        fn process(&mut self, delta: f64) -> bool {
            unimplemented!()
        }
        fn finalize(&mut self) {
            unimplemented!()
        }
    }
    impl SceneTree {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
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
        pub fn notify(&mut self, what: MainLoopNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: MainLoopNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn get_root(&self) -> Option<Gd<Window>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_root");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1757182445i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_root" , 1757182445i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Window>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_group(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("has_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "has_group" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_auto_accept_quit(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_auto_accept_quit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_auto_accept_quit" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_auto_accept_quit(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_auto_accept_quit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_auto_accept_quit" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_quit_on_go_back(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_quit_on_go_back");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_quit_on_go_back" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_quit_on_go_back(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_quit_on_go_back");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_quit_on_go_back" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_debug_collisions_hint(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_debug_collisions_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_debug_collisions_hint" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_debugging_collisions_hint(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_debugging_collisions_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_debugging_collisions_hint" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_debug_paths_hint(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_debug_paths_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_debug_paths_hint" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_debugging_paths_hint(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_debugging_paths_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_debugging_paths_hint" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_debug_navigation_hint(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_debug_navigation_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_debug_navigation_hint" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_debugging_navigation_hint(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_debugging_navigation_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_debugging_navigation_hint" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_edited_scene_root(&mut self, scene: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_edited_scene_root");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_edited_scene_root" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&scene)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_edited_scene_root(&self) -> Option<Gd<Node>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_edited_scene_root");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3160264692i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_edited_scene_root" , 3160264692i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Node>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_pause(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_pause");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_pause" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_paused(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_paused");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_paused" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_timer(
            &mut self,
            time_sec: f64,
            process_always: bool,
            process_in_physics: bool,
            ignore_time_scale: bool,
        ) -> Option<Gd<SceneTreeTimer>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("create_timer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1780978058i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "create_timer" , 1780978058i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&time_sec),
                    <bool as sys::GodotFfi>::sys_const(&process_always),
                    <bool as sys::GodotFfi>::sys_const(&process_in_physics),
                    <bool as sys::GodotFfi>::sys_const(&ignore_time_scale),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<SceneTreeTimer>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_tween(&mut self) -> Option<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("create_tween");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3426978995i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "create_tween" , 3426978995i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Tween>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_processed_tweens(&mut self) -> Array<Gd<Tween>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_processed_tweens");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_processed_tweens" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Tween>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_node_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_node_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_node_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_frame(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_frame");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_frame" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn quit(&mut self, exit_code: i64) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("quit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1995695955i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "quit" , 1995695955i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&exit_code)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn queue_delete(&mut self, obj: Gd<Object>) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("queue_delete");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3975164845i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "queue_delete" , 3975164845i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Object> as AsArg>::as_arg_ptr(&obj)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn call_group_flags(
            &mut self,
            flags: i64,
            group: StringName,
            method: StringName,
            varargs: &[Variant],
        ) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("call_group_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1527739229i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "call_group_flags" , 1527739229i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [
                    <i64 as ToVariant>::to_variant(&flags),
                    <StringName as ToVariant>::to_variant(&group),
                    <StringName as ToVariant>::to_variant(&method),
                ];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let mut __err = sys::default_call_error();
                let return_ptr = std::ptr::null_mut();
                __call_fn(
                    __method_bind,
                    self.object_ptr,
                    __args_ptr,
                    __args.len() as i64,
                    return_ptr,
                    std::ptr::addr_of_mut!(__err),
                );
                if __err.error != sys::GDEXTENSION_CALL_OK {
                    let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                    __arg_types.extend(varargs.iter().map(Variant::get_type));
                    let __vararg_str = varargs
                        .iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    sys::panic_call_error(
                        &__err,
                        &format!(
                            "call_group_flags({flags:?}, {group:?}, {method:?}; {__vararg_str})"
                        ),
                        &__arg_types,
                    );
                }
            }
        }
        pub fn notify_group_flags(
            &mut self,
            call_flags: i64,
            group: StringName,
            notification: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("notify_group_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1245489420i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "notify_group_flags" , 1245489420i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&call_flags),
                    <StringName as sys::GodotFfi>::sys_const(&group),
                    <i64 as sys::GodotFfi>::sys_const(&notification),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_group_flags(
            &mut self,
            call_flags: i64,
            group: StringName,
            property: GodotString,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_group_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3497599527i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_group_flags" , 3497599527i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&call_flags),
                    <StringName as sys::GodotFfi>::sys_const(&group),
                    <GodotString as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn call_group(&mut self, group: StringName, method: StringName, varargs: &[Variant]) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("call_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1257962832i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "call_group" , 1257962832i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [
                    <StringName as ToVariant>::to_variant(&group),
                    <StringName as ToVariant>::to_variant(&method),
                ];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let mut __err = sys::default_call_error();
                let return_ptr = std::ptr::null_mut();
                __call_fn(
                    __method_bind,
                    self.object_ptr,
                    __args_ptr,
                    __args.len() as i64,
                    return_ptr,
                    std::ptr::addr_of_mut!(__err),
                );
                if __err.error != sys::GDEXTENSION_CALL_OK {
                    let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                    __arg_types.extend(varargs.iter().map(Variant::get_type));
                    let __vararg_str = varargs
                        .iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    sys::panic_call_error(
                        &__err,
                        &format!("call_group({group:?}, {method:?}; {__vararg_str})"),
                        &__arg_types,
                    );
                }
            }
        }
        pub fn notify_group(&mut self, group: StringName, notification: i64) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("notify_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2415702435i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "notify_group" , 2415702435i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&group),
                    <i64 as sys::GodotFfi>::sys_const(&notification),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_group(&mut self, group: StringName, property: GodotString, value: Variant) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1279312029i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_group" , 1279312029i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&group),
                    <GodotString as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_nodes_in_group(&mut self, group: StringName) -> Array<Gd<Node>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_nodes_in_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    689397652i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_nodes_in_group" , 689397652i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&group)];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Node>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_first_node_in_group(&mut self, group: StringName) -> Option<Gd<Node>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_first_node_in_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4071044623i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_first_node_in_group" , 4071044623i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&group)];
                let __args_ptr = __args.as_ptr();
                <Gd<Node>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_current_scene(&mut self, child_node: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_current_scene");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_current_scene" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&child_node)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_current_scene(&self) -> Option<Gd<Node>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_current_scene");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3160264692i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_current_scene" , 3160264692i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Node>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn change_scene_to_file(&mut self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("change_scene_to_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166001499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "change_scene_to_file" , 166001499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn change_scene_to_packed(&mut self, packed_scene: Gd<PackedScene>) -> global::Error {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("change_scene_to_packed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    107349098i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "change_scene_to_packed" , 107349098i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<PackedScene> as AsArg>::as_arg_ptr(&packed_scene)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn reload_current_scene(&mut self) -> global::Error {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("reload_current_scene");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166280745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "reload_current_scene" , 166280745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn unload_current_scene(&mut self) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("unload_current_scene");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "unload_current_scene" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_multiplayer(&mut self, multiplayer: Gd<MultiplayerApi>, root_path: NodePath) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_multiplayer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2385607013i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_multiplayer" , 2385607013i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<MultiplayerApi> as AsArg>::as_arg_ptr(&multiplayer),
                    <NodePath as sys::GodotFfi>::sys_const(&root_path),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_multiplayer(&self, for_path: NodePath) -> Option<Gd<MultiplayerApi>> {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("get_multiplayer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3453401404i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "get_multiplayer" , 3453401404i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&for_path)];
                let __args_ptr = __args.as_ptr();
                <Gd<MultiplayerApi>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_multiplayer_poll_enabled(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("set_multiplayer_poll_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "set_multiplayer_poll_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_multiplayer_poll_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("SceneTree");
                let __method_name = StringName::from("is_multiplayer_poll_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SceneTree" , "is_multiplayer_poll_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for SceneTree {
        type Base = crate::engine::MainLoop;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "SceneTree";
    }
    impl crate::obj::EngineClass for SceneTree {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::MainLoop> for SceneTree {}
    impl crate::obj::Inherits<crate::engine::Object> for SceneTree {}
    impl std::ops::Deref for SceneTree {
        type Target = crate::engine::MainLoop;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for SceneTree {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_SceneTree {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::SceneTree> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::MainLoop> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct GroupCallFlags {
    ord: i32,
}
impl GroupCallFlags {
    pub const GROUP_CALL_DEFAULT: Self = Self { ord: 0 };
    pub const GROUP_CALL_REVERSE: Self = Self { ord: 1 };
    pub const GROUP_CALL_DEFERRED: Self = Self { ord: 2 };
    pub const GROUP_CALL_UNIQUE: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for GroupCallFlags {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for GroupCallFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
