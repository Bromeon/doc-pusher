#![doc = "Sidecar module for class [`ENetPacketPeer`][crate::engine::ENetPacketPeer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ENetPacketPeer` enums](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `ENetPacketPeer.`\n\nInherits [`PacketPeer`][crate::engine::PacketPeer].\n\nRelated symbols:\n\n* [`e_net_packet_peer`][crate::engine::e_net_packet_peer]: sidecar module with related enum/flag types\n* [`ENetPacketPeerVirtual`][crate::engine::ENetPacketPeerVirtual]: virtual methods\n\n\nSee also [Godot docs for `ENetPacketPeer`](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ENetPacketPeer {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`ENetPacketPeer`][crate::engine::ENetPacketPeer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ENetPacketPeer` methods](https://docs.godotengine.org/en/stable/classes/class_enetpacketpeer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ENetPacketPeerVirtual:
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
    impl ENetPacketPeer {
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
        pub fn peer_disconnect(&mut self, data: i64) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("peer_disconnect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1995695955i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "peer_disconnect" , 1995695955i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&data)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn peer_disconnect_later(&mut self, data: i64) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("peer_disconnect_later");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1995695955i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "peer_disconnect_later" , 1995695955i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&data)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn peer_disconnect_now(&mut self, data: i64) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("peer_disconnect_now");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1995695955i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "peer_disconnect_now" , 1995695955i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&data)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn ping(&mut self) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("ping");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "ping" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn ping_interval(&mut self, ping_interval: i64) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("ping_interval");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "ping_interval" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&ping_interval)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reset(&mut self) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("reset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "reset" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn send(&mut self, channel: i64, packet: PackedByteArray, flags: i64) -> global::Error {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("send");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    120522849i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "send" , 120522849i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&channel),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&packet),
                    <i64 as sys::GodotFfi>::sys_const(&flags),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn throttle_configure(&mut self, interval: i64, acceleration: i64, deceleration: i64) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("throttle_configure");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1649997291i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "throttle_configure" , 1649997291i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&interval),
                    <i64 as sys::GodotFfi>::sys_const(&acceleration),
                    <i64 as sys::GodotFfi>::sys_const(&deceleration),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_timeout(&mut self, timeout: i64, timeout_min: i64, timeout_max: i64) {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("set_timeout");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1649997291i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "set_timeout" , 1649997291i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&timeout),
                    <i64 as sys::GodotFfi>::sys_const(&timeout_min),
                    <i64 as sys::GodotFfi>::sys_const(&timeout_max),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_remote_address(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("get_remote_address");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "get_remote_address" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_remote_port(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("get_remote_port");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "get_remote_port" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_statistic(&mut self, statistic: e_net_packet_peer::PeerStatistic) -> f64 {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("get_statistic");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1642578323i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "get_statistic" , 1642578323i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<e_net_packet_peer::PeerStatistic as sys::GodotFfi>::sys_const(&statistic)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_state(&self) -> e_net_packet_peer::PeerState {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("get_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    711068532i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "get_state" , 711068532i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <e_net_packet_peer::PeerState as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn get_channels(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("get_channels");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "get_channels" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_active(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("ENetPacketPeer");
                let __method_name = StringName::from("is_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "ENetPacketPeer" , "is_active" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub const PACKET_LOSS_SCALE: i32 = 65536i32;
        pub const PACKET_THROTTLE_SCALE: i32 = 32i32;
        pub const FLAG_RELIABLE: i32 = 1i32;
        pub const FLAG_UNSEQUENCED: i32 = 2i32;
        pub const FLAG_UNRELIABLE_FRAGMENT: i32 = 8i32;
    }
    impl crate::obj::GodotClass for ENetPacketPeer {
        type Base = crate::engine::PacketPeer;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "ENetPacketPeer";
    }
    impl crate::obj::EngineClass for ENetPacketPeer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::PacketPeer> for ENetPacketPeer {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for ENetPacketPeer {}
    impl crate::obj::Inherits<crate::engine::Object> for ENetPacketPeer {}
    impl std::ops::Deref for ENetPacketPeer {
        type Target = crate::engine::PacketPeer;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for ENetPacketPeer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_ENetPacketPeer {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::ENetPacketPeer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::PacketPeer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PeerState {
    ord: i32,
}
impl PeerState {
    pub const STATE_DISCONNECTED: Self = Self { ord: 0 };
    pub const STATE_CONNECTING: Self = Self { ord: 1 };
    pub const STATE_ACKNOWLEDGING_CONNECT: Self = Self { ord: 2 };
    pub const STATE_CONNECTION_PENDING: Self = Self { ord: 3 };
    pub const STATE_CONNECTION_SUCCEEDED: Self = Self { ord: 4 };
    pub const STATE_CONNECTED: Self = Self { ord: 5 };
    pub const STATE_DISCONNECT_LATER: Self = Self { ord: 6 };
    pub const STATE_DISCONNECTING: Self = Self { ord: 7 };
    pub const STATE_ACKNOWLEDGING_DISCONNECT: Self = Self { ord: 8 };
    pub const STATE_ZOMBIE: Self = Self { ord: 9 };
}
impl crate::obj::EngineEnum for PeerState {
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
            | ord @ 9i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for PeerState {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PeerStatistic {
    ord: i32,
}
impl PeerStatistic {
    pub const PEER_PACKET_LOSS: Self = Self { ord: 0 };
    pub const PEER_PACKET_LOSS_VARIANCE: Self = Self { ord: 1 };
    pub const PEER_PACKET_LOSS_EPOCH: Self = Self { ord: 2 };
    pub const PEER_ROUND_TRIP_TIME: Self = Self { ord: 3 };
    pub const PEER_ROUND_TRIP_TIME_VARIANCE: Self = Self { ord: 4 };
    pub const PEER_LAST_ROUND_TRIP_TIME: Self = Self { ord: 5 };
    pub const PEER_LAST_ROUND_TRIP_TIME_VARIANCE: Self = Self { ord: 6 };
    pub const PEER_PACKET_THROTTLE: Self = Self { ord: 7 };
    pub const PEER_PACKET_THROTTLE_LIMIT: Self = Self { ord: 8 };
    pub const PEER_PACKET_THROTTLE_COUNTER: Self = Self { ord: 9 };
    pub const PEER_PACKET_THROTTLE_EPOCH: Self = Self { ord: 10 };
    pub const PEER_PACKET_THROTTLE_ACCELERATION: Self = Self { ord: 11 };
    pub const PEER_PACKET_THROTTLE_DECELERATION: Self = Self { ord: 12 };
    pub const PEER_PACKET_THROTTLE_INTERVAL: Self = Self { ord: 13 };
}
impl crate::obj::EngineEnum for PeerStatistic {
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
unsafe impl sys::GodotFfi for PeerStatistic {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
