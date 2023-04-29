#![doc = "Sidecar module for class [`EditorVcsInterface`][crate::engine::EditorVcsInterface].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `EditorVCSInterface` enums](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `EditorVCSInterface.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`editor_vcs_interface`][crate::engine::editor_vcs_interface]: sidecar module with related enum/flag types\n* [`EditorVcsInterfaceVirtual`][crate::engine::EditorVcsInterfaceVirtual]: virtual methods\n\n\nSee also [Godot docs for `EditorVCSInterface`](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct EditorVcsInterface {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`EditorVcsInterface`][crate::engine::EditorVcsInterface].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `EditorVCSInterface` methods](https://docs.godotengine.org/en/stable/classes/class_editorvcsinterface.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait EditorVcsInterfaceVirtual:
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
        fn initialize(&mut self, project_path: GodotString) -> bool {
            unimplemented!()
        }
        fn set_credentials(
            &mut self,
            username: GodotString,
            password: GodotString,
            ssh_public_key_path: GodotString,
            ssh_private_key_path: GodotString,
            ssh_passphrase: GodotString,
        ) {
            unimplemented!()
        }
        fn get_modified_files_data(&mut self) -> Array<Dictionary> {
            unimplemented!()
        }
        fn stage_file(&mut self, file_path: GodotString) {
            unimplemented!()
        }
        fn unstage_file(&mut self, file_path: GodotString) {
            unimplemented!()
        }
        fn discard_file(&mut self, file_path: GodotString) {
            unimplemented!()
        }
        fn commit(&mut self, msg: GodotString) {
            unimplemented!()
        }
        fn get_diff(&mut self, identifier: GodotString, area: i64) -> Array<Dictionary> {
            unimplemented!()
        }
        fn shut_down(&mut self) -> bool {
            unimplemented!()
        }
        fn get_vcs_name(&mut self) -> GodotString {
            unimplemented!()
        }
        fn get_previous_commits(&mut self, max_commits: i64) -> Array<Dictionary> {
            unimplemented!()
        }
        fn get_branch_list(&mut self) -> Array<GodotString> {
            unimplemented!()
        }
        fn get_remotes(&mut self) -> Array<GodotString> {
            unimplemented!()
        }
        fn create_branch(&mut self, branch_name: GodotString) {
            unimplemented!()
        }
        fn remove_branch(&mut self, branch_name: GodotString) {
            unimplemented!()
        }
        fn create_remote(&mut self, remote_name: GodotString, remote_url: GodotString) {
            unimplemented!()
        }
        fn remove_remote(&mut self, remote_name: GodotString) {
            unimplemented!()
        }
        fn get_current_branch_name(&mut self) -> GodotString {
            unimplemented!()
        }
        fn checkout_branch(&mut self, branch_name: GodotString) -> bool {
            unimplemented!()
        }
        fn pull(&mut self, remote: GodotString) {
            unimplemented!()
        }
        fn push(&mut self, remote: GodotString, force: bool) {
            unimplemented!()
        }
        fn fetch(&mut self, remote: GodotString) {
            unimplemented!()
        }
        fn get_line_diff(
            &mut self,
            file_path: GodotString,
            text: GodotString,
        ) -> Array<Dictionary> {
            unimplemented!()
        }
    }
    impl EditorVcsInterface {
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
        pub fn create_diff_line(
            &mut self,
            new_line_no: i64,
            old_line_no: i64,
            content: GodotString,
            status: GodotString,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("create_diff_line");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2901184053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "create_diff_line" , 2901184053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&new_line_no),
                    <i64 as sys::GodotFfi>::sys_const(&old_line_no),
                    <GodotString as sys::GodotFfi>::sys_const(&content),
                    <GodotString as sys::GodotFfi>::sys_const(&status),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_diff_hunk(
            &mut self,
            old_start: i64,
            new_start: i64,
            old_lines: i64,
            new_lines: i64,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("create_diff_hunk");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3784842090i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "create_diff_hunk" , 3784842090i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&old_start),
                    <i64 as sys::GodotFfi>::sys_const(&new_start),
                    <i64 as sys::GodotFfi>::sys_const(&old_lines),
                    <i64 as sys::GodotFfi>::sys_const(&new_lines),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_diff_file(
            &mut self,
            new_file: GodotString,
            old_file: GodotString,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("create_diff_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2723227684i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "create_diff_file" , 2723227684i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&new_file),
                    <GodotString as sys::GodotFfi>::sys_const(&old_file),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_commit(
            &mut self,
            msg: GodotString,
            author: GodotString,
            id: GodotString,
            unix_timestamp: i64,
            offset_minutes: i64,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("create_commit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1075983584i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "create_commit" , 1075983584i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&msg),
                    <GodotString as sys::GodotFfi>::sys_const(&author),
                    <GodotString as sys::GodotFfi>::sys_const(&id),
                    <i64 as sys::GodotFfi>::sys_const(&unix_timestamp),
                    <i64 as sys::GodotFfi>::sys_const(&offset_minutes),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_status_file(
            &mut self,
            file_path: GodotString,
            change_type: editor_vcs_interface::ChangeType,
            area: editor_vcs_interface::TreeArea,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("create_status_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1083471673i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "create_status_file" , 1083471673i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&file_path),
                    <editor_vcs_interface::ChangeType as sys::GodotFfi>::sys_const(&change_type),
                    <editor_vcs_interface::TreeArea as sys::GodotFfi>::sys_const(&area),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_diff_hunks_into_diff_file(
            &mut self,
            diff_file: Dictionary,
            diff_hunks: Array<Dictionary>,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("add_diff_hunks_into_diff_file");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4015243225i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "add_diff_hunks_into_diff_file" , 4015243225i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Dictionary as sys::GodotFfi>::sys_const(&diff_file),
                    <Array<Dictionary> as sys::GodotFfi>::sys_const(&diff_hunks),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_line_diffs_into_diff_hunk(
            &mut self,
            diff_hunk: Dictionary,
            line_diffs: Array<Dictionary>,
        ) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("add_line_diffs_into_diff_hunk");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4015243225i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "add_line_diffs_into_diff_hunk" , 4015243225i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Dictionary as sys::GodotFfi>::sys_const(&diff_hunk),
                    <Array<Dictionary> as sys::GodotFfi>::sys_const(&line_diffs),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn popup_error(&mut self, msg: GodotString) {
            unsafe {
                let __class_name = StringName::from("EditorVCSInterface");
                let __method_name = StringName::from("popup_error");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "EditorVCSInterface" , "popup_error" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&msg)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
    }
    impl crate::obj::GodotClass for EditorVcsInterface {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "EditorVCSInterface";
    }
    impl crate::obj::EngineClass for EditorVcsInterface {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for EditorVcsInterface {}
    impl std::ops::Deref for EditorVcsInterface {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for EditorVcsInterface {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_EditorVcsInterface {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::EditorVcsInterface> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ChangeType {
    ord: i32,
}
impl ChangeType {
    pub const CHANGE_TYPE_NEW: Self = Self { ord: 0 };
    pub const CHANGE_TYPE_MODIFIED: Self = Self { ord: 1 };
    pub const CHANGE_TYPE_RENAMED: Self = Self { ord: 2 };
    pub const CHANGE_TYPE_DELETED: Self = Self { ord: 3 };
    pub const CHANGE_TYPE_TYPECHANGE: Self = Self { ord: 4 };
    pub const CHANGE_TYPE_UNMERGED: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for ChangeType {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 | ord @ 2i32 | ord @ 3i32 | ord @ 4i32 | ord @ 5i32 => {
                Some(Self { ord })
            }
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ChangeType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TreeArea {
    ord: i32,
}
impl TreeArea {
    pub const TREE_AREA_COMMIT: Self = Self { ord: 0 };
    pub const TREE_AREA_STAGED: Self = Self { ord: 1 };
    pub const TREE_AREA_UNSTAGED: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for TreeArea {
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
unsafe impl sys::GodotFfi for TreeArea {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
