#![doc = "Sidecar module for class [`Skeleton3D`][crate::engine::Skeleton3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Skeleton3D` enums](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Skeleton3D.`\n\nInherits [`Node3D`][crate::engine::Node3D].\n\nRelated symbols:\n\n* [`Skeleton3DVirtual`][crate::engine::Skeleton3DVirtual]: virtual methods\n* [`Skeleton3DNotification`][crate::engine::notify::Skeleton3DNotification]: notification type\n\n\nSee also [Godot docs for `Skeleton3D`](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Skeleton3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Skeleton3D`][crate::engine::Skeleton3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Skeleton3D` methods](https://docs.godotengine.org/en/stable/classes/class_skeleton3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Skeleton3DVirtual:
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
        fn on_notification(&mut self, what: Skeleton3DNotification) {
            unimplemented!()
        }
        fn process(&mut self, delta: f64) {
            unimplemented!()
        }
        fn physics_process(&mut self, delta: f64) {
            unimplemented!()
        }
        fn enter_tree(&mut self) {
            unimplemented!()
        }
        fn exit_tree(&mut self) {
            unimplemented!()
        }
        fn ready(&mut self) {
            unimplemented!()
        }
        fn get_configuration_warnings(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn shortcut_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn unhandled_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
        fn unhandled_key_input(&mut self, event: Gd<InputEvent>) {
            unimplemented!()
        }
    }
    #[doc = "Notification type for class [`Skeleton3D`][crate::engine::Skeleton3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum Skeleton3DNotification {
        UpdateSkeleton = 50i32,
        TransformChanged = 2000i32,
        EnterWorld = 41i32,
        ExitWorld = 42i32,
        VisibilityChangedOrNodeRecacheRequested = 43i32,
        LocalTransformChanged = 44i32,
        EnterTree = 10i32,
        ExitTree = 11i32,
        MovedInParent = 12i32,
        Ready = 13i32,
        Paused = 14i32,
        Unpaused = 15i32,
        PhysicsProcess = 16i32,
        Process = 17i32,
        Parented = 18i32,
        Unparented = 19i32,
        SceneInstantiated = 20i32,
        DragBegin = 21i32,
        DragEnd = 22i32,
        PathRenamed = 23i32,
        InternalProcess = 25i32,
        InternalPhysicsProcess = 26i32,
        PostEnterTree = 27i32,
        Disabled = 28i32,
        Enabled = 29i32,
        EditorPreSave = 9001i32,
        EditorPostSave = 9002i32,
        WmMouseEnter = 1002i32,
        WmMouseExit = 1003i32,
        WmWindowFocusIn = 1004i32,
        WmWindowFocusOut = 1005i32,
        WmCloseRequest = 1006i32,
        WmGoBackRequest = 1007i32,
        WmSizeChanged = 1008i32,
        WmDpiChange = 1009i32,
        VpMouseEnter = 1010i32,
        VpMouseExit = 1011i32,
        OsMemoryWarning = 2009i32,
        TranslationChanged = 2010i32,
        WmAbout = 2011i32,
        Crash = 2012i32,
        OsImeUpdate = 2013i32,
        ApplicationResumed = 2014i32,
        ApplicationPaused = 2015i32,
        ApplicationFocusIn = 2016i32,
        ApplicationFocusOut = 2017i32,
        TextServerChanged = 2018i32,
        Postinitialize = 0i32,
        Predelete = 1i32,
        #[doc = r" Since Godot represents notifications as integers, it's always possible that a notification outside the known types"]
        #[doc = r" is received. For example, the user can manually issue notifications through `Object.notification()`."]
        Unknown(i32),
    }
    impl From<i32> for Skeleton3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                50i32 => Self::UpdateSkeleton,
                2000i32 => Self::TransformChanged,
                41i32 => Self::EnterWorld,
                42i32 => Self::ExitWorld,
                43i32 => Self::VisibilityChangedOrNodeRecacheRequested,
                44i32 => Self::LocalTransformChanged,
                10i32 => Self::EnterTree,
                11i32 => Self::ExitTree,
                12i32 => Self::MovedInParent,
                13i32 => Self::Ready,
                14i32 => Self::Paused,
                15i32 => Self::Unpaused,
                16i32 => Self::PhysicsProcess,
                17i32 => Self::Process,
                18i32 => Self::Parented,
                19i32 => Self::Unparented,
                20i32 => Self::SceneInstantiated,
                21i32 => Self::DragBegin,
                22i32 => Self::DragEnd,
                23i32 => Self::PathRenamed,
                25i32 => Self::InternalProcess,
                26i32 => Self::InternalPhysicsProcess,
                27i32 => Self::PostEnterTree,
                28i32 => Self::Disabled,
                29i32 => Self::Enabled,
                9001i32 => Self::EditorPreSave,
                9002i32 => Self::EditorPostSave,
                1002i32 => Self::WmMouseEnter,
                1003i32 => Self::WmMouseExit,
                1004i32 => Self::WmWindowFocusIn,
                1005i32 => Self::WmWindowFocusOut,
                1006i32 => Self::WmCloseRequest,
                1007i32 => Self::WmGoBackRequest,
                1008i32 => Self::WmSizeChanged,
                1009i32 => Self::WmDpiChange,
                1010i32 => Self::VpMouseEnter,
                1011i32 => Self::VpMouseExit,
                2009i32 => Self::OsMemoryWarning,
                2010i32 => Self::TranslationChanged,
                2011i32 => Self::WmAbout,
                2012i32 => Self::Crash,
                2013i32 => Self::OsImeUpdate,
                2014i32 => Self::ApplicationResumed,
                2015i32 => Self::ApplicationPaused,
                2016i32 => Self::ApplicationFocusIn,
                2017i32 => Self::ApplicationFocusOut,
                2018i32 => Self::TextServerChanged,
                0i32 => Self::Postinitialize,
                1i32 => Self::Predelete,
                other_int => Self::Unknown(other_int),
            }
        }
    }
    impl From<Skeleton3DNotification> for i32 {
        fn from(notification: Skeleton3DNotification) -> i32 {
            match notification {
                Skeleton3DNotification::UpdateSkeleton => 50i32,
                Skeleton3DNotification::TransformChanged => 2000i32,
                Skeleton3DNotification::EnterWorld => 41i32,
                Skeleton3DNotification::ExitWorld => 42i32,
                Skeleton3DNotification::VisibilityChangedOrNodeRecacheRequested => 43i32,
                Skeleton3DNotification::LocalTransformChanged => 44i32,
                Skeleton3DNotification::EnterTree => 10i32,
                Skeleton3DNotification::ExitTree => 11i32,
                Skeleton3DNotification::MovedInParent => 12i32,
                Skeleton3DNotification::Ready => 13i32,
                Skeleton3DNotification::Paused => 14i32,
                Skeleton3DNotification::Unpaused => 15i32,
                Skeleton3DNotification::PhysicsProcess => 16i32,
                Skeleton3DNotification::Process => 17i32,
                Skeleton3DNotification::Parented => 18i32,
                Skeleton3DNotification::Unparented => 19i32,
                Skeleton3DNotification::SceneInstantiated => 20i32,
                Skeleton3DNotification::DragBegin => 21i32,
                Skeleton3DNotification::DragEnd => 22i32,
                Skeleton3DNotification::PathRenamed => 23i32,
                Skeleton3DNotification::InternalProcess => 25i32,
                Skeleton3DNotification::InternalPhysicsProcess => 26i32,
                Skeleton3DNotification::PostEnterTree => 27i32,
                Skeleton3DNotification::Disabled => 28i32,
                Skeleton3DNotification::Enabled => 29i32,
                Skeleton3DNotification::EditorPreSave => 9001i32,
                Skeleton3DNotification::EditorPostSave => 9002i32,
                Skeleton3DNotification::WmMouseEnter => 1002i32,
                Skeleton3DNotification::WmMouseExit => 1003i32,
                Skeleton3DNotification::WmWindowFocusIn => 1004i32,
                Skeleton3DNotification::WmWindowFocusOut => 1005i32,
                Skeleton3DNotification::WmCloseRequest => 1006i32,
                Skeleton3DNotification::WmGoBackRequest => 1007i32,
                Skeleton3DNotification::WmSizeChanged => 1008i32,
                Skeleton3DNotification::WmDpiChange => 1009i32,
                Skeleton3DNotification::VpMouseEnter => 1010i32,
                Skeleton3DNotification::VpMouseExit => 1011i32,
                Skeleton3DNotification::OsMemoryWarning => 2009i32,
                Skeleton3DNotification::TranslationChanged => 2010i32,
                Skeleton3DNotification::WmAbout => 2011i32,
                Skeleton3DNotification::Crash => 2012i32,
                Skeleton3DNotification::OsImeUpdate => 2013i32,
                Skeleton3DNotification::ApplicationResumed => 2014i32,
                Skeleton3DNotification::ApplicationPaused => 2015i32,
                Skeleton3DNotification::ApplicationFocusIn => 2016i32,
                Skeleton3DNotification::ApplicationFocusOut => 2017i32,
                Skeleton3DNotification::TextServerChanged => 2018i32,
                Skeleton3DNotification::Postinitialize => 0i32,
                Skeleton3DNotification::Predelete => 1i32,
                Skeleton3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Skeleton3D {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
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
        pub fn notify(&mut self, what: Skeleton3DNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Skeleton3DNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn add_bone(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("add_bone");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "add_bone" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn find_bone(&self, name: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("find_bone");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321353865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "find_bone" , 1321353865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_bone_name(&self, bone_idx: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone_name(&mut self, bone_idx: i64, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    501894301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_name" , 501894301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_parent(&self, bone_idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_parent" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone_parent(&mut self, bone_idx: i64, parent_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_parent" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <i64 as sys::GodotFfi>::sys_const(&parent_idx),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_version(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_version");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_version" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn unparent_bone_and_rest(&mut self, bone_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("unparent_bone_and_rest");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "unparent_bone_and_rest" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_children(&self, bone_idx: i64) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_children");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1706082319i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_children" , 1706082319i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_parentless_bones(&self) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_parentless_bones");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1930428628i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_parentless_bones" , 1930428628i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_bone_rest(&self, bone_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_rest");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965739696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_rest" , 1965739696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone_rest(&mut self, bone_idx: i64, rest: Transform3D) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_rest");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3616898986i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_rest" , 3616898986i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <Transform3D as sys::GodotFfi>::sys_const(&rest),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_global_rest(&self, bone_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_global_rest");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965739696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_global_rest" , 1965739696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_skin_from_rest_transforms(&mut self) -> Option<Gd<Skin>> {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("create_skin_from_rest_transforms");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1032037385i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "create_skin_from_rest_transforms" , 1032037385i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Skin>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn register_skin(&mut self, skin: Gd<Skin>) -> Option<Gd<SkinReference>> {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("register_skin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3405789568i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "register_skin" , 3405789568i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Skin> as AsArg>::as_arg_ptr(&skin)];
                let __args_ptr = __args.as_ptr();
                <Gd<SkinReference>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn localize_rests(&mut self) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("localize_rests");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "localize_rests" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_bones(&mut self) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("clear_bones");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "clear_bones" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_pose(&self, bone_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_pose");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965739696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_pose" , 1965739696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone_pose_position(&mut self, bone_idx: i64, position: Vector3) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_pose_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1530502735i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_pose_position" , 1530502735i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <Vector3 as sys::GodotFfi>::sys_const(&position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_bone_pose_rotation(&mut self, bone_idx: i64, rotation: Quaternion) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_pose_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2823819782i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_pose_rotation" , 2823819782i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <Quaternion as sys::GodotFfi>::sys_const(&rotation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_bone_pose_scale(&mut self, bone_idx: i64, scale: Vector3) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_pose_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1530502735i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_pose_scale" , 1530502735i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <Vector3 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_pose_position(&self, bone_idx: i64) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_pose_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    711720468i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_pose_position" , 711720468i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_bone_pose_rotation(&self, bone_idx: i64) -> Quaternion {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_pose_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    476865136i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_pose_rotation" , 476865136i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_bone_pose_scale(&self, bone_idx: i64) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_pose_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    711720468i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_pose_scale" , 711720468i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn reset_bone_pose(&mut self, bone_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("reset_bone_pose");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "reset_bone_pose" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reset_bone_poses(&mut self) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("reset_bone_poses");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "reset_bone_poses" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_bone_enabled(&self, bone_idx: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("is_bone_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "is_bone_enabled" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_bone_enabled(&mut self, bone_idx: i64, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4023243586i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_enabled" , 4023243586i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_bones_global_pose_override(&mut self) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("clear_bones_global_pose_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "clear_bones_global_pose_override" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_bone_global_pose_override(
            &mut self,
            bone_idx: i64,
            pose: Transform3D,
            amount: f64,
            persistent: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_bone_global_pose_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3483398371i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_bone_global_pose_override" , 3483398371i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&bone_idx),
                    <Transform3D as sys::GodotFfi>::sys_const(&pose),
                    <f64 as sys::GodotFfi>::sys_const(&amount),
                    <bool as sys::GodotFfi>::sys_const(&persistent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_bone_global_pose_override(&self, bone_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_global_pose_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965739696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_global_pose_override" , 1965739696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_bone_global_pose(&self, bone_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_global_pose");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965739696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_global_pose" , 1965739696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_bone_global_pose_no_override(&self, bone_idx: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_bone_global_pose_no_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1965739696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_bone_global_pose_no_override" , 1965739696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn force_update_all_bone_transforms(&mut self) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("force_update_all_bone_transforms");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "force_update_all_bone_transforms" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn force_update_bone_child_transform(&mut self, bone_idx: i64) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("force_update_bone_child_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "force_update_bone_child_transform" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&bone_idx)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_motion_scale(&mut self, motion_scale: f64) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_motion_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_motion_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&motion_scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_motion_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_motion_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_motion_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_show_rest_only(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_show_rest_only");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_show_rest_only" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_show_rest_only(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("is_show_rest_only");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "is_show_rest_only" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_animate_physical_bones(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("set_animate_physical_bones");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "set_animate_physical_bones" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_animate_physical_bones(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("get_animate_physical_bones");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "get_animate_physical_bones" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn physical_bones_stop_simulation(&mut self) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("physical_bones_stop_simulation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "physical_bones_stop_simulation" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn physical_bones_start_simulation(&mut self, bones: Array<StringName>) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("physical_bones_start_simulation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2787316981i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "physical_bones_start_simulation" , 2787316981i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<StringName> as sys::GodotFfi>::sys_const(&bones)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn physical_bones_add_collision_exception(&mut self, exception: Rid) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("physical_bones_add_collision_exception");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "physical_bones_add_collision_exception" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&exception)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn physical_bones_remove_collision_exception(&mut self, exception: Rid) {
            unsafe {
                let __class_name = StringName::from("Skeleton3D");
                let __method_name = StringName::from("physical_bones_remove_collision_exception");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Skeleton3D" , "physical_bones_remove_collision_exception" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&exception)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub(crate) const NOTIFICATION_UPDATE_SKELETON: i32 = 50i32;
    }
    impl crate::obj::GodotClass for Skeleton3D {
        type Base = crate::engine::Node3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Skeleton3D";
    }
    impl crate::obj::EngineClass for Skeleton3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Node3D> for Skeleton3D {}
    impl crate::obj::Inherits<crate::engine::Node> for Skeleton3D {}
    impl crate::obj::Inherits<crate::engine::Object> for Skeleton3D {}
    impl std::ops::Deref for Skeleton3D {
        type Target = crate::engine::Node3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Skeleton3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Skeleton3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Skeleton3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
