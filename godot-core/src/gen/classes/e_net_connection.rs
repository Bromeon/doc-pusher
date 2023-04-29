#![doc = "Sidecar module for class [`ENetConnection`][crate::engine::ENetConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetConnection` enums](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `ENetConnection.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`e_net_connection`][crate::engine::e_net_connection]: sidecar module with related enum/flag types\n* [`ENetConnectionVirtual`][crate::engine::ENetConnectionVirtual]: virtual methods\n\n\nSee also [Godot docs for `ENetConnection`](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ENetConnection {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`ENetConnection`][crate::engine::ENetConnection].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetConnection` methods](https://docs.godotengine.org/en/stable/classes/class_enetconnection.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ENetConnectionVirtual:
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
    impl ENetConnection {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
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
        pub fn create_host_bound(
            &mut self,
            bind_address: GodotString,
            bind_port: i64,
            max_peers: i64,
            max_channels: i64,
            in_bandwidth: i64,
            out_bandwidth: i64,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("create_host_bound");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866250949i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "create_host_bound" , 866250949i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&bind_address),
                    <i64 as sys::GodotFfi>::sys_const(&bind_port),
                    <i64 as sys::GodotFfi>::sys_const(&max_peers),
                    <i64 as sys::GodotFfi>::sys_const(&max_channels),
                    <i64 as sys::GodotFfi>::sys_const(&in_bandwidth),
                    <i64 as sys::GodotFfi>::sys_const(&out_bandwidth),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_host(
            &mut self,
            max_peers: i64,
            max_channels: i64,
            in_bandwidth: i64,
            out_bandwidth: i64,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("create_host");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    117198950i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "create_host" , 117198950i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&max_peers),
                    <i64 as sys::GodotFfi>::sys_const(&max_channels),
                    <i64 as sys::GodotFfi>::sys_const(&in_bandwidth),
                    <i64 as sys::GodotFfi>::sys_const(&out_bandwidth),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn destroy(&mut self) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("destroy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "destroy" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn connect_to_host(
            &mut self,
            address: GodotString,
            port: i64,
            channels: i64,
            data: i64,
        ) -> Option<Gd<ENetPacketPeer>> {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("connect_to_host");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    385984708i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "connect_to_host" , 385984708i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&address),
                    <i64 as sys::GodotFfi>::sys_const(&port),
                    <i64 as sys::GodotFfi>::sys_const(&channels),
                    <i64 as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<ENetPacketPeer>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn service(&mut self, timeout: i64) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("service");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2402345344i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "service" , 2402345344i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&timeout)];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn flush(&mut self) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("flush");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "flush" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn bandwidth_limit(&mut self, in_bandwidth: i64, out_bandwidth: i64) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("bandwidth_limit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2302169788i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "bandwidth_limit" , 2302169788i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&in_bandwidth),
                    <i64 as sys::GodotFfi>::sys_const(&out_bandwidth),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn channel_limit(&mut self, limit: i64) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("channel_limit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "channel_limit" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&limit)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn broadcast(&mut self, channel: i64, packet: PackedByteArray, flags: i64) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("broadcast");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2772371345i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "broadcast" , 2772371345i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&channel),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&packet),
                    <i64 as sys::GodotFfi>::sys_const(&flags),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compress(&mut self, mode: e_net_connection::CompressionMode) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("compress");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2660215187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "compress" , 2660215187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<e_net_connection::CompressionMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn dtls_server_setup(&mut self, server_options: Gd<TlsOptions>) -> global::Error {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("dtls_server_setup");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1262296096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "dtls_server_setup" , 1262296096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<TlsOptions> as AsArg>::as_arg_ptr(&server_options)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn dtls_client_setup(
            &mut self,
            hostname: GodotString,
            client_options: Gd<TlsOptions>,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("dtls_client_setup");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3097527179i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "dtls_client_setup" , 3097527179i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&hostname),
                    <Gd<TlsOptions> as AsArg>::as_arg_ptr(&client_options),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn refuse_new_connections(&mut self, refuse: bool) {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("refuse_new_connections");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "refuse_new_connections" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&refuse)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pop_statistic(&mut self, statistic: e_net_connection::HostStatistic) -> f64 {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("pop_statistic");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2166904170i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "pop_statistic" , 2166904170i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<e_net_connection::HostStatistic as sys::GodotFfi>::sys_const(&statistic)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_max_channels(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("get_max_channels");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "get_max_channels" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_local_port(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("get_local_port");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "get_local_port" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_peers(&mut self) -> Array<Gd<ENetPacketPeer>> {
            unsafe {
                let __class_name = StringName::from("ENetConnection");
                let __method_name = StringName::from("get_peers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetConnection" , "get_peers" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<ENetPacketPeer>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for ENetConnection {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "ENetConnection";
    }
    impl crate::obj::EngineClass for ENetConnection {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for ENetConnection {}
    impl crate::obj::Inherits<crate::engine::Object> for ENetConnection {}
    impl std::ops::Deref for ENetConnection {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for ENetConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_ENetConnection {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::ENetConnection> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CompressionMode {
    ord: i32,
}
impl CompressionMode {
    pub const COMPRESS_NONE: Self = Self { ord: 0 };
    pub const COMPRESS_RANGE_CODER: Self = Self { ord: 1 };
    pub const COMPRESS_FASTLZ: Self = Self { ord: 2 };
    pub const COMPRESS_ZLIB: Self = Self { ord: 3 };
    pub const COMPRESS_ZSTD: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for CompressionMode {
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
unsafe impl sys::GodotFfi for CompressionMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EventType {
    ord: i32,
}
impl EventType {
    pub const EVENT_ERROR: Self = Self { ord: -1 };
    pub const EVENT_NONE: Self = Self { ord: 0 };
    pub const EVENT_CONNECT: Self = Self { ord: 1 };
    pub const EVENT_DISCONNECT: Self = Self { ord: 2 };
    pub const EVENT_RECEIVE: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for EventType {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ -1i32 | ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for EventType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct HostStatistic {
    ord: i32,
}
impl HostStatistic {
    pub const HOST_TOTAL_SENT_DATA: Self = Self { ord: 0 };
    pub const HOST_TOTAL_SENT_PACKETS: Self = Self { ord: 1 };
    pub const HOST_TOTAL_RECEIVED_DATA: Self = Self { ord: 2 };
    pub const HOST_TOTAL_RECEIVED_PACKETS: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for HostStatistic {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for HostStatistic {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
