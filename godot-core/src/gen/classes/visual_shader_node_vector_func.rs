#![doc = "Sidecar module for class [`VisualShaderNodeVectorFunc`][crate::engine::VisualShaderNodeVectorFunc].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `VisualShaderNodeVectorFunc.`\n\nInherits [`VisualShaderNodeVectorBase`][crate::engine::VisualShaderNodeVectorBase].\n\nRelated symbols:\n\n* [`visual_shader_node_vector_func`][crate::engine::visual_shader_node_vector_func]: sidecar module with related enum/flag types\n* [`VisualShaderNodeVectorFuncVirtual`][crate::engine::VisualShaderNodeVectorFuncVirtual]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct VisualShaderNodeVectorFunc {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeVectorFunc`][crate::engine::VisualShaderNodeVectorFunc].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeVectorFunc` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodevectorfunc.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait VisualShaderNodeVectorFuncVirtual:
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
    impl VisualShaderNodeVectorFunc {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeVectorFunc");
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
        pub fn set_function(&mut self, func: visual_shader_node_vector_func::Function) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeVectorFunc");
                let __method_name = StringName::from("set_function");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    629964457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeVectorFunc" , "set_function" , 629964457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader_node_vector_func::Function as sys::GodotFfi>::sys_const(&func),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_function(&self) -> visual_shader_node_vector_func::Function {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeVectorFunc");
                let __method_name = StringName::from("get_function");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4047776843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeVectorFunc" , "get_function" , 4047776843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <visual_shader_node_vector_func::Function as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
    }
    impl crate::obj::GodotClass for VisualShaderNodeVectorFunc {
        type Base = crate::engine::VisualShaderNodeVectorBase;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "VisualShaderNodeVectorFunc";
    }
    impl crate::obj::EngineClass for VisualShaderNodeVectorFunc {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::VisualShaderNodeVectorBase>
        for VisualShaderNodeVectorFunc
    {
    }
    impl crate::obj::Inherits<crate::engine::VisualShaderNode> for VisualShaderNodeVectorFunc {}
    impl crate::obj::Inherits<crate::engine::Resource> for VisualShaderNodeVectorFunc {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for VisualShaderNodeVectorFunc {}
    impl crate::obj::Inherits<crate::engine::Object> for VisualShaderNodeVectorFunc {}
    impl std::ops::Deref for VisualShaderNodeVectorFunc {
        type Target = crate::engine::VisualShaderNodeVectorBase;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeVectorFunc {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_VisualShaderNodeVectorFunc {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNodeVectorFunc> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNodeVectorBase> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Function {
    ord: i32,
}
impl Function {
    pub const FUNC_NORMALIZE: Self = Self { ord: 0 };
    pub const FUNC_SATURATE: Self = Self { ord: 1 };
    pub const FUNC_NEGATE: Self = Self { ord: 2 };
    pub const FUNC_RECIPROCAL: Self = Self { ord: 3 };
    pub const FUNC_ABS: Self = Self { ord: 4 };
    pub const FUNC_ACOS: Self = Self { ord: 5 };
    pub const FUNC_ACOSH: Self = Self { ord: 6 };
    pub const FUNC_ASIN: Self = Self { ord: 7 };
    pub const FUNC_ASINH: Self = Self { ord: 8 };
    pub const FUNC_ATAN: Self = Self { ord: 9 };
    pub const FUNC_ATANH: Self = Self { ord: 10 };
    pub const FUNC_CEIL: Self = Self { ord: 11 };
    pub const FUNC_COS: Self = Self { ord: 12 };
    pub const FUNC_COSH: Self = Self { ord: 13 };
    pub const FUNC_DEGREES: Self = Self { ord: 14 };
    pub const FUNC_EXP: Self = Self { ord: 15 };
    pub const FUNC_EXP2: Self = Self { ord: 16 };
    pub const FUNC_FLOOR: Self = Self { ord: 17 };
    pub const FUNC_FRACT: Self = Self { ord: 18 };
    pub const FUNC_INVERSE_SQRT: Self = Self { ord: 19 };
    pub const FUNC_LOG: Self = Self { ord: 20 };
    pub const FUNC_LOG2: Self = Self { ord: 21 };
    pub const FUNC_RADIANS: Self = Self { ord: 22 };
    pub const FUNC_ROUND: Self = Self { ord: 23 };
    pub const FUNC_ROUNDEVEN: Self = Self { ord: 24 };
    pub const FUNC_SIGN: Self = Self { ord: 25 };
    pub const FUNC_SIN: Self = Self { ord: 26 };
    pub const FUNC_SINH: Self = Self { ord: 27 };
    pub const FUNC_SQRT: Self = Self { ord: 28 };
    pub const FUNC_TAN: Self = Self { ord: 29 };
    pub const FUNC_TANH: Self = Self { ord: 30 };
    pub const FUNC_TRUNC: Self = Self { ord: 31 };
    pub const FUNC_ONEMINUS: Self = Self { ord: 32 };
    pub const FUNC_MAX: Self = Self { ord: 33 };
}
impl crate::obj::EngineEnum for Function {
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
            | ord @ 16i32
            | ord @ 17i32
            | ord @ 18i32
            | ord @ 19i32
            | ord @ 20i32
            | ord @ 21i32
            | ord @ 22i32
            | ord @ 23i32
            | ord @ 24i32
            | ord @ 25i32
            | ord @ 26i32
            | ord @ 27i32
            | ord @ 28i32
            | ord @ 29i32
            | ord @ 30i32
            | ord @ 31i32
            | ord @ 32i32
            | ord @ 33i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Function {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
