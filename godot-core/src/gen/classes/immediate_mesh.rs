#![doc = "Sidecar module for class [`ImmediateMesh`][crate::engine::ImmediateMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ImmediateMesh` enums](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `ImmediateMesh.`\n\nInherits [`Mesh`][crate::engine::Mesh].\n\nRelated symbols:\n\n* [`ImmediateMeshVirtual`][crate::engine::ImmediateMeshVirtual]: virtual methods\n\n\nSee also [Godot docs for `ImmediateMesh`](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ImmediateMesh {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`ImmediateMesh`][crate::engine::ImmediateMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ImmediateMesh` methods](https://docs.godotengine.org/en/stable/classes/class_immediatemesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ImmediateMeshVirtual:
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
    impl ImmediateMesh {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
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
        pub fn surface_begin(&mut self, primitive: mesh::PrimitiveType, material: Gd<Material>) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_begin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3716480242i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_begin" , 3716480242i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <mesh::PrimitiveType as sys::GodotFfi>::sys_const(&primitive),
                    <Gd<Material> as AsArg>::as_arg_ptr(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_set_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_set_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_set_normal(&mut self, normal: Vector3) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_set_normal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_set_normal" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&normal)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_set_tangent(&mut self, tangent: Plane) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_set_tangent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3505987427i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_set_tangent" , 3505987427i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Plane as sys::GodotFfi>::sys_const(&tangent)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_set_uv(&mut self, uv: Vector2) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_set_uv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_set_uv" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&uv)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_set_uv2(&mut self, uv2: Vector2) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_set_uv2");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_set_uv2" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&uv2)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_add_vertex(&mut self, vertex: Vector3) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_add_vertex");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_add_vertex" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&vertex)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_add_vertex_2d(&mut self, vertex: Vector2) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_add_vertex_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_add_vertex_2d" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&vertex)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_end(&mut self) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("surface_end");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "surface_end" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_surfaces(&mut self) {
            unsafe {
                let __class_name = StringName::from("ImmediateMesh");
                let __method_name = StringName::from("clear_surfaces");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ImmediateMesh" , "clear_surfaces" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for ImmediateMesh {
        type Base = crate::engine::Mesh;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "ImmediateMesh";
    }
    impl crate::obj::EngineClass for ImmediateMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Mesh> for ImmediateMesh {}
    impl crate::obj::Inherits<crate::engine::Resource> for ImmediateMesh {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for ImmediateMesh {}
    impl crate::obj::Inherits<crate::engine::Object> for ImmediateMesh {}
    impl std::ops::Deref for ImmediateMesh {
        type Target = crate::engine::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for ImmediateMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_ImmediateMesh {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::ImmediateMesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Mesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
