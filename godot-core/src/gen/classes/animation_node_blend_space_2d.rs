#![doc = "Sidecar module for class [`AnimationNodeBlendSpace2D`][crate::engine::AnimationNodeBlendSpace2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeBlendSpace2D` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `AnimationNodeBlendSpace2D.`\n\nInherits [`AnimationRootNode`][crate::engine::AnimationRootNode].\n\nRelated symbols:\n\n* [`animation_node_blend_space_2d`][crate::engine::animation_node_blend_space_2d]: sidecar module with related enum/flag types\n* [`AnimationNodeBlendSpace2DVirtual`][crate::engine::AnimationNodeBlendSpace2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeBlendSpace2D`](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct AnimationNodeBlendSpace2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`AnimationNodeBlendSpace2D`][crate::engine::AnimationNodeBlendSpace2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeBlendSpace2D` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodeblendspace2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait AnimationNodeBlendSpace2DVirtual:
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
    impl AnimationNodeBlendSpace2D {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
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
        pub fn add_blend_point(
            &mut self,
            node: Gd<AnimationRootNode>,
            pos: Vector2,
            at_index: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("add_blend_point");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1533588937i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "add_blend_point" , 1533588937i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<AnimationRootNode> as AsArg>::as_arg_ptr(&node),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <i64 as sys::GodotFfi>::sys_const(&at_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_blend_point_position(&mut self, point: i64, pos: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_blend_point_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    163021252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_blend_point_position" , 163021252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&point),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_point_position(&self, point: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_blend_point_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2299179447i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_blend_point_position" , 2299179447i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&point)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_blend_point_node(&mut self, point: i64, node: Gd<AnimationRootNode>) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_blend_point_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4240341528i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_blend_point_node" , 4240341528i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&point),
                    <Gd<AnimationRootNode> as AsArg>::as_arg_ptr(&node),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_point_node(&self, point: i64) -> Option<Gd<AnimationRootNode>> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_blend_point_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    665599029i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_blend_point_node" , 665599029i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&point)];
                let __args_ptr = __args.as_ptr();
                <Gd<AnimationRootNode>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_blend_point(&mut self, point: i64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("remove_blend_point");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "remove_blend_point" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&point)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_point_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_blend_point_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_blend_point_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_triangle(&mut self, x: i64, y: i64, z: i64, at_index: i64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("add_triangle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642454959i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "add_triangle" , 642454959i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&x),
                    <i64 as sys::GodotFfi>::sys_const(&y),
                    <i64 as sys::GodotFfi>::sys_const(&z),
                    <i64 as sys::GodotFfi>::sys_const(&at_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_triangle_point(&mut self, triangle: i64, point: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_triangle_point");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    50157827i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_triangle_point" , 50157827i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&triangle),
                    <i64 as sys::GodotFfi>::sys_const(&point),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_triangle(&mut self, triangle: i64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("remove_triangle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "remove_triangle" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&triangle)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_triangle_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_triangle_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_triangle_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_min_space(&mut self, min_space: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_min_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_min_space" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&min_space)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_min_space(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_min_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_min_space" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_max_space(&mut self, max_space: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_max_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_max_space" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&max_space)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_max_space(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_max_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_max_space" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_snap(&mut self, snap: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_snap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_snap" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&snap)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_snap(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_snap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_snap" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_x_label(&mut self, text: GodotString) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_x_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_x_label" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_x_label(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_x_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_x_label" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_y_label(&mut self, text: GodotString) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_y_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_y_label" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_y_label(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_y_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_y_label" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_auto_triangles(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_auto_triangles");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_auto_triangles" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_auto_triangles(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_auto_triangles");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_auto_triangles" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_blend_mode(&mut self, mode: animation_node_blend_space_2d::BlendMode) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    81193520i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_blend_mode" , 81193520i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <animation_node_blend_space_2d::BlendMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_mode(&self) -> animation_node_blend_space_2d::BlendMode {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("get_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1398433632i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "get_blend_mode" , 1398433632i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <animation_node_blend_space_2d::BlendMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_use_sync(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("set_use_sync");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "set_use_sync" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_using_sync(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeBlendSpace2D");
                let __method_name = StringName::from("is_using_sync");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeBlendSpace2D" , "is_using_sync" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for AnimationNodeBlendSpace2D {
        type Base = crate::engine::AnimationRootNode;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "AnimationNodeBlendSpace2D";
    }
    impl crate::obj::EngineClass for AnimationNodeBlendSpace2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::AnimationRootNode> for AnimationNodeBlendSpace2D {}
    impl crate::obj::Inherits<crate::engine::AnimationNode> for AnimationNodeBlendSpace2D {}
    impl crate::obj::Inherits<crate::engine::Resource> for AnimationNodeBlendSpace2D {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for AnimationNodeBlendSpace2D {}
    impl crate::obj::Inherits<crate::engine::Object> for AnimationNodeBlendSpace2D {}
    impl std::ops::Deref for AnimationNodeBlendSpace2D {
        type Target = crate::engine::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for AnimationNodeBlendSpace2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_AnimationNodeBlendSpace2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::AnimationNodeBlendSpace2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationRootNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationNode> for $Class {}
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
    pub const BLEND_MODE_INTERPOLATED: Self = Self { ord: 0 };
    pub const BLEND_MODE_DISCRETE: Self = Self { ord: 1 };
    pub const BLEND_MODE_DISCRETE_CARRY: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for BlendMode {
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
unsafe impl sys::GodotFfi for BlendMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
