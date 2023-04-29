#![doc = "Sidecar module for class [`CanvasItem`][crate::engine::CanvasItem].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CanvasItem` enums](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `CanvasItem.`\n\nInherits [`Node`][crate::engine::Node].\n\nRelated symbols:\n\n* [`canvas_item`][crate::engine::canvas_item]: sidecar module with related enum/flag types\n* [`CanvasItemVirtual`][crate::engine::CanvasItemVirtual]: virtual methods\n* [`CanvasItemNotification`][crate::engine::notify::CanvasItemNotification]: notification type\n\n\nSee also [Godot docs for `CanvasItem`](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct CanvasItem {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`CanvasItem`][crate::engine::CanvasItem].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CanvasItem` methods](https://docs.godotengine.org/en/stable/classes/class_canvasitem.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait CanvasItemVirtual:
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
        fn on_notification(&mut self, what: CanvasItemNotification) {
            unimplemented!()
        }
        fn draw(&mut self) {
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
    #[doc = "Notification type for class [`CanvasItem`][crate::engine::CanvasItem]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum CanvasItemNotification {
        TransformChanged = 2000i32,
        LocalTransformChanged = 35i32,
        DrawOrNodeRecacheRequested = 30i32,
        VisibilityChangedOrNodeRecacheRequested = 31i32,
        EnterCanvas = 32i32,
        ExitCanvas = 33i32,
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
    impl From<i32> for CanvasItemNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                2000i32 => Self::TransformChanged,
                35i32 => Self::LocalTransformChanged,
                30i32 => Self::DrawOrNodeRecacheRequested,
                31i32 => Self::VisibilityChangedOrNodeRecacheRequested,
                32i32 => Self::EnterCanvas,
                33i32 => Self::ExitCanvas,
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
    impl From<CanvasItemNotification> for i32 {
        fn from(notification: CanvasItemNotification) -> i32 {
            match notification {
                CanvasItemNotification::TransformChanged => 2000i32,
                CanvasItemNotification::LocalTransformChanged => 35i32,
                CanvasItemNotification::DrawOrNodeRecacheRequested => 30i32,
                CanvasItemNotification::VisibilityChangedOrNodeRecacheRequested => 31i32,
                CanvasItemNotification::EnterCanvas => 32i32,
                CanvasItemNotification::ExitCanvas => 33i32,
                CanvasItemNotification::EnterTree => 10i32,
                CanvasItemNotification::ExitTree => 11i32,
                CanvasItemNotification::MovedInParent => 12i32,
                CanvasItemNotification::Ready => 13i32,
                CanvasItemNotification::Paused => 14i32,
                CanvasItemNotification::Unpaused => 15i32,
                CanvasItemNotification::PhysicsProcess => 16i32,
                CanvasItemNotification::Process => 17i32,
                CanvasItemNotification::Parented => 18i32,
                CanvasItemNotification::Unparented => 19i32,
                CanvasItemNotification::SceneInstantiated => 20i32,
                CanvasItemNotification::DragBegin => 21i32,
                CanvasItemNotification::DragEnd => 22i32,
                CanvasItemNotification::PathRenamed => 23i32,
                CanvasItemNotification::InternalProcess => 25i32,
                CanvasItemNotification::InternalPhysicsProcess => 26i32,
                CanvasItemNotification::PostEnterTree => 27i32,
                CanvasItemNotification::Disabled => 28i32,
                CanvasItemNotification::Enabled => 29i32,
                CanvasItemNotification::EditorPreSave => 9001i32,
                CanvasItemNotification::EditorPostSave => 9002i32,
                CanvasItemNotification::WmMouseEnter => 1002i32,
                CanvasItemNotification::WmMouseExit => 1003i32,
                CanvasItemNotification::WmWindowFocusIn => 1004i32,
                CanvasItemNotification::WmWindowFocusOut => 1005i32,
                CanvasItemNotification::WmCloseRequest => 1006i32,
                CanvasItemNotification::WmGoBackRequest => 1007i32,
                CanvasItemNotification::WmSizeChanged => 1008i32,
                CanvasItemNotification::WmDpiChange => 1009i32,
                CanvasItemNotification::VpMouseEnter => 1010i32,
                CanvasItemNotification::VpMouseExit => 1011i32,
                CanvasItemNotification::OsMemoryWarning => 2009i32,
                CanvasItemNotification::TranslationChanged => 2010i32,
                CanvasItemNotification::WmAbout => 2011i32,
                CanvasItemNotification::Crash => 2012i32,
                CanvasItemNotification::OsImeUpdate => 2013i32,
                CanvasItemNotification::ApplicationResumed => 2014i32,
                CanvasItemNotification::ApplicationPaused => 2015i32,
                CanvasItemNotification::ApplicationFocusIn => 2016i32,
                CanvasItemNotification::ApplicationFocusOut => 2017i32,
                CanvasItemNotification::TextServerChanged => 2018i32,
                CanvasItemNotification::Postinitialize => 0i32,
                CanvasItemNotification::Predelete => 1i32,
                CanvasItemNotification::Unknown(int) => int,
            }
        }
    }
    impl CanvasItem {
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
        pub fn notify(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: CanvasItemNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn get_canvas_item(&self) -> Rid {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_canvas_item");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2944877500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_canvas_item" , 2944877500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_visible(&mut self, visible: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_visible" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&visible)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_visible(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_visible" , 36873697i64);
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
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_visible_in_tree");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_visible_in_tree" , 36873697i64);
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
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("show");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "show" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn hide(&mut self) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("hide");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "hide" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn queue_redraw(&mut self) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("queue_redraw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "queue_redraw" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_to_front(&mut self) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("move_to_front");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "move_to_front" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_as_top_level(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_as_top_level");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_as_top_level" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_set_as_top_level(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_set_as_top_level");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_set_as_top_level" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_light_mask(&mut self, light_mask: i64) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_light_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_light_mask" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&light_mask)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_light_mask(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_light_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_light_mask" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_modulate(&mut self, modulate: Color) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_modulate" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&modulate)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_modulate(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_modulate" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_self_modulate(&mut self, self_modulate: Color) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_self_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_self_modulate" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&self_modulate)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_self_modulate(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_self_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_self_modulate" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_z_index(&mut self, z_index: i64) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_z_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_z_index" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&z_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_z_index(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_z_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_z_index" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_z_as_relative(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_z_as_relative");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_z_as_relative" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_z_relative(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_z_relative");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_z_relative" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_y_sort_enabled(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_y_sort_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_y_sort_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_y_sort_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_y_sort_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_y_sort_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_draw_behind_parent(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_draw_behind_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_draw_behind_parent" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_draw_behind_parent_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_draw_behind_parent_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_draw_behind_parent_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_line(
            &mut self,
            from: Vector2,
            to: Vector2,
            color: Color,
            width: f64,
            antialiased: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_line");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2516941890i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_line" , 2516941890i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&from),
                    <Vector2 as sys::GodotFfi>::sys_const(&to),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <bool as sys::GodotFfi>::sys_const(&antialiased),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_dashed_line(
            &mut self,
            from: Vector2,
            to: Vector2,
            color: Color,
            width: f64,
            dash: f64,
            aligned: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_dashed_line");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2175215884i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_dashed_line" , 2175215884i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&from),
                    <Vector2 as sys::GodotFfi>::sys_const(&to),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <f64 as sys::GodotFfi>::sys_const(&dash),
                    <bool as sys::GodotFfi>::sys_const(&aligned),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_polyline(
            &mut self,
            points: PackedVector2Array,
            color: Color,
            width: f64,
            antialiased: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_polyline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4175878946i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_polyline" , 4175878946i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <bool as sys::GodotFfi>::sys_const(&antialiased),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_polyline_colors(
            &mut self,
            points: PackedVector2Array,
            colors: PackedColorArray,
            width: f64,
            antialiased: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_polyline_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2239164197i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_polyline_colors" , 2239164197i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <bool as sys::GodotFfi>::sys_const(&antialiased),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_arc(
            &mut self,
            center: Vector2,
            radius: f64,
            start_angle: f64,
            end_angle: f64,
            point_count: i64,
            color: Color,
            width: f64,
            antialiased: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_arc");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3486841771i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_arc" , 3486841771i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&center),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                    <f64 as sys::GodotFfi>::sys_const(&start_angle),
                    <f64 as sys::GodotFfi>::sys_const(&end_angle),
                    <i64 as sys::GodotFfi>::sys_const(&point_count),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <bool as sys::GodotFfi>::sys_const(&antialiased),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_multiline(&mut self, points: PackedVector2Array, color: Color, width: f64) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_multiline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4230657331i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_multiline" , 4230657331i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_multiline_colors(
            &mut self,
            points: PackedVector2Array,
            colors: PackedColorArray,
            width: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_multiline_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    235933050i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_multiline_colors" , 235933050i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_rect(&mut self, rect: Rect2, color: Color, filled: bool, width: f64) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    84391229i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_rect" , 84391229i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <bool as sys::GodotFfi>::sys_const(&filled),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_circle(&mut self, position: Vector2, radius: f64, color: Color) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_circle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3063020269i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_circle" , 3063020269i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_texture(&mut self, texture: Gd<Texture2D>, position: Vector2, modulate: Color) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1695860435i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_texture" , 1695860435i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_texture_rect(
            &mut self,
            texture: Gd<Texture2D>,
            rect: Rect2,
            tile: bool,
            modulate: Color,
            transpose: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_texture_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3204081724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_texture_rect" , 3204081724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <bool as sys::GodotFfi>::sys_const(&tile),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_texture_rect_region(
            &mut self,
            texture: Gd<Texture2D>,
            rect: Rect2,
            src_rect: Rect2,
            modulate: Color,
            transpose: bool,
            clip_uv: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_texture_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3196597532i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_texture_rect_region" , 3196597532i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                    <bool as sys::GodotFfi>::sys_const(&clip_uv),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_msdf_texture_rect_region(
            &mut self,
            texture: Gd<Texture2D>,
            rect: Rect2,
            src_rect: Rect2,
            modulate: Color,
            outline: f64,
            pixel_range: f64,
            scale: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_msdf_texture_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2672026175i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_msdf_texture_rect_region" , 2672026175i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <f64 as sys::GodotFfi>::sys_const(&outline),
                    <f64 as sys::GodotFfi>::sys_const(&pixel_range),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_lcd_texture_rect_region(
            &mut self,
            texture: Gd<Texture2D>,
            rect: Rect2,
            src_rect: Rect2,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_lcd_texture_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    169610548i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_lcd_texture_rect_region" , 169610548i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_style_box(&mut self, style_box: Gd<StyleBox>, rect: Rect2) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_style_box");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    388176283i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_style_box" , 388176283i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<StyleBox> as AsArg>::as_arg_ptr(&style_box),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_primitive(
            &mut self,
            points: PackedVector2Array,
            colors: PackedColorArray,
            uvs: PackedVector2Array,
            texture: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_primitive");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2248678295i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_primitive" , 2248678295i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_polygon(
            &mut self,
            points: PackedVector2Array,
            colors: PackedColorArray,
            uvs: PackedVector2Array,
            texture: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2683625537i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_polygon" , 2683625537i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_colored_polygon(
            &mut self,
            points: PackedVector2Array,
            color: Color,
            uvs: PackedVector2Array,
            texture: Gd<Texture2D>,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_colored_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1659099617i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_colored_polygon" , 1659099617i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_string(
            &self,
            font: Gd<Font>,
            pos: Vector2,
            text: GodotString,
            alignment: global::HorizontalAlignment,
            width: f64,
            font_size: i64,
            modulate: Color,
            jst_flags: text_server::JustificationFlag,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_string");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2552080639i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_string" , 2552080639i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <global::HorizontalAlignment as sys::GodotFfi>::sys_const(&alignment),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_multiline_string(
            &self,
            font: Gd<Font>,
            pos: Vector2,
            text: GodotString,
            alignment: global::HorizontalAlignment,
            width: f64,
            font_size: i64,
            max_lines: i64,
            modulate: Color,
            brk_flags: text_server::LineBreakFlag,
            jst_flags: text_server::JustificationFlag,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_multiline_string");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4002645436i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_multiline_string" , 4002645436i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <global::HorizontalAlignment as sys::GodotFfi>::sys_const(&alignment),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <i64 as sys::GodotFfi>::sys_const(&max_lines),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <text_server::LineBreakFlag as sys::GodotFfi>::sys_const(&brk_flags),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_string_outline(
            &self,
            font: Gd<Font>,
            pos: Vector2,
            text: GodotString,
            alignment: global::HorizontalAlignment,
            width: f64,
            font_size: i64,
            size: i64,
            modulate: Color,
            jst_flags: text_server::JustificationFlag,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_string_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    850005221i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_string_outline" , 850005221i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <global::HorizontalAlignment as sys::GodotFfi>::sys_const(&alignment),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_multiline_string_outline(
            &self,
            font: Gd<Font>,
            pos: Vector2,
            text: GodotString,
            alignment: global::HorizontalAlignment,
            width: f64,
            font_size: i64,
            max_lines: i64,
            size: i64,
            modulate: Color,
            brk_flags: text_server::LineBreakFlag,
            jst_flags: text_server::JustificationFlag,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_multiline_string_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3717870722i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_multiline_string_outline" , 3717870722i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <global::HorizontalAlignment as sys::GodotFfi>::sys_const(&alignment),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <i64 as sys::GodotFfi>::sys_const(&max_lines),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <text_server::LineBreakFlag as sys::GodotFfi>::sys_const(&brk_flags),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_char(
            &self,
            font: Gd<Font>,
            pos: Vector2,
            char: GodotString,
            font_size: i64,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_char");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2329089032i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_char" , 2329089032i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <GodotString as sys::GodotFfi>::sys_const(&char),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_char_outline(
            &self,
            font: Gd<Font>,
            pos: Vector2,
            char: GodotString,
            font_size: i64,
            size: i64,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_char_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    419453826i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_char_outline" , 419453826i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <GodotString as sys::GodotFfi>::sys_const(&char),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_mesh(
            &mut self,
            mesh: Gd<Mesh>,
            texture: Gd<Texture2D>,
            transform: Transform2D,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1634855856i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_mesh" , 1634855856i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Mesh> as AsArg>::as_arg_ptr(&mesh),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_multimesh(&mut self, multimesh: Gd<MultiMesh>, texture: Gd<Texture2D>) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_multimesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    937992368i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_multimesh" , 937992368i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<MultiMesh> as AsArg>::as_arg_ptr(&multimesh),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_set_transform(&mut self, position: Vector2, rotation: f64, scale: Vector2) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4181505845i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_set_transform" , 4181505845i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <f64 as sys::GodotFfi>::sys_const(&rotation),
                    <Vector2 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_set_transform_matrix(&mut self, xform: Transform2D) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_set_transform_matrix");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2761652528i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_set_transform_matrix" , 2761652528i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Transform2D as sys::GodotFfi>::sys_const(&xform)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_animation_slice(
            &mut self,
            animation_length: f64,
            slice_begin: f64,
            slice_end: f64,
            offset: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_animation_slice");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2295343543i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_animation_slice" , 2295343543i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&animation_length),
                    <f64 as sys::GodotFfi>::sys_const(&slice_begin),
                    <f64 as sys::GodotFfi>::sys_const(&slice_end),
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_end_animation(&mut self) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("draw_end_animation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "draw_end_animation" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_transform(&self) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814499831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_transform" , 3814499831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_global_transform(&self) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_global_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814499831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_global_transform" , 3814499831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_global_transform_with_canvas(&self) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_global_transform_with_canvas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814499831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_global_transform_with_canvas" , 3814499831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_viewport_transform(&self) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_viewport_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814499831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_viewport_transform" , 3814499831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_viewport_rect(&self) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_viewport_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1639390495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_viewport_rect" , 1639390495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_canvas_transform(&self) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_canvas_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814499831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_canvas_transform" , 3814499831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_screen_transform(&self) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_screen_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814499831i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_screen_transform" , 3814499831i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_local_mouse_position(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_local_mouse_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_local_mouse_position" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_global_mouse_position(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_global_mouse_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_global_mouse_position" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_canvas(&self) -> Rid {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_canvas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2944877500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_canvas" , 2944877500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_world_2d(&self) -> Option<Gd<World2D>> {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_world_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2339128592i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_world_2d" , 2339128592i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<World2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_material(&mut self, material: Gd<Material>) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2757459619i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_material" , 2757459619i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Material> as AsArg>::as_arg_ptr(&material)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_material(&self) -> Option<Gd<Material>> {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    5934680i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_material" , 5934680i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Material>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_use_parent_material(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_use_parent_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_use_parent_material" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_use_parent_material(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_use_parent_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_use_parent_material" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_notify_local_transform(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_notify_local_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_notify_local_transform" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_local_transform_notification_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_local_transform_notification_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_local_transform_notification_enabled" , 36873697i64);
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
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_notify_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_notify_transform" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_transform_notification_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("is_transform_notification_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "is_transform_notification_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn force_update_transform(&mut self) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("force_update_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "force_update_transform" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn make_canvas_position_local(&self, screen_point: Vector2) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("make_canvas_position_local");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2656412154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "make_canvas_position_local" , 2656412154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&screen_point)];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn make_input_local(&self, event: Gd<InputEvent>) -> Option<Gd<InputEvent>> {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("make_input_local");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    811130057i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "make_input_local" , 811130057i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<InputEvent> as AsArg>::as_arg_ptr(&event)];
                let __args_ptr = __args.as_ptr();
                <Gd<InputEvent>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_visibility_layer(&mut self, layer: i64) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_visibility_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_visibility_layer" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_visibility_layer(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_visibility_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_visibility_layer" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_visibility_layer_bit(&mut self, layer: i64, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_visibility_layer_bit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_visibility_layer_bit" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_visibility_layer_bit(&self, layer: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_visibility_layer_bit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_visibility_layer_bit" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_filter(&mut self, mode: canvas_item::TextureFilter) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1037999706i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_texture_filter" , 1037999706i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<canvas_item::TextureFilter as sys::GodotFfi>::sys_const(
                    &mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_filter(&self) -> canvas_item::TextureFilter {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    121960042i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_texture_filter" , 121960042i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item::TextureFilter as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_repeat(&mut self, mode: canvas_item::TextureRepeat) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1716472974i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_texture_repeat" , 1716472974i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<canvas_item::TextureRepeat as sys::GodotFfi>::sys_const(
                    &mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_repeat(&self) -> canvas_item::TextureRepeat {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2667158319i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_texture_repeat" , 2667158319i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item::TextureRepeat as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_clip_children_mode(&mut self, mode: canvas_item::ClipChildrenMode) {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("set_clip_children_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1319393776i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "set_clip_children_mode" , 1319393776i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<canvas_item::ClipChildrenMode as sys::GodotFfi>::sys_const(
                    &mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_clip_children_mode(&self) -> canvas_item::ClipChildrenMode {
            unsafe {
                let __class_name = StringName::from("CanvasItem");
                let __method_name = StringName::from("get_clip_children_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3581808349i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CanvasItem" , "get_clip_children_mode" , 3581808349i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <canvas_item::ClipChildrenMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub(crate) const NOTIFICATION_TRANSFORM_CHANGED: i32 = 2000i32;
        pub(crate) const NOTIFICATION_LOCAL_TRANSFORM_CHANGED: i32 = 35i32;
        pub(crate) const NOTIFICATION_DRAW: i32 = 30i32;
        pub(crate) const NOTIFICATION_VISIBILITY_CHANGED: i32 = 31i32;
        pub(crate) const NOTIFICATION_ENTER_CANVAS: i32 = 32i32;
        pub(crate) const NOTIFICATION_EXIT_CANVAS: i32 = 33i32;
    }
    impl crate::obj::GodotClass for CanvasItem {
        type Base = crate::engine::Node;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "CanvasItem";
    }
    impl crate::obj::EngineClass for CanvasItem {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Node> for CanvasItem {}
    impl crate::obj::Inherits<crate::engine::Object> for CanvasItem {}
    impl std::ops::Deref for CanvasItem {
        type Target = crate::engine::Node;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for CanvasItem {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_CanvasItem {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::CanvasItem> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureFilter {
    ord: i32,
}
impl TextureFilter {
    pub const TEXTURE_FILTER_PARENT_NODE: Self = Self { ord: 0 };
    pub const TEXTURE_FILTER_NEAREST: Self = Self { ord: 1 };
    pub const TEXTURE_FILTER_LINEAR: Self = Self { ord: 2 };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self { ord: 3 };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self { ord: 4 };
    pub const TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC: Self = Self { ord: 5 };
    pub const TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC: Self = Self { ord: 6 };
    pub const TEXTURE_FILTER_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for TextureFilter {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TextureFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureRepeat {
    ord: i32,
}
impl TextureRepeat {
    pub const TEXTURE_REPEAT_PARENT_NODE: Self = Self { ord: 0 };
    pub const TEXTURE_REPEAT_DISABLED: Self = Self { ord: 1 };
    pub const TEXTURE_REPEAT_ENABLED: Self = Self { ord: 2 };
    pub const TEXTURE_REPEAT_MIRROR: Self = Self { ord: 3 };
    pub const TEXTURE_REPEAT_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for TextureRepeat {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TextureRepeat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ClipChildrenMode {
    ord: i32,
}
impl ClipChildrenMode {
    pub const CLIP_CHILDREN_DISABLED: Self = Self { ord: 0 };
    pub const CLIP_CHILDREN_ONLY: Self = Self { ord: 1 };
    pub const CLIP_CHILDREN_AND_DRAW: Self = Self { ord: 2 };
    pub const CLIP_CHILDREN_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ClipChildrenMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ClipChildrenMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
