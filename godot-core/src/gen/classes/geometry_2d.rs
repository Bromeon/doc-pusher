#![doc = "Sidecar module for class [`Geometry2D`][crate::engine::Geometry2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Geometry2D` enums](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Geometry2D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`geometry_2d`][crate::engine::geometry_2d]: sidecar module with related enum/flag types\n* [`Geometry2DVirtual`][crate::engine::Geometry2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `Geometry2D`](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Geometry2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Geometry2D`][crate::engine::Geometry2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Geometry2D` methods](https://docs.godotengine.org/en/stable/classes/class_geometry2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Geometry2DVirtual:
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
    impl Geometry2D {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
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
        pub fn is_point_in_circle(
            &mut self,
            point: Vector2,
            circle_position: Vector2,
            circle_radius: f64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("is_point_in_circle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2929491703i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "is_point_in_circle" , 2929491703i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&point),
                    <Vector2 as sys::GodotFfi>::sys_const(&circle_position),
                    <f64 as sys::GodotFfi>::sys_const(&circle_radius),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn segment_intersects_circle(
            &mut self,
            segment_from: Vector2,
            segment_to: Vector2,
            circle_position: Vector2,
            circle_radius: f64,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("segment_intersects_circle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1356928167i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "segment_intersects_circle" , 1356928167i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&segment_from),
                    <Vector2 as sys::GodotFfi>::sys_const(&segment_to),
                    <Vector2 as sys::GodotFfi>::sys_const(&circle_position),
                    <f64 as sys::GodotFfi>::sys_const(&circle_radius),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn segment_intersects_segment(
            &mut self,
            from_a: Vector2,
            to_a: Vector2,
            from_b: Vector2,
            to_b: Vector2,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("segment_intersects_segment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2058025344i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "segment_intersects_segment" , 2058025344i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&from_a),
                    <Vector2 as sys::GodotFfi>::sys_const(&to_a),
                    <Vector2 as sys::GodotFfi>::sys_const(&from_b),
                    <Vector2 as sys::GodotFfi>::sys_const(&to_b),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn line_intersects_line(
            &mut self,
            from_a: Vector2,
            dir_a: Vector2,
            from_b: Vector2,
            dir_b: Vector2,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("line_intersects_line");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2058025344i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "line_intersects_line" , 2058025344i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&from_a),
                    <Vector2 as sys::GodotFfi>::sys_const(&dir_a),
                    <Vector2 as sys::GodotFfi>::sys_const(&from_b),
                    <Vector2 as sys::GodotFfi>::sys_const(&dir_b),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_points_between_segments(
            &mut self,
            p1: Vector2,
            q1: Vector2,
            p2: Vector2,
            q2: Vector2,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("get_closest_points_between_segments");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3344690961i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "get_closest_points_between_segments" , 3344690961i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&p1),
                    <Vector2 as sys::GodotFfi>::sys_const(&q1),
                    <Vector2 as sys::GodotFfi>::sys_const(&p2),
                    <Vector2 as sys::GodotFfi>::sys_const(&q2),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_point_to_segment(
            &mut self,
            point: Vector2,
            s1: Vector2,
            s2: Vector2,
        ) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("get_closest_point_to_segment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4172901909i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "get_closest_point_to_segment" , 4172901909i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&point),
                    <Vector2 as sys::GodotFfi>::sys_const(&s1),
                    <Vector2 as sys::GodotFfi>::sys_const(&s2),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_closest_point_to_segment_uncapped(
            &mut self,
            point: Vector2,
            s1: Vector2,
            s2: Vector2,
        ) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("get_closest_point_to_segment_uncapped");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4172901909i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "get_closest_point_to_segment_uncapped" , 4172901909i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&point),
                    <Vector2 as sys::GodotFfi>::sys_const(&s1),
                    <Vector2 as sys::GodotFfi>::sys_const(&s2),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn point_is_inside_triangle(
            &self,
            point: Vector2,
            a: Vector2,
            b: Vector2,
            c: Vector2,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("point_is_inside_triangle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1025948137i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "point_is_inside_triangle" , 1025948137i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&point),
                    <Vector2 as sys::GodotFfi>::sys_const(&a),
                    <Vector2 as sys::GodotFfi>::sys_const(&b),
                    <Vector2 as sys::GodotFfi>::sys_const(&c),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_polygon_clockwise(&mut self, polygon: PackedVector2Array) -> bool {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("is_polygon_clockwise");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1361156557i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "is_polygon_clockwise" , 1361156557i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector2Array as sys::GodotFfi>::sys_const(&polygon)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_point_in_polygon(&mut self, point: Vector2, polygon: PackedVector2Array) -> bool {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("is_point_in_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    738277916i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "is_point_in_polygon" , 738277916i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&point),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn triangulate_polygon(&mut self, polygon: PackedVector2Array) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("triangulate_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1389921771i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "triangulate_polygon" , 1389921771i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector2Array as sys::GodotFfi>::sys_const(&polygon)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn triangulate_delaunay(&mut self, points: PackedVector2Array) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("triangulate_delaunay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1389921771i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "triangulate_delaunay" , 1389921771i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector2Array as sys::GodotFfi>::sys_const(&points)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn convex_hull(&mut self, points: PackedVector2Array) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("convex_hull");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2004331998i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "convex_hull" , 2004331998i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector2Array as sys::GodotFfi>::sys_const(&points)];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn decompose_polygon_in_convex(
            &mut self,
            polygon: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("decompose_polygon_in_convex");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3982393695i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "decompose_polygon_in_convex" , 3982393695i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector2Array as sys::GodotFfi>::sys_const(&polygon)];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn merge_polygons(
            &mut self,
            polygon_a: PackedVector2Array,
            polygon_b: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("merge_polygons");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3637387053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "merge_polygons" , 3637387053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_a),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_b),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clip_polygons(
            &mut self,
            polygon_a: PackedVector2Array,
            polygon_b: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("clip_polygons");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3637387053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "clip_polygons" , 3637387053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_a),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_b),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn intersect_polygons(
            &mut self,
            polygon_a: PackedVector2Array,
            polygon_b: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("intersect_polygons");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3637387053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "intersect_polygons" , 3637387053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_a),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_b),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn exclude_polygons(
            &mut self,
            polygon_a: PackedVector2Array,
            polygon_b: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("exclude_polygons");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3637387053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "exclude_polygons" , 3637387053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_a),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon_b),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clip_polyline_with_polygon(
            &mut self,
            polyline: PackedVector2Array,
            polygon: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("clip_polyline_with_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3637387053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "clip_polyline_with_polygon" , 3637387053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polyline),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn intersect_polyline_with_polygon(
            &mut self,
            polyline: PackedVector2Array,
            polygon: PackedVector2Array,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("intersect_polyline_with_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3637387053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "intersect_polyline_with_polygon" , 3637387053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polyline),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn offset_polygon(
            &mut self,
            polygon: PackedVector2Array,
            delta: f64,
            join_type: geometry_2d::PolyJoinType,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("offset_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3837618924i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "offset_polygon" , 3837618924i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polygon),
                    <f64 as sys::GodotFfi>::sys_const(&delta),
                    <geometry_2d::PolyJoinType as sys::GodotFfi>::sys_const(&join_type),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn offset_polyline(
            &mut self,
            polyline: PackedVector2Array,
            delta: f64,
            join_type: geometry_2d::PolyJoinType,
            end_type: geometry_2d::PolyEndType,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("offset_polyline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    328033063i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "offset_polyline" , 328033063i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&polyline),
                    <f64 as sys::GodotFfi>::sys_const(&delta),
                    <geometry_2d::PolyJoinType as sys::GodotFfi>::sys_const(&join_type),
                    <geometry_2d::PolyEndType as sys::GodotFfi>::sys_const(&end_type),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn make_atlas(&mut self, sizes: PackedVector2Array) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("Geometry2D");
                let __method_name = StringName::from("make_atlas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1337682371i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Geometry2D" , "make_atlas" , 1337682371i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector2Array as sys::GodotFfi>::sys_const(&sizes)];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Geometry2D {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Geometry2D";
    }
    impl crate::obj::EngineClass for Geometry2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for Geometry2D {}
    impl std::ops::Deref for Geometry2D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Geometry2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Geometry2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Geometry2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PolyBooleanOperation {
    ord: i32,
}
impl PolyBooleanOperation {
    pub const OPERATION_UNION: Self = Self { ord: 0 };
    pub const OPERATION_DIFFERENCE: Self = Self { ord: 1 };
    pub const OPERATION_INTERSECTION: Self = Self { ord: 2 };
    pub const OPERATION_XOR: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for PolyBooleanOperation {
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
unsafe impl sys::GodotFfi for PolyBooleanOperation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PolyJoinType {
    ord: i32,
}
impl PolyJoinType {
    pub const JOIN_SQUARE: Self = Self { ord: 0 };
    pub const JOIN_ROUND: Self = Self { ord: 1 };
    pub const JOIN_MITER: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for PolyJoinType {
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
unsafe impl sys::GodotFfi for PolyJoinType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PolyEndType {
    ord: i32,
}
impl PolyEndType {
    pub const END_POLYGON: Self = Self { ord: 0 };
    pub const END_JOINED: Self = Self { ord: 1 };
    pub const END_BUTT: Self = Self { ord: 2 };
    pub const END_SQUARE: Self = Self { ord: 3 };
    pub const END_ROUND: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for PolyEndType {
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
unsafe impl sys::GodotFfi for PolyEndType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
