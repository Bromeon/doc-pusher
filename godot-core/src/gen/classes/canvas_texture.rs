#![doc = "Sidecar module for class [`CanvasTexture`][crate::engine::CanvasTexture].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasTexture` enums](https://docs.godotengine.org/en/stable/classes/class_canvastexture.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `CanvasTexture.`\n\nInherits [`Texture2D`][crate::engine::Texture2D].\n\nRelated symbols:\n\n* [`CanvasTextureVirtual`][crate::engine::CanvasTextureVirtual]: virtual methods\n\n\nSee also [Godot docs for `CanvasTexture`](https://docs.godotengine.org/en/stable/classes/class_canvastexture.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct CanvasTexture {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`CanvasTexture`][crate::engine::CanvasTexture].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CanvasTexture` methods](https://docs.godotengine.org/en/stable/classes/class_canvastexture.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait CanvasTextureVirtual:
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
    impl CanvasTexture {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
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
        pub fn set_diffuse_texture(&mut self, texture: Gd<Texture2D>) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_diffuse_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4051416890i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_diffuse_texture" , 4051416890i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Texture2D> as AsArg>::as_arg_ptr(&texture)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_diffuse_texture(&self) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_diffuse_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635182373i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_diffuse_texture" , 3635182373i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_normal_texture(&mut self, texture: Gd<Texture2D>) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_normal_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4051416890i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_normal_texture" , 4051416890i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Texture2D> as AsArg>::as_arg_ptr(&texture)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_normal_texture(&self) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_normal_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635182373i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_normal_texture" , 3635182373i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_specular_texture(&mut self, texture: Gd<Texture2D>) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_specular_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4051416890i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_specular_texture" , 4051416890i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Texture2D> as AsArg>::as_arg_ptr(&texture)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_specular_texture(&self) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_specular_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635182373i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_specular_texture" , 3635182373i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_specular_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_specular_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_specular_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_specular_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_specular_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_specular_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_specular_shininess(&mut self, shininess: f64) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_specular_shininess");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_specular_shininess" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&shininess)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_specular_shininess(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_specular_shininess");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_specular_shininess" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_filter(&mut self, filter: canvas_item::TextureFilter) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1037999706i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_texture_filter" , 1037999706i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<canvas_item::TextureFilter as sys::GodotFfi>::sys_const(
                    &filter,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_filter(&self) -> canvas_item::TextureFilter {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    121960042i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_texture_filter" , 121960042i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item::TextureFilter as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_repeat(&mut self, repeat: canvas_item::TextureRepeat) {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("set_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1716472974i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "set_texture_repeat" , 1716472974i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<canvas_item::TextureRepeat as sys::GodotFfi>::sys_const(
                    &repeat,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_repeat(&self) -> canvas_item::TextureRepeat {
            unsafe {
                let __class_name = StringName::from("CanvasTexture");
                let __method_name = StringName::from("get_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2667158319i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasTexture" , "get_texture_repeat" , 2667158319i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item::TextureRepeat as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for CanvasTexture {
        type Base = crate::engine::Texture2D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "CanvasTexture";
    }
    impl crate::obj::EngineClass for CanvasTexture {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Texture2D> for CanvasTexture {}
    impl crate::obj::Inherits<crate::engine::Texture> for CanvasTexture {}
    impl crate::obj::Inherits<crate::engine::Resource> for CanvasTexture {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for CanvasTexture {}
    impl crate::obj::Inherits<crate::engine::Object> for CanvasTexture {}
    impl std::ops::Deref for CanvasTexture {
        type Target = crate::engine::Texture2D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for CanvasTexture {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_CanvasTexture {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::CanvasTexture> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Texture2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Texture> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
