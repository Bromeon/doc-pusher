#![doc = "Sidecar module for class [`Curve2D`][crate::engine::Curve2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Curve2D` enums](https://docs.godotengine.org/en/stable/classes/class_curve2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Curve2D.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`Curve2DVirtual`][crate::engine::Curve2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `Curve2D`](https://docs.godotengine.org/en/stable/classes/class_curve2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Curve2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Curve2D`][crate::engine::Curve2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Curve2D` methods](https://docs.godotengine.org/en/stable/classes/class_curve2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Curve2DVirtual:
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
    impl Curve2D {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Curve2D");
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
        pub fn get_point_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_point_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_point_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_point_count(&mut self, count: i64) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("set_point_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "set_point_count" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&count)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_point(&mut self, position: Vector2, in_: Vector2, out: Vector2, index: i64) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("add_point");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2437345566i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "add_point" , 2437345566i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <Vector2 as sys::GodotFfi>::sys_const(&in_),
                    <Vector2 as sys::GodotFfi>::sys_const(&out),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_point_position(&mut self, idx: i64, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("set_point_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    163021252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "set_point_position" , 163021252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_point_position(&self, idx: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_point_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2299179447i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_point_position" , 2299179447i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_point_in(&mut self, idx: i64, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("set_point_in");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    163021252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "set_point_in" , 163021252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_point_in(&self, idx: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_point_in");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2299179447i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_point_in" , 2299179447i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_point_out(&mut self, idx: i64, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("set_point_out");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    163021252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "set_point_out" , 163021252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_point_out(&self, idx: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_point_out");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2299179447i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_point_out" , 2299179447i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_point(&mut self, idx: i64) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("remove_point");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "remove_point" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_points(&mut self) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("clear_points");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "clear_points" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sample(&self, idx: i64, t: f64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("sample");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    26514310i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "sample" , 26514310i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&idx),
                    <f64 as sys::GodotFfi>::sys_const(&t),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn samplef(&self, fofs: f64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("samplef");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3588506812i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "samplef" , 3588506812i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&fofs)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bake_interval(&mut self, distance: f64) {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("set_bake_interval");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "set_bake_interval" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&distance)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bake_interval(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_bake_interval");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_bake_interval" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_baked_length(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_baked_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_baked_length" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn sample_baked(&self, offset: f64, cubic: bool) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("sample_baked");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3750540263i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "sample_baked" , 3750540263i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                    <bool as sys::GodotFfi>::sys_const(&cubic),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn sample_baked_with_rotation(&self, offset: f64, cubic: bool) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("sample_baked_with_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    255128112i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "sample_baked_with_rotation" , 255128112i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                    <bool as sys::GodotFfi>::sys_const(&cubic),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_baked_points(&self) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_baked_points");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2961356807i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_baked_points" , 2961356807i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_point(&self, to_point: Vector2) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_closest_point");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2656412154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_closest_point" , 2656412154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to_point)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_offset(&self, to_point: Vector2) -> f64 {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("get_closest_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2276447920i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "get_closest_offset" , 2276447920i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&to_point)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tessellate(&self, max_stages: i64, tolerance_degrees: f64) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("tessellate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    958145977i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "tessellate" , 958145977i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&max_stages),
                    <f64 as sys::GodotFfi>::sys_const(&tolerance_degrees),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tessellate_even_length(
            &self,
            max_stages: i64,
            tolerance_length: f64,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Curve2D");
                let __method_name = StringName::from("tessellate_even_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4229237857i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Curve2D" , "tessellate_even_length" , 4229237857i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&max_stages),
                    <f64 as sys::GodotFfi>::sys_const(&tolerance_length),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Curve2D {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Curve2D";
    }
    impl crate::obj::EngineClass for Curve2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Curve2D {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Curve2D {}
    impl crate::obj::Inherits<crate::engine::Object> for Curve2D {}
    impl std::ops::Deref for Curve2D {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Curve2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Curve2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Curve2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
