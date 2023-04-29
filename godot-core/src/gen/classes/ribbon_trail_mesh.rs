#![doc = "Sidecar module for class [`RibbonTrailMesh`][crate::engine::RibbonTrailMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RibbonTrailMesh` enums](https://docs.godotengine.org/en/stable/classes/class_ribbontrailmesh.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `RibbonTrailMesh.`\n\nInherits [`PrimitiveMesh`][crate::engine::PrimitiveMesh].\n\nRelated symbols:\n\n* [`ribbon_trail_mesh`][crate::engine::ribbon_trail_mesh]: sidecar module with related enum/flag types\n* [`RibbonTrailMeshVirtual`][crate::engine::RibbonTrailMeshVirtual]: virtual methods\n\n\nSee also [Godot docs for `RibbonTrailMesh`](https://docs.godotengine.org/en/stable/classes/class_ribbontrailmesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct RibbonTrailMesh {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`RibbonTrailMesh`][crate::engine::RibbonTrailMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RibbonTrailMesh` methods](https://docs.godotengine.org/en/stable/classes/class_ribbontrailmesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait RibbonTrailMeshVirtual:
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
        fn create_mesh_array(&self) -> VariantArray {
            unimplemented!()
        }
        fn get_surface_count(&self) -> i64 {
            unimplemented!()
        }
        fn surface_get_array_len(&self, index: i64) -> i64 {
            unimplemented!()
        }
        fn surface_get_array_index_len(&self, index: i64) -> i64 {
            unimplemented!()
        }
        fn surface_get_arrays(&self, index: i64) -> VariantArray {
            unimplemented!()
        }
        fn surface_get_blend_shape_arrays(&self, index: i64) -> Array<VariantArray> {
            unimplemented!()
        }
        fn surface_get_lods(&self, index: i64) -> Dictionary {
            unimplemented!()
        }
        fn surface_get_format(&self, index: i64) -> i64 {
            unimplemented!()
        }
        fn surface_get_primitive_type(&self, index: i64) -> i64 {
            unimplemented!()
        }
        fn surface_set_material(&mut self, index: i64, material: Gd<Material>) {
            unimplemented!()
        }
        fn surface_get_material(&self, index: i64) -> Option<Gd<Material>> {
            unimplemented!()
        }
        fn get_blend_shape_count(&self) -> i64 {
            unimplemented!()
        }
        fn get_blend_shape_name(&self, index: i64) -> StringName {
            unimplemented!()
        }
        fn set_blend_shape_name(&mut self, index: i64, name: StringName) {
            unimplemented!()
        }
        fn get_aabb(&self) -> Aabb {
            unimplemented!()
        }
    }
    impl RibbonTrailMesh {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
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
        pub fn set_size(&mut self, size: f64) {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "set_size" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_size(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "get_size" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_sections(&mut self, sections: i64) {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("set_sections");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "set_sections" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&sections)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_sections(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("get_sections");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "get_sections" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_section_length(&mut self, section_length: f64) {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("set_section_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "set_section_length" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&section_length)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_section_length(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("get_section_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "get_section_length" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_section_segments(&mut self, section_segments: i64) {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("set_section_segments");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "set_section_segments" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&section_segments)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_section_segments(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("get_section_segments");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "get_section_segments" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_curve(&mut self, curve: Gd<Curve>) {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("set_curve");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    270443179i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "set_curve" , 270443179i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Curve> as AsArg>::as_arg_ptr(&curve)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_curve(&self) -> Option<Gd<Curve>> {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("get_curve");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2460114913i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "get_curve" , 2460114913i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Curve>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_shape(&mut self, shape: ribbon_trail_mesh::Shape) {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("set_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1684440262i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "set_shape" , 1684440262i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<ribbon_trail_mesh::Shape as sys::GodotFfi>::sys_const(
                    &shape,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_shape(&self) -> ribbon_trail_mesh::Shape {
            unsafe {
                let __class_name = StringName::from("RibbonTrailMesh");
                let __method_name = StringName::from("get_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1317484155i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RibbonTrailMesh" , "get_shape" , 1317484155i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <ribbon_trail_mesh::Shape as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for RibbonTrailMesh {
        type Base = crate::engine::PrimitiveMesh;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "RibbonTrailMesh";
    }
    impl crate::obj::EngineClass for RibbonTrailMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::PrimitiveMesh> for RibbonTrailMesh {}
    impl crate::obj::Inherits<crate::engine::Mesh> for RibbonTrailMesh {}
    impl crate::obj::Inherits<crate::engine::Resource> for RibbonTrailMesh {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for RibbonTrailMesh {}
    impl crate::obj::Inherits<crate::engine::Object> for RibbonTrailMesh {}
    impl std::ops::Deref for RibbonTrailMesh {
        type Target = crate::engine::PrimitiveMesh;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for RibbonTrailMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_RibbonTrailMesh {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::RibbonTrailMesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::PrimitiveMesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Mesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Shape {
    ord: i32,
}
impl Shape {
    pub const SHAPE_FLAT: Self = Self { ord: 0 };
    pub const SHAPE_CROSS: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for Shape {
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
unsafe impl sys::GodotFfi for Shape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
