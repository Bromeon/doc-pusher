#![doc = "Sidecar module for class [`Node3D`][crate::engine::Node3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Node3D` enums](https://docs.godotengine.org/en/stable/classes/class_node3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Node3D.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`node_3d`][crate::engine::node_3d]: sidecar module with related enum/flag types\n* [`Node3DVirtual`][crate::engine::Node3DVirtual]: virtual methods\n* [`Node3DNotification`][crate::engine::notify::Node3DNotification]: notification type\n\n\nSee also [Godot docs for `Node3D`](https://docs.godotengine.org/en/stable/classes/class_node3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Node3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Node3D`][crate::engine::Node3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Node3D` methods](https://docs.godotengine.org/en/stable/classes/class_node3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Node3DVirtual:
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
        fn on_notification(&mut self, what: Node3DNotification) {
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
    #[doc = "Notification type for class [`Node3D`][crate::engine::Node3D]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum Node3DNotification {
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
    impl From<i32> for Node3DNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
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
    impl From<Node3DNotification> for i32 {
        fn from(notification: Node3DNotification) -> i32 {
            match notification {
                Node3DNotification::TransformChanged => 2000i32,
                Node3DNotification::EnterWorld => 41i32,
                Node3DNotification::ExitWorld => 42i32,
                Node3DNotification::VisibilityChangedOrNodeRecacheRequested => 43i32,
                Node3DNotification::LocalTransformChanged => 44i32,
                Node3DNotification::EnterTree => 10i32,
                Node3DNotification::ExitTree => 11i32,
                Node3DNotification::MovedInParent => 12i32,
                Node3DNotification::Ready => 13i32,
                Node3DNotification::Paused => 14i32,
                Node3DNotification::Unpaused => 15i32,
                Node3DNotification::PhysicsProcess => 16i32,
                Node3DNotification::Process => 17i32,
                Node3DNotification::Parented => 18i32,
                Node3DNotification::Unparented => 19i32,
                Node3DNotification::SceneInstantiated => 20i32,
                Node3DNotification::DragBegin => 21i32,
                Node3DNotification::DragEnd => 22i32,
                Node3DNotification::PathRenamed => 23i32,
                Node3DNotification::InternalProcess => 25i32,
                Node3DNotification::InternalPhysicsProcess => 26i32,
                Node3DNotification::PostEnterTree => 27i32,
                Node3DNotification::Disabled => 28i32,
                Node3DNotification::Enabled => 29i32,
                Node3DNotification::EditorPreSave => 9001i32,
                Node3DNotification::EditorPostSave => 9002i32,
                Node3DNotification::WmMouseEnter => 1002i32,
                Node3DNotification::WmMouseExit => 1003i32,
                Node3DNotification::WmWindowFocusIn => 1004i32,
                Node3DNotification::WmWindowFocusOut => 1005i32,
                Node3DNotification::WmCloseRequest => 1006i32,
                Node3DNotification::WmGoBackRequest => 1007i32,
                Node3DNotification::WmSizeChanged => 1008i32,
                Node3DNotification::WmDpiChange => 1009i32,
                Node3DNotification::VpMouseEnter => 1010i32,
                Node3DNotification::VpMouseExit => 1011i32,
                Node3DNotification::OsMemoryWarning => 2009i32,
                Node3DNotification::TranslationChanged => 2010i32,
                Node3DNotification::WmAbout => 2011i32,
                Node3DNotification::Crash => 2012i32,
                Node3DNotification::OsImeUpdate => 2013i32,
                Node3DNotification::ApplicationResumed => 2014i32,
                Node3DNotification::ApplicationPaused => 2015i32,
                Node3DNotification::ApplicationFocusIn => 2016i32,
                Node3DNotification::ApplicationFocusOut => 2017i32,
                Node3DNotification::TextServerChanged => 2018i32,
                Node3DNotification::Postinitialize => 0i32,
                Node3DNotification::Predelete => 1i32,
                Node3DNotification::Unknown(int) => int,
            }
        }
    }
    impl Node3D {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Node3D");
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
        pub fn notify(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: Node3DNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn set_transform(&mut self, local: Transform3D) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2952846383i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_transform" , 2952846383i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Transform3D as sys::GodotFfi>::sys_const(&local)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transform(&self) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3229777777i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_transform" , 3229777777i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_position(&mut self, position: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_position" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_position(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_position" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_rotation(&mut self, euler_radians: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_rotation" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&euler_radians)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rotation(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_rotation" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_rotation_degrees(&mut self, euler_degrees: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_rotation_degrees");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_rotation_degrees" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&euler_degrees)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rotation_degrees(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_rotation_degrees");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_rotation_degrees" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_rotation_order(&mut self, order: global::EulerOrder) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_rotation_order");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1820889989i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_rotation_order" , 1820889989i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::EulerOrder as sys::GodotFfi>::sys_const(&order)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rotation_order(&self) -> global::EulerOrder {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_rotation_order");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    916939469i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_rotation_order" , 916939469i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::EulerOrder as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_rotation_edit_mode(&mut self, edit_mode: node_3d::RotationEditMode) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_rotation_edit_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    141483330i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_rotation_edit_mode" , 141483330i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<node_3d::RotationEditMode as sys::GodotFfi>::sys_const(
                    &edit_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rotation_edit_mode(&self) -> node_3d::RotationEditMode {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_rotation_edit_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1572188370i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_rotation_edit_mode" , 1572188370i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <node_3d::RotationEditMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_scale(&mut self, scale: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_scale" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_scale(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_scale" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_quaternion(&mut self, quaternion: Quaternion) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_quaternion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1727505552i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_quaternion" , 1727505552i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Quaternion as sys::GodotFfi>::sys_const(&quaternion)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_quaternion(&self) -> Quaternion {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_quaternion");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1222331677i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_quaternion" , 1222331677i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Quaternion as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_basis(&mut self, basis: Basis) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_basis");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1055510324i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_basis" , 1055510324i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Basis as sys::GodotFfi>::sys_const(&basis)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_basis(&self) -> Basis {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_basis");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2716978435i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_basis" , 2716978435i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Basis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_global_transform(&mut self, global: Transform3D) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_global_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2952846383i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_global_transform" , 2952846383i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Transform3D as sys::GodotFfi>::sys_const(&global)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_global_transform(&self) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_global_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3229777777i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_global_transform" , 3229777777i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_global_position(&mut self, position: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_global_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_global_position" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_global_position(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_global_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_global_position" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_global_rotation(&mut self, euler_radians: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_global_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_global_rotation" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&euler_radians)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_global_rotation(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_global_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_global_rotation" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_global_rotation_degrees(&mut self, euler_degrees: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_global_rotation_degrees");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_global_rotation_degrees" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&euler_degrees)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_global_rotation_degrees(&self) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_global_rotation_degrees");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3360562783i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_global_rotation_degrees" , 3360562783i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_parent_node_3d(&self) -> Option<Gd<Node3D>> {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_parent_node_3d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    151077316i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_parent_node_3d" , 151077316i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Node3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_ignore_transform_notification(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_ignore_transform_notification");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_ignore_transform_notification" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_as_top_level");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_as_top_level" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_set_as_top_level(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("is_set_as_top_level");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "is_set_as_top_level" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_disable_scale(&mut self, disable: bool) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_disable_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_disable_scale" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&disable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_scale_disabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("is_scale_disabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "is_scale_disabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_world_3d(&self) -> Option<Gd<World3D>> {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_world_3d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    317588385i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_world_3d" , 317588385i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<World3D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn force_update_transform(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("force_update_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "force_update_transform" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_visibility_parent(&mut self, path: NodePath) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_visibility_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1348162250i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_visibility_parent" , 1348162250i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_visibility_parent(&self) -> NodePath {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_visibility_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4075236667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_visibility_parent" , 4075236667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn update_gizmos(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("update_gizmos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "update_gizmos" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_gizmo(&mut self, gizmo: Gd<Node3DGizmo>) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("add_gizmo");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1544533845i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "add_gizmo" , 1544533845i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node3DGizmo> as AsArg>::as_arg_ptr(&gizmo)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_gizmos(&self) -> Array<Gd<Node3DGizmo>> {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("get_gizmos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "get_gizmos" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Node3DGizmo>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn clear_gizmos(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("clear_gizmos");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "clear_gizmos" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_subgizmo_selection(
            &mut self,
            gizmo: Gd<Node3DGizmo>,
            id: i64,
            transform: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_subgizmo_selection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3317607635i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_subgizmo_selection" , 3317607635i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Node3DGizmo> as AsArg>::as_arg_ptr(&gizmo),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_subgizmo_selection(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("clear_subgizmo_selection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "clear_subgizmo_selection" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_visible(&mut self, visible: bool) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_visible" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&visible)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_visible(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("is_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "is_visible" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_visible_in_tree(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("is_visible_in_tree");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "is_visible_in_tree" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn show(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("show");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "show" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn hide(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("hide");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "hide" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_notify_local_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_notify_local_transform" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_local_transform_notification_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("is_local_transform_notification_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "is_local_transform_notification_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_notify_transform(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_notify_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_notify_transform" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_transform_notification_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("is_transform_notification_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "is_transform_notification_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn rotate(&mut self, axis: Vector3, angle: f64) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("rotate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3436291937i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "rotate" , 3436291937i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&axis),
                    <f64 as sys::GodotFfi>::sys_const(&angle),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_rotate(&mut self, axis: Vector3, angle: f64) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("global_rotate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3436291937i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "global_rotate" , 3436291937i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&axis),
                    <f64 as sys::GodotFfi>::sys_const(&angle),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_scale(&mut self, scale: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("global_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "global_scale" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_translate(&mut self, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("global_translate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "global_translate" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rotate_object_local(&mut self, axis: Vector3, angle: f64) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("rotate_object_local");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3436291937i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "rotate_object_local" , 3436291937i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&axis),
                    <f64 as sys::GodotFfi>::sys_const(&angle),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn scale_object_local(&mut self, scale: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("scale_object_local");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "scale_object_local" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn translate_object_local(&mut self, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("translate_object_local");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "translate_object_local" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rotate_x(&mut self, angle: f64) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("rotate_x");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "rotate_x" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rotate_y(&mut self, angle: f64) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("rotate_y");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "rotate_y" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn rotate_z(&mut self, angle: f64) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("rotate_z");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "rotate_z" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&angle)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn translate(&mut self, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("translate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3460891852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "translate" , 3460891852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn orthonormalize(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("orthonormalize");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "orthonormalize" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_identity(&mut self) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("set_identity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "set_identity" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn look_at(&mut self, target: Vector3, up: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("look_at");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1002852006i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "look_at" , 1002852006i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&target),
                    <Vector3 as sys::GodotFfi>::sys_const(&up),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn look_at_from_position(&mut self, position: Vector3, target: Vector3, up: Vector3) {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("look_at_from_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    735115603i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "look_at_from_position" , 735115603i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&position),
                    <Vector3 as sys::GodotFfi>::sys_const(&target),
                    <Vector3 as sys::GodotFfi>::sys_const(&up),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn to_local(&self, global_point: Vector3) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("to_local");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    192990374i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "to_local" , 192990374i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&global_point)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn to_global(&self, local_point: Vector3) -> Vector3 {
            unsafe {
                let __class_name = StringName::from("Node3D");
                let __method_name = StringName::from("to_global");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    192990374i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Node3D" , "to_global" , 192990374i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector3 as sys::GodotFfi>::sys_const(&local_point)];
                let __args_ptr = __args.as_ptr();
                <Vector3 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_ENTER_WORLD: i32 = 41i32;
        pub(crate) const NOTIFICATION_EXIT_WORLD: i32 = 42i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 43i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 44i32;
    }
    impl crate::obj::GodotClass for Node3D {
        type Base = crate::engine::Node;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Node3D";
    }
    impl crate::obj::EngineClass for Node3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Node> for Node3D {}
    impl crate::obj::Inherits<crate::engine::Object> for Node3D {}
    impl std::ops::Deref for Node3D {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Node3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Node3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Node3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct RotationEditMode {
    ord: i32,
}
impl RotationEditMode {
    pub const ROTATION_EDIT_MODE_EULER: Self = Self { ord: 0 };
    pub const ROTATION_EDIT_MODE_QUATERNION: Self = Self { ord: 1 };
    pub const ROTATION_EDIT_MODE_BASIS: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for RotationEditMode {
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
unsafe impl sys::GodotFfi for RotationEditMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
