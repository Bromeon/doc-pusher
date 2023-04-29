#![doc = "Sidecar module for class [`ScriptLanguageExtension`][crate::engine::ScriptLanguageExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `ScriptLanguageExtension` enums](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `ScriptLanguageExtension.`\n\nInherits [`ScriptLanguage`][crate::engine::ScriptLanguage].\n\nRelated symbols:\n\n* [`script_language_extension`][crate::engine::script_language_extension]: sidecar module with related enum/flag types\n* [`ScriptLanguageExtensionVirtual`][crate::engine::ScriptLanguageExtensionVirtual]: virtual methods\n\n\nSee also [Godot docs for `ScriptLanguageExtension`](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ScriptLanguageExtension {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`ScriptLanguageExtension`][crate::engine::ScriptLanguageExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `ScriptLanguageExtension` methods](https://docs.godotengine.org/en/stable/classes/class_scriptlanguageextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait ScriptLanguageExtensionVirtual:
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
        fn get_name(&self) -> GodotString {
            unimplemented!()
        }
        fn init_ext(&mut self) {
            unimplemented!()
        }
        fn get_type(&self) -> GodotString {
            unimplemented!()
        }
        fn get_extension(&self) -> GodotString {
            unimplemented!()
        }
        fn finish(&mut self) {
            unimplemented!()
        }
        fn get_reserved_words(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn is_control_flow_keyword(&self, keyword: GodotString) -> bool {
            unimplemented!()
        }
        fn get_comment_delimiters(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn get_string_delimiters(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn make_template(
            &self,
            template: GodotString,
            class_name: GodotString,
            base_class_name: GodotString,
        ) -> Option<Gd<Script>> {
            unimplemented!()
        }
        fn get_built_in_templates(&self, object: StringName) -> Array<Dictionary> {
            unimplemented!()
        }
        fn is_using_templates(&mut self) -> bool {
            unimplemented!()
        }
        fn validate(
            &self,
            script: GodotString,
            path: GodotString,
            validate_functions: bool,
            validate_errors: bool,
            validate_warnings: bool,
            validate_safe_lines: bool,
        ) -> Dictionary {
            unimplemented!()
        }
        fn validate_path(&self, path: GodotString) -> GodotString {
            unimplemented!()
        }
        fn create_script(&self) -> Option<Gd<Object>> {
            unimplemented!()
        }
        fn has_named_classes(&self) -> bool {
            unimplemented!()
        }
        fn supports_builtin_mode(&self) -> bool {
            unimplemented!()
        }
        fn supports_documentation(&self) -> bool {
            unimplemented!()
        }
        fn can_inherit_from_file(&self) -> bool {
            unimplemented!()
        }
        fn find_function(&self, class_name: GodotString, function_name: GodotString) -> i64 {
            unimplemented!()
        }
        fn make_function(
            &self,
            class_name: GodotString,
            function_name: GodotString,
            function_args: PackedStringArray,
        ) -> GodotString {
            unimplemented!()
        }
        fn open_in_external_editor(
            &mut self,
            script: Gd<Script>,
            line: i64,
            column: i64,
        ) -> global::Error {
            unimplemented!()
        }
        fn overrides_external_editor(&mut self) -> bool {
            unimplemented!()
        }
        fn complete_code(
            &self,
            code: GodotString,
            path: GodotString,
            owner: Gd<Object>,
        ) -> Dictionary {
            unimplemented!()
        }
        fn lookup_code(
            &self,
            code: GodotString,
            symbol: GodotString,
            path: GodotString,
            owner: Gd<Object>,
        ) -> Dictionary {
            unimplemented!()
        }
        fn auto_indent_code(&self, code: GodotString, from_line: i64, to_line: i64) -> GodotString {
            unimplemented!()
        }
        fn add_global_constant(&mut self, name: StringName, value: Variant) {
            unimplemented!()
        }
        fn add_named_global_constant(&mut self, name: StringName, value: Variant) {
            unimplemented!()
        }
        fn remove_named_global_constant(&mut self, name: StringName) {
            unimplemented!()
        }
        fn thread_enter(&mut self) {
            unimplemented!()
        }
        fn thread_exit(&mut self) {
            unimplemented!()
        }
        fn debug_get_error(&self) -> GodotString {
            unimplemented!()
        }
        fn debug_get_stack_level_count(&self) -> i64 {
            unimplemented!()
        }
        fn debug_get_stack_level_line(&self, level: i64) -> i64 {
            unimplemented!()
        }
        fn debug_get_stack_level_function(&self, level: i64) -> GodotString {
            unimplemented!()
        }
        fn debug_get_stack_level_locals(
            &mut self,
            level: i64,
            max_subitems: i64,
            max_depth: i64,
        ) -> Dictionary {
            unimplemented!()
        }
        fn debug_get_stack_level_members(
            &mut self,
            level: i64,
            max_subitems: i64,
            max_depth: i64,
        ) -> Dictionary {
            unimplemented!()
        }
        fn debug_get_globals(&mut self, max_subitems: i64, max_depth: i64) -> Dictionary {
            unimplemented!()
        }
        fn debug_parse_stack_level_expression(
            &mut self,
            level: i64,
            expression: GodotString,
            max_subitems: i64,
            max_depth: i64,
        ) -> GodotString {
            unimplemented!()
        }
        fn debug_get_current_stack_info(&mut self) -> Array<Dictionary> {
            unimplemented!()
        }
        fn reload_all_scripts(&mut self) {
            unimplemented!()
        }
        fn reload_tool_script(&mut self, script: Gd<Script>, soft_reload: bool) {
            unimplemented!()
        }
        fn get_recognized_extensions(&self) -> PackedStringArray {
            unimplemented!()
        }
        fn get_public_functions(&self) -> Array<Dictionary> {
            unimplemented!()
        }
        fn get_public_constants(&self) -> Dictionary {
            unimplemented!()
        }
        fn get_public_annotations(&self) -> Array<Dictionary> {
            unimplemented!()
        }
        fn profiling_start(&mut self) {
            unimplemented!()
        }
        fn profiling_stop(&mut self) {
            unimplemented!()
        }
        fn frame(&mut self) {
            unimplemented!()
        }
        fn handles_global_class_type(&self, type_: GodotString) -> bool {
            unimplemented!()
        }
        fn get_global_class_name(&self, path: GodotString) -> Dictionary {
            unimplemented!()
        }
    }
    impl ScriptLanguageExtension {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("ScriptLanguageExtension");
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
    }
    impl crate::obj::GodotClass for ScriptLanguageExtension {
        type Base = crate::engine::ScriptLanguage;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "ScriptLanguageExtension";
    }
    impl crate::obj::EngineClass for ScriptLanguageExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::ScriptLanguage> for ScriptLanguageExtension {}
    impl crate::obj::Inherits<crate::engine::Object> for ScriptLanguageExtension {}
    impl std::ops::Deref for ScriptLanguageExtension {
        type Target = crate::engine::ScriptLanguage;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for ScriptLanguageExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_ScriptLanguageExtension {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::ScriptLanguageExtension> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::ScriptLanguage> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LookupResultType {
    ord: i32,
}
impl LookupResultType {
    pub const LOOKUP_RESULT_SCRIPT_LOCATION: Self = Self { ord: 0 };
    pub const LOOKUP_RESULT_CLASS: Self = Self { ord: 1 };
    pub const LOOKUP_RESULT_CLASS_CONSTANT: Self = Self { ord: 2 };
    pub const LOOKUP_RESULT_CLASS_PROPERTY: Self = Self { ord: 3 };
    pub const LOOKUP_RESULT_CLASS_METHOD: Self = Self { ord: 4 };
    pub const LOOKUP_RESULT_CLASS_SIGNAL: Self = Self { ord: 5 };
    pub const LOOKUP_RESULT_CLASS_ENUM: Self = Self { ord: 6 };
    pub const LOOKUP_RESULT_CLASS_TBD_GLOBALSCOPE: Self = Self { ord: 7 };
    pub const LOOKUP_RESULT_CLASS_ANNOTATION: Self = Self { ord: 8 };
    pub const LOOKUP_RESULT_MAX: Self = Self { ord: 9 };
}
impl crate::obj::EngineEnum for LookupResultType {
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
            | ord @ 9i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for LookupResultType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CodeCompletionLocation {
    ord: i32,
}
impl CodeCompletionLocation {
    pub const LOCATION_LOCAL: Self = Self { ord: 0 };
    pub const LOCATION_PARENT_MASK: Self = Self { ord: 256 };
    pub const LOCATION_OTHER_USER_CODE: Self = Self { ord: 512 };
    pub const LOCATION_OTHER: Self = Self { ord: 1024 };
}
impl crate::obj::EngineEnum for CodeCompletionLocation {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 256i32 | ord @ 512i32 | ord @ 1024i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CodeCompletionLocation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CodeCompletionKind {
    ord: i32,
}
impl CodeCompletionKind {
    pub const CODE_COMPLETION_KIND_CLASS: Self = Self { ord: 0 };
    pub const CODE_COMPLETION_KIND_FUNCTION: Self = Self { ord: 1 };
    pub const CODE_COMPLETION_KIND_SIGNAL: Self = Self { ord: 2 };
    pub const CODE_COMPLETION_KIND_VARIABLE: Self = Self { ord: 3 };
    pub const CODE_COMPLETION_KIND_MEMBER: Self = Self { ord: 4 };
    pub const CODE_COMPLETION_KIND_ENUM: Self = Self { ord: 5 };
    pub const CODE_COMPLETION_KIND_CONSTANT: Self = Self { ord: 6 };
    pub const CODE_COMPLETION_KIND_NODE_PATH: Self = Self { ord: 7 };
    pub const CODE_COMPLETION_KIND_FILE_PATH: Self = Self { ord: 8 };
    pub const CODE_COMPLETION_KIND_PLAIN_TEXT: Self = Self { ord: 9 };
    pub const CODE_COMPLETION_KIND_MAX: Self = Self { ord: 10 };
}
impl crate::obj::EngineEnum for CodeCompletionKind {
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
            | ord @ 10i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CodeCompletionKind {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
