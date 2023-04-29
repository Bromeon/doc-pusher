#![doc = "Sidecar module for class [`EditorImportPlugin`][crate::engine::EditorImportPlugin].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorImportPlugin` enums](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorImportPlugin.`\n\nInherits [`ResourceImporter`][crate::engine::ResourceImporter].\n\nRelated symbols:\n\n* [`EditorImportPluginVirtual`][crate::engine::EditorImportPluginVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorImportPlugin`](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorImportPlugin {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorImportPlugin`][crate::engine::EditorImportPlugin].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorImportPlugin` methods](https://docs.godotengine.org/en/stable/classes/class_editorimportplugin.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorImportPluginVirtual:
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
        fn get_importer_name(&self) -> GodotString {
            unimplemented!()
        }
        fn get_visible_name(&self) -> GodotString {
            unimplemented!()
        }
        fn get_preset_count(&self) -> i64 {
            unimplemented!()
        }
        fn get_preset_name(&self, preset_index: i64) -> GodotString {
            unimplemented!()
        }
        fn get_recognized_extensions(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn get_import_options(&self, path: GodotString, preset_index: i64) -> Array<Dictionary> {
            unimplemented!()
        }
        fn get_save_extension(&self) -> GodotString {
            unimplemented!()
        }
        fn get_resource_type(&self) -> GodotString {
            unimplemented!()
        }
        fn get_priority(&self) -> f64 {
            unimplemented!()
        }
        fn get_import_order(&self) -> i64 {
            unimplemented!()
        }
        fn get_option_visibility(
            &self,
            path: GodotString,
            option_name: StringName,
            options: Dictionary,
        ) -> bool {
            unimplemented!()
        }
        fn import(
            &self,
            source_file: GodotString,
            save_path: GodotString,
            options: Dictionary,
            platform_variants: Array<GodotString>,
            gen_files: Array<GodotString>,
        ) -> global::Error {
            unimplemented!()
        }
    }
    impl EditorImportPlugin {
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
        pub fn append_import_external_resource(
            &mut self,
            path: GodotString,
            custom_options: Dictionary,
            custom_importer: GodotString,
            generator_parameters: Variant,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("EditorImportPlugin");
                let __method_name = StringName::from("append_import_external_resource");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3645925746i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorImportPlugin" , "append_import_external_resource" , 3645925746i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                    <Dictionary as sys::GodotFfi>::sys_const(&custom_options),
                    <GodotString as sys::GodotFfi>::sys_const(&custom_importer),
                    <Variant as sys::GodotFfi>::sys_const(&generator_parameters),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for EditorImportPlugin {
        type Base = crate::engine::ResourceImporter;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "EditorImportPlugin";
    }
    impl crate::obj::EngineClass for EditorImportPlugin {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::ResourceImporter> for EditorImportPlugin {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for EditorImportPlugin {}
    impl crate::obj::Inherits<crate::engine::Object> for EditorImportPlugin {}
    impl std::ops::Deref for EditorImportPlugin {
        type Target = crate::engine::ResourceImporter;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorImportPlugin {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorImportPlugin {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorImportPlugin> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::ResourceImporter> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
