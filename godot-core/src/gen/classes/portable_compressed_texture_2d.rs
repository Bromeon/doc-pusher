#![doc = "Sidecar module for class [`PortableCompressedTexture2D`][crate::engine::PortableCompressedTexture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PortableCompressedTexture2D` enums](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `PortableCompressedTexture2D.`\n\nInherits [`Texture2D`][crate::engine::Texture2D].\n\nRelated symbols:\n\n* [`portable_compressed_texture_2d`][crate::engine::portable_compressed_texture_2d]: sidecar module with related enum/flag types\n* [`PortableCompressedTexture2DVirtual`][crate::engine::PortableCompressedTexture2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `PortableCompressedTexture2D`](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PortableCompressedTexture2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`PortableCompressedTexture2D`][crate::engine::PortableCompressedTexture2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PortableCompressedTexture2D` methods](https://docs.godotengine.org/en/stable/classes/class_portablecompressedtexture2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PortableCompressedTexture2DVirtual:
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
        fn get_width(&self) -> i64 {
            unimplemented!()
        }
        fn get_height(&self) -> i64 {
            unimplemented!()
        }
        fn is_pixel_opaque(&self, x: i64, y: i64) -> bool {
            unimplemented!()
        }
        fn has_alpha(&self) -> bool {
            unimplemented!()
        }
        fn draw(&self, to_canvas_item: Rid, pos: Vector2, modulate: Color, transpose: bool) {
            unimplemented!()
        }
        fn draw_rect(
            &self,
            to_canvas_item: Rid,
            rect: Rect2,
            tile: bool,
            modulate: Color,
            transpose: bool,
        ) {
            unimplemented!()
        }
        fn draw_rect_region(
            &self,
            to_canvas_item: Rid,
            rect: Rect2,
            src_rect: Rect2,
            modulate: Color,
            transpose: bool,
            clip_uv: bool,
        ) {
            unimplemented!()
        }
    }
    impl PortableCompressedTexture2D {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
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
        pub fn create_from_image(
            &mut self,
            image: Gd<Image>,
            compression_mode: portable_compressed_texture_2d::CompressionMode,
            normal_map: bool,
            lossy_quality: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("create_from_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    97251393i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "create_from_image" , 97251393i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&image),
                    <portable_compressed_texture_2d::CompressionMode as sys::GodotFfi>::sys_const(
                        &compression_mode,
                    ),
                    <bool as sys::GodotFfi>::sys_const(&normal_map),
                    <f64 as sys::GodotFfi>::sys_const(&lossy_quality),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_format(&self) -> image::Format {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("get_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3847873762i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "get_format" , 3847873762i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <image::Format as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_compression_mode(&self) -> portable_compressed_texture_2d::CompressionMode {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("get_compression_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3265612739i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "get_compression_mode" , 3265612739i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < portable_compressed_texture_2d :: CompressionMode as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_size_override(&mut self, size: Vector2) {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("set_size_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "set_size_override" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_size_override(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("get_size_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "get_size_override" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_keep_compressed_buffer(&mut self, keep: bool) {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("set_keep_compressed_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "set_keep_compressed_buffer" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&keep)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_keeping_compressed_buffer(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("is_keeping_compressed_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "is_keeping_compressed_buffer" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_keep_all_compressed_buffers(keep: bool) {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("set_keep_all_compressed_buffers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "set_keep_all_compressed_buffers" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&keep)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
            }
        }
        pub fn is_keeping_all_compressed_buffers() -> bool {
            unsafe {
                let __class_name = StringName::from("PortableCompressedTexture2D");
                let __method_name = StringName::from("is_keeping_all_compressed_buffers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PortableCompressedTexture2D" , "is_keeping_all_compressed_buffers" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for PortableCompressedTexture2D {
        type Base = crate::engine::Texture2D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "PortableCompressedTexture2D";
    }
    impl crate::obj::EngineClass for PortableCompressedTexture2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Texture2D> for PortableCompressedTexture2D {}
    impl crate::obj::Inherits<crate::engine::Texture> for PortableCompressedTexture2D {}
    impl crate::obj::Inherits<crate::engine::Resource> for PortableCompressedTexture2D {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for PortableCompressedTexture2D {}
    impl crate::obj::Inherits<crate::engine::Object> for PortableCompressedTexture2D {}
    impl std::ops::Deref for PortableCompressedTexture2D {
        type Target = crate::engine::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for PortableCompressedTexture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_PortableCompressedTexture2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::PortableCompressedTexture2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Texture2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Texture> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CompressionMode {
    ord: i32,
}
impl CompressionMode {
    pub const COMPRESSION_MODE_LOSSLESS: Self = Self { ord: 0 };
    pub const COMPRESSION_MODE_LOSSY: Self = Self { ord: 1 };
    pub const COMPRESSION_MODE_BASIS_UNIVERSAL: Self = Self { ord: 2 };
    pub const COMPRESSION_MODE_S3TC: Self = Self { ord: 3 };
    pub const COMPRESSION_MODE_ETC2: Self = Self { ord: 4 };
    pub const COMPRESSION_MODE_BPTC: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for CompressionMode {
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
unsafe impl sys::GodotFfi for CompressionMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
