#![doc = "Sidecar module for class [`WebRtcPeerConnection`][crate::engine::WebRtcPeerConnection].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WebRTCPeerConnection` enums](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `WebRTCPeerConnection.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`web_rtc_peer_connection`][crate::engine::web_rtc_peer_connection]: sidecar module with related enum/flag types\n* [`WebRtcPeerConnectionVirtual`][crate::engine::WebRtcPeerConnectionVirtual]: virtual methods\n\n\nSee also [Godot docs for `WebRTCPeerConnection`](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct WebRtcPeerConnection {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`WebRtcPeerConnection`][crate::engine::WebRtcPeerConnection].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WebRTCPeerConnection` methods](https://docs.godotengine.org/en/stable/classes/class_webrtcpeerconnection.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait WebRtcPeerConnectionVirtual:
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
    impl WebRtcPeerConnection {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
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
        pub fn set_default_extension(extension_class: StringName) {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("set_default_extension");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "set_default_extension" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&extension_class)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, std::ptr::null_mut(), __args_ptr, return_ptr);
            }
        }
        pub fn initialize(&mut self, configuration: Dictionary) -> global::Error {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("initialize");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2625064318i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "initialize" , 2625064318i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&configuration)];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_data_channel(
            &mut self,
            label: GodotString,
            options: Dictionary,
        ) -> Option<Gd<WebRtcDataChannel>> {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("create_data_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3997447457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "create_data_channel" , 3997447457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&label),
                    <Dictionary as sys::GodotFfi>::sys_const(&options),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<WebRtcDataChannel>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_offer(&mut self) -> global::Error {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("create_offer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166280745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "create_offer" , 166280745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_local_description(
            &mut self,
            type_: GodotString,
            sdp: GodotString,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("set_local_description");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    852856452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "set_local_description" , 852856452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&type_),
                    <GodotString as sys::GodotFfi>::sys_const(&sdp),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_remote_description(
            &mut self,
            type_: GodotString,
            sdp: GodotString,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("set_remote_description");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    852856452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "set_remote_description" , 852856452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&type_),
                    <GodotString as sys::GodotFfi>::sys_const(&sdp),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_ice_candidate(
            &mut self,
            media: GodotString,
            index: i64,
            name: GodotString,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("add_ice_candidate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3958950400i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "add_ice_candidate" , 3958950400i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&media),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn poll(&mut self) -> global::Error {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("poll");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166280745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "poll" , 166280745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn close(&mut self) {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("close");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "close" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_connection_state(&self) -> web_rtc_peer_connection::ConnectionState {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("get_connection_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2275710506i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "get_connection_state" , 2275710506i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <web_rtc_peer_connection::ConnectionState as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn get_gathering_state(&self) -> web_rtc_peer_connection::GatheringState {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("get_gathering_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4262591401i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "get_gathering_state" , 4262591401i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <web_rtc_peer_connection::GatheringState as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn get_signaling_state(&self) -> web_rtc_peer_connection::SignalingState {
            unsafe {
                let __class_name = StringName::from("WebRTCPeerConnection");
                let __method_name = StringName::from("get_signaling_state");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3342956226i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WebRTCPeerConnection" , "get_signaling_state" , 3342956226i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <web_rtc_peer_connection::SignalingState as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
    }
    impl crate::obj::GodotClass for WebRtcPeerConnection {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "WebRTCPeerConnection";
    }
    impl crate::obj::EngineClass for WebRtcPeerConnection {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for WebRtcPeerConnection {}
    impl crate::obj::Inherits<crate::engine::Object> for WebRtcPeerConnection {}
    impl std::ops::Deref for WebRtcPeerConnection {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for WebRtcPeerConnection {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_WebRtcPeerConnection {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::WebRtcPeerConnection> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ConnectionState {
    ord: i32,
}
impl ConnectionState {
    pub const STATE_NEW: Self = Self { ord: 0 };
    pub const STATE_CONNECTING: Self = Self { ord: 1 };
    pub const STATE_CONNECTED: Self = Self { ord: 2 };
    pub const STATE_DISCONNECTED: Self = Self { ord: 3 };
    pub const STATE_FAILED: Self = Self { ord: 4 };
    pub const STATE_CLOSED: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for ConnectionState {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ConnectionState {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct GatheringState {
    ord: i32,
}
impl GatheringState {
    pub const GATHERING_STATE_NEW: Self = Self { ord: 0 };
    pub const GATHERING_STATE_GATHERING: Self = Self { ord: 1 };
    pub const GATHERING_STATE_COMPLETE: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for GatheringState {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for GatheringState {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SignalingState {
    ord: i32,
}
impl SignalingState {
    pub const SIGNALING_STATE_STABLE: Self = Self { ord: 0 };
    pub const SIGNALING_STATE_HAVE_LOCAL_OFFER: Self = Self { ord: 1 };
    pub const SIGNALING_STATE_HAVE_REMOTE_OFFER: Self = Self { ord: 2 };
    pub const SIGNALING_STATE_HAVE_LOCAL_PRANSWER: Self = Self { ord: 3 };
    pub const SIGNALING_STATE_HAVE_REMOTE_PRANSWER: Self = Self { ord: 4 };
    pub const SIGNALING_STATE_CLOSED: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for SignalingState {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for SignalingState {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
