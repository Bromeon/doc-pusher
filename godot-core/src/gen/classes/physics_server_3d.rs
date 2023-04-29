#![doc = "Sidecar module for class [`PhysicsServer3D`][crate::engine::PhysicsServer3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `PhysicsServer3D.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`physics_server_3d`][crate::engine::physics_server_3d]: sidecar module with related enum/flag types\n* [`PhysicsServer3DVirtual`][crate::engine::PhysicsServer3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer3D`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PhysicsServer3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`PhysicsServer3D`][crate::engine::PhysicsServer3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PhysicsServer3DVirtual:
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
    impl PhysicsServer3D {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
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
        pub fn world_boundary_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("world_boundary_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "world_boundary_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn separation_ray_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("separation_ray_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "separation_ray_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn sphere_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("sphere_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "sphere_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn box_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("box_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "box_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn capsule_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("capsule_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "capsule_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn cylinder_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("cylinder_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "cylinder_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn convex_polygon_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("convex_polygon_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "convex_polygon_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn concave_polygon_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("concave_polygon_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "concave_polygon_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn heightmap_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("heightmap_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "heightmap_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn custom_shape_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("custom_shape_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "custom_shape_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shape_set_data(&mut self, shape: Rid, data: Variant) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("shape_set_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3175752987i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "shape_set_data" , 3175752987i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shape),
                    <Variant as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shape_get_type(&self, shape: Rid) -> physics_server_3d::ShapeType {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("shape_get_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3418923367i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "shape_get_type" , 3418923367i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shape)];
                let __args_ptr = __args.as_ptr();
                <physics_server_3d::ShapeType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn shape_get_data(&self, shape: Rid) -> Variant {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("shape_get_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4171304767i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "shape_get_data" , 4171304767i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shape)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn space_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("space_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "space_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn space_set_active(&mut self, space: Rid, active: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("space_set_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "space_set_active" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&space),
                    <bool as sys::GodotFfi>::sys_const(&active),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn space_is_active(&self, space: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("space_is_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "space_is_active" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&space)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn space_set_param(
            &mut self,
            space: Rid,
            param: physics_server_3d::SpaceParameter,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("space_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2406017470i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "space_set_param" , 2406017470i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&space),
                    <physics_server_3d::SpaceParameter as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn space_get_param(&self, space: Rid, param: physics_server_3d::SpaceParameter) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("space_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1523206731i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "space_get_param" , 1523206731i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&space),
                    <physics_server_3d::SpaceParameter as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn space_get_direct_state(
            &mut self,
            space: Rid,
        ) -> Option<Gd<PhysicsDirectSpaceState3D>> {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("space_get_direct_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2048616813i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "space_get_direct_state" , 2048616813i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&space)];
                let __args_ptr = __args.as_ptr();
                <Gd<PhysicsDirectSpaceState3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_set_space(&mut self, area: Rid, space: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_space" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <Rid as sys::GodotFfi>::sys_const(&space),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_get_space(&self, area: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814569979i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_space" , 3814569979i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_add_shape(
            &mut self,
            area: Rid,
            shape: Rid,
            transform: Transform3D,
            disabled: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_add_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4040559639i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_add_shape" , 4040559639i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <Rid as sys::GodotFfi>::sys_const(&shape),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                    <bool as sys::GodotFfi>::sys_const(&disabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_shape(&mut self, area: Rid, shape_idx: i64, shape: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2310537182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_shape" , 2310537182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                    <Rid as sys::GodotFfi>::sys_const(&shape),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_shape_transform(
            &mut self,
            area: Rid,
            shape_idx: i64,
            transform: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_shape_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    675327471i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_shape_transform" , 675327471i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i64, disabled: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_shape_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2658558584i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_shape_disabled" , 2658558584i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                    <bool as sys::GodotFfi>::sys_const(&disabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_get_shape_count(&self, area: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_shape_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_shape_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_get_shape(&self, area: Rid, shape_idx: i64) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1066463050i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_shape" , 1066463050i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_get_shape_transform(&self, area: Rid, shape_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_shape_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1050775521i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_shape_transform" , 1050775521i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_remove_shape(&mut self, area: Rid, shape_idx: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_remove_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_remove_shape" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_clear_shapes(&mut self, area: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_clear_shapes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_clear_shapes" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_collision_layer(&mut self, area: Rid, layer: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_collision_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_collision_layer" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_get_collision_layer(&self, area: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_collision_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_collision_layer" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_set_collision_mask(&mut self, area: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_collision_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_collision_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_get_collision_mask(&self, area: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_collision_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_collision_mask" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_set_param(
            &mut self,
            area: Rid,
            param: physics_server_3d::AreaParameter,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2980114638i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_param" , 2980114638i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <physics_server_3d::AreaParameter as sys::GodotFfi>::sys_const(&param),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_transform(&mut self, area: Rid, transform: Transform3D) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3935195649i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_transform" , 3935195649i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_get_param(
            &self,
            area: Rid,
            param: physics_server_3d::AreaParameter,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    890056067i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_param" , 890056067i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <physics_server_3d::AreaParameter as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_get_transform(&self, area: Rid) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1128465797i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_transform" , 1128465797i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_attach_object_instance_id(&mut self, area: Rid, id: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_attach_object_instance_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_attach_object_instance_id" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_get_object_instance_id(&self, area: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_get_object_instance_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_get_object_instance_id" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&area)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_monitor_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3379118538i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_monitor_callback" , 3379118538i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_area_monitor_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3379118538i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_area_monitor_callback" , 3379118538i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <Callable as sys::GodotFfi>::sys_const(&callback),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_monitorable(&mut self, area: Rid, monitorable: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_monitorable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_monitorable" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <bool as sys::GodotFfi>::sys_const(&monitorable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn area_set_ray_pickable(&mut self, area: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("area_set_ray_pickable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "area_set_ray_pickable" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&area),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_space(&mut self, body: Rid, space: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_space" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Rid as sys::GodotFfi>::sys_const(&space),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_space(&self, body: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_space");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814569979i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_space" , 3814569979i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_mode(&mut self, body: Rid, mode: physics_server_3d::BodyMode) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    606803466i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_mode" , 606803466i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_mode(&self, body: Rid) -> physics_server_3d::BodyMode {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2488819728i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_mode" , 2488819728i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <physics_server_3d::BodyMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn body_set_collision_layer(&mut self, body: Rid, layer: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_collision_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_collision_layer" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_collision_layer(&self, body: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_collision_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_collision_layer" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_collision_mask(&mut self, body: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_collision_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_collision_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_collision_mask(&self, body: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_collision_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_collision_mask" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_collision_priority(&mut self, body: Rid, priority: f64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_collision_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_collision_priority" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <f64 as sys::GodotFfi>::sys_const(&priority),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_collision_priority(&self, body: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_collision_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_collision_priority" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_add_shape(
            &mut self,
            body: Rid,
            shape: Rid,
            transform: Transform3D,
            disabled: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_add_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4040559639i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_add_shape" , 4040559639i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Rid as sys::GodotFfi>::sys_const(&shape),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                    <bool as sys::GodotFfi>::sys_const(&disabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_shape(&mut self, body: Rid, shape_idx: i64, shape: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2310537182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_shape" , 2310537182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                    <Rid as sys::GodotFfi>::sys_const(&shape),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_shape_transform(
            &mut self,
            body: Rid,
            shape_idx: i64,
            transform: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_shape_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    675327471i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_shape_transform" , 675327471i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i64, disabled: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_shape_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2658558584i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_shape_disabled" , 2658558584i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                    <bool as sys::GodotFfi>::sys_const(&disabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_shape_count(&self, body: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_shape_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_shape_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_get_shape(&self, body: Rid, shape_idx: i64) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1066463050i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_shape" , 1066463050i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_get_shape_transform(&self, body: Rid, shape_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_shape_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1050775521i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_shape_transform" , 1050775521i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_remove_shape(&mut self, body: Rid, shape_idx: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_remove_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_remove_shape" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&shape_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_clear_shapes(&mut self, body: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_clear_shapes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_clear_shapes" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_attach_object_instance_id(&mut self, body: Rid, id: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_attach_object_instance_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_attach_object_instance_id" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_object_instance_id(&self, body: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_object_instance_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_object_instance_id" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_enable_continuous_collision_detection(&mut self, body: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name =
                    StringName::from("body_set_enable_continuous_collision_detection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_enable_continuous_collision_detection" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_is_continuous_collision_detection_enabled(&self, body: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name =
                    StringName::from("body_is_continuous_collision_detection_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_is_continuous_collision_detection_enabled" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_param(
            &mut self,
            body: Rid,
            param: physics_server_3d::BodyParameter,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    910941953i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_param" , 910941953i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyParameter as sys::GodotFfi>::sys_const(&param),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_param(
            &self,
            body: Rid,
            param: physics_server_3d::BodyParameter,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3385027841i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_param" , 3385027841i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyParameter as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_reset_mass_properties(&mut self, body: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_reset_mass_properties");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_reset_mass_properties" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_state(
            &mut self,
            body: Rid,
            state: physics_server_3d::BodyState,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    599977762i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_state" , 599977762i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyState as sys::GodotFfi>::sys_const(&state),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_state(&self, body: Rid, state: physics_server_3d::BodyState) -> Variant {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1850449534i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_state" , 1850449534i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyState as sys::GodotFfi>::sys_const(&state),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_apply_central_impulse");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_apply_central_impulse" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&impulse),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_apply_impulse(&mut self, body: Rid, impulse: Vector3, position: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_apply_impulse");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    110375048i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_apply_impulse" , 110375048i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&impulse),
                    <Vector3 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_apply_torque_impulse(&mut self, body: Rid, impulse: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_apply_torque_impulse");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_apply_torque_impulse" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&impulse),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_apply_central_force(&mut self, body: Rid, force: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_apply_central_force");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_apply_central_force" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&force),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_apply_force(&mut self, body: Rid, force: Vector3, position: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_apply_force");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    110375048i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_apply_force" , 110375048i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&force),
                    <Vector3 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_apply_torque(&mut self, body: Rid, torque: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_apply_torque");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_apply_torque" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&torque),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_add_constant_central_force(&mut self, body: Rid, force: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_add_constant_central_force");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_add_constant_central_force" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&force),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_add_constant_force(&mut self, body: Rid, force: Vector3, position: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_add_constant_force");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    110375048i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_add_constant_force" , 110375048i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&force),
                    <Vector3 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_add_constant_torque(&mut self, body: Rid, torque: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_add_constant_torque");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_add_constant_torque" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&torque),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_constant_force(&mut self, body: Rid, force: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_constant_force");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_constant_force" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&force),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_constant_force(&self, body: Rid) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_constant_force");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    531438156i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_constant_force" , 531438156i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_constant_torque(&mut self, body: Rid, torque: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_constant_torque");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_constant_torque" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&torque),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_constant_torque(&self, body: Rid) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_constant_torque");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    531438156i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_constant_torque" , 531438156i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_axis_velocity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_axis_velocity" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Vector3 as sys::GodotFfi>::sys_const(&axis_velocity),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_axis_lock(
            &mut self,
            body: Rid,
            axis: physics_server_3d::BodyAxis,
            lock: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_axis_lock");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2020836892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_axis_lock" , 2020836892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyAxis as sys::GodotFfi>::sys_const(&axis),
                    <bool as sys::GodotFfi>::sys_const(&lock),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_is_axis_locked(&self, body: Rid, axis: physics_server_3d::BodyAxis) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_is_axis_locked");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    587853580i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_is_axis_locked" , 587853580i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <physics_server_3d::BodyAxis as sys::GodotFfi>::sys_const(&axis),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_add_collision_exception");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_add_collision_exception" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Rid as sys::GodotFfi>::sys_const(&excepted_body),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_remove_collision_exception");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_remove_collision_exception" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Rid as sys::GodotFfi>::sys_const(&excepted_body),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_max_contacts_reported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_max_contacts_reported" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <i64 as sys::GodotFfi>::sys_const(&amount),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_get_max_contacts_reported(&self, body: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_max_contacts_reported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_max_contacts_reported" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_omit_force_integration");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_omit_force_integration" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_is_omitting_force_integration(&self, body: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_is_omitting_force_integration");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_is_omitting_force_integration" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_set_force_integration_callback(
            &mut self,
            body: Rid,
            callable: Callable,
            userdata: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_force_integration_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3059434249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_force_integration_callback" , 3059434249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                    <Variant as sys::GodotFfi>::sys_const(&userdata),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_set_ray_pickable(&mut self, body: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_set_ray_pickable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_set_ray_pickable" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn body_test_motion(
            &mut self,
            body: Rid,
            parameters: Gd<PhysicsTestMotionParameters3D>,
            result: Gd<PhysicsTestMotionResult3D>,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_test_motion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1944921792i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_test_motion" , 1944921792i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&body),
                    <Gd<PhysicsTestMotionParameters3D> as AsArg>::as_arg_ptr(&parameters),
                    <Gd<PhysicsTestMotionResult3D> as AsArg>::as_arg_ptr(&result),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_get_direct_state(&mut self, body: Rid) -> Option<Gd<PhysicsDirectBodyState3D>> {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("body_get_direct_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3029727957i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "body_get_direct_state" , 3029727957i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <Gd<PhysicsDirectBodyState3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn soft_body_get_bounds(&self, body: Rid) -> Aabb {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("soft_body_get_bounds");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    974181306i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "soft_body_get_bounds" , 974181306i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_clear(&mut self, joint: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_clear" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&joint)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn joint_make_pin(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_A: Vector3,
            body_B: Rid,
            local_B: Vector3,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_make_pin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4280171926i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_make_pin" , 4280171926i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Rid as sys::GodotFfi>::sys_const(&body_A),
                    <Vector3 as sys::GodotFfi>::sys_const(&local_A),
                    <Rid as sys::GodotFfi>::sys_const(&body_B),
                    <Vector3 as sys::GodotFfi>::sys_const(&local_B),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pin_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::PinJointParam,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("pin_joint_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    810685294i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "pin_joint_set_param" , 810685294i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::PinJointParam as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pin_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::PinJointParam,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("pin_joint_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2817972347i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "pin_joint_get_param" , 2817972347i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::PinJointParam as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn pin_joint_set_local_a(&mut self, joint: Rid, local_A: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("pin_joint_set_local_a");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "pin_joint_set_local_a" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Vector3 as sys::GodotFfi>::sys_const(&local_A),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pin_joint_get_local_a(&self, joint: Rid) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("pin_joint_get_local_a");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    531438156i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "pin_joint_get_local_a" , 531438156i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&joint)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn pin_joint_set_local_b(&mut self, joint: Rid, local_B: Vector3) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("pin_joint_set_local_b");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "pin_joint_set_local_b" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Vector3 as sys::GodotFfi>::sys_const(&local_B),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pin_joint_get_local_b(&self, joint: Rid) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("pin_joint_get_local_b");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    531438156i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "pin_joint_get_local_b" , 531438156i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&joint)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_make_hinge(
            &mut self,
            joint: Rid,
            body_A: Rid,
            hinge_A: Transform3D,
            body_B: Rid,
            hinge_B: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_make_hinge");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1684107643i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_make_hinge" , 1684107643i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Rid as sys::GodotFfi>::sys_const(&body_A),
                    <Transform3D as sys::GodotFfi>::sys_const(&hinge_A),
                    <Rid as sys::GodotFfi>::sys_const(&body_B),
                    <Transform3D as sys::GodotFfi>::sys_const(&hinge_B),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn hinge_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::HingeJointParam,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("hinge_joint_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3165502333i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "hinge_joint_set_param" , 3165502333i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::HingeJointParam as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn hinge_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::HingeJointParam,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("hinge_joint_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2129207581i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "hinge_joint_get_param" , 2129207581i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::HingeJointParam as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn hinge_joint_set_flag(
            &mut self,
            joint: Rid,
            flag: physics_server_3d::HingeJointFlag,
            enabled: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("hinge_joint_set_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1601626188i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "hinge_joint_set_flag" , 1601626188i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::HingeJointFlag as sys::GodotFfi>::sys_const(&flag),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn hinge_joint_get_flag(
            &self,
            joint: Rid,
            flag: physics_server_3d::HingeJointFlag,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("hinge_joint_get_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4165147865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "hinge_joint_get_flag" , 4165147865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::HingeJointFlag as sys::GodotFfi>::sys_const(&flag),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_make_slider(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_ref_A: Transform3D,
            body_B: Rid,
            local_ref_B: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_make_slider");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1684107643i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_make_slider" , 1684107643i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Rid as sys::GodotFfi>::sys_const(&body_A),
                    <Transform3D as sys::GodotFfi>::sys_const(&local_ref_A),
                    <Rid as sys::GodotFfi>::sys_const(&body_B),
                    <Transform3D as sys::GodotFfi>::sys_const(&local_ref_B),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn slider_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::SliderJointParam,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("slider_joint_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2264833593i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "slider_joint_set_param" , 2264833593i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::SliderJointParam as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn slider_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::SliderJointParam,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("slider_joint_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3498644957i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "slider_joint_get_param" , 3498644957i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::SliderJointParam as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_make_cone_twist(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_ref_A: Transform3D,
            body_B: Rid,
            local_ref_B: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_make_cone_twist");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1684107643i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_make_cone_twist" , 1684107643i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Rid as sys::GodotFfi>::sys_const(&body_A),
                    <Transform3D as sys::GodotFfi>::sys_const(&local_ref_A),
                    <Rid as sys::GodotFfi>::sys_const(&body_B),
                    <Transform3D as sys::GodotFfi>::sys_const(&local_ref_B),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn cone_twist_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::ConeTwistJointParam,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("cone_twist_joint_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    808587618i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "cone_twist_joint_set_param" , 808587618i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::ConeTwistJointParam as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn cone_twist_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::ConeTwistJointParam,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("cone_twist_joint_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1134789658i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "cone_twist_joint_get_param" , 1134789658i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <physics_server_3d::ConeTwistJointParam as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_get_type(&self, joint: Rid) -> physics_server_3d::JointType {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_get_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4290791900i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_get_type" , 4290791900i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&joint)];
                let __args_ptr = __args.as_ptr();
                <physics_server_3d::JointType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn joint_set_solver_priority(&mut self, joint: Rid, priority: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_set_solver_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_set_solver_priority" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <i64 as sys::GodotFfi>::sys_const(&priority),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn joint_get_solver_priority(&self, joint: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_get_solver_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_get_solver_priority" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&joint)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_disable_collisions_between_bodies");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_disable_collisions_between_bodies" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_is_disabled_collisions_between_bodies");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_is_disabled_collisions_between_bodies" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&joint)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn joint_make_generic_6dof(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_ref_A: Transform3D,
            body_B: Rid,
            local_ref_B: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("joint_make_generic_6dof");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1684107643i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "joint_make_generic_6dof" , 1684107643i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Rid as sys::GodotFfi>::sys_const(&body_A),
                    <Transform3D as sys::GodotFfi>::sys_const(&local_ref_A),
                    <Rid as sys::GodotFfi>::sys_const(&body_B),
                    <Transform3D as sys::GodotFfi>::sys_const(&local_ref_B),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn generic_6dof_joint_set_param(
            &mut self,
            joint: Rid,
            axis: Vector3Axis,
            param: physics_server_3d::G6DOFJointAxisParam,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("generic_6dof_joint_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2600081391i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "generic_6dof_joint_set_param" , 2600081391i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Vector3Axis as sys::GodotFfi>::sys_const(&axis),
                    <physics_server_3d::G6DOFJointAxisParam as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn generic_6dof_joint_get_param(
            &self,
            joint: Rid,
            axis: Vector3Axis,
            param: physics_server_3d::G6DOFJointAxisParam,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("generic_6dof_joint_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    467122058i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "generic_6dof_joint_get_param" , 467122058i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Vector3Axis as sys::GodotFfi>::sys_const(&axis),
                    <physics_server_3d::G6DOFJointAxisParam as sys::GodotFfi>::sys_const(&param),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn generic_6dof_joint_set_flag(
            &mut self,
            joint: Rid,
            axis: Vector3Axis,
            flag: physics_server_3d::G6DOFJointAxisFlag,
            enable: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("generic_6dof_joint_set_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3570926903i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "generic_6dof_joint_set_flag" , 3570926903i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Vector3Axis as sys::GodotFfi>::sys_const(&axis),
                    <physics_server_3d::G6DOFJointAxisFlag as sys::GodotFfi>::sys_const(&flag),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn generic_6dof_joint_get_flag(
            &self,
            joint: Rid,
            axis: Vector3Axis,
            flag: physics_server_3d::G6DOFJointAxisFlag,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("generic_6dof_joint_get_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4158090196i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "generic_6dof_joint_get_flag" , 4158090196i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&joint),
                    <Vector3Axis as sys::GodotFfi>::sys_const(&axis),
                    <physics_server_3d::G6DOFJointAxisFlag as sys::GodotFfi>::sys_const(&flag),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn free_rid(&mut self, rid: Rid) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("free_rid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "free_rid" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&rid)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_active(&mut self, active: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("set_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "set_active" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&active)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_process_info(&mut self, process_info: physics_server_3d::ProcessInfo) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3D");
                let __method_name = StringName::from("get_process_info");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1332958745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3D" , "get_process_info" , 1332958745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<physics_server_3d::ProcessInfo as sys::GodotFfi>::sys_const(&process_info)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for PhysicsServer3D {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "PhysicsServer3D";
    }
    impl crate::obj::EngineClass for PhysicsServer3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for PhysicsServer3D {}
    impl std::ops::Deref for PhysicsServer3D {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for PhysicsServer3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_PhysicsServer3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::PhysicsServer3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct JointType {
    ord: i32,
}
impl JointType {
    pub const JOINT_TYPE_PIN: Self = Self { ord: 0 };
    pub const JOINT_TYPE_HINGE: Self = Self { ord: 1 };
    pub const JOINT_TYPE_SLIDER: Self = Self { ord: 2 };
    pub const JOINT_TYPE_CONE_TWIST: Self = Self { ord: 3 };
    pub const JOINT_TYPE_6DOF: Self = Self { ord: 4 };
    pub const JOINT_TYPE_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for JointType {
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
unsafe impl sys::GodotFfi for JointType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PinJointParam {
    ord: i32,
}
impl PinJointParam {
    pub const PIN_JOINT_BIAS: Self = Self { ord: 0 };
    pub const PIN_JOINT_DAMPING: Self = Self { ord: 1 };
    pub const PIN_JOINT_IMPULSE_CLAMP: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for PinJointParam {
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
unsafe impl sys::GodotFfi for PinJointParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct HingeJointParam {
    ord: i32,
}
impl HingeJointParam {
    pub const HINGE_JOINT_BIAS: Self = Self { ord: 0 };
    pub const HINGE_JOINT_LIMIT_UPPER: Self = Self { ord: 1 };
    pub const HINGE_JOINT_LIMIT_LOWER: Self = Self { ord: 2 };
    pub const HINGE_JOINT_LIMIT_BIAS: Self = Self { ord: 3 };
    pub const HINGE_JOINT_LIMIT_SOFTNESS: Self = Self { ord: 4 };
    pub const HINGE_JOINT_LIMIT_RELAXATION: Self = Self { ord: 5 };
    pub const HINGE_JOINT_MOTOR_TARGET_VELOCITY: Self = Self { ord: 6 };
    pub const HINGE_JOINT_MOTOR_MAX_IMPULSE: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for HingeJointParam {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for HingeJointParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct HingeJointFlag {
    ord: i32,
}
impl HingeJointFlag {
    pub const HINGE_JOINT_FLAG_USE_LIMIT: Self = Self { ord: 0 };
    pub const HINGE_JOINT_FLAG_ENABLE_MOTOR: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for HingeJointFlag {
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
unsafe impl sys::GodotFfi for HingeJointFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SliderJointParam {
    ord: i32,
}
impl SliderJointParam {
    pub const SLIDER_JOINT_LINEAR_LIMIT_UPPER: Self = Self { ord: 0 };
    pub const SLIDER_JOINT_LINEAR_LIMIT_LOWER: Self = Self { ord: 1 };
    pub const SLIDER_JOINT_LINEAR_LIMIT_SOFTNESS: Self = Self { ord: 2 };
    pub const SLIDER_JOINT_LINEAR_LIMIT_RESTITUTION: Self = Self { ord: 3 };
    pub const SLIDER_JOINT_LINEAR_LIMIT_DAMPING: Self = Self { ord: 4 };
    pub const SLIDER_JOINT_LINEAR_MOTION_SOFTNESS: Self = Self { ord: 5 };
    pub const SLIDER_JOINT_LINEAR_MOTION_RESTITUTION: Self = Self { ord: 6 };
    pub const SLIDER_JOINT_LINEAR_MOTION_DAMPING: Self = Self { ord: 7 };
    pub const SLIDER_JOINT_LINEAR_ORTHOGONAL_SOFTNESS: Self = Self { ord: 8 };
    pub const SLIDER_JOINT_LINEAR_ORTHOGONAL_RESTITUTION: Self = Self { ord: 9 };
    pub const SLIDER_JOINT_LINEAR_ORTHOGONAL_DAMPING: Self = Self { ord: 10 };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_UPPER: Self = Self { ord: 11 };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_LOWER: Self = Self { ord: 12 };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_SOFTNESS: Self = Self { ord: 13 };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_RESTITUTION: Self = Self { ord: 14 };
    pub const SLIDER_JOINT_ANGULAR_LIMIT_DAMPING: Self = Self { ord: 15 };
    pub const SLIDER_JOINT_ANGULAR_MOTION_SOFTNESS: Self = Self { ord: 16 };
    pub const SLIDER_JOINT_ANGULAR_MOTION_RESTITUTION: Self = Self { ord: 17 };
    pub const SLIDER_JOINT_ANGULAR_MOTION_DAMPING: Self = Self { ord: 18 };
    pub const SLIDER_JOINT_ANGULAR_ORTHOGONAL_SOFTNESS: Self = Self { ord: 19 };
    pub const SLIDER_JOINT_ANGULAR_ORTHOGONAL_RESTITUTION: Self = Self { ord: 20 };
    pub const SLIDER_JOINT_ANGULAR_ORTHOGONAL_DAMPING: Self = Self { ord: 21 };
    pub const SLIDER_JOINT_MAX: Self = Self { ord: 22 };
}
impl crate::obj::EngineEnum for SliderJointParam {
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
            | ord @ 22i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for SliderJointParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ConeTwistJointParam {
    ord: i32,
}
impl ConeTwistJointParam {
    pub const CONE_TWIST_JOINT_SWING_SPAN: Self = Self { ord: 0 };
    pub const CONE_TWIST_JOINT_TWIST_SPAN: Self = Self { ord: 1 };
    pub const CONE_TWIST_JOINT_BIAS: Self = Self { ord: 2 };
    pub const CONE_TWIST_JOINT_SOFTNESS: Self = Self { ord: 3 };
    pub const CONE_TWIST_JOINT_RELAXATION: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for ConeTwistJointParam {
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
unsafe impl sys::GodotFfi for ConeTwistJointParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct G6DOFJointAxisParam {
    ord: i32,
}
impl G6DOFJointAxisParam {
    pub const G6DOF_JOINT_LINEAR_LOWER_LIMIT: Self = Self { ord: 0 };
    pub const G6DOF_JOINT_LINEAR_UPPER_LIMIT: Self = Self { ord: 1 };
    pub const G6DOF_JOINT_LINEAR_LIMIT_SOFTNESS: Self = Self { ord: 2 };
    pub const G6DOF_JOINT_LINEAR_RESTITUTION: Self = Self { ord: 3 };
    pub const G6DOF_JOINT_LINEAR_DAMPING: Self = Self { ord: 4 };
    pub const G6DOF_JOINT_LINEAR_MOTOR_TARGET_VELOCITY: Self = Self { ord: 5 };
    pub const G6DOF_JOINT_LINEAR_MOTOR_FORCE_LIMIT: Self = Self { ord: 6 };
    pub const G6DOF_JOINT_ANGULAR_LOWER_LIMIT: Self = Self { ord: 10 };
    pub const G6DOF_JOINT_ANGULAR_UPPER_LIMIT: Self = Self { ord: 11 };
    pub const G6DOF_JOINT_ANGULAR_LIMIT_SOFTNESS: Self = Self { ord: 12 };
    pub const G6DOF_JOINT_ANGULAR_DAMPING: Self = Self { ord: 13 };
    pub const G6DOF_JOINT_ANGULAR_RESTITUTION: Self = Self { ord: 14 };
    pub const G6DOF_JOINT_ANGULAR_FORCE_LIMIT: Self = Self { ord: 15 };
    pub const G6DOF_JOINT_ANGULAR_ERP: Self = Self { ord: 16 };
    pub const G6DOF_JOINT_ANGULAR_MOTOR_TARGET_VELOCITY: Self = Self { ord: 17 };
    pub const G6DOF_JOINT_ANGULAR_MOTOR_FORCE_LIMIT: Self = Self { ord: 18 };
}
impl crate::obj::EngineEnum for G6DOFJointAxisParam {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 10i32
            | ord @ 11i32
            | ord @ 12i32
            | ord @ 13i32
            | ord @ 14i32
            | ord @ 15i32
            | ord @ 16i32
            | ord @ 17i32
            | ord @ 18i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for G6DOFJointAxisParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct G6DOFJointAxisFlag {
    ord: i32,
}
impl G6DOFJointAxisFlag {
    pub const G6DOF_JOINT_FLAG_ENABLE_LINEAR_LIMIT: Self = Self { ord: 0 };
    pub const G6DOF_JOINT_FLAG_ENABLE_ANGULAR_LIMIT: Self = Self { ord: 1 };
    pub const G6DOF_JOINT_FLAG_ENABLE_MOTOR: Self = Self { ord: 4 };
    pub const G6DOF_JOINT_FLAG_ENABLE_LINEAR_MOTOR: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for G6DOFJointAxisFlag {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 4i32 | ord @ 5i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for G6DOFJointAxisFlag {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShapeType {
    ord: i32,
}
impl ShapeType {
    pub const SHAPE_WORLD_BOUNDARY: Self = Self { ord: 0 };
    pub const SHAPE_SEPARATION_RAY: Self = Self { ord: 1 };
    pub const SHAPE_SPHERE: Self = Self { ord: 2 };
    pub const SHAPE_BOX: Self = Self { ord: 3 };
    pub const SHAPE_CAPSULE: Self = Self { ord: 4 };
    pub const SHAPE_CYLINDER: Self = Self { ord: 5 };
    pub const SHAPE_CONVEX_POLYGON: Self = Self { ord: 6 };
    pub const SHAPE_CONCAVE_POLYGON: Self = Self { ord: 7 };
    pub const SHAPE_HEIGHTMAP: Self = Self { ord: 8 };
    pub const SHAPE_SOFT_BODY: Self = Self { ord: 9 };
    pub const SHAPE_CUSTOM: Self = Self { ord: 10 };
}
impl crate::obj::EngineEnum for ShapeType {
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
            | ord @ 10i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ShapeType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AreaParameter {
    ord: i32,
}
impl AreaParameter {
    pub const AREA_PARAM_GRAVITY_OVERRIDE_MODE: Self = Self { ord: 0 };
    pub const AREA_PARAM_GRAVITY: Self = Self { ord: 1 };
    pub const AREA_PARAM_GRAVITY_VECTOR: Self = Self { ord: 2 };
    pub const AREA_PARAM_GRAVITY_IS_POINT: Self = Self { ord: 3 };
    pub const AREA_PARAM_GRAVITY_POINT_UNIT_DISTANCE: Self = Self { ord: 4 };
    pub const AREA_PARAM_LINEAR_DAMP_OVERRIDE_MODE: Self = Self { ord: 5 };
    pub const AREA_PARAM_LINEAR_DAMP: Self = Self { ord: 6 };
    pub const AREA_PARAM_ANGULAR_DAMP_OVERRIDE_MODE: Self = Self { ord: 7 };
    pub const AREA_PARAM_ANGULAR_DAMP: Self = Self { ord: 8 };
    pub const AREA_PARAM_PRIORITY: Self = Self { ord: 9 };
    pub const AREA_PARAM_WIND_FORCE_MAGNITUDE: Self = Self { ord: 10 };
    pub const AREA_PARAM_WIND_SOURCE: Self = Self { ord: 11 };
    pub const AREA_PARAM_WIND_DIRECTION: Self = Self { ord: 12 };
    pub const AREA_PARAM_WIND_ATTENUATION_FACTOR: Self = Self { ord: 13 };
}
impl crate::obj::EngineEnum for AreaParameter {
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
            | ord @ 13i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for AreaParameter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AreaSpaceOverrideMode {
    ord: i32,
}
impl AreaSpaceOverrideMode {
    pub const AREA_SPACE_OVERRIDE_DISABLED: Self = Self { ord: 0 };
    pub const AREA_SPACE_OVERRIDE_COMBINE: Self = Self { ord: 1 };
    pub const AREA_SPACE_OVERRIDE_COMBINE_REPLACE: Self = Self { ord: 2 };
    pub const AREA_SPACE_OVERRIDE_REPLACE: Self = Self { ord: 3 };
    pub const AREA_SPACE_OVERRIDE_REPLACE_COMBINE: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for AreaSpaceOverrideMode {
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
unsafe impl sys::GodotFfi for AreaSpaceOverrideMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BodyMode {
    ord: i32,
}
impl BodyMode {
    pub const BODY_MODE_STATIC: Self = Self { ord: 0 };
    pub const BODY_MODE_KINEMATIC: Self = Self { ord: 1 };
    pub const BODY_MODE_RIGID: Self = Self { ord: 2 };
    pub const BODY_MODE_RIGID_LINEAR: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for BodyMode {
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
unsafe impl sys::GodotFfi for BodyMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BodyParameter {
    ord: i32,
}
impl BodyParameter {
    pub const BODY_PARAM_BOUNCE: Self = Self { ord: 0 };
    pub const BODY_PARAM_FRICTION: Self = Self { ord: 1 };
    pub const BODY_PARAM_MASS: Self = Self { ord: 2 };
    pub const BODY_PARAM_INERTIA: Self = Self { ord: 3 };
    pub const BODY_PARAM_CENTER_OF_MASS: Self = Self { ord: 4 };
    pub const BODY_PARAM_GRAVITY_SCALE: Self = Self { ord: 5 };
    pub const BODY_PARAM_LINEAR_DAMP_MODE: Self = Self { ord: 6 };
    pub const BODY_PARAM_ANGULAR_DAMP_MODE: Self = Self { ord: 7 };
    pub const BODY_PARAM_LINEAR_DAMP: Self = Self { ord: 8 };
    pub const BODY_PARAM_ANGULAR_DAMP: Self = Self { ord: 9 };
    pub const BODY_PARAM_MAX: Self = Self { ord: 10 };
}
impl crate::obj::EngineEnum for BodyParameter {
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
            | ord @ 10i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for BodyParameter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BodyDampMode {
    ord: i32,
}
impl BodyDampMode {
    pub const BODY_DAMP_MODE_COMBINE: Self = Self { ord: 0 };
    pub const BODY_DAMP_MODE_REPLACE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for BodyDampMode {
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
unsafe impl sys::GodotFfi for BodyDampMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BodyState {
    ord: i32,
}
impl BodyState {
    pub const BODY_STATE_TRANSFORM: Self = Self { ord: 0 };
    pub const BODY_STATE_LINEAR_VELOCITY: Self = Self { ord: 1 };
    pub const BODY_STATE_ANGULAR_VELOCITY: Self = Self { ord: 2 };
    pub const BODY_STATE_SLEEPING: Self = Self { ord: 3 };
    pub const BODY_STATE_CAN_SLEEP: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for BodyState {
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
unsafe impl sys::GodotFfi for BodyState {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AreaBodyStatus {
    ord: i32,
}
impl AreaBodyStatus {
    pub const AREA_BODY_ADDED: Self = Self { ord: 0 };
    pub const AREA_BODY_REMOVED: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for AreaBodyStatus {
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
unsafe impl sys::GodotFfi for AreaBodyStatus {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ProcessInfo {
    ord: i32,
}
impl ProcessInfo {
    pub const INFO_ACTIVE_OBJECTS: Self = Self { ord: 0 };
    pub const INFO_COLLISION_PAIRS: Self = Self { ord: 1 };
    pub const INFO_ISLAND_COUNT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ProcessInfo {
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
unsafe impl sys::GodotFfi for ProcessInfo {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SpaceParameter {
    ord: i32,
}
impl SpaceParameter {
    pub const SPACE_PARAM_CONTACT_RECYCLE_RADIUS: Self = Self { ord: 0 };
    pub const SPACE_PARAM_CONTACT_MAX_SEPARATION: Self = Self { ord: 1 };
    pub const SPACE_PARAM_CONTACT_MAX_ALLOWED_PENETRATION: Self = Self { ord: 2 };
    pub const SPACE_PARAM_CONTACT_DEFAULT_BIAS: Self = Self { ord: 3 };
    pub const SPACE_PARAM_BODY_LINEAR_VELOCITY_SLEEP_THRESHOLD: Self = Self { ord: 4 };
    pub const SPACE_PARAM_BODY_ANGULAR_VELOCITY_SLEEP_THRESHOLD: Self = Self { ord: 5 };
    pub const SPACE_PARAM_BODY_TIME_TO_SLEEP: Self = Self { ord: 6 };
    pub const SPACE_PARAM_SOLVER_ITERATIONS: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for SpaceParameter {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for SpaceParameter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BodyAxis {
    ord: i32,
}
impl BodyAxis {
    pub const BODY_AXIS_LINEAR_X: Self = Self { ord: 1 };
    pub const BODY_AXIS_LINEAR_Y: Self = Self { ord: 2 };
    pub const BODY_AXIS_LINEAR_Z: Self = Self { ord: 4 };
    pub const BODY_AXIS_ANGULAR_X: Self = Self { ord: 8 };
    pub const BODY_AXIS_ANGULAR_Y: Self = Self { ord: 16 };
    pub const BODY_AXIS_ANGULAR_Z: Self = Self { ord: 32 };
}
impl crate::obj::EngineEnum for BodyAxis {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 | ord @ 32i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for BodyAxis {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
