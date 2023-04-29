#![doc = "Sidecar module for class [`BaseMaterial3D`][crate::engine::BaseMaterial3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BaseMaterial3D` enums](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `BaseMaterial3D.`\n\nInherits [`Material`][crate::engine::Material].\n\nRelated symbols:\n\n* [`base_material_3d`][crate::engine::base_material_3d]: sidecar module with related enum/flag types\n* [`BaseMaterial3DVirtual`][crate::engine::BaseMaterial3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `BaseMaterial3D`](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct BaseMaterial3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`BaseMaterial3D`][crate::engine::BaseMaterial3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BaseMaterial3D` methods](https://docs.godotengine.org/en/stable/classes/class_basematerial3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait BaseMaterial3DVirtual:
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
        fn get_shader_rid(&self) -> Rid {
            unimplemented!()
        }
        fn get_shader_mode(&self) -> shader::Mode {
            unimplemented!()
        }
        fn can_do_next_pass(&self) -> bool {
            unimplemented!()
        }
        fn can_use_render_priority(&self) -> bool {
            unimplemented!()
        }
    }
    impl BaseMaterial3D {
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
        pub fn set_albedo(&mut self, albedo: Color) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_albedo");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_albedo" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&albedo)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_albedo(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_albedo");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_albedo" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_transparency(&mut self, transparency: base_material_3d::Transparency) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_transparency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3435651667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_transparency" , 3435651667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::Transparency as sys::GodotFfi>::sys_const(&transparency)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transparency(&self) -> base_material_3d::Transparency {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_transparency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    990903061i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_transparency" , 990903061i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::Transparency as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: base_material_3d::AlphaAntiAliasing) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_alpha_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3212649852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_alpha_antialiasing" , 3212649852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::AlphaAntiAliasing as sys::GodotFfi>::sys_const(&alpha_aa),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_antialiasing(&self) -> base_material_3d::AlphaAntiAliasing {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_alpha_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2889939400i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_alpha_antialiasing" , 2889939400i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::AlphaAntiAliasing as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_alpha_antialiasing_edge");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_alpha_antialiasing_edge" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&edge)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_antialiasing_edge(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_alpha_antialiasing_edge");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_alpha_antialiasing_edge" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_shading_mode(&mut self, shading_mode: base_material_3d::ShadingMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_shading_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3368750322i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_shading_mode" , 3368750322i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::ShadingMode as sys::GodotFfi>::sys_const(
                    &shading_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_shading_mode(&self) -> base_material_3d::ShadingMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_shading_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2132070559i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_shading_mode" , 2132070559i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::ShadingMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_specular(&mut self, specular: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_specular");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_specular" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&specular)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_specular(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_specular");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_specular" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_metallic(&mut self, metallic: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_metallic");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_metallic" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&metallic)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_metallic(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_metallic");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_metallic" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_roughness(&mut self, roughness: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_roughness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_roughness" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&roughness)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_roughness(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_roughness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_roughness" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission(&mut self, emission: Color) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_emission");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_emission" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&emission)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_emission");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_emission" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_energy_multiplier(&mut self, emission_energy_multiplier: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_emission_energy_multiplier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_emission_energy_multiplier" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(
                    &emission_energy_multiplier,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_energy_multiplier(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_emission_energy_multiplier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_emission_energy_multiplier" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_intensity(&mut self, emission_energy_multiplier: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_emission_intensity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_emission_intensity" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(
                    &emission_energy_multiplier,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_intensity(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_emission_intensity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_emission_intensity" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_normal_scale(&mut self, normal_scale: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_normal_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_normal_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&normal_scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_normal_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_normal_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_normal_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_rim(&mut self, rim: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_rim");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_rim" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&rim)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rim(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_rim");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_rim" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_rim_tint(&mut self, rim_tint: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_rim_tint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_rim_tint" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&rim_tint)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rim_tint(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_rim_tint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_rim_tint" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_clearcoat(&mut self, clearcoat: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_clearcoat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_clearcoat" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&clearcoat)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_clearcoat(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_clearcoat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_clearcoat" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_clearcoat_roughness(&mut self, clearcoat_roughness: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_clearcoat_roughness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_clearcoat_roughness" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&clearcoat_roughness)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_clearcoat_roughness(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_clearcoat_roughness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_clearcoat_roughness" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_anisotropy(&mut self, anisotropy: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_anisotropy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_anisotropy" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&anisotropy)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_anisotropy(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_anisotropy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_anisotropy" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_heightmap_scale(&mut self, heightmap_scale: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_heightmap_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_heightmap_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&heightmap_scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_heightmap_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_heightmap_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_heightmap_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_subsurface_scattering_strength(&mut self, strength: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_subsurface_scattering_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_subsurface_scattering_strength" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&strength)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_subsurface_scattering_strength(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_subsurface_scattering_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_subsurface_scattering_strength" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_transmittance_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_transmittance_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_transmittance_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transmittance_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_transmittance_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_transmittance_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_transmittance_depth(&mut self, depth: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_transmittance_depth");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_transmittance_depth" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&depth)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transmittance_depth(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_transmittance_depth");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_transmittance_depth" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_transmittance_boost(&mut self, boost: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_transmittance_boost");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_transmittance_boost" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&boost)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transmittance_boost(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_transmittance_boost");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_transmittance_boost" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_backlight(&mut self, backlight: Color) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_backlight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_backlight" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&backlight)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_backlight(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_backlight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_backlight" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_refraction(&mut self, refraction: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_refraction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_refraction" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&refraction)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_refraction(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_refraction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_refraction" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_point_size(&mut self, point_size: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_point_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_point_size" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&point_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_point_size(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_point_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_point_size" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_detail_uv(&mut self, detail_uv: base_material_3d::DetailUV) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_detail_uv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    456801921i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_detail_uv" , 456801921i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::DetailUV as sys::GodotFfi>::sys_const(
                    &detail_uv,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_detail_uv(&self) -> base_material_3d::DetailUV {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_detail_uv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2306920512i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_detail_uv" , 2306920512i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::DetailUV as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_blend_mode(&mut self, blend_mode: base_material_3d::BlendMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2830186259i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_blend_mode" , 2830186259i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::BlendMode as sys::GodotFfi>::sys_const(
                    &blend_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_mode(&self) -> base_material_3d::BlendMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4022690962i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_blend_mode" , 4022690962i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::BlendMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_depth_draw_mode(&mut self, depth_draw_mode: base_material_3d::DepthDrawMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_depth_draw_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1456584748i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_depth_draw_mode" , 1456584748i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::DepthDrawMode as sys::GodotFfi>::sys_const(&depth_draw_mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_depth_draw_mode(&self) -> base_material_3d::DepthDrawMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_depth_draw_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2578197639i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_depth_draw_mode" , 2578197639i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::DepthDrawMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_cull_mode(&mut self, cull_mode: base_material_3d::CullMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_cull_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2338909218i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_cull_mode" , 2338909218i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::CullMode as sys::GodotFfi>::sys_const(
                    &cull_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cull_mode(&self) -> base_material_3d::CullMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_cull_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1941499586i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_cull_mode" , 1941499586i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::CullMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_diffuse_mode(&mut self, diffuse_mode: base_material_3d::DiffuseMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_diffuse_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1045299638i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_diffuse_mode" , 1045299638i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::DiffuseMode as sys::GodotFfi>::sys_const(
                    &diffuse_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_diffuse_mode(&self) -> base_material_3d::DiffuseMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_diffuse_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3973617136i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_diffuse_mode" , 3973617136i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::DiffuseMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_specular_mode(&mut self, specular_mode: base_material_3d::SpecularMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_specular_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    584737147i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_specular_mode" , 584737147i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::SpecularMode as sys::GodotFfi>::sys_const(&specular_mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_specular_mode(&self) -> base_material_3d::SpecularMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_specular_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2569953298i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_specular_mode" , 2569953298i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::SpecularMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_flag(&mut self, flag: base_material_3d::Flags, enable: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3070159527i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_flag" , 3070159527i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::Flags as sys::GodotFfi>::sys_const(&flag),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_flag(&self, flag: base_material_3d::Flags) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410065i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_flag" , 1286410065i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::Flags as sys::GodotFfi>::sys_const(&flag)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_filter(&mut self, mode: base_material_3d::TextureFilter) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    22904437i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_texture_filter" , 22904437i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::TextureFilter as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_filter(&self) -> base_material_3d::TextureFilter {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3289213076i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_texture_filter" , 3289213076i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::TextureFilter as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_feature(&mut self, feature: base_material_3d::Feature, enable: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2819288693i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_feature" , 2819288693i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::Feature as sys::GodotFfi>::sys_const(&feature),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_feature(&self, feature: base_material_3d::Feature) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965241794i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_feature" , 1965241794i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::Feature as sys::GodotFfi>::sys_const(
                    &feature,
                )];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture(
            &mut self,
            param: base_material_3d::TextureParam,
            texture: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    464208135i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_texture" , 464208135i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::TextureParam as sys::GodotFfi>::sys_const(&param),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture(&self, param: base_material_3d::TextureParam) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    329605813i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_texture" , 329605813i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::TextureParam as sys::GodotFfi>::sys_const(&param)];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_detail_blend_mode(&mut self, detail_blend_mode: base_material_3d::BlendMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_detail_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2830186259i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_detail_blend_mode" , 2830186259i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::BlendMode as sys::GodotFfi>::sys_const(
                    &detail_blend_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_detail_blend_mode(&self) -> base_material_3d::BlendMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_detail_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4022690962i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_detail_blend_mode" , 4022690962i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::BlendMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_uv1_scale(&mut self, scale: Vector3) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_uv1_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_uv1_scale" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_uv1_scale(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_uv1_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_uv1_scale" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uv1_offset(&mut self, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_uv1_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_uv1_offset" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_uv1_offset(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_uv1_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_uv1_offset" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uv1_triplanar_blend_sharpness(&mut self, sharpness: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_uv1_triplanar_blend_sharpness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_uv1_triplanar_blend_sharpness" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&sharpness)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_uv1_triplanar_blend_sharpness(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_uv1_triplanar_blend_sharpness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_uv1_triplanar_blend_sharpness" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uv2_scale(&mut self, scale: Vector3) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_uv2_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_uv2_scale" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_uv2_scale(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_uv2_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_uv2_scale" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uv2_offset(&mut self, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_uv2_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_uv2_offset" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_uv2_offset(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_uv2_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_uv2_offset" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uv2_triplanar_blend_sharpness(&mut self, sharpness: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_uv2_triplanar_blend_sharpness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_uv2_triplanar_blend_sharpness" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&sharpness)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_uv2_triplanar_blend_sharpness(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_uv2_triplanar_blend_sharpness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_uv2_triplanar_blend_sharpness" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_billboard_mode(&mut self, mode: base_material_3d::BillboardMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_billboard_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4202036497i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_billboard_mode" , 4202036497i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::BillboardMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_billboard_mode(&self) -> base_material_3d::BillboardMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_billboard_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1283840139i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_billboard_mode" , 1283840139i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::BillboardMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_particles_anim_h_frames(&mut self, frames: i64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_particles_anim_h_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_particles_anim_h_frames" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&frames)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_anim_h_frames(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_particles_anim_h_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_particles_anim_h_frames" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_particles_anim_v_frames(&mut self, frames: i64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_particles_anim_v_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_particles_anim_v_frames" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&frames)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_anim_v_frames(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_particles_anim_v_frames");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_particles_anim_v_frames" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_particles_anim_loop(&mut self, loop_: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_particles_anim_loop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_particles_anim_loop" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&loop_)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_particles_anim_loop(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_particles_anim_loop");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_particles_anim_loop" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_heightmap_deep_parallax(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_heightmap_deep_parallax");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_heightmap_deep_parallax" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_heightmap_deep_parallax_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("is_heightmap_deep_parallax_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "is_heightmap_deep_parallax_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_heightmap_deep_parallax_min_layers(&mut self, layer: i64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_heightmap_deep_parallax_min_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_heightmap_deep_parallax_min_layers" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_heightmap_deep_parallax_min_layers(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_heightmap_deep_parallax_min_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_heightmap_deep_parallax_min_layers" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_heightmap_deep_parallax_max_layers(&mut self, layer: i64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_heightmap_deep_parallax_max_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_heightmap_deep_parallax_max_layers" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_heightmap_deep_parallax_max_layers(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_heightmap_deep_parallax_max_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_heightmap_deep_parallax_max_layers" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_heightmap_deep_parallax_flip_tangent(&mut self, flip: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_heightmap_deep_parallax_flip_tangent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_heightmap_deep_parallax_flip_tangent" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&flip)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_heightmap_deep_parallax_flip_tangent(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_heightmap_deep_parallax_flip_tangent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_heightmap_deep_parallax_flip_tangent" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_heightmap_deep_parallax_flip_binormal(&mut self, flip: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_heightmap_deep_parallax_flip_binormal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_heightmap_deep_parallax_flip_binormal" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&flip)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_heightmap_deep_parallax_flip_binormal(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_heightmap_deep_parallax_flip_binormal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_heightmap_deep_parallax_flip_binormal" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_grow(&mut self, amount: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_grow");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_grow" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&amount)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_grow(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_grow");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_grow" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_emission_operator(&mut self, operator: base_material_3d::EmissionOperator) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_emission_operator");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3825128922i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_emission_operator" , 3825128922i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::EmissionOperator as sys::GodotFfi>::sys_const(&operator)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_emission_operator(&self) -> base_material_3d::EmissionOperator {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_emission_operator");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    974205018i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_emission_operator" , 974205018i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::EmissionOperator as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_ao_light_affect(&mut self, amount: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_ao_light_affect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_ao_light_affect" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&amount)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_ao_light_affect(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_ao_light_affect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_ao_light_affect" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_alpha_scissor_threshold");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_alpha_scissor_threshold" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&threshold)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_scissor_threshold(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_alpha_scissor_threshold");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_alpha_scissor_threshold" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_alpha_hash_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_alpha_hash_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&threshold)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_hash_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_alpha_hash_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_alpha_hash_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_grow_enabled(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_grow_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_grow_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_grow_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("is_grow_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "is_grow_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_metallic_texture_channel(&mut self, channel: base_material_3d::TextureChannel) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_metallic_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    744167988i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_metallic_texture_channel" , 744167988i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::TextureChannel as sys::GodotFfi>::sys_const(&channel)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_metallic_texture_channel(&self) -> base_material_3d::TextureChannel {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_metallic_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    568133867i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_metallic_texture_channel" , 568133867i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::TextureChannel as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_roughness_texture_channel(&mut self, channel: base_material_3d::TextureChannel) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_roughness_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    744167988i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_roughness_texture_channel" , 744167988i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::TextureChannel as sys::GodotFfi>::sys_const(&channel)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_roughness_texture_channel(&self) -> base_material_3d::TextureChannel {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_roughness_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    568133867i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_roughness_texture_channel" , 568133867i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::TextureChannel as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_ao_texture_channel(&mut self, channel: base_material_3d::TextureChannel) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_ao_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    744167988i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_ao_texture_channel" , 744167988i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::TextureChannel as sys::GodotFfi>::sys_const(&channel)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_ao_texture_channel(&self) -> base_material_3d::TextureChannel {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_ao_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    568133867i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_ao_texture_channel" , 568133867i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::TextureChannel as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_refraction_texture_channel(
            &mut self,
            channel: base_material_3d::TextureChannel,
        ) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_refraction_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    744167988i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_refraction_texture_channel" , 744167988i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::TextureChannel as sys::GodotFfi>::sys_const(&channel)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_refraction_texture_channel(&self) -> base_material_3d::TextureChannel {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_refraction_texture_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    568133867i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_refraction_texture_channel" , 568133867i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::TextureChannel as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_proximity_fade_enabled(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_proximity_fade_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_proximity_fade_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_proximity_fade_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("is_proximity_fade_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "is_proximity_fade_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_proximity_fade_distance(&mut self, distance: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_proximity_fade_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_proximity_fade_distance" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&distance)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_proximity_fade_distance(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_proximity_fade_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_proximity_fade_distance" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_msdf_pixel_range(&mut self, range: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_msdf_pixel_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_msdf_pixel_range" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&range)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_msdf_pixel_range(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_msdf_pixel_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_msdf_pixel_range" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_msdf_outline_size(&mut self, size: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_msdf_outline_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_msdf_outline_size" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_msdf_outline_size(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_msdf_outline_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_msdf_outline_size" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_distance_fade(&mut self, mode: base_material_3d::DistanceFadeMode) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_distance_fade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1379478617i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_distance_fade" , 1379478617i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<base_material_3d::DistanceFadeMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_distance_fade(&self) -> base_material_3d::DistanceFadeMode {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_distance_fade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2694575734i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_distance_fade" , 2694575734i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::DistanceFadeMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_distance_fade_max_distance(&mut self, distance: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_distance_fade_max_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_distance_fade_max_distance" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&distance)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_distance_fade_max_distance(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_distance_fade_max_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_distance_fade_max_distance" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_distance_fade_min_distance(&mut self, distance: f64) {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("set_distance_fade_min_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "set_distance_fade_min_distance" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&distance)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_distance_fade_min_distance(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("BaseMaterial3D");
                let __method_name = StringName::from("get_distance_fade_min_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BaseMaterial3D" , "get_distance_fade_min_distance" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for BaseMaterial3D {
        type Base = crate::engine::Material;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "BaseMaterial3D";
    }
    impl crate::obj::EngineClass for BaseMaterial3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Material> for BaseMaterial3D {}
    impl crate::obj::Inherits<crate::engine::Resource> for BaseMaterial3D {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for BaseMaterial3D {}
    impl crate::obj::Inherits<crate::engine::Object> for BaseMaterial3D {}
    impl std::ops::Deref for BaseMaterial3D {
        type Target = crate::engine::Material;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for BaseMaterial3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_BaseMaterial3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::BaseMaterial3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Material> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureParam {
    ord: i32,
}
impl TextureParam {
    pub const TEXTURE_ALBEDO: Self = Self { ord: 0 };
    pub const TEXTURE_METALLIC: Self = Self { ord: 1 };
    pub const TEXTURE_ROUGHNESS: Self = Self { ord: 2 };
    pub const TEXTURE_EMISSION: Self = Self { ord: 3 };
    pub const TEXTURE_NORMAL: Self = Self { ord: 4 };
    pub const TEXTURE_RIM: Self = Self { ord: 5 };
    pub const TEXTURE_CLEARCOAT: Self = Self { ord: 6 };
    pub const TEXTURE_FLOWMAP: Self = Self { ord: 7 };
    pub const TEXTURE_AMBIENT_OCCLUSION: Self = Self { ord: 8 };
    pub const TEXTURE_HEIGHTMAP: Self = Self { ord: 9 };
    pub const TEXTURE_SUBSURFACE_SCATTERING: Self = Self { ord: 10 };
    pub const TEXTURE_SUBSURFACE_TRANSMITTANCE: Self = Self { ord: 11 };
    pub const TEXTURE_BACKLIGHT: Self = Self { ord: 12 };
    pub const TEXTURE_REFRACTION: Self = Self { ord: 13 };
    pub const TEXTURE_DETAIL_MASK: Self = Self { ord: 14 };
    pub const TEXTURE_DETAIL_ALBEDO: Self = Self { ord: 15 };
    pub const TEXTURE_DETAIL_NORMAL: Self = Self { ord: 16 };
    pub const TEXTURE_ORM: Self = Self { ord: 17 };
    pub const TEXTURE_MAX: Self = Self { ord: 18 };
}
impl crate::obj::EngineEnum for TextureParam {
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
            | ord @ 18i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TextureParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureFilter {
    ord: i32,
}
impl TextureFilter {
    pub const TEXTURE_FILTER_NEAREST: Self = Self { ord: 0 };
    pub const TEXTURE_FILTER_LINEAR: Self = Self { ord: 1 };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self { ord: 2 };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self { ord: 3 };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC: Self = Self { ord: 4 };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC: Self = Self { ord: 5 };
    pub const TEXTURE_FILTER_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for TextureFilter {
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
unsafe impl sys::GodotFfi for TextureFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DetailUV {
    ord: i32,
}
impl DetailUV {
    pub const DETAIL_UV_1: Self = Self { ord: 0 };
    pub const DETAIL_UV_2: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for DetailUV {
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
unsafe impl sys::GodotFfi for DetailUV {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Transparency {
    ord: i32,
}
impl Transparency {
    pub const TRANSPARENCY_DISABLED: Self = Self { ord: 0 };
    pub const TRANSPARENCY_ALPHA: Self = Self { ord: 1 };
    pub const TRANSPARENCY_ALPHA_SCISSOR: Self = Self { ord: 2 };
    pub const TRANSPARENCY_ALPHA_HASH: Self = Self { ord: 3 };
    pub const TRANSPARENCY_ALPHA_DEPTH_PRE_PASS: Self = Self { ord: 4 };
    pub const TRANSPARENCY_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for Transparency {
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
unsafe impl sys::GodotFfi for Transparency {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShadingMode {
    ord: i32,
}
impl ShadingMode {
    pub const SHADING_MODE_UNSHADED: Self = Self { ord: 0 };
    pub const SHADING_MODE_PER_PIXEL: Self = Self { ord: 1 };
    pub const SHADING_MODE_PER_VERTEX: Self = Self { ord: 2 };
    pub const SHADING_MODE_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ShadingMode {
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
unsafe impl sys::GodotFfi for ShadingMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Feature {
    ord: i32,
}
impl Feature {
    pub const FEATURE_EMISSION: Self = Self { ord: 0 };
    pub const FEATURE_NORMAL_MAPPING: Self = Self { ord: 1 };
    pub const FEATURE_RIM: Self = Self { ord: 2 };
    pub const FEATURE_CLEARCOAT: Self = Self { ord: 3 };
    pub const FEATURE_ANISOTROPY: Self = Self { ord: 4 };
    pub const FEATURE_AMBIENT_OCCLUSION: Self = Self { ord: 5 };
    pub const FEATURE_HEIGHT_MAPPING: Self = Self { ord: 6 };
    pub const FEATURE_SUBSURFACE_SCATTERING: Self = Self { ord: 7 };
    pub const FEATURE_SUBSURFACE_TRANSMITTANCE: Self = Self { ord: 8 };
    pub const FEATURE_BACKLIGHT: Self = Self { ord: 9 };
    pub const FEATURE_REFRACTION: Self = Self { ord: 10 };
    pub const FEATURE_DETAIL: Self = Self { ord: 11 };
    pub const FEATURE_MAX: Self = Self { ord: 12 };
}
impl crate::obj::EngineEnum for Feature {
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
unsafe impl sys::GodotFfi for Feature {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlendMode {
    ord: i32,
}
impl BlendMode {
    pub const BLEND_MODE_MIX: Self = Self { ord: 0 };
    pub const BLEND_MODE_ADD: Self = Self { ord: 1 };
    pub const BLEND_MODE_SUB: Self = Self { ord: 2 };
    pub const BLEND_MODE_MUL: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for BlendMode {
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
unsafe impl sys::GodotFfi for BlendMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AlphaAntiAliasing {
    ord: i32,
}
impl AlphaAntiAliasing {
    pub const ALPHA_ANTIALIASING_OFF: Self = Self { ord: 0 };
    pub const ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE: Self = Self { ord: 1 };
    pub const ALPHA_ANTIALIASING_ALPHA_TO_COVERAGE_AND_TO_ONE: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for AlphaAntiAliasing {
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
unsafe impl sys::GodotFfi for AlphaAntiAliasing {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DepthDrawMode {
    ord: i32,
}
impl DepthDrawMode {
    pub const DEPTH_DRAW_OPAQUE_ONLY: Self = Self { ord: 0 };
    pub const DEPTH_DRAW_ALWAYS: Self = Self { ord: 1 };
    pub const DEPTH_DRAW_DISABLED: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for DepthDrawMode {
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
unsafe impl sys::GodotFfi for DepthDrawMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CullMode {
    ord: i32,
}
impl CullMode {
    pub const CULL_BACK: Self = Self { ord: 0 };
    pub const CULL_FRONT: Self = Self { ord: 1 };
    pub const CULL_DISABLED: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for CullMode {
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
unsafe impl sys::GodotFfi for CullMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Flags {
    ord: i32,
}
impl Flags {
    pub const FLAG_DISABLE_DEPTH_TEST: Self = Self { ord: 0 };
    pub const FLAG_ALBEDO_FROM_VERTEX_COLOR: Self = Self { ord: 1 };
    pub const FLAG_SRGB_VERTEX_COLOR: Self = Self { ord: 2 };
    pub const FLAG_USE_POINT_SIZE: Self = Self { ord: 3 };
    pub const FLAG_FIXED_SIZE: Self = Self { ord: 4 };
    pub const FLAG_BILLBOARD_KEEP_SCALE: Self = Self { ord: 5 };
    pub const FLAG_UV1_USE_TRIPLANAR: Self = Self { ord: 6 };
    pub const FLAG_UV2_USE_TRIPLANAR: Self = Self { ord: 7 };
    pub const FLAG_UV1_USE_WORLD_TRIPLANAR: Self = Self { ord: 8 };
    pub const FLAG_UV2_USE_WORLD_TRIPLANAR: Self = Self { ord: 9 };
    pub const FLAG_AO_ON_UV2: Self = Self { ord: 10 };
    pub const FLAG_EMISSION_ON_UV2: Self = Self { ord: 11 };
    pub const FLAG_ALBEDO_TEXTURE_FORCE_SRGB: Self = Self { ord: 12 };
    pub const FLAG_DONT_RECEIVE_SHADOWS: Self = Self { ord: 13 };
    pub const FLAG_DISABLE_AMBIENT_LIGHT: Self = Self { ord: 14 };
    pub const FLAG_USE_SHADOW_TO_OPACITY: Self = Self { ord: 15 };
    pub const FLAG_USE_TEXTURE_REPEAT: Self = Self { ord: 16 };
    pub const FLAG_INVERT_HEIGHTMAP: Self = Self { ord: 17 };
    pub const FLAG_SUBSURFACE_MODE_SKIN: Self = Self { ord: 18 };
    pub const FLAG_PARTICLE_TRAILS_MODE: Self = Self { ord: 19 };
    pub const FLAG_ALBEDO_TEXTURE_MSDF: Self = Self { ord: 20 };
    pub const FLAG_MAX: Self = Self { ord: 21 };
}
impl crate::obj::EngineEnum for Flags {
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
            | ord @ 21i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Flags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DiffuseMode {
    ord: i32,
}
impl DiffuseMode {
    pub const DIFFUSE_BURLEY: Self = Self { ord: 0 };
    pub const DIFFUSE_LAMBERT: Self = Self { ord: 1 };
    pub const DIFFUSE_LAMBERT_WRAP: Self = Self { ord: 2 };
    pub const DIFFUSE_TOON: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for DiffuseMode {
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
unsafe impl sys::GodotFfi for DiffuseMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SpecularMode {
    ord: i32,
}
impl SpecularMode {
    pub const SPECULAR_SCHLICK_GGX: Self = Self { ord: 0 };
    pub const SPECULAR_TOON: Self = Self { ord: 1 };
    pub const SPECULAR_DISABLED: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for SpecularMode {
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
unsafe impl sys::GodotFfi for SpecularMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BillboardMode {
    ord: i32,
}
impl BillboardMode {
    pub const BILLBOARD_DISABLED: Self = Self { ord: 0 };
    pub const BILLBOARD_ENABLED: Self = Self { ord: 1 };
    pub const BILLBOARD_FIXED_Y: Self = Self { ord: 2 };
    pub const BILLBOARD_PARTICLES: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for BillboardMode {
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
unsafe impl sys::GodotFfi for BillboardMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureChannel {
    ord: i32,
}
impl TextureChannel {
    pub const TEXTURE_CHANNEL_RED: Self = Self { ord: 0 };
    pub const TEXTURE_CHANNEL_GREEN: Self = Self { ord: 1 };
    pub const TEXTURE_CHANNEL_BLUE: Self = Self { ord: 2 };
    pub const TEXTURE_CHANNEL_ALPHA: Self = Self { ord: 3 };
    pub const TEXTURE_CHANNEL_GRAYSCALE: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for TextureChannel {
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
unsafe impl sys::GodotFfi for TextureChannel {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EmissionOperator {
    ord: i32,
}
impl EmissionOperator {
    pub const EMISSION_OP_ADD: Self = Self { ord: 0 };
    pub const EMISSION_OP_MULTIPLY: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for EmissionOperator {
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
unsafe impl sys::GodotFfi for EmissionOperator {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DistanceFadeMode {
    ord: i32,
}
impl DistanceFadeMode {
    pub const DISTANCE_FADE_DISABLED: Self = Self { ord: 0 };
    pub const DISTANCE_FADE_PIXEL_ALPHA: Self = Self { ord: 1 };
    pub const DISTANCE_FADE_PIXEL_DITHER: Self = Self { ord: 2 };
    pub const DISTANCE_FADE_OBJECT_DITHER: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for DistanceFadeMode {
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
unsafe impl sys::GodotFfi for DistanceFadeMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
