#![doc = "Sidecar module for class [`TextServer`][crate::engine::TextServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TextServer` enums](https://docs.godotengine.org/en/stable/classes/class_textserver.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `TextServer.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`text_server`][crate::engine::text_server]: sidecar module with related enum/flag types\n* [`TextServerVirtual`][crate::engine::TextServerVirtual]: virtual methods\n\n\nSee also [Godot docs for `TextServer`](https://docs.godotengine.org/en/stable/classes/class_textserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct TextServer {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`TextServer`][crate::engine::TextServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TextServer` methods](https://docs.godotengine.org/en/stable/classes/class_textserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait TextServerVirtual:
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
    impl TextServer {
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
        pub fn has_feature(&self, feature: text_server::Feature) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("has_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3967367083i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "has_feature" , 3967367083i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::Feature as sys::GodotFfi>::sys_const(&feature)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("get_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "get_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_features(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("get_features");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "get_features" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_support_data(&mut self, filename: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("load_support_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2323990056i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "load_support_data" , 2323990056i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&filename)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_support_data_filename(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("get_support_data_filename");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "get_support_data_filename" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_support_data_info(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("get_support_data_info");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "get_support_data_info" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_support_data(&self, filename: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("save_support_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "save_support_data" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&filename)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_locale_right_to_left(&self, locale: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("is_locale_right_to_left");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "is_locale_right_to_left" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&locale)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn name_to_tag(&self, name: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("name_to_tag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321353865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "name_to_tag" , 1321353865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tag_to_name(&self, tag: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("tag_to_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "tag_to_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&tag)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has(&mut self, rid: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("has");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "has" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&rid)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn free_rid(&mut self, rid: Rid) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("free_rid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "free_rid" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&rid)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_font(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("create_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "create_font" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_data(&mut self, font_rid: Rid, data: PackedByteArray) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1355495400i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_data" , 1355495400i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_set_face_index(&mut self, font_rid: Rid, face_index: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_face_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_face_index" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&face_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_face_index(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_face_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_face_index" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_face_count(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_face_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_face_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_style(&mut self, font_rid: Rid, style: text_server::FontStyle) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_style");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    898466325i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_style" , 898466325i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <text_server::FontStyle as sys::GodotFfi>::sys_const(&style),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_style(&self, font_rid: Rid) -> text_server::FontStyle {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_style");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3082502592i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_style" , 3082502592i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <text_server::FontStyle as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_name(&mut self, font_rid: Rid, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_name" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_name(&self, font_rid: Rid) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642473191i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_name" , 642473191i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_style_name(&mut self, font_rid: Rid, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_style_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_style_name" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_style_name(&self, font_rid: Rid) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_style_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642473191i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_style_name" , 642473191i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_weight(&mut self, font_rid: Rid, weight: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_weight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_weight" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&weight),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_weight(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_weight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_weight" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_stretch(&mut self, font_rid: Rid, weight: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_stretch");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_stretch" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&weight),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_stretch(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_stretch");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_stretch" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_antialiasing(
            &mut self,
            font_rid: Rid,
            antialiasing: text_server::FontAntialiasing,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    958337235i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_antialiasing" , 958337235i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <text_server::FontAntialiasing as sys::GodotFfi>::sys_const(&antialiasing),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_antialiasing(&self, font_rid: Rid) -> text_server::FontAntialiasing {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3389420495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_antialiasing" , 3389420495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <text_server::FontAntialiasing as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn font_set_generate_mipmaps(&mut self, font_rid: Rid, generate_mipmaps: bool) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_generate_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_generate_mipmaps" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <bool as sys::GodotFfi>::sys_const(&generate_mipmaps),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_generate_mipmaps(&self, font_rid: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_generate_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_generate_mipmaps" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_multichannel_signed_distance_field(&mut self, font_rid: Rid, msdf: bool) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_multichannel_signed_distance_field");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_multichannel_signed_distance_field" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <bool as sys::GodotFfi>::sys_const(&msdf),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_is_multichannel_signed_distance_field(&self, font_rid: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_is_multichannel_signed_distance_field");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_is_multichannel_signed_distance_field" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_msdf_pixel_range(&mut self, font_rid: Rid, msdf_pixel_range: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_msdf_pixel_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_msdf_pixel_range" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&msdf_pixel_range),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_msdf_pixel_range(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_msdf_pixel_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_msdf_pixel_range" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_msdf_size(&mut self, font_rid: Rid, msdf_size: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_msdf_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_msdf_size" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&msdf_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_msdf_size(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_msdf_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_msdf_size" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_fixed_size(&mut self, font_rid: Rid, fixed_size: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_fixed_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_fixed_size" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&fixed_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_fixed_size(&self, font_rid: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_fixed_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_fixed_size" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_allow_system_fallback(
            &mut self,
            font_rid: Rid,
            allow_system_fallback: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_allow_system_fallback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_allow_system_fallback" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <bool as sys::GodotFfi>::sys_const(&allow_system_fallback),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_is_allow_system_fallback(&self, font_rid: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_is_allow_system_fallback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_is_allow_system_fallback" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_force_autohinter(&mut self, font_rid: Rid, force_autohinter: bool) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_force_autohinter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_force_autohinter" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <bool as sys::GodotFfi>::sys_const(&force_autohinter),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_is_force_autohinter(&self, font_rid: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_is_force_autohinter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_is_force_autohinter" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_hinting(&mut self, font_rid: Rid, hinting: text_server::Hinting) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_hinting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1520010864i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_hinting" , 1520010864i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <text_server::Hinting as sys::GodotFfi>::sys_const(&hinting),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_hinting(&self, font_rid: Rid) -> text_server::Hinting {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_hinting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3971592737i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_hinting" , 3971592737i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <text_server::Hinting as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_subpixel_positioning(
            &mut self,
            font_rid: Rid,
            subpixel_positioning: text_server::SubpixelPositioning,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_subpixel_positioning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3830459669i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_subpixel_positioning" , 3830459669i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <text_server::SubpixelPositioning as sys::GodotFfi>::sys_const(
                        &subpixel_positioning,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_subpixel_positioning(
            &self,
            font_rid: Rid,
        ) -> text_server::SubpixelPositioning {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_subpixel_positioning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2752233671i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_subpixel_positioning" , 2752233671i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <text_server::SubpixelPositioning as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn font_set_embolden(&mut self, font_rid: Rid, strength: f64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_embolden");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_embolden" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <f64 as sys::GodotFfi>::sys_const(&strength),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_embolden(&self, font_rid: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_embolden");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_embolden" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_transform(&mut self, font_rid: Rid, transform: Transform2D) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_transform" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_transform(&self, font_rid: Rid) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    213527486i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_transform" , 213527486i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_variation_coordinates(
            &mut self,
            font_rid: Rid,
            variation_coordinates: Dictionary,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_variation_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1217542888i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_variation_coordinates" , 1217542888i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Dictionary as sys::GodotFfi>::sys_const(&variation_coordinates),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_variation_coordinates(&self, font_rid: Rid) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_variation_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1882737106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_variation_coordinates" , 1882737106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_oversampling(&mut self, font_rid: Rid, oversampling: f64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_oversampling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_oversampling" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <f64 as sys::GodotFfi>::sys_const(&oversampling),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_oversampling(&self, font_rid: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_oversampling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_oversampling" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_size_cache_list(&self, font_rid: Rid) -> Array<Vector2i> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_size_cache_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_size_cache_list" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <Array<Vector2i> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_clear_size_cache(&mut self, font_rid: Rid) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_clear_size_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_clear_size_cache" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_remove_size_cache(&mut self, font_rid: Rid, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_remove_size_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2450610377i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_remove_size_cache" , 2450610377i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_set_ascent(&mut self, font_rid: Rid, size: i64, ascent: f64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_ascent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1892459533i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_ascent" , 1892459533i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&ascent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_ascent(&self, font_rid: Rid, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_ascent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    755457166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_ascent" , 755457166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_descent(&mut self, font_rid: Rid, size: i64, descent: f64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_descent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1892459533i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_descent" , 1892459533i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&descent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_descent(&self, font_rid: Rid, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_descent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    755457166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_descent" , 755457166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_underline_position(
            &mut self,
            font_rid: Rid,
            size: i64,
            underline_position: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_underline_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1892459533i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_underline_position" , 1892459533i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&underline_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_underline_position(&self, font_rid: Rid, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_underline_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    755457166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_underline_position" , 755457166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_underline_thickness(
            &mut self,
            font_rid: Rid,
            size: i64,
            underline_thickness: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_underline_thickness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1892459533i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_underline_thickness" , 1892459533i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&underline_thickness),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_underline_thickness(&self, font_rid: Rid, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_underline_thickness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    755457166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_underline_thickness" , 755457166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_scale(&mut self, font_rid: Rid, size: i64, scale: f64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1892459533i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_scale" , 1892459533i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_scale(&self, font_rid: Rid, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    755457166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_scale" , 755457166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_texture_count(&self, font_rid: Rid, size: Vector2i) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_texture_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1311001310i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_texture_count" , 1311001310i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_clear_textures(&mut self, font_rid: Rid, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_clear_textures");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2450610377i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_clear_textures" , 2450610377i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_remove_texture(&mut self, font_rid: Rid, size: Vector2i, texture_index: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_remove_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3810512262i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_remove_texture" , 3810512262i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_set_texture_image(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
            image: Gd<Image>,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_texture_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2354485091i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_texture_image" , 2354485091i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                    <Gd<Image> as AsArg>::as_arg_ptr(&image),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_texture_image(
            &self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
        ) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_texture_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2451761155i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_texture_image" , 2451761155i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_texture_offsets(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
            offset: PackedInt32Array,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_texture_offsets");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3005398047i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_texture_offsets" , 3005398047i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_texture_offsets(
            &self,
            font_rid: Rid,
            size: Vector2i,
            texture_index: i64,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_texture_offsets");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3420028887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_texture_offsets" , 3420028887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_glyph_list(&self, font_rid: Rid, size: Vector2i) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    46086620i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_list" , 46086620i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_clear_glyphs(&mut self, font_rid: Rid, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_clear_glyphs");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2450610377i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_clear_glyphs" , 2450610377i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_remove_glyph(&mut self, font_rid: Rid, size: Vector2i, glyph: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_remove_glyph");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3810512262i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_remove_glyph" , 3810512262i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_glyph_advance(&self, font_rid: Rid, size: i64, glyph: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_advance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2555689501i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_advance" , 2555689501i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_glyph_advance(
            &mut self,
            font_rid: Rid,
            size: i64,
            glyph: i64,
            advance: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_glyph_advance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3219397315i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_glyph_advance" , 3219397315i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Vector2 as sys::GodotFfi>::sys_const(&advance),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_glyph_offset(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    513728628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_offset" , 513728628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_glyph_offset(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            offset: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_glyph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1812632090i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_glyph_offset" , 1812632090i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Vector2 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_glyph_size(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    513728628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_size" , 513728628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_glyph_size(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            gl_size: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_glyph_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1812632090i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_glyph_size" , 1812632090i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Vector2 as sys::GodotFfi>::sys_const(&gl_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_glyph_uv_rect(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_uv_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2274268786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_uv_rect" , 2274268786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_glyph_uv_rect(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            uv_rect: Rect2,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_glyph_uv_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1973324081i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_glyph_uv_rect" , 1973324081i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Rect2 as sys::GodotFfi>::sys_const(&uv_rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_glyph_texture_idx(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_texture_idx");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4292800474i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_texture_idx" , 4292800474i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_glyph_texture_idx(
            &mut self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
            texture_idx: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_glyph_texture_idx");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4254580980i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_glyph_texture_idx" , 4254580980i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <i64 as sys::GodotFfi>::sys_const(&texture_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_glyph_texture_rid(&self, font_rid: Rid, size: Vector2i, glyph: i64) -> Rid {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_texture_rid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1451696141i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_texture_rid" , 1451696141i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_glyph_texture_size(
            &self,
            font_rid: Rid,
            size: Vector2i,
            glyph: i64,
        ) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_texture_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    513728628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_texture_size" , 513728628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_glyph_contours(&self, font: Rid, size: i64, index: i64) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_contours");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2903964473i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_contours" , 2903964473i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_kerning_list(&self, font_rid: Rid, size: i64) -> Array<Vector2i> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_kerning_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1778388067i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_kerning_list" , 1778388067i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<Vector2i> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_clear_kerning_map(&mut self, font_rid: Rid, size: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_clear_kerning_map");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_clear_kerning_map" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_remove_kerning(&mut self, font_rid: Rid, size: i64, glyph_pair: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_remove_kerning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2141860016i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_remove_kerning" , 2141860016i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2i as sys::GodotFfi>::sys_const(&glyph_pair),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_set_kerning(
            &mut self,
            font_rid: Rid,
            size: i64,
            glyph_pair: Vector2i,
            kerning: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_kerning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3630965883i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_kerning" , 3630965883i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2i as sys::GodotFfi>::sys_const(&glyph_pair),
                    <Vector2 as sys::GodotFfi>::sys_const(&kerning),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_kerning(&self, font_rid: Rid, size: i64, glyph_pair: Vector2i) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_kerning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1019980169i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_kerning" , 1019980169i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2i as sys::GodotFfi>::sys_const(&glyph_pair),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_glyph_index(
            &self,
            font_rid: Rid,
            size: i64,
            char: i64,
            variation_selector: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_glyph_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1765635060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_glyph_index" , 1765635060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&char),
                    <i64 as sys::GodotFfi>::sys_const(&variation_selector),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_has_char(&self, font_rid: Rid, char: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_has_char");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3120086654i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_has_char" , 3120086654i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <i64 as sys::GodotFfi>::sys_const(&char),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_supported_chars(&self, font_rid: Rid) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_supported_chars");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642473191i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_supported_chars" , 642473191i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_render_range(&mut self, font_rid: Rid, size: Vector2i, start: i64, end: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_render_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4254580980i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_render_range" , 4254580980i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <i64 as sys::GodotFfi>::sys_const(&end),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_render_glyph(&mut self, font_rid: Rid, size: Vector2i, index: i64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_render_glyph");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3810512262i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_render_glyph" , 3810512262i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_draw_glyph(
            &self,
            font_rid: Rid,
            canvas: Rid,
            size: i64,
            pos: Vector2,
            index: i64,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_draw_glyph");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1821196351i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_draw_glyph" , 1821196351i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_draw_glyph_outline(
            &self,
            font_rid: Rid,
            canvas: Rid,
            size: i64,
            outline_size: i64,
            pos: Vector2,
            index: i64,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_draw_glyph_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1124898203i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_draw_glyph_outline" , 1124898203i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&outline_size),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_is_language_supported(&self, font_rid: Rid, language: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_is_language_supported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3199320846i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_is_language_supported" , 3199320846i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_language_support_override(
            &mut self,
            font_rid: Rid,
            language: GodotString,
            supported: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_language_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2313957094i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_language_support_override" , 2313957094i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                    <bool as sys::GodotFfi>::sys_const(&supported),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_language_support_override(
            &mut self,
            font_rid: Rid,
            language: GodotString,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_language_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2829184646i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_language_support_override" , 2829184646i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_remove_language_support_override(
            &mut self,
            font_rid: Rid,
            language: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_remove_language_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_remove_language_support_override" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_language_support_overrides(&mut self, font_rid: Rid) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_language_support_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2801473409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_language_support_overrides" , 2801473409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_is_script_supported(&self, font_rid: Rid, script: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_is_script_supported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3199320846i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_is_script_supported" , 3199320846i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&script),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_script_support_override(
            &mut self,
            font_rid: Rid,
            script: GodotString,
            supported: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_script_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2313957094i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_script_support_override" , 2313957094i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&script),
                    <bool as sys::GodotFfi>::sys_const(&supported),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_script_support_override(
            &mut self,
            font_rid: Rid,
            script: GodotString,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_script_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2829184646i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_script_support_override" , 2829184646i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&script),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_remove_script_support_override(&mut self, font_rid: Rid, script: GodotString) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_remove_script_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_remove_script_support_override" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <GodotString as sys::GodotFfi>::sys_const(&script),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_script_support_overrides(&mut self, font_rid: Rid) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_script_support_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2801473409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_script_support_overrides" , 2801473409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_opentype_feature_overrides(
            &mut self,
            font_rid: Rid,
            overrides: Dictionary,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_opentype_feature_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1217542888i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_opentype_feature_overrides" , 1217542888i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&font_rid),
                    <Dictionary as sys::GodotFfi>::sys_const(&overrides),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn font_get_opentype_feature_overrides(&self, font_rid: Rid) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_opentype_feature_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1882737106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_opentype_feature_overrides" , 1882737106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_supported_feature_list(&self, font_rid: Rid) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_supported_feature_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1882737106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_supported_feature_list" , 1882737106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_supported_variation_list(&self, font_rid: Rid) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_supported_variation_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1882737106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_supported_variation_list" , 1882737106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&font_rid)];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_get_global_oversampling(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_get_global_oversampling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_get_global_oversampling" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn font_set_global_oversampling(&mut self, oversampling: f64) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("font_set_global_oversampling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "font_set_global_oversampling" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&oversampling)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_hex_code_box_size(&self, size: i64, index: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("get_hex_code_box_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3016396712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "get_hex_code_box_size" , 3016396712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_hex_code_box(
            &self,
            canvas: Rid,
            size: i64,
            pos: Vector2,
            index: i64,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("draw_hex_code_box");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1602046441i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "draw_hex_code_box" , 1602046441i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_shaped_text(
            &mut self,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("create_shaped_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1231398698i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "create_shaped_text" , 1231398698i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_clear(&mut self, rid: Rid) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_clear" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&rid)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_set_direction(
            &mut self,
            shaped: Rid,
            direction: text_server::Direction,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2616949700i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_direction" , 2616949700i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_direction(&self, shaped: Rid) -> text_server::Direction {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3065904362i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_direction" , 3065904362i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <text_server::Direction as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_inferred_direction(&self, shaped: Rid) -> text_server::Direction {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_inferred_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3065904362i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_inferred_direction" , 3065904362i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <text_server::Direction as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_set_bidi_override(&mut self, shaped: Rid, override_: VariantArray) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_bidi_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    684822712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_bidi_override" , 684822712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <VariantArray as sys::GodotFfi>::sys_const(&override_),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_set_custom_punctuation(&mut self, shaped: Rid, punct: GodotString) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_custom_punctuation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_custom_punctuation" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <GodotString as sys::GodotFfi>::sys_const(&punct),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_custom_punctuation(&self, shaped: Rid) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_custom_punctuation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642473191i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_custom_punctuation" , 642473191i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_set_orientation(
            &mut self,
            shaped: Rid,
            orientation: text_server::Orientation,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_orientation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    104095128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_orientation" , 104095128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_orientation(&self, shaped: Rid) -> text_server::Orientation {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_orientation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3142708106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_orientation" , 3142708106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <text_server::Orientation as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_set_preserve_invalid(&mut self, shaped: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_preserve_invalid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_preserve_invalid" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_preserve_invalid(&self, shaped: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_preserve_invalid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_preserve_invalid" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_set_preserve_control(&mut self, shaped: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_preserve_control");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_preserve_control" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_preserve_control(&self, shaped: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_preserve_control");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_preserve_control" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_set_spacing(
            &mut self,
            shaped: Rid,
            spacing: text_server::SpacingType,
            value: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_set_spacing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1307259930i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_set_spacing" , 1307259930i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <text_server::SpacingType as sys::GodotFfi>::sys_const(&spacing),
                    <i64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_spacing(
            &self,
            shaped: Rid,
            spacing: text_server::SpacingType,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_spacing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1213653558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_spacing" , 1213653558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <text_server::SpacingType as sys::GodotFfi>::sys_const(&spacing),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_add_string(
            &mut self,
            shaped: Rid,
            text: GodotString,
            fonts: Array<Rid>,
            size: i64,
            opentype_features: Dictionary,
            language: GodotString,
            meta: Variant,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_add_string");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2621279422i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_add_string" , 2621279422i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <Array<Rid> as sys::GodotFfi>::sys_const(&fonts),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Dictionary as sys::GodotFfi>::sys_const(&opentype_features),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                    <Variant as sys::GodotFfi>::sys_const(&meta),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_add_object(
            &mut self,
            shaped: Rid,
            key: Variant,
            size: Vector2,
            inline_align: global::InlineAlignment,
            length: i64,
            baseline: f64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_add_object");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2838446185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_add_object" , 2838446185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <Variant as sys::GodotFfi>::sys_const(&key),
                    <Vector2 as sys::GodotFfi>::sys_const(&size),
                    <global::InlineAlignment as sys::GodotFfi>::sys_const(&inline_align),
                    <i64 as sys::GodotFfi>::sys_const(&length),
                    <f64 as sys::GodotFfi>::sys_const(&baseline),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_resize_object(
            &mut self,
            shaped: Rid,
            key: Variant,
            size: Vector2,
            inline_align: global::InlineAlignment,
            baseline: f64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_resize_object");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2353789835i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_resize_object" , 2353789835i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <Variant as sys::GodotFfi>::sys_const(&key),
                    <Vector2 as sys::GodotFfi>::sys_const(&size),
                    <global::InlineAlignment as sys::GodotFfi>::sys_const(&inline_align),
                    <f64 as sys::GodotFfi>::sys_const(&baseline),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_get_span_count(&self, shaped: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_get_span_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_get_span_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_get_span_meta(&self, shaped: Rid, index: i64) -> Variant {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_get_span_meta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4069510997i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_get_span_meta" , 4069510997i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_set_span_update_font(
            &mut self,
            shaped: Rid,
            index: i64,
            fonts: Array<Rid>,
            size: i64,
            opentype_features: Dictionary,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_set_span_update_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1578983057i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_set_span_update_font" , 1578983057i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Array<Rid> as sys::GodotFfi>::sys_const(&fonts),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Dictionary as sys::GodotFfi>::sys_const(&opentype_features),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_substr(&self, shaped: Rid, start: i64, length: i64) -> Rid {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_substr");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1937682086i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_substr" , 1937682086i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <i64 as sys::GodotFfi>::sys_const(&length),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_parent(&self, shaped: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814569979i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_parent" , 3814569979i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_fit_to_width(
            &mut self,
            shaped: Rid,
            width: f64,
            jst_flags: text_server::JustificationFlag,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_fit_to_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    603718830i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_fit_to_width" , 603718830i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_tab_align(&mut self, shaped: Rid, tab_stops: PackedFloat32Array) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_tab_align");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1283669550i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_tab_align" , 1283669550i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <PackedFloat32Array as sys::GodotFfi>::sys_const(&tab_stops),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_shape(&mut self, shaped: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_shape" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_is_ready(&self, shaped: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_is_ready");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_is_ready" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_glyphs(&self, shaped: Rid) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_glyphs");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_glyphs" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_sort_logical(&mut self, shaped: Rid) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_sort_logical");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2670461153i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_sort_logical" , 2670461153i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_glyph_count(&self, shaped: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_glyph_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_glyph_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_range(&self, shaped: Rid) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    733700038i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_range" , 733700038i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_line_breaks_adv(
            &self,
            shaped: Rid,
            width: PackedFloat32Array,
            start: i64,
            once: bool,
            break_flags: text_server::LineBreakFlag,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_line_breaks_adv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4206849830i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_line_breaks_adv" , 4206849830i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <PackedFloat32Array as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <bool as sys::GodotFfi>::sys_const(&once),
                    <text_server::LineBreakFlag as sys::GodotFfi>::sys_const(&break_flags),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_line_breaks(
            &self,
            shaped: Rid,
            width: f64,
            start: i64,
            break_flags: text_server::LineBreakFlag,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_line_breaks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    303410369i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_line_breaks" , 303410369i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <text_server::LineBreakFlag as sys::GodotFfi>::sys_const(&break_flags),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_word_breaks(
            &self,
            shaped: Rid,
            grapheme_flags: text_server::GraphemeFlag,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_word_breaks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3299477123i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_word_breaks" , 3299477123i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <text_server::GraphemeFlag as sys::GodotFfi>::sys_const(&grapheme_flags),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_trim_pos(&self, shaped: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_trim_pos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_trim_pos" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_ellipsis_pos(&self, shaped: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_ellipsis_pos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_ellipsis_pos" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_ellipsis_glyphs(&self, shaped: Rid) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_ellipsis_glyphs");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_ellipsis_glyphs" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_ellipsis_glyph_count(&self, shaped: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_ellipsis_glyph_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_ellipsis_glyph_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_overrun_trim_to_width(
            &mut self,
            shaped: Rid,
            width: f64,
            overrun_trim_flags: text_server::TextOverrunFlag,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_overrun_trim_to_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1572579718i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_overrun_trim_to_width" , 1572579718i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <text_server::TextOverrunFlag as sys::GodotFfi>::sys_const(&overrun_trim_flags),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_objects(&self, shaped: Rid) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_objects");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_objects" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_object_rect(&self, shaped: Rid, key: Variant) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_object_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    447978354i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_object_rect" , 447978354i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <Variant as sys::GodotFfi>::sys_const(&key),
                ];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_size(&self, shaped: Rid) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2440833711i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_size" , 2440833711i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_ascent(&self, shaped: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_ascent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_ascent" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_descent(&self, shaped: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_descent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_descent" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_width(&self, shaped: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_width" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_underline_position(&self, shaped: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_underline_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_underline_position" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_underline_thickness(&self, shaped: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_underline_thickness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_underline_thickness" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shaped)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_carets(&self, shaped: Rid, position: i64) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_carets");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1574219346i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_carets" , 1574219346i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_selection(
            &self,
            shaped: Rid,
            start: i64,
            end: i64,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_selection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3714187733i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_selection" , 3714187733i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <i64 as sys::GodotFfi>::sys_const(&end),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_hit_test_grapheme(&self, shaped: Rid, coords: f64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_hit_test_grapheme");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3149310417i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_hit_test_grapheme" , 3149310417i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <f64 as sys::GodotFfi>::sys_const(&coords),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_hit_test_position(&self, shaped: Rid, coords: f64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_hit_test_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3149310417i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_hit_test_position" , 3149310417i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <f64 as sys::GodotFfi>::sys_const(&coords),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_get_grapheme_bounds(&self, shaped: Rid, pos: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_grapheme_bounds");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2546185844i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_grapheme_bounds" , 2546185844i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&pos),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_next_grapheme_pos(&self, shaped: Rid, pos: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_next_grapheme_pos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1120910005i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_next_grapheme_pos" , 1120910005i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&pos),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_prev_grapheme_pos(&self, shaped: Rid, pos: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_prev_grapheme_pos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1120910005i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_prev_grapheme_pos" , 1120910005i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&pos),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shaped_text_draw(
            &self,
            shaped: Rid,
            canvas: Rid,
            pos: Vector2,
            clip_l: f64,
            clip_r: f64,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    70679950i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_draw" , 70679950i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <f64 as sys::GodotFfi>::sys_const(&clip_l),
                    <f64 as sys::GodotFfi>::sys_const(&clip_r),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_draw_outline(
            &self,
            shaped: Rid,
            canvas: Rid,
            pos: Vector2,
            clip_l: f64,
            clip_r: f64,
            outline_size: i64,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_draw_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2673671346i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_draw_outline" , 2673671346i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <f64 as sys::GodotFfi>::sys_const(&clip_l),
                    <f64 as sys::GodotFfi>::sys_const(&clip_r),
                    <i64 as sys::GodotFfi>::sys_const(&outline_size),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shaped_text_get_dominant_direction_in_range(
            &self,
            shaped: Rid,
            start: i64,
            end: i64,
        ) -> text_server::Direction {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("shaped_text_get_dominant_direction_in_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3326907668i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "shaped_text_get_dominant_direction_in_range" , 3326907668i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shaped),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <i64 as sys::GodotFfi>::sys_const(&end),
                ];
                let __args_ptr = __args.as_ptr();
                <text_server::Direction as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn format_number(&self, number: GodotString, language: GodotString) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("format_number");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2305636099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "format_number" , 2305636099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&number),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn parse_number(&self, number: GodotString, language: GodotString) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("parse_number");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2305636099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "parse_number" , 2305636099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&number),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn percent_sign(&self, language: GodotString) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("percent_sign");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    993269549i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "percent_sign" , 993269549i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&language)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn string_get_word_breaks(
            &self,
            string: GodotString,
            language: GodotString,
            chars_per_line: i64,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("string_get_word_breaks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1398910359i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "string_get_word_breaks" , 1398910359i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&string),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                    <i64 as sys::GodotFfi>::sys_const(&chars_per_line),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_confusable(&self, string: GodotString, dict: PackedStringArray) -> i64 {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("is_confusable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1433197768i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "is_confusable" , 1433197768i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&string),
                    <PackedStringArray as sys::GodotFfi>::sys_const(&dict),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn spoof_check(&self, string: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("spoof_check");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "spoof_check" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&string)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn strip_diacritics(&self, string: GodotString) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("strip_diacritics");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3135753539i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "strip_diacritics" , 3135753539i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&string)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_valid_identifier(&self, string: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("is_valid_identifier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "is_valid_identifier" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&string)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn string_to_upper(&self, string: GodotString, language: GodotString) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("string_to_upper");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2305636099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "string_to_upper" , 2305636099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&string),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn string_to_lower(&self, string: GodotString, language: GodotString) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("string_to_lower");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2305636099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "string_to_lower" , 2305636099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&string),
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn parse_structured_text(
            &self,
            parser_type: text_server::StructuredTextParser,
            args: VariantArray,
            text: GodotString,
        ) -> Array<Vector3i> {
            unsafe {
                let __class_name = StringName::from("TextServer");
                let __method_name = StringName::from("parse_structured_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3310685015i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TextServer" , "parse_structured_text" , 3310685015i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <text_server::StructuredTextParser as sys::GodotFfi>::sys_const(&parser_type),
                    <VariantArray as sys::GodotFfi>::sys_const(&args),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<Vector3i> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for TextServer {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "TextServer";
    }
    impl crate::obj::EngineClass for TextServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for TextServer {}
    impl crate::obj::Inherits<crate::engine::Object> for TextServer {}
    impl std::ops::Deref for TextServer {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for TextServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_TextServer {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::TextServer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FontAntialiasing {
    ord: i32,
}
impl FontAntialiasing {
    pub const FONT_ANTIALIASING_NONE: Self = Self { ord: 0 };
    pub const FONT_ANTIALIASING_GRAY: Self = Self { ord: 1 };
    pub const FONT_ANTIALIASING_LCD: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for FontAntialiasing {
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
unsafe impl sys::GodotFfi for FontAntialiasing {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FontLCDSubpixelLayout {
    ord: i32,
}
impl FontLCDSubpixelLayout {
    pub const FONT_LCD_SUBPIXEL_LAYOUT_NONE: Self = Self { ord: 0 };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_HRGB: Self = Self { ord: 1 };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_HBGR: Self = Self { ord: 2 };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_VRGB: Self = Self { ord: 3 };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_VBGR: Self = Self { ord: 4 };
    pub const FONT_LCD_SUBPIXEL_LAYOUT_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for FontLCDSubpixelLayout {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for FontLCDSubpixelLayout {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Direction {
    ord: i32,
}
impl Direction {
    pub const DIRECTION_AUTO: Self = Self { ord: 0 };
    pub const DIRECTION_LTR: Self = Self { ord: 1 };
    pub const DIRECTION_RTL: Self = Self { ord: 2 };
    pub const DIRECTION_INHERITED: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for Direction {
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
unsafe impl sys::GodotFfi for Direction {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Orientation {
    ord: i32,
}
impl Orientation {
    pub const ORIENTATION_HORIZONTAL: Self = Self { ord: 0 };
    pub const ORIENTATION_VERTICAL: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for Orientation {
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
unsafe impl sys::GodotFfi for Orientation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct JustificationFlag {
    ord: i32,
}
impl JustificationFlag {
    pub const JUSTIFICATION_NONE: Self = Self { ord: 0 };
    pub const JUSTIFICATION_KASHIDA: Self = Self { ord: 1 };
    pub const JUSTIFICATION_WORD_BOUND: Self = Self { ord: 2 };
    pub const JUSTIFICATION_TRIM_EDGE_SPACES: Self = Self { ord: 4 };
    pub const JUSTIFICATION_AFTER_LAST_TAB: Self = Self { ord: 8 };
    pub const JUSTIFICATION_CONSTRAIN_ELLIPSIS: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for JustificationFlag {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for JustificationFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for JustificationFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AutowrapMode {
    ord: i32,
}
impl AutowrapMode {
    pub const AUTOWRAP_OFF: Self = Self { ord: 0 };
    pub const AUTOWRAP_ARBITRARY: Self = Self { ord: 1 };
    pub const AUTOWRAP_WORD: Self = Self { ord: 2 };
    pub const AUTOWRAP_WORD_SMART: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for AutowrapMode {
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
unsafe impl sys::GodotFfi for AutowrapMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct LineBreakFlag {
    ord: i32,
}
impl LineBreakFlag {
    pub const BREAK_NONE: Self = Self { ord: 0 };
    pub const BREAK_MANDATORY: Self = Self { ord: 1 };
    pub const BREAK_WORD_BOUND: Self = Self { ord: 2 };
    pub const BREAK_GRAPHEME_BOUND: Self = Self { ord: 4 };
    pub const BREAK_ADAPTIVE: Self = Self { ord: 8 };
    pub const BREAK_TRIM_EDGE_SPACES: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for LineBreakFlag {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for LineBreakFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for LineBreakFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VisibleCharactersBehavior {
    ord: i32,
}
impl VisibleCharactersBehavior {
    pub const VC_CHARS_BEFORE_SHAPING: Self = Self { ord: 0 };
    pub const VC_CHARS_AFTER_SHAPING: Self = Self { ord: 1 };
    pub const VC_GLYPHS_AUTO: Self = Self { ord: 2 };
    pub const VC_GLYPHS_LTR: Self = Self { ord: 3 };
    pub const VC_GLYPHS_RTL: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for VisibleCharactersBehavior {
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
unsafe impl sys::GodotFfi for VisibleCharactersBehavior {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OverrunBehavior {
    ord: i32,
}
impl OverrunBehavior {
    pub const OVERRUN_NO_TRIMMING: Self = Self { ord: 0 };
    pub const OVERRUN_TRIM_CHAR: Self = Self { ord: 1 };
    pub const OVERRUN_TRIM_WORD: Self = Self { ord: 2 };
    pub const OVERRUN_TRIM_ELLIPSIS: Self = Self { ord: 3 };
    pub const OVERRUN_TRIM_WORD_ELLIPSIS: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for OverrunBehavior {
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
unsafe impl sys::GodotFfi for OverrunBehavior {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct TextOverrunFlag {
    ord: i32,
}
impl TextOverrunFlag {
    pub const OVERRUN_NO_TRIM: Self = Self { ord: 0 };
    pub const OVERRUN_TRIM: Self = Self { ord: 1 };
    pub const OVERRUN_TRIM_WORD_ONLY: Self = Self { ord: 2 };
    pub const OVERRUN_ADD_ELLIPSIS: Self = Self { ord: 4 };
    pub const OVERRUN_ENFORCE_ELLIPSIS: Self = Self { ord: 8 };
    pub const OVERRUN_JUSTIFICATION_AWARE: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for TextOverrunFlag {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TextOverrunFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for TextOverrunFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct GraphemeFlag {
    ord: i32,
}
impl GraphemeFlag {
    pub const GRAPHEME_IS_VALID: Self = Self { ord: 1 };
    pub const GRAPHEME_IS_RTL: Self = Self { ord: 2 };
    pub const GRAPHEME_IS_VIRTUAL: Self = Self { ord: 4 };
    pub const GRAPHEME_IS_SPACE: Self = Self { ord: 8 };
    pub const GRAPHEME_IS_BREAK_HARD: Self = Self { ord: 16 };
    pub const GRAPHEME_IS_BREAK_SOFT: Self = Self { ord: 32 };
    pub const GRAPHEME_IS_TAB: Self = Self { ord: 64 };
    pub const GRAPHEME_IS_ELONGATION: Self = Self { ord: 128 };
    pub const GRAPHEME_IS_PUNCTUATION: Self = Self { ord: 256 };
    pub const GRAPHEME_IS_UNDERSCORE: Self = Self { ord: 512 };
    pub const GRAPHEME_IS_CONNECTED: Self = Self { ord: 1024 };
    pub const GRAPHEME_IS_SAFE_TO_INSERT_TATWEEL: Self = Self { ord: 2048 };
}
impl crate::obj::EngineEnum for GraphemeFlag {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32
            | ord @ 2i32
            | ord @ 4i32
            | ord @ 8i32
            | ord @ 16i32
            | ord @ 32i32
            | ord @ 64i32
            | ord @ 128i32
            | ord @ 256i32
            | ord @ 512i32
            | ord @ 1024i32
            | ord @ 2048i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for GraphemeFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for GraphemeFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Hinting {
    ord: i32,
}
impl Hinting {
    pub const HINTING_NONE: Self = Self { ord: 0 };
    pub const HINTING_LIGHT: Self = Self { ord: 1 };
    pub const HINTING_NORMAL: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for Hinting {
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
unsafe impl sys::GodotFfi for Hinting {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SubpixelPositioning {
    ord: i32,
}
impl SubpixelPositioning {
    pub const SUBPIXEL_POSITIONING_DISABLED: Self = Self { ord: 0 };
    pub const SUBPIXEL_POSITIONING_AUTO: Self = Self { ord: 1 };
    pub const SUBPIXEL_POSITIONING_ONE_HALF: Self = Self { ord: 2 };
    pub const SUBPIXEL_POSITIONING_ONE_QUARTER: Self = Self { ord: 3 };
    pub const SUBPIXEL_POSITIONING_ONE_HALF_MAX_SIZE: Self = Self { ord: 20 };
    pub const SUBPIXEL_POSITIONING_ONE_QUARTER_MAX_SIZE: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for SubpixelPositioning {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 16i32 | ord @ 20i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for SubpixelPositioning {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Feature {
    ord: i32,
}
impl Feature {
    pub const FEATURE_SIMPLE_LAYOUT: Self = Self { ord: 1 };
    pub const FEATURE_BIDI_LAYOUT: Self = Self { ord: 2 };
    pub const FEATURE_VERTICAL_LAYOUT: Self = Self { ord: 4 };
    pub const FEATURE_SHAPING: Self = Self { ord: 8 };
    pub const FEATURE_KASHIDA_JUSTIFICATION: Self = Self { ord: 16 };
    pub const FEATURE_BREAK_ITERATORS: Self = Self { ord: 32 };
    pub const FEATURE_FONT_BITMAP: Self = Self { ord: 64 };
    pub const FEATURE_FONT_DYNAMIC: Self = Self { ord: 128 };
    pub const FEATURE_FONT_MSDF: Self = Self { ord: 256 };
    pub const FEATURE_FONT_SYSTEM: Self = Self { ord: 512 };
    pub const FEATURE_FONT_VARIABLE: Self = Self { ord: 1024 };
    pub const FEATURE_CONTEXT_SENSITIVE_CASE_CONVERSION: Self = Self { ord: 2048 };
    pub const FEATURE_USE_SUPPORT_DATA: Self = Self { ord: 4096 };
    pub const FEATURE_UNICODE_IDENTIFIERS: Self = Self { ord: 8192 };
    pub const FEATURE_UNICODE_SECURITY: Self = Self { ord: 16384 };
}
impl crate::obj::EngineEnum for Feature {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32
            | ord @ 2i32
            | ord @ 4i32
            | ord @ 8i32
            | ord @ 16i32
            | ord @ 32i32
            | ord @ 64i32
            | ord @ 128i32
            | ord @ 256i32
            | ord @ 512i32
            | ord @ 1024i32
            | ord @ 2048i32
            | ord @ 4096i32
            | ord @ 8192i32
            | ord @ 16384i32 => Some(Self { ord }),
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
pub struct ContourPointTag {
    ord: i32,
}
impl ContourPointTag {
    pub const CONTOUR_CURVE_TAG_ON: Self = Self { ord: 1 };
    pub const CONTOUR_CURVE_TAG_OFF_CONIC: Self = Self { ord: 0 };
    pub const CONTOUR_CURVE_TAG_OFF_CUBIC: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ContourPointTag {
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
unsafe impl sys::GodotFfi for ContourPointTag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SpacingType {
    ord: i32,
}
impl SpacingType {
    pub const SPACING_GLYPH: Self = Self { ord: 0 };
    pub const SPACING_SPACE: Self = Self { ord: 1 };
    pub const SPACING_TOP: Self = Self { ord: 2 };
    pub const SPACING_BOTTOM: Self = Self { ord: 3 };
    pub const SPACING_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for SpacingType {
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
unsafe impl sys::GodotFfi for SpacingType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct FontStyle {
    ord: i32,
}
impl FontStyle {
    pub const FONT_BOLD: Self = Self { ord: 1 };
    pub const FONT_ITALIC: Self = Self { ord: 2 };
    pub const FONT_FIXED_WIDTH: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for FontStyle {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for FontStyle {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for FontStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct StructuredTextParser {
    ord: i32,
}
impl StructuredTextParser {
    pub const STRUCTURED_TEXT_DEFAULT: Self = Self { ord: 0 };
    pub const STRUCTURED_TEXT_URI: Self = Self { ord: 1 };
    pub const STRUCTURED_TEXT_FILE: Self = Self { ord: 2 };
    pub const STRUCTURED_TEXT_EMAIL: Self = Self { ord: 3 };
    pub const STRUCTURED_TEXT_LIST: Self = Self { ord: 4 };
    pub const STRUCTURED_TEXT_GDSCRIPT: Self = Self { ord: 5 };
    pub const STRUCTURED_TEXT_CUSTOM: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for StructuredTextParser {
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
unsafe impl sys::GodotFfi for StructuredTextParser {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
