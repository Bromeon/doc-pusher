#![doc = "Sidecar module for class [`PhysicalBone2D`][crate::engine::PhysicalBone2D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicalBone2D` enums](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `PhysicalBone2D.`\n\nInherits [`RigidBody2D`][crate::engine::RigidBody2D].\n\nRelated symbols:\n\n* [`PhysicalBone2DVirtual`][crate::engine::PhysicalBone2DVirtual]: virtual methods\n\n\nSee also [Godot docs for `PhysicalBone2D`](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PhysicalBone2D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`PhysicalBone2D`][crate::engine::PhysicalBone2D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicalBone2D` methods](https://docs.godotengine.org/en/stable/classes/class_physicalbone2d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PhysicalBone2DVirtual:
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented!()
        }
        fn integrate_forces(&mut self, state: Gd<PhysicsDirectBodyState2D>) {
            unimplemented!()
        }
        fn input_event(&mut self, viewport: Gd<Viewport>, event: Gd<InputEvent>, shape_idx: i64) {
            unimplemented!()
        }
        fn mouse_enter(&mut self) {
            unimplemented!()
        }
        fn mouse_exit(&mut self) {
            unimplemented!()
        }
        fn mouse_shape_enter(&mut self, shape_idx: i64) {
            unimplemented!()
        }
        fn mouse_shape_exit(&mut self, shape_idx: i64) {
            unimplemented!()
        }
        fn draw(&mut self) {
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
    impl PhysicalBone2D {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
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
        pub fn notify(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn get_joint(&self) -> Option<Gd<Joint2D>> {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("get_joint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3582132112i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "get_joint" , 3582132112i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Joint2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_auto_configure_joint(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("get_auto_configure_joint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "get_auto_configure_joint" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_auto_configure_joint(&mut self, auto_configure_joint: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("set_auto_configure_joint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "set_auto_configure_joint" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&auto_configure_joint)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_simulate_physics(&mut self, simulate_physics: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("set_simulate_physics");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "set_simulate_physics" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&simulate_physics)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_simulate_physics(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("get_simulate_physics");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "get_simulate_physics" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_simulating_physics(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("is_simulating_physics");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "is_simulating_physics" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone2d_nodepath(&mut self, nodepath: NodePath) {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("set_bone2d_nodepath");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1348162250i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "set_bone2d_nodepath" , 1348162250i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&nodepath)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone2d_nodepath(&self) -> NodePath {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("get_bone2d_nodepath");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4075236667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "get_bone2d_nodepath" , 4075236667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone2d_index(&mut self, bone_index: i64) {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("set_bone2d_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "set_bone2d_index" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone2d_index(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("get_bone2d_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "get_bone2d_index" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_follow_bone_when_simulating(&mut self, follow_bone: bool) {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("set_follow_bone_when_simulating");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "set_follow_bone_when_simulating" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&follow_bone)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_follow_bone_when_simulating(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicalBone2D");
                let __method_name = StringName::from("get_follow_bone_when_simulating");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicalBone2D" , "get_follow_bone_when_simulating" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for PhysicalBone2D {
        type Base = crate::engine::RigidBody2D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "PhysicalBone2D";
    }
    impl crate::obj::EngineClass for PhysicalBone2D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RigidBody2D> for PhysicalBone2D {}
    impl crate::obj::Inherits<crate::engine::PhysicsBody2D> for PhysicalBone2D {}
    impl crate::obj::Inherits<crate::engine::CollisionObject2D> for PhysicalBone2D {}
    impl crate::obj::Inherits<crate::engine::Node2D> for PhysicalBone2D {}
    impl crate::obj::Inherits<crate::engine::CanvasItem> for PhysicalBone2D {}
    impl crate::obj::Inherits<crate::engine::Node> for PhysicalBone2D {}
    impl crate::obj::Inherits<crate::engine::Object> for PhysicalBone2D {}
    impl std::ops::Deref for PhysicalBone2D {
        type Target = crate::engine::RigidBody2D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for PhysicalBone2D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_PhysicalBone2D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::PhysicalBone2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RigidBody2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::PhysicsBody2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::CollisionObject2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::CanvasItem> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
