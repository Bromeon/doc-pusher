#![doc = "Sidecar module for class [`NavigationPathQueryParameters3D`][crate::engine::NavigationPathQueryParameters3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `NavigationPathQueryParameters3D` enums](https://docs.godotengine.org/en/stable/classes/class_navigationpathqueryparameters3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `NavigationPathQueryParameters3D.`\n\nInherits [`RefCounted`][crate::engine::RefCounted].\n\nRelated symbols:\n\n* [`navigation_path_query_parameters_3d`][crate::engine::navigation_path_query_parameters_3d]: sidecar module with related enum/flag types\n* [`NavigationPathQueryParameters3DVirtual`][crate::engine::NavigationPathQueryParameters3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `NavigationPathQueryParameters3D`](https://docs.godotengine.org/en/stable/classes/class_navigationpathqueryparameters3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct NavigationPathQueryParameters3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`NavigationPathQueryParameters3D`][crate::engine::NavigationPathQueryParameters3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `NavigationPathQueryParameters3D` methods](https://docs.godotengine.org/en/stable/classes/class_navigationpathqueryparameters3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait NavigationPathQueryParameters3DVirtual:
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
    impl NavigationPathQueryParameters3D {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
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
        pub fn set_pathfinding_algorithm(
            &mut self,
            pathfinding_algorithm: navigation_path_query_parameters_3d::PathfindingAlgorithm,
        ) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_pathfinding_algorithm");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    394560454i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_pathfinding_algorithm" , 394560454i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< navigation_path_query_parameters_3d :: PathfindingAlgorithm as sys :: GodotFfi > :: sys_const (& pathfinding_algorithm)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_pathfinding_algorithm(
            &self,
        ) -> navigation_path_query_parameters_3d::PathfindingAlgorithm {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_pathfinding_algorithm");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3398491350i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_pathfinding_algorithm" , 3398491350i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < navigation_path_query_parameters_3d :: PathfindingAlgorithm as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_path_postprocessing(
            &mut self,
            path_postprocessing: navigation_path_query_parameters_3d::PathPostProcessing,
        ) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_path_postprocessing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2267362344i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_path_postprocessing" , 2267362344i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< navigation_path_query_parameters_3d :: PathPostProcessing as sys :: GodotFfi > :: sys_const (& path_postprocessing)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_path_postprocessing(
            &self,
        ) -> navigation_path_query_parameters_3d::PathPostProcessing {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_path_postprocessing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3883858360i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_path_postprocessing" , 3883858360i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < navigation_path_query_parameters_3d :: PathPostProcessing as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_map(&mut self, map: Rid) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_map");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_map" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&map)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_map(&self) -> Rid {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_map");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2944877500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_map" , 2944877500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_start_position(&mut self, start_position: Vector3) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_start_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_start_position" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&start_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_start_position(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_start_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_start_position" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_target_position(&mut self, target_position: Vector3) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_target_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_target_position" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&target_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_target_position(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_target_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_target_position" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_navigation_layers(&mut self, navigation_layers: i64) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_navigation_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_navigation_layers" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&navigation_layers)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_navigation_layers(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_navigation_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_navigation_layers" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_metadata_flags(
            &mut self,
            flags: navigation_path_query_parameters_3d::PathMetadataFlags,
        ) {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("set_metadata_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2713846708i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "set_metadata_flags" , 2713846708i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< navigation_path_query_parameters_3d :: PathMetadataFlags as sys :: GodotFfi > :: sys_const (& flags)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_metadata_flags(&self) -> navigation_path_query_parameters_3d::PathMetadataFlags {
            unsafe {
                let __class_name = StringName::from("NavigationPathQueryParameters3D");
                let __method_name = StringName::from("get_metadata_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1582332802i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "NavigationPathQueryParameters3D" , "get_metadata_flags" , 1582332802i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < navigation_path_query_parameters_3d :: PathMetadataFlags as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
    }
    impl crate::obj::GodotClass for NavigationPathQueryParameters3D {
        type Base = crate::engine::RefCounted;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "NavigationPathQueryParameters3D";
    }
    impl crate::obj::EngineClass for NavigationPathQueryParameters3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::RefCounted> for NavigationPathQueryParameters3D {}
    impl crate::obj::Inherits<crate::engine::Object> for NavigationPathQueryParameters3D {}
    impl std::ops::Deref for NavigationPathQueryParameters3D {
        type Target = crate::engine::RefCounted;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for NavigationPathQueryParameters3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_NavigationPathQueryParameters3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::NavigationPathQueryParameters3D>
                for $Class
            {
            }
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PathfindingAlgorithm {
    ord: i32,
}
impl PathfindingAlgorithm {
    pub const PATHFINDING_ALGORITHM_ASTAR: Self = Self { ord: 0 };
}
impl crate::obj::EngineEnum for PathfindingAlgorithm {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for PathfindingAlgorithm {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PathPostProcessing {
    ord: i32,
}
impl PathPostProcessing {
    pub const PATH_POSTPROCESSING_CORRIDORFUNNEL: Self = Self { ord: 0 };
    pub const PATH_POSTPROCESSING_EDGECENTERED: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for PathPostProcessing {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for PathPostProcessing {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct PathMetadataFlags {
    ord: i32,
}
impl PathMetadataFlags {
    pub const PATH_METADATA_INCLUDE_NONE: Self = Self { ord: 0 };
    pub const PATH_METADATA_INCLUDE_TYPES: Self = Self { ord: 1 };
    pub const PATH_METADATA_INCLUDE_RIDS: Self = Self { ord: 2 };
    pub const PATH_METADATA_INCLUDE_OWNERS: Self = Self { ord: 4 };
    pub const PATH_METADATA_INCLUDE_ALL: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for PathMetadataFlags {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for PathMetadataFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for PathMetadataFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
