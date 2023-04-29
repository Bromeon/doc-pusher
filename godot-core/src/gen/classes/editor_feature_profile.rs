#![doc = "Sidecar module for class [`EditorFeatureProfile`][crate::engine::EditorFeatureProfile].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorFeatureProfile` enums](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorFeatureProfile.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`editor_feature_profile`][crate::engine::editor_feature_profile]: sidecar module with related enum/flag types\n* [`EditorFeatureProfileVirtual`][crate::engine::EditorFeatureProfileVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorFeatureProfile`](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorFeatureProfile {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorFeatureProfile`][crate::engine::EditorFeatureProfile].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorFeatureProfile` methods](https://docs.godotengine.org/en/stable/classes/class_editorfeatureprofile.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorFeatureProfileVirtual:
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
    impl EditorFeatureProfile {
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
        pub fn set_disable_class(&mut self, class_name: StringName, disable: bool) {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("set_disable_class");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2524380260i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "set_disable_class" , 2524380260i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&class_name),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_class_disabled(&self, class_name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("is_class_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "is_class_disabled" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&class_name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_disable_class_editor(&mut self, class_name: StringName, disable: bool) {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("set_disable_class_editor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2524380260i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "set_disable_class_editor" , 2524380260i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&class_name),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_class_editor_disabled(&self, class_name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("is_class_editor_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "is_class_editor_disabled" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&class_name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_disable_class_property(
            &mut self,
            class_name: StringName,
            property: StringName,
            disable: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("set_disable_class_property");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    865197084i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "set_disable_class_property" , 865197084i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&class_name),
                    <StringName as sys::GodotFfi>::sys_const(&property),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_class_property_disabled(
            &self,
            class_name: StringName,
            property: StringName,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("is_class_property_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    471820014i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "is_class_property_disabled" , 471820014i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&class_name),
                    <StringName as sys::GodotFfi>::sys_const(&property),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_disable_feature(
            &mut self,
            feature: editor_feature_profile::Feature,
            disable: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("set_disable_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1884871044i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "set_disable_feature" , 1884871044i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <editor_feature_profile::Feature as sys::GodotFfi>::sys_const(&feature),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_feature_disabled(&self, feature: editor_feature_profile::Feature) -> bool {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("is_feature_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2974403161i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "is_feature_disabled" , 2974403161i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<editor_feature_profile::Feature as sys::GodotFfi>::sys_const(&feature)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_feature_name(
            &mut self,
            feature: editor_feature_profile::Feature,
        ) -> GodotString {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("get_feature_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3401335809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "get_feature_name" , 3401335809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<editor_feature_profile::Feature as sys::GodotFfi>::sys_const(&feature)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn save_to_file(&mut self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("save_to_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166001499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "save_to_file" , 166001499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn load_from_file(&mut self, path: GodotString) -> global::Error {
            unsafe {
                let __class_name = StringName::from("EditorFeatureProfile");
                let __method_name = StringName::from("load_from_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166001499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorFeatureProfile" , "load_from_file" , 166001499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for EditorFeatureProfile {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "EditorFeatureProfile";
    }
    impl crate::obj::EngineClass for EditorFeatureProfile {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for EditorFeatureProfile {}
    impl crate::obj::Inherits<crate::engine::Object> for EditorFeatureProfile {}
    impl std::ops::Deref for EditorFeatureProfile {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorFeatureProfile {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorFeatureProfile {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorFeatureProfile> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Feature {
    ord: i32,
}
impl Feature {
    pub const FEATURE_3D: Self = Self { ord: 0 };
    pub const FEATURE_SCRIPT: Self = Self { ord: 1 };
    pub const FEATURE_ASSET_LIB: Self = Self { ord: 2 };
    pub const FEATURE_SCENE_TREE: Self = Self { ord: 3 };
    pub const FEATURE_NODE_DOCK: Self = Self { ord: 4 };
    pub const FEATURE_FILESYSTEM_DOCK: Self = Self { ord: 5 };
    pub const FEATURE_IMPORT_DOCK: Self = Self { ord: 6 };
    pub const FEATURE_HISTORY_DOCK: Self = Self { ord: 7 };
    pub const FEATURE_MAX: Self = Self { ord: 8 };
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
            | ord @ 8i32 => Some(Self { ord }),
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
