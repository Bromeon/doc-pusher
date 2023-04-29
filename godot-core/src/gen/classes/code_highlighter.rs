#![doc = "Sidecar module for class [`CodeHighlighter`][crate::engine::CodeHighlighter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `CodeHighlighter` enums](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `CodeHighlighter.`\n\nInherits [`SyntaxHighlighter`][crate::engine::SyntaxHighlighter].\n\nRelated symbols:\n\n* [`CodeHighlighterVirtual`][crate::engine::CodeHighlighterVirtual]: virtual methods\n\n\nSee also [Godot docs for `CodeHighlighter`](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct CodeHighlighter {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`CodeHighlighter`][crate::engine::CodeHighlighter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `CodeHighlighter` methods](https://docs.godotengine.org/en/stable/classes/class_codehighlighter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait CodeHighlighterVirtual:
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
        fn get_line_syntax_highlighting(&self, line: i64) -> Dictionary {
            unimplemented!()
        }
        fn clear_highlighting_cache(&mut self) {
            unimplemented!()
        }
        fn update_cache(&mut self) {
            unimplemented!()
        }
    }
    impl CodeHighlighter {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
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
        pub fn add_keyword_color(&mut self, keyword: GodotString, color: Color) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("add_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1636512886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "add_keyword_color" , 1636512886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&keyword),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_keyword_color(&mut self, keyword: GodotString) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("remove_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "remove_keyword_color" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&keyword)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_keyword_color(&self, keyword: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("has_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "has_keyword_color" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&keyword)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_keyword_color(&self, keyword: GodotString) -> Color {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3855908743i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_keyword_color" , 3855908743i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&keyword)];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_keyword_colors(&mut self, keywords: Dictionary) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_keyword_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155329257i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_keyword_colors" , 4155329257i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&keywords)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_keyword_colors(&mut self) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("clear_keyword_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "clear_keyword_colors" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_keyword_colors(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_keyword_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_keyword_colors" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_member_keyword_color(&mut self, member_keyword: GodotString, color: Color) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("add_member_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1636512886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "add_member_keyword_color" , 1636512886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&member_keyword),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_member_keyword_color(&mut self, member_keyword: GodotString) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("remove_member_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "remove_member_keyword_color" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&member_keyword)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_member_keyword_color(&self, member_keyword: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("has_member_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "has_member_keyword_color" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&member_keyword)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_member_keyword_color(&self, member_keyword: GodotString) -> Color {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_member_keyword_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3855908743i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_member_keyword_color" , 3855908743i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&member_keyword)];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_member_keyword_colors(&mut self, member_keyword: Dictionary) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_member_keyword_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155329257i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_member_keyword_colors" , 4155329257i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&member_keyword)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_member_keyword_colors(&mut self) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("clear_member_keyword_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "clear_member_keyword_colors" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_member_keyword_colors(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_member_keyword_colors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_member_keyword_colors" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_color_region(
            &mut self,
            start_key: GodotString,
            end_key: GodotString,
            color: Color,
            line_only: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("add_color_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2924977451i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "add_color_region" , 2924977451i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&start_key),
                    <GodotString as sys::GodotFfi>::sys_const(&end_key),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <bool as sys::GodotFfi>::sys_const(&line_only),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_color_region(&mut self, start_key: GodotString) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("remove_color_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "remove_color_region" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&start_key)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_color_region(&self, start_key: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("has_color_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "has_color_region" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&start_key)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_color_regions(&mut self, color_regions: Dictionary) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_color_regions");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155329257i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_color_regions" , 4155329257i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&color_regions)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_color_regions(&mut self) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("clear_color_regions");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "clear_color_regions" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color_regions(&self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_color_regions");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3102165223i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_color_regions" , 3102165223i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_function_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_function_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_function_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_function_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_function_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_function_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_number_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_number_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_number_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_number_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_number_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_number_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_symbol_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_symbol_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_symbol_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_symbol_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_symbol_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_symbol_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_member_variable_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("set_member_variable_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "set_member_variable_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_member_variable_color(&self) -> Color {
            unsafe {
                let __class_name = StringName::from("CodeHighlighter");
                let __method_name = StringName::from("get_member_variable_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3444240500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "CodeHighlighter" , "get_member_variable_color" , 3444240500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for CodeHighlighter {
        type Base = crate::engine::SyntaxHighlighter;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "CodeHighlighter";
    }
    impl crate::obj::EngineClass for CodeHighlighter {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::SyntaxHighlighter> for CodeHighlighter {}
    impl crate::obj::Inherits<crate::engine::Resource> for CodeHighlighter {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for CodeHighlighter {}
    impl crate::obj::Inherits<crate::engine::Object> for CodeHighlighter {}
    impl std::ops::Deref for CodeHighlighter {
        type Target = crate::engine::SyntaxHighlighter;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for CodeHighlighter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_CodeHighlighter {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::CodeHighlighter> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::SyntaxHighlighter> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
