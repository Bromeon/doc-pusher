#![doc = "Sidecar module for class [`DisplayServer`][crate::engine::DisplayServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `DisplayServer` enums](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `DisplayServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`display_server`][crate::engine::display_server]: sidecar module with related enum/flag types\n* [`DisplayServerVirtual`][crate::engine::DisplayServerVirtual]: virtual methods\n\n\nSee also [Godot docs for `DisplayServer`](https://docs.godotengine.org/en/stable/classes/class_displayserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct DisplayServer {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`DisplayServer`][crate::engine::DisplayServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `DisplayServer` methods](https://docs.godotengine.org/en/stable/classes/class_displayserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait DisplayServerVirtual:
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
    impl DisplayServer {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __object_ptr =
                    sys::interface_fn!(global_get_singleton)(__class_name.string_sys());
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
        pub fn has_feature(&self, feature: display_server::Feature) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("has_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    334065950i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "has_feature" , 334065950i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<display_server::Feature as sys::GodotFfi>::sys_const(
                    &feature,
                )];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_submenu_item(
            &mut self,
            menu_root: GodotString,
            label: GodotString,
            submenu: GodotString,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_submenu_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3806306913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_submenu_item" , 3806306913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <GodotString as sys::GodotFfi>::sys_const(&submenu),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_item(
            &mut self,
            menu_root: GodotString,
            label: GodotString,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3415468211i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_item" , 3415468211i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_check_item(
            &mut self,
            menu_root: GodotString,
            label: GodotString,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_check_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3415468211i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_check_item" , 3415468211i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_icon_item(
            &mut self,
            menu_root: GodotString,
            icon: Gd<Texture2D>,
            label: GodotString,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_icon_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1700867534i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_icon_item" , 1700867534i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&icon),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_icon_check_item(
            &mut self,
            menu_root: GodotString,
            icon: Gd<Texture2D>,
            label: GodotString,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_icon_check_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1700867534i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_icon_check_item" , 1700867534i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&icon),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_radio_check_item(
            &mut self,
            menu_root: GodotString,
            label: GodotString,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_radio_check_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3415468211i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_radio_check_item" , 3415468211i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_icon_radio_check_item(
            &mut self,
            menu_root: GodotString,
            icon: Gd<Texture2D>,
            label: GodotString,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_icon_radio_check_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1700867534i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_icon_radio_check_item" , 1700867534i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&icon),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_multistate_item(
            &mut self,
            menu_root: GodotString,
            label: GodotString,
            max_states: i64,
            default_state: i64,
            callback: Callable,
            key_callback: Callable,
            tag: Variant,
            accelerator: global::Key,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_multistate_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    635750054i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_multistate_item" , 635750054i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <i64 as sys::GodotFfi>::sys_const(&max_states),
                    <i64 as sys::GodotFfi>::sys_const(&default_state),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                    <global::Key as sys::GodotFfi>::sys_const(&accelerator),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_add_separator(&mut self, menu_root: GodotString, index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_add_separator");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1041533178i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_add_separator" , 1041533178i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_index_from_text(
            &self,
            menu_root: GodotString,
            text: GodotString,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_index_from_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2878152881i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_index_from_text" , 2878152881i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_index_from_tag(
            &self,
            menu_root: GodotString,
            tag: Variant,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_index_from_tag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2941063483i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_index_from_tag" , 2941063483i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_is_item_checked(&self, menu_root: GodotString, idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_is_item_checked");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3511468594i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_is_item_checked" , 3511468594i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_is_item_checkable(&self, menu_root: GodotString, idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_is_item_checkable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3511468594i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_is_item_checkable" , 3511468594i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_is_item_radio_checkable(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_is_item_radio_checkable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3511468594i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_is_item_radio_checkable" , 3511468594i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_callback(&self, menu_root: GodotString, idx: i64) -> Callable {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    748666903i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_callback" , 748666903i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Callable as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_key_callback(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> Callable {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_key_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    748666903i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_key_callback" , 748666903i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Callable as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_tag(&self, menu_root: GodotString, idx: i64) -> Variant {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_tag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    330672633i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_tag" , 330672633i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_text(&self, menu_root: GodotString, idx: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    591067909i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_text" , 591067909i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_submenu(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_submenu");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    591067909i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_submenu" , 591067909i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_accelerator(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> global::Key {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_accelerator");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    936065394i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_accelerator" , 936065394i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Key as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_is_item_disabled(&self, menu_root: GodotString, idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_is_item_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3511468594i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_is_item_disabled" , 3511468594i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_tooltip(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_tooltip");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    591067909i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_tooltip" , 591067909i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_state(&self, menu_root: GodotString, idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3422818498i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_state" , 3422818498i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_max_states(&self, menu_root: GodotString, idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_max_states");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3422818498i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_max_states" , 3422818498i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_icon(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3591713183i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_icon" , 3591713183i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_get_item_indentation_level(
            &self,
            menu_root: GodotString,
            idx: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_indentation_level");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3422818498i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_indentation_level" , 3422818498i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_set_item_checked(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            checked: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_checked");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4108344793i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_checked" , 4108344793i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <bool as sys::GodotFfi>::sys_const(&checked),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_checkable(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            checkable: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_checkable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4108344793i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_checkable" , 4108344793i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <bool as sys::GodotFfi>::sys_const(&checkable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_radio_checkable(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            checkable: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_radio_checkable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4108344793i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_radio_checkable" , 4108344793i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <bool as sys::GodotFfi>::sys_const(&checkable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_callback(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            callback: Callable,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3809915389i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_callback" , 3809915389i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_key_callback(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            key_callback: Callable,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_key_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3809915389i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_key_callback" , 3809915389i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Callable as sys::GodotFfi>::sys_const(&key_callback),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_tag(&mut self, menu_root: GodotString, idx: i64, tag: Variant) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_tag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    453659863i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_tag" , 453659863i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Variant as sys::GodotFfi>::sys_const(&tag),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_text(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            text: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    965966136i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_text" , 965966136i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_submenu(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            submenu: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_submenu");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    965966136i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_submenu" , 965966136i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <GodotString as sys::GodotFfi>::sys_const(&submenu),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_accelerator(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            keycode: global::Key,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_accelerator");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    566943293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_accelerator" , 566943293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <global::Key as sys::GodotFfi>::sys_const(&keycode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_disabled(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            disabled: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4108344793i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_disabled" , 4108344793i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <bool as sys::GodotFfi>::sys_const(&disabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_tooltip(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            tooltip: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_tooltip");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    965966136i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_tooltip" , 965966136i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <GodotString as sys::GodotFfi>::sys_const(&tooltip),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_state(&mut self, menu_root: GodotString, idx: i64, state: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3474840532i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_state" , 3474840532i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <i64 as sys::GodotFfi>::sys_const(&state),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_max_states(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            max_states: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_max_states");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3474840532i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_max_states" , 3474840532i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <i64 as sys::GodotFfi>::sys_const(&max_states),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_icon(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            icon: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3201338066i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_icon" , 3201338066i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&icon),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_set_item_indentation_level(
            &mut self,
            menu_root: GodotString,
            idx: i64,
            level: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_set_item_indentation_level");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3474840532i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_set_item_indentation_level" , 3474840532i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <i64 as sys::GodotFfi>::sys_const(&level),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_get_item_count(&self, menu_root: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_get_item_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321353865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_get_item_count" , 1321353865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&menu_root)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_menu_remove_item(&mut self, menu_root: GodotString, idx: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_remove_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2956805083i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_remove_item" , 2956805083i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&menu_root),
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_menu_clear(&mut self, menu_root: GodotString) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("global_menu_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "global_menu_clear" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&menu_root)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn tts_is_speaking(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_is_speaking");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_is_speaking" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tts_is_paused(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_is_paused");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_is_paused" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tts_get_voices(&self) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_get_voices");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_get_voices" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tts_get_voices_for_language(&self, language: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_get_voices_for_language");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_get_voices_for_language" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&language)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tts_speak(
            &mut self,
            text: GodotString,
            voice: GodotString,
            volume: i64,
            pitch: f64,
            rate: f64,
            utterance_id: i64,
            interrupt: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_speak");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3723082199i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_speak" , 3723082199i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <GodotString as sys::GodotFfi>::sys_const(&voice),
                    <i64 as sys::GodotFfi>::sys_const(&volume),
                    <f64 as sys::GodotFfi>::sys_const(&pitch),
                    <f64 as sys::GodotFfi>::sys_const(&rate),
                    <i64 as sys::GodotFfi>::sys_const(&utterance_id),
                    <bool as sys::GodotFfi>::sys_const(&interrupt),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn tts_pause(&mut self) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_pause");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_pause" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn tts_resume(&mut self) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_resume");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_resume" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn tts_stop(&mut self) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_stop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_stop" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn tts_set_utterance_callback(
            &mut self,
            event: display_server::TTSUtteranceEvent,
            callable: Callable,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tts_set_utterance_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    109679083i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tts_set_utterance_callback" , 109679083i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::TTSUtteranceEvent as sys::GodotFfi>::sys_const(&event),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_dark_mode_supported(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("is_dark_mode_supported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "is_dark_mode_supported" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_dark_mode(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("is_dark_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "is_dark_mode" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_accent_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_accent_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_accent_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mouse_set_mode(&mut self, mouse_mode: display_server::MouseMode) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("mouse_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    348288463i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "mouse_set_mode" , 348288463i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<display_server::MouseMode as sys::GodotFfi>::sys_const(
                    &mouse_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mouse_get_mode(&self) -> display_server::MouseMode {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("mouse_get_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1353961651i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "mouse_get_mode" , 1353961651i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <display_server::MouseMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2i) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("warp_mouse");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "warp_mouse" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mouse_get_position(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("mouse_get_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "mouse_get_position" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mouse_get_button_state(&self) -> global::MouseButtonMask {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("mouse_get_button_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2512161324i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "mouse_get_button_state" , 2512161324i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::MouseButtonMask as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clipboard_set(&mut self, clipboard: GodotString) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("clipboard_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "clipboard_set" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&clipboard)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clipboard_get(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("clipboard_get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "clipboard_get" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clipboard_has(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("clipboard_has");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "clipboard_has" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clipboard_set_primary(&mut self, clipboard_primary: GodotString) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("clipboard_set_primary");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "clipboard_set_primary" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(
                    &clipboard_primary,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clipboard_get_primary(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("clipboard_get_primary");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "clipboard_get_primary" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_display_cutouts(&self) -> Array<Rect2> {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_display_cutouts");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_display_cutouts" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Rect2> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_display_safe_area(&self) -> Rect2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_display_safe_area");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    410525958i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_display_safe_area" , 410525958i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_screen_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_screen_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_screen_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_primary_screen(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_primary_screen");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_primary_screen" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_screen_from_rect(&self, rect: Rect2) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_screen_from_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    741354659i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_screen_from_rect" , 741354659i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rect2 as sys::GodotFfi>::sys_const(&rect)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_position(&self, screen: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1725937825i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_position" , 1725937825i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_size(&self, screen: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1725937825i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_size" , 1725937825i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_usable_rect(&self, screen: i64) -> Rect2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_usable_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2439012528i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_usable_rect" , 2439012528i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_dpi(&self, screen: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_dpi");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    181039630i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_dpi" , 181039630i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_scale(&self, screen: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    909105437i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_scale" , 909105437i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_touchscreen_available(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("is_touchscreen_available");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4162880507i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "is_touchscreen_available" , 4162880507i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_max_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_max_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_max_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_refresh_rate(&self, screen: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_refresh_rate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    909105437i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_refresh_rate" , 909105437i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_set_orientation(
            &mut self,
            orientation: display_server::ScreenOrientation,
            screen: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_set_orientation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2629526904i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_set_orientation" , 2629526904i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::ScreenOrientation as sys::GodotFfi>::sys_const(&orientation),
                    <i64 as sys::GodotFfi>::sys_const(&screen),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn screen_get_orientation(&self, screen: i64) -> display_server::ScreenOrientation {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_get_orientation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    133818562i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_get_orientation" , 133818562i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <display_server::ScreenOrientation as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn screen_set_keep_on(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_set_keep_on");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_set_keep_on" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn screen_is_kept_on(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("screen_is_kept_on");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "screen_is_kept_on" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_window_list(&self) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_window_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1930428628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_window_list" , 1930428628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_window_at_screen_position(&self, position: Vector2i) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_window_at_screen_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2485466453i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_window_at_screen_position" , 2485466453i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_get_native_handle(
            &self,
            handle_type: display_server::HandleType,
            window_id: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_native_handle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2709193271i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_native_handle" , 2709193271i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::HandleType as sys::GodotFfi>::sys_const(&handle_type),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_get_active_popup(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_active_popup");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_active_popup" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_popup_safe_rect(&mut self, window: i64, rect: Rect2i) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_popup_safe_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3317281434i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_popup_safe_rect" , 3317281434i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&window),
                    <Rect2i as sys::GodotFfi>::sys_const(&rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_popup_safe_rect(&self, window: i64) -> Rect2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_popup_safe_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2161169500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_popup_safe_rect" , 2161169500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window)];
                let __args_ptr = __args.as_ptr();
                <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_title(&mut self, title: GodotString, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_title");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3043792800i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_title" , 3043792800i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&title),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_mouse_passthrough(&mut self, region: PackedVector2Array, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_mouse_passthrough");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3958815166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_mouse_passthrough" , 3958815166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&region),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_current_screen(&self, window_id: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_current_screen");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1591665591i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_current_screen" , 1591665591i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_current_screen(&mut self, screen: i64, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_current_screen");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3023605688i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_current_screen" , 3023605688i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&screen),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_position(&self, window_id: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    763922886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_position" , 763922886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_get_position_with_decorations(&self, window_id: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_position_with_decorations");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    763922886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_position_with_decorations" , 763922886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_position(&mut self, position: Vector2i, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614040015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_position" , 3614040015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&position),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_size(&self, window_id: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    763922886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_size" , 763922886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_size(&mut self, size: Vector2i, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614040015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_size" , 3614040015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_rect_changed_callback(&mut self, callback: Callable, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_rect_changed_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3653650673i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_rect_changed_callback" , 3653650673i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_window_event_callback(&mut self, callback: Callable, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_window_event_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3653650673i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_window_event_callback" , 3653650673i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_input_event_callback(&mut self, callback: Callable, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_input_event_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3653650673i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_input_event_callback" , 3653650673i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_input_text_callback(&mut self, callback: Callable, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_input_text_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3653650673i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_input_text_callback" , 3653650673i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_drop_files_callback(&mut self, callback: Callable, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_drop_files_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3653650673i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_drop_files_callback" , 3653650673i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_attached_instance_id(&self, window_id: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_attached_instance_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1591665591i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_attached_instance_id" , 1591665591i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_get_max_size(&self, window_id: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_max_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    763922886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_max_size" , 763922886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_max_size(&mut self, max_size: Vector2i, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_max_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614040015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_max_size" , 3614040015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&max_size),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_min_size(&self, window_id: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_min_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    763922886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_min_size" , 763922886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_min_size(&mut self, min_size: Vector2i, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_min_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614040015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_min_size" , 3614040015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&min_size),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_size_with_decorations(&self, window_id: i64) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_size_with_decorations");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    763922886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_size_with_decorations" , 763922886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_get_mode(&self, window_id: i64) -> display_server::WindowMode {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2185728461i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_mode" , 2185728461i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <display_server::WindowMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_mode(&mut self, mode: display_server::WindowMode, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2942569511i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_mode" , 2942569511i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::WindowMode as sys::GodotFfi>::sys_const(&mode),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_flag(
            &mut self,
            flag: display_server::WindowFlags,
            enabled: bool,
            window_id: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3971592565i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_flag" , 3971592565i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::WindowFlags as sys::GodotFfi>::sys_const(&flag),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_flag(&self, flag: display_server::WindowFlags, window_id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2662949986i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_flag" , 2662949986i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::WindowFlags as sys::GodotFfi>::sys_const(&flag),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_window_buttons_offset(&mut self, offset: Vector2i, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_window_buttons_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614040015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_window_buttons_offset" , 3614040015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&offset),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_safe_title_margins(&self, window_id: i64) -> Vector3i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_safe_title_margins");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2295066620i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_safe_title_margins" , 2295066620i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <Vector3i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_request_attention(&mut self, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_request_attention");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1995695955i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_request_attention" , 1995695955i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_move_to_foreground(&mut self, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_move_to_foreground");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1995695955i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_move_to_foreground" , 1995695955i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_can_draw(&self, window_id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_can_draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1051549951i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_can_draw" , 1051549951i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_set_transient(&mut self, window_id: i64, parent_window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_transient");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_transient" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                    <i64 as sys::GodotFfi>::sys_const(&parent_window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_exclusive(&mut self, window_id: i64, exclusive: bool) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_exclusive");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_exclusive" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                    <bool as sys::GodotFfi>::sys_const(&exclusive),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_ime_active(&mut self, active: bool, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_ime_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    450484987i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_ime_active" , 450484987i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <bool as sys::GodotFfi>::sys_const(&active),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_ime_position(&mut self, position: Vector2i, window_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_ime_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614040015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_ime_position" , 3614040015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&position),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_set_vsync_mode(
            &mut self,
            vsync_mode: display_server::VSyncMode,
            window_id: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_set_vsync_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1708924624i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_set_vsync_mode" , 1708924624i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <display_server::VSyncMode as sys::GodotFfi>::sys_const(&vsync_mode),
                    <i64 as sys::GodotFfi>::sys_const(&window_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn window_get_vsync_mode(&self, window_id: i64) -> display_server::VSyncMode {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_get_vsync_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    578873795i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_get_vsync_mode" , 578873795i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <display_server::VSyncMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_is_maximize_allowed(&self, window_id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_is_maximize_allowed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1051549951i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_is_maximize_allowed" , 1051549951i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&window_id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_maximize_on_title_dbl_click(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_maximize_on_title_dbl_click");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_maximize_on_title_dbl_click" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn window_minimize_on_title_dbl_click(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("window_minimize_on_title_dbl_click");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "window_minimize_on_title_dbl_click" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn ime_get_selection(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("ime_get_selection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "ime_get_selection" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn ime_get_text(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("ime_get_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "ime_get_text" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn virtual_keyboard_show(
            &mut self,
            existing_text: GodotString,
            position: Rect2,
            type_: display_server::VirtualKeyboardType,
            max_length: i64,
            cursor_start: i64,
            cursor_end: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("virtual_keyboard_show");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    860410478i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "virtual_keyboard_show" , 860410478i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&existing_text),
                    <Rect2 as sys::GodotFfi>::sys_const(&position),
                    <display_server::VirtualKeyboardType as sys::GodotFfi>::sys_const(&type_),
                    <i64 as sys::GodotFfi>::sys_const(&max_length),
                    <i64 as sys::GodotFfi>::sys_const(&cursor_start),
                    <i64 as sys::GodotFfi>::sys_const(&cursor_end),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn virtual_keyboard_hide(&mut self) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("virtual_keyboard_hide");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "virtual_keyboard_hide" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn virtual_keyboard_get_height(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("virtual_keyboard_get_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "virtual_keyboard_get_height" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn cursor_set_shape(&mut self, shape: display_server::CursorShape) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("cursor_set_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2026291549i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "cursor_set_shape" , 2026291549i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<display_server::CursorShape as sys::GodotFfi>::sys_const(
                    &shape,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn cursor_get_shape(&self) -> display_server::CursorShape {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("cursor_get_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1087724927i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "cursor_get_shape" , 1087724927i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <display_server::CursorShape as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn cursor_set_custom_image(
            &mut self,
            cursor: Gd<Resource>,
            shape: display_server::CursorShape,
            hotspot: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("cursor_set_custom_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1358907026i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "cursor_set_custom_image" , 1358907026i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Resource> as AsArg>::as_arg_ptr(&cursor),
                    <display_server::CursorShape as sys::GodotFfi>::sys_const(&shape),
                    <Vector2 as sys::GodotFfi>::sys_const(&hotspot),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_swap_cancel_ok(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("get_swap_cancel_ok");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "get_swap_cancel_ok" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn enable_for_stealing_focus(&mut self, process_id: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("enable_for_stealing_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "enable_for_stealing_focus" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&process_id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn dialog_show(
            &mut self,
            title: GodotString,
            description: GodotString,
            buttons: PackedStringArray,
            callback: Callable,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("dialog_show");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4115553226i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "dialog_show" , 4115553226i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&title),
                    <GodotString as sys::GodotFfi>::sys_const(&description),
                    <PackedStringArray as sys::GodotFfi>::sys_const(&buttons),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn dialog_input_text(
            &mut self,
            title: GodotString,
            description: GodotString,
            existing_text: GodotString,
            callback: Callable,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("dialog_input_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3088703427i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "dialog_input_text" , 3088703427i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&title),
                    <GodotString as sys::GodotFfi>::sys_const(&description),
                    <GodotString as sys::GodotFfi>::sys_const(&existing_text),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn keyboard_get_layout_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("keyboard_get_layout_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "keyboard_get_layout_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn keyboard_get_current_layout(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("keyboard_get_current_layout");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "keyboard_get_current_layout" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn keyboard_set_current_layout(&mut self, index: i64) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("keyboard_set_current_layout");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "keyboard_set_current_layout" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn keyboard_get_layout_language(&self, index: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("keyboard_get_layout_language");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "keyboard_get_layout_language" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn keyboard_get_layout_name(&self, index: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("keyboard_get_layout_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "keyboard_get_layout_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn keyboard_get_keycode_from_physical(&self, keycode: global::Key) -> global::Key {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("keyboard_get_keycode_from_physical");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3447613187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "keyboard_get_keycode_from_physical" , 3447613187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::Key as sys::GodotFfi>::sys_const(&keycode)];
                let __args_ptr = __args.as_ptr();
                <global::Key as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn process_events(&mut self) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("process_events");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "process_events" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn force_process_and_drop_events(&mut self) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("force_process_and_drop_events");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "force_process_and_drop_events" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_native_icon(&mut self, filename: GodotString) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("set_native_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "set_native_icon" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&filename)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_icon(&mut self, image: Gd<Image>) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("set_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    532598488i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "set_icon" , 532598488i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Image> as AsArg>::as_arg_ptr(&image)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn tablet_get_driver_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tablet_get_driver_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tablet_get_driver_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tablet_get_driver_name(&self, idx: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tablet_get_driver_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tablet_get_driver_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tablet_get_current_driver(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tablet_get_current_driver");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tablet_get_current_driver" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tablet_set_current_driver(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("DisplayServer");
                let __method_name = StringName::from("tablet_set_current_driver");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "DisplayServer" , "tablet_set_current_driver" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub const SCREEN_PRIMARY: i32 = -2i32;
        pub const SCREEN_OF_MAIN_WINDOW: i32 = -1i32;
        pub const MAIN_WINDOW_ID: i32 = 0i32;
        pub const INVALID_WINDOW_ID: i32 = -1i32;
    }
    impl crate::obj::GodotClass for DisplayServer {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "DisplayServer";
    }
    impl crate::obj::EngineClass for DisplayServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for DisplayServer {}
    impl std::ops::Deref for DisplayServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for DisplayServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_DisplayServer {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::DisplayServer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Feature {
    ord: i32,
}
impl Feature {
    pub const FEATURE_GLOBAL_MENU: Self = Self { ord: 0 };
    pub const FEATURE_SUBWINDOWS: Self = Self { ord: 1 };
    pub const FEATURE_TOUCHSCREEN: Self = Self { ord: 2 };
    pub const FEATURE_MOUSE: Self = Self { ord: 3 };
    pub const FEATURE_MOUSE_WARP: Self = Self { ord: 4 };
    pub const FEATURE_CLIPBOARD: Self = Self { ord: 5 };
    pub const FEATURE_VIRTUAL_KEYBOARD: Self = Self { ord: 6 };
    pub const FEATURE_CURSOR_SHAPE: Self = Self { ord: 7 };
    pub const FEATURE_CUSTOM_CURSOR_SHAPE: Self = Self { ord: 8 };
    pub const FEATURE_NATIVE_DIALOG: Self = Self { ord: 9 };
    pub const FEATURE_IME: Self = Self { ord: 10 };
    pub const FEATURE_WINDOW_TRANSPARENCY: Self = Self { ord: 11 };
    pub const FEATURE_HIDPI: Self = Self { ord: 12 };
    pub const FEATURE_ICON: Self = Self { ord: 13 };
    pub const FEATURE_NATIVE_ICON: Self = Self { ord: 14 };
    pub const FEATURE_ORIENTATION: Self = Self { ord: 15 };
    pub const FEATURE_SWAP_BUFFERS: Self = Self { ord: 16 };
    pub const FEATURE_CLIPBOARD_PRIMARY: Self = Self { ord: 18 };
    pub const FEATURE_TEXT_TO_SPEECH: Self = Self { ord: 19 };
    pub const FEATURE_EXTEND_TO_TITLE: Self = Self { ord: 20 };
}
impl crate::obj::EngineEnum for Feature {
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
            | ord @ 11i32
            | ord @ 12i32
            | ord @ 13i32
            | ord @ 14i32
            | ord @ 15i32
            | ord @ 16i32
            | ord @ 18i32
            | ord @ 19i32
            | ord @ 20i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Feature {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MouseMode {
    ord: i32,
}
impl MouseMode {
    pub const MOUSE_MODE_VISIBLE: Self = Self { ord: 0 };
    pub const MOUSE_MODE_HIDDEN: Self = Self { ord: 1 };
    pub const MOUSE_MODE_CAPTURED: Self = Self { ord: 2 };
    pub const MOUSE_MODE_CONFINED: Self = Self { ord: 3 };
    pub const MOUSE_MODE_CONFINED_HIDDEN: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for MouseMode {
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
unsafe impl sys::GodotFfi for MouseMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ScreenOrientation {
    ord: i32,
}
impl ScreenOrientation {
    pub const SCREEN_LANDSCAPE: Self = Self { ord: 0 };
    pub const SCREEN_PORTRAIT: Self = Self { ord: 1 };
    pub const SCREEN_REVERSE_LANDSCAPE: Self = Self { ord: 2 };
    pub const SCREEN_REVERSE_PORTRAIT: Self = Self { ord: 3 };
    pub const SCREEN_SENSOR_LANDSCAPE: Self = Self { ord: 4 };
    pub const SCREEN_SENSOR_PORTRAIT: Self = Self { ord: 5 };
    pub const SCREEN_SENSOR: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for ScreenOrientation {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ScreenOrientation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VirtualKeyboardType {
    ord: i32,
}
impl VirtualKeyboardType {
    pub const KEYBOARD_TYPE_DEFAULT: Self = Self { ord: 0 };
    pub const KEYBOARD_TYPE_MULTILINE: Self = Self { ord: 1 };
    pub const KEYBOARD_TYPE_NUMBER: Self = Self { ord: 2 };
    pub const KEYBOARD_TYPE_NUMBER_DECIMAL: Self = Self { ord: 3 };
    pub const KEYBOARD_TYPE_PHONE: Self = Self { ord: 4 };
    pub const KEYBOARD_TYPE_EMAIL_ADDRESS: Self = Self { ord: 5 };
    pub const KEYBOARD_TYPE_PASSWORD: Self = Self { ord: 6 };
    pub const KEYBOARD_TYPE_URL: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for VirtualKeyboardType {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for VirtualKeyboardType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CursorShape {
    ord: i32,
}
impl CursorShape {
    pub const CURSOR_ARROW: Self = Self { ord: 0 };
    pub const CURSOR_IBEAM: Self = Self { ord: 1 };
    pub const CURSOR_POINTING_HAND: Self = Self { ord: 2 };
    pub const CURSOR_CROSS: Self = Self { ord: 3 };
    pub const CURSOR_WAIT: Self = Self { ord: 4 };
    pub const CURSOR_BUSY: Self = Self { ord: 5 };
    pub const CURSOR_DRAG: Self = Self { ord: 6 };
    pub const CURSOR_CAN_DROP: Self = Self { ord: 7 };
    pub const CURSOR_FORBIDDEN: Self = Self { ord: 8 };
    pub const CURSOR_VSIZE: Self = Self { ord: 9 };
    pub const CURSOR_HSIZE: Self = Self { ord: 10 };
    pub const CURSOR_BDIAGSIZE: Self = Self { ord: 11 };
    pub const CURSOR_FDIAGSIZE: Self = Self { ord: 12 };
    pub const CURSOR_MOVE: Self = Self { ord: 13 };
    pub const CURSOR_VSPLIT: Self = Self { ord: 14 };
    pub const CURSOR_HSPLIT: Self = Self { ord: 15 };
    pub const CURSOR_HELP: Self = Self { ord: 16 };
    pub const CURSOR_MAX: Self = Self { ord: 17 };
}
impl crate::obj::EngineEnum for CursorShape {
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
            | ord @ 11i32
            | ord @ 12i32
            | ord @ 13i32
            | ord @ 14i32
            | ord @ 15i32
            | ord @ 16i32
            | ord @ 17i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CursorShape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct WindowMode {
    ord: i32,
}
impl WindowMode {
    pub const WINDOW_MODE_WINDOWED: Self = Self { ord: 0 };
    pub const WINDOW_MODE_MINIMIZED: Self = Self { ord: 1 };
    pub const WINDOW_MODE_MAXIMIZED: Self = Self { ord: 2 };
    pub const WINDOW_MODE_FULLSCREEN: Self = Self { ord: 3 };
    pub const WINDOW_MODE_EXCLUSIVE_FULLSCREEN: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for WindowMode {
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
unsafe impl sys::GodotFfi for WindowMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct WindowFlags {
    ord: i32,
}
impl WindowFlags {
    pub const WINDOW_FLAG_RESIZE_DISABLED: Self = Self { ord: 0 };
    pub const WINDOW_FLAG_BORDERLESS: Self = Self { ord: 1 };
    pub const WINDOW_FLAG_ALWAYS_ON_TOP: Self = Self { ord: 2 };
    pub const WINDOW_FLAG_TRANSPARENT: Self = Self { ord: 3 };
    pub const WINDOW_FLAG_NO_FOCUS: Self = Self { ord: 4 };
    pub const WINDOW_FLAG_POPUP: Self = Self { ord: 5 };
    pub const WINDOW_FLAG_EXTEND_TO_TITLE: Self = Self { ord: 6 };
    pub const WINDOW_FLAG_MOUSE_PASSTHROUGH: Self = Self { ord: 7 };
    pub const WINDOW_FLAG_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for WindowFlags {
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
unsafe impl sys::GodotFfi for WindowFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct WindowEvent {
    ord: i32,
}
impl WindowEvent {
    pub const WINDOW_EVENT_MOUSE_ENTER: Self = Self { ord: 0 };
    pub const WINDOW_EVENT_MOUSE_EXIT: Self = Self { ord: 1 };
    pub const WINDOW_EVENT_FOCUS_IN: Self = Self { ord: 2 };
    pub const WINDOW_EVENT_FOCUS_OUT: Self = Self { ord: 3 };
    pub const WINDOW_EVENT_CLOSE_REQUEST: Self = Self { ord: 4 };
    pub const WINDOW_EVENT_GO_BACK_REQUEST: Self = Self { ord: 5 };
    pub const WINDOW_EVENT_DPI_CHANGE: Self = Self { ord: 6 };
    pub const WINDOW_EVENT_TITLEBAR_CHANGE: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for WindowEvent {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for WindowEvent {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VSyncMode {
    ord: i32,
}
impl VSyncMode {
    pub const VSYNC_DISABLED: Self = Self { ord: 0 };
    pub const VSYNC_ENABLED: Self = Self { ord: 1 };
    pub const VSYNC_ADAPTIVE: Self = Self { ord: 2 };
    pub const VSYNC_MAILBOX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for VSyncMode {
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
unsafe impl sys::GodotFfi for VSyncMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct HandleType {
    ord: i32,
}
impl HandleType {
    pub const DISPLAY_HANDLE: Self = Self { ord: 0 };
    pub const WINDOW_HANDLE: Self = Self { ord: 1 };
    pub const WINDOW_VIEW: Self = Self { ord: 2 };
    pub const OPENGL_CONTEXT: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for HandleType {
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
unsafe impl sys::GodotFfi for HandleType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TTSUtteranceEvent {
    ord: i32,
}
impl TTSUtteranceEvent {
    pub const TTS_UTTERANCE_STARTED: Self = Self { ord: 0 };
    pub const TTS_UTTERANCE_ENDED: Self = Self { ord: 1 };
    pub const TTS_UTTERANCE_CANCELED: Self = Self { ord: 2 };
    pub const TTS_UTTERANCE_BOUNDARY: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for TTSUtteranceEvent {
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
unsafe impl sys::GodotFfi for TTSUtteranceEvent {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
