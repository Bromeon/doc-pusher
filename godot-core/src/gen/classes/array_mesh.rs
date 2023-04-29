#![doc = "Sidecar module for class [`ArrayMesh`][crate::engine::ArrayMesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ArrayMesh` enums](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `ArrayMesh.`\n\nInherits [`Mesh`][crate::engine::Mesh].\n\nRelated symbols:\n\n* [`ArrayMeshVirtual`][crate::engine::ArrayMeshVirtual]: virtual methods\n\n\nSee also [Godot docs for `ArrayMesh`](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ArrayMesh {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`ArrayMesh`][crate::engine::ArrayMesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ArrayMesh` methods](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ArrayMeshVirtual:
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
    impl ArrayMesh {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
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
        pub fn add_blend_shape(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("add_blend_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "add_blend_shape" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_shape_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("get_blend_shape_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "get_blend_shape_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_blend_shape_name(&self, index: i64) -> StringName {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("get_blend_shape_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    659327637i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "get_blend_shape_name" , 659327637i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_blend_shape_name(&mut self, index: i64, name: StringName) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("set_blend_shape_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3780747571i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "set_blend_shape_name" , 3780747571i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_blend_shapes(&mut self) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("clear_blend_shapes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "clear_blend_shapes" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_blend_shape_mode(&mut self, mode: mesh::BlendShapeMode) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("set_blend_shape_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    227983991i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "set_blend_shape_mode" , 227983991i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<mesh::BlendShapeMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_blend_shape_mode(&self) -> mesh::BlendShapeMode {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("get_blend_shape_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    836485024i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "get_blend_shape_mode" , 836485024i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <mesh::BlendShapeMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_surface_from_arrays(
            &mut self,
            primitive: mesh::PrimitiveType,
            arrays: VariantArray,
            blend_shapes: Array<VariantArray>,
            lods: Dictionary,
            flags: mesh::ArrayFormat,
        ) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("add_surface_from_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    172284304i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "add_surface_from_arrays" , 172284304i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <mesh::PrimitiveType as sys::GodotFfi>::sys_const(&primitive),
                    <VariantArray as sys::GodotFfi>::sys_const(&arrays),
                    <Array<VariantArray> as sys::GodotFfi>::sys_const(&blend_shapes),
                    <Dictionary as sys::GodotFfi>::sys_const(&lods),
                    <mesh::ArrayFormat as sys::GodotFfi>::sys_const(&flags),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_surfaces(&mut self) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("clear_surfaces");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "clear_surfaces" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_update_vertex_region(
            &mut self,
            surf_idx: i64,
            offset: i64,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_update_vertex_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3837166854i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_update_vertex_region" , 3837166854i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&surf_idx),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_update_attribute_region(
            &mut self,
            surf_idx: i64,
            offset: i64,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_update_attribute_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3837166854i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_update_attribute_region" , 3837166854i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&surf_idx),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_update_skin_region(
            &mut self,
            surf_idx: i64,
            offset: i64,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_update_skin_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3837166854i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_update_skin_region" , 3837166854i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&surf_idx),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_get_array_len(&self, surf_idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_get_array_len");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_get_array_len" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_get_array_index_len(&self, surf_idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_get_array_index_len");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_get_array_index_len" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_get_format(&self, surf_idx: i64) -> mesh::ArrayFormat {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_get_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3718287884i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_get_format" , 3718287884i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <mesh::ArrayFormat as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_get_primitive_type(&self, surf_idx: i64) -> mesh::PrimitiveType {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_get_primitive_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4141943888i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_get_primitive_type" , 4141943888i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <mesh::PrimitiveType as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_find_by_name(&self, name: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_find_by_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321353865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_find_by_name" , 1321353865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_set_name(&mut self, surf_idx: i64, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_set_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    501894301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_set_name" , 501894301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&surf_idx),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_get_name(&self, surf_idx: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("surface_get_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "surface_get_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn regen_normal_maps(&mut self) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("regen_normal_maps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "regen_normal_maps" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_unwrap(
            &mut self,
            transform: Transform3D,
            texel_size: f64,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("lightmap_unwrap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1476641071i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "lightmap_unwrap" , 1476641071i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                    <f64 as sys::GodotFfi>::sys_const(&texel_size),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_custom_aabb(&mut self, aabb: Aabb) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("set_custom_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    259215842i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "set_custom_aabb" , 259215842i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Aabb as sys::GodotFfi>::sys_const(&aabb)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_custom_aabb(&self) -> Aabb {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("get_custom_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1068685055i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "get_custom_aabb" , 1068685055i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_shadow_mesh(&mut self, mesh: Gd<ArrayMesh>) {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("set_shadow_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3377897901i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "set_shadow_mesh" , 3377897901i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<ArrayMesh> as AsArg>::as_arg_ptr(&mesh)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_shadow_mesh(&self) -> Option<Gd<ArrayMesh>> {
            unsafe {
                let __class_name = StringName::from("ArrayMesh");
                let __method_name = StringName::from("get_shadow_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3206942465i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ArrayMesh" , "get_shadow_mesh" , 3206942465i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<ArrayMesh>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for ArrayMesh {
        type Base = crate::engine::Mesh;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "ArrayMesh";
    }
    impl crate::obj::EngineClass for ArrayMesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Mesh> for ArrayMesh {}
    impl crate::obj::Inherits<crate::engine::Resource> for ArrayMesh {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for ArrayMesh {}
    impl crate::obj::Inherits<crate::engine::Object> for ArrayMesh {}
    impl std::ops::Deref for ArrayMesh {
        type Target = crate::engine::Mesh;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for ArrayMesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_ArrayMesh {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::ArrayMesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Mesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
