#![doc = "Sidecar module for class [`Container`][crate::engine::Container].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Container` enums](https://docs.godotengine.org/en/stable/classes/class_container.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Container.`\n\nInherits [`Control`][crate::engine::Control].\n\nRelated symbols:\n\n* [`ContainerVirtual`][crate::engine::ContainerVirtual]: virtual methods\n* [`ContainerNotification`][crate::engine::notify::ContainerNotification]: notification type\n\n\nSee also [Godot docs for `Container`](https://docs.godotengine.org/en/stable/classes/class_container.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Container {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Container`][crate::engine::Container].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Container` methods](https://docs.godotengine.org/en/stable/classes/class_container.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ContainerVirtual:
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
        fn on_notification(&mut self, what: ContainerNotification) {
            unimplemented!()
        }
        fn get_allowed_size_flags_horizontal(&self) -> PackedInt32Array {
            unimplemented!()
        }
        fn get_allowed_size_flags_vertical(&self) -> PackedInt32Array {
            unimplemented!()
        }
        fn has_point(&self, point: Vector2) -> bool {
            unimplemented!()
        }
        fn structured_text_parser(&self, args: VariantArray, text: GodotString) -> Array<Vector3i> {
            unimplemented!()
        }
        fn get_minimum_size(&self) -> Vector2 {
            unimplemented!()
        }
        fn get_drag_data(&self, at_position: Vector2) -> Variant {
            unimplemented!()
        }
        fn can_drop_data(&self, at_position: Vector2, data: Variant) -> bool {
            unimplemented!()
        }
        fn drop_data(&mut self, at_position: Vector2, data: Variant) {
            unimplemented!()
        }
        fn make_custom_tooltip(&self, for_text: GodotString) -> Option<Gd<Object>> {
            unimplemented!()
        }
        fn gui_input(&mut self, event: Gd<InputEvent>) {
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
    #[doc = "Notification type for class [`Container`][crate::engine::Container]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum ContainerNotification {
        PreSortChildren = 50i32,
        SortChildren = 51i32,
        Resized = 40i32,
        MouseEnter = 41i32,
        MouseExit = 42i32,
        FocusEnter = 43i32,
        FocusExit = 44i32,
        ThemeChanged = 45i32,
        ScrollBegin = 47i32,
        ScrollEnd = 48i32,
        LayoutDirectionChanged = 49i32,
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
    impl From<i32> for ContainerNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
                50i32 => Self::PreSortChildren,
                51i32 => Self::SortChildren,
                40i32 => Self::Resized,
                41i32 => Self::MouseEnter,
                42i32 => Self::MouseExit,
                43i32 => Self::FocusEnter,
                44i32 => Self::FocusExit,
                45i32 => Self::ThemeChanged,
                47i32 => Self::ScrollBegin,
                48i32 => Self::ScrollEnd,
                49i32 => Self::LayoutDirectionChanged,
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
    impl From<ContainerNotification> for i32 {
        fn from(notification: ContainerNotification) -> i32 {
            match notification {
                ContainerNotification::PreSortChildren => 50i32,
                ContainerNotification::SortChildren => 51i32,
                ContainerNotification::Resized => 40i32,
                ContainerNotification::MouseEnter => 41i32,
                ContainerNotification::MouseExit => 42i32,
                ContainerNotification::FocusEnter => 43i32,
                ContainerNotification::FocusExit => 44i32,
                ContainerNotification::ThemeChanged => 45i32,
                ContainerNotification::ScrollBegin => 47i32,
                ContainerNotification::ScrollEnd => 48i32,
                ContainerNotification::LayoutDirectionChanged => 49i32,
                ContainerNotification::TransformChanged => 2000i32,
                ContainerNotification::LocalTransformChanged => 35i32,
                ContainerNotification::DrawOrNodeRecacheRequested => 30i32,
                ContainerNotification::VisibilityChangedOrNodeRecacheRequested => 31i32,
                ContainerNotification::EnterCanvas => 32i32,
                ContainerNotification::ExitCanvas => 33i32,
                ContainerNotification::EnterTree => 10i32,
                ContainerNotification::ExitTree => 11i32,
                ContainerNotification::MovedInParent => 12i32,
                ContainerNotification::Ready => 13i32,
                ContainerNotification::Paused => 14i32,
                ContainerNotification::Unpaused => 15i32,
                ContainerNotification::PhysicsProcess => 16i32,
                ContainerNotification::Process => 17i32,
                ContainerNotification::Parented => 18i32,
                ContainerNotification::Unparented => 19i32,
                ContainerNotification::SceneInstantiated => 20i32,
                ContainerNotification::DragBegin => 21i32,
                ContainerNotification::DragEnd => 22i32,
                ContainerNotification::PathRenamed => 23i32,
                ContainerNotification::InternalProcess => 25i32,
                ContainerNotification::InternalPhysicsProcess => 26i32,
                ContainerNotification::PostEnterTree => 27i32,
                ContainerNotification::Disabled => 28i32,
                ContainerNotification::Enabled => 29i32,
                ContainerNotification::EditorPreSave => 9001i32,
                ContainerNotification::EditorPostSave => 9002i32,
                ContainerNotification::WmMouseEnter => 1002i32,
                ContainerNotification::WmMouseExit => 1003i32,
                ContainerNotification::WmWindowFocusIn => 1004i32,
                ContainerNotification::WmWindowFocusOut => 1005i32,
                ContainerNotification::WmCloseRequest => 1006i32,
                ContainerNotification::WmGoBackRequest => 1007i32,
                ContainerNotification::WmSizeChanged => 1008i32,
                ContainerNotification::WmDpiChange => 1009i32,
                ContainerNotification::VpMouseEnter => 1010i32,
                ContainerNotification::VpMouseExit => 1011i32,
                ContainerNotification::OsMemoryWarning => 2009i32,
                ContainerNotification::TranslationChanged => 2010i32,
                ContainerNotification::WmAbout => 2011i32,
                ContainerNotification::Crash => 2012i32,
                ContainerNotification::OsImeUpdate => 2013i32,
                ContainerNotification::ApplicationResumed => 2014i32,
                ContainerNotification::ApplicationPaused => 2015i32,
                ContainerNotification::ApplicationFocusIn => 2016i32,
                ContainerNotification::ApplicationFocusOut => 2017i32,
                ContainerNotification::TextServerChanged => 2018i32,
                ContainerNotification::Postinitialize => 0i32,
                ContainerNotification::Predelete => 1i32,
                ContainerNotification::Unknown(int) => int,
            }
        }
    }
    impl Container {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Container");
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
        pub fn notify(&mut self, what: ContainerNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ContainerNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn queue_sort(&mut self) {
            unsafe {
                let __class_name = StringName::from("Container");
                let __method_name = StringName::from("queue_sort");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Container" , "queue_sort" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fit_child_in_rect(&mut self, child: Gd<Control>, rect: Rect2) {
            unsafe {
                let __class_name = StringName::from("Container");
                let __method_name = StringName::from("fit_child_in_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1993438598i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Container" , "fit_child_in_rect" , 1993438598i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Control> as AsArg>::as_arg_ptr(&child),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub(crate) const NOTIFICATION_PRE_SORT_CHILDREN: i32 = 50i32;
        pub(crate) const NOTIFICATION_SORT_CHILDREN: i32 = 51i32;
    }
    impl crate::obj::GodotClass for Container {
        type Base = crate::engine::Control;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Container";
    }
    impl crate::obj::EngineClass for Container {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Control> for Container {}
    impl crate::obj::Inherits<crate::engine::CanvasItem> for Container {}
    impl crate::obj::Inherits<crate::engine::Node> for Container {}
    impl crate::obj::Inherits<crate::engine::Object> for Container {}
    impl std::ops::Deref for Container {
        type Target = crate::engine::Control;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Container {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Container {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Container> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Control> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::CanvasItem> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
