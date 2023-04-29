#![doc = "Sidecar module for class [`Theme`][crate::engine::Theme].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Theme` enums](https://docs.godotengine.org/en/stable/classes/class_theme.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Theme.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`theme`][crate::engine::theme]: sidecar module with related enum/flag types\n* [`ThemeVirtual`][crate::engine::ThemeVirtual]: virtual methods\n\n\nSee also [Godot docs for `Theme`](https://docs.godotengine.org/en/stable/classes/class_theme.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Theme {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Theme`][crate::engine::Theme].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Theme` methods](https://docs.godotengine.org/en/stable/classes/class_theme.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ThemeVirtual:
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
    impl Theme {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Theme");
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
        pub fn set_icon(
            &mut self,
            name: StringName,
            theme_type: StringName,
            texture: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2188371082i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_icon" , 2188371082i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_icon(&self, name: StringName, theme_type: StringName) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    934555193i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_icon" , 934555193i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_icon(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_icon" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_icon(
            &mut self,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642128662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_icon" , 642128662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_icon(&mut self, name: StringName, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_icon" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_icon_list(&self, theme_type: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_icon_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_icon_list" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_icon_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_icon_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_icon_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_stylebox(
            &mut self,
            name: StringName,
            theme_type: StringName,
            texture: Gd<StyleBox>,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2075907568i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_stylebox" , 2075907568i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <Gd<StyleBox> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_stylebox(
            &self,
            name: StringName,
            theme_type: StringName,
        ) -> Option<Gd<StyleBox>> {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3405608165i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_stylebox" , 3405608165i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<StyleBox>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_stylebox(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_stylebox" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_stylebox(
            &mut self,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642128662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_stylebox" , 642128662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_stylebox(&mut self, name: StringName, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_stylebox" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_stylebox_list(&self, theme_type: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_stylebox_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_stylebox_list" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_stylebox_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_stylebox_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_stylebox_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_font(&mut self, name: StringName, theme_type: StringName, font: Gd<Font>) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    177292320i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_font" , 177292320i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_font(&self, name: StringName, theme_type: StringName) -> Option<Gd<Font>> {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3445063586i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_font" , 3445063586i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Font>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_font(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_font" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_font(
            &mut self,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642128662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_font" , 642128662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_font(&mut self, name: StringName, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_font" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_font_list(&self, theme_type: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_font_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_font_list" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_font_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_font_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_font_size(&mut self, name: StringName, theme_type: StringName, font_size: i64) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    281601298i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_font_size" , 281601298i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_font_size(&self, name: StringName, theme_type: StringName) -> i64 {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2419549490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_font_size" , 2419549490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_font_size(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_font_size" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_font_size(
            &mut self,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642128662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_font_size" , 642128662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_font_size(&mut self, name: StringName, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_font_size" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_font_size_list(&self, theme_type: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_font_size_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_font_size_list" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_size_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_font_size_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_font_size_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_color(&mut self, name: StringName, theme_type: StringName, color: Color) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4111215154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_color" , 4111215154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color(&self, name: StringName, theme_type: StringName) -> Color {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2015923404i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_color" , 2015923404i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_color(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_color" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_color(
            &mut self,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642128662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_color" , 642128662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_color(&mut self, name: StringName, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_color" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color_list(&self, theme_type: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_color_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_color_list" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_color_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_color_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_color_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_constant(&mut self, name: StringName, theme_type: StringName, constant: i64) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    281601298i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_constant" , 281601298i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <i64 as sys::GodotFfi>::sys_const(&constant),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_constant(&self, name: StringName, theme_type: StringName) -> i64 {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2419549490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_constant" , 2419549490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_constant(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_constant" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_constant(
            &mut self,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642128662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_constant" , 642128662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_constant(&mut self, name: StringName, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_constant" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_constant_list(&self, theme_type: GodotString) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_constant_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4291131558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_constant_list" , 4291131558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_constant_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_constant_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_constant_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_default_base_scale(&mut self, base_scale: f64) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_default_base_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_default_base_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&base_scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_default_base_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_default_base_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_default_base_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_default_base_scale(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_default_base_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_default_base_scale" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_default_font(&mut self, font: Gd<Font>) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_default_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1262170328i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_default_font" , 1262170328i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Font> as AsArg>::as_arg_ptr(&font)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_default_font(&self) -> Option<Gd<Font>> {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_default_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3229501585i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_default_font" , 3229501585i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Font>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_default_font(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_default_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_default_font" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_default_font_size(&mut self, font_size: i64) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_default_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_default_font_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&font_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_default_font_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_default_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_default_font_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_default_font_size(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_default_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_default_font_size" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_theme_item(
            &mut self,
            data_type: theme::DataType,
            name: StringName,
            theme_type: StringName,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_theme_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2492983623i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_theme_item" , 2492983623i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <theme::DataType as sys::GodotFfi>::sys_const(&data_type),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_theme_item(
            &self,
            data_type: theme::DataType,
            name: StringName,
            theme_type: StringName,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_theme_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2191024021i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_theme_item" , 2191024021i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <theme::DataType as sys::GodotFfi>::sys_const(&data_type),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_item(
            &self,
            data_type: theme::DataType,
            name: StringName,
            theme_type: StringName,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("has_theme_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1739311056i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "has_theme_item" , 1739311056i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <theme::DataType as sys::GodotFfi>::sys_const(&data_type),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rename_theme_item(
            &mut self,
            data_type: theme::DataType,
            old_name: StringName,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("rename_theme_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3900867553i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "rename_theme_item" , 3900867553i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <theme::DataType as sys::GodotFfi>::sys_const(&data_type),
                    <StringName as sys::GodotFfi>::sys_const(&old_name),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_theme_item(
            &mut self,
            data_type: theme::DataType,
            name: StringName,
            theme_type: StringName,
        ) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_theme_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2965505587i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_theme_item" , 2965505587i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <theme::DataType as sys::GodotFfi>::sys_const(&data_type),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_theme_item_list(
            &self,
            data_type: theme::DataType,
            theme_type: GodotString,
        ) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_theme_item_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3726716710i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_theme_item_list" , 3726716710i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <theme::DataType as sys::GodotFfi>::sys_const(&data_type),
                    <GodotString as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_item_type_list(&self, data_type: theme::DataType) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_theme_item_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1316004935i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_theme_item_type_list" , 1316004935i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<theme::DataType as sys::GodotFfi>::sys_const(&data_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_type_variation(&mut self, theme_type: StringName, base_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("set_type_variation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3740211285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "set_type_variation" , 3740211285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <StringName as sys::GodotFfi>::sys_const(&base_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_type_variation(&self, theme_type: StringName, base_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("is_type_variation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "is_type_variation" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                    <StringName as sys::GodotFfi>::sys_const(&base_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_type_variation(&mut self, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear_type_variation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear_type_variation" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_type_variation_base(&self, theme_type: StringName) -> StringName {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_type_variation_base");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965194235i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_type_variation_base" , 1965194235i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_type_variation_list(&self, base_type: StringName) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_type_variation_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1761182771i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_type_variation_list" , 1761182771i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&base_type)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_type(&mut self, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("add_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "add_type" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_type(&mut self, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("remove_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "remove_type" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_type_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("get_type_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "get_type_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn merge_with(&mut self, other: Gd<Theme>) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("merge_with");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2326690814i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "merge_with" , 2326690814i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Theme> as AsArg>::as_arg_ptr(&other)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear(&mut self) {
            unsafe {
                let __class_name = StringName::from("Theme");
                let __method_name = StringName::from("clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Theme" , "clear" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for Theme {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Theme";
    }
    impl crate::obj::EngineClass for Theme {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Theme {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Theme {}
    impl crate::obj::Inherits<crate::engine::Object> for Theme {}
    impl std::ops::Deref for Theme {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Theme {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Theme {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Theme> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DataType {
    ord: i32,
}
impl DataType {
    pub const DATA_TYPE_COLOR: Self = Self { ord: 0 };
    pub const DATA_TYPE_CONSTANT: Self = Self { ord: 1 };
    pub const DATA_TYPE_FONT: Self = Self { ord: 2 };
    pub const DATA_TYPE_FONT_SIZE: Self = Self { ord: 3 };
    pub const DATA_TYPE_ICON: Self = Self { ord: 4 };
    pub const DATA_TYPE_STYLEBOX: Self = Self { ord: 5 };
    pub const DATA_TYPE_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for DataType {
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
unsafe impl sys::GodotFfi for DataType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
