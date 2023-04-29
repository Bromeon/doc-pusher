#![doc = "Sidecar module for class [`Shape2D`][crate::engine::Shape2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Shape2D` enums](https://docs.godotengine.org/en/stable/classes/class_shape2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Shape2D.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`Shape2DVirtual`][crate::engine::Shape2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `Shape2D`](https://docs.godotengine.org/en/stable/classes/class_shape2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Shape2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Shape2D`][crate::engine::Shape2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Shape2D` methods](https://docs.godotengine.org/en/stable/classes/class_shape2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Shape2DVirtual:
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
    impl Shape2D {
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
        pub fn set_custom_solver_bias(&mut self, bias: f64) {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("set_custom_solver_bias");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "set_custom_solver_bias" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&bias)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_custom_solver_bias(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("get_custom_solver_bias");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "get_custom_solver_bias" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn collide(
            &mut self,
            local_xform: Transform2D,
            with_shape: Gd<Shape2D>,
            shape_xform: Transform2D,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("collide");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3709843132i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "collide" , 3709843132i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Transform2D as sys::GodotFfi>::sys_const(&local_xform),
                    <Gd<Shape2D> as AsArg>::as_arg_ptr(&with_shape),
                    <Transform2D as sys::GodotFfi>::sys_const(&shape_xform),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn collide_with_motion(
            &mut self,
            local_xform: Transform2D,
            local_motion: Vector2,
            with_shape: Gd<Shape2D>,
            shape_xform: Transform2D,
            shape_motion: Vector2,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("collide_with_motion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2869556801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "collide_with_motion" , 2869556801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Transform2D as sys::GodotFfi>::sys_const(&local_xform),
                    <Vector2 as sys::GodotFfi>::sys_const(&local_motion),
                    <Gd<Shape2D> as AsArg>::as_arg_ptr(&with_shape),
                    <Transform2D as sys::GodotFfi>::sys_const(&shape_xform),
                    <Vector2 as sys::GodotFfi>::sys_const(&shape_motion),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn collide_and_get_contacts(
            &mut self,
            local_xform: Transform2D,
            with_shape: Gd<Shape2D>,
            shape_xform: Transform2D,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("collide_and_get_contacts");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3056932662i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "collide_and_get_contacts" , 3056932662i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Transform2D as sys::GodotFfi>::sys_const(&local_xform),
                    <Gd<Shape2D> as AsArg>::as_arg_ptr(&with_shape),
                    <Transform2D as sys::GodotFfi>::sys_const(&shape_xform),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn collide_with_motion_and_get_contacts(
            &mut self,
            local_xform: Transform2D,
            local_motion: Vector2,
            with_shape: Gd<Shape2D>,
            shape_xform: Transform2D,
            shape_motion: Vector2,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("collide_with_motion_and_get_contacts");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3620351573i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "collide_with_motion_and_get_contacts" , 3620351573i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Transform2D as sys::GodotFfi>::sys_const(&local_xform),
                    <Vector2 as sys::GodotFfi>::sys_const(&local_motion),
                    <Gd<Shape2D> as AsArg>::as_arg_ptr(&with_shape),
                    <Transform2D as sys::GodotFfi>::sys_const(&shape_xform),
                    <Vector2 as sys::GodotFfi>::sys_const(&shape_motion),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw(&mut self, canvas_item: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "draw" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rect(&self) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("Shape2D");
                let __method_name = StringName::from("get_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1639390495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Shape2D" , "get_rect" , 1639390495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Shape2D {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Shape2D";
    }
    impl crate::obj::EngineClass for Shape2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Shape2D {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Shape2D {}
    impl crate::obj::Inherits<crate::engine::Object> for Shape2D {}
    impl std::ops::Deref for Shape2D {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Shape2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Shape2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Shape2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
