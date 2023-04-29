#![doc = "Sidecar module for class [`EditorPlugin`][crate::engine::EditorPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorPlugin.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`editor_plugin`][crate::engine::editor_plugin]: sidecar module with related enum/flag types\n* [`EditorPluginVirtual`][crate::engine::EditorPluginVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorPlugin {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorPlugin`][crate::engine::EditorPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorPluginVirtual:
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
        fn on_notification(&mut self, what: NodeNotification) {
            unimplemented!()
        }
        fn forward_canvas_gui_input(&mut self, event: Gd<InputEvent>) -> bool {
            unimplemented!()
        }
        fn forward_canvas_draw_over_viewport(&mut self, viewport_control: Gd<Control>) {
            unimplemented!()
        }
        fn forward_canvas_force_draw_over_viewport(&mut self, viewport_control: Gd<Control>) {
            unimplemented!()
        }
        fn forward_3d_gui_input(
            &mut self,
            viewport_camera: Gd<Camera3D>,
            event: Gd<InputEvent>,
        ) -> i64 {
            unimplemented!()
        }
        fn forward_3d_draw_over_viewport(&mut self, viewport_control: Gd<Control>) {
            unimplemented!()
        }
        fn forward_3d_force_draw_over_viewport(&mut self, viewport_control: Gd<Control>) {
            unimplemented!()
        }
        fn get_plugin_name(&self) -> GodotString {
            unimplemented!()
        }
        fn get_plugin_icon(&self) -> Option<Gd<Texture2D>> {
            unimplemented!()
        }
        fn has_main_screen(&self) -> bool {
            unimplemented!()
        }
        fn make_visible(&mut self, visible: bool) {
            unimplemented!()
        }
        fn edit(&mut self, object: Gd<Object>) {
            unimplemented!()
        }
        fn handles(&self, object: Gd<Object>) -> bool {
            unimplemented!()
        }
        fn get_state(&self) -> Dictionary {
            unimplemented!()
        }
        fn set_state(&mut self, state: Dictionary) {
            unimplemented!()
        }
        fn clear(&mut self) {
            unimplemented!()
        }
        fn save_external_data(&mut self) {
            unimplemented!()
        }
        fn apply_changes(&mut self) {
            unimplemented!()
        }
        fn get_breakpoints(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn set_window_layout(&mut self, configuration: Gd<ConfigFile>) {
            unimplemented!()
        }
        fn get_window_layout(&mut self, configuration: Gd<ConfigFile>) {
            unimplemented!()
        }
        fn build(&mut self) -> bool {
            unimplemented!()
        }
        fn enable_plugin(&mut self) {
            unimplemented!()
        }
        fn disable_plugin(&mut self) {
            unimplemented!()
        }
        fn process(&mut self, delta: f64) {
            unimplemented!()
        }
        fn physics_process(&mut self, delta: f64) {
            unimplemented!()
        }
        fn enter_tree(&mut self) {
            unimplemented!()
        }
        fn exit_tree(&mut self) {
            unimplemented!()
        }
        fn ready(&mut self) {
            unimplemented!()
        }
        fn get_configuration_warnings(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn shortcut_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn unhandled_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
    }
    impl EditorPlugin {
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
        pub fn notify(&mut self, what: NodeNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: NodeNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn add_control_to_container(
            &mut self,
            container: editor_plugin::CustomControlContainer,
            control: Gd<Control>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_control_to_container");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3092750152i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_control_to_container" , 3092750152i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <editor_plugin::CustomControlContainer as sys::GodotFfi>::sys_const(&container),
                    <Gd<Control> as AsArg>::as_arg_ptr(&control),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_control_to_bottom_panel(
            &mut self,
            control: Gd<Control>,
            title: GodotString,
        ) -> Option<Gd<Button>> {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_control_to_bottom_panel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3526039376i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_control_to_bottom_panel" , 3526039376i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Control> as AsArg>::as_arg_ptr(&control),
                    <GodotString as sys::GodotFfi>::sys_const(&title),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Button>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_control_to_dock(&mut self, slot: editor_plugin::DockSlot, control: Gd<Control>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_control_to_dock");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3354871258i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_control_to_dock" , 3354871258i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <editor_plugin::DockSlot as sys::GodotFfi>::sys_const(&slot),
                    <Gd<Control> as AsArg>::as_arg_ptr(&control),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_control_from_docks(&mut self, control: Gd<Control>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_control_from_docks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1496901182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_control_from_docks" , 1496901182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Control> as AsArg>::as_arg_ptr(&control)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_control_from_bottom_panel(&mut self, control: Gd<Control>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_control_from_bottom_panel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1496901182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_control_from_bottom_panel" , 1496901182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Control> as AsArg>::as_arg_ptr(&control)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_control_from_container(
            &mut self,
            container: editor_plugin::CustomControlContainer,
            control: Gd<Control>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_control_from_container");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3092750152i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_control_from_container" , 3092750152i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <editor_plugin::CustomControlContainer as sys::GodotFfi>::sys_const(&container),
                    <Gd<Control> as AsArg>::as_arg_ptr(&control),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_tool_menu_item(&mut self, name: GodotString, callable: Callable) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_tool_menu_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2137474292i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_tool_menu_item" , 2137474292i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_tool_submenu_item(&mut self, name: GodotString, submenu: Gd<PopupMenu>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_tool_submenu_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1019428915i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_tool_submenu_item" , 1019428915i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Gd<PopupMenu> as AsArg>::as_arg_ptr(&submenu),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_tool_menu_item(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_tool_menu_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_tool_menu_item" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_export_as_menu(&mut self) -> Option<Gd<PopupMenu>> {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("get_export_as_menu");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1775878644i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "get_export_as_menu" , 1775878644i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<PopupMenu>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_custom_type(
            &mut self,
            type_: GodotString,
            base: GodotString,
            script: Gd<Script>,
            icon: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_custom_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1986814599i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_custom_type" , 1986814599i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&type_),
                    <GodotString as sys::GodotFfi>::sys_const(&base),
                    <Gd<Script> as AsArg>::as_arg_ptr(&script),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&icon),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_custom_type(&mut self, type_: GodotString) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_custom_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_custom_type" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&type_)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_autoload_singleton(&mut self, name: GodotString, path: GodotString) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_autoload_singleton");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3186203200i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_autoload_singleton" , 3186203200i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_autoload_singleton(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_autoload_singleton");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_autoload_singleton" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn update_overlays(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("update_overlays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "update_overlays" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn make_bottom_panel_item_visible(&mut self, item: Gd<Control>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("make_bottom_panel_item_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1496901182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "make_bottom_panel_item_visible" , 1496901182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Control> as AsArg>::as_arg_ptr(&item)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn hide_bottom_panel(&mut self) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("hide_bottom_panel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "hide_bottom_panel" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_undo_redo(&mut self) -> Option<Gd<EditorUndoRedoManager>> {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("get_undo_redo");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    773492341i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "get_undo_redo" , 773492341i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<EditorUndoRedoManager>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_undo_redo_inspector_hook_callback(&mut self, callable: Callable) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_undo_redo_inspector_hook_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1611583062i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_undo_redo_inspector_hook_callback" , 1611583062i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Callable as sys::GodotFfi>::sys_const(&callable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_undo_redo_inspector_hook_callback(&mut self, callable: Callable) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_undo_redo_inspector_hook_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1611583062i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_undo_redo_inspector_hook_callback" , 1611583062i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Callable as sys::GodotFfi>::sys_const(&callable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn queue_save_layout(&mut self) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("queue_save_layout");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "queue_save_layout" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_translation_parser_plugin(&mut self, parser: Gd<EditorTranslationParserPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_translation_parser_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3116463128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_translation_parser_plugin" , 3116463128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorTranslationParserPlugin> as AsArg>::as_arg_ptr(
                    &parser,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_translation_parser_plugin(
            &mut self,
            parser: Gd<EditorTranslationParserPlugin>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_translation_parser_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3116463128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_translation_parser_plugin" , 3116463128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorTranslationParserPlugin> as AsArg>::as_arg_ptr(
                    &parser,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_import_plugin(
            &mut self,
            importer: Gd<EditorImportPlugin>,
            first_priority: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_import_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3113975762i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_import_plugin" , 3113975762i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<EditorImportPlugin> as AsArg>::as_arg_ptr(&importer),
                    <bool as sys::GodotFfi>::sys_const(&first_priority),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_import_plugin(&mut self, importer: Gd<EditorImportPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_import_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2312482773i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_import_plugin" , 2312482773i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorImportPlugin> as AsArg>::as_arg_ptr(&importer)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_scene_format_importer_plugin(
            &mut self,
            scene_format_importer: Gd<EditorSceneFormatImporter>,
            first_priority: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_scene_format_importer_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2764104752i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_scene_format_importer_plugin" , 2764104752i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<EditorSceneFormatImporter> as AsArg>::as_arg_ptr(&scene_format_importer),
                    <bool as sys::GodotFfi>::sys_const(&first_priority),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_scene_format_importer_plugin(
            &mut self,
            scene_format_importer: Gd<EditorSceneFormatImporter>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_scene_format_importer_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2637776123i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_scene_format_importer_plugin" , 2637776123i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorSceneFormatImporter> as AsArg>::as_arg_ptr(
                    &scene_format_importer,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_scene_post_import_plugin(
            &mut self,
            scene_import_plugin: Gd<EditorScenePostImportPlugin>,
            first_priority: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_scene_post_import_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3492436322i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_scene_post_import_plugin" , 3492436322i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<EditorScenePostImportPlugin> as AsArg>::as_arg_ptr(&scene_import_plugin),
                    <bool as sys::GodotFfi>::sys_const(&first_priority),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_scene_post_import_plugin(
            &mut self,
            scene_import_plugin: Gd<EditorScenePostImportPlugin>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_scene_post_import_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3045178206i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_scene_post_import_plugin" , 3045178206i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorScenePostImportPlugin> as AsArg>::as_arg_ptr(
                    &scene_import_plugin,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_export_plugin(&mut self, plugin: Gd<EditorExportPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_export_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4095952207i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_export_plugin" , 4095952207i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorExportPlugin> as AsArg>::as_arg_ptr(&plugin)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_export_plugin(&mut self, plugin: Gd<EditorExportPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_export_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4095952207i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_export_plugin" , 4095952207i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorExportPlugin> as AsArg>::as_arg_ptr(&plugin)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_node_3d_gizmo_plugin(&mut self, plugin: Gd<EditorNode3DGizmoPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_node_3d_gizmo_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1541015022i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_node_3d_gizmo_plugin" , 1541015022i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorNode3DGizmoPlugin> as AsArg>::as_arg_ptr(&plugin)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_node_3d_gizmo_plugin(&mut self, plugin: Gd<EditorNode3DGizmoPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_node_3d_gizmo_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1541015022i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_node_3d_gizmo_plugin" , 1541015022i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorNode3DGizmoPlugin> as AsArg>::as_arg_ptr(&plugin)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_inspector_plugin(&mut self, plugin: Gd<EditorInspectorPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_inspector_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    546395733i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_inspector_plugin" , 546395733i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorInspectorPlugin> as AsArg>::as_arg_ptr(&plugin)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_inspector_plugin(&mut self, plugin: Gd<EditorInspectorPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_inspector_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    546395733i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_inspector_plugin" , 546395733i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorInspectorPlugin> as AsArg>::as_arg_ptr(&plugin)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_resource_conversion_plugin(
            &mut self,
            plugin: Gd<EditorResourceConversionPlugin>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_resource_conversion_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2124849111i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_resource_conversion_plugin" , 2124849111i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorResourceConversionPlugin> as AsArg>::as_arg_ptr(
                    &plugin,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_resource_conversion_plugin(
            &mut self,
            plugin: Gd<EditorResourceConversionPlugin>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_resource_conversion_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2124849111i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_resource_conversion_plugin" , 2124849111i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorResourceConversionPlugin> as AsArg>::as_arg_ptr(
                    &plugin,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_input_event_forwarding_always_enabled(&mut self) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("set_input_event_forwarding_always_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "set_input_event_forwarding_always_enabled" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_force_draw_over_forwarding_enabled(&mut self) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("set_force_draw_over_forwarding_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "set_force_draw_over_forwarding_enabled" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_editor_interface(&mut self) -> Option<Gd<EditorInterface>> {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("get_editor_interface");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4223731786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "get_editor_interface" , 4223731786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<EditorInterface>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_script_create_dialog(&mut self) -> Option<Gd<ScriptCreateDialog>> {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("get_script_create_dialog");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3121871482i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "get_script_create_dialog" , 3121871482i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<ScriptCreateDialog>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_debugger_plugin(&mut self, script: Gd<EditorDebuggerPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("add_debugger_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3749880309i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "add_debugger_plugin" , 3749880309i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorDebuggerPlugin> as AsArg>::as_arg_ptr(&script)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_debugger_plugin(&mut self, script: Gd<EditorDebuggerPlugin>) {
            unsafe {
                let __class_name = StringName::from("EditorPlugin");
                let __method_name = StringName::from("remove_debugger_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3749880309i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorPlugin" , "remove_debugger_plugin" , 3749880309i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<EditorDebuggerPlugin> as AsArg>::as_arg_ptr(&script)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for EditorPlugin {
        type Base = crate::engine::Node;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "EditorPlugin";
    }
    impl crate::obj::EngineClass for EditorPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Node> for EditorPlugin {}
    impl crate::obj::Inherits<crate::engine::Object> for EditorPlugin {}
    impl std::ops::Deref for EditorPlugin {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorPlugin {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorPlugin> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CustomControlContainer {
    ord: i32,
}
impl CustomControlContainer {
    pub const CONTAINER_TOOLBAR: Self = Self { ord: 0 };
    pub const CONTAINER_SPATIAL_EDITOR_MENU: Self = Self { ord: 1 };
    pub const CONTAINER_SPATIAL_EDITOR_SIDE_LEFT: Self = Self { ord: 2 };
    pub const CONTAINER_SPATIAL_EDITOR_SIDE_RIGHT: Self = Self { ord: 3 };
    pub const CONTAINER_SPATIAL_EDITOR_BOTTOM: Self = Self { ord: 4 };
    pub const CONTAINER_CANVAS_EDITOR_MENU: Self = Self { ord: 5 };
    pub const CONTAINER_CANVAS_EDITOR_SIDE_LEFT: Self = Self { ord: 6 };
    pub const CONTAINER_CANVAS_EDITOR_SIDE_RIGHT: Self = Self { ord: 7 };
    pub const CONTAINER_CANVAS_EDITOR_BOTTOM: Self = Self { ord: 8 };
    pub const CONTAINER_INSPECTOR_BOTTOM: Self = Self { ord: 9 };
    pub const CONTAINER_PROJECT_SETTING_TAB_LEFT: Self = Self { ord: 10 };
    pub const CONTAINER_PROJECT_SETTING_TAB_RIGHT: Self = Self { ord: 11 };
}
impl crate::obj::EngineEnum for CustomControlContainer {
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
            | ord @ 10i32
            | ord @ 11i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CustomControlContainer {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DockSlot {
    ord: i32,
}
impl DockSlot {
    pub const DOCK_SLOT_LEFT_UL: Self = Self { ord: 0 };
    pub const DOCK_SLOT_LEFT_BL: Self = Self { ord: 1 };
    pub const DOCK_SLOT_LEFT_UR: Self = Self { ord: 2 };
    pub const DOCK_SLOT_LEFT_BR: Self = Self { ord: 3 };
    pub const DOCK_SLOT_RIGHT_UL: Self = Self { ord: 4 };
    pub const DOCK_SLOT_RIGHT_BL: Self = Self { ord: 5 };
    pub const DOCK_SLOT_RIGHT_UR: Self = Self { ord: 6 };
    pub const DOCK_SLOT_RIGHT_BR: Self = Self { ord: 7 };
    pub const DOCK_SLOT_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for DockSlot {
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
unsafe impl sys::GodotFfi for DockSlot {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AfterGUIInput {
    ord: i32,
}
impl AfterGUIInput {
    pub const AFTER_GUI_INPUT_PASS: Self = Self { ord: 0 };
    pub const AFTER_GUI_INPUT_STOP: Self = Self { ord: 1 };
    pub const AFTER_GUI_INPUT_CUSTOM: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for AfterGUIInput {
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
unsafe impl sys::GodotFfi for AfterGUIInput {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
