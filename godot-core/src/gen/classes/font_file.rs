#![doc = "Sidecar module for class [`FontFile`][crate::engine::FontFile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FontFile` enums](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `FontFile.`\n\nInherits [`Font`][crate::engine::Font].\n\nRelated symbols:\n\n* [`FontFileVirtual`][crate::engine::FontFileVirtual]: virtual methods\n\n\nSee also [Godot docs for `FontFile`](https://docs.godotengine.org/en/stable/classes/class_fontfile.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct FontFile {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`FontFile`][crate::engine::FontFile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FontFile` methods](https://docs.godotengine.org/en/stable/classes/class_fontfile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait FontFileVirtual:
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
    impl FontFile {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("FontFile");
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
        pub fn load_bitmap_font(&mut self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("load_bitmap_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166001499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "load_bitmap_font" , 166001499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_dynamic_font(&mut self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("load_dynamic_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166001499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "load_dynamic_font" , 166001499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_data(&mut self, data: PackedByteArray) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2971499966i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_data" , 2971499966i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&data)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_data(&self) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2362200018i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_data" , 2362200018i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_font_name(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_font_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_font_name" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_font_style_name(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_font_style_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_font_style_name" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_font_style(&mut self, style: text_server::FontStyle) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_font_style");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    918070724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_font_style" , 918070724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::FontStyle as sys::GodotFfi>::sys_const(&style)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_font_weight(&mut self, weight: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_font_weight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_font_weight" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&weight)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_font_stretch(&mut self, stretch: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_font_stretch");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_font_stretch" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&stretch)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_antialiasing(&mut self, antialiasing: text_server::FontAntialiasing) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1669900i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_antialiasing" , 1669900i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::FontAntialiasing as sys::GodotFfi>::sys_const(
                    &antialiasing,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_antialiasing(&self) -> text_server::FontAntialiasing {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4262718649i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_antialiasing" , 4262718649i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::FontAntialiasing as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_generate_mipmaps(&mut self, generate_mipmaps: bool) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_generate_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_generate_mipmaps" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&generate_mipmaps)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_generate_mipmaps(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_generate_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_generate_mipmaps" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_multichannel_signed_distance_field(&mut self, msdf: bool) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_multichannel_signed_distance_field");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_multichannel_signed_distance_field" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&msdf)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_multichannel_signed_distance_field(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("is_multichannel_signed_distance_field");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "is_multichannel_signed_distance_field" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_msdf_pixel_range(&mut self, msdf_pixel_range: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_msdf_pixel_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_msdf_pixel_range" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&msdf_pixel_range)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_msdf_pixel_range(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_msdf_pixel_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_msdf_pixel_range" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_msdf_size(&mut self, msdf_size: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_msdf_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_msdf_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&msdf_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_msdf_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_msdf_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_msdf_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fixed_size(&mut self, fixed_size: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_fixed_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_fixed_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&fixed_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fixed_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_fixed_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_fixed_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_allow_system_fallback(&mut self, allow_system_fallback: bool) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_allow_system_fallback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_allow_system_fallback" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&allow_system_fallback)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_allow_system_fallback(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("is_allow_system_fallback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "is_allow_system_fallback" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_force_autohinter(&mut self, force_autohinter: bool) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_force_autohinter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_force_autohinter" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&force_autohinter)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_force_autohinter(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("is_force_autohinter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "is_force_autohinter" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_hinting(&mut self, hinting: text_server::Hinting) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_hinting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1827459492i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_hinting" , 1827459492i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::Hinting as sys::GodotFfi>::sys_const(&hinting)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_hinting(&self) -> text_server::Hinting {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_hinting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3683214614i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_hinting" , 3683214614i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::Hinting as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_subpixel_positioning(
            &mut self,
            subpixel_positioning: text_server::SubpixelPositioning,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_subpixel_positioning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4225742182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_subpixel_positioning" , 4225742182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <text_server::SubpixelPositioning as sys::GodotFfi>::sys_const(
                        &subpixel_positioning,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_subpixel_positioning(&self) -> text_server::SubpixelPositioning {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_subpixel_positioning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1069238588i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_subpixel_positioning" , 1069238588i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::SubpixelPositioning as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_oversampling(&mut self, oversampling: f64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_oversampling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_oversampling" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&oversampling)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_oversampling(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_oversampling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_oversampling" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_cache_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_cache_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_cache_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_cache(&mut self) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("clear_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "clear_cache" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_cache(&mut self, cache_index: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_cache" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_size_cache_list(&self, cache_index: i64) -> Array<Vector2i> {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_size_cache_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    663333327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_size_cache_list" , 663333327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                <Array<Vector2i> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_size_cache(&mut self, cache_index: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("clear_size_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "clear_size_cache" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_size_cache(&mut self, cache_index: i64, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_size_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2311374912i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_size_cache" , 2311374912i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_variation_coordinates(
            &mut self,
            cache_index: i64,
            variation_coordinates: Dictionary,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_variation_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    64545446i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_variation_coordinates" , 64545446i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Dictionary as sys::GodotFfi>::sys_const(&variation_coordinates),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_variation_coordinates(&self, cache_index: i64) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_variation_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3485342025i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_variation_coordinates" , 3485342025i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_embolden(&mut self, cache_index: i64, strength: f64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_embolden");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1602489585i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_embolden" , 1602489585i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <f64 as sys::GodotFfi>::sys_const(&strength),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_embolden(&self, cache_index: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_embolden");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2339986948i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_embolden" , 2339986948i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_transform(&mut self, cache_index: i64, transform: Transform2D) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    30160968i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_transform" , 30160968i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transform(&self, cache_index: i64) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3836996910i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_transform" , 3836996910i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_face_index(&mut self, cache_index: i64, face_index: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_face_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_face_index" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&face_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_face_index(&self, cache_index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_face_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_face_index" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&cache_index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cache_ascent(&mut self, cache_index: i64, size: i64, ascent: f64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_cache_ascent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_cache_ascent" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&ascent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cache_ascent(&self, cache_index: i64, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_cache_ascent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_cache_ascent" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cache_descent(&mut self, cache_index: i64, size: i64, descent: f64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_cache_descent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_cache_descent" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&descent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cache_descent(&self, cache_index: i64, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_cache_descent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_cache_descent" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cache_underline_position(
            &mut self,
            cache_index: i64,
            size: i64,
            underline_position: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_cache_underline_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_cache_underline_position" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&underline_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cache_underline_position(&self, cache_index: i64, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_cache_underline_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_cache_underline_position" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cache_underline_thickness(
            &mut self,
            cache_index: i64,
            size: i64,
            underline_thickness: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_cache_underline_thickness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_cache_underline_thickness" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&underline_thickness),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cache_underline_thickness(&self, cache_index: i64, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_cache_underline_thickness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_cache_underline_thickness" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cache_scale(&mut self, cache_index: i64, size: i64, scale: f64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_cache_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3506521499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_cache_scale" , 3506521499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cache_scale(&self, cache_index: i64, size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_cache_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3085491603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_cache_scale" , 3085491603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_texture_count(&self, cache_index: i64, size: Vector2i) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_texture_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1987661582i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_texture_count" , 1987661582i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_textures(&mut self, cache_index: i64, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("clear_textures");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2311374912i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "clear_textures" , 2311374912i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_texture(&mut self, cache_index: i64, size: Vector2i, texture_index: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2328951467i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_texture" , 2328951467i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_texture_image(
            &mut self,
            cache_index: i64,
            size: Vector2i,
            texture_index: i64,
            image: Gd<Image>,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_texture_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4157974066i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_texture_image" , 4157974066i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                    <Gd<Image> as AsArg>::as_arg_ptr(&image),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_image(
            &self,
            cache_index: i64,
            size: Vector2i,
            texture_index: i64,
        ) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_texture_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3878418953i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_texture_image" , 3878418953i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_offsets(
            &mut self,
            cache_index: i64,
            size: Vector2i,
            texture_index: i64,
            offset: PackedInt32Array,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_texture_offsets");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2849993437i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_texture_offsets" , 2849993437i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_offsets(
            &self,
            cache_index: i64,
            size: Vector2i,
            texture_index: i64,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_texture_offsets");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3703444828i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_texture_offsets" , 3703444828i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&texture_index),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_glyph_list(&self, cache_index: i64, size: Vector2i) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    681709689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_list" , 681709689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_glyphs(&mut self, cache_index: i64, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("clear_glyphs");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2311374912i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "clear_glyphs" , 2311374912i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_glyph(&mut self, cache_index: i64, size: Vector2i, glyph: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_glyph");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2328951467i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_glyph" , 2328951467i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_glyph_advance(
            &mut self,
            cache_index: i64,
            size: i64,
            glyph: i64,
            advance: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_glyph_advance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    947991729i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_glyph_advance" , 947991729i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Vector2 as sys::GodotFfi>::sys_const(&advance),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_glyph_advance(&self, cache_index: i64, size: i64, glyph: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_advance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1601573536i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_advance" , 1601573536i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_glyph_offset(
            &mut self,
            cache_index: i64,
            size: Vector2i,
            glyph: i64,
            offset: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_glyph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    921719850i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_glyph_offset" , 921719850i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Vector2 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_glyph_offset(&self, cache_index: i64, size: Vector2i, glyph: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3205412300i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_offset" , 3205412300i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_glyph_size(
            &mut self,
            cache_index: i64,
            size: Vector2i,
            glyph: i64,
            gl_size: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_glyph_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    921719850i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_glyph_size" , 921719850i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Vector2 as sys::GodotFfi>::sys_const(&gl_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_glyph_size(&self, cache_index: i64, size: Vector2i, glyph: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3205412300i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_size" , 3205412300i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_glyph_uv_rect(
            &mut self,
            cache_index: i64,
            size: Vector2i,
            glyph: i64,
            uv_rect: Rect2,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_glyph_uv_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3821620992i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_glyph_uv_rect" , 3821620992i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <Rect2 as sys::GodotFfi>::sys_const(&uv_rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_glyph_uv_rect(&self, cache_index: i64, size: Vector2i, glyph: i64) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_uv_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927917900i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_uv_rect" , 3927917900i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_glyph_texture_idx(
            &mut self,
            cache_index: i64,
            size: Vector2i,
            glyph: i64,
            texture_idx: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_glyph_texture_idx");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    355564111i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_glyph_texture_idx" , 355564111i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                    <i64 as sys::GodotFfi>::sys_const(&texture_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_glyph_texture_idx(&self, cache_index: i64, size: Vector2i, glyph: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_texture_idx");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1629411054i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_texture_idx" , 1629411054i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&glyph),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_kerning_list(&self, cache_index: i64, size: i64) -> Array<Vector2i> {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_kerning_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2345056839i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_kerning_list" , 2345056839i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<Vector2i> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_kerning_map(&mut self, cache_index: i64, size: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("clear_kerning_map");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "clear_kerning_map" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_kerning(&mut self, cache_index: i64, size: i64, glyph_pair: Vector2i) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_kerning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3930204747i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_kerning" , 3930204747i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2i as sys::GodotFfi>::sys_const(&glyph_pair),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_kerning(
            &mut self,
            cache_index: i64,
            size: i64,
            glyph_pair: Vector2i,
            kerning: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_kerning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3182200918i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_kerning" , 3182200918i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2i as sys::GodotFfi>::sys_const(&glyph_pair),
                    <Vector2 as sys::GodotFfi>::sys_const(&kerning),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_kerning(&self, cache_index: i64, size: i64, glyph_pair: Vector2i) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_kerning");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1611912865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_kerning" , 1611912865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2i as sys::GodotFfi>::sys_const(&glyph_pair),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn render_range(&mut self, cache_index: i64, size: Vector2i, start: i64, end: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("render_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    355564111i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "render_range" , 355564111i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&start),
                    <i64 as sys::GodotFfi>::sys_const(&end),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn render_glyph(&mut self, cache_index: i64, size: Vector2i, index: i64) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("render_glyph");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2328951467i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "render_glyph" , 2328951467i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&cache_index),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_language_support_override(&mut self, language: GodotString, supported: bool) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_language_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2678287736i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_language_support_override" , 2678287736i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&language),
                    <bool as sys::GodotFfi>::sys_const(&supported),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_language_support_override(&self, language: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_language_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_language_support_override" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&language)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_language_support_override(&mut self, language: GodotString) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_language_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_language_support_override" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&language)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_language_support_overrides(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_language_support_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_language_support_overrides" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_script_support_override(&mut self, script: GodotString, supported: bool) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_script_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2678287736i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_script_support_override" , 2678287736i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&script),
                    <bool as sys::GodotFfi>::sys_const(&supported),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_script_support_override(&self, script: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_script_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_script_support_override" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&script)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_script_support_override(&mut self, script: GodotString) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("remove_script_support_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "remove_script_support_override" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&script)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_script_support_overrides(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_script_support_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_script_support_overrides" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_opentype_feature_overrides(&mut self, overrides: Dictionary) {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("set_opentype_feature_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155329257i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "set_opentype_feature_overrides" , 4155329257i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&overrides)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_opentype_feature_overrides(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_opentype_feature_overrides");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_opentype_feature_overrides" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_glyph_index(&self, size: i64, char: i64, variation_selector: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("FontFile");
                let __method_name = StringName::from("get_glyph_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    864943070i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FontFile" , "get_glyph_index" , 864943070i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
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
    }
    impl crate::obj::GodotClass for FontFile {
        type Base = crate::engine::Font;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "FontFile";
    }
    impl crate::obj::EngineClass for FontFile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Font> for FontFile {}
    impl crate::obj::Inherits<crate::engine::Resource> for FontFile {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for FontFile {}
    impl crate::obj::Inherits<crate::engine::Object> for FontFile {}
    impl std::ops::Deref for FontFile {
        type Target = crate::engine::Font;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for FontFile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_FontFile {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::FontFile> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Font> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
