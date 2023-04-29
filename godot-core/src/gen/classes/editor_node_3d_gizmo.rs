#![doc = "Sidecar module for class [`EditorNode3DGizmo`][crate::engine::EditorNode3DGizmo].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmo` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorNode3DGizmo.`\n\nInherits [`Node3DGizmo`][crate::engine::Node3DGizmo].\n\nRelated symbols:\n\n* [`EditorNode3DGizmoVirtual`][crate::engine::EditorNode3DGizmoVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmo`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorNode3DGizmo {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorNode3DGizmo`][crate::engine::EditorNode3DGizmo].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorNode3DGizmo` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmo.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorNode3DGizmoVirtual:
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
        fn redraw(&mut self) {
            unimplemented!()
        }
        fn get_handle_name(&self, id: i64, secondary: bool) -> GodotString {
            unimplemented!()
        }
        fn is_handle_highlighted(&self, id: i64, secondary: bool) -> bool {
            unimplemented!()
        }
        fn get_handle_value(&self, id: i64, secondary: bool) -> Variant {
            unimplemented!()
        }
        fn set_handle(&mut self, id: i64, secondary: bool, camera: Gd<Camera3D>, point: Vector2) {
            unimplemented!()
        }
        fn commit_handle(&mut self, id: i64, secondary: bool, restore: Variant, cancel: bool) {
            unimplemented!()
        }
        fn subgizmos_intersect_ray(&self, camera: Gd<Camera3D>, point: Vector2) -> i64 {
            unimplemented!()
        }
        fn subgizmos_intersect_frustum(
            &self,
            camera: Gd<Camera3D>,
            frustum: Array<Plane>,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn set_subgizmo_transform(&mut self, id: i64, transform: Transform3D) {
            unimplemented!()
        }
        fn get_subgizmo_transform(&self, id: i64) -> Transform3D {
            unimplemented!()
        }
        fn commit_subgizmos(
            &mut self,
            ids: PackedInt32Array,
            restores: Array<Transform3D>,
            cancel: bool,
        ) {
            unimplemented!()
        }
    }
    impl EditorNode3DGizmo {
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
        pub fn add_lines(
            &mut self,
            lines: PackedVector3Array,
            material: Gd<Material>,
            billboard: bool,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("add_lines");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    302451090i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "add_lines" , 302451090i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&lines),
                    <Gd<Material> as AsArg>::as_arg_ptr(&material),
                    <bool as sys::GodotFfi>::sys_const(&billboard),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_mesh(
            &mut self,
            mesh: Gd<Mesh>,
            material: Gd<Material>,
            transform: Transform3D,
            skeleton: Gd<SkinReference>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("add_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1868867708i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "add_mesh" , 1868867708i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Mesh> as AsArg>::as_arg_ptr(&mesh),
                    <Gd<Material> as AsArg>::as_arg_ptr(&material),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                    <Gd<SkinReference> as AsArg>::as_arg_ptr(&skeleton),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_collision_segments(&mut self, segments: PackedVector3Array) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("add_collision_segments");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    334873810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "add_collision_segments" , 334873810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedVector3Array as sys::GodotFfi>::sys_const(&segments)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_collision_triangles(&mut self, triangles: Gd<TriangleMesh>) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("add_collision_triangles");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    54901064i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "add_collision_triangles" , 54901064i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<TriangleMesh> as AsArg>::as_arg_ptr(&triangles)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_unscaled_billboard(
            &mut self,
            material: Gd<Material>,
            default_scale: f64,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("add_unscaled_billboard");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3719733075i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "add_unscaled_billboard" , 3719733075i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Material> as AsArg>::as_arg_ptr(&material),
                    <f64 as sys::GodotFfi>::sys_const(&default_scale),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_handles(
            &mut self,
            handles: PackedVector3Array,
            material: Gd<Material>,
            ids: PackedInt32Array,
            billboard: bool,
            secondary: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("add_handles");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2254560097i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "add_handles" , 2254560097i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&handles),
                    <Gd<Material> as AsArg>::as_arg_ptr(&material),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&ids),
                    <bool as sys::GodotFfi>::sys_const(&billboard),
                    <bool as sys::GodotFfi>::sys_const(&secondary),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_node_3d(&mut self, node: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("set_node_3d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "set_node_3d" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&node)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_node_3d(&self) -> Option<Gd<Node3D>> {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("get_node_3d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    151077316i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "get_node_3d" , 151077316i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Node3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_plugin(&self) -> Option<Gd<EditorNode3DGizmoPlugin>> {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("get_plugin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4250544552i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "get_plugin" , 4250544552i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<EditorNode3DGizmoPlugin>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear(&mut self) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "clear" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_hidden(&mut self, hidden: bool) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("set_hidden");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "set_hidden" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&hidden)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_subgizmo_selected(&self, id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("is_subgizmo_selected");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "is_subgizmo_selected" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_subgizmo_selection(&self) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmo");
                let __method_name = StringName::from("get_subgizmo_selection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1930428628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmo" , "get_subgizmo_selection" , 1930428628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for EditorNode3DGizmo {
        type Base = crate::engine::Node3DGizmo;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "EditorNode3DGizmo";
    }
    impl crate::obj::EngineClass for EditorNode3DGizmo {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Node3DGizmo> for EditorNode3DGizmo {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for EditorNode3DGizmo {}
    impl crate::obj::Inherits<crate::engine::Object> for EditorNode3DGizmo {}
    impl std::ops::Deref for EditorNode3DGizmo {
        type Target = crate::engine::Node3DGizmo;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmo {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorNode3DGizmo {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorNode3DGizmo> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node3DGizmo> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
