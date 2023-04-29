#![doc = "Sidecar module for class [`PhysicsBody3D`][crate::engine::PhysicsBody3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsBody3D` enums](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `PhysicsBody3D.`\n\nInherits [`CollisionObject3D`][crate::engine::CollisionObject3D].\n\nRelated symbols:\n\n* [`PhysicsBody3DVirtual`][crate::engine::PhysicsBody3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `PhysicsBody3D`](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PhysicsBody3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`PhysicsBody3D`][crate::engine::PhysicsBody3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsBody3D` methods](https://docs.godotengine.org/en/stable/classes/class_physicsbody3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PhysicsBody3DVirtual:
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
        fn on_notification(&mut self, what: Node3DNotification) {
            unimplemented!()
        }
        fn input_event(
            &mut self,
            camera: Gd<Camera3D>,
            event: Gd<InputEvent>,
            position: Vector3,
            normal: Vector3,
            shape_idx: i64,
        ) {
            unimplemented!()
        }
        fn mouse_enter(&mut self) {
            unimplemented!()
        }
        fn mouse_exit(&mut self) {
            unimplemented!()
        }
        fn process(&mut self, delta: f64) {
            unimplemented!()
        }
        fn physics_process(&mut self, delta: f64) {
            unimplemented!()
        }
        fn enter_tree(&mut self) {
            unimplemented!()
        }
        fn exit_tree(&mut self) {
            unimplemented!()
        }
        fn ready(&mut self) {
            unimplemented!()
        }
        fn get_configuration_warnings(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn shortcut_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn unhandled_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
    }
    impl PhysicsBody3D {
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
        pub fn notify(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn move_and_collide(
            &mut self,
            motion: Vector3,
            test_only: bool,
            safe_margin: f64,
            recovery_as_collision: bool,
            max_collisions: i64,
        ) -> Option<Gd<KinematicCollision3D>> {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("move_and_collide");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1140990067i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "move_and_collide" , 1140990067i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&motion),
                    <bool as sys::GodotFfi>::sys_const(&test_only),
                    <f64 as sys::GodotFfi>::sys_const(&safe_margin),
                    <bool as sys::GodotFfi>::sys_const(&recovery_as_collision),
                    <i64 as sys::GodotFfi>::sys_const(&max_collisions),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<KinematicCollision3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn test_move(
            &mut self,
            from: Transform3D,
            motion: Vector3,
            collision: Gd<KinematicCollision3D>,
            safe_margin: f64,
            recovery_as_collision: bool,
            max_collisions: i64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("test_move");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2082761915i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "test_move" , 2082761915i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Transform3D as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&motion),
                    <Gd<KinematicCollision3D> as AsArg>::as_arg_ptr(&collision),
                    <f64 as sys::GodotFfi>::sys_const(&safe_margin),
                    <bool as sys::GodotFfi>::sys_const(&recovery_as_collision),
                    <i64 as sys::GodotFfi>::sys_const(&max_collisions),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_axis_lock(&mut self, axis: physics_server_3d::BodyAxis, lock: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("set_axis_lock");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1787895195i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "set_axis_lock" , 1787895195i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <physics_server_3d::BodyAxis as sys::GodotFfi>::sys_const(&axis),
                    <bool as sys::GodotFfi>::sys_const(&lock),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_axis_lock(&self, axis: physics_server_3d::BodyAxis) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("get_axis_lock");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2264617709i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "get_axis_lock" , 2264617709i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<physics_server_3d::BodyAxis as sys::GodotFfi>::sys_const(
                    &axis,
                )];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_collision_exceptions(&mut self) -> Array<Gd<PhysicsBody3D>> {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("get_collision_exceptions");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "get_collision_exceptions" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<PhysicsBody3D>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_collision_exception_with(&mut self, body: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("add_collision_exception_with");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "add_collision_exception_with" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&body)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_collision_exception_with(&mut self, body: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("PhysicsBody3D");
                let __method_name = StringName::from("remove_collision_exception_with");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsBody3D" , "remove_collision_exception_with" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&body)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for PhysicsBody3D {
        type Base = crate::engine::CollisionObject3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "PhysicsBody3D";
    }
    impl crate::obj::EngineClass for PhysicsBody3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::CollisionObject3D> for PhysicsBody3D {}
    impl crate::obj::Inherits<crate::engine::Node3D> for PhysicsBody3D {}
    impl crate::obj::Inherits<crate::engine::Node> for PhysicsBody3D {}
    impl crate::obj::Inherits<crate::engine::Object> for PhysicsBody3D {}
    impl std::ops::Deref for PhysicsBody3D {
        type Target = crate::engine::CollisionObject3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for PhysicsBody3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_PhysicsBody3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::PhysicsBody3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::CollisionObject3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
