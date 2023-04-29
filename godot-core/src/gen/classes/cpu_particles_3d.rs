#![doc = "Sidecar module for class [`CpuParticles3D`][crate::engine::CpuParticles3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CPUParticles3D` enums](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `CPUParticles3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`cpu_particles_3d`][crate::engine::cpu_particles_3d]: sidecar module with related enum/flag types\n* [`CpuParticles3DVirtual`][crate::engine::CpuParticles3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `CPUParticles3D`](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct CpuParticles3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`CpuParticles3D`][crate::engine::CpuParticles3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CPUParticles3D` methods](https://docs.godotengine.org/en/stable/classes/class_cpuparticles3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait CpuParticles3DVirtual:
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
    impl CpuParticles3D {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
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
        pub fn set_emitting(&mut self, emitting: bool) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emitting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emitting" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&emitting)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_amount(&mut self, amount: i64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_amount");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_amount" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&amount)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_lifetime(&mut self, secs: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_lifetime");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_lifetime" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&secs)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_one_shot(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_one_shot");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_one_shot" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_pre_process_time(&mut self, secs: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_pre_process_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_pre_process_time" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&secs)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_explosiveness_ratio(&mut self, ratio: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_explosiveness_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_explosiveness_ratio" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&ratio)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_randomness_ratio(&mut self, ratio: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_randomness_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_randomness_ratio" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&ratio)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_lifetime_randomness(&mut self, random: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_lifetime_randomness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_lifetime_randomness" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&random)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_use_local_coordinates(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_use_local_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_use_local_coordinates" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_fixed_fps(&mut self, fps: i64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_fixed_fps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_fixed_fps" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&fps)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_fractional_delta(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_fractional_delta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_fractional_delta" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_speed_scale(&mut self, scale: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_speed_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_speed_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_emitting(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("is_emitting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "is_emitting" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_amount(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_amount");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_amount" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_lifetime(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_lifetime");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_lifetime" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_one_shot(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_one_shot");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_one_shot" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_pre_process_time(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_pre_process_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_pre_process_time" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_explosiveness_ratio(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_explosiveness_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_explosiveness_ratio" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_randomness_ratio(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_randomness_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_randomness_ratio" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_lifetime_randomness(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_lifetime_randomness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_lifetime_randomness" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_use_local_coordinates(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_use_local_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_use_local_coordinates" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_fixed_fps(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_fixed_fps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_fixed_fps" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_fractional_delta(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_fractional_delta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_fractional_delta" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_speed_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_speed_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_speed_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_draw_order(&mut self, order: cpu_particles_3d::DrawOrder) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_draw_order");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1427401774i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_draw_order" , 1427401774i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<cpu_particles_3d::DrawOrder as sys::GodotFfi>::sys_const(
                    &order,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_draw_order(&self) -> cpu_particles_3d::DrawOrder {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_draw_order");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321900776i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_draw_order" , 1321900776i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <cpu_particles_3d::DrawOrder as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_mesh(&mut self, mesh: Gd<Mesh>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    194775623i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_mesh" , 194775623i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Mesh> as AsArg>::as_arg_ptr(&mesh)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_mesh(&self) -> Option<Gd<Mesh>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1808005922i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_mesh" , 1808005922i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Mesh>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn restart(&mut self) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("restart");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "restart" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_direction(&mut self, direction: Vector3) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_direction" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&direction)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_direction(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_direction" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_spread(&mut self, degrees: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_spread");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_spread" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&degrees)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_spread(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_spread");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_spread" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_flatness(&mut self, amount: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_flatness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_flatness" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&amount)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_flatness(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_flatness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_flatness" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_param_min(&mut self, param: cpu_particles_3d::Parameter, value: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_param_min");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    557936109i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_param_min" , 557936109i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <cpu_particles_3d::Parameter as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_param_min(&self, param: cpu_particles_3d::Parameter) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_param_min");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    597646162i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_param_min" , 597646162i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<cpu_particles_3d::Parameter as sys::GodotFfi>::sys_const(
                    &param,
                )];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_param_max(&mut self, param: cpu_particles_3d::Parameter, value: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_param_max");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    557936109i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_param_max" , 557936109i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <cpu_particles_3d::Parameter as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_param_max(&self, param: cpu_particles_3d::Parameter) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_param_max");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    597646162i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_param_max" , 597646162i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<cpu_particles_3d::Parameter as sys::GodotFfi>::sys_const(
                    &param,
                )];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_param_curve(&mut self, param: cpu_particles_3d::Parameter, curve: Gd<Curve>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_param_curve");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4044142537i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_param_curve" , 4044142537i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <cpu_particles_3d::Parameter as sys::GodotFfi>::sys_const(&param),
                    <Gd<Curve> as AsArg>::as_arg_ptr(&curve),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_param_curve(&self, param: cpu_particles_3d::Parameter) -> Option<Gd<Curve>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_param_curve");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4132790277i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_param_curve" , 4132790277i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<cpu_particles_3d::Parameter as sys::GodotFfi>::sys_const(
                    &param,
                )];
                let __args_ptr = __args.as_ptr();
                <Gd<Curve>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_color_ramp(&mut self, ramp: Gd<Gradient>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_color_ramp");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2756054477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_color_ramp" , 2756054477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Gradient> as AsArg>::as_arg_ptr(&ramp)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color_ramp(&self) -> Option<Gd<Gradient>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_color_ramp");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    132272999i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_color_ramp" , 132272999i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Gradient>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_color_initial_ramp(&mut self, ramp: Gd<Gradient>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_color_initial_ramp");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2756054477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_color_initial_ramp" , 2756054477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Gradient> as AsArg>::as_arg_ptr(&ramp)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color_initial_ramp(&self) -> Option<Gd<Gradient>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_color_initial_ramp");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    132272999i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_color_initial_ramp" , 132272999i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Gradient>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_particle_flag(
            &mut self,
            particle_flag: cpu_particles_3d::ParticleFlags,
            enable: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_particle_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3515406498i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_particle_flag" , 3515406498i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <cpu_particles_3d::ParticleFlags as sys::GodotFfi>::sys_const(&particle_flag),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particle_flag(&self, particle_flag: cpu_particles_3d::ParticleFlags) -> bool {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_particle_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2845201987i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_particle_flag" , 2845201987i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <cpu_particles_3d::ParticleFlags as sys::GodotFfi>::sys_const(&particle_flag),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_shape(&mut self, shape: cpu_particles_3d::EmissionShape) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    491823814i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_shape" , 491823814i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<cpu_particles_3d::EmissionShape as sys::GodotFfi>::sys_const(&shape)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_shape(&self) -> cpu_particles_3d::EmissionShape {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2961454842i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_shape" , 2961454842i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <cpu_particles_3d::EmissionShape as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_emission_sphere_radius(&mut self, radius: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_sphere_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_sphere_radius" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&radius)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_sphere_radius(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_sphere_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_sphere_radius" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_box_extents(&mut self, extents: Vector3) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_box_extents");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_box_extents" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&extents)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_box_extents(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_box_extents");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_box_extents" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_points(&mut self, array: PackedVector3Array) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_points");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    334873810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_points" , 334873810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector3Array as sys::GodotFfi>::sys_const(&array)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_points(&self) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_points");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    497664490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_points" , 497664490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_normals(&mut self, array: PackedVector3Array) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_normals");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    334873810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_normals" , 334873810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector3Array as sys::GodotFfi>::sys_const(&array)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_normals(&self) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_normals");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    497664490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_normals" , 497664490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_colors(&mut self, array: PackedColorArray) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3546319833i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_colors" , 3546319833i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedColorArray as sys::GodotFfi>::sys_const(&array)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_colors(&self) -> PackedColorArray {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1392750486i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_colors" , 1392750486i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedColorArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_ring_axis(&mut self, axis: Vector3) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_ring_axis");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_ring_axis" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&axis)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_ring_axis(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_ring_axis");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_ring_axis" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_ring_height(&mut self, height: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_ring_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_ring_height" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&height)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_ring_height(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_ring_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_ring_height" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_ring_radius(&mut self, radius: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_ring_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_ring_radius" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&radius)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_ring_radius(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_ring_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_ring_radius" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_ring_inner_radius(&mut self, inner_radius: f64) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_emission_ring_inner_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_emission_ring_inner_radius" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&inner_radius)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_ring_inner_radius(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_emission_ring_inner_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_emission_ring_inner_radius" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_gravity(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_gravity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_gravity" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_gravity(&mut self, accel_vec: Vector3) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_gravity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_gravity" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&accel_vec)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_split_scale(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_split_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_split_scale" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_split_scale(&mut self, split_scale: bool) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_split_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_split_scale" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&split_scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_scale_curve_x(&self) -> Option<Gd<Curve>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_scale_curve_x");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2460114913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_scale_curve_x" , 2460114913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Curve>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_scale_curve_x(&mut self, scale_curve: Gd<Curve>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_scale_curve_x");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    270443179i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_scale_curve_x" , 270443179i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Curve> as AsArg>::as_arg_ptr(&scale_curve)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_scale_curve_y(&self) -> Option<Gd<Curve>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_scale_curve_y");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2460114913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_scale_curve_y" , 2460114913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Curve>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_scale_curve_y(&mut self, scale_curve: Gd<Curve>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_scale_curve_y");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    270443179i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_scale_curve_y" , 270443179i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Curve> as AsArg>::as_arg_ptr(&scale_curve)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_scale_curve_z(&self) -> Option<Gd<Curve>> {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("get_scale_curve_z");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2460114913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "get_scale_curve_z" , 2460114913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Curve>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_scale_curve_z(&mut self, scale_curve: Gd<Curve>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("set_scale_curve_z");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    270443179i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "set_scale_curve_z" , 270443179i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Curve> as AsArg>::as_arg_ptr(&scale_curve)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn convert_from_particles(&mut self, particles: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("CPUParticles3D");
                let __method_name = StringName::from("convert_from_particles");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CPUParticles3D" , "convert_from_particles" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&particles)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for CpuParticles3D {
        type Base = crate::engine::GeometryInstance3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "CPUParticles3D";
    }
    impl crate::obj::EngineClass for CpuParticles3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::GeometryInstance3D> for CpuParticles3D {}
    impl crate::obj::Inherits<crate::engine::VisualInstance3D> for CpuParticles3D {}
    impl crate::obj::Inherits<crate::engine::Node3D> for CpuParticles3D {}
    impl crate::obj::Inherits<crate::engine::Node> for CpuParticles3D {}
    impl crate::obj::Inherits<crate::engine::Object> for CpuParticles3D {}
    impl std::ops::Deref for CpuParticles3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for CpuParticles3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_CpuParticles3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::CpuParticles3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::GeometryInstance3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualInstance3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DrawOrder {
    ord: i32,
}
impl DrawOrder {
    pub const DRAW_ORDER_INDEX: Self = Self { ord: 0 };
    pub const DRAW_ORDER_LIFETIME: Self = Self { ord: 1 };
    pub const DRAW_ORDER_VIEW_DEPTH: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for DrawOrder {
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
unsafe impl sys::GodotFfi for DrawOrder {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Parameter {
    ord: i32,
}
impl Parameter {
    pub const PARAM_INITIAL_LINEAR_VELOCITY: Self = Self { ord: 0 };
    pub const PARAM_ANGULAR_VELOCITY: Self = Self { ord: 1 };
    pub const PARAM_ORBIT_VELOCITY: Self = Self { ord: 2 };
    pub const PARAM_LINEAR_ACCEL: Self = Self { ord: 3 };
    pub const PARAM_RADIAL_ACCEL: Self = Self { ord: 4 };
    pub const PARAM_TANGENTIAL_ACCEL: Self = Self { ord: 5 };
    pub const PARAM_DAMPING: Self = Self { ord: 6 };
    pub const PARAM_ANGLE: Self = Self { ord: 7 };
    pub const PARAM_SCALE: Self = Self { ord: 8 };
    pub const PARAM_HUE_VARIATION: Self = Self { ord: 9 };
    pub const PARAM_ANIM_SPEED: Self = Self { ord: 10 };
    pub const PARAM_ANIM_OFFSET: Self = Self { ord: 11 };
    pub const PARAM_MAX: Self = Self { ord: 12 };
}
impl crate::obj::EngineEnum for Parameter {
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
            | ord @ 12i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Parameter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ParticleFlags {
    ord: i32,
}
impl ParticleFlags {
    pub const PARTICLE_FLAG_ALIGN_Y_TO_VELOCITY: Self = Self { ord: 0 };
    pub const PARTICLE_FLAG_ROTATE_Y: Self = Self { ord: 1 };
    pub const PARTICLE_FLAG_DISABLE_Z: Self = Self { ord: 2 };
    pub const PARTICLE_FLAG_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ParticleFlags {
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
unsafe impl sys::GodotFfi for ParticleFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EmissionShape {
    ord: i32,
}
impl EmissionShape {
    pub const EMISSION_SHAPE_POINT: Self = Self { ord: 0 };
    pub const EMISSION_SHAPE_SPHERE: Self = Self { ord: 1 };
    pub const EMISSION_SHAPE_SPHERE_SURFACE: Self = Self { ord: 2 };
    pub const EMISSION_SHAPE_BOX: Self = Self { ord: 3 };
    pub const EMISSION_SHAPE_POINTS: Self = Self { ord: 4 };
    pub const EMISSION_SHAPE_DIRECTED_POINTS: Self = Self { ord: 5 };
    pub const EMISSION_SHAPE_RING: Self = Self { ord: 6 };
    pub const EMISSION_SHAPE_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for EmissionShape {
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
unsafe impl sys::GodotFfi for EmissionShape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
