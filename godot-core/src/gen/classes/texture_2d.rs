#![doc = "Sidecar module for class [`Texture2D`][crate::engine::Texture2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Texture2D` enums](https://docs.godotengine.org/en/stable/classes/class_texture2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Texture2D.`\n\nInherits [`Texture`][crate::engine::Texture].\n\nRelated symbols:\n\n* [`Texture2DVirtual`][crate::engine::Texture2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `Texture2D`](https://docs.godotengine.org/en/stable/classes/class_texture2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Texture2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Texture2D`][crate::engine::Texture2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Texture2D` methods](https://docs.godotengine.org/en/stable/classes/class_texture2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Texture2DVirtual:
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
    impl Texture2D {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Texture2D");
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
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("get_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "get_width" , 3905245786i64);
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
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("get_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "get_height" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_size(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "get_size" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_alpha(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("has_alpha");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "has_alpha" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw(&self, canvas_item: Rid, position: Vector2, modulate: Color, transpose: bool) {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1115460088i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "draw" , 1115460088i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_rect(
            &self,
            canvas_item: Rid,
            rect: Rect2,
            tile: bool,
            modulate: Color,
            transpose: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("draw_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    575156982i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "draw_rect" , 575156982i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <bool as sys::GodotFfi>::sys_const(&tile),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_rect_region(
            &self,
            canvas_item: Rid,
            rect: Rect2,
            src_rect: Rect2,
            modulate: Color,
            transpose: bool,
            clip_uv: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("draw_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1066564656i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "draw_rect_region" , 1066564656i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                    <bool as sys::GodotFfi>::sys_const(&clip_uv),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_image(&self) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("get_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4190603485i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "get_image" , 4190603485i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_placeholder(&self) -> Option<Gd<Resource>> {
            unsafe {
                let __class_name = StringName::from("Texture2D");
                let __method_name = StringName::from("create_placeholder");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    121922552i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Texture2D" , "create_placeholder" , 121922552i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Resource>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Texture2D {
        type Base = crate::engine::Texture;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Texture2D";
    }
    impl crate::obj::EngineClass for Texture2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Texture> for Texture2D {}
    impl crate::obj::Inherits<crate::engine::Resource> for Texture2D {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Texture2D {}
    impl crate::obj::Inherits<crate::engine::Object> for Texture2D {}
    impl std::ops::Deref for Texture2D {
        type Target = crate::engine::Texture;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Texture2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Texture2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Texture2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Texture> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
