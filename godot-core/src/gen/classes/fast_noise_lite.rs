#![doc = "Sidecar module for class [`FastNoiseLite`][crate::engine::FastNoiseLite].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `FastNoiseLite` enums](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `FastNoiseLite.`\n\nInherits [`Noise`][crate::engine::Noise].\n\nRelated symbols:\n\n* [`fast_noise_lite`][crate::engine::fast_noise_lite]: sidecar module with related enum/flag types\n* [`FastNoiseLiteVirtual`][crate::engine::FastNoiseLiteVirtual]: virtual methods\n\n\nSee also [Godot docs for `FastNoiseLite`](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct FastNoiseLite {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`FastNoiseLite`][crate::engine::FastNoiseLite].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `FastNoiseLite` methods](https://docs.godotengine.org/en/stable/classes/class_fastnoiselite.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait FastNoiseLiteVirtual:
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
    impl FastNoiseLite {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
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
        pub fn set_noise_type(&mut self, type_: fast_noise_lite::NoiseType) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_noise_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2624461392i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_noise_type" , 2624461392i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<fast_noise_lite::NoiseType as sys::GodotFfi>::sys_const(
                    &type_,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_noise_type(&self) -> fast_noise_lite::NoiseType {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_noise_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1458108610i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_noise_type" , 1458108610i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <fast_noise_lite::NoiseType as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_seed(&mut self, seed: i64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_seed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_seed" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&seed)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_seed(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_seed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_seed" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_frequency(&mut self, freq: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_frequency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_frequency" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&freq)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_frequency(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_frequency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_frequency" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_offset(&mut self, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_offset" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_offset(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_offset" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fractal_type(&mut self, type_: fast_noise_lite::FractalType) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_fractal_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4132731174i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_fractal_type" , 4132731174i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<fast_noise_lite::FractalType as sys::GodotFfi>::sys_const(
                    &type_,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fractal_type(&self) -> fast_noise_lite::FractalType {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_fractal_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1036889279i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_fractal_type" , 1036889279i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <fast_noise_lite::FractalType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_fractal_octaves(&mut self, octave_count: i64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_fractal_octaves");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_fractal_octaves" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&octave_count)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fractal_octaves(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_fractal_octaves");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_fractal_octaves" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fractal_lacunarity(&mut self, lacunarity: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_fractal_lacunarity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_fractal_lacunarity" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&lacunarity)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fractal_lacunarity(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_fractal_lacunarity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_fractal_lacunarity" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fractal_gain(&mut self, gain: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_fractal_gain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_fractal_gain" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&gain)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fractal_gain(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_fractal_gain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_fractal_gain" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fractal_weighted_strength(&mut self, weighted_strength: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_fractal_weighted_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_fractal_weighted_strength" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&weighted_strength)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fractal_weighted_strength(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_fractal_weighted_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_fractal_weighted_strength" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_fractal_ping_pong_strength(&mut self, ping_pong_strength: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_fractal_ping_pong_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_fractal_ping_pong_strength" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&ping_pong_strength)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fractal_ping_pong_strength(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_fractal_ping_pong_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_fractal_ping_pong_strength" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cellular_distance_function(
            &mut self,
            func: fast_noise_lite::CellularDistanceFunction,
        ) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_cellular_distance_function");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1006013267i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_cellular_distance_function" , 1006013267i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <fast_noise_lite::CellularDistanceFunction as sys::GodotFfi>::sys_const(&func),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cellular_distance_function(&self) -> fast_noise_lite::CellularDistanceFunction {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_cellular_distance_function");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2021274088i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_cellular_distance_function" , 2021274088i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <fast_noise_lite::CellularDistanceFunction as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_cellular_jitter(&mut self, jitter: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_cellular_jitter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_cellular_jitter" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&jitter)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cellular_jitter(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_cellular_jitter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_cellular_jitter" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cellular_return_type(&mut self, ret: fast_noise_lite::CellularReturnType) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_cellular_return_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2654169698i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_cellular_return_type" , 2654169698i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<fast_noise_lite::CellularReturnType as sys::GodotFfi>::sys_const(&ret)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cellular_return_type(&self) -> fast_noise_lite::CellularReturnType {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_cellular_return_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3699796343i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_cellular_return_type" , 3699796343i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <fast_noise_lite::CellularReturnType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_domain_warp_enabled(&mut self, domain_warp_enabled: bool) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&domain_warp_enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_domain_warp_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("is_domain_warp_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "is_domain_warp_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_domain_warp_type(&mut self, domain_warp_type: fast_noise_lite::DomainWarpType) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3629692980i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_type" , 3629692980i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <fast_noise_lite::DomainWarpType as sys::GodotFfi>::sys_const(
                        &domain_warp_type,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_type(&self) -> fast_noise_lite::DomainWarpType {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2980162020i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_type" , 2980162020i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <fast_noise_lite::DomainWarpType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_domain_warp_amplitude(&mut self, domain_warp_amplitude: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_amplitude");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_amplitude" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&domain_warp_amplitude)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_amplitude(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_amplitude");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_amplitude" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_domain_warp_frequency(&mut self, domain_warp_frequency: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_frequency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_frequency" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&domain_warp_frequency)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_frequency(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_frequency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_frequency" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_domain_warp_fractal_type(
            &mut self,
            domain_warp_fractal_type: fast_noise_lite::DomainWarpFractalType,
        ) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_fractal_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3999408287i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_fractal_type" , 3999408287i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <fast_noise_lite::DomainWarpFractalType as sys::GodotFfi>::sys_const(
                        &domain_warp_fractal_type,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_fractal_type(&self) -> fast_noise_lite::DomainWarpFractalType {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_fractal_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    407716934i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_fractal_type" , 407716934i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <fast_noise_lite::DomainWarpFractalType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_domain_warp_fractal_octaves(&mut self, domain_warp_octave_count: i64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_fractal_octaves");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_fractal_octaves" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&domain_warp_octave_count)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_fractal_octaves(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_fractal_octaves");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_fractal_octaves" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_domain_warp_fractal_lacunarity(&mut self, domain_warp_lacunarity: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_fractal_lacunarity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_fractal_lacunarity" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&domain_warp_lacunarity)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_fractal_lacunarity(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_fractal_lacunarity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_fractal_lacunarity" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_domain_warp_fractal_gain(&mut self, domain_warp_gain: f64) {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("set_domain_warp_fractal_gain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "set_domain_warp_fractal_gain" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&domain_warp_gain)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_domain_warp_fractal_gain(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("FastNoiseLite");
                let __method_name = StringName::from("get_domain_warp_fractal_gain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "FastNoiseLite" , "get_domain_warp_fractal_gain" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for FastNoiseLite {
        type Base = crate::engine::Noise;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "FastNoiseLite";
    }
    impl crate::obj::EngineClass for FastNoiseLite {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Noise> for FastNoiseLite {}
    impl crate::obj::Inherits<crate::engine::Resource> for FastNoiseLite {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for FastNoiseLite {}
    impl crate::obj::Inherits<crate::engine::Object> for FastNoiseLite {}
    impl std::ops::Deref for FastNoiseLite {
        type Target = crate::engine::Noise;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for FastNoiseLite {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_FastNoiseLite {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::FastNoiseLite> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Noise> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NoiseType {
    ord: i32,
}
impl NoiseType {
    pub const TYPE_VALUE: Self = Self { ord: 5 };
    pub const TYPE_VALUE_CUBIC: Self = Self { ord: 4 };
    pub const TYPE_PERLIN: Self = Self { ord: 3 };
    pub const TYPE_CELLULAR: Self = Self { ord: 2 };
    pub const TYPE_SIMPLEX: Self = Self { ord: 0 };
    pub const TYPE_SIMPLEX_SMOOTH: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for NoiseType {
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
unsafe impl sys::GodotFfi for NoiseType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FractalType {
    ord: i32,
}
impl FractalType {
    pub const FRACTAL_NONE: Self = Self { ord: 0 };
    pub const FRACTAL_FBM: Self = Self { ord: 1 };
    pub const FRACTAL_RIDGED: Self = Self { ord: 2 };
    pub const FRACTAL_PING_PONG: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for FractalType {
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
unsafe impl sys::GodotFfi for FractalType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CellularDistanceFunction {
    ord: i32,
}
impl CellularDistanceFunction {
    pub const DISTANCE_EUCLIDEAN: Self = Self { ord: 0 };
    pub const DISTANCE_EUCLIDEAN_SQUARED: Self = Self { ord: 1 };
    pub const DISTANCE_MANHATTAN: Self = Self { ord: 2 };
    pub const DISTANCE_HYBRID: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for CellularDistanceFunction {
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
unsafe impl sys::GodotFfi for CellularDistanceFunction {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CellularReturnType {
    ord: i32,
}
impl CellularReturnType {
    pub const RETURN_CELL_VALUE: Self = Self { ord: 0 };
    pub const RETURN_DISTANCE: Self = Self { ord: 1 };
    pub const RETURN_DISTANCE2: Self = Self { ord: 2 };
    pub const RETURN_DISTANCE2_ADD: Self = Self { ord: 3 };
    pub const RETURN_DISTANCE2_SUB: Self = Self { ord: 4 };
    pub const RETURN_DISTANCE2_MUL: Self = Self { ord: 5 };
    pub const RETURN_DISTANCE2_DIV: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for CellularReturnType {
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
unsafe impl sys::GodotFfi for CellularReturnType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DomainWarpType {
    ord: i32,
}
impl DomainWarpType {
    pub const DOMAIN_WARP_SIMPLEX: Self = Self { ord: 0 };
    pub const DOMAIN_WARP_SIMPLEX_REDUCED: Self = Self { ord: 1 };
    pub const DOMAIN_WARP_BASIC_GRID: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for DomainWarpType {
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
unsafe impl sys::GodotFfi for DomainWarpType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DomainWarpFractalType {
    ord: i32,
}
impl DomainWarpFractalType {
    pub const DOMAIN_WARP_FRACTAL_NONE: Self = Self { ord: 0 };
    pub const DOMAIN_WARP_FRACTAL_PROGRESSIVE: Self = Self { ord: 1 };
    pub const DOMAIN_WARP_FRACTAL_INDEPENDENT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for DomainWarpFractalType {
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
unsafe impl sys::GodotFfi for DomainWarpFractalType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
