#![doc = "Sidecar module for class [`Geometry3D`][crate::engine::Geometry3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry3D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Geometry3D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`Geometry3DVirtual`][crate::engine::Geometry3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `Geometry3D`](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Geometry3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Geometry3D`][crate::engine::Geometry3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Geometry3D` methods](https://docs.godotengine.org/en/stable/classes/class_geometry3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Geometry3DVirtual:
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
    impl Geometry3D {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
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
        pub fn build_box_planes(&mut self, extents: Vector3) -> Array<Plane> {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("build_box_planes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3622277145i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "build_box_planes" , 3622277145i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&extents)];
                let __args_ptr = __args.as_ptr();
                <Array<Plane> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn build_cylinder_planes(
            &mut self,
            radius: f64,
            height: f64,
            sides: i64,
            axis: Vector3Axis,
        ) -> Array<Plane> {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("build_cylinder_planes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3142160516i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "build_cylinder_planes" , 3142160516i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                    <f64 as sys::GodotFfi>::sys_const(&height),
                    <i64 as sys::GodotFfi>::sys_const(&sides),
                    <Vector3Axis as sys::GodotFfi>::sys_const(&axis),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<Plane> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn build_capsule_planes(
            &mut self,
            radius: f64,
            height: f64,
            sides: i64,
            lats: i64,
            axis: Vector3Axis,
        ) -> Array<Plane> {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("build_capsule_planes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    410870045i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "build_capsule_planes" , 410870045i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                    <f64 as sys::GodotFfi>::sys_const(&height),
                    <i64 as sys::GodotFfi>::sys_const(&sides),
                    <i64 as sys::GodotFfi>::sys_const(&lats),
                    <Vector3Axis as sys::GodotFfi>::sys_const(&axis),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<Plane> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_points_between_segments(
            &mut self,
            p1: Vector3,
            p2: Vector3,
            q1: Vector3,
            q2: Vector3,
        ) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("get_closest_points_between_segments");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1056373962i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "get_closest_points_between_segments" , 1056373962i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&p1),
                    <Vector3 as sys::GodotFfi>::sys_const(&p2),
                    <Vector3 as sys::GodotFfi>::sys_const(&q1),
                    <Vector3 as sys::GodotFfi>::sys_const(&q2),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_point_to_segment(
            &mut self,
            point: Vector3,
            s1: Vector3,
            s2: Vector3,
        ) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("get_closest_point_to_segment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2168193209i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "get_closest_point_to_segment" , 2168193209i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&point),
                    <Vector3 as sys::GodotFfi>::sys_const(&s1),
                    <Vector3 as sys::GodotFfi>::sys_const(&s2),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_point_to_segment_uncapped(
            &mut self,
            point: Vector3,
            s1: Vector3,
            s2: Vector3,
        ) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("get_closest_point_to_segment_uncapped");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2168193209i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "get_closest_point_to_segment_uncapped" , 2168193209i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&point),
                    <Vector3 as sys::GodotFfi>::sys_const(&s1),
                    <Vector3 as sys::GodotFfi>::sys_const(&s2),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn ray_intersects_triangle(
            &mut self,
            from: Vector3,
            dir: Vector3,
            a: Vector3,
            b: Vector3,
            c: Vector3,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("ray_intersects_triangle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1718655448i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "ray_intersects_triangle" , 1718655448i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&dir),
                    <Vector3 as sys::GodotFfi>::sys_const(&a),
                    <Vector3 as sys::GodotFfi>::sys_const(&b),
                    <Vector3 as sys::GodotFfi>::sys_const(&c),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn segment_intersects_triangle(
            &mut self,
            from: Vector3,
            to: Vector3,
            a: Vector3,
            b: Vector3,
            c: Vector3,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("segment_intersects_triangle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1718655448i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "segment_intersects_triangle" , 1718655448i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&to),
                    <Vector3 as sys::GodotFfi>::sys_const(&a),
                    <Vector3 as sys::GodotFfi>::sys_const(&b),
                    <Vector3 as sys::GodotFfi>::sys_const(&c),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn segment_intersects_sphere(
            &mut self,
            from: Vector3,
            to: Vector3,
            sphere_position: Vector3,
            sphere_radius: f64,
        ) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("segment_intersects_sphere");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4080141172i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "segment_intersects_sphere" , 4080141172i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&to),
                    <Vector3 as sys::GodotFfi>::sys_const(&sphere_position),
                    <f64 as sys::GodotFfi>::sys_const(&sphere_radius),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn segment_intersects_cylinder(
            &mut self,
            from: Vector3,
            to: Vector3,
            height: f64,
            radius: f64,
        ) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("segment_intersects_cylinder");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2361316491i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "segment_intersects_cylinder" , 2361316491i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&to),
                    <f64 as sys::GodotFfi>::sys_const(&height),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn segment_intersects_convex(
            &mut self,
            from: Vector3,
            to: Vector3,
            planes: VariantArray,
        ) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("segment_intersects_convex");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    537425332i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "segment_intersects_convex" , 537425332i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&to),
                    <VariantArray as sys::GodotFfi>::sys_const(&planes),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clip_polygon(
            &mut self,
            points: PackedVector3Array,
            plane: Plane,
        ) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("Geometry3D");
                let __method_name = StringName::from("clip_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2603188319i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry3D" , "clip_polygon" , 2603188319i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&points),
                    <Plane as sys::GodotFfi>::sys_const(&plane),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Geometry3D {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Geometry3D";
    }
    impl crate::obj::EngineClass for Geometry3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for Geometry3D {}
    impl std::ops::Deref for Geometry3D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Geometry3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Geometry3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Geometry3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
