#![doc = "Sidecar module for class [`Upnp`][crate::engine::Upnp].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `UPNP` enums](https://docs.godotengine.org/en/stable/classes/class_upnp.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `UPNP.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`upnp`][crate::engine::upnp]: sidecar module with related enum/flag types\n* [`UpnpVirtual`][crate::engine::UpnpVirtual]: virtual methods\n\n\nSee also [Godot docs for `UPNP`](https://docs.godotengine.org/en/stable/classes/class_upnp.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Upnp {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Upnp`][crate::engine::Upnp].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `UPNP` methods](https://docs.godotengine.org/en/stable/classes/class_upnp.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait UpnpVirtual:
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
    impl Upnp {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("UPNP");
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
        pub fn get_device_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("get_device_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "get_device_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_device(&self, index: i64) -> Option<Gd<UpnpDevice>> {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("get_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2193290270i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "get_device" , 2193290270i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <Gd<UpnpDevice>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_device(&mut self, device: Gd<UpnpDevice>) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("add_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    986715920i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "add_device" , 986715920i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<UpnpDevice> as AsArg>::as_arg_ptr(&device)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_device(&mut self, index: i64, device: Gd<UpnpDevice>) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("set_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3015133723i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "set_device" , 3015133723i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Gd<UpnpDevice> as AsArg>::as_arg_ptr(&device),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_device(&mut self, index: i64) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("remove_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "remove_device" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_devices(&mut self) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("clear_devices");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "clear_devices" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_gateway(&self) -> Option<Gd<UpnpDevice>> {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("get_gateway");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2276800779i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "get_gateway" , 2276800779i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<UpnpDevice>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn discover(&mut self, timeout: i64, ttl: i64, device_filter: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("discover");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1575334765i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "discover" , 1575334765i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&timeout),
                    <i64 as sys::GodotFfi>::sys_const(&ttl),
                    <GodotString as sys::GodotFfi>::sys_const(&device_filter),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn query_external_address(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("query_external_address");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "query_external_address" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_port_mapping(
            &self,
            port: i64,
            port_internal: i64,
            desc: GodotString,
            proto: GodotString,
            duration: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("add_port_mapping");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3358934458i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "add_port_mapping" , 3358934458i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&port),
                    <i64 as sys::GodotFfi>::sys_const(&port_internal),
                    <GodotString as sys::GodotFfi>::sys_const(&desc),
                    <GodotString as sys::GodotFfi>::sys_const(&proto),
                    <i64 as sys::GodotFfi>::sys_const(&duration),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn delete_port_mapping(&self, port: i64, proto: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("delete_port_mapping");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    760296170i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "delete_port_mapping" , 760296170i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&port),
                    <GodotString as sys::GodotFfi>::sys_const(&proto),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_discover_multicast_if(&mut self, m_if: GodotString) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("set_discover_multicast_if");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "set_discover_multicast_if" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&m_if)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_discover_multicast_if(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("get_discover_multicast_if");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "get_discover_multicast_if" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_discover_local_port(&mut self, port: i64) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("set_discover_local_port");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "set_discover_local_port" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&port)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_discover_local_port(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("get_discover_local_port");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "get_discover_local_port" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_discover_ipv6(&mut self, ipv6: bool) {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("set_discover_ipv6");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "set_discover_ipv6" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&ipv6)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_discover_ipv6(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("UPNP");
                let __method_name = StringName::from("is_discover_ipv6");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "UPNP" , "is_discover_ipv6" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Upnp {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "UPNP";
    }
    impl crate::obj::EngineClass for Upnp {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for Upnp {}
    impl crate::obj::Inherits<crate::engine::Object> for Upnp {}
    impl std::ops::Deref for Upnp {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Upnp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Upnp {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Upnp> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct UPNPResult {
    ord: i32,
}
impl UPNPResult {
    pub const UPNP_RESULT_SUCCESS: Self = Self { ord: 0 };
    pub const UPNP_RESULT_NOT_AUTHORIZED: Self = Self { ord: 1 };
    pub const UPNP_RESULT_PORT_MAPPING_NOT_FOUND: Self = Self { ord: 2 };
    pub const UPNP_RESULT_INCONSISTENT_PARAMETERS: Self = Self { ord: 3 };
    pub const UPNP_RESULT_NO_SUCH_ENTRY_IN_ARRAY: Self = Self { ord: 4 };
    pub const UPNP_RESULT_ACTION_FAILED: Self = Self { ord: 5 };
    pub const UPNP_RESULT_SRC_IP_WILDCARD_NOT_PERMITTED: Self = Self { ord: 6 };
    pub const UPNP_RESULT_EXT_PORT_WILDCARD_NOT_PERMITTED: Self = Self { ord: 7 };
    pub const UPNP_RESULT_INT_PORT_WILDCARD_NOT_PERMITTED: Self = Self { ord: 8 };
    pub const UPNP_RESULT_REMOTE_HOST_MUST_BE_WILDCARD: Self = Self { ord: 9 };
    pub const UPNP_RESULT_EXT_PORT_MUST_BE_WILDCARD: Self = Self { ord: 10 };
    pub const UPNP_RESULT_NO_PORT_MAPS_AVAILABLE: Self = Self { ord: 11 };
    pub const UPNP_RESULT_CONFLICT_WITH_OTHER_MECHANISM: Self = Self { ord: 12 };
    pub const UPNP_RESULT_CONFLICT_WITH_OTHER_MAPPING: Self = Self { ord: 13 };
    pub const UPNP_RESULT_SAME_PORT_VALUES_REQUIRED: Self = Self { ord: 14 };
    pub const UPNP_RESULT_ONLY_PERMANENT_LEASE_SUPPORTED: Self = Self { ord: 15 };
    pub const UPNP_RESULT_INVALID_GATEWAY: Self = Self { ord: 16 };
    pub const UPNP_RESULT_INVALID_PORT: Self = Self { ord: 17 };
    pub const UPNP_RESULT_INVALID_PROTOCOL: Self = Self { ord: 18 };
    pub const UPNP_RESULT_INVALID_DURATION: Self = Self { ord: 19 };
    pub const UPNP_RESULT_INVALID_ARGS: Self = Self { ord: 20 };
    pub const UPNP_RESULT_INVALID_RESPONSE: Self = Self { ord: 21 };
    pub const UPNP_RESULT_INVALID_PARAM: Self = Self { ord: 22 };
    pub const UPNP_RESULT_HTTP_ERROR: Self = Self { ord: 23 };
    pub const UPNP_RESULT_SOCKET_ERROR: Self = Self { ord: 24 };
    pub const UPNP_RESULT_MEM_ALLOC_ERROR: Self = Self { ord: 25 };
    pub const UPNP_RESULT_NO_GATEWAY: Self = Self { ord: 26 };
    pub const UPNP_RESULT_NO_DEVICES: Self = Self { ord: 27 };
    pub const UPNP_RESULT_UNKNOWN_ERROR: Self = Self { ord: 28 };
}
impl crate::obj::EngineEnum for UPNPResult {
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
            | ord @ 13i32
            | ord @ 14i32
            | ord @ 15i32
            | ord @ 16i32
            | ord @ 17i32
            | ord @ 18i32
            | ord @ 19i32
            | ord @ 20i32
            | ord @ 21i32
            | ord @ 22i32
            | ord @ 23i32
            | ord @ 24i32
            | ord @ 25i32
            | ord @ 26i32
            | ord @ 27i32
            | ord @ 28i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for UPNPResult {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
