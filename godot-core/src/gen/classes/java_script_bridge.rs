#![doc = "Sidecar module for class [`JavaScriptBridge`][crate::engine::JavaScriptBridge].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `JavaScriptBridge` enums](https://docs.godotengine.org/en/stable/classes/class_javascriptbridge.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `JavaScriptBridge.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`JavaScriptBridgeVirtual`][crate::engine::JavaScriptBridgeVirtual]: virtual methods\n\n\nSee also [Godot docs for `JavaScriptBridge`](https://docs.godotengine.org/en/stable/classes/class_javascriptbridge.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct JavaScriptBridge {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`JavaScriptBridge`][crate::engine::JavaScriptBridge].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `JavaScriptBridge` methods](https://docs.godotengine.org/en/stable/classes/class_javascriptbridge.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait JavaScriptBridgeVirtual:
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
    impl JavaScriptBridge {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __object_ptr =
                    sys::interface_fn!(global_get_singleton)(__class_name.string_sys());
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
        pub fn eval(&mut self, code: GodotString, use_global_execution_context: bool) -> Variant {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("eval");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    218087648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "eval" , 218087648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&code),
                    <bool as sys::GodotFfi>::sys_const(&use_global_execution_context),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_interface(&mut self, interface: GodotString) -> Option<Gd<JavaScriptObject>> {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("get_interface");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1355533281i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "get_interface" , 1355533281i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&interface)];
                let __args_ptr = __args.as_ptr();
                <Gd<JavaScriptObject>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_callback(&mut self, callable: Callable) -> Option<Gd<JavaScriptObject>> {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("create_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    422818440i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "create_callback" , 422818440i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Callable as sys::GodotFfi>::sys_const(&callable)];
                let __args_ptr = __args.as_ptr();
                <Gd<JavaScriptObject>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_object(&mut self, object: GodotString, varargs: &[Variant]) -> Variant {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("create_object");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3093893586i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "create_object" , 3093893586i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [<GodotString as ToVariant>::to_variant(&object)];
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
                            &format!("create_object({object:?}; {__vararg_str})"),
                            &__arg_types,
                        );
                    }
                });
                variant
            }
        }
        pub fn download_buffer(
            &mut self,
            buffer: PackedByteArray,
            name: GodotString,
            mime: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("download_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4123979296i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "download_buffer" , 4123979296i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedByteArray as sys::GodotFfi>::sys_const(&buffer),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <GodotString as sys::GodotFfi>::sys_const(&mime),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn pwa_needs_update(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("pwa_needs_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "pwa_needs_update" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn pwa_update(&mut self) -> global::Error {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("pwa_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    166280745i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "pwa_update" , 166280745i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn force_fs_sync(&mut self) {
            unsafe {
                let __class_name = StringName::from("JavaScriptBridge");
                let __method_name = StringName::from("force_fs_sync");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "JavaScriptBridge" , "force_fs_sync" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for JavaScriptBridge {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "JavaScriptBridge";
    }
    impl crate::obj::EngineClass for JavaScriptBridge {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for JavaScriptBridge {}
    impl std::ops::Deref for JavaScriptBridge {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for JavaScriptBridge {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_JavaScriptBridge {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::JavaScriptBridge> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
