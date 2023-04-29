#![doc = "Sidecar module for class [`SurfaceTool`][crate::engine::SurfaceTool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `SurfaceTool` enums](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `SurfaceTool.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`surface_tool`][crate::engine::surface_tool]: sidecar module with related enum/flag types\n* [`SurfaceToolVirtual`][crate::engine::SurfaceToolVirtual]: virtual methods\n\n\nSee also [Godot docs for `SurfaceTool`](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SurfaceTool {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`SurfaceTool`][crate::engine::SurfaceTool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `SurfaceTool` methods](https://docs.godotengine.org/en/stable/classes/class_surfacetool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait SurfaceToolVirtual:
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
    impl SurfaceTool {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
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
        pub fn set_skin_weight_count(&mut self, count: surface_tool::SkinWeightCount) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_skin_weight_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    618679515i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_skin_weight_count" , 618679515i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<surface_tool::SkinWeightCount as sys::GodotFfi>::sys_const(
                    &count,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_skin_weight_count(&self) -> surface_tool::SkinWeightCount {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("get_skin_weight_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1072401130i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "get_skin_weight_count" , 1072401130i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <surface_tool::SkinWeightCount as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_custom_format(
            &mut self,
            channel_index: i64,
            format: surface_tool::CustomFormat,
        ) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_custom_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4087759856i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_custom_format" , 4087759856i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&channel_index),
                    <surface_tool::CustomFormat as sys::GodotFfi>::sys_const(&format),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_custom_format(&self, channel_index: i64) -> surface_tool::CustomFormat {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("get_custom_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    839863283i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "get_custom_format" , 839863283i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&channel_index)];
                let __args_ptr = __args.as_ptr();
                <surface_tool::CustomFormat as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn begin(&mut self, primitive: mesh::PrimitiveType) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("begin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2230304113i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "begin" , 2230304113i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<mesh::PrimitiveType as sys::GodotFfi>::sys_const(
                    &primitive,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_vertex(&mut self, vertex: Vector3) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("add_vertex");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "add_vertex" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&vertex)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_normal(&mut self, normal: Vector3) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_normal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_normal" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&normal)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_tangent(&mut self, tangent: Plane) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_tangent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3505987427i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_tangent" , 3505987427i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Plane as sys::GodotFfi>::sys_const(&tangent)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_uv(&mut self, uv: Vector2) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_uv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_uv" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&uv)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_uv2(&mut self, uv2: Vector2) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_uv2");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_uv2" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&uv2)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_bones(&mut self, bones: PackedInt32Array) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_bones");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614634198i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_bones" , 3614634198i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedInt32Array as sys::GodotFfi>::sys_const(&bones)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_weights(&mut self, weights: PackedFloat32Array) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_weights");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2899603908i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_weights" , 2899603908i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedFloat32Array as sys::GodotFfi>::sys_const(&weights)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_custom(&mut self, channel_index: i64, custom_color: Color) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_custom");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2878471219i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_custom" , 2878471219i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&channel_index),
                    <Color as sys::GodotFfi>::sys_const(&custom_color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_smooth_group(&mut self, index: i64) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_smooth_group");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_smooth_group" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_triangle_fan(
            &mut self,
            vertices: PackedVector3Array,
            uvs: PackedVector2Array,
            colors: PackedColorArray,
            uv2s: PackedVector2Array,
            normals: PackedVector3Array,
            tangents: VariantArray,
        ) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("add_triangle_fan");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    297960074i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "add_triangle_fan" , 297960074i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&vertices),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uv2s),
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&normals),
                    <VariantArray as sys::GodotFfi>::sys_const(&tangents),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_index(&mut self, index: i64) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("add_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "add_index" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn index(&mut self) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "index" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn deindex(&mut self) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("deindex");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "deindex" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn generate_normals(&mut self, flip: bool) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("generate_normals");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    107499316i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "generate_normals" , 107499316i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&flip)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn generate_tangents(&mut self) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("generate_tangents");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "generate_tangents" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn optimize_indices_for_cache(&mut self) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("optimize_indices_for_cache");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "optimize_indices_for_cache" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_aabb(&self) -> Aabb {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("get_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1068685055i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "get_aabb" , 1068685055i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn generate_lod(
            &mut self,
            nd_threshold: f64,
            target_index_count: i64,
        ) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("generate_lod");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1894448909i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "generate_lod" , 1894448909i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&nd_threshold),
                    <i64 as sys::GodotFfi>::sys_const(&target_index_count),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_material(&mut self, material: Gd<Material>) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2757459619i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "set_material" , 2757459619i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Material> as AsArg>::as_arg_ptr(&material)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_primitive_type(&self) -> mesh::PrimitiveType {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("get_primitive_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    768822145i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "get_primitive_type" , 768822145i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <mesh::PrimitiveType as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear(&mut self) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "clear" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_from(&mut self, existing: Gd<Mesh>, surface: i64) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("create_from");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1767024570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "create_from" , 1767024570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Mesh> as AsArg>::as_arg_ptr(&existing),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_from_blend_shape(
            &mut self,
            existing: Gd<Mesh>,
            surface: i64,
            blend_shape: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("create_from_blend_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1306185582i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "create_from_blend_shape" , 1306185582i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Mesh> as AsArg>::as_arg_ptr(&existing),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <GodotString as sys::GodotFfi>::sys_const(&blend_shape),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn append_from(&mut self, existing: Gd<Mesh>, surface: i64, transform: Transform3D) {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("append_from");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2217967155i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "append_from" , 2217967155i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Mesh> as AsArg>::as_arg_ptr(&existing),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn commit(&mut self, existing: Gd<ArrayMesh>, flags: i64) -> Option<Gd<ArrayMesh>> {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("commit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4107864055i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "commit" , 4107864055i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<ArrayMesh> as AsArg>::as_arg_ptr(&existing),
                    <i64 as sys::GodotFfi>::sys_const(&flags),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<ArrayMesh>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn commit_to_arrays(&mut self) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("SurfaceTool");
                let __method_name = StringName::from("commit_to_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "SurfaceTool" , "commit_to_arrays" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for SurfaceTool {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "SurfaceTool";
    }
    impl crate::obj::EngineClass for SurfaceTool {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for SurfaceTool {}
    impl crate::obj::Inherits<crate::engine::Object> for SurfaceTool {}
    impl std::ops::Deref for SurfaceTool {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for SurfaceTool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_SurfaceTool {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::SurfaceTool> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CustomFormat {
    ord: i32,
}
impl CustomFormat {
    pub const CUSTOM_RGBA8_UNORM: Self = Self { ord: 0 };
    pub const CUSTOM_RGBA8_SNORM: Self = Self { ord: 1 };
    pub const CUSTOM_RG_HALF: Self = Self { ord: 2 };
    pub const CUSTOM_RGBA_HALF: Self = Self { ord: 3 };
    pub const CUSTOM_R_FLOAT: Self = Self { ord: 4 };
    pub const CUSTOM_RG_FLOAT: Self = Self { ord: 5 };
    pub const CUSTOM_RGB_FLOAT: Self = Self { ord: 6 };
    pub const CUSTOM_RGBA_FLOAT: Self = Self { ord: 7 };
    pub const CUSTOM_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for CustomFormat {
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
unsafe impl sys::GodotFfi for CustomFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SkinWeightCount {
    ord: i32,
}
impl SkinWeightCount {
    pub const SKIN_4_WEIGHTS: Self = Self { ord: 0 };
    pub const SKIN_8_WEIGHTS: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for SkinWeightCount {
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
unsafe impl sys::GodotFfi for SkinWeightCount {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
