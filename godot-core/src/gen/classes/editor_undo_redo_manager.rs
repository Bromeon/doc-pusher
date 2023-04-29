#![doc = "Sidecar module for class [`EditorUndoRedoManager`][crate::engine::EditorUndoRedoManager].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorUndoRedoManager` enums](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorUndoRedoManager.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`editor_undo_redo_manager`][crate::engine::editor_undo_redo_manager]: sidecar module with related enum/flag types\n* [`EditorUndoRedoManagerVirtual`][crate::engine::EditorUndoRedoManagerVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorUndoRedoManager`](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorUndoRedoManager {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorUndoRedoManager`][crate::engine::EditorUndoRedoManager].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorUndoRedoManager` methods](https://docs.godotengine.org/en/stable/classes/class_editorundoredomanager.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorUndoRedoManagerVirtual:
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
    impl EditorUndoRedoManager {
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
        pub fn create_action(
            &mut self,
            name: GodotString,
            merge_mode: undo_redo::MergeMode,
            custom_context: Gd<Object>,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("create_action");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3766330317i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "create_action" , 3766330317i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <undo_redo::MergeMode as sys::GodotFfi>::sys_const(&merge_mode),
                    <Gd<Object> as AsArg>::as_arg_ptr(&custom_context),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn commit_action(&mut self, execute: bool) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("commit_action");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3216645846i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "commit_action" , 3216645846i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&execute)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_committing_action(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("is_committing_action");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "is_committing_action" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_do_method(
            &mut self,
            object: Gd<Object>,
            method: StringName,
            varargs: &[Variant],
        ) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("add_do_method");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1517810467i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "add_do_method" , 1517810467i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [
                    <Gd<Object> as ToVariant>::to_variant(&object),
                    <StringName as ToVariant>::to_variant(&method),
                ];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let mut __err = sys::default_call_error();
                let return_ptr = std::ptr::null_mut();
                __call_fn(
                    __method_bind,
                    self.object_ptr,
                    __args_ptr,
                    __args.len() as i64,
                    return_ptr,
                    std::ptr::addr_of_mut!(__err),
                );
                if __err.error != sys::GDEXTENSION_CALL_OK {
                    let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                    __arg_types.extend(varargs.iter().map(Variant::get_type));
                    let __vararg_str = varargs
                        .iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    sys::panic_call_error(
                        &__err,
                        &format!("add_do_method({object:?}, {method:?}; {__vararg_str})"),
                        &__arg_types,
                    );
                }
            }
        }
        pub fn add_undo_method(
            &mut self,
            object: Gd<Object>,
            method: StringName,
            varargs: &[Variant],
        ) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("add_undo_method");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1517810467i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "add_undo_method" , 1517810467i64);
                let __call_fn = sys::interface_fn!(object_method_bind_call);
                let __explicit_args = [
                    <Gd<Object> as ToVariant>::to_variant(&object),
                    <StringName as ToVariant>::to_variant(&method),
                ];
                let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __args.extend(__explicit_args.iter().map(Variant::var_sys_const));
                __args.extend(varargs.iter().map(Variant::var_sys_const));
                let __args_ptr = __args.as_ptr();
                let mut __err = sys::default_call_error();
                let return_ptr = std::ptr::null_mut();
                __call_fn(
                    __method_bind,
                    self.object_ptr,
                    __args_ptr,
                    __args.len() as i64,
                    return_ptr,
                    std::ptr::addr_of_mut!(__err),
                );
                if __err.error != sys::GDEXTENSION_CALL_OK {
                    let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                    __arg_types.extend(varargs.iter().map(Variant::get_type));
                    let __vararg_str = varargs
                        .iter()
                        .map(|v| format!("{v}"))
                        .collect::<Vec<_>>()
                        .join(", ");
                    sys::panic_call_error(
                        &__err,
                        &format!("add_undo_method({object:?}, {method:?}; {__vararg_str})"),
                        &__arg_types,
                    );
                }
            }
        }
        pub fn add_do_property(
            &mut self,
            object: Gd<Object>,
            property: StringName,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("add_do_property");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1017172818i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "add_do_property" , 1017172818i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Object> as AsArg>::as_arg_ptr(&object),
                    <StringName as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_undo_property(
            &mut self,
            object: Gd<Object>,
            property: StringName,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("add_undo_property");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1017172818i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "add_undo_property" , 1017172818i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Object> as AsArg>::as_arg_ptr(&object),
                    <StringName as sys::GodotFfi>::sys_const(&property),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_do_reference(&mut self, object: Gd<Object>) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("add_do_reference");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3975164845i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "add_do_reference" , 3975164845i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Object> as AsArg>::as_arg_ptr(&object)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_undo_reference(&mut self, object: Gd<Object>) {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("add_undo_reference");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3975164845i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "add_undo_reference" , 3975164845i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Object> as AsArg>::as_arg_ptr(&object)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_object_history_id(&self, object: Gd<Object>) -> i64 {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("get_object_history_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1107568780i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "get_object_history_id" , 1107568780i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Object> as AsArg>::as_arg_ptr(&object)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_history_undo_redo(&self, id: i64) -> Option<Gd<UndoRedo>> {
            unsafe {
                let __class_name = StringName::from("EditorUndoRedoManager");
                let __method_name = StringName::from("get_history_undo_redo");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2417974513i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorUndoRedoManager" , "get_history_undo_redo" , 2417974513i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&id)];
                let __args_ptr = __args.as_ptr();
                <Gd<UndoRedo>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for EditorUndoRedoManager {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "EditorUndoRedoManager";
    }
    impl crate::obj::EngineClass for EditorUndoRedoManager {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for EditorUndoRedoManager {}
    impl std::ops::Deref for EditorUndoRedoManager {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorUndoRedoManager {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorUndoRedoManager {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorUndoRedoManager> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SpecialHistory {
    ord: i32,
}
impl SpecialHistory {
    pub const GLOBAL_HISTORY: Self = Self { ord: 0 };
    pub const REMOTE_HISTORY: Self = Self { ord: -9 };
    pub const INVALID_HISTORY: Self = Self { ord: -99 };
}
impl crate::obj::EngineEnum for SpecialHistory {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ -99i32 | ord @ -9i32 | ord @ 0i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for SpecialHistory {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
