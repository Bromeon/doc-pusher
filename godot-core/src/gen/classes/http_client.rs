#![doc = "Sidecar module for class [`HttpClient`][crate::engine::HttpClient].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `HTTPClient` enums](https://docs.godotengine.org/en/stable/classes/class_httpclient.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `HTTPClient.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`http_client`][crate::engine::http_client]: sidecar module with related enum/flag types\n* [`HttpClientVirtual`][crate::engine::HttpClientVirtual]: virtual methods\n\n\nSee also [Godot docs for `HTTPClient`](https://docs.godotengine.org/en/stable/classes/class_httpclient.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct HttpClient {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`HttpClient`][crate::engine::HttpClient].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `HTTPClient` methods](https://docs.godotengine.org/en/stable/classes/class_httpclient.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait HttpClientVirtual:
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
    impl HttpClient {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
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
        pub fn connect_to_host(
            &mut self,
            host: GodotString,
            port: i64,
            tls_options: Gd<TlsOptions>,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("connect_to_host");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1970282951i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "connect_to_host" , 1970282951i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&host),
                    <i64 as sys::GodotFfi>::sys_const(&port),
                    <Gd<TlsOptions> as AsArg>::as_arg_ptr(&tls_options),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_connection(&mut self, connection: Gd<StreamPeer>) {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("set_connection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3281897016i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "set_connection" , 3281897016i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<StreamPeer> as AsArg>::as_arg_ptr(&connection)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_connection(&self) -> Option<Gd<StreamPeer>> {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_connection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2741655269i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_connection" , 2741655269i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<StreamPeer>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn request_raw(
            &mut self,
            method: http_client::Method,
            url: GodotString,
            headers: PackedStringArray,
            body: PackedByteArray,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("request_raw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    540161961i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "request_raw" , 540161961i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <http_client::Method as sys::GodotFfi>::sys_const(&method),
                    <GodotString as sys::GodotFfi>::sys_const(&url),
                    <PackedStringArray as sys::GodotFfi>::sys_const(&headers),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&body),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn request(
            &mut self,
            method: http_client::Method,
            url: GodotString,
            headers: PackedStringArray,
            body: GodotString,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("request");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3249905507i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "request" , 3249905507i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <http_client::Method as sys::GodotFfi>::sys_const(&method),
                    <GodotString as sys::GodotFfi>::sys_const(&url),
                    <PackedStringArray as sys::GodotFfi>::sys_const(&headers),
                    <GodotString as sys::GodotFfi>::sys_const(&body),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn close(&mut self) {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("close");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "close" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_response(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("has_response");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "has_response" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_response_chunked(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("is_response_chunked");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "is_response_chunked" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_response_code(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_response_code");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_response_code" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_response_headers(&mut self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_response_headers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2981934095i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_response_headers" , 2981934095i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_response_headers_as_dictionary(&mut self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_response_headers_as_dictionary");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2382534195i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_response_headers_as_dictionary" , 2382534195i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_response_body_length(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_response_body_length");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_response_body_length" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn read_response_body_chunk(&mut self) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("read_response_body_chunk");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2115431945i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "read_response_body_chunk" , 2115431945i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_read_chunk_size(&mut self, bytes: i64) {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("set_read_chunk_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "set_read_chunk_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bytes)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_read_chunk_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_read_chunk_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_read_chunk_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_blocking_mode(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("set_blocking_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "set_blocking_mode" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_blocking_mode_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("is_blocking_mode_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "is_blocking_mode_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_status(&self) -> http_client::Status {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("get_status");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1426656811i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "get_status" , 1426656811i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <http_client::Status as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn poll(&mut self) -> global::Error {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("poll");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166280745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "poll" , 166280745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_http_proxy(&mut self, host: GodotString, port: i64) {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("set_http_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2956805083i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "set_http_proxy" , 2956805083i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&host),
                    <i64 as sys::GodotFfi>::sys_const(&port),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_https_proxy(&mut self, host: GodotString, port: i64) {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("set_https_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2956805083i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "set_https_proxy" , 2956805083i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&host),
                    <i64 as sys::GodotFfi>::sys_const(&port),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn query_string_from_dict(&mut self, fields: Dictionary) -> GodotString {
            unsafe {
                let __class_name = StringName::from("HTTPClient");
                let __method_name = StringName::from("query_string_from_dict");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2538086567i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "HTTPClient" , "query_string_from_dict" , 2538086567i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&fields)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for HttpClient {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "HTTPClient";
    }
    impl crate::obj::EngineClass for HttpClient {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for HttpClient {}
    impl crate::obj::Inherits<crate::engine::Object> for HttpClient {}
    impl std::ops::Deref for HttpClient {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for HttpClient {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_HttpClient {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::HttpClient> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Method {
    ord: i32,
}
impl Method {
    pub const METHOD_GET: Self = Self { ord: 0 };
    pub const METHOD_HEAD: Self = Self { ord: 1 };
    pub const METHOD_POST: Self = Self { ord: 2 };
    pub const METHOD_PUT: Self = Self { ord: 3 };
    pub const METHOD_DELETE: Self = Self { ord: 4 };
    pub const METHOD_OPTIONS: Self = Self { ord: 5 };
    pub const METHOD_TRACE: Self = Self { ord: 6 };
    pub const METHOD_CONNECT: Self = Self { ord: 7 };
    pub const METHOD_PATCH: Self = Self { ord: 8 };
    pub const METHOD_MAX: Self = Self { ord: 9 };
}
impl crate::obj::EngineEnum for Method {
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
unsafe impl sys::GodotFfi for Method {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Status {
    ord: i32,
}
impl Status {
    pub const STATUS_DISCONNECTED: Self = Self { ord: 0 };
    pub const STATUS_RESOLVING: Self = Self { ord: 1 };
    pub const STATUS_CANT_RESOLVE: Self = Self { ord: 2 };
    pub const STATUS_CONNECTING: Self = Self { ord: 3 };
    pub const STATUS_CANT_CONNECT: Self = Self { ord: 4 };
    pub const STATUS_CONNECTED: Self = Self { ord: 5 };
    pub const STATUS_REQUESTING: Self = Self { ord: 6 };
    pub const STATUS_BODY: Self = Self { ord: 7 };
    pub const STATUS_CONNECTION_ERROR: Self = Self { ord: 8 };
    pub const STATUS_TLS_HANDSHAKE_ERROR: Self = Self { ord: 9 };
}
impl crate::obj::EngineEnum for Status {
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
unsafe impl sys::GodotFfi for Status {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ResponseCode {
    ord: i32,
}
impl ResponseCode {
    pub const RESPONSE_CONTINUE: Self = Self { ord: 100 };
    pub const RESPONSE_SWITCHING_PROTOCOLS: Self = Self { ord: 101 };
    pub const RESPONSE_PROCESSING: Self = Self { ord: 102 };
    pub const RESPONSE_OK: Self = Self { ord: 200 };
    pub const RESPONSE_CREATED: Self = Self { ord: 201 };
    pub const RESPONSE_ACCEPTED: Self = Self { ord: 202 };
    pub const RESPONSE_NON_AUTHORITATIVE_INFORMATION: Self = Self { ord: 203 };
    pub const RESPONSE_NO_CONTENT: Self = Self { ord: 204 };
    pub const RESPONSE_RESET_CONTENT: Self = Self { ord: 205 };
    pub const RESPONSE_PARTIAL_CONTENT: Self = Self { ord: 206 };
    pub const RESPONSE_MULTI_STATUS: Self = Self { ord: 207 };
    pub const RESPONSE_ALREADY_REPORTED: Self = Self { ord: 208 };
    pub const RESPONSE_IM_USED: Self = Self { ord: 226 };
    pub const RESPONSE_MULTIPLE_CHOICES: Self = Self { ord: 300 };
    pub const RESPONSE_MOVED_PERMANENTLY: Self = Self { ord: 301 };
    pub const RESPONSE_FOUND: Self = Self { ord: 302 };
    pub const RESPONSE_SEE_OTHER: Self = Self { ord: 303 };
    pub const RESPONSE_NOT_MODIFIED: Self = Self { ord: 304 };
    pub const RESPONSE_USE_PROXY: Self = Self { ord: 305 };
    pub const RESPONSE_SWITCH_PROXY: Self = Self { ord: 306 };
    pub const RESPONSE_TEMPORARY_REDIRECT: Self = Self { ord: 307 };
    pub const RESPONSE_PERMANENT_REDIRECT: Self = Self { ord: 308 };
    pub const RESPONSE_BAD_REQUEST: Self = Self { ord: 400 };
    pub const RESPONSE_UNAUTHORIZED: Self = Self { ord: 401 };
    pub const RESPONSE_PAYMENT_REQUIRED: Self = Self { ord: 402 };
    pub const RESPONSE_FORBIDDEN: Self = Self { ord: 403 };
    pub const RESPONSE_NOT_FOUND: Self = Self { ord: 404 };
    pub const RESPONSE_METHOD_NOT_ALLOWED: Self = Self { ord: 405 };
    pub const RESPONSE_NOT_ACCEPTABLE: Self = Self { ord: 406 };
    pub const RESPONSE_PROXY_AUTHENTICATION_REQUIRED: Self = Self { ord: 407 };
    pub const RESPONSE_REQUEST_TIMEOUT: Self = Self { ord: 408 };
    pub const RESPONSE_CONFLICT: Self = Self { ord: 409 };
    pub const RESPONSE_GONE: Self = Self { ord: 410 };
    pub const RESPONSE_LENGTH_REQUIRED: Self = Self { ord: 411 };
    pub const RESPONSE_PRECONDITION_FAILED: Self = Self { ord: 412 };
    pub const RESPONSE_REQUEST_ENTITY_TOO_LARGE: Self = Self { ord: 413 };
    pub const RESPONSE_REQUEST_URI_TOO_LONG: Self = Self { ord: 414 };
    pub const RESPONSE_UNSUPPORTED_MEDIA_TYPE: Self = Self { ord: 415 };
    pub const RESPONSE_REQUESTED_RANGE_NOT_SATISFIABLE: Self = Self { ord: 416 };
    pub const RESPONSE_EXPECTATION_FAILED: Self = Self { ord: 417 };
    pub const RESPONSE_IM_A_TEAPOT: Self = Self { ord: 418 };
    pub const RESPONSE_MISDIRECTED_REQUEST: Self = Self { ord: 421 };
    pub const RESPONSE_UNPROCESSABLE_ENTITY: Self = Self { ord: 422 };
    pub const RESPONSE_LOCKED: Self = Self { ord: 423 };
    pub const RESPONSE_FAILED_DEPENDENCY: Self = Self { ord: 424 };
    pub const RESPONSE_UPGRADE_REQUIRED: Self = Self { ord: 426 };
    pub const RESPONSE_PRECONDITION_REQUIRED: Self = Self { ord: 428 };
    pub const RESPONSE_TOO_MANY_REQUESTS: Self = Self { ord: 429 };
    pub const RESPONSE_REQUEST_HEADER_FIELDS_TOO_LARGE: Self = Self { ord: 431 };
    pub const RESPONSE_UNAVAILABLE_FOR_LEGAL_REASONS: Self = Self { ord: 451 };
    pub const RESPONSE_INTERNAL_SERVER_ERROR: Self = Self { ord: 500 };
    pub const RESPONSE_NOT_IMPLEMENTED: Self = Self { ord: 501 };
    pub const RESPONSE_BAD_GATEWAY: Self = Self { ord: 502 };
    pub const RESPONSE_SERVICE_UNAVAILABLE: Self = Self { ord: 503 };
    pub const RESPONSE_GATEWAY_TIMEOUT: Self = Self { ord: 504 };
    pub const RESPONSE_HTTP_VERSION_NOT_SUPPORTED: Self = Self { ord: 505 };
    pub const RESPONSE_VARIANT_ALSO_NEGOTIATES: Self = Self { ord: 506 };
    pub const RESPONSE_INSUFFICIENT_STORAGE: Self = Self { ord: 507 };
    pub const RESPONSE_LOOP_DETECTED: Self = Self { ord: 508 };
    pub const RESPONSE_NOT_EXTENDED: Self = Self { ord: 510 };
    pub const RESPONSE_NETWORK_AUTH_REQUIRED: Self = Self { ord: 511 };
}
impl crate::obj::EngineEnum for ResponseCode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 100i32
            | ord @ 101i32
            | ord @ 102i32
            | ord @ 200i32
            | ord @ 201i32
            | ord @ 202i32
            | ord @ 203i32
            | ord @ 204i32
            | ord @ 205i32
            | ord @ 206i32
            | ord @ 207i32
            | ord @ 208i32
            | ord @ 226i32
            | ord @ 300i32
            | ord @ 301i32
            | ord @ 302i32
            | ord @ 303i32
            | ord @ 304i32
            | ord @ 305i32
            | ord @ 306i32
            | ord @ 307i32
            | ord @ 308i32
            | ord @ 400i32
            | ord @ 401i32
            | ord @ 402i32
            | ord @ 403i32
            | ord @ 404i32
            | ord @ 405i32
            | ord @ 406i32
            | ord @ 407i32
            | ord @ 408i32
            | ord @ 409i32
            | ord @ 410i32
            | ord @ 411i32
            | ord @ 412i32
            | ord @ 413i32
            | ord @ 414i32
            | ord @ 415i32
            | ord @ 416i32
            | ord @ 417i32
            | ord @ 418i32
            | ord @ 421i32
            | ord @ 422i32
            | ord @ 423i32
            | ord @ 424i32
            | ord @ 426i32
            | ord @ 428i32
            | ord @ 429i32
            | ord @ 431i32
            | ord @ 451i32
            | ord @ 500i32
            | ord @ 501i32
            | ord @ 502i32
            | ord @ 503i32
            | ord @ 504i32
            | ord @ 505i32
            | ord @ 506i32
            | ord @ 507i32
            | ord @ 508i32
            | ord @ 510i32
            | ord @ 511i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ResponseCode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
