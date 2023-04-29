#![doc = "Sidecar module for class [`GpuParticlesCollisionHeightField3D`][crate::engine::GpuParticlesCollisionHeightField3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GPUParticlesCollisionHeightField3D` enums](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionheightfield3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `GPUParticlesCollisionHeightField3D.`\n\nInherits [`GpuParticlesCollision3D`][crate::engine::GpuParticlesCollision3D].\n\nRelated symbols:\n\n* [`gpu_particles_collision_height_field_3d`][crate::engine::gpu_particles_collision_height_field_3d]: sidecar module with related enum/flag types\n* [`GpuParticlesCollisionHeightField3DVirtual`][crate::engine::GpuParticlesCollisionHeightField3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `GPUParticlesCollisionHeightField3D`](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionheightfield3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct GpuParticlesCollisionHeightField3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`GpuParticlesCollisionHeightField3D`][crate::engine::GpuParticlesCollisionHeightField3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GPUParticlesCollisionHeightField3D` methods](https://docs.godotengine.org/en/stable/classes/class_gpuparticlescollisionheightfield3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait GpuParticlesCollisionHeightField3DVirtual:
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
        fn get_aabb(&self) -> Aabb {
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
    impl GpuParticlesCollisionHeightField3D {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
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
        pub fn notify(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn set_size(&mut self, size: Vector3) {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "set_size" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_size(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "get_size" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_resolution(
            &mut self,
            resolution: gpu_particles_collision_height_field_3d::Resolution,
        ) {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("set_resolution");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1009996517i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "set_resolution" , 1009996517i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< gpu_particles_collision_height_field_3d :: Resolution as sys :: GodotFfi > :: sys_const (& resolution)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_resolution(&self) -> gpu_particles_collision_height_field_3d::Resolution {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("get_resolution");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1156065644i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "get_resolution" , 1156065644i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < gpu_particles_collision_height_field_3d :: Resolution as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_update_mode(
            &mut self,
            update_mode: gpu_particles_collision_height_field_3d::UpdateMode,
        ) {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("set_update_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    673680859i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "set_update_mode" , 673680859i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< gpu_particles_collision_height_field_3d :: UpdateMode as sys :: GodotFfi > :: sys_const (& update_mode)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_update_mode(&self) -> gpu_particles_collision_height_field_3d::UpdateMode {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("get_update_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1998141380i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "get_update_mode" , 1998141380i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < gpu_particles_collision_height_field_3d :: UpdateMode as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_follow_camera_enabled(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("set_follow_camera_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "set_follow_camera_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_follow_camera_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("GPUParticlesCollisionHeightField3D");
                let __method_name = StringName::from("is_follow_camera_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GPUParticlesCollisionHeightField3D" , "is_follow_camera_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for GpuParticlesCollisionHeightField3D {
        type Base = crate::engine::GpuParticlesCollision3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "GPUParticlesCollisionHeightField3D";
    }
    impl crate::obj::EngineClass for GpuParticlesCollisionHeightField3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::GpuParticlesCollision3D>
        for GpuParticlesCollisionHeightField3D
    {
    }
    impl crate::obj::Inherits<crate::engine::VisualInstance3D> for GpuParticlesCollisionHeightField3D {}
    impl crate::obj::Inherits<crate::engine::Node3D> for GpuParticlesCollisionHeightField3D {}
    impl crate::obj::Inherits<crate::engine::Node> for GpuParticlesCollisionHeightField3D {}
    impl crate::obj::Inherits<crate::engine::Object> for GpuParticlesCollisionHeightField3D {}
    impl std::ops::Deref for GpuParticlesCollisionHeightField3D {
        type Target = crate::engine::GpuParticlesCollision3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for GpuParticlesCollisionHeightField3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_GpuParticlesCollisionHeightField3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::GpuParticlesCollisionHeightField3D>
                for $Class
            {
            }
            impl ::godot::obj::Inherits<::godot::engine::GpuParticlesCollision3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualInstance3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Resolution {
    ord: i32,
}
impl Resolution {
    pub const RESOLUTION_256: Self = Self { ord: 0 };
    pub const RESOLUTION_512: Self = Self { ord: 1 };
    pub const RESOLUTION_1024: Self = Self { ord: 2 };
    pub const RESOLUTION_2048: Self = Self { ord: 3 };
    pub const RESOLUTION_4096: Self = Self { ord: 4 };
    pub const RESOLUTION_8192: Self = Self { ord: 5 };
    pub const RESOLUTION_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for Resolution {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Resolution {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct UpdateMode {
    ord: i32,
}
impl UpdateMode {
    pub const UPDATE_MODE_WHEN_MOVED: Self = Self { ord: 0 };
    pub const UPDATE_MODE_ALWAYS: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for UpdateMode {
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
unsafe impl sys::GodotFfi for UpdateMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
