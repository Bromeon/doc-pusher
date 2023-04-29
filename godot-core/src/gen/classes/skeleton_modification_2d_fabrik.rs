#![doc = "Sidecar module for class [`SkeletonModification2Dfabrik`][crate::engine::SkeletonModification2Dfabrik].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SkeletonModification2DFABRIK` enums](https://docs.godotengine.org/en/stable/classes/class_skeletonmodification2dfabrik.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `SkeletonModification2DFABRIK.`\n\nInherits [`SkeletonModification2D`][crate::engine::SkeletonModification2D].\n\nRelated symbols:\n\n* [`SkeletonModification2DfabrikVirtual`][crate::engine::SkeletonModification2DfabrikVirtual]: virtual methods\n\n\nSee also [Godot docs for `SkeletonModification2DFABRIK`](https://docs.godotengine.org/en/stable/classes/class_skeletonmodification2dfabrik.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SkeletonModification2Dfabrik {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`SkeletonModification2Dfabrik`][crate::engine::SkeletonModification2Dfabrik].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SkeletonModification2DFABRIK` methods](https://docs.godotengine.org/en/stable/classes/class_skeletonmodification2dfabrik.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait SkeletonModification2DfabrikVirtual:
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
        fn execute(&mut self, delta: f64) {
            unimplemented!()
        }
        fn setup_modification(&mut self, modification_stack: Gd<SkeletonModificationStack2D>) {
            unimplemented!()
        }
        fn draw_editor_gizmo(&mut self) {
            unimplemented!()
        }
    }
    impl SkeletonModification2Dfabrik {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
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
        pub fn set_target_node(&mut self, target_nodepath: NodePath) {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("set_target_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1348162250i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "set_target_node" , 1348162250i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&target_nodepath)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_target_node(&self) -> NodePath {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("get_target_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4075236667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "get_target_node" , 4075236667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fabrik_data_chain_length(&mut self, length: i64) {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("set_fabrik_data_chain_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "set_fabrik_data_chain_length" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&length)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fabrik_data_chain_length(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("get_fabrik_data_chain_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "get_fabrik_data_chain_length" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fabrik_joint_bone2d_node(&mut self, joint_idx: i64, bone2d_nodepath: NodePath) {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("set_fabrik_joint_bone2d_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2761262315i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "set_fabrik_joint_bone2d_node" , 2761262315i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&joint_idx),
                    <NodePath as sys::GodotFfi>::sys_const(&bone2d_nodepath),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fabrik_joint_bone2d_node(&self, joint_idx: i64) -> NodePath {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("get_fabrik_joint_bone2d_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    408788394i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "get_fabrik_joint_bone2d_node" , 408788394i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&joint_idx)];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fabrik_joint_bone_index(&mut self, joint_idx: i64, bone_idx: i64) {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("set_fabrik_joint_bone_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "set_fabrik_joint_bone_index" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&joint_idx),
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fabrik_joint_bone_index(&self, joint_idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("get_fabrik_joint_bone_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "get_fabrik_joint_bone_index" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&joint_idx)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fabrik_joint_magnet_position(
            &mut self,
            joint_idx: i64,
            magnet_position: Vector2,
        ) {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("set_fabrik_joint_magnet_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    163021252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "set_fabrik_joint_magnet_position" , 163021252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&joint_idx),
                    <Vector2 as sys::GodotFfi>::sys_const(&magnet_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fabrik_joint_magnet_position(&self, joint_idx: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("get_fabrik_joint_magnet_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2299179447i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "get_fabrik_joint_magnet_position" , 2299179447i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&joint_idx)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fabrik_joint_use_target_rotation(
            &mut self,
            joint_idx: i64,
            use_target_rotation: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("set_fabrik_joint_use_target_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "set_fabrik_joint_use_target_rotation" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&joint_idx),
                    <bool as sys::GodotFfi>::sys_const(&use_target_rotation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fabrik_joint_use_target_rotation(&self, joint_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("SkeletonModification2DFABRIK");
                let __method_name = StringName::from("get_fabrik_joint_use_target_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SkeletonModification2DFABRIK" , "get_fabrik_joint_use_target_rotation" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&joint_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for SkeletonModification2Dfabrik {
        type Base = crate::engine::SkeletonModification2D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "SkeletonModification2DFABRIK";
    }
    impl crate::obj::EngineClass for SkeletonModification2Dfabrik {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::SkeletonModification2D> for SkeletonModification2Dfabrik {}
    impl crate::obj::Inherits<crate::engine::Resource> for SkeletonModification2Dfabrik {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for SkeletonModification2Dfabrik {}
    impl crate::obj::Inherits<crate::engine::Object> for SkeletonModification2Dfabrik {}
    impl std::ops::Deref for SkeletonModification2Dfabrik {
        type Target = crate::engine::SkeletonModification2D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for SkeletonModification2Dfabrik {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_SkeletonModification2Dfabrik {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::SkeletonModification2Dfabrik> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::SkeletonModification2D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
