#![doc = "Sidecar module for class [`Font`][crate::engine::Font].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `Font` enums](https://docs.godotengine.org/en/stable/classes/class_font.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `Font.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`FontVirtual`][crate::engine::FontVirtual]: virtual methods\n\n\nSee also [Godot docs for `Font`](https://docs.godotengine.org/en/stable/classes/class_font.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Font {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`Font`][crate::engine::Font].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `Font` methods](https://docs.godotengine.org/en/stable/classes/class_font.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait FontVirtual:
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
    impl Font {
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
        pub fn set_fallbacks(&mut self, fallbacks: Array<Gd<Font>>) {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("set_fallbacks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "set_fallbacks" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<Font>> as sys::GodotFfi>::sys_const(&fallbacks)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_fallbacks(&self) -> Array<Gd<Font>> {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_fallbacks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_fallbacks" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Font>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn find_variation(
            &self,
            variation_coordinates: Dictionary,
            face_index: i64,
            strength: f64,
            transform: Transform2D,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("find_variation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3705324482i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "find_variation" , 3705324482i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Dictionary as sys::GodotFfi>::sys_const(&variation_coordinates),
                    <i64 as sys::GodotFfi>::sys_const(&face_index),
                    <f64 as sys::GodotFfi>::sys_const(&strength),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_rids(&self) -> Array<Rid> {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_rids");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3995934104i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_rids" , 3995934104i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Rid> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_height(&self, font_size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    378113874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_height" , 378113874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&font_size)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_ascent(&self, font_size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_ascent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    378113874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_ascent" , 378113874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&font_size)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_descent(&self, font_size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_descent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    378113874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_descent" , 378113874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&font_size)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_underline_position(&self, font_size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_underline_position");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    378113874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_underline_position" , 378113874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&font_size)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_underline_thickness(&self, font_size: i64) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_underline_thickness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    378113874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_underline_thickness" , 378113874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&font_size)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_font_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_font_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_style_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_font_style_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_font_style_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_style(&self) -> text_server::FontStyle {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_font_style");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2520224254i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_font_style" , 2520224254i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <text_server::FontStyle as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_weight(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_font_weight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_font_weight" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_font_stretch(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_font_stretch");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_font_stretch" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_spacing(&self, spacing: text_server::SpacingType) -> i64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_spacing");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1310880908i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_spacing" , 1310880908i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<text_server::SpacingType as sys::GodotFfi>::sys_const(
                    &spacing,
                )];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_opentype_features(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_opentype_features");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_opentype_features" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cache_capacity(&mut self, single_line: i64, multi_line: i64) {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("set_cache_capacity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "set_cache_capacity" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&single_line),
                    <i64 as sys::GodotFfi>::sys_const(&multi_line),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_string_size(
            &self,
            text: GodotString,
            alignment: global::HorizontalAlignment,
            width: f64,
            font_size: i64,
            jst_flags: text_server::JustificationFlag,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_string_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3678918099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_string_size" , 3678918099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <global::HorizontalAlignment as sys::GodotFfi>::sys_const(&alignment),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_multiline_string_size(
            &self,
            text: GodotString,
            alignment: global::HorizontalAlignment,
            width: f64,
            font_size: i64,
            max_lines: i64,
            brk_flags: text_server::LineBreakFlag,
            jst_flags: text_server::JustificationFlag,
            direction: text_server::Direction,
            orientation: text_server::Orientation,
        ) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_multiline_string_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2427690650i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_multiline_string_size" , 2427690650i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&text),
                    <global::HorizontalAlignment as sys::GodotFfi>::sys_const(&alignment),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <i64 as sys::GodotFfi>::sys_const(&max_lines),
                    <text_server::LineBreakFlag as sys::GodotFfi>::sys_const(&brk_flags),
                    <text_server::JustificationFlag as sys::GodotFfi>::sys_const(&jst_flags),
                    <text_server::Direction as sys::GodotFfi>::sys_const(&direction),
                    <text_server::Orientation as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_string(
            &self,
            canvas_item: Rid,
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
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("draw_string");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2565402639i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "draw_string" , 2565402639i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
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
            canvas_item: Rid,
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
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("draw_multiline_string");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    348869189i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "draw_multiline_string" , 348869189i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
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
            canvas_item: Rid,
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
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("draw_string_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    657875837i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "draw_string_outline" , 657875837i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
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
            canvas_item: Rid,
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
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("draw_multiline_string_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1649790182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "draw_multiline_string_outline" , 1649790182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
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
        pub fn get_char_size(&self, char: i64, font_size: i64) -> Vector2 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_char_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3016396712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_char_size" , 3016396712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&char),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                ];
                let __args_ptr = __args.as_ptr();
                <Vector2 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_char(
            &self,
            canvas_item: Rid,
            pos: Vector2,
            char: i64,
            font_size: i64,
            modulate: Color,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("draw_char");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1462476057i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "draw_char" , 1462476057i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <i64 as sys::GodotFfi>::sys_const(&char),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_char_outline(
            &self,
            canvas_item: Rid,
            pos: Vector2,
            char: i64,
            font_size: i64,
            size: i64,
            modulate: Color,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("draw_char_outline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4161008124i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "draw_char_outline" , 4161008124i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_item),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <i64 as sys::GodotFfi>::sys_const(&char),
                    <i64 as sys::GodotFfi>::sys_const(&font_size),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_char(&self, char: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("has_char");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "has_char" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&char)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_supported_chars(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_supported_chars");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_supported_chars" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_language_supported(&self, language: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("is_language_supported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "is_language_supported" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&language)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn is_script_supported(&self, script: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("is_script_supported");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "is_script_supported" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&script)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_supported_feature_list(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_supported_feature_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_supported_feature_list" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_supported_variation_list(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_supported_variation_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_supported_variation_list" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_face_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("Font");
                let __method_name = StringName::from("get_face_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "Font" , "get_face_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for Font {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "Font";
    }
    impl crate::obj::EngineClass for Font {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for Font {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for Font {}
    impl crate::obj::Inherits<crate::engine::Object> for Font {}
    impl std::ops::Deref for Font {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for Font {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_Font {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::Font> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
