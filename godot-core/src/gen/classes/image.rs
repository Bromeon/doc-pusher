#![doc = "Sidecar module for class [`Image`][crate::engine::Image].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Image` enums](https://docs.godotengine.org/en/stable/classes/class_image.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Image.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`image`][crate::engine::image]: sidecar module with related enum/flag types\n* [`ImageVirtual`][crate::engine::ImageVirtual]: virtual methods\n\n\nSee also [Godot docs for `Image`](https://docs.godotengine.org/en/stable/classes/class_image.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Image {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Image`][crate::engine::Image].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Image` methods](https://docs.godotengine.org/en/stable/classes/class_image.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ImageVirtual:
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
    impl Image {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Image");
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
        pub fn get_width(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_width" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_height(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_height" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_size(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_size" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_mipmaps(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("has_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "has_mipmaps" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_format(&self) -> image::Format {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3847873762i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_format" , 3847873762i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <image::Format as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_data(&self) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2362200018i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_data" , 2362200018i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn convert(&mut self, format: image::Format) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("convert");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2120693146i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "convert" , 2120693146i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<image::Format as sys::GodotFfi>::sys_const(&format)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_mipmap_offset(&self, mipmap: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_mipmap_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_mipmap_offset" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&mipmap)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn resize_to_po2(&mut self, square: bool, interpolation: image::Interpolation) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("resize_to_po2");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4189212329i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "resize_to_po2" , 4189212329i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <bool as sys::GodotFfi>::sys_const(&square),
                    <image::Interpolation as sys::GodotFfi>::sys_const(&interpolation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn resize(&mut self, width: i64, height: i64, interpolation: image::Interpolation) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("resize");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2461393748i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "resize" , 2461393748i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                    <image::Interpolation as sys::GodotFfi>::sys_const(&interpolation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shrink_x2(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("shrink_x2");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "shrink_x2" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn crop(&mut self, width: i64, height: i64) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("crop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "crop" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn flip_x(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("flip_x");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "flip_x" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn flip_y(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("flip_y");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "flip_y" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn generate_mipmaps(&mut self, renormalize: bool) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("generate_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1633102583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "generate_mipmaps" , 1633102583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&renormalize)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_mipmaps(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("clear_mipmaps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "clear_mipmaps" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create(
            width: i64,
            height: i64,
            use_mipmaps: bool,
            format: image::Format,
        ) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    986942177i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "create" , 986942177i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                    <bool as sys::GodotFfi>::sys_const(&use_mipmaps),
                    <image::Format as sys::GodotFfi>::sys_const(&format),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_from_data(
            width: i64,
            height: i64,
            use_mipmaps: bool,
            format: image::Format,
            data: PackedByteArray,
        ) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("create_from_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    299398494i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "create_from_data" , 299398494i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                    <bool as sys::GodotFfi>::sys_const(&use_mipmaps),
                    <image::Format as sys::GodotFfi>::sys_const(&format),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_data(
            &mut self,
            width: i64,
            height: i64,
            use_mipmaps: bool,
            format: image::Format,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("set_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2740482212i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "set_data" , 2740482212i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                    <bool as sys::GodotFfi>::sys_const(&use_mipmaps),
                    <image::Format as sys::GodotFfi>::sys_const(&format),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_empty(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("is_empty");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "is_empty" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load(&mut self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166001499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load" , 166001499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_from_file(path: GodotString) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load_from_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    736337515i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load_from_file" , 736337515i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_png(&self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_png");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2113323047i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_png" , 2113323047i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_png_to_buffer(&self) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_png_to_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2362200018i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_png_to_buffer" , 2362200018i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_jpg(&self, path: GodotString, quality: f64) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_jpg");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    578836491i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_jpg" , 578836491i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                    <f64 as sys::GodotFfi>::sys_const(&quality),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_jpg_to_buffer(&self, quality: f64) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_jpg_to_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    310747435i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_jpg_to_buffer" , 310747435i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&quality)];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_exr(&self, path: GodotString, grayscale: bool) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_exr");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3108122999i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_exr" , 3108122999i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                    <bool as sys::GodotFfi>::sys_const(&grayscale),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_exr_to_buffer(&self, grayscale: bool) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_exr_to_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3178917920i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_exr_to_buffer" , 3178917920i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&grayscale)];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_webp(&self, path: GodotString, lossy: bool, quality: f64) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_webp");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3594949219i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_webp" , 3594949219i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                    <bool as sys::GodotFfi>::sys_const(&lossy),
                    <f64 as sys::GodotFfi>::sys_const(&quality),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_webp_to_buffer(&self, lossy: bool, quality: f64) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("save_webp_to_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1235769281i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "save_webp_to_buffer" , 1235769281i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <bool as sys::GodotFfi>::sys_const(&lossy),
                    <f64 as sys::GodotFfi>::sys_const(&quality),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn detect_alpha(&self) -> image::AlphaMode {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("detect_alpha");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2030116505i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "detect_alpha" , 2030116505i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <image::AlphaMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_invisible(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("is_invisible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "is_invisible" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn detect_used_channels(&self, source: image::CompressSource) -> image::UsedChannels {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("detect_used_channels");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2703139984i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "detect_used_channels" , 2703139984i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<image::CompressSource as sys::GodotFfi>::sys_const(&source)];
                let __args_ptr = __args.as_ptr();
                <image::UsedChannels as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn compress(
            &mut self,
            mode: image::CompressMode,
            source: image::CompressSource,
            astc_format: image::ASTCFormat,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("compress");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4094210332i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "compress" , 4094210332i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <image::CompressMode as sys::GodotFfi>::sys_const(&mode),
                    <image::CompressSource as sys::GodotFfi>::sys_const(&source),
                    <image::ASTCFormat as sys::GodotFfi>::sys_const(&astc_format),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn compress_from_channels(
            &mut self,
            mode: image::CompressMode,
            channels: image::UsedChannels,
            astc_format: image::ASTCFormat,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("compress_from_channels");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    279105990i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "compress_from_channels" , 279105990i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <image::CompressMode as sys::GodotFfi>::sys_const(&mode),
                    <image::UsedChannels as sys::GodotFfi>::sys_const(&channels),
                    <image::ASTCFormat as sys::GodotFfi>::sys_const(&astc_format),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn decompress(&mut self) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("decompress");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166280745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "decompress" , 166280745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_compressed(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("is_compressed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "is_compressed" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rotate_90(&mut self, direction: global::ClockDirection) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("rotate_90");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1901204267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "rotate_90" , 1901204267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::ClockDirection as sys::GodotFfi>::sys_const(
                    &direction,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rotate_180(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("rotate_180");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "rotate_180" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fix_alpha_edges(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("fix_alpha_edges");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "fix_alpha_edges" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn premultiply_alpha(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("premultiply_alpha");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "premultiply_alpha" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn srgb_to_linear(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("srgb_to_linear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "srgb_to_linear" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn normal_map_to_xy(&mut self) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("normal_map_to_xy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "normal_map_to_xy" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rgbe_to_srgb(&mut self) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("rgbe_to_srgb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    564927088i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "rgbe_to_srgb" , 564927088i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bump_map_to_normal_map(&mut self, bump_scale: f64) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("bump_map_to_normal_map");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    336773324i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "bump_map_to_normal_map" , 336773324i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&bump_scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_image_metrics(
            &mut self,
            compared_image: Gd<Image>,
            use_luma: bool,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("compute_image_metrics");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3080961247i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "compute_image_metrics" , 3080961247i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&compared_image),
                    <bool as sys::GodotFfi>::sys_const(&use_luma),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn blit_rect(&mut self, src: Gd<Image>, src_rect: Rect2i, dst: Vector2i) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("blit_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2903928755i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "blit_rect" , 2903928755i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&src),
                    <Rect2i as sys::GodotFfi>::sys_const(&src_rect),
                    <Vector2i as sys::GodotFfi>::sys_const(&dst),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn blit_rect_mask(
            &mut self,
            src: Gd<Image>,
            mask: Gd<Image>,
            src_rect: Rect2i,
            dst: Vector2i,
        ) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("blit_rect_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3383581145i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "blit_rect_mask" , 3383581145i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&src),
                    <Gd<Image> as AsArg>::as_arg_ptr(&mask),
                    <Rect2i as sys::GodotFfi>::sys_const(&src_rect),
                    <Vector2i as sys::GodotFfi>::sys_const(&dst),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn blend_rect(&mut self, src: Gd<Image>, src_rect: Rect2i, dst: Vector2i) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("blend_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2903928755i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "blend_rect" , 2903928755i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&src),
                    <Rect2i as sys::GodotFfi>::sys_const(&src_rect),
                    <Vector2i as sys::GodotFfi>::sys_const(&dst),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn blend_rect_mask(
            &mut self,
            src: Gd<Image>,
            mask: Gd<Image>,
            src_rect: Rect2i,
            dst: Vector2i,
        ) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("blend_rect_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3383581145i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "blend_rect_mask" , 3383581145i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&src),
                    <Gd<Image> as AsArg>::as_arg_ptr(&mask),
                    <Rect2i as sys::GodotFfi>::sys_const(&src_rect),
                    <Vector2i as sys::GodotFfi>::sys_const(&dst),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fill(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("fill");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "fill" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fill_rect(&mut self, rect: Rect2i, color: Color) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("fill_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    514693913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "fill_rect" , 514693913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rect2i as sys::GodotFfi>::sys_const(&rect),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_used_rect(&self) -> Rect2i {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_used_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    410525958i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_used_rect" , 410525958i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_region(&self, region: Rect2i) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2601441065i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_region" , 2601441065i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rect2i as sys::GodotFfi>::sys_const(&region)];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn copy_from(&mut self, src: Gd<Image>) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("copy_from");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    532598488i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "copy_from" , 532598488i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Image> as AsArg>::as_arg_ptr(&src)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_pixelv(&self, point: Vector2i) -> Color {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_pixelv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1532707496i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_pixelv" , 1532707496i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&point)];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_pixel(&self, x: i64, y: i64) -> Color {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("get_pixel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2165839948i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "get_pixel" , 2165839948i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&x),
                    <i64 as sys::GodotFfi>::sys_const(&y),
                ];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_pixelv(&mut self, point: Vector2i, color: Color) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("set_pixelv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    287851464i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "set_pixelv" , 287851464i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&point),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_pixel(&mut self, x: i64, y: i64, color: Color) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("set_pixel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3733378741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "set_pixel" , 3733378741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&x),
                    <i64 as sys::GodotFfi>::sys_const(&y),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn adjust_bcs(&mut self, brightness: f64, contrast: f64, saturation: f64) {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("adjust_bcs");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2385087082i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "adjust_bcs" , 2385087082i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&brightness),
                    <f64 as sys::GodotFfi>::sys_const(&contrast),
                    <f64 as sys::GodotFfi>::sys_const(&saturation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn load_png_from_buffer(&mut self, buffer: PackedByteArray) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load_png_from_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    680677267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load_png_from_buffer" , 680677267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&buffer)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_jpg_from_buffer(&mut self, buffer: PackedByteArray) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load_jpg_from_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    680677267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load_jpg_from_buffer" , 680677267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&buffer)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_webp_from_buffer(&mut self, buffer: PackedByteArray) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load_webp_from_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    680677267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load_webp_from_buffer" , 680677267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&buffer)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_tga_from_buffer(&mut self, buffer: PackedByteArray) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load_tga_from_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    680677267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load_tga_from_buffer" , 680677267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&buffer)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_bmp_from_buffer(&mut self, buffer: PackedByteArray) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Image");
                let __method_name = StringName::from("load_bmp_from_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    680677267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Image" , "load_bmp_from_buffer" , 680677267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&buffer)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub const MAX_WIDTH: i32 = 16777216i32;
        pub const MAX_HEIGHT: i32 = 16777216i32;
    }
    impl crate::obj::GodotClass for Image {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Image";
    }
    impl crate::obj::EngineClass for Image {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Image {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Image {}
    impl crate::obj::Inherits<crate::engine::Object> for Image {}
    impl std::ops::Deref for Image {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Image {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Image {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Image> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Format {
    ord: i32,
}
impl Format {
    pub const FORMAT_L8: Self = Self { ord: 0 };
    pub const FORMAT_LA8: Self = Self { ord: 1 };
    pub const FORMAT_R8: Self = Self { ord: 2 };
    pub const FORMAT_RG8: Self = Self { ord: 3 };
    pub const FORMAT_RGB8: Self = Self { ord: 4 };
    pub const FORMAT_RGBA8: Self = Self { ord: 5 };
    pub const FORMAT_RGBA4444: Self = Self { ord: 6 };
    pub const FORMAT_RGB565: Self = Self { ord: 7 };
    pub const FORMAT_RF: Self = Self { ord: 8 };
    pub const FORMAT_RGF: Self = Self { ord: 9 };
    pub const FORMAT_RGBF: Self = Self { ord: 10 };
    pub const FORMAT_RGBAF: Self = Self { ord: 11 };
    pub const FORMAT_RH: Self = Self { ord: 12 };
    pub const FORMAT_RGH: Self = Self { ord: 13 };
    pub const FORMAT_RGBH: Self = Self { ord: 14 };
    pub const FORMAT_RGBAH: Self = Self { ord: 15 };
    pub const FORMAT_RGBE9995: Self = Self { ord: 16 };
    pub const FORMAT_DXT1: Self = Self { ord: 17 };
    pub const FORMAT_DXT3: Self = Self { ord: 18 };
    pub const FORMAT_DXT5: Self = Self { ord: 19 };
    pub const FORMAT_RGTC_R: Self = Self { ord: 20 };
    pub const FORMAT_RGTC_RG: Self = Self { ord: 21 };
    pub const FORMAT_BPTC_RGBA: Self = Self { ord: 22 };
    pub const FORMAT_BPTC_RGBF: Self = Self { ord: 23 };
    pub const FORMAT_BPTC_RGBFU: Self = Self { ord: 24 };
    pub const FORMAT_ETC: Self = Self { ord: 25 };
    pub const FORMAT_ETC2_R11: Self = Self { ord: 26 };
    pub const FORMAT_ETC2_R11S: Self = Self { ord: 27 };
    pub const FORMAT_ETC2_RG11: Self = Self { ord: 28 };
    pub const FORMAT_ETC2_RG11S: Self = Self { ord: 29 };
    pub const FORMAT_ETC2_RGB8: Self = Self { ord: 30 };
    pub const FORMAT_ETC2_RGBA8: Self = Self { ord: 31 };
    pub const FORMAT_ETC2_RGB8A1: Self = Self { ord: 32 };
    pub const FORMAT_ETC2_RA_AS_RG: Self = Self { ord: 33 };
    pub const FORMAT_DXT5_RA_AS_RG: Self = Self { ord: 34 };
    pub const FORMAT_ASTC_4x4: Self = Self { ord: 35 };
    pub const FORMAT_ASTC_4x4_HDR: Self = Self { ord: 36 };
    pub const FORMAT_ASTC_8x8: Self = Self { ord: 37 };
    pub const FORMAT_ASTC_8x8_HDR: Self = Self { ord: 38 };
    pub const FORMAT_MAX: Self = Self { ord: 39 };
}
impl crate::obj::EngineEnum for Format {
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
            | ord @ 17i32
            | ord @ 18i32
            | ord @ 19i32
            | ord @ 20i32
            | ord @ 21i32
            | ord @ 22i32
            | ord @ 23i32
            | ord @ 24i32
            | ord @ 25i32
            | ord @ 26i32
            | ord @ 27i32
            | ord @ 28i32
            | ord @ 29i32
            | ord @ 30i32
            | ord @ 31i32
            | ord @ 32i32
            | ord @ 33i32
            | ord @ 34i32
            | ord @ 35i32
            | ord @ 36i32
            | ord @ 37i32
            | ord @ 38i32
            | ord @ 39i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Format {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Interpolation {
    ord: i32,
}
impl Interpolation {
    pub const INTERPOLATE_NEAREST: Self = Self { ord: 0 };
    pub const INTERPOLATE_BILINEAR: Self = Self { ord: 1 };
    pub const INTERPOLATE_CUBIC: Self = Self { ord: 2 };
    pub const INTERPOLATE_TRILINEAR: Self = Self { ord: 3 };
    pub const INTERPOLATE_LANCZOS: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for Interpolation {
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
unsafe impl sys::GodotFfi for Interpolation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AlphaMode {
    ord: i32,
}
impl AlphaMode {
    pub const ALPHA_NONE: Self = Self { ord: 0 };
    pub const ALPHA_BIT: Self = Self { ord: 1 };
    pub const ALPHA_BLEND: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for AlphaMode {
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
unsafe impl sys::GodotFfi for AlphaMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CompressMode {
    ord: i32,
}
impl CompressMode {
    pub const COMPRESS_S3TC: Self = Self { ord: 0 };
    pub const COMPRESS_ETC: Self = Self { ord: 1 };
    pub const COMPRESS_ETC2: Self = Self { ord: 2 };
    pub const COMPRESS_BPTC: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for CompressMode {
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
unsafe impl sys::GodotFfi for CompressMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct UsedChannels {
    ord: i32,
}
impl UsedChannels {
    pub const USED_CHANNELS_L: Self = Self { ord: 0 };
    pub const USED_CHANNELS_LA: Self = Self { ord: 1 };
    pub const USED_CHANNELS_R: Self = Self { ord: 2 };
    pub const USED_CHANNELS_RG: Self = Self { ord: 3 };
    pub const USED_CHANNELS_RGB: Self = Self { ord: 4 };
    pub const USED_CHANNELS_RGBA: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for UsedChannels {
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
unsafe impl sys::GodotFfi for UsedChannels {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CompressSource {
    ord: i32,
}
impl CompressSource {
    pub const COMPRESS_SOURCE_GENERIC: Self = Self { ord: 0 };
    pub const COMPRESS_SOURCE_SRGB: Self = Self { ord: 1 };
    pub const COMPRESS_SOURCE_NORMAL: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for CompressSource {
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
unsafe impl sys::GodotFfi for CompressSource {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ASTCFormat {
    ord: i32,
}
impl ASTCFormat {
    pub const ASTC_FORMAT_4x4: Self = Self { ord: 0 };
    pub const ASTC_FORMAT_8x8: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for ASTCFormat {
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
unsafe impl sys::GodotFfi for ASTCFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
