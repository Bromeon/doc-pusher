#![doc = "Sidecar module for class [`VisualShader`][crate::engine::VisualShader].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShader` enums](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `VisualShader.`\n\nInherits [`Shader`][crate::engine::Shader].\n\nRelated symbols:\n\n* [`visual_shader`][crate::engine::visual_shader]: sidecar module with related enum/flag types\n* [`VisualShaderVirtual`][crate::engine::VisualShaderVirtual]: virtual methods\n\n\nSee also [Godot docs for `VisualShader`](https://docs.godotengine.org/en/stable/classes/class_visualshader.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct VisualShader {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`VisualShader`][crate::engine::VisualShader].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShader` methods](https://docs.godotengine.org/en/stable/classes/class_visualshader.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait VisualShaderVirtual:
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
    impl VisualShader {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("VisualShader");
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
        pub fn set_mode(&mut self, mode: shader::Mode) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3978014962i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "set_mode" , 3978014962i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<shader::Mode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_node(
            &mut self,
            type_: visual_shader::Type,
            node: Gd<VisualShaderNode>,
            position: Vector2,
            id: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("add_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1560769431i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "add_node" , 1560769431i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <Gd<VisualShaderNode> as AsArg>::as_arg_ptr(&node),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_node(
            &self,
            type_: visual_shader::Type,
            id: i64,
        ) -> Option<Gd<VisualShaderNode>> {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("get_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3784670312i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "get_node" , 3784670312i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<VisualShaderNode>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_node_position(
            &mut self,
            type_: visual_shader::Type,
            id: i64,
            position: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("set_node_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726660721i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "set_node_position" , 2726660721i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_node_position(&self, type_: visual_shader::Type, id: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("get_node_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2175036082i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "get_node_position" , 2175036082i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_node_list(&self, type_: visual_shader::Type) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("get_node_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2370592410i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "get_node_list" , 2370592410i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<visual_shader::Type as sys::GodotFfi>::sys_const(&type_)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_valid_node_id(&self, type_: visual_shader::Type) -> i64 {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("get_valid_node_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    629467342i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "get_valid_node_id" , 629467342i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<visual_shader::Type as sys::GodotFfi>::sys_const(&type_)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_node(&mut self, type_: visual_shader::Type, id: i64) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("remove_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844050912i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "remove_node" , 844050912i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn replace_node(&mut self, type_: visual_shader::Type, id: i64, new_class: StringName) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("replace_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3144735253i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "replace_node" , 3144735253i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                    <StringName as sys::GodotFfi>::sys_const(&new_class),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_node_connection(
            &self,
            type_: visual_shader::Type,
            from_node: i64,
            from_port: i64,
            to_node: i64,
            to_port: i64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("is_node_connection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3922381898i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "is_node_connection" , 3922381898i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&from_node),
                    <i64 as sys::GodotFfi>::sys_const(&from_port),
                    <i64 as sys::GodotFfi>::sys_const(&to_node),
                    <i64 as sys::GodotFfi>::sys_const(&to_port),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn can_connect_nodes(
            &self,
            type_: visual_shader::Type,
            from_node: i64,
            from_port: i64,
            to_node: i64,
            to_port: i64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("can_connect_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3922381898i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "can_connect_nodes" , 3922381898i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&from_node),
                    <i64 as sys::GodotFfi>::sys_const(&from_port),
                    <i64 as sys::GodotFfi>::sys_const(&to_node),
                    <i64 as sys::GodotFfi>::sys_const(&to_port),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn connect_nodes(
            &mut self,
            type_: visual_shader::Type,
            from_node: i64,
            from_port: i64,
            to_node: i64,
            to_port: i64,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("connect_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3081049573i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "connect_nodes" , 3081049573i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&from_node),
                    <i64 as sys::GodotFfi>::sys_const(&from_port),
                    <i64 as sys::GodotFfi>::sys_const(&to_node),
                    <i64 as sys::GodotFfi>::sys_const(&to_port),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn disconnect_nodes(
            &mut self,
            type_: visual_shader::Type,
            from_node: i64,
            from_port: i64,
            to_node: i64,
            to_port: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("disconnect_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2268060358i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "disconnect_nodes" , 2268060358i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&from_node),
                    <i64 as sys::GodotFfi>::sys_const(&from_port),
                    <i64 as sys::GodotFfi>::sys_const(&to_node),
                    <i64 as sys::GodotFfi>::sys_const(&to_port),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn connect_nodes_forced(
            &mut self,
            type_: visual_shader::Type,
            from_node: i64,
            from_port: i64,
            to_node: i64,
            to_port: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("connect_nodes_forced");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2268060358i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "connect_nodes_forced" , 2268060358i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader::Type as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&from_node),
                    <i64 as sys::GodotFfi>::sys_const(&from_port),
                    <i64 as sys::GodotFfi>::sys_const(&to_node),
                    <i64 as sys::GodotFfi>::sys_const(&to_port),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_node_connections(&self, type_: visual_shader::Type) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("get_node_connections");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1441964831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "get_node_connections" , 1441964831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<visual_shader::Type as sys::GodotFfi>::sys_const(&type_)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_graph_offset(&mut self, offset: Vector2) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("set_graph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "set_graph_offset" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_graph_offset(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("get_graph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "get_graph_offset" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_varying(
            &mut self,
            name: GodotString,
            mode: visual_shader::VaryingMode,
            type_: visual_shader::VaryingType,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("add_varying");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2084110726i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "add_varying" , 2084110726i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <visual_shader::VaryingMode as sys::GodotFfi>::sys_const(&mode),
                    <visual_shader::VaryingType as sys::GodotFfi>::sys_const(&type_),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_varying(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("remove_varying");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "remove_varying" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_varying(&self, name: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("VisualShader");
                let __method_name = StringName::from("has_varying");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShader" , "has_varying" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub const NODE_ID_INVALID: i32 = -1i32;
        pub const NODE_ID_OUTPUT: i32 = 0i32;
    }
    impl crate::obj::GodotClass for VisualShader {
        type Base = crate::engine::Shader;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "VisualShader";
    }
    impl crate::obj::EngineClass for VisualShader {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Shader> for VisualShader {}
    impl crate::obj::Inherits<crate::engine::Resource> for VisualShader {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for VisualShader {}
    impl crate::obj::Inherits<crate::engine::Object> for VisualShader {}
    impl std::ops::Deref for VisualShader {
        type Target = crate::engine::Shader;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for VisualShader {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_VisualShader {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::VisualShader> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Shader> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Type {
    ord: i32,
}
impl Type {
    pub const TYPE_VERTEX: Self = Self { ord: 0 };
    pub const TYPE_FRAGMENT: Self = Self { ord: 1 };
    pub const TYPE_LIGHT: Self = Self { ord: 2 };
    pub const TYPE_START: Self = Self { ord: 3 };
    pub const TYPE_PROCESS: Self = Self { ord: 4 };
    pub const TYPE_COLLIDE: Self = Self { ord: 5 };
    pub const TYPE_START_CUSTOM: Self = Self { ord: 6 };
    pub const TYPE_PROCESS_CUSTOM: Self = Self { ord: 7 };
    pub const TYPE_SKY: Self = Self { ord: 8 };
    pub const TYPE_FOG: Self = Self { ord: 9 };
    pub const TYPE_MAX: Self = Self { ord: 10 };
}
impl crate::obj::EngineEnum for Type {
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
unsafe impl sys::GodotFfi for Type {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VaryingMode {
    ord: i32,
}
impl VaryingMode {
    pub const VARYING_MODE_VERTEX_TO_FRAG_LIGHT: Self = Self { ord: 0 };
    pub const VARYING_MODE_FRAG_TO_LIGHT: Self = Self { ord: 1 };
    pub const VARYING_MODE_MAX: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for VaryingMode {
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
unsafe impl sys::GodotFfi for VaryingMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VaryingType {
    ord: i32,
}
impl VaryingType {
    pub const VARYING_TYPE_FLOAT: Self = Self { ord: 0 };
    pub const VARYING_TYPE_INT: Self = Self { ord: 1 };
    pub const VARYING_TYPE_UINT: Self = Self { ord: 2 };
    pub const VARYING_TYPE_VECTOR_2D: Self = Self { ord: 3 };
    pub const VARYING_TYPE_VECTOR_3D: Self = Self { ord: 4 };
    pub const VARYING_TYPE_VECTOR_4D: Self = Self { ord: 5 };
    pub const VARYING_TYPE_BOOLEAN: Self = Self { ord: 6 };
    pub const VARYING_TYPE_TRANSFORM: Self = Self { ord: 7 };
    pub const VARYING_TYPE_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for VaryingType {
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
unsafe impl sys::GodotFfi for VaryingType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
