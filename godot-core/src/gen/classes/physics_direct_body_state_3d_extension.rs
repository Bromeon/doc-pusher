#![doc = "Sidecar module for class [`PhysicsDirectBodyState3DExtension`][crate::engine::PhysicsDirectBodyState3DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsDirectBodyState3DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3dextension.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `PhysicsDirectBodyState3DExtension.`\n\nInherits [`PhysicsDirectBodyState3D`][crate::engine::PhysicsDirectBodyState3D].\n\nRelated symbols:\n\n* [`PhysicsDirectBodyState3DExtensionVirtual`][crate::engine::PhysicsDirectBodyState3DExtensionVirtual]: virtual methods\n\n\nSee also [Godot docs for `PhysicsDirectBodyState3DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3dextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PhysicsDirectBodyState3DExtension {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`PhysicsDirectBodyState3DExtension`][crate::engine::PhysicsDirectBodyState3DExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsDirectBodyState3DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsdirectbodystate3dextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PhysicsDirectBodyState3DExtensionVirtual:
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
        fn get_total_gravity(&self) -> Vector3 {
            unimplemented!()
        }
        fn get_total_linear_damp(&self) -> f64 {
            unimplemented!()
        }
        fn get_total_angular_damp(&self) -> f64 {
            unimplemented!()
        }
        fn get_center_of_mass(&self) -> Vector3 {
            unimplemented!()
        }
        fn get_center_of_mass_local(&self) -> Vector3 {
            unimplemented!()
        }
        fn get_principal_inertia_axes(&self) -> Basis {
            unimplemented!()
        }
        fn get_inverse_mass(&self) -> f64 {
            unimplemented!()
        }
        fn get_inverse_inertia(&self) -> Vector3 {
            unimplemented!()
        }
        fn get_inverse_inertia_tensor(&self) -> Basis {
            unimplemented!()
        }
        fn set_linear_velocity(&mut self, velocity: Vector3) {
            unimplemented!()
        }
        fn get_linear_velocity(&self) -> Vector3 {
            unimplemented!()
        }
        fn set_angular_velocity(&mut self, velocity: Vector3) {
            unimplemented!()
        }
        fn get_angular_velocity(&self) -> Vector3 {
            unimplemented!()
        }
        fn set_transform(&mut self, transform: Transform3D) {
            unimplemented!()
        }
        fn get_transform(&self) -> Transform3D {
            unimplemented!()
        }
        fn get_velocity_at_local_position(&self, local_position: Vector3) -> Vector3 {
            unimplemented!()
        }
        fn apply_central_impulse(&mut self, impulse: Vector3) {
            unimplemented!()
        }
        fn apply_impulse(&mut self, impulse: Vector3, position: Vector3) {
            unimplemented!()
        }
        fn apply_torque_impulse(&mut self, impulse: Vector3) {
            unimplemented!()
        }
        fn apply_central_force(&mut self, force: Vector3) {
            unimplemented!()
        }
        fn apply_force(&mut self, force: Vector3, position: Vector3) {
            unimplemented!()
        }
        fn apply_torque(&mut self, torque: Vector3) {
            unimplemented!()
        }
        fn add_constant_central_force(&mut self, force: Vector3) {
            unimplemented!()
        }
        fn add_constant_force(&mut self, force: Vector3, position: Vector3) {
            unimplemented!()
        }
        fn add_constant_torque(&mut self, torque: Vector3) {
            unimplemented!()
        }
        fn set_constant_force(&mut self, force: Vector3) {
            unimplemented!()
        }
        fn get_constant_force(&self) -> Vector3 {
            unimplemented!()
        }
        fn set_constant_torque(&mut self, torque: Vector3) {
            unimplemented!()
        }
        fn get_constant_torque(&self) -> Vector3 {
            unimplemented!()
        }
        fn set_sleep_state(&mut self, enabled: bool) {
            unimplemented!()
        }
        fn is_sleeping(&self) -> bool {
            unimplemented!()
        }
        fn get_contact_count(&self) -> i64 {
            unimplemented!()
        }
        fn get_contact_local_position(&self, contact_idx: i64) -> Vector3 {
            unimplemented!()
        }
        fn get_contact_local_normal(&self, contact_idx: i64) -> Vector3 {
            unimplemented!()
        }
        fn get_contact_impulse(&self, contact_idx: i64) -> Vector3 {
            unimplemented!()
        }
        fn get_contact_local_shape(&self, contact_idx: i64) -> i64 {
            unimplemented!()
        }
        fn get_contact_collider(&self, contact_idx: i64) -> Rid {
            unimplemented!()
        }
        fn get_contact_collider_position(&self, contact_idx: i64) -> Vector3 {
            unimplemented!()
        }
        fn get_contact_collider_id(&self, contact_idx: i64) -> i64 {
            unimplemented!()
        }
        fn get_contact_collider_object(&self, contact_idx: i64) -> Option<Gd<Object>> {
            unimplemented!()
        }
        fn get_contact_collider_shape(&self, contact_idx: i64) -> i64 {
            unimplemented!()
        }
        fn get_contact_collider_velocity_at_position(&self, contact_idx: i64) -> Vector3 {
            unimplemented!()
        }
        fn get_step(&self) -> f64 {
            unimplemented!()
        }
        fn integrate_forces(&mut self) {
            unimplemented!()
        }
        fn get_space_state(&mut self) -> Option<Gd<PhysicsDirectSpaceState3D>> {
            unimplemented!()
        }
    }
    impl PhysicsDirectBodyState3DExtension {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("PhysicsDirectBodyState3DExtension");
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
    }
    impl crate::obj::GodotClass for PhysicsDirectBodyState3DExtension {
        type Base = crate::engine::PhysicsDirectBodyState3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "PhysicsDirectBodyState3DExtension";
    }
    impl crate::obj::EngineClass for PhysicsDirectBodyState3DExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::PhysicsDirectBodyState3D>
        for PhysicsDirectBodyState3DExtension
    {
    }
    impl crate::obj::Inherits<crate::engine::Object> for PhysicsDirectBodyState3DExtension {}
    impl std::ops::Deref for PhysicsDirectBodyState3DExtension {
        type Target = crate::engine::PhysicsDirectBodyState3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for PhysicsDirectBodyState3DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_PhysicsDirectBodyState3DExtension {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::PhysicsDirectBodyState3DExtension>
                for $Class
            {
            }
            impl ::godot::obj::Inherits<::godot::engine::PhysicsDirectBodyState3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
