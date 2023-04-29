#![doc = "Sidecar module for class [`Crypto`][crate::engine::Crypto].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Crypto` enums](https://docs.godotengine.org/en/stable/classes/class_crypto.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Crypto.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`CryptoVirtual`][crate::engine::CryptoVirtual]: virtual methods\n\n\nSee also [Godot docs for `Crypto`](https://docs.godotengine.org/en/stable/classes/class_crypto.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Crypto {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Crypto`][crate::engine::Crypto].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Crypto` methods](https://docs.godotengine.org/en/stable/classes/class_crypto.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait CryptoVirtual:
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
    impl Crypto {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Crypto");
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
        pub fn generate_random_bytes(&mut self, size: i64) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("generate_random_bytes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    47165747i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "generate_random_bytes" , 47165747i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn generate_rsa(&mut self, size: i64) -> Option<Gd<CryptoKey>> {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("generate_rsa");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1237515462i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "generate_rsa" , 1237515462i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                <Gd<CryptoKey>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn generate_self_signed_certificate(
            &mut self,
            key: Gd<CryptoKey>,
            issuer_name: GodotString,
            not_before: GodotString,
            not_after: GodotString,
        ) -> Option<Gd<X509Certificate>> {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("generate_self_signed_certificate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    947314696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "generate_self_signed_certificate" , 947314696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<CryptoKey> as AsArg>::as_arg_ptr(&key),
                    <GodotString as sys::GodotFfi>::sys_const(&issuer_name),
                    <GodotString as sys::GodotFfi>::sys_const(&not_before),
                    <GodotString as sys::GodotFfi>::sys_const(&not_after),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<X509Certificate>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn sign(
            &mut self,
            hash_type: hashing_context::HashType,
            hash: PackedByteArray,
            key: Gd<CryptoKey>,
        ) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("sign");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1673662703i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "sign" , 1673662703i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <hashing_context::HashType as sys::GodotFfi>::sys_const(&hash_type),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&hash),
                    <Gd<CryptoKey> as AsArg>::as_arg_ptr(&key),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn verify(
            &mut self,
            hash_type: hashing_context::HashType,
            hash: PackedByteArray,
            signature: PackedByteArray,
            key: Gd<CryptoKey>,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("verify");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2805902225i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "verify" , 2805902225i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <hashing_context::HashType as sys::GodotFfi>::sys_const(&hash_type),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&hash),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&signature),
                    <Gd<CryptoKey> as AsArg>::as_arg_ptr(&key),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn encrypt(
            &mut self,
            key: Gd<CryptoKey>,
            plaintext: PackedByteArray,
        ) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("encrypt");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2361793670i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "encrypt" , 2361793670i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<CryptoKey> as AsArg>::as_arg_ptr(&key),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&plaintext),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn decrypt(
            &mut self,
            key: Gd<CryptoKey>,
            ciphertext: PackedByteArray,
        ) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("decrypt");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2361793670i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "decrypt" , 2361793670i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<CryptoKey> as AsArg>::as_arg_ptr(&key),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&ciphertext),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn hmac_digest(
            &mut self,
            hash_type: hashing_context::HashType,
            key: PackedByteArray,
            msg: PackedByteArray,
        ) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("hmac_digest");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2368951203i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "hmac_digest" , 2368951203i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <hashing_context::HashType as sys::GodotFfi>::sys_const(&hash_type),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&key),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&msg),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn constant_time_compare(
            &mut self,
            trusted: PackedByteArray,
            received: PackedByteArray,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("Crypto");
                let __method_name = StringName::from("constant_time_compare");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1024142237i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Crypto" , "constant_time_compare" , 1024142237i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedByteArray as sys::GodotFfi>::sys_const(&trusted),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&received),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Crypto {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Crypto";
    }
    impl crate::obj::EngineClass for Crypto {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for Crypto {}
    impl crate::obj::Inherits<crate::engine::Object> for Crypto {}
    impl std::ops::Deref for Crypto {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Crypto {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Crypto {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Crypto> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
