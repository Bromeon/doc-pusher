#![doc = "Sidecar module for class [`AnimationNode`][crate::engine::AnimationNode].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `AnimationNode` enums](https://docs.godotengine.org/en/stable/classes/class_animationnode.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `AnimationNode.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`animation_node`][crate::engine::animation_node]: sidecar module with related enum/flag types\n* [`AnimationNodeVirtual`][crate::engine::AnimationNodeVirtual]: virtual methods\n\n\nSee also [Godot docs for `AnimationNode`](https://docs.godotengine.org/en/stable/classes/class_animationnode.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct AnimationNode {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`AnimationNode`][crate::engine::AnimationNode].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `AnimationNode` methods](https://docs.godotengine.org/en/stable/classes/class_animationnode.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait AnimationNodeVirtual:
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
        fn get_child_nodes(&self) -> Dictionary {
            unimplemented!()
        }
        fn get_parameter_list(&self) -> VariantArray {
            unimplemented!()
        }
        fn get_child_by_name(&self, name: StringName) -> Option<Gd<AnimationNode>> {
            unimplemented!()
        }
        fn get_parameter_default_value(&self, parameter: StringName) -> Variant {
            unimplemented!()
        }
        fn is_parameter_read_only(&self, parameter: StringName) -> bool {
            unimplemented!()
        }
        fn process(&self, time: f64, seek: bool, is_external_seeking: bool) -> f64 {
            unimplemented!()
        }
        fn get_caption(&self) -> GodotString {
            unimplemented!()
        }
        fn has_filter(&self) -> bool {
            unimplemented!()
        }
    }
    impl AnimationNode {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
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
        pub fn add_input(&mut self, name: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("add_input");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2323990056i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "add_input" , 2323990056i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_input(&mut self, index: i64) {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("remove_input");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "remove_input" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_input_name(&mut self, input: i64, name: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("set_input_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    215573526i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "set_input_name" , 215573526i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&input),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_input_name(&self, input: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("get_input_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "get_input_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&input)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_input_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("get_input_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "get_input_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn find_input(&self, name: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("find_input");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321353865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "find_input" , 1321353865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_filter_path(&mut self, path: NodePath, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("set_filter_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3868023870i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "set_filter_path" , 3868023870i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <NodePath as sys::GodotFfi>::sys_const(&path),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_path_filtered(&self, path: NodePath) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("is_path_filtered");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    861721659i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "is_path_filtered" , 861721659i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<NodePath as sys::GodotFfi>::sys_const(&path)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_filter_enabled(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("set_filter_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "set_filter_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_filter_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("is_filter_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "is_filter_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn blend_animation(
            &mut self,
            animation: StringName,
            time: f64,
            delta: f64,
            seeked: bool,
            is_external_seeking: bool,
            blend: f64,
            looped_flag: animation::LoopedFlag,
        ) {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("blend_animation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    11797022i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "blend_animation" , 11797022i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&animation),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <f64 as sys::GodotFfi>::sys_const(&delta),
                    <bool as sys::GodotFfi>::sys_const(&seeked),
                    <bool as sys::GodotFfi>::sys_const(&is_external_seeking),
                    <f64 as sys::GodotFfi>::sys_const(&blend),
                    <animation::LoopedFlag as sys::GodotFfi>::sys_const(&looped_flag),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn blend_node(
            &mut self,
            name: StringName,
            node: Gd<AnimationNode>,
            time: f64,
            seek: bool,
            is_external_seeking: bool,
            blend: f64,
            filter: animation_node::FilterAction,
            sync: bool,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("blend_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    308530085i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "blend_node" , 308530085i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Gd<AnimationNode> as AsArg>::as_arg_ptr(&node),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <bool as sys::GodotFfi>::sys_const(&seek),
                    <bool as sys::GodotFfi>::sys_const(&is_external_seeking),
                    <f64 as sys::GodotFfi>::sys_const(&blend),
                    <animation_node::FilterAction as sys::GodotFfi>::sys_const(&filter),
                    <bool as sys::GodotFfi>::sys_const(&sync),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn blend_input(
            &mut self,
            input_index: i64,
            time: f64,
            seek: bool,
            is_external_seeking: bool,
            blend: f64,
            filter: animation_node::FilterAction,
            sync: bool,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("blend_input");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1365393708i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "blend_input" , 1365393708i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&input_index),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                    <bool as sys::GodotFfi>::sys_const(&seek),
                    <bool as sys::GodotFfi>::sys_const(&is_external_seeking),
                    <f64 as sys::GodotFfi>::sys_const(&blend),
                    <animation_node::FilterAction as sys::GodotFfi>::sys_const(&filter),
                    <bool as sys::GodotFfi>::sys_const(&sync),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_parameter(&mut self, name: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("set_parameter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "set_parameter" , 3776071444i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_parameter(&self, name: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("AnimationNode");
                let __method_name = StringName::from("get_parameter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2760726917i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "AnimationNode" , "get_parameter" , 2760726917i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for AnimationNode {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "AnimationNode";
    }
    impl crate::obj::EngineClass for AnimationNode {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for AnimationNode {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for AnimationNode {}
    impl crate::obj::Inherits<crate::engine::Object> for AnimationNode {}
    impl std::ops::Deref for AnimationNode {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for AnimationNode {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_AnimationNode {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::AnimationNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FilterAction {
    ord: i32,
}
impl FilterAction {
    pub const FILTER_IGNORE: Self = Self { ord: 0 };
    pub const FILTER_PASS: Self = Self { ord: 1 };
    pub const FILTER_STOP: Self = Self { ord: 2 };
    pub const FILTER_BLEND: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for FilterAction {
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
unsafe impl sys::GodotFfi for FilterAction {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
