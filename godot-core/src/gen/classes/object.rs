#![doc = "Sidecar module for class [`Object`][crate::engine::Object].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Object` enums](https://docs.godotengine.org/en/stable/classes/class_object.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Object.`\n\nThis is the base class for all other classes at the root of the hierarchy. Every instance of `Object` can be stored in a [`Gd`][crate::obj::Gd] smart pointer.\n\nRelated symbols:\n\n* [`object`][crate::engine::object]: sidecar module with related enum/flag types\n* [`ObjectVirtual`][crate::engine::ObjectVirtual]: virtual methods\n* [`ObjectNotification`][crate::engine::notify::ObjectNotification]: notification type\n\n\nSee also [Godot docs for `Object`](https://docs.godotengine.org/en/stable/classes/class_object.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Object {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Object`][crate::engine::Object].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Object` methods](https://docs.godotengine.org/en/stable/classes/class_object.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ObjectVirtual:
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
    #[doc = "Notification type for class [`Object`][crate::engine::Object]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum ObjectNotification {
        Postinitialize = 0i32,
        Predelete = 1i32,
        #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object.notification()`."]
        Unknown(i32),
    }
    impl From<i32> for ObjectNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                0i32 => Self::Postinitialize,
                1i32 => Self::Predelete,
                other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From<ObjectNotification> for i32 {
        fn from(notification: ObjectNotification) -> i32 {
            match notification {
                ObjectNotification::Postinitialize => 0i32,
                ObjectNotification::Predelete => 1i32,
                ObjectNotification::Unknown(int) => int,
            }
        }
    }
    impl Object {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Object");
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
        pub fn get_class(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_class");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_class" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_class(&self, class: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("is_class");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "is_class" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&class)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set(&mut self, property: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set" , 3776071444i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get(&self, property: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2760726917i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get" , 2760726917i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&property)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_indexed(&mut self, property_path: NodePath, value: Variant) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set_indexed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3500910842i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set_indexed" , 3500910842i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <NodePath as sys::GodotFfi>::sys_const(&property_path),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_indexed(&self, property_path: NodePath) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_indexed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4006125091i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_indexed" , 4006125091i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&property_path)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_property_list(&self) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_property_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_property_list" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_method_list(&self) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_method_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_method_list" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn property_can_revert(&self, property: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("property_can_revert");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "property_can_revert" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&property)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn property_get_revert(&self, property: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("property_get_revert");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2760726917i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "property_get_revert" , 2760726917i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&property)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub(crate) fn notification(&mut self, what: i64, reversed: bool) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("notification");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4023243586i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "notification" , 4023243586i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&what),
                    <bool as sys::GodotFfi>::sys_const(&reversed),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub(crate) fn to_string(&mut self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("to_string");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2841200299i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "to_string" , 2841200299i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_script(&mut self, script: Variant) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set_script");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1114965689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set_script" , 1114965689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Variant as sys::GodotFfi>::sys_const(&script)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_script(&self) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_script");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1214101251i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_script" , 1214101251i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_meta(&mut self, name: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set_meta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set_meta" , 3776071444i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_meta(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("remove_meta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "remove_meta" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_meta(&self, name: StringName, default: Variant) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_meta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3990617847i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_meta" , 3990617847i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Variant as sys::GodotFfi>::sys_const(&default),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_meta(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("has_meta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "has_meta" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_meta_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_meta_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_meta_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_user_signal(&mut self, signal: GodotString, arguments: VariantArray) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("add_user_signal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3780025912i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "add_user_signal" , 3780025912i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&signal),
                    <VariantArray as sys::GodotFfi>::sys_const(&arguments),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_user_signal(&self, signal: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("has_user_signal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "has_user_signal" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&signal)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn emit_signal(&mut self, signal: StringName, varargs: &[Variant]) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("emit_signal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4047867050i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "emit_signal" , 4047867050i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [<StringName as ToVariant>::to_variant(&signal)];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let variant = Variant::from_var_sys_init(|return_ptr| {
                    let mut __err = sys::default_call_error();
                    __call_fn(
                        __method_bind,
                        self.object_ptr,
                        __args_ptr,
                        __args.len() as i64,
                        return_ptr,
                        std::ptr::addr_of_mut!(__err),
                    );
                    if __err.error != sys::GDEXTENSION_CALL_OK {
                        let mut __arg_types =
                            Vec::with_capacity(__explicit_args.len() + varargs.len());
                        __arg_types.extend(varargs.iter().map(Variant::get_type));
                        let __vararg_str = varargs
                            .iter()
                            .map(|v| format!("{v}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        sys::panic_call_error(
                            &__err,
                            &format!("emit_signal({signal:?}; {__vararg_str})"),
                            &__arg_types,
                        );
                    }
                });
                variant.to()
            }
        }
        pub fn call(&mut self, method: StringName, varargs: &[Variant]) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("call");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3400424181i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "call" , 3400424181i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [<StringName as ToVariant>::to_variant(&method)];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let variant = Variant::from_var_sys_init(|return_ptr| {
                    let mut __err = sys::default_call_error();
                    __call_fn(
                        __method_bind,
                        self.object_ptr,
                        __args_ptr,
                        __args.len() as i64,
                        return_ptr,
                        std::ptr::addr_of_mut!(__err),
                    );
                    if __err.error != sys::GDEXTENSION_CALL_OK {
                        let mut __arg_types =
                            Vec::with_capacity(__explicit_args.len() + varargs.len());
                        __arg_types.extend(varargs.iter().map(Variant::get_type));
                        let __vararg_str = varargs
                            .iter()
                            .map(|v| format!("{v}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        sys::panic_call_error(
                            &__err,
                            &format!("call({method:?}; {__vararg_str})"),
                            &__arg_types,
                        );
                    }
                });
                variant
            }
        }
        pub fn call_deferred(&mut self, method: StringName, varargs: &[Variant]) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("call_deferred");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3400424181i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "call_deferred" , 3400424181i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [<StringName as ToVariant>::to_variant(&method)];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let variant = Variant::from_var_sys_init(|return_ptr| {
                    let mut __err = sys::default_call_error();
                    __call_fn(
                        __method_bind,
                        self.object_ptr,
                        __args_ptr,
                        __args.len() as i64,
                        return_ptr,
                        std::ptr::addr_of_mut!(__err),
                    );
                    if __err.error != sys::GDEXTENSION_CALL_OK {
                        let mut __arg_types =
                            Vec::with_capacity(__explicit_args.len() + varargs.len());
                        __arg_types.extend(varargs.iter().map(Variant::get_type));
                        let __vararg_str = varargs
                            .iter()
                            .map(|v| format!("{v}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        sys::panic_call_error(
                            &__err,
                            &format!("call_deferred({method:?}; {__vararg_str})"),
                            &__arg_types,
                        );
                    }
                });
                variant
            }
        }
        pub fn set_deferred(&mut self, property: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set_deferred");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set_deferred" , 3776071444i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn callv(&mut self, method: StringName, arg_array: VariantArray) -> Variant {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("callv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1260104456i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "callv" , 1260104456i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&method),
                    <VariantArray as sys::GodotFfi>::sys_const(&arg_array),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_method(&self, method: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("has_method");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "has_method" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&method)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_signal(&self, signal: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("has_signal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "has_signal" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&signal)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_signal_list(&self) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_signal_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_signal_list" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_signal_connection_list(&self, signal: StringName) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_signal_connection_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3147814860i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_signal_connection_list" , 3147814860i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&signal)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_incoming_connections(&self) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("get_incoming_connections");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "get_incoming_connections" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn connect(
            &mut self,
            signal: StringName,
            callable: Callable,
            flags: i64,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("connect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1469446357i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "connect" , 1469446357i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&signal),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                    <i64 as sys::GodotFfi>::sys_const(&flags),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn disconnect(&mut self, signal: StringName, callable: Callable) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("disconnect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1874754934i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "disconnect" , 1874754934i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&signal),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_connected(&self, signal: StringName, callable: Callable) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("is_connected");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    768136979i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "is_connected" , 768136979i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&signal),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_block_signals(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set_block_signals");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set_block_signals" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_blocking_signals(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("is_blocking_signals");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "is_blocking_signals" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn notify_property_list_changed(&mut self) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("notify_property_list_changed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "notify_property_list_changed" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_message_translation(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("set_message_translation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "set_message_translation" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn can_translate_messages(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("can_translate_messages");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "can_translate_messages" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tr(&self, message: StringName, context: StringName) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("tr");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2475554935i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "tr" , 2475554935i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&message),
                    <StringName as sys::GodotFfi>::sys_const(&context),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn tr_n(
            &self,
            message: StringName,
            plural_message: StringName,
            n: i64,
            context: StringName,
        ) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("tr_n");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4021311862i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "tr_n" , 4021311862i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&message),
                    <StringName as sys::GodotFfi>::sys_const(&plural_message),
                    <i64 as sys::GodotFfi>::sys_const(&n),
                    <StringName as sys::GodotFfi>::sys_const(&context),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_queued_for_deletion(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Object");
                let __method_name = StringName::from("is_queued_for_deletion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Object" , "is_queued_for_deletion" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub(crate) const NOTIFICATION_POSTINITIALIZE: i32 = 0i32;
        pub(crate) const NOTIFICATION_PREDELETE: i32 = 1i32;
    }
    impl crate::obj::GodotClass for Object {
        type Base = ();
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::DynamicRefCount;
        const CLASS_NAME: &'static str = "Object";
    }
    impl crate::obj::EngineClass for Object {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl std::ops::Deref for Object {
        type Target = ();
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Object {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Object {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ConnectFlags {
    ord: i32,
}
impl ConnectFlags {
    pub const CONNECT_DEFERRED: Self = Self { ord: 1 };
    pub const CONNECT_PERSIST: Self = Self { ord: 2 };
    pub const CONNECT_ONE_SHOT: Self = Self { ord: 4 };
    pub const CONNECT_REFERENCE_COUNTED: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for ConnectFlags {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ConnectFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
