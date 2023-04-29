#![doc = "Sidecar module for class [`AnimationNodeStateMachine`][crate::engine::AnimationNodeStateMachine].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNodeStateMachine` enums](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `AnimationNodeStateMachine.`\n\nInherits [`AnimationRootNode`][crate::engine::AnimationRootNode].\n\nRelated symbols:\n\n* [`AnimationNodeStateMachineVirtual`][crate::engine::AnimationNodeStateMachineVirtual]: virtual methods\n\n\nSee also [Godot docs for `AnimationNodeStateMachine`](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct AnimationNodeStateMachine {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`AnimationNodeStateMachine`][crate::engine::AnimationNodeStateMachine].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNodeStateMachine` methods](https://docs.godotengine.org/en/stable/classes/class_animationnodestatemachine.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait AnimationNodeStateMachineVirtual:
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
    impl AnimationNodeStateMachine {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
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
        pub fn add_node(&mut self, name: StringName, node: Gd<AnimationNode>, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("add_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2055804584i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "add_node" , 2055804584i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<AnimationNode> as AsArg>::as_arg_ptr(&node),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn replace_node(&mut self, name: StringName, node: Gd<AnimationNode>) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("replace_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2559412862i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "replace_node" , 2559412862i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<AnimationNode> as AsArg>::as_arg_ptr(&node),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_node(&self, name: StringName) -> Option<Gd<AnimationNode>> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    625644256i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_node" , 625644256i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <Gd<AnimationNode>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_node(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("remove_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "remove_node" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rename_node(&mut self, name: StringName, new_name: StringName) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("rename_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "rename_node" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&new_name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_node(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("has_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "has_node" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_node_name(&self, node: Gd<AnimationNode>) -> StringName {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_node_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    739213945i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_node_name" , 739213945i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<AnimationNode> as AsArg>::as_arg_ptr(&node)];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_node_position(&mut self, name: StringName, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("set_node_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1999414630i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "set_node_position" , 1999414630i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_node_position(&self, name: StringName) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_node_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3100822709i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_node_position" , 3100822709i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_transition(&self, from: StringName, to: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("has_transition");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "has_transition" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&from),
                    <StringName as sys::GodotFfi>::sys_const(&to),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_transition(
            &mut self,
            from: StringName,
            to: StringName,
            transition: Gd<AnimationNodeStateMachineTransition>,
        ) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("add_transition");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    795486887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "add_transition" , 795486887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&from),
                    <StringName as sys::GodotFfi>::sys_const(&to),
                    <Gd<AnimationNodeStateMachineTransition> as AsArg>::as_arg_ptr(&transition),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transition(&self, idx: i64) -> Option<Gd<AnimationNodeStateMachineTransition>> {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_transition");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4192381260i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_transition" , 4192381260i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <Gd<AnimationNodeStateMachineTransition>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_transition_from(&self, idx: i64) -> StringName {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_transition_from");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    659327637i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_transition_from" , 659327637i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_transition_to(&self, idx: i64) -> StringName {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_transition_to");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    659327637i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_transition_to" , 659327637i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_transition_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_transition_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_transition_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_transition_by_index(&mut self, idx: i64) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("remove_transition_by_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "remove_transition_by_index" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_transition(&mut self, from: StringName, to: StringName) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("remove_transition");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "remove_transition" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&from),
                    <StringName as sys::GodotFfi>::sys_const(&to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("set_graph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "set_graph_offset" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_graph_offset(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("get_graph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "get_graph_offset" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_allow_transition_to_self(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("set_allow_transition_to_self");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "set_allow_transition_to_self" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_allow_transition_to_self(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNodeStateMachine");
                let __method_name = StringName::from("is_allow_transition_to_self");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNodeStateMachine" , "is_allow_transition_to_self" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for AnimationNodeStateMachine {
        type Base = crate::engine::AnimationRootNode;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "AnimationNodeStateMachine";
    }
    impl crate::obj::EngineClass for AnimationNodeStateMachine {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::AnimationRootNode> for AnimationNodeStateMachine {}
    impl crate::obj::Inherits<crate::engine::AnimationNode> for AnimationNodeStateMachine {}
    impl crate::obj::Inherits<crate::engine::Resource> for AnimationNodeStateMachine {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for AnimationNodeStateMachine {}
    impl crate::obj::Inherits<crate::engine::Object> for AnimationNodeStateMachine {}
    impl std::ops::Deref for AnimationNodeStateMachine {
        type Target = crate::engine::AnimationRootNode;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for AnimationNodeStateMachine {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_AnimationNodeStateMachine {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::AnimationNodeStateMachine> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationRootNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::AnimationNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
