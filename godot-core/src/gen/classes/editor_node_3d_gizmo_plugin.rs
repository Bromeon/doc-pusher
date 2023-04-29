#![doc = "Sidecar module for class [`EditorNode3DGizmoPlugin`][crate::engine::EditorNode3DGizmoPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorNode3DGizmoPlugin.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`EditorNode3DGizmoPluginVirtual`][crate::engine::EditorNode3DGizmoPluginVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin`](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorNode3DGizmoPlugin {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorNode3DGizmoPlugin`][crate::engine::EditorNode3DGizmoPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorNode3DGizmoPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editornode3dgizmoplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorNode3DGizmoPluginVirtual:
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
        fn has_gizmo(&self, for_node_3d: Gd<Node3D>) -> bool {
            unimplemented!()
        }
        fn create_gizmo(&self, for_node_3d: Gd<Node3D>) -> Option<Gd<EditorNode3DGizmo>> {
            unimplemented!()
        }
        fn get_gizmo_name(&self) -> GodotString {
            unimplemented!()
        }
        fn get_priority(&self) -> i64 {
            unimplemented!()
        }
        fn can_be_hidden(&self) -> bool {
            unimplemented!()
        }
        fn is_selectable_when_hidden(&self) -> bool {
            unimplemented!()
        }
        fn redraw(&mut self, gizmo: Gd<EditorNode3DGizmo>) {
            unimplemented!()
        }
        fn get_handle_name(
            &self,
            gizmo: Gd<EditorNode3DGizmo>,
            handle_id: i64,
            secondary: bool,
        ) -> GodotString {
            unimplemented!()
        }
        fn is_handle_highlighted(
            &self,
            gizmo: Gd<EditorNode3DGizmo>,
            handle_id: i64,
            secondary: bool,
        ) -> bool {
            unimplemented!()
        }
        fn get_handle_value(
            &self,
            gizmo: Gd<EditorNode3DGizmo>,
            handle_id: i64,
            secondary: bool,
        ) -> Variant {
            unimplemented!()
        }
        fn set_handle(
            &mut self,
            gizmo: Gd<EditorNode3DGizmo>,
            handle_id: i64,
            secondary: bool,
            camera: Gd<Camera3D>,
            screen_pos: Vector2,
        ) {
            unimplemented!()
        }
        fn commit_handle(
            &mut self,
            gizmo: Gd<EditorNode3DGizmo>,
            handle_id: i64,
            secondary: bool,
            restore: Variant,
            cancel: bool,
        ) {
            unimplemented!()
        }
        fn subgizmos_intersect_ray(
            &self,
            gizmo: Gd<EditorNode3DGizmo>,
            camera: Gd<Camera3D>,
            screen_pos: Vector2,
        ) -> i64 {
            unimplemented!()
        }
        fn subgizmos_intersect_frustum(
            &self,
            gizmo: Gd<EditorNode3DGizmo>,
            camera: Gd<Camera3D>,
            frustum_planes: Array<Plane>,
        ) -> PackedInt32Array {
            unimplemented!()
        }
        fn get_subgizmo_transform(
            &self,
            gizmo: Gd<EditorNode3DGizmo>,
            subgizmo_id: i64,
        ) -> Transform3D {
            unimplemented!()
        }
        fn set_subgizmo_transform(
            &mut self,
            gizmo: Gd<EditorNode3DGizmo>,
            subgizmo_id: i64,
            transform: Transform3D,
        ) {
            unimplemented!()
        }
        fn commit_subgizmos(
            &mut self,
            gizmo: Gd<EditorNode3DGizmo>,
            ids: PackedInt32Array,
            restores: Array<Transform3D>,
            cancel: bool,
        ) {
            unimplemented!()
        }
    }
    impl EditorNode3DGizmoPlugin {
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
        pub fn create_material(
            &mut self,
            name: GodotString,
            color: Color,
            billboard: bool,
            on_top: bool,
            use_vertex_color: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmoPlugin");
                let __method_name = StringName::from("create_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3486012546i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmoPlugin" , "create_material" , 3486012546i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <bool as sys::GodotFfi>::sys_const(&billboard),
                    <bool as sys::GodotFfi>::sys_const(&on_top),
                    <bool as sys::GodotFfi>::sys_const(&use_vertex_color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_icon_material(
            &mut self,
            name: GodotString,
            texture: Gd<Texture2D>,
            on_top: bool,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmoPlugin");
                let __method_name = StringName::from("create_icon_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2976007329i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmoPlugin" , "create_icon_material" , 2976007329i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <bool as sys::GodotFfi>::sys_const(&on_top),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_handle_material(
            &mut self,
            name: GodotString,
            billboard: bool,
            texture: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmoPlugin");
                let __method_name = StringName::from("create_handle_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2486475223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmoPlugin" , "create_handle_material" , 2486475223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <bool as sys::GodotFfi>::sys_const(&billboard),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_material(&mut self, name: GodotString, material: Gd<StandardMaterial3D>) {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmoPlugin");
                let __method_name = StringName::from("add_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1374068695i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmoPlugin" , "add_material" , 1374068695i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Gd<StandardMaterial3D> as AsArg>::as_arg_ptr(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_material(
            &mut self,
            name: GodotString,
            gizmo: Gd<EditorNode3DGizmo>,
        ) -> Option<Gd<StandardMaterial3D>> {
            unsafe {
                let __class_name = StringName::from("EditorNode3DGizmoPlugin");
                let __method_name = StringName::from("get_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3501703615i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorNode3DGizmoPlugin" , "get_material" , 3501703615i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Gd<EditorNode3DGizmo> as AsArg>::as_arg_ptr(&gizmo),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<StandardMaterial3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for EditorNode3DGizmoPlugin {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "EditorNode3DGizmoPlugin";
    }
    impl crate::obj::EngineClass for EditorNode3DGizmoPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for EditorNode3DGizmoPlugin {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for EditorNode3DGizmoPlugin {}
    impl crate::obj::Inherits<crate::engine::Object> for EditorNode3DGizmoPlugin {}
    impl std::ops::Deref for EditorNode3DGizmoPlugin {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorNode3DGizmoPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorNode3DGizmoPlugin {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorNode3DGizmoPlugin> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
