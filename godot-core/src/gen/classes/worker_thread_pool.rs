#![doc = "Sidecar module for class [`WorkerThreadPool`][crate::engine::WorkerThreadPool].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `WorkerThreadPool` enums](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `WorkerThreadPool.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`WorkerThreadPoolVirtual`][crate::engine::WorkerThreadPoolVirtual]: virtual methods\n\n\nSee also [Godot docs for `WorkerThreadPool`](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct WorkerThreadPool {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`WorkerThreadPool`][crate::engine::WorkerThreadPool].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `WorkerThreadPool` methods](https://docs.godotengine.org/en/stable/classes/class_workerthreadpool.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait WorkerThreadPoolVirtual:
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
    impl WorkerThreadPool {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
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
        pub fn add_task(
            &mut self,
            action: Callable,
            high_priority: bool,
            description: GodotString,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("add_task");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3976347598i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "add_task" , 3976347598i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&action),
                    <bool as sys::GodotFfi>::sys_const(&high_priority),
                    <GodotString as sys::GodotFfi>::sys_const(&description),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_task_completed(&self, task_id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("is_task_completed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "is_task_completed" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&task_id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn wait_for_task_completion(&mut self, task_id: i64) {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("wait_for_task_completion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "wait_for_task_completion" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&task_id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_group_task(
            &mut self,
            action: Callable,
            elements: i64,
            tasks_needed: i64,
            high_priority: bool,
            description: GodotString,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("add_group_task");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2377228549i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "add_group_task" , 2377228549i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&action),
                    <i64 as sys::GodotFfi>::sys_const(&elements),
                    <i64 as sys::GodotFfi>::sys_const(&tasks_needed),
                    <bool as sys::GodotFfi>::sys_const(&high_priority),
                    <GodotString as sys::GodotFfi>::sys_const(&description),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_group_task_completed(&self, group_id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("is_group_task_completed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "is_group_task_completed" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&group_id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_group_processed_element_count(&self, group_id: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("get_group_processed_element_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "get_group_processed_element_count" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&group_id)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn wait_for_group_task_completion(&mut self, group_id: i64) {
            unsafe {
                let __class_name = StringName::from("WorkerThreadPool");
                let __method_name = StringName::from("wait_for_group_task_completion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "WorkerThreadPool" , "wait_for_group_task_completion" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&group_id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for WorkerThreadPool {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "WorkerThreadPool";
    }
    impl crate::obj::EngineClass for WorkerThreadPool {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for WorkerThreadPool {}
    impl std::ops::Deref for WorkerThreadPool {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for WorkerThreadPool {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_WorkerThreadPool {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::WorkerThreadPool> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
