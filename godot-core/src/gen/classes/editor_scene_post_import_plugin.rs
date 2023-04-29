#![doc = "Sidecar module for class [`EditorScenePostImportPlugin`][crate::engine::EditorScenePostImportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorScenePostImportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorscenepostimportplugin.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorScenePostImportPlugin.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`editor_scene_post_import_plugin`][crate::engine::editor_scene_post_import_plugin]: sidecar module with related enum/flag types\n* [`EditorScenePostImportPluginVirtual`][crate::engine::EditorScenePostImportPluginVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorScenePostImportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorscenepostimportplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorScenePostImportPlugin {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorScenePostImportPlugin`][crate::engine::EditorScenePostImportPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorScenePostImportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorscenepostimportplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorScenePostImportPluginVirtual:
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
        fn get_internal_import_options(&mut self, category: i64) {
            unimplemented!()
        }
        fn get_internal_option_visibility(
            &self,
            category: i64,
            for_animation: bool,
            option: GodotString,
        ) -> Variant {
            unimplemented!()
        }
        fn get_internal_option_update_view_required(
            &self,
            category: i64,
            option: GodotString,
        ) -> Variant {
            unimplemented!()
        }
        fn internal_process(
            &mut self,
            category: i64,
            base_node: Gd<Node>,
            node: Gd<Node>,
            resource: Gd<Resource>,
        ) {
            unimplemented!()
        }
        fn get_import_options(&mut self, path: GodotString) {
            unimplemented!()
        }
        fn get_option_visibility(
            &self,
            path: GodotString,
            for_animation: bool,
            option: GodotString,
        ) -> Variant {
            unimplemented!()
        }
        fn pre_process(&mut self, scene: Gd<Node>) {
            unimplemented!()
        }
        fn post_process(&mut self, scene: Gd<Node>) {
            unimplemented!()
        }
    }
    impl EditorScenePostImportPlugin {
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
        pub fn get_option_value(&self, name: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("EditorScenePostImportPlugin");
                let __method_name = StringName::from("get_option_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2760726917i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorScenePostImportPlugin" , "get_option_value" , 2760726917i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_import_option(&mut self, name: GodotString, value: Variant) {
            unsafe {
                let __class_name = StringName::from("EditorScenePostImportPlugin");
                let __method_name = StringName::from("add_import_option");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    402577236i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorScenePostImportPlugin" , "add_import_option" , 402577236i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_import_option_advanced(
            &mut self,
            type_: VariantType,
            name: GodotString,
            default_value: Variant,
            hint: global::PropertyHint,
            hint_string: GodotString,
            usage_flags: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorScenePostImportPlugin");
                let __method_name = StringName::from("add_import_option_advanced");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3774155785i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorScenePostImportPlugin" , "add_import_option_advanced" , 3774155785i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <VariantType as sys::GodotFfi>::sys_const(&type_),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Variant as sys::GodotFfi>::sys_const(&default_value),
                    <global::PropertyHint as sys::GodotFfi>::sys_const(&hint),
                    <GodotString as sys::GodotFfi>::sys_const(&hint_string),
                    <i64 as sys::GodotFfi>::sys_const(&usage_flags),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for EditorScenePostImportPlugin {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "EditorScenePostImportPlugin";
    }
    impl crate::obj::EngineClass for EditorScenePostImportPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for EditorScenePostImportPlugin {}
    impl crate::obj::Inherits<crate::engine::Object> for EditorScenePostImportPlugin {}
    impl std::ops::Deref for EditorScenePostImportPlugin {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorScenePostImportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorScenePostImportPlugin {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorScenePostImportPlugin> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct InternalImportCategory {
    ord: i32,
}
impl InternalImportCategory {
    pub const INTERNAL_IMPORT_CATEGORY_NODE: Self = Self { ord: 0 };
    pub const INTERNAL_IMPORT_CATEGORY_MESH_3D_NODE: Self = Self { ord: 1 };
    pub const INTERNAL_IMPORT_CATEGORY_MESH: Self = Self { ord: 2 };
    pub const INTERNAL_IMPORT_CATEGORY_MATERIAL: Self = Self { ord: 3 };
    pub const INTERNAL_IMPORT_CATEGORY_ANIMATION: Self = Self { ord: 4 };
    pub const INTERNAL_IMPORT_CATEGORY_ANIMATION_NODE: Self = Self { ord: 5 };
    pub const INTERNAL_IMPORT_CATEGORY_SKELETON_3D_NODE: Self = Self { ord: 6 };
    pub const INTERNAL_IMPORT_CATEGORY_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for InternalImportCategory {
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
unsafe impl sys::GodotFfi for InternalImportCategory {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
