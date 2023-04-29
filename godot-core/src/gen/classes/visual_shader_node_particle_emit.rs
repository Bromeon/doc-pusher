#![doc = "Sidecar module for class [`VisualShaderNodeParticleEmit`][crate::engine::VisualShaderNodeParticleEmit].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeParticleEmit` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparticleemit.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `VisualShaderNodeParticleEmit.`\n\nInherits [`VisualShaderNode`][crate::engine::VisualShaderNode].\n\nRelated symbols:\n\n* [`visual_shader_node_particle_emit`][crate::engine::visual_shader_node_particle_emit]: sidecar module with related enum/flag types\n* [`VisualShaderNodeParticleEmitVirtual`][crate::engine::VisualShaderNodeParticleEmitVirtual]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeParticleEmit`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparticleemit.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct VisualShaderNodeParticleEmit {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeParticleEmit`][crate::engine::VisualShaderNodeParticleEmit].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeParticleEmit` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodeparticleemit.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait VisualShaderNodeParticleEmitVirtual:
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
    impl VisualShaderNodeParticleEmit {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeParticleEmit");
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
        pub fn set_flags(&mut self, flags: visual_shader_node_particle_emit::EmitFlags) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeParticleEmit");
                let __method_name = StringName::from("set_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3960756792i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeParticleEmit" , "set_flags" , 3960756792i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader_node_particle_emit::EmitFlags as sys::GodotFfi>::sys_const(
                        &flags,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_flags(&self) -> visual_shader_node_particle_emit::EmitFlags {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeParticleEmit");
                let __method_name = StringName::from("get_flags");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    171277835i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeParticleEmit" , "get_flags" , 171277835i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < visual_shader_node_particle_emit :: EmitFlags as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
    }
    impl crate::obj::GodotClass for VisualShaderNodeParticleEmit {
        type Base = crate::engine::VisualShaderNode;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "VisualShaderNodeParticleEmit";
    }
    impl crate::obj::EngineClass for VisualShaderNodeParticleEmit {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::VisualShaderNode> for VisualShaderNodeParticleEmit {}
    impl crate::obj::Inherits<crate::engine::Resource> for VisualShaderNodeParticleEmit {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for VisualShaderNodeParticleEmit {}
    impl crate::obj::Inherits<crate::engine::Object> for VisualShaderNodeParticleEmit {}
    impl std::ops::Deref for VisualShaderNodeParticleEmit {
        type Target = crate::engine::VisualShaderNode;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeParticleEmit {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_VisualShaderNodeParticleEmit {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNodeParticleEmit> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EmitFlags {
    ord: i32,
}
impl EmitFlags {
    pub const EMIT_FLAG_POSITION: Self = Self { ord: 1 };
    pub const EMIT_FLAG_ROT_SCALE: Self = Self { ord: 2 };
    pub const EMIT_FLAG_VELOCITY: Self = Self { ord: 4 };
    pub const EMIT_FLAG_COLOR: Self = Self { ord: 8 };
    pub const EMIT_FLAG_CUSTOM: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for EmitFlags {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 8i32 | ord @ 16i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for EmitFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
