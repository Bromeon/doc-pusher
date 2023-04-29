#![doc = "Sidecar module for class [`Control`][crate::engine::Control].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Control` enums](https://docs.godotengine.org/en/stable/classes/class_control.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Control.`\n\nInherits [`CanvasItem`][crate::engine::CanvasItem].\n\nRelated symbols:\n\n* [`control`][crate::engine::control]: sidecar module with related enum/flag types\n* [`ControlVirtual`][crate::engine::ControlVirtual]: virtual methods\n* [`ControlNotification`][crate::engine::notify::ControlNotification]: notification type\n\n\nSee also [Godot docs for `Control`](https://docs.godotengine.org/en/stable/classes/class_control.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Control {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Control`][crate::engine::Control].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Control` methods](https://docs.godotengine.org/en/stable/classes/class_control.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ControlVirtual:
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
        fn on_notification(&mut self, what: ControlNotification) {
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
    #[doc = "Notification type for class [`Control`][crate::engine::Control]."]
    #[doc = r""]
    #[doc = r" Makes it easier to keep an overview all possible notification variants for a given class, including"]
    #[doc = r" notifications defined in base classes."]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    #[repr(i32)]
    pub enum ControlNotification {
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
    impl From<i32> for ControlNotification {
        #[doc = r" Always succeeds, mapping unknown integers to the `Unknown` variant."]
        fn from(enumerator: i32) -> Self {
            match enumerator {
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
    impl From<ControlNotification> for i32 {
        fn from(notification: ControlNotification) -> i32 {
            match notification {
                ControlNotification::Resized => 40i32,
                ControlNotification::MouseEnter => 41i32,
                ControlNotification::MouseExit => 42i32,
                ControlNotification::FocusEnter => 43i32,
                ControlNotification::FocusExit => 44i32,
                ControlNotification::ThemeChanged => 45i32,
                ControlNotification::ScrollBegin => 47i32,
                ControlNotification::ScrollEnd => 48i32,
                ControlNotification::LayoutDirectionChanged => 49i32,
                ControlNotification::TransformChanged => 2000i32,
                ControlNotification::LocalTransformChanged => 35i32,
                ControlNotification::DrawOrNodeRecacheRequested => 30i32,
                ControlNotification::VisibilityChangedOrNodeRecacheRequested => 31i32,
                ControlNotification::EnterCanvas => 32i32,
                ControlNotification::ExitCanvas => 33i32,
                ControlNotification::EnterTree => 10i32,
                ControlNotification::ExitTree => 11i32,
                ControlNotification::MovedInParent => 12i32,
                ControlNotification::Ready => 13i32,
                ControlNotification::Paused => 14i32,
                ControlNotification::Unpaused => 15i32,
                ControlNotification::PhysicsProcess => 16i32,
                ControlNotification::Process => 17i32,
                ControlNotification::Parented => 18i32,
                ControlNotification::Unparented => 19i32,
                ControlNotification::SceneInstantiated => 20i32,
                ControlNotification::DragBegin => 21i32,
                ControlNotification::DragEnd => 22i32,
                ControlNotification::PathRenamed => 23i32,
                ControlNotification::InternalProcess => 25i32,
                ControlNotification::InternalPhysicsProcess => 26i32,
                ControlNotification::PostEnterTree => 27i32,
                ControlNotification::Disabled => 28i32,
                ControlNotification::Enabled => 29i32,
                ControlNotification::EditorPreSave => 9001i32,
                ControlNotification::EditorPostSave => 9002i32,
                ControlNotification::WmMouseEnter => 1002i32,
                ControlNotification::WmMouseExit => 1003i32,
                ControlNotification::WmWindowFocusIn => 1004i32,
                ControlNotification::WmWindowFocusOut => 1005i32,
                ControlNotification::WmCloseRequest => 1006i32,
                ControlNotification::WmGoBackRequest => 1007i32,
                ControlNotification::WmSizeChanged => 1008i32,
                ControlNotification::WmDpiChange => 1009i32,
                ControlNotification::VpMouseEnter => 1010i32,
                ControlNotification::VpMouseExit => 1011i32,
                ControlNotification::OsMemoryWarning => 2009i32,
                ControlNotification::TranslationChanged => 2010i32,
                ControlNotification::WmAbout => 2011i32,
                ControlNotification::Crash => 2012i32,
                ControlNotification::OsImeUpdate => 2013i32,
                ControlNotification::ApplicationResumed => 2014i32,
                ControlNotification::ApplicationPaused => 2015i32,
                ControlNotification::ApplicationFocusIn => 2016i32,
                ControlNotification::ApplicationFocusOut => 2017i32,
                ControlNotification::TextServerChanged => 2018i32,
                ControlNotification::Postinitialize => 0i32,
                ControlNotification::Predelete => 1i32,
                ControlNotification::Unknown(int) => int,
            }
        }
    }
    impl Control {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Control");
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
        pub fn notify(&mut self, what: ControlNotification) {
            self.notification(i32::from(what) as i64, false);
        }
        #[doc = r" ⚠️ Like [`Self::notify()`], but starts at the most-derived class and goes up the hierarchy."]
        #[doc = r""]
        #[doc = r" See docs of that method, including the panics."]
        pub fn notify_reversed(&mut self, what: ControlNotification) {
            self.notification(i32::from(what) as i64, true);
        }
        pub fn accept_event(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("accept_event");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "accept_event" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_minimum_size(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_minimum_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_minimum_size" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_combined_minimum_size(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_combined_minimum_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_combined_minimum_size" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_anchors_preset(&mut self, preset: control::LayoutPreset, keep_offsets: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_anchors_preset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    509135270i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_anchors_preset" , 509135270i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <control::LayoutPreset as sys::GodotFfi>::sys_const(&preset),
                    <bool as sys::GodotFfi>::sys_const(&keep_offsets),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_offsets_preset(
            &mut self,
            preset: control::LayoutPreset,
            resize_mode: control::LayoutPresetMode,
            margin: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_offsets_preset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3651818904i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_offsets_preset" , 3651818904i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <control::LayoutPreset as sys::GodotFfi>::sys_const(&preset),
                    <control::LayoutPresetMode as sys::GodotFfi>::sys_const(&resize_mode),
                    <i64 as sys::GodotFfi>::sys_const(&margin),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_anchors_and_offsets_preset(
            &mut self,
            preset: control::LayoutPreset,
            resize_mode: control::LayoutPresetMode,
            margin: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_anchors_and_offsets_preset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3651818904i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_anchors_and_offsets_preset" , 3651818904i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <control::LayoutPreset as sys::GodotFfi>::sys_const(&preset),
                    <control::LayoutPresetMode as sys::GodotFfi>::sys_const(&resize_mode),
                    <i64 as sys::GodotFfi>::sys_const(&margin),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_anchor(
            &mut self,
            side: global::Side,
            anchor: f64,
            keep_offset: bool,
            push_opposite_anchor: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_anchor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2589937826i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_anchor" , 2589937826i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <global::Side as sys::GodotFfi>::sys_const(&side),
                    <f64 as sys::GodotFfi>::sys_const(&anchor),
                    <bool as sys::GodotFfi>::sys_const(&keep_offset),
                    <bool as sys::GodotFfi>::sys_const(&push_opposite_anchor),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_anchor(&self, side: global::Side) -> f64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_anchor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2869120046i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_anchor" , 2869120046i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::Side as sys::GodotFfi>::sys_const(&side)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_offset(&mut self, side: global::Side, offset: f64) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4290182280i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_offset" , 4290182280i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <global::Side as sys::GodotFfi>::sys_const(&side),
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_offset(&self, offset: global::Side) -> f64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2869120046i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_offset" , 2869120046i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::Side as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_anchor_and_offset(
            &mut self,
            side: global::Side,
            anchor: f64,
            offset: f64,
            push_opposite_anchor: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_anchor_and_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4031722181i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_anchor_and_offset" , 4031722181i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <global::Side as sys::GodotFfi>::sys_const(&side),
                    <f64 as sys::GodotFfi>::sys_const(&anchor),
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                    <bool as sys::GodotFfi>::sys_const(&push_opposite_anchor),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_begin(&mut self, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_begin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_begin" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_end(&mut self, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_end");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_end" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_position(&mut self, position: Vector2, keep_offsets: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2436320129i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_position" , 2436320129i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <bool as sys::GodotFfi>::sys_const(&keep_offsets),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_size(&mut self, size: Vector2, keep_offsets: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2436320129i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_size" , 2436320129i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&size),
                    <bool as sys::GodotFfi>::sys_const(&keep_offsets),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reset_size(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("reset_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "reset_size" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_custom_minimum_size(&mut self, size: Vector2) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_custom_minimum_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_custom_minimum_size" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_global_position(&mut self, position: Vector2, keep_offsets: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_global_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2436320129i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_global_position" , 2436320129i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2 as sys::GodotFfi>::sys_const(&position),
                    <bool as sys::GodotFfi>::sys_const(&keep_offsets),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_rotation(&mut self, radians: f64) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_rotation" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&radians)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_rotation_degrees(&mut self, degrees: f64) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_rotation_degrees");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_rotation_degrees" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&degrees)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_scale(&mut self, scale: Vector2) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_scale" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&scale)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_pivot_offset(&mut self, pivot_offset: Vector2) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_pivot_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_pivot_offset" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&pivot_offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_begin(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_begin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_begin" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_end(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_end");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_end" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_position(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_position" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_size(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_size" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_rotation(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_rotation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_rotation" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_rotation_degrees(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_rotation_degrees");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_rotation_degrees" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_scale(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_scale" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_pivot_offset(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_pivot_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_pivot_offset" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_custom_minimum_size(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_custom_minimum_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_custom_minimum_size" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_parent_area_size(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_parent_area_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_parent_area_size" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_global_position(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_global_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_global_position" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_screen_position(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_screen_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_screen_position" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_rect(&self) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1639390495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_rect" , 1639390495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_global_rect(&self) -> Rect2 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_global_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1639390495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_global_rect" , 1639390495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rect2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_focus_mode(&mut self, mode: control::FocusMode) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_focus_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3232914922i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_focus_mode" , 3232914922i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::FocusMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_focus_mode(&self) -> control::FocusMode {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_focus_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2132829277i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_focus_mode" , 2132829277i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::FocusMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_focus(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_focus" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn grab_focus(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("grab_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "grab_focus" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn release_focus(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("release_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "release_focus" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn find_prev_valid_focus(&self) -> Option<Gd<Control>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("find_prev_valid_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2783021301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "find_prev_valid_focus" , 2783021301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Control>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn find_next_valid_focus(&self) -> Option<Gd<Control>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("find_next_valid_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2783021301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "find_next_valid_focus" , 2783021301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Control>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_h_size_flags(&mut self, flags: control::SizeFlags) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_h_size_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    394851643i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_h_size_flags" , 394851643i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::SizeFlags as sys::GodotFfi>::sys_const(&flags)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_h_size_flags(&self) -> control::SizeFlags {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_h_size_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3781367401i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_h_size_flags" , 3781367401i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::SizeFlags as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_stretch_ratio(&mut self, ratio: f64) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_stretch_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_stretch_ratio" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&ratio)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_stretch_ratio(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_stretch_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_stretch_ratio" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_v_size_flags(&mut self, flags: control::SizeFlags) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_v_size_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    394851643i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_v_size_flags" , 394851643i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::SizeFlags as sys::GodotFfi>::sys_const(&flags)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_v_size_flags(&self) -> control::SizeFlags {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_v_size_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3781367401i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_v_size_flags" , 3781367401i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::SizeFlags as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_theme(&mut self, theme: Gd<Theme>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_theme");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2326690814i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_theme" , 2326690814i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Theme> as AsArg>::as_arg_ptr(&theme)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_theme(&self) -> Option<Gd<Theme>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3846893731i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme" , 3846893731i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Theme>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_theme_type_variation(&mut self, theme_type: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_theme_type_variation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_theme_type_variation" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&theme_type)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_theme_type_variation(&self) -> StringName {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_type_variation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2002593661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_type_variation" , 2002593661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn begin_bulk_theme_override(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("begin_bulk_theme_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "begin_bulk_theme_override" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn end_bulk_theme_override(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("end_bulk_theme_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "end_bulk_theme_override" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_theme_icon_override(&mut self, name: StringName, texture: Gd<Texture2D>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("add_theme_icon_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1373065600i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "add_theme_icon_override" , 1373065600i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_theme_stylebox_override(&mut self, name: StringName, stylebox: Gd<StyleBox>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("add_theme_stylebox_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4188838905i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "add_theme_stylebox_override" , 4188838905i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<StyleBox> as AsArg>::as_arg_ptr(&stylebox),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_theme_font_override(&mut self, name: StringName, font: Gd<Font>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("add_theme_font_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3518018674i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "add_theme_font_override" , 3518018674i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<Font> as AsArg>::as_arg_ptr(&font),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_theme_font_size_override(&mut self, name: StringName, font_size: i64) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("add_theme_font_size_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2415702435i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "add_theme_font_size_override" , 2415702435i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_theme_color_override(&mut self, name: StringName, color: Color) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("add_theme_color_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4260178595i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "add_theme_color_override" , 4260178595i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_theme_constant_override(&mut self, name: StringName, constant: i64) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("add_theme_constant_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2415702435i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "add_theme_constant_override" , 2415702435i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <i64 as sys::GodotFfi>::sys_const(&constant),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_theme_icon_override(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("remove_theme_icon_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "remove_theme_icon_override" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_theme_stylebox_override(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("remove_theme_stylebox_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "remove_theme_stylebox_override" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_theme_font_override(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("remove_theme_font_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "remove_theme_font_override" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_theme_font_size_override(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("remove_theme_font_size_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "remove_theme_font_size_override" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_theme_color_override(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("remove_theme_color_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "remove_theme_color_override" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_theme_constant_override(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("remove_theme_constant_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "remove_theme_constant_override" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_theme_icon(
            &self,
            name: StringName,
            theme_type: StringName,
        ) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2336455395i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_icon" , 2336455395i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_stylebox(
            &self,
            name: StringName,
            theme_type: StringName,
        ) -> Option<Gd<StyleBox>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2759935355i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_stylebox" , 2759935355i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<StyleBox>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_font(&self, name: StringName, theme_type: StringName) -> Option<Gd<Font>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    387378635i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_font" , 387378635i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Font>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_font_size(&self, name: StringName, theme_type: StringName) -> i64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    229578101i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_font_size" , 229578101i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_color(&self, name: StringName, theme_type: StringName) -> Color {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2377051548i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_color" , 2377051548i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_constant(&self, name: StringName, theme_type: StringName) -> i64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    229578101i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_constant" , 229578101i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_icon_override(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_icon_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_icon_override" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_stylebox_override(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_stylebox_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_stylebox_override" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_font_override(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_font_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_font_override" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_font_size_override(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_font_size_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_font_size_override" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_color_override(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_color_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_color_override" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_constant_override(&self, name: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_constant_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2619796661i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_constant_override" , 2619796661i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_icon(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_icon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1187511791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_icon" , 1187511791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_stylebox(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_stylebox");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1187511791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_stylebox" , 1187511791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_font(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1187511791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_font" , 1187511791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_font_size(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1187511791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_font_size" , 1187511791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_color(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1187511791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_color" , 1187511791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_theme_constant(&self, name: StringName, theme_type: StringName) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("has_theme_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1187511791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "has_theme_constant" , 1187511791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <StringName as sys::GodotFfi>::sys_const(&theme_type),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_default_base_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_default_base_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_default_base_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_default_font(&self) -> Option<Gd<Font>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_default_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3229501585i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_default_font" , 3229501585i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Font>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_theme_default_font_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_theme_default_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_theme_default_font_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_parent_control(&self) -> Option<Gd<Control>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_parent_control");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2783021301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_parent_control" , 2783021301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Control>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_h_grow_direction(&mut self, direction: control::GrowDirection) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_h_grow_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2022385301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_h_grow_direction" , 2022385301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::GrowDirection as sys::GodotFfi>::sys_const(
                    &direction,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_h_grow_direction(&self) -> control::GrowDirection {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_h_grow_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635610155i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_h_grow_direction" , 3635610155i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::GrowDirection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_v_grow_direction(&mut self, direction: control::GrowDirection) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_v_grow_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2022385301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_v_grow_direction" , 2022385301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::GrowDirection as sys::GodotFfi>::sys_const(
                    &direction,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_v_grow_direction(&self) -> control::GrowDirection {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_v_grow_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635610155i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_v_grow_direction" , 3635610155i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::GrowDirection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tooltip_text(&mut self, hint: GodotString) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_tooltip_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_tooltip_text" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&hint)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tooltip_text(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_tooltip_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_tooltip_text" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_tooltip(&self, at_position: Vector2) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_tooltip");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2895288280i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_tooltip" , 2895288280i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&at_position)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_default_cursor_shape(&mut self, shape: control::CursorShape) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_default_cursor_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    217062046i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_default_cursor_shape" , 217062046i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::CursorShape as sys::GodotFfi>::sys_const(&shape)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_default_cursor_shape(&self) -> control::CursorShape {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_default_cursor_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2359535750i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_default_cursor_shape" , 2359535750i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::CursorShape as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_cursor_shape(&self, position: Vector2) -> control::CursorShape {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_cursor_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1395773853i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_cursor_shape" , 1395773853i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                <control::CursorShape as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_focus_neighbor(&mut self, side: global::Side, neighbor: NodePath) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_focus_neighbor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2024461774i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_focus_neighbor" , 2024461774i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <global::Side as sys::GodotFfi>::sys_const(&side),
                    <NodePath as sys::GodotFfi>::sys_const(&neighbor),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_focus_neighbor(&self, side: global::Side) -> NodePath {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_focus_neighbor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2757935761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_focus_neighbor" , 2757935761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::Side as sys::GodotFfi>::sys_const(&side)];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_focus_next(&mut self, next: NodePath) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_focus_next");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1348162250i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_focus_next" , 1348162250i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&next)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_focus_next(&self) -> NodePath {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_focus_next");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4075236667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_focus_next" , 4075236667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_focus_previous(&mut self, previous: NodePath) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_focus_previous");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1348162250i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_focus_previous" , 1348162250i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&previous)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_focus_previous(&self) -> NodePath {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_focus_previous");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4075236667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_focus_previous" , 4075236667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <NodePath as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn force_drag(&mut self, data: Variant, preview: Gd<Control>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("force_drag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3191844692i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "force_drag" , 3191844692i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Variant as sys::GodotFfi>::sys_const(&data),
                    <Gd<Control> as AsArg>::as_arg_ptr(&preview),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_mouse_filter(&mut self, filter: control::MouseFilter) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_mouse_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3891156122i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_mouse_filter" , 3891156122i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::MouseFilter as sys::GodotFfi>::sys_const(&filter)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_mouse_filter(&self) -> control::MouseFilter {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_mouse_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1572545674i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_mouse_filter" , 1572545674i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::MouseFilter as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_force_pass_scroll_events(&mut self, force_pass_scroll_events: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_force_pass_scroll_events");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_force_pass_scroll_events" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(
                    &force_pass_scroll_events,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_force_pass_scroll_events(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("is_force_pass_scroll_events");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "is_force_pass_scroll_events" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_clip_contents(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_clip_contents");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_clip_contents" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_clipping_contents(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("is_clipping_contents");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "is_clipping_contents" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn grab_click_focus(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("grab_click_focus");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "grab_click_focus" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_drag_forwarding(
            &mut self,
            drag_func: Callable,
            can_drop_func: Callable,
            drop_func: Callable,
        ) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_drag_forwarding");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1076571380i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_drag_forwarding" , 1076571380i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Callable as sys::GodotFfi>::sys_const(&drag_func),
                    <Callable as sys::GodotFfi>::sys_const(&can_drop_func),
                    <Callable as sys::GodotFfi>::sys_const(&drop_func),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_drag_preview(&mut self, control: Gd<Control>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_drag_preview");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1496901182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_drag_preview" , 1496901182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Control> as AsArg>::as_arg_ptr(&control)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_drag_successful(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("is_drag_successful");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "is_drag_successful" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn warp_mouse(&mut self, position: Vector2) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("warp_mouse");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "warp_mouse" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_shortcut_context(&mut self, node: Gd<Node>) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_shortcut_context");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1078189570i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_shortcut_context" , 1078189570i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Node> as AsArg>::as_arg_ptr(&node)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_shortcut_context(&self) -> Option<Gd<Node>> {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_shortcut_context");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3160264692i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_shortcut_context" , 3160264692i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Node>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn update_minimum_size(&mut self) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("update_minimum_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "update_minimum_size" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_layout_direction(&mut self, direction: control::LayoutDirection) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_layout_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3310692370i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_layout_direction" , 3310692370i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<control::LayoutDirection as sys::GodotFfi>::sys_const(
                    &direction,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_layout_direction(&self) -> control::LayoutDirection {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("get_layout_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1546772008i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "get_layout_direction" , 1546772008i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <control::LayoutDirection as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_layout_rtl(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("is_layout_rtl");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "is_layout_rtl" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_auto_translate(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_auto_translate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_auto_translate" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_auto_translating(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("is_auto_translating");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "is_auto_translating" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_localize_numeral_system(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("set_localize_numeral_system");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "set_localize_numeral_system" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_localizing_numeral_system(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Control");
                let __method_name = StringName::from("is_localizing_numeral_system");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Control" , "is_localizing_numeral_system" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub(crate) const NOTIFICATION_RESIZED: i32 = 40i32;
        pub(crate) const NOTIFICATION_MOUSE_ENTER: i32 = 41i32;
        pub(crate) const NOTIFICATION_MOUSE_EXIT: i32 = 42i32;
        pub(crate) const NOTIFICATION_FOCUS_ENTER: i32 = 43i32;
        pub(crate) const NOTIFICATION_FOCUS_EXIT: i32 = 44i32;
        pub(crate) const NOTIFICATION_THEME_CHANGED: i32 = 45i32;
        pub(crate) const NOTIFICATION_SCROLL_BEGIN: i32 = 47i32;
        pub(crate) const NOTIFICATION_SCROLL_END: i32 = 48i32;
        pub(crate) const NOTIFICATION_LAYOUT_DIRECTION_CHANGED: i32 = 49i32;
    }
    impl crate::obj::GodotClass for Control {
        type Base = crate::engine::CanvasItem;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Control";
    }
    impl crate::obj::EngineClass for Control {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::CanvasItem> for Control {}
    impl crate::obj::Inherits<crate::engine::Node> for Control {}
    impl crate::obj::Inherits<crate::engine::Object> for Control {}
    impl std::ops::Deref for Control {
        type Target = crate::engine::CanvasItem;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Control {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Control {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Control> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::CanvasItem> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FocusMode {
    ord: i32,
}
impl FocusMode {
    pub const FOCUS_NONE: Self = Self { ord: 0 };
    pub const FOCUS_CLICK: Self = Self { ord: 1 };
    pub const FOCUS_ALL: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for FocusMode {
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
unsafe impl sys::GodotFfi for FocusMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CursorShape {
    ord: i32,
}
impl CursorShape {
    pub const CURSOR_ARROW: Self = Self { ord: 0 };
    pub const CURSOR_IBEAM: Self = Self { ord: 1 };
    pub const CURSOR_POINTING_HAND: Self = Self { ord: 2 };
    pub const CURSOR_CROSS: Self = Self { ord: 3 };
    pub const CURSOR_WAIT: Self = Self { ord: 4 };
    pub const CURSOR_BUSY: Self = Self { ord: 5 };
    pub const CURSOR_DRAG: Self = Self { ord: 6 };
    pub const CURSOR_CAN_DROP: Self = Self { ord: 7 };
    pub const CURSOR_FORBIDDEN: Self = Self { ord: 8 };
    pub const CURSOR_VSIZE: Self = Self { ord: 9 };
    pub const CURSOR_HSIZE: Self = Self { ord: 10 };
    pub const CURSOR_BDIAGSIZE: Self = Self { ord: 11 };
    pub const CURSOR_FDIAGSIZE: Self = Self { ord: 12 };
    pub const CURSOR_MOVE: Self = Self { ord: 13 };
    pub const CURSOR_VSPLIT: Self = Self { ord: 14 };
    pub const CURSOR_HSPLIT: Self = Self { ord: 15 };
    pub const CURSOR_HELP: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for CursorShape {
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
            | ord @ 16i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CursorShape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LayoutPreset {
    ord: i32,
}
impl LayoutPreset {
    pub const PRESET_TOP_LEFT: Self = Self { ord: 0 };
    pub const PRESET_TOP_RIGHT: Self = Self { ord: 1 };
    pub const PRESET_BOTTOM_LEFT: Self = Self { ord: 2 };
    pub const PRESET_BOTTOM_RIGHT: Self = Self { ord: 3 };
    pub const PRESET_CENTER_LEFT: Self = Self { ord: 4 };
    pub const PRESET_CENTER_TOP: Self = Self { ord: 5 };
    pub const PRESET_CENTER_RIGHT: Self = Self { ord: 6 };
    pub const PRESET_CENTER_BOTTOM: Self = Self { ord: 7 };
    pub const PRESET_CENTER: Self = Self { ord: 8 };
    pub const PRESET_LEFT_WIDE: Self = Self { ord: 9 };
    pub const PRESET_TOP_WIDE: Self = Self { ord: 10 };
    pub const PRESET_RIGHT_WIDE: Self = Self { ord: 11 };
    pub const PRESET_BOTTOM_WIDE: Self = Self { ord: 12 };
    pub const PRESET_VCENTER_WIDE: Self = Self { ord: 13 };
    pub const PRESET_HCENTER_WIDE: Self = Self { ord: 14 };
    pub const PRESET_FULL_RECT: Self = Self { ord: 15 };
}
impl crate::obj::EngineEnum for LayoutPreset {
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
            | ord @ 15i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for LayoutPreset {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LayoutPresetMode {
    ord: i32,
}
impl LayoutPresetMode {
    pub const PRESET_MODE_MINSIZE: Self = Self { ord: 0 };
    pub const PRESET_MODE_KEEP_WIDTH: Self = Self { ord: 1 };
    pub const PRESET_MODE_KEEP_HEIGHT: Self = Self { ord: 2 };
    pub const PRESET_MODE_KEEP_SIZE: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for LayoutPresetMode {
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
unsafe impl sys::GodotFfi for LayoutPresetMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct SizeFlags {
    ord: i32,
}
impl SizeFlags {
    pub const SIZE_SHRINK_BEGIN: Self = Self { ord: 0 };
    pub const SIZE_FILL: Self = Self { ord: 1 };
    pub const SIZE_EXPAND: Self = Self { ord: 2 };
    pub const SIZE_EXPAND_FILL: Self = Self { ord: 3 };
    pub const SIZE_SHRINK_CENTER: Self = Self { ord: 4 };
    pub const SIZE_SHRINK_END: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for SizeFlags {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 8i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for SizeFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for SizeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MouseFilter {
    ord: i32,
}
impl MouseFilter {
    pub const MOUSE_FILTER_STOP: Self = Self { ord: 0 };
    pub const MOUSE_FILTER_PASS: Self = Self { ord: 1 };
    pub const MOUSE_FILTER_IGNORE: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for MouseFilter {
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
unsafe impl sys::GodotFfi for MouseFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct GrowDirection {
    ord: i32,
}
impl GrowDirection {
    pub const GROW_DIRECTION_BEGIN: Self = Self { ord: 0 };
    pub const GROW_DIRECTION_END: Self = Self { ord: 1 };
    pub const GROW_DIRECTION_BOTH: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for GrowDirection {
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
unsafe impl sys::GodotFfi for GrowDirection {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Anchor {
    ord: i32,
}
impl Anchor {
    pub const ANCHOR_BEGIN: Self = Self { ord: 0 };
    pub const ANCHOR_END: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for Anchor {
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
unsafe impl sys::GodotFfi for Anchor {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LayoutDirection {
    ord: i32,
}
impl LayoutDirection {
    pub const LAYOUT_DIRECTION_INHERITED: Self = Self { ord: 0 };
    pub const LAYOUT_DIRECTION_LOCALE: Self = Self { ord: 1 };
    pub const LAYOUT_DIRECTION_LTR: Self = Self { ord: 2 };
    pub const LAYOUT_DIRECTION_RTL: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for LayoutDirection {
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
unsafe impl sys::GodotFfi for LayoutDirection {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextDirection {
    ord: i32,
}
impl TextDirection {
    pub const TEXT_DIRECTION_INHERITED: Self = Self { ord: 3 };
    pub const TEXT_DIRECTION_AUTO: Self = Self { ord: 0 };
    pub const TEXT_DIRECTION_LTR: Self = Self { ord: 1 };
    pub const TEXT_DIRECTION_RTL: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for TextDirection {
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
unsafe impl sys::GodotFfi for TextDirection {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
