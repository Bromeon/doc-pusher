#![doc = "Sidecar module for class [`Mesh`][crate::engine::Mesh].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Mesh` enums](https://docs.godotengine.org/en/stable/classes/class_mesh.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Mesh.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`mesh`][crate::engine::mesh]: sidecar module with related enum/flag types\n* [`MeshVirtual`][crate::engine::MeshVirtual]: virtual methods\n\n\nSee also [Godot docs for `Mesh`](https://docs.godotengine.org/en/stable/classes/class_mesh.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Mesh {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Mesh`][crate::engine::Mesh].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Mesh` methods](https://docs.godotengine.org/en/stable/classes/class_mesh.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait MeshVirtual:
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
    impl Mesh {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Mesh");
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
        pub fn set_lightmap_size_hint(&mut self, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("set_lightmap_size_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "set_lightmap_size_hint" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_lightmap_size_hint(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("get_lightmap_size_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "get_lightmap_size_hint" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_aabb(&self) -> Aabb {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("get_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1068685055i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "get_aabb" , 1068685055i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_surface_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("get_surface_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "get_surface_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_get_arrays(&self, surf_idx: i64) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("surface_get_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    663333327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "surface_get_arrays" , 663333327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_get_blend_shape_arrays(&self, surf_idx: i64) -> Array<VariantArray> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("surface_get_blend_shape_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    663333327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "surface_get_blend_shape_arrays" , 663333327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <Array<VariantArray> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn surface_set_material(&mut self, surf_idx: i64, material: Gd<Material>) {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("surface_set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3671737478i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "surface_set_material" , 3671737478i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&surf_idx),
                    <Gd<Material> as AsArg>::as_arg_ptr(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn surface_get_material(&self, surf_idx: i64) -> Option<Gd<Material>> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("surface_get_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2897466400i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "surface_get_material" , 2897466400i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&surf_idx)];
                let __args_ptr = __args.as_ptr();
                <Gd<Material>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_placeholder(&self) -> Option<Gd<Resource>> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("create_placeholder");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    121922552i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "create_placeholder" , 121922552i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Resource>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_trimesh_shape(&self) -> Option<Gd<ConcavePolygonShape3D>> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("create_trimesh_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4160111210i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "create_trimesh_shape" , 4160111210i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<ConcavePolygonShape3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_convex_shape(
            &self,
            clean: bool,
            simplify: bool,
        ) -> Option<Gd<ConvexPolygonShape3D>> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("create_convex_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2529984628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "create_convex_shape" , 2529984628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <bool as sys::GodotFfi>::sys_const(&clean),
                    <bool as sys::GodotFfi>::sys_const(&simplify),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<ConvexPolygonShape3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_outline(&self, margin: f64) -> Option<Gd<Mesh>> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("create_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1208642001i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "create_outline" , 1208642001i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&margin)];
                let __args_ptr = __args.as_ptr();
                <Gd<Mesh>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_faces(&self) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("get_faces");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    497664490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "get_faces" , 497664490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn generate_triangle_mesh(&self) -> Option<Gd<TriangleMesh>> {
            unsafe {
                let __class_name = StringName::from("Mesh");
                let __method_name = StringName::from("generate_triangle_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3476533166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Mesh" , "generate_triangle_mesh" , 3476533166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<TriangleMesh>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Mesh {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Mesh";
    }
    impl crate::obj::EngineClass for Mesh {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Mesh {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Mesh {}
    impl crate::obj::Inherits<crate::engine::Object> for Mesh {}
    impl std::ops::Deref for Mesh {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Mesh {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Mesh {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Mesh> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PrimitiveType {
    ord: i32,
}
impl PrimitiveType {
    pub const PRIMITIVE_POINTS: Self = Self { ord: 0 };
    pub const PRIMITIVE_LINES: Self = Self { ord: 1 };
    pub const PRIMITIVE_LINE_STRIP: Self = Self { ord: 2 };
    pub const PRIMITIVE_TRIANGLES: Self = Self { ord: 3 };
    pub const PRIMITIVE_TRIANGLE_STRIP: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for PrimitiveType {
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
unsafe impl sys::GodotFfi for PrimitiveType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ArrayType {
    ord: i32,
}
impl ArrayType {
    pub const ARRAY_VERTEX: Self = Self { ord: 0 };
    pub const ARRAY_NORMAL: Self = Self { ord: 1 };
    pub const ARRAY_TANGENT: Self = Self { ord: 2 };
    pub const ARRAY_COLOR: Self = Self { ord: 3 };
    pub const ARRAY_TEX_UV: Self = Self { ord: 4 };
    pub const ARRAY_TEX_UV2: Self = Self { ord: 5 };
    pub const ARRAY_CUSTOM0: Self = Self { ord: 6 };
    pub const ARRAY_CUSTOM1: Self = Self { ord: 7 };
    pub const ARRAY_CUSTOM2: Self = Self { ord: 8 };
    pub const ARRAY_CUSTOM3: Self = Self { ord: 9 };
    pub const ARRAY_BONES: Self = Self { ord: 10 };
    pub const ARRAY_WEIGHTS: Self = Self { ord: 11 };
    pub const ARRAY_INDEX: Self = Self { ord: 12 };
    pub const ARRAY_MAX: Self = Self { ord: 13 };
}
impl crate::obj::EngineEnum for ArrayType {
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
            | ord @ 13i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ArrayType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ArrayCustomFormat {
    ord: i32,
}
impl ArrayCustomFormat {
    pub const ARRAY_CUSTOM_RGBA8_UNORM: Self = Self { ord: 0 };
    pub const ARRAY_CUSTOM_RGBA8_SNORM: Self = Self { ord: 1 };
    pub const ARRAY_CUSTOM_RG_HALF: Self = Self { ord: 2 };
    pub const ARRAY_CUSTOM_RGBA_HALF: Self = Self { ord: 3 };
    pub const ARRAY_CUSTOM_R_FLOAT: Self = Self { ord: 4 };
    pub const ARRAY_CUSTOM_RG_FLOAT: Self = Self { ord: 5 };
    pub const ARRAY_CUSTOM_RGB_FLOAT: Self = Self { ord: 6 };
    pub const ARRAY_CUSTOM_RGBA_FLOAT: Self = Self { ord: 7 };
    pub const ARRAY_CUSTOM_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
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
            | ord @ 8i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ArrayCustomFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct ArrayFormat {
    ord: i32,
}
impl ArrayFormat {
    pub const ARRAY_FORMAT_VERTEX: Self = Self { ord: 1 };
    pub const ARRAY_FORMAT_NORMAL: Self = Self { ord: 2 };
    pub const ARRAY_FORMAT_TANGENT: Self = Self { ord: 4 };
    pub const ARRAY_FORMAT_COLOR: Self = Self { ord: 8 };
    pub const ARRAY_FORMAT_TEX_UV: Self = Self { ord: 16 };
    pub const ARRAY_FORMAT_TEX_UV2: Self = Self { ord: 32 };
    pub const ARRAY_FORMAT_CUSTOM0: Self = Self { ord: 64 };
    pub const ARRAY_FORMAT_CUSTOM1: Self = Self { ord: 128 };
    pub const ARRAY_FORMAT_CUSTOM2: Self = Self { ord: 256 };
    pub const ARRAY_FORMAT_CUSTOM3: Self = Self { ord: 512 };
    pub const ARRAY_FORMAT_BONES: Self = Self { ord: 1024 };
    pub const ARRAY_FORMAT_WEIGHTS: Self = Self { ord: 2048 };
    pub const ARRAY_FORMAT_INDEX: Self = Self { ord: 4096 };
    pub const ARRAY_FORMAT_BLEND_SHAPE_MASK: Self = Self { ord: 7 };
    pub const ARRAY_FORMAT_CUSTOM_BASE: Self = Self { ord: 13 };
    pub const ARRAY_FORMAT_CUSTOM_BITS: Self = Self { ord: 3 };
    pub const ARRAY_FORMAT_CUSTOM0_SHIFT: Self = Self { ord: 13 };
    pub const ARRAY_FORMAT_CUSTOM1_SHIFT: Self = Self { ord: 16 };
    pub const ARRAY_FORMAT_CUSTOM2_SHIFT: Self = Self { ord: 19 };
    pub const ARRAY_FORMAT_CUSTOM3_SHIFT: Self = Self { ord: 22 };
    pub const ARRAY_FORMAT_CUSTOM_MASK: Self = Self { ord: 7 };
    pub const ARRAY_COMPRESS_FLAGS_BASE: Self = Self { ord: 25 };
    pub const ARRAY_FLAG_USE_2D_VERTICES: Self = Self { ord: 33554432 };
    pub const ARRAY_FLAG_USE_DYNAMIC_UPDATE: Self = Self { ord: 67108864 };
    pub const ARRAY_FLAG_USE_8_BONE_WEIGHTS: Self = Self { ord: 134217728 };
    pub const ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY: Self = Self { ord: 268435456 };
}
impl crate::obj::EngineEnum for ArrayFormat {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 7i32
            | ord @ 8i32
            | ord @ 13i32
            | ord @ 16i32
            | ord @ 19i32
            | ord @ 22i32
            | ord @ 25i32
            | ord @ 32i32
            | ord @ 64i32
            | ord @ 128i32
            | ord @ 256i32
            | ord @ 512i32
            | ord @ 1024i32
            | ord @ 2048i32
            | ord @ 4096i32
            | ord @ 33554432i32
            | ord @ 67108864i32
            | ord @ 134217728i32
            | ord @ 268435456i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ArrayFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlendShapeMode {
    ord: i32,
}
impl BlendShapeMode {
    pub const BLEND_SHAPE_MODE_NORMALIZED: Self = Self { ord: 0 };
    pub const BLEND_SHAPE_MODE_RELATIVE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for BlendShapeMode {
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
unsafe impl sys::GodotFfi for BlendShapeMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
