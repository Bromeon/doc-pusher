#![doc = "Sidecar module for class [`EngineDebugger`][crate::engine::EngineDebugger].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EngineDebugger` enums](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EngineDebugger.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`EngineDebuggerVirtual`][crate::engine::EngineDebuggerVirtual]: virtual methods\n\n\nSee also [Godot docs for `EngineDebugger`](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EngineDebugger {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EngineDebugger`][crate::engine::EngineDebugger].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EngineDebugger` methods](https://docs.godotengine.org/en/stable/classes/class_enginedebugger.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EngineDebuggerVirtual:
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
    impl EngineDebugger {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
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
        pub fn is_active(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("is_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "is_active" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn register_profiler(&mut self, name: StringName, profiler: Gd<EngineProfiler>) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("register_profiler");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3651669560i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "register_profiler" , 3651669560i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<EngineProfiler> as AsArg>::as_arg_ptr(&profiler),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn unregister_profiler(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("unregister_profiler");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "unregister_profiler" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_profiling(&mut self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("is_profiling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2041966384i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "is_profiling" , 2041966384i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_profiler(&mut self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("has_profiler");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2041966384i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "has_profiler" , 2041966384i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn profiler_add_frame_data(&mut self, name: StringName, data: VariantArray) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("profiler_add_frame_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1895267858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "profiler_add_frame_data" , 1895267858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <VariantArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn profiler_enable(&mut self, name: StringName, enable: bool, arguments: VariantArray) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("profiler_enable");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    438160728i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "profiler_enable" , 438160728i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <VariantArray as sys::GodotFfi>::sys_const(&arguments),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn register_message_capture(&mut self, name: StringName, callable: Callable) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("register_message_capture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1874754934i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "register_message_capture" , 1874754934i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn unregister_message_capture(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("unregister_message_capture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "unregister_message_capture" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_capture(&mut self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("has_capture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2041966384i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "has_capture" , 2041966384i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn send_message(&mut self, message: GodotString, data: VariantArray) {
            unsafe {
                let __class_name = StringName::from("EngineDebugger");
                let __method_name = StringName::from("send_message");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1209351045i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EngineDebugger" , "send_message" , 1209351045i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&message),
                    <VariantArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for EngineDebugger {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "EngineDebugger";
    }
    impl crate::obj::EngineClass for EngineDebugger {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for EngineDebugger {}
    impl std::ops::Deref for EngineDebugger {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EngineDebugger {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EngineDebugger {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EngineDebugger> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
