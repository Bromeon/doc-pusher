#![doc = "Sidecar module for class [`VisualShaderNodeTextureParameter`][crate::engine::VisualShaderNodeTextureParameter].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` enums](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `VisualShaderNodeTextureParameter.`\n\nInherits [`VisualShaderNodeParameter`][crate::engine::VisualShaderNodeParameter].\n\nRelated symbols:\n\n* [`visual_shader_node_texture_parameter`][crate::engine::visual_shader_node_texture_parameter]: sidecar module with related enum/flag types\n* [`VisualShaderNodeTextureParameterVirtual`][crate::engine::VisualShaderNodeTextureParameterVirtual]: virtual methods\n\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter`](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct VisualShaderNodeTextureParameter {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`VisualShaderNodeTextureParameter`][crate::engine::VisualShaderNodeTextureParameter].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `VisualShaderNodeTextureParameter` methods](https://docs.godotengine.org/en/stable/classes/class_visualshadernodetextureparameter.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait VisualShaderNodeTextureParameterVirtual:
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
    impl VisualShaderNodeTextureParameter {
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
        pub fn set_texture_type(
            &mut self,
            type_: visual_shader_node_texture_parameter::TextureType,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("set_texture_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2227296876i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "set_texture_type" , 2227296876i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <visual_shader_node_texture_parameter::TextureType as sys::GodotFfi>::sys_const(
                        &type_,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_type(&self) -> visual_shader_node_texture_parameter::TextureType {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("get_texture_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    367922070i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "get_texture_type" , 367922070i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < visual_shader_node_texture_parameter :: TextureType as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_color_default(
            &mut self,
            color: visual_shader_node_texture_parameter::ColorDefault,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("set_color_default");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4217624432i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "set_color_default" , 4217624432i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< visual_shader_node_texture_parameter :: ColorDefault as sys :: GodotFfi > :: sys_const (& color)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_color_default(&self) -> visual_shader_node_texture_parameter::ColorDefault {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("get_color_default");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3837060134i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "get_color_default" , 3837060134i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < visual_shader_node_texture_parameter :: ColorDefault as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_texture_filter(
            &mut self,
            filter: visual_shader_node_texture_parameter::TextureFilter,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("set_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2147684752i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "set_texture_filter" , 2147684752i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< visual_shader_node_texture_parameter :: TextureFilter as sys :: GodotFfi > :: sys_const (& filter)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_filter(&self) -> visual_shader_node_texture_parameter::TextureFilter {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("get_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4184490817i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "get_texture_filter" , 4184490817i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < visual_shader_node_texture_parameter :: TextureFilter as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_texture_repeat(
            &mut self,
            repeat: visual_shader_node_texture_parameter::TextureRepeat,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("set_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2036143070i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "set_texture_repeat" , 2036143070i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< visual_shader_node_texture_parameter :: TextureRepeat as sys :: GodotFfi > :: sys_const (& repeat)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_repeat(&self) -> visual_shader_node_texture_parameter::TextureRepeat {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("get_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1690132794i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "get_texture_repeat" , 1690132794i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < visual_shader_node_texture_parameter :: TextureRepeat as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn set_texture_source(
            &mut self,
            source: visual_shader_node_texture_parameter::TextureSource,
        ) {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("set_texture_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1212687372i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "set_texture_source" , 1212687372i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< visual_shader_node_texture_parameter :: TextureSource as sys :: GodotFfi > :: sys_const (& source)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_source(&self) -> visual_shader_node_texture_parameter::TextureSource {
            unsafe {
                let __class_name = StringName::from("VisualShaderNodeTextureParameter");
                let __method_name = StringName::from("get_texture_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2039092262i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "VisualShaderNodeTextureParameter" , "get_texture_source" , 2039092262i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                < visual_shader_node_texture_parameter :: TextureSource as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
    }
    impl crate::obj::GodotClass for VisualShaderNodeTextureParameter {
        type Base = crate::engine::VisualShaderNodeParameter;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "VisualShaderNodeTextureParameter";
    }
    impl crate::obj::EngineClass for VisualShaderNodeTextureParameter {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::VisualShaderNodeParameter>
        for VisualShaderNodeTextureParameter
    {
    }
    impl crate::obj::Inherits<crate::engine::VisualShaderNode> for VisualShaderNodeTextureParameter {}
    impl crate::obj::Inherits<crate::engine::Resource> for VisualShaderNodeTextureParameter {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for VisualShaderNodeTextureParameter {}
    impl crate::obj::Inherits<crate::engine::Object> for VisualShaderNodeTextureParameter {}
    impl std::ops::Deref for VisualShaderNodeTextureParameter {
        type Target = crate::engine::VisualShaderNodeParameter;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for VisualShaderNodeTextureParameter {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_VisualShaderNodeTextureParameter {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNodeTextureParameter>
                for $Class
            {
            }
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNodeParameter> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::VisualShaderNode> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureType {
    ord: i32,
}
impl TextureType {
    pub const TYPE_DATA: Self = Self { ord: 0 };
    pub const TYPE_COLOR: Self = Self { ord: 1 };
    pub const TYPE_NORMAL_MAP: Self = Self { ord: 2 };
    pub const TYPE_ANISOTROPY: Self = Self { ord: 3 };
    pub const TYPE_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for TextureType {
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
unsafe impl sys::GodotFfi for TextureType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ColorDefault {
    ord: i32,
}
impl ColorDefault {
    pub const COLOR_DEFAULT_WHITE: Self = Self { ord: 0 };
    pub const COLOR_DEFAULT_BLACK: Self = Self { ord: 1 };
    pub const COLOR_DEFAULT_TRANSPARENT: Self = Self { ord: 2 };
    pub const COLOR_DEFAULT_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ColorDefault {
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
unsafe impl sys::GodotFfi for ColorDefault {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureFilter {
    ord: i32,
}
impl TextureFilter {
    pub const FILTER_DEFAULT: Self = Self { ord: 0 };
    pub const FILTER_NEAREST: Self = Self { ord: 1 };
    pub const FILTER_LINEAR: Self = Self { ord: 2 };
    pub const FILTER_NEAREST_MIPMAP: Self = Self { ord: 3 };
    pub const FILTER_LINEAR_MIPMAP: Self = Self { ord: 4 };
    pub const FILTER_NEAREST_MIPMAP_ANISOTROPIC: Self = Self { ord: 5 };
    pub const FILTER_LINEAR_MIPMAP_ANISOTROPIC: Self = Self { ord: 6 };
    pub const FILTER_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for TextureFilter {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32
            | ord @ 7i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TextureFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureRepeat {
    ord: i32,
}
impl TextureRepeat {
    pub const REPEAT_DEFAULT: Self = Self { ord: 0 };
    pub const REPEAT_ENABLED: Self = Self { ord: 1 };
    pub const REPEAT_DISABLED: Self = Self { ord: 2 };
    pub const REPEAT_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for TextureRepeat {
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
unsafe impl sys::GodotFfi for TextureRepeat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureSource {
    ord: i32,
}
impl TextureSource {
    pub const SOURCE_NONE: Self = Self { ord: 0 };
    pub const SOURCE_SCREEN: Self = Self { ord: 1 };
    pub const SOURCE_DEPTH: Self = Self { ord: 2 };
    pub const SOURCE_NORMAL_ROUGHNESS: Self = Self { ord: 3 };
    pub const SOURCE_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for TextureSource {
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
unsafe impl sys::GodotFfi for TextureSource {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
