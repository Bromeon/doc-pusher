#![doc = "Sidecar module for class [`BoxContainer`][crate::engine::BoxContainer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `BoxContainer` enums](https://docs.godotengine.org/en/stable/classes/class_boxcontainer.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `BoxContainer.`\n\nInherits [`Container`][crate::engine::Container].\n\nRelated symbols:\n\n* [`box_container`][crate::engine::box_container]: sidecar module with related enum/flag types\n* [`BoxContainerVirtual`][crate::engine::BoxContainerVirtual]: virtual methods\n\n\nSee also [Godot docs for `BoxContainer`](https://docs.godotengine.org/en/stable/classes/class_boxcontainer.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct BoxContainer {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`BoxContainer`][crate::engine::BoxContainer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `BoxContainer` methods](https://docs.godotengine.org/en/stable/classes/class_boxcontainer.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait BoxContainerVirtual:
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
    impl BoxContainer {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("BoxContainer");
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
        pub fn add_spacer(&mut self, begin: bool) -> Option<Gd<Control>> {
            unsafe {
                let __class_name = StringName::from("BoxContainer");
                let __method_name = StringName::from("add_spacer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1326660695i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BoxContainer" , "add_spacer" , 1326660695i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&begin)];
                let __args_ptr = __args.as_ptr();
                <Gd<Control>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_alignment(&mut self, alignment: box_container::AlignmentMode) {
            unsafe {
                let __class_name = StringName::from("BoxContainer");
                let __method_name = StringName::from("set_alignment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2456745134i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BoxContainer" , "set_alignment" , 2456745134i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<box_container::AlignmentMode as sys::GodotFfi>::sys_const(
                    &alignment,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alignment(&self) -> box_container::AlignmentMode {
            unsafe {
                let __class_name = StringName::from("BoxContainer");
                let __method_name = StringName::from("get_alignment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1915476527i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BoxContainer" , "get_alignment" , 1915476527i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <box_container::AlignmentMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_vertical(&mut self, vertical: bool) {
            unsafe {
                let __class_name = StringName::from("BoxContainer");
                let __method_name = StringName::from("set_vertical");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BoxContainer" , "set_vertical" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&vertical)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_vertical(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("BoxContainer");
                let __method_name = StringName::from("is_vertical");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "BoxContainer" , "is_vertical" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for BoxContainer {
        type Base = crate::engine::Container;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "BoxContainer";
    }
    impl crate::obj::EngineClass for BoxContainer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Container> for BoxContainer {}
    impl crate::obj::Inherits<crate::engine::Control> for BoxContainer {}
    impl crate::obj::Inherits<crate::engine::CanvasItem> for BoxContainer {}
    impl crate::obj::Inherits<crate::engine::Node> for BoxContainer {}
    impl crate::obj::Inherits<crate::engine::Object> for BoxContainer {}
    impl std::ops::Deref for BoxContainer {
        type Target = crate::engine::Container;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for BoxContainer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_BoxContainer {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::BoxContainer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Container> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Control> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::CanvasItem> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AlignmentMode {
    ord: i32,
}
impl AlignmentMode {
    pub const ALIGNMENT_BEGIN: Self = Self { ord: 0 };
    pub const ALIGNMENT_CENTER: Self = Self { ord: 1 };
    pub const ALIGNMENT_END: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for AlignmentMode {
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
unsafe impl sys::GodotFfi for AlignmentMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
