#![doc = "Sidecar module for class [`Performance`][crate::engine::Performance].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Performance` enums](https://docs.godotengine.org/en/stable/classes/class_performance.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Performance.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`performance`][crate::engine::performance]: sidecar module with related enum/flag types\n* [`PerformanceVirtual`][crate::engine::PerformanceVirtual]: virtual methods\n\n\nSee also [Godot docs for `Performance`](https://docs.godotengine.org/en/stable/classes/class_performance.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Performance {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Performance`][crate::engine::Performance].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Performance` methods](https://docs.godotengine.org/en/stable/classes/class_performance.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PerformanceVirtual:
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
    impl Performance {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Performance");
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
        pub fn get_monitor(&self, monitor: performance::Monitor) -> f64 {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("get_monitor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1943275655i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "get_monitor" , 1943275655i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<performance::Monitor as sys::GodotFfi>::sys_const(&monitor)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_custom_monitor(
            &mut self,
            id: StringName,
            callable: Callable,
            arguments: VariantArray,
        ) {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("add_custom_monitor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2865980031i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "add_custom_monitor" , 2865980031i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&id),
                    <Callable as sys::GodotFfi>::sys_const(&callable),
                    <VariantArray as sys::GodotFfi>::sys_const(&arguments),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_custom_monitor(&mut self, id: StringName) {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("remove_custom_monitor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "remove_custom_monitor" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_custom_monitor(&mut self, id: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("has_custom_monitor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2041966384i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "has_custom_monitor" , 2041966384i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_custom_monitor(&mut self, id: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("get_custom_monitor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2138907829i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "get_custom_monitor" , 2138907829i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&id)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_monitor_modification_time(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("get_monitor_modification_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "get_monitor_modification_time" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_custom_monitor_names(&mut self) -> Array<StringName> {
            unsafe {
                let __class_name = StringName::from("Performance");
                let __method_name = StringName::from("get_custom_monitor_names");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Performance" , "get_custom_monitor_names" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<StringName> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Performance {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Performance";
    }
    impl crate::obj::EngineClass for Performance {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for Performance {}
    impl std::ops::Deref for Performance {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Performance {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Performance {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Performance> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Monitor {
    ord: i32,
}
impl Monitor {
    pub const TIME_FPS: Self = Self { ord: 0 };
    pub const TIME_PROCESS: Self = Self { ord: 1 };
    pub const TIME_PHYSICS_PROCESS: Self = Self { ord: 2 };
    pub const TIME_NAVIGATION_PROCESS: Self = Self { ord: 3 };
    pub const MEMORY_STATIC: Self = Self { ord: 4 };
    pub const MEMORY_STATIC_MAX: Self = Self { ord: 5 };
    pub const MEMORY_MESSAGE_BUFFER_MAX: Self = Self { ord: 6 };
    pub const OBJECT_COUNT: Self = Self { ord: 7 };
    pub const OBJECT_RESOURCE_COUNT: Self = Self { ord: 8 };
    pub const OBJECT_NODE_COUNT: Self = Self { ord: 9 };
    pub const OBJECT_ORPHAN_NODE_COUNT: Self = Self { ord: 10 };
    pub const RENDER_TOTAL_OBJECTS_IN_FRAME: Self = Self { ord: 11 };
    pub const RENDER_TOTAL_PRIMITIVES_IN_FRAME: Self = Self { ord: 12 };
    pub const RENDER_TOTAL_DRAW_CALLS_IN_FRAME: Self = Self { ord: 13 };
    pub const RENDER_VIDEO_MEM_USED: Self = Self { ord: 14 };
    pub const RENDER_TEXTURE_MEM_USED: Self = Self { ord: 15 };
    pub const RENDER_BUFFER_MEM_USED: Self = Self { ord: 16 };
    pub const PHYSICS_2D_ACTIVE_OBJECTS: Self = Self { ord: 17 };
    pub const PHYSICS_2D_COLLISION_PAIRS: Self = Self { ord: 18 };
    pub const PHYSICS_2D_ISLAND_COUNT: Self = Self { ord: 19 };
    pub const PHYSICS_3D_ACTIVE_OBJECTS: Self = Self { ord: 20 };
    pub const PHYSICS_3D_COLLISION_PAIRS: Self = Self { ord: 21 };
    pub const PHYSICS_3D_ISLAND_COUNT: Self = Self { ord: 22 };
    pub const AUDIO_OUTPUT_LATENCY: Self = Self { ord: 23 };
    pub const NAVIGATION_ACTIVE_MAPS: Self = Self { ord: 24 };
    pub const NAVIGATION_REGION_COUNT: Self = Self { ord: 25 };
    pub const NAVIGATION_AGENT_COUNT: Self = Self { ord: 26 };
    pub const NAVIGATION_LINK_COUNT: Self = Self { ord: 27 };
    pub const NAVIGATION_POLYGON_COUNT: Self = Self { ord: 28 };
    pub const NAVIGATION_EDGE_COUNT: Self = Self { ord: 29 };
    pub const NAVIGATION_EDGE_MERGE_COUNT: Self = Self { ord: 30 };
    pub const NAVIGATION_EDGE_CONNECTION_COUNT: Self = Self { ord: 31 };
    pub const NAVIGATION_EDGE_FREE_COUNT: Self = Self { ord: 32 };
    pub const MONITOR_MAX: Self = Self { ord: 33 };
}
impl crate::obj::EngineEnum for Monitor {
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
            | ord @ 28i32
            | ord @ 29i32
            | ord @ 30i32
            | ord @ 31i32
            | ord @ 32i32
            | ord @ 33i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Monitor {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
