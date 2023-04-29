#![doc = "Sidecar module for class [`Label3D`][crate::engine::Label3D].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Label3D` enums](https://docs.godotengine.org/en/stable/classes/class_label3d.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Label3D.`\n\nInherits [`GeometryInstance3D`][crate::engine::GeometryInstance3D].\n\nRelated symbols:\n\n* [`label_3d`][crate::engine::label_3d]: sidecar module with related enum/flag types\n* [`Label3DVirtual`][crate::engine::Label3DVirtual]: virtual methods\n\n\nSee also [Godot docs for `Label3D`](https://docs.godotengine.org/en/stable/classes/class_label3d.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Label3D {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Label3D`][crate::engine::Label3D].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Label3D` methods](https://docs.godotengine.org/en/stable/classes/class_label3d.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait Label3DVirtual:
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
        fn get_aabb(&self) -> Aabb {
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
    impl Label3D {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("Label3D");
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
        pub fn set_horizontal_alignment(&mut self, alignment: global::HorizontalAlignment) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_horizontal_alignment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2312603777i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_horizontal_alignment" , 2312603777i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::HorizontalAlignment as sys::GodotFfi>::sys_const(
                    &alignment,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_horizontal_alignment(&self) -> global::HorizontalAlignment {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_horizontal_alignment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    341400642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_horizontal_alignment" , 341400642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::HorizontalAlignment as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_vertical_alignment(&mut self, alignment: global::VerticalAlignment) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_vertical_alignment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1796458609i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_vertical_alignment" , 1796458609i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<global::VerticalAlignment as sys::GodotFfi>::sys_const(
                    &alignment,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_vertical_alignment(&self) -> global::VerticalAlignment {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_vertical_alignment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3274884059i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_vertical_alignment" , 3274884059i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <global::VerticalAlignment as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_modulate(&mut self, modulate: Color) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_modulate" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&modulate)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_modulate(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_modulate" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_outline_modulate(&mut self, modulate: Color) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_outline_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_outline_modulate" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&modulate)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_outline_modulate(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_outline_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_outline_modulate" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_text(&mut self, text: GodotString) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_text" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&text)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_text(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_text");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_text" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_text_direction(&mut self, direction: text_server::Direction) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_text_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1418190634i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_text_direction" , 1418190634i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::Direction as sys::GodotFfi>::sys_const(
                    &direction,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_text_direction(&self) -> text_server::Direction {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_text_direction");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2516697328i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_text_direction" , 2516697328i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::Direction as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_language(&mut self, language: GodotString) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_language");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_language" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&language)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_language(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_language");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_language" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_structured_text_bidi_override(
            &mut self,
            parser: text_server::StructuredTextParser,
        ) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_structured_text_bidi_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    55961453i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_structured_text_bidi_override" , 55961453i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<text_server::StructuredTextParser as sys::GodotFfi>::sys_const(&parser)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_structured_text_bidi_override(&self) -> text_server::StructuredTextParser {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_structured_text_bidi_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3385126229i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_structured_text_bidi_override" , 3385126229i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::StructuredTextParser as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_structured_text_bidi_override_options(&mut self, args: VariantArray) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_structured_text_bidi_override_options");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_structured_text_bidi_override_options" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<VariantArray as sys::GodotFfi>::sys_const(&args)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_structured_text_bidi_override_options(&self) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_structured_text_bidi_override_options");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_structured_text_bidi_override_options" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uppercase(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_uppercase");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_uppercase" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_uppercase(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("is_uppercase");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "is_uppercase" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_render_priority(&mut self, priority: i64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_render_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_render_priority" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&priority)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_render_priority(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_render_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_render_priority" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_outline_render_priority(&mut self, priority: i64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_outline_render_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_outline_render_priority" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&priority)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_outline_render_priority(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_outline_render_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_outline_render_priority" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_font(&mut self, font: Gd<Font>) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1262170328i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_font" , 1262170328i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Font> as AsArg>::as_arg_ptr(&font)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_font(&self) -> Option<Gd<Font>> {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_font");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3229501585i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_font" , 3229501585i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Font>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_font_size(&mut self, size: i64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_font_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_font_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_font_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_font_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_outline_size(&mut self, outline_size: i64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_outline_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_outline_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&outline_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_outline_size(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_outline_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_outline_size" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_line_spacing(&mut self, line_spacing: f64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_line_spacing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_line_spacing" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&line_spacing)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_line_spacing(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_line_spacing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_line_spacing" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_autowrap_mode(&mut self, autowrap_mode: text_server::AutowrapMode) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_autowrap_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3289138044i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_autowrap_mode" , 3289138044i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::AutowrapMode as sys::GodotFfi>::sys_const(
                    &autowrap_mode,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_autowrap_mode(&self) -> text_server::AutowrapMode {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_autowrap_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1549071663i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_autowrap_mode" , 1549071663i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::AutowrapMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_width(&mut self, width: f64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_width" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&width)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_width(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_width" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_pixel_size(&mut self, pixel_size: f64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_pixel_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_pixel_size" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&pixel_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_pixel_size(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_pixel_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_pixel_size" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_offset(&mut self, offset: Vector2) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    743155724i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_offset" , 743155724i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2 as sys::GodotFfi>::sys_const(&offset)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_offset(&self) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3341600327i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_offset" , 3341600327i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_draw_flag(&mut self, flag: label_3d::DrawFlags, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_draw_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1285833066i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_draw_flag" , 1285833066i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <label_3d::DrawFlags as sys::GodotFfi>::sys_const(&flag),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_draw_flag(&self, flag: label_3d::DrawFlags) -> bool {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_draw_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    259226453i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_draw_flag" , 259226453i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<label_3d::DrawFlags as sys::GodotFfi>::sys_const(&flag)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_billboard_mode(&mut self, mode: base_material_3d::BillboardMode) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_billboard_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4202036497i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_billboard_mode" , 4202036497i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::BillboardMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_billboard_mode(&self) -> base_material_3d::BillboardMode {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_billboard_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1283840139i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_billboard_mode" , 1283840139i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::BillboardMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_alpha_cut_mode(&mut self, mode: label_3d::AlphaCutMode) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_alpha_cut_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2549142916i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_alpha_cut_mode" , 2549142916i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<label_3d::AlphaCutMode as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_cut_mode(&self) -> label_3d::AlphaCutMode {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_alpha_cut_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    219468601i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_alpha_cut_mode" , 219468601i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <label_3d::AlphaCutMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_alpha_scissor_threshold(&mut self, threshold: f64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_alpha_scissor_threshold");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_alpha_scissor_threshold" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&threshold)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_scissor_threshold(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_alpha_scissor_threshold");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_alpha_scissor_threshold" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_alpha_hash_scale(&mut self, threshold: f64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_alpha_hash_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_alpha_hash_scale" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&threshold)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_hash_scale(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_alpha_hash_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_alpha_hash_scale" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_alpha_antialiasing(&mut self, alpha_aa: base_material_3d::AlphaAntiAliasing) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_alpha_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3212649852i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_alpha_antialiasing" , 3212649852i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <base_material_3d::AlphaAntiAliasing as sys::GodotFfi>::sys_const(&alpha_aa),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_antialiasing(&self) -> base_material_3d::AlphaAntiAliasing {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_alpha_antialiasing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2889939400i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_alpha_antialiasing" , 2889939400i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::AlphaAntiAliasing as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_alpha_antialiasing_edge(&mut self, edge: f64) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_alpha_antialiasing_edge");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_alpha_antialiasing_edge" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&edge)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alpha_antialiasing_edge(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_alpha_antialiasing_edge");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_alpha_antialiasing_edge" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_filter(&mut self, mode: base_material_3d::TextureFilter) {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("set_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    22904437i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "set_texture_filter" , 22904437i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<base_material_3d::TextureFilter as sys::GodotFfi>::sys_const(&mode)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_filter(&self) -> base_material_3d::TextureFilter {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("get_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3289213076i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "get_texture_filter" , 3289213076i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <base_material_3d::TextureFilter as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn generate_triangle_mesh(&self) -> Option<Gd<TriangleMesh>> {
            unsafe {
                let __class_name = StringName::from("Label3D");
                let __method_name = StringName::from("generate_triangle_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3476533166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Label3D" , "generate_triangle_mesh" , 3476533166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<TriangleMesh>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Label3D {
        type Base = crate::engine::GeometryInstance3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "Label3D";
    }
    impl crate::obj::EngineClass for Label3D {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::GeometryInstance3D> for Label3D {}
    impl crate::obj::Inherits<crate::engine::VisualInstance3D> for Label3D {}
    impl crate::obj::Inherits<crate::engine::Node3D> for Label3D {}
    impl crate::obj::Inherits<crate::engine::Node> for Label3D {}
    impl crate::obj::Inherits<crate::engine::Object> for Label3D {}
    impl std::ops::Deref for Label3D {
        type Target = crate::engine::GeometryInstance3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Label3D {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Label3D {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Label3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::GeometryInstance3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualInstance3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Node> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DrawFlags {
    ord: i32,
}
impl DrawFlags {
    pub const FLAG_SHADED: Self = Self { ord: 0 };
    pub const FLAG_DOUBLE_SIDED: Self = Self { ord: 1 };
    pub const FLAG_DISABLE_DEPTH_TEST: Self = Self { ord: 2 };
    pub const FLAG_FIXED_SIZE: Self = Self { ord: 3 };
    pub const FLAG_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for DrawFlags {
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
unsafe impl sys::GodotFfi for DrawFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AlphaCutMode {
    ord: i32,
}
impl AlphaCutMode {
    pub const ALPHA_CUT_DISABLED: Self = Self { ord: 0 };
    pub const ALPHA_CUT_DISCARD: Self = Self { ord: 1 };
    pub const ALPHA_CUT_OPAQUE_PREPASS: Self = Self { ord: 2 };
    pub const ALPHA_CUT_HASH: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for AlphaCutMode {
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
unsafe impl sys::GodotFfi for AlphaCutMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
