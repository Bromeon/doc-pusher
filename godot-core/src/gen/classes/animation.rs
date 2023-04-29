#![doc = "Sidecar module for class [`Animation`][crate::engine::Animation].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Animation` enums](https://docs.godotengine.org/en/stable/classes/class_animation.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Animation.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`animation`][crate::engine::animation]: sidecar module with related enum/flag types\n* [`AnimationVirtual`][crate::engine::AnimationVirtual]: virtual methods\n\n\nSee also [Godot docs for `Animation`](https://docs.godotengine.org/en/stable/classes/class_animation.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Animation {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Animation`][crate::engine::Animation].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Animation` methods](https://docs.godotengine.org/en/stable/classes/class_animation.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait AnimationVirtual:
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
    impl Animation {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Animation");
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
        pub fn add_track(&mut self, type_: animation::TrackType, at_position: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("add_track");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2393815928i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "add_track" , 2393815928i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <animation::TrackType as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&at_position),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_track(&mut self, track_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("remove_track");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "remove_track" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_track_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("get_track_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "get_track_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_get_type(&self, track_idx: i64) -> animation::TrackType {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3445944217i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_type" , 3445944217i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <animation::TrackType as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_get_path(&self, track_idx: i64) -> NodePath {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    408788394i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_path" , 408788394i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_set_path(&mut self, track_idx: i64, path: NodePath) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2761262315i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_path" , 2761262315i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <NodePath as sys::GodotFfi>::sys_const(&path),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn find_track(&self, path: NodePath, type_: animation::TrackType) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("find_track");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    245376003i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "find_track" , 245376003i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <NodePath as sys::GodotFfi>::sys_const(&path),
                    <animation::TrackType as sys::GodotFfi>::sys_const(&type_),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_move_up(&mut self, track_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_move_up");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_move_up" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_move_down(&mut self, track_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_move_down");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_move_down" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_move_to(&mut self, track_idx: i64, to_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_move_to");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_move_to" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&to_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_swap(&mut self, track_idx: i64, with_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_swap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_swap" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&with_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_set_imported(&mut self, track_idx: i64, imported: bool) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_imported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_imported" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <bool as sys::GodotFfi>::sys_const(&imported),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_is_imported(&self, track_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_is_imported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_is_imported" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_set_enabled(&mut self, track_idx: i64, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_enabled" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_is_enabled(&self, track_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_is_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_is_enabled" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn position_track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            position: Vector3,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("position_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2540608232i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "position_track_insert_key" , 2540608232i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <Vector3 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rotation_track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            rotation: Quaternion,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("rotation_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4165004800i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "rotation_track_insert_key" , 4165004800i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <Quaternion as sys::GodotFfi>::sys_const(&rotation),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn scale_track_insert_key(&mut self, track_idx: i64, time: f64, scale: Vector3) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("scale_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2540608232i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "scale_track_insert_key" , 2540608232i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <Vector3 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn blend_shape_track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            amount: f64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("blend_shape_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1534913637i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "blend_shape_track_insert_key" , 1534913637i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <f64 as sys::GodotFfi>::sys_const(&amount),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            key: Variant,
            transition: f64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1985425300i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_insert_key" , 1985425300i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <Variant as sys::GodotFfi>::sys_const(&key),
                    <f64 as sys::GodotFfi>::sys_const(&transition),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_remove_key(&mut self, track_idx: i64, key_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_remove_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_remove_key" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_remove_key_at_time(&mut self, track_idx: i64, time: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_remove_key_at_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1602489585i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_remove_key_at_time" , 1602489585i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_set_key_value(&mut self, track_idx: i64, key: i64, value: Variant) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_key_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2060538656i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_key_value" , 2060538656i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_set_key_transition(&mut self, track_idx: i64, key_idx: i64, transition: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_key_transition");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_key_transition" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <f64 as sys::GodotFfi>::sys_const(&transition),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_set_key_time(&mut self, track_idx: i64, key_idx: i64, time: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_key_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_key_time" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_get_key_transition(&self, track_idx: i64, key_idx: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_key_transition");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_key_transition" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_get_key_count(&self, track_idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_key_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_key_count" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_get_key_value(&self, track_idx: i64, key_idx: i64) -> Variant {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_key_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    678354945i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_key_value" , 678354945i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_get_key_time(&self, track_idx: i64, key_idx: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_key_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_key_time" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_find_key(
            &self,
            track_idx: i64,
            time: f64,
            find_mode: animation::FindMode,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_find_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3898229885i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_find_key" , 3898229885i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <animation::FindMode as sys::GodotFfi>::sys_const(&find_mode),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_set_interpolation_type(
            &mut self,
            track_idx: i64,
            interpolation: animation::InterpolationType,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_interpolation_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4112932513i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_interpolation_type" , 4112932513i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <animation::InterpolationType as sys::GodotFfi>::sys_const(&interpolation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_get_interpolation_type(&self, track_idx: i64) -> animation::InterpolationType {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_interpolation_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1530756894i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_interpolation_type" , 1530756894i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <animation::InterpolationType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn track_set_interpolation_loop_wrap(&mut self, track_idx: i64, interpolation: bool) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_set_interpolation_loop_wrap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_set_interpolation_loop_wrap" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <bool as sys::GodotFfi>::sys_const(&interpolation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn track_get_interpolation_loop_wrap(&self, track_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_get_interpolation_loop_wrap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_get_interpolation_loop_wrap" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn track_is_compressed(&self, track_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("track_is_compressed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "track_is_compressed" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn value_track_set_update_mode(&mut self, track_idx: i64, mode: animation::UpdateMode) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("value_track_set_update_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2854058312i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "value_track_set_update_mode" , 2854058312i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <animation::UpdateMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn value_track_get_update_mode(&self, track_idx: i64) -> animation::UpdateMode {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("value_track_get_update_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1440326473i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "value_track_get_update_mode" , 1440326473i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <animation::UpdateMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn value_track_interpolate(&self, track_idx: i64, time_sec: f64) -> Variant {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("value_track_interpolate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    491147702i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "value_track_interpolate" , 491147702i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time_sec),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn method_track_get_name(&self, track_idx: i64, key_idx: i64) -> StringName {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("method_track_get_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    351665558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "method_track_get_name" , 351665558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn method_track_get_params(&self, track_idx: i64, key_idx: i64) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("method_track_get_params");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2345056839i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "method_track_get_params" , 2345056839i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bezier_track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            value: f64,
            in_handle: Vector2,
            out_handle: Vector2,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1057544502i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_insert_key" , 1057544502i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                    <Vector2 as sys::GodotFfi>::sys_const(&in_handle),
                    <Vector2 as sys::GodotFfi>::sys_const(&out_handle),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bezier_track_set_key_value(&mut self, track_idx: i64, key_idx: i64, value: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_set_key_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_set_key_value" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn bezier_track_set_key_in_handle(
            &mut self,
            track_idx: i64,
            key_idx: i64,
            in_handle: Vector2,
            balanced_value_time_ratio: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_set_key_in_handle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1028302688i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_set_key_in_handle" , 1028302688i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <Vector2 as sys::GodotFfi>::sys_const(&in_handle),
                    <f64 as sys::GodotFfi>::sys_const(&balanced_value_time_ratio),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn bezier_track_set_key_out_handle(
            &mut self,
            track_idx: i64,
            key_idx: i64,
            out_handle: Vector2,
            balanced_value_time_ratio: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_set_key_out_handle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1028302688i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_set_key_out_handle" , 1028302688i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <Vector2 as sys::GodotFfi>::sys_const(&out_handle),
                    <f64 as sys::GodotFfi>::sys_const(&balanced_value_time_ratio),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn bezier_track_get_key_value(&self, track_idx: i64, key_idx: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_get_key_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_get_key_value" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bezier_track_get_key_in_handle(&self, track_idx: i64, key_idx: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_get_key_in_handle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3016396712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_get_key_in_handle" , 3016396712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bezier_track_get_key_out_handle(&self, track_idx: i64, key_idx: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_get_key_out_handle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3016396712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_get_key_out_handle" , 3016396712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bezier_track_interpolate(&self, track_idx: i64, time: f64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("bezier_track_interpolate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1900462983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "bezier_track_interpolate" , 1900462983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn audio_track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            stream: Gd<Resource>,
            start_offset: f64,
            end_offset: f64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3489962123i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_insert_key" , 3489962123i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <Gd<Resource> as AsArg>::as_arg_ptr(&stream),
                    <f64 as sys::GodotFfi>::sys_const(&start_offset),
                    <f64 as sys::GodotFfi>::sys_const(&end_offset),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn audio_track_set_key_stream(
            &mut self,
            track_idx: i64,
            key_idx: i64,
            stream: Gd<Resource>,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_set_key_stream");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3886397084i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_set_key_stream" , 3886397084i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <Gd<Resource> as AsArg>::as_arg_ptr(&stream),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn audio_track_set_key_start_offset(
            &mut self,
            track_idx: i64,
            key_idx: i64,
            offset: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_set_key_start_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_set_key_start_offset" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn audio_track_set_key_end_offset(
            &mut self,
            track_idx: i64,
            key_idx: i64,
            offset: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_set_key_end_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_set_key_end_offset" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn audio_track_get_key_stream(
            &self,
            track_idx: i64,
            key_idx: i64,
        ) -> Option<Gd<Resource>> {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_get_key_stream");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    635277205i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_get_key_stream" , 635277205i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Resource>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn audio_track_get_key_start_offset(&self, track_idx: i64, key_idx: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_get_key_start_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_get_key_start_offset" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn audio_track_get_key_end_offset(&self, track_idx: i64, key_idx: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_get_key_end_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_get_key_end_offset" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn audio_track_set_use_blend(&mut self, track_idx: i64, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_set_use_blend");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_set_use_blend" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn audio_track_is_use_blend(&self, track_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("audio_track_is_use_blend");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "audio_track_is_use_blend" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&track_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn animation_track_insert_key(
            &mut self,
            track_idx: i64,
            time: f64,
            animation: StringName,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("animation_track_insert_key");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    158676774i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "animation_track_insert_key" , 158676774i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <StringName as sys::GodotFfi>::sys_const(&animation),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn animation_track_set_key_animation(
            &mut self,
            track_idx: i64,
            key_idx: i64,
            animation: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("animation_track_set_key_animation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    117615382i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "animation_track_set_key_animation" , 117615382i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                    <StringName as sys::GodotFfi>::sys_const(&animation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn animation_track_get_key_animation(
            &self,
            track_idx: i64,
            key_idx: i64,
        ) -> StringName {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("animation_track_get_key_animation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    351665558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "animation_track_get_key_animation" , 351665558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <i64 as sys::GodotFfi>::sys_const(&key_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_length(&mut self, time_sec: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("set_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "set_length" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&time_sec)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_length(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("get_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "get_length" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_loop_mode(&mut self, loop_mode: animation::LoopMode) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("set_loop_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3155355575i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "set_loop_mode" , 3155355575i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<animation::LoopMode as sys::GodotFfi>::sys_const(
                    &loop_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_loop_mode(&self) -> animation::LoopMode {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("get_loop_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1988889481i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "get_loop_mode" , 1988889481i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <animation::LoopMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_step(&mut self, size_sec: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("set_step");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "set_step" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&size_sec)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_step(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("get_step");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "get_step" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear(&mut self) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "clear" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn copy_track(&mut self, track_idx: i64, to_animation: Gd<Animation>) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("copy_track");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    148001024i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "copy_track" , 148001024i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&track_idx),
                    <Gd<Animation> as AsArg>::as_arg_ptr(&to_animation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compress(&mut self, page_size: i64, fps: i64, split_tolerance: f64) {
            unsafe {
                let __class_name = StringName::from("Animation");
                let __method_name = StringName::from("compress");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1804059263i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Animation" , "compress" , 1804059263i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&page_size),
                    <i64 as sys::GodotFfi>::sys_const(&fps),
                    <f64 as sys::GodotFfi>::sys_const(&split_tolerance),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for Animation {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Animation";
    }
    impl crate::obj::EngineClass for Animation {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Animation {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Animation {}
    impl crate::obj::Inherits<crate::engine::Object> for Animation {}
    impl std::ops::Deref for Animation {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Animation {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Animation {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Animation> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TrackType {
    ord: i32,
}
impl TrackType {
    pub const TYPE_VALUE: Self = Self { ord: 0 };
    pub const TYPE_POSITION_3D: Self = Self { ord: 1 };
    pub const TYPE_ROTATION_3D: Self = Self { ord: 2 };
    pub const TYPE_SCALE_3D: Self = Self { ord: 3 };
    pub const TYPE_BLEND_SHAPE: Self = Self { ord: 4 };
    pub const TYPE_METHOD: Self = Self { ord: 5 };
    pub const TYPE_BEZIER: Self = Self { ord: 6 };
    pub const TYPE_AUDIO: Self = Self { ord: 7 };
    pub const TYPE_ANIMATION: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for TrackType {
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
            | ord @ 8i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TrackType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct InterpolationType {
    ord: i32,
}
impl InterpolationType {
    pub const INTERPOLATION_NEAREST: Self = Self { ord: 0 };
    pub const INTERPOLATION_LINEAR: Self = Self { ord: 1 };
    pub const INTERPOLATION_CUBIC: Self = Self { ord: 2 };
    pub const INTERPOLATION_LINEAR_ANGLE: Self = Self { ord: 3 };
    pub const INTERPOLATION_CUBIC_ANGLE: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for InterpolationType {
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
unsafe impl sys::GodotFfi for InterpolationType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct UpdateMode {
    ord: i32,
}
impl UpdateMode {
    pub const UPDATE_CONTINUOUS: Self = Self { ord: 0 };
    pub const UPDATE_DISCRETE: Self = Self { ord: 1 };
    pub const UPDATE_CAPTURE: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for UpdateMode {
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
unsafe impl sys::GodotFfi for UpdateMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LoopMode {
    ord: i32,
}
impl LoopMode {
    pub const LOOP_NONE: Self = Self { ord: 0 };
    pub const LOOP_LINEAR: Self = Self { ord: 1 };
    pub const LOOP_PINGPONG: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LoopMode {
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
unsafe impl sys::GodotFfi for LoopMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LoopedFlag {
    ord: i32,
}
impl LoopedFlag {
    pub const LOOPED_FLAG_NONE: Self = Self { ord: 0 };
    pub const LOOPED_FLAG_END: Self = Self { ord: 1 };
    pub const LOOPED_FLAG_START: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LoopedFlag {
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
unsafe impl sys::GodotFfi for LoopedFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FindMode {
    ord: i32,
}
impl FindMode {
    pub const FIND_MODE_NEAREST: Self = Self { ord: 0 };
    pub const FIND_MODE_APPROX: Self = Self { ord: 1 };
    pub const FIND_MODE_EXACT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for FindMode {
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
unsafe impl sys::GodotFfi for FindMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
