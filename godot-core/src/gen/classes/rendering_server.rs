#![doc = "Sidecar module for class [`RenderingServer`][crate::engine::RenderingServer].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingServer` enums](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `RenderingServer.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`rendering_server`][crate::engine::rendering_server]: sidecar module with related enum/flag types\n* [`RenderingServerVirtual`][crate::engine::RenderingServerVirtual]: virtual methods\n\n\nSee also [Godot docs for `RenderingServer`](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct RenderingServer {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`RenderingServer`][crate::engine::RenderingServer].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderingServer` methods](https://docs.godotengine.org/en/stable/classes/class_renderingserver.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait RenderingServerVirtual:
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
    impl RenderingServer {
        pub fn singleton() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __object_ptr =
                    sys::interface_fn!(global_get_singleton)(__class_name.string_sys());
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
        pub fn texture_2d_create(&mut self, image: Gd<Image>) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2010018390i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_create" , 2010018390i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Image> as AsArg>::as_arg_ptr(&image)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_2d_layered_create(
            &mut self,
            layers: Array<Gd<Image>>,
            layered_type: rendering_server::TextureLayeredType,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_layered_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    913689023i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_layered_create" , 913689023i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Gd<Image>> as sys::GodotFfi>::sys_const(&layers),
                    <rendering_server::TextureLayeredType as sys::GodotFfi>::sys_const(
                        &layered_type,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_3d_create(
            &mut self,
            format: image::Format,
            width: i64,
            height: i64,
            depth: i64,
            mipmaps: bool,
            data: Array<Gd<Image>>,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_3d_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4036838706i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_3d_create" , 4036838706i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <image::Format as sys::GodotFfi>::sys_const(&format),
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                    <i64 as sys::GodotFfi>::sys_const(&depth),
                    <bool as sys::GodotFfi>::sys_const(&mipmaps),
                    <Array<Gd<Image>> as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_proxy_create(&mut self, base: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_proxy_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    41030802i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_proxy_create" , 41030802i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&base)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_2d_update(&mut self, texture: Rid, image: Gd<Image>, layer: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    999539803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_update" , 999539803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Gd<Image> as AsArg>::as_arg_ptr(&image),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_3d_update(&mut self, texture: Rid, data: Array<Gd<Image>>) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_3d_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    684822712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_3d_update" , 684822712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Array<Gd<Image>> as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_proxy_update(&mut self, texture: Rid, proxy_to: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_proxy_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_proxy_update" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Rid as sys::GodotFfi>::sys_const(&proxy_to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_2d_placeholder_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_placeholder_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_placeholder_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_2d_layered_placeholder_create(
            &mut self,
            layered_type: rendering_server::TextureLayeredType,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_layered_placeholder_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1394585590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_layered_placeholder_create" , 1394585590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::TextureLayeredType as sys::GodotFfi>::sys_const(
                        &layered_type,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_3d_placeholder_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_3d_placeholder_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_3d_placeholder_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_2d_get(&self, texture: Rid) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4206205781i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_get" , 4206205781i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&texture)];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_2d_layer_get(&self, texture: Rid, layer: i64) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_2d_layer_get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2705440895i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_2d_layer_get" , 2705440895i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_3d_get(&self, texture: Rid) -> Array<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_3d_get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_3d_get" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&texture)];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Image>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_replace(&mut self, texture: Rid, by_texture: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_replace");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_replace" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Rid as sys::GodotFfi>::sys_const(&by_texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_set_size_override(&mut self, texture: Rid, width: i64, height: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_set_size_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4288446313i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_set_size_override" , 4288446313i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_set_path(&mut self, texture: Rid, path: GodotString) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_set_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_set_path" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_get_path(&self, texture: Rid) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_get_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642473191i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_get_path" , 642473191i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&texture)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_set_force_redraw_if_visible(&mut self, texture: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_set_force_redraw_if_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_set_force_redraw_if_visible" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn texture_get_rd_texture(&self, texture: Rid, srgb: bool) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("texture_get_rd_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2790148051i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "texture_get_rd_texture" , 2790148051i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <bool as sys::GodotFfi>::sys_const(&srgb),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_set_code(&mut self, shader: Rid, code: GodotString) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_set_code");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_set_code" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <GodotString as sys::GodotFfi>::sys_const(&code),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shader_set_path_hint(&mut self, shader: Rid, path: GodotString) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_set_path_hint");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_set_path_hint" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <GodotString as sys::GodotFfi>::sys_const(&path),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shader_get_code(&self, shader: Rid) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_get_code");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    642473191i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_get_code" , 642473191i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shader)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_shader_parameter_list(&self, shader: Rid) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_shader_parameter_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_shader_parameter_list" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shader)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_get_parameter_default(&self, shader: Rid, name: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_get_parameter_default");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2621281810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_get_parameter_default" , 2621281810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_set_default_texture_parameter(
            &mut self,
            shader: Rid,
            name: StringName,
            texture: Rid,
            index: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_set_default_texture_parameter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3864903085i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_set_default_texture_parameter" , 3864903085i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn shader_get_default_texture_parameter(
            &self,
            shader: Rid,
            name: StringName,
            index: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("shader_get_default_texture_parameter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2523186822i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "shader_get_default_texture_parameter" , 2523186822i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn material_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("material_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "material_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn material_set_shader(&mut self, shader_material: Rid, shader: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("material_set_shader");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "material_set_shader" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader_material),
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn material_set_param(&mut self, material: Rid, parameter: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("material_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3477296213i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "material_set_param" , 3477296213i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&material),
                    <StringName as sys::GodotFfi>::sys_const(&parameter),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn material_get_param(&self, material: Rid, parameter: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("material_get_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2621281810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "material_get_param" , 2621281810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&material),
                    <StringName as sys::GodotFfi>::sys_const(&parameter),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn material_set_render_priority(&mut self, material: Rid, priority: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("material_set_render_priority");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "material_set_render_priority" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&material),
                    <i64 as sys::GodotFfi>::sys_const(&priority),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn material_set_next_pass(&mut self, material: Rid, next_material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("material_set_next_pass");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "material_set_next_pass" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&material),
                    <Rid as sys::GodotFfi>::sys_const(&next_material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_create_from_surfaces(
            &mut self,
            surfaces: Array<Dictionary>,
            blend_shape_count: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_create_from_surfaces");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4007581507i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_create_from_surfaces" , 4007581507i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Dictionary> as sys::GodotFfi>::sys_const(&surfaces),
                    <i64 as sys::GodotFfi>::sys_const(&blend_shape_count),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_surface_get_format_offset(
            &self,
            format: rendering_server::ArrayFormat,
            vertex_count: i64,
            array_index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_format_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2981368685i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_format_offset" , 2981368685i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::ArrayFormat as sys::GodotFfi>::sys_const(&format),
                    <i64 as sys::GodotFfi>::sys_const(&vertex_count),
                    <i64 as sys::GodotFfi>::sys_const(&array_index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_surface_get_format_vertex_stride(
            &self,
            format: rendering_server::ArrayFormat,
            vertex_count: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_format_vertex_stride");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3188363337i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_format_vertex_stride" , 3188363337i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::ArrayFormat as sys::GodotFfi>::sys_const(&format),
                    <i64 as sys::GodotFfi>::sys_const(&vertex_count),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_surface_get_format_attribute_stride(
            &self,
            format: rendering_server::ArrayFormat,
            vertex_count: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_format_attribute_stride");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3188363337i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_format_attribute_stride" , 3188363337i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::ArrayFormat as sys::GodotFfi>::sys_const(&format),
                    <i64 as sys::GodotFfi>::sys_const(&vertex_count),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_surface_get_format_skin_stride(
            &self,
            format: rendering_server::ArrayFormat,
            vertex_count: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_format_skin_stride");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3188363337i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_format_skin_stride" , 3188363337i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::ArrayFormat as sys::GodotFfi>::sys_const(&format),
                    <i64 as sys::GodotFfi>::sys_const(&vertex_count),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_add_surface(&mut self, mesh: Rid, surface: Dictionary) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_add_surface");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1217542888i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_add_surface" , 1217542888i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <Dictionary as sys::GodotFfi>::sys_const(&surface),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_add_surface_from_arrays(
            &mut self,
            mesh: Rid,
            primitive: rendering_server::PrimitiveType,
            arrays: VariantArray,
            blend_shapes: VariantArray,
            lods: Dictionary,
            compress_format: rendering_server::ArrayFormat,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_add_surface_from_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1247008646i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_add_surface_from_arrays" , 1247008646i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <rendering_server::PrimitiveType as sys::GodotFfi>::sys_const(&primitive),
                    <VariantArray as sys::GodotFfi>::sys_const(&arrays),
                    <VariantArray as sys::GodotFfi>::sys_const(&blend_shapes),
                    <Dictionary as sys::GodotFfi>::sys_const(&lods),
                    <rendering_server::ArrayFormat as sys::GodotFfi>::sys_const(&compress_format),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_get_blend_shape_count(&self, mesh: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_get_blend_shape_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_get_blend_shape_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&mesh)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_set_blend_shape_mode(
            &mut self,
            mesh: Rid,
            mode: rendering_server::BlendShapeMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_set_blend_shape_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1294662092i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_set_blend_shape_mode" , 1294662092i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <rendering_server::BlendShapeMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_get_blend_shape_mode(&self, mesh: Rid) -> rendering_server::BlendShapeMode {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_get_blend_shape_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4282291819i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_get_blend_shape_mode" , 4282291819i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&mesh)];
                let __args_ptr = __args.as_ptr();
                <rendering_server::BlendShapeMode as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn mesh_surface_set_material(&mut self, mesh: Rid, surface: i64, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2310537182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_set_material" , 2310537182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_surface_get_material(&self, mesh: Rid, surface: i64) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1066463050i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_material" , 1066463050i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_get_surface(&mut self, mesh: Rid, surface: i64) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_get_surface");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    186674697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_get_surface" , 186674697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                ];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_surface_get_arrays(&self, mesh: Rid, surface: i64) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1778388067i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_arrays" , 1778388067i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                ];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_surface_get_blend_shape_arrays(
            &self,
            mesh: Rid,
            surface: i64,
        ) -> Array<VariantArray> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_get_blend_shape_arrays");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1778388067i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_get_blend_shape_arrays" , 1778388067i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<VariantArray> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_get_surface_count(&self, mesh: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_get_surface_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_get_surface_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&mesh)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_set_custom_aabb(&mut self, mesh: Rid, aabb: Aabb) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_set_custom_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3696536120i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_set_custom_aabb" , 3696536120i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <Aabb as sys::GodotFfi>::sys_const(&aabb),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_get_custom_aabb(&self, mesh: Rid) -> Aabb {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_get_custom_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    974181306i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_get_custom_aabb" , 974181306i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&mesh)];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn mesh_clear(&mut self, mesh: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_clear" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&mesh)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_surface_update_vertex_region(
            &mut self,
            mesh: Rid,
            surface: i64,
            offset: i64,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_update_vertex_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2900195149i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_update_vertex_region" , 2900195149i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_surface_update_attribute_region(
            &mut self,
            mesh: Rid,
            surface: i64,
            offset: i64,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_update_attribute_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2900195149i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_update_attribute_region" , 2900195149i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_surface_update_skin_region(
            &mut self,
            mesh: Rid,
            surface: i64,
            offset: i64,
            data: PackedByteArray,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_surface_update_skin_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2900195149i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_surface_update_skin_region" , 2900195149i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn mesh_set_shadow_mesh(&mut self, mesh: Rid, shadow_mesh: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("mesh_set_shadow_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "mesh_set_shadow_mesh" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <Rid as sys::GodotFfi>::sys_const(&shadow_mesh),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_allocate_data(
            &mut self,
            multimesh: Rid,
            instances: i64,
            transform_format: rendering_server::MultimeshTransformFormat,
            color_format: bool,
            custom_data_format: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_allocate_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    283685892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_allocate_data" , 283685892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&instances),
                    <rendering_server::MultimeshTransformFormat as sys::GodotFfi>::sys_const(
                        &transform_format,
                    ),
                    <bool as sys::GodotFfi>::sys_const(&color_format),
                    <bool as sys::GodotFfi>::sys_const(&custom_data_format),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_get_instance_count(&self, multimesh: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_get_instance_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_get_instance_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&multimesh)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_set_mesh(&mut self, multimesh: Rid, mesh: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_set_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_set_mesh" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_instance_set_transform(
            &mut self,
            multimesh: Rid,
            index: i64,
            transform: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    675327471i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_set_transform" , 675327471i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_instance_set_transform_2d(
            &mut self,
            multimesh: Rid,
            index: i64,
            transform: Transform2D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_set_transform_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    736082694i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_set_transform_2d" , 736082694i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_instance_set_color(&mut self, multimesh: Rid, index: i64, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    176975443i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_set_color" , 176975443i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_instance_set_custom_data(
            &mut self,
            multimesh: Rid,
            index: i64,
            custom_data: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_set_custom_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    176975443i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_set_custom_data" , 176975443i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                    <Color as sys::GodotFfi>::sys_const(&custom_data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_get_mesh(&self, multimesh: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_get_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814569979i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_get_mesh" , 3814569979i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&multimesh)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_get_aabb(&self, multimesh: Rid) -> Aabb {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_get_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    974181306i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_get_aabb" , 974181306i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&multimesh)];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_instance_get_transform(&self, multimesh: Rid, index: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1050775521i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_get_transform" , 1050775521i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_instance_get_transform_2d(
            &self,
            multimesh: Rid,
            index: i64,
        ) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_get_transform_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1324854622i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_get_transform_2d" , 1324854622i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_instance_get_color(&self, multimesh: Rid, index: i64) -> Color {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_get_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2946315076i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_get_color" , 2946315076i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_instance_get_custom_data(&self, multimesh: Rid, index: i64) -> Color {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_instance_get_custom_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2946315076i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_instance_get_custom_data" , 2946315076i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_set_visible_instances(&mut self, multimesh: Rid, visible: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_set_visible_instances");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_set_visible_instances" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <i64 as sys::GodotFfi>::sys_const(&visible),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_get_visible_instances(&self, multimesh: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_get_visible_instances");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_get_visible_instances" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&multimesh)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn multimesh_set_buffer(&mut self, multimesh: Rid, buffer: PackedFloat32Array) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_set_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2960552364i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_set_buffer" , 2960552364i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&multimesh),
                    <PackedFloat32Array as sys::GodotFfi>::sys_const(&buffer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn multimesh_get_buffer(&self, multimesh: Rid) -> PackedFloat32Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("multimesh_get_buffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3964669176i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "multimesh_get_buffer" , 3964669176i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&multimesh)];
                let __args_ptr = __args.as_ptr();
                <PackedFloat32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn skeleton_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn skeleton_allocate_data(&mut self, skeleton: Rid, bones: i64, is_2d_skeleton: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_allocate_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1904426712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_allocate_data" , 1904426712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                    <i64 as sys::GodotFfi>::sys_const(&bones),
                    <bool as sys::GodotFfi>::sys_const(&is_2d_skeleton),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn skeleton_get_bone_count(&self, skeleton: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_get_bone_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2198884583i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_get_bone_count" , 2198884583i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&skeleton)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn skeleton_bone_set_transform(
            &mut self,
            skeleton: Rid,
            bone: i64,
            transform: Transform3D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_bone_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    675327471i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_bone_set_transform" , 675327471i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                    <i64 as sys::GodotFfi>::sys_const(&bone),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn skeleton_bone_get_transform(&self, skeleton: Rid, bone: i64) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_bone_get_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1050775521i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_bone_get_transform" , 1050775521i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                    <i64 as sys::GodotFfi>::sys_const(&bone),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn skeleton_bone_set_transform_2d(
            &mut self,
            skeleton: Rid,
            bone: i64,
            transform: Transform2D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_bone_set_transform_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    736082694i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_bone_set_transform_2d" , 736082694i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                    <i64 as sys::GodotFfi>::sys_const(&bone),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn skeleton_bone_get_transform_2d(&self, skeleton: Rid, bone: i64) -> Transform2D {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_bone_get_transform_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1324854622i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_bone_get_transform_2d" , 1324854622i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                    <i64 as sys::GodotFfi>::sys_const(&bone),
                ];
                let __args_ptr = __args.as_ptr();
                <Transform2D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn skeleton_set_base_transform_2d(
            &mut self,
            skeleton: Rid,
            base_transform: Transform2D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("skeleton_set_base_transform_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "skeleton_set_base_transform_2d" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                    <Transform2D as sys::GodotFfi>::sys_const(&base_transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn directional_light_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("directional_light_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "directional_light_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn omni_light_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("omni_light_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "omni_light_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn spot_light_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("spot_light_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "spot_light_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn light_set_color(&mut self, light: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_color" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_param(
            &mut self,
            light: Rid,
            param: rendering_server::LightParam,
            value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_param");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    501936875i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_param" , 501936875i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::LightParam as sys::GodotFfi>::sys_const(&param),
                    <f64 as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_shadow(&mut self, light: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_shadow");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_shadow" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_projector(&mut self, light: Rid, texture: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_projector");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_projector" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_negative(&mut self, light: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_negative");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_negative" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_cull_mask(&mut self, light: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_distance_fade(
            &mut self,
            decal: Rid,
            enabled: bool,
            begin: f64,
            shadow: f64,
            length: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_distance_fade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1622292572i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_distance_fade" , 1622292572i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                    <f64 as sys::GodotFfi>::sys_const(&begin),
                    <f64 as sys::GodotFfi>::sys_const(&shadow),
                    <f64 as sys::GodotFfi>::sys_const(&length),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_reverse_cull_face_mode(&mut self, light: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_reverse_cull_face_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_reverse_cull_face_mode" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_bake_mode(
            &mut self,
            light: Rid,
            bake_mode: rendering_server::LightBakeMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_bake_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1048525260i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_bake_mode" , 1048525260i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::LightBakeMode as sys::GodotFfi>::sys_const(&bake_mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_set_max_sdfgi_cascade(&mut self, light: Rid, cascade: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_set_max_sdfgi_cascade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_set_max_sdfgi_cascade" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <i64 as sys::GodotFfi>::sys_const(&cascade),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_omni_set_shadow_mode(
            &mut self,
            light: Rid,
            mode: rendering_server::LightOmniShadowMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_omni_set_shadow_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2552677200i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_omni_set_shadow_mode" , 2552677200i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::LightOmniShadowMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_directional_set_shadow_mode(
            &mut self,
            light: Rid,
            mode: rendering_server::LightDirectionalShadowMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_directional_set_shadow_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    380462970i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_directional_set_shadow_mode" , 380462970i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::LightDirectionalShadowMode as sys::GodotFfi>::sys_const(
                        &mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_directional_set_blend_splits(&mut self, light: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_directional_set_blend_splits");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_directional_set_blend_splits" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_directional_set_sky_mode(
            &mut self,
            light: Rid,
            mode: rendering_server::LightDirectionalSkyMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_directional_set_sky_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2559740754i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_directional_set_sky_mode" , 2559740754i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::LightDirectionalSkyMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn light_projectors_set_filter(
            &mut self,
            filter: rendering_server::LightProjectorFilter,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("light_projectors_set_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    43944325i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "light_projectors_set_filter" , 43944325i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::LightProjectorFilter as sys::GodotFfi>::sys_const(&filter),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn positional_soft_shadow_filter_set_quality(
            &mut self,
            quality: rendering_server::ShadowQuality,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("positional_soft_shadow_filter_set_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3613045266i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "positional_soft_shadow_filter_set_quality" , 3613045266i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<rendering_server::ShadowQuality as sys::GodotFfi>::sys_const(&quality)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn directional_soft_shadow_filter_set_quality(
            &mut self,
            quality: rendering_server::ShadowQuality,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("directional_soft_shadow_filter_set_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3613045266i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "directional_soft_shadow_filter_set_quality" , 3613045266i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<rendering_server::ShadowQuality as sys::GodotFfi>::sys_const(&quality)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn directional_shadow_atlas_set_size(&mut self, size: i64, is_16bits: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("directional_shadow_atlas_set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "directional_shadow_atlas_set_size" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <bool as sys::GodotFfi>::sys_const(&is_16bits),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn reflection_probe_set_update_mode(
            &mut self,
            probe: Rid,
            mode: rendering_server::ReflectionProbeUpdateMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_update_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3853670147i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_update_mode" , 3853670147i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <rendering_server::ReflectionProbeUpdateMode as sys::GodotFfi>::sys_const(
                        &mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_intensity(&mut self, probe: Rid, intensity: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_intensity");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_intensity" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <f64 as sys::GodotFfi>::sys_const(&intensity),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_ambient_mode(
            &mut self,
            probe: Rid,
            mode: rendering_server::ReflectionProbeAmbientMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_ambient_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    184163074i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_ambient_mode" , 184163074i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <rendering_server::ReflectionProbeAmbientMode as sys::GodotFfi>::sys_const(
                        &mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_ambient_color(&mut self, probe: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_ambient_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_ambient_color" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_ambient_energy(&mut self, probe: Rid, energy: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_ambient_energy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_ambient_energy" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_max_distance(&mut self, probe: Rid, distance: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_max_distance");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_max_distance" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <f64 as sys::GodotFfi>::sys_const(&distance),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_size(&mut self, probe: Rid, size: Vector3) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_size" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <Vector3 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_origin_offset(&mut self, probe: Rid, offset: Vector3) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_origin_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_origin_offset" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <Vector3 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_as_interior(&mut self, probe: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_as_interior");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_as_interior" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_enable_box_projection(&mut self, probe: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_enable_box_projection");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_enable_box_projection" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_enable_shadows(&mut self, probe: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_enable_shadows");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_enable_shadows" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_cull_mask(&mut self, probe: Rid, layers: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <i64 as sys::GodotFfi>::sys_const(&layers),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_resolution(&mut self, probe: Rid, resolution: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_resolution");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_resolution" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <i64 as sys::GodotFfi>::sys_const(&resolution),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn reflection_probe_set_mesh_lod_threshold(&mut self, probe: Rid, pixels: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("reflection_probe_set_mesh_lod_threshold");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "reflection_probe_set_mesh_lod_threshold" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&probe),
                    <f64 as sys::GodotFfi>::sys_const(&pixels),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn decal_set_size(&mut self, decal: Rid, size: Vector3) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_size" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <Vector3 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_texture(
            &mut self,
            decal: Rid,
            type_: rendering_server::DecalTexture,
            texture: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3953344054i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_texture" , 3953344054i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <rendering_server::DecalTexture as sys::GodotFfi>::sys_const(&type_),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_emission_energy(&mut self, decal: Rid, energy: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_emission_energy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_emission_energy" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_albedo_mix(&mut self, decal: Rid, albedo_mix: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_albedo_mix");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_albedo_mix" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <f64 as sys::GodotFfi>::sys_const(&albedo_mix),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_modulate(&mut self, decal: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_modulate" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_cull_mask(&mut self, decal: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_distance_fade(
            &mut self,
            decal: Rid,
            enabled: bool,
            begin: f64,
            length: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_distance_fade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2972769666i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_distance_fade" , 2972769666i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                    <f64 as sys::GodotFfi>::sys_const(&begin),
                    <f64 as sys::GodotFfi>::sys_const(&length),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_fade(&mut self, decal: Rid, above: f64, below: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_fade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2513314492i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_fade" , 2513314492i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <f64 as sys::GodotFfi>::sys_const(&above),
                    <f64 as sys::GodotFfi>::sys_const(&below),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decal_set_normal_fade(&mut self, decal: Rid, fade: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decal_set_normal_fade");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decal_set_normal_fade" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&decal),
                    <f64 as sys::GodotFfi>::sys_const(&fade),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn decals_set_filter(&mut self, filter: rendering_server::DecalFilter) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("decals_set_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3519875702i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "decals_set_filter" , 3519875702i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_server::DecalFilter as sys::GodotFfi>::sys_const(
                    &filter,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn gi_set_use_half_resolution(&mut self, half_resolution: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("gi_set_use_half_resolution");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "gi_set_use_half_resolution" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&half_resolution)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_allocate_data(
            &mut self,
            voxel_gi: Rid,
            to_cell_xform: Transform3D,
            aabb: Aabb,
            octree_size: Vector3i,
            octree_cells: PackedByteArray,
            data_cells: PackedByteArray,
            distance_field: PackedByteArray,
            level_counts: PackedInt32Array,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_allocate_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4108223027i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_allocate_data" , 4108223027i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <Transform3D as sys::GodotFfi>::sys_const(&to_cell_xform),
                    <Aabb as sys::GodotFfi>::sys_const(&aabb),
                    <Vector3i as sys::GodotFfi>::sys_const(&octree_size),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&octree_cells),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data_cells),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&distance_field),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&level_counts),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_get_octree_size(&self, voxel_gi: Rid) -> Vector3i {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_get_octree_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2607699645i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_get_octree_size" , 2607699645i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&voxel_gi)];
                let __args_ptr = __args.as_ptr();
                <Vector3i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_get_octree_cells(&self, voxel_gi: Rid) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_get_octree_cells");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3348040486i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_get_octree_cells" , 3348040486i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&voxel_gi)];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_get_data_cells(&self, voxel_gi: Rid) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_get_data_cells");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3348040486i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_get_data_cells" , 3348040486i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&voxel_gi)];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_get_distance_field(&self, voxel_gi: Rid) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_get_distance_field");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3348040486i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_get_distance_field" , 3348040486i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&voxel_gi)];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_get_level_counts(&self, voxel_gi: Rid) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_get_level_counts");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    788230395i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_get_level_counts" , 788230395i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&voxel_gi)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_get_to_cell_xform(&self, voxel_gi: Rid) -> Transform3D {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_get_to_cell_xform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1128465797i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_get_to_cell_xform" , 1128465797i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&voxel_gi)];
                let __args_ptr = __args.as_ptr();
                <Transform3D as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn voxel_gi_set_dynamic_range(&mut self, voxel_gi: Rid, range: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_dynamic_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_dynamic_range" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <f64 as sys::GodotFfi>::sys_const(&range),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_propagation(&mut self, voxel_gi: Rid, amount: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_propagation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_propagation" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <f64 as sys::GodotFfi>::sys_const(&amount),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_energy(&mut self, voxel_gi: Rid, energy: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_energy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_energy" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_baked_exposure_normalization(
            &mut self,
            voxel_gi: Rid,
            baked_exposure: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_baked_exposure_normalization");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_baked_exposure_normalization" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <f64 as sys::GodotFfi>::sys_const(&baked_exposure),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_bias(&mut self, voxel_gi: Rid, bias: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_bias");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_bias" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <f64 as sys::GodotFfi>::sys_const(&bias),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_normal_bias(&mut self, voxel_gi: Rid, bias: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_normal_bias");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_normal_bias" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <f64 as sys::GodotFfi>::sys_const(&bias),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_interior(&mut self, voxel_gi: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_interior");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_interior" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_use_two_bounces(&mut self, voxel_gi: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_use_two_bounces");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_use_two_bounces" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&voxel_gi),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn voxel_gi_set_quality(&mut self, quality: rendering_server::VoxelGIQuality) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("voxel_gi_set_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1538689978i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "voxel_gi_set_quality" , 1538689978i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<rendering_server::VoxelGIQuality as sys::GodotFfi>::sys_const(&quality)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn lightmap_set_textures(&mut self, lightmap: Rid, light: Rid, uses_sh: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_set_textures");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2646464759i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_set_textures" , 2646464759i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&lightmap),
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&uses_sh),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_set_probe_bounds(&mut self, lightmap: Rid, bounds: Aabb) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_set_probe_bounds");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3696536120i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_set_probe_bounds" , 3696536120i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&lightmap),
                    <Aabb as sys::GodotFfi>::sys_const(&bounds),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_set_probe_interior(&mut self, lightmap: Rid, interior: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_set_probe_interior");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_set_probe_interior" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&lightmap),
                    <bool as sys::GodotFfi>::sys_const(&interior),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_set_probe_capture_data(
            &mut self,
            lightmap: Rid,
            points: PackedVector3Array,
            point_sh: PackedColorArray,
            tetrahedra: PackedInt32Array,
            bsp_tree: PackedInt32Array,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_set_probe_capture_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3217845880i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_set_probe_capture_data" , 3217845880i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&lightmap),
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&point_sh),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&tetrahedra),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&bsp_tree),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_get_probe_capture_points(&self, lightmap: Rid) -> PackedVector3Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_get_probe_capture_points");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    808965560i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_get_probe_capture_points" , 808965560i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&lightmap)];
                let __args_ptr = __args.as_ptr();
                <PackedVector3Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn lightmap_get_probe_capture_sh(&self, lightmap: Rid) -> PackedColorArray {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_get_probe_capture_sh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1569415609i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_get_probe_capture_sh" , 1569415609i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&lightmap)];
                let __args_ptr = __args.as_ptr();
                <PackedColorArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn lightmap_get_probe_capture_tetrahedra(&self, lightmap: Rid) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_get_probe_capture_tetrahedra");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    788230395i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_get_probe_capture_tetrahedra" , 788230395i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&lightmap)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn lightmap_get_probe_capture_bsp_tree(&self, lightmap: Rid) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_get_probe_capture_bsp_tree");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    788230395i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_get_probe_capture_bsp_tree" , 788230395i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&lightmap)];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn lightmap_set_baked_exposure_normalization(
            &mut self,
            lightmap: Rid,
            baked_exposure: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_set_baked_exposure_normalization");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_set_baked_exposure_normalization" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&lightmap),
                    <f64 as sys::GodotFfi>::sys_const(&baked_exposure),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn lightmap_set_probe_capture_update_speed(&mut self, speed: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("lightmap_set_probe_capture_update_speed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    373806689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "lightmap_set_probe_capture_update_speed" , 373806689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<f64 as sys::GodotFfi>::sys_const(&speed)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn particles_set_mode(
            &mut self,
            particles: Rid,
            mode: rendering_server::ParticlesMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3492270028i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_mode" , 3492270028i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <rendering_server::ParticlesMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_emitting(&mut self, particles: Rid, emitting: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_emitting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_emitting" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <bool as sys::GodotFfi>::sys_const(&emitting),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_get_emitting(&mut self, particles: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_get_emitting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_get_emitting" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&particles)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn particles_set_amount(&mut self, particles: Rid, amount: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_amount");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_amount" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <i64 as sys::GodotFfi>::sys_const(&amount),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_lifetime(&mut self, particles: Rid, lifetime: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_lifetime");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_lifetime" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <f64 as sys::GodotFfi>::sys_const(&lifetime),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_one_shot(&mut self, particles: Rid, one_shot: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_one_shot");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_one_shot" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <bool as sys::GodotFfi>::sys_const(&one_shot),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_pre_process_time(&mut self, particles: Rid, time: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_pre_process_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_pre_process_time" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <f64 as sys::GodotFfi>::sys_const(&time),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_explosiveness_ratio(&mut self, particles: Rid, ratio: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_explosiveness_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_explosiveness_ratio" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <f64 as sys::GodotFfi>::sys_const(&ratio),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_randomness_ratio(&mut self, particles: Rid, ratio: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_randomness_ratio");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_randomness_ratio" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <f64 as sys::GodotFfi>::sys_const(&ratio),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_custom_aabb(&mut self, particles: Rid, aabb: Aabb) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_custom_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3696536120i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_custom_aabb" , 3696536120i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Aabb as sys::GodotFfi>::sys_const(&aabb),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_speed_scale(&mut self, particles: Rid, scale: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_speed_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_speed_scale" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_use_local_coordinates(&mut self, particles: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_use_local_coordinates");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_use_local_coordinates" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_process_material(&mut self, particles: Rid, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_process_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_process_material" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_fixed_fps(&mut self, particles: Rid, fps: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_fixed_fps");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_fixed_fps" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <i64 as sys::GodotFfi>::sys_const(&fps),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_interpolate(&mut self, particles: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_interpolate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_interpolate" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_fractional_delta(&mut self, particles: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_fractional_delta");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_fractional_delta" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_collision_base_size(&mut self, particles: Rid, size: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_collision_base_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_collision_base_size" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <f64 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_transform_align(
            &mut self,
            particles: Rid,
            align: rendering_server::ParticlesTransformAlign,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_transform_align");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3264971368i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_transform_align" , 3264971368i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <rendering_server::ParticlesTransformAlign as sys::GodotFfi>::sys_const(&align),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_trails(&mut self, particles: Rid, enable: bool, length_sec: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_trails");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2010054925i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_trails" , 2010054925i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <f64 as sys::GodotFfi>::sys_const(&length_sec),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_trail_bind_poses(
            &mut self,
            particles: Rid,
            bind_poses: Array<Transform3D>,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_trail_bind_poses");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    684822712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_trail_bind_poses" , 684822712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Array<Transform3D> as sys::GodotFfi>::sys_const(&bind_poses),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_is_inactive(&mut self, particles: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_is_inactive");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_is_inactive" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&particles)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn particles_request_process(&mut self, particles: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_request_process");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_request_process" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&particles)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_restart(&mut self, particles: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_restart");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_restart" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&particles)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_subemitter(&mut self, particles: Rid, subemitter_particles: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_subemitter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_subemitter" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Rid as sys::GodotFfi>::sys_const(&subemitter_particles),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_emit(
            &mut self,
            particles: Rid,
            transform: Transform3D,
            velocity: Vector3,
            color: Color,
            custom: Color,
            emit_flags: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_emit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4043136117i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_emit" , 4043136117i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                    <Vector3 as sys::GodotFfi>::sys_const(&velocity),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <Color as sys::GodotFfi>::sys_const(&custom),
                    <i64 as sys::GodotFfi>::sys_const(&emit_flags),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_draw_order(
            &mut self,
            particles: Rid,
            order: rendering_server::ParticlesDrawOrder,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_draw_order");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    935028487i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_draw_order" , 935028487i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <rendering_server::ParticlesDrawOrder as sys::GodotFfi>::sys_const(&order),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_draw_passes(&mut self, particles: Rid, count: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_draw_passes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_draw_passes" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <i64 as sys::GodotFfi>::sys_const(&count),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_set_draw_pass_mesh(&mut self, particles: Rid, pass: i64, mesh: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_draw_pass_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2310537182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_draw_pass_mesh" , 2310537182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <i64 as sys::GodotFfi>::sys_const(&pass),
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_get_current_aabb(&mut self, particles: Rid) -> Aabb {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_get_current_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3952830260i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_get_current_aabb" , 3952830260i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&particles)];
                let __args_ptr = __args.as_ptr();
                <Aabb as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn particles_set_emission_transform(&mut self, particles: Rid, transform: Transform3D) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_set_emission_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3935195649i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_set_emission_transform" , 3935195649i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn particles_collision_set_collision_type(
            &mut self,
            particles_collision: Rid,
            type_: rendering_server::ParticlesCollisionType,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_set_collision_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1497044930i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_collision_type" , 1497044930i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <rendering_server::ParticlesCollisionType as sys::GodotFfi>::sys_const(&type_),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_cull_mask(&mut self, particles_collision: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_set_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_sphere_radius(
            &mut self,
            particles_collision: Rid,
            radius: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_set_sphere_radius");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_sphere_radius" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_box_extents(
            &mut self,
            particles_collision: Rid,
            extents: Vector3,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_set_box_extents");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_box_extents" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <Vector3 as sys::GodotFfi>::sys_const(&extents),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_attractor_strength(
            &mut self,
            particles_collision: Rid,
            setrngth: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_set_attractor_strength");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_attractor_strength" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <f64 as sys::GodotFfi>::sys_const(&setrngth),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_attractor_directionality(
            &mut self,
            particles_collision: Rid,
            amount: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("particles_collision_set_attractor_directionality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_attractor_directionality" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <f64 as sys::GodotFfi>::sys_const(&amount),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_attractor_attenuation(
            &mut self,
            particles_collision: Rid,
            curve: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("particles_collision_set_attractor_attenuation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_attractor_attenuation" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <f64 as sys::GodotFfi>::sys_const(&curve),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_field_texture(
            &mut self,
            particles_collision: Rid,
            texture: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_set_field_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_field_texture" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&particles_collision),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_height_field_update(&mut self, particles_collision: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("particles_collision_height_field_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_height_field_update" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&particles_collision)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn particles_collision_set_height_field_resolution(
            &mut self,
            particles_collision: Rid,
            resolution: rendering_server::ParticlesCollisionHeightfieldResolution,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("particles_collision_set_height_field_resolution");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    962977297i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "particles_collision_set_height_field_resolution" , 962977297i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< Rid as sys :: GodotFfi > :: sys_const (& particles_collision) , < rendering_server :: ParticlesCollisionHeightfieldResolution as sys :: GodotFfi > :: sys_const (& resolution)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fog_volume_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("fog_volume_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "fog_volume_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn fog_volume_set_shape(
            &mut self,
            fog_volume: Rid,
            shape: rendering_server::FogVolumeShape,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("fog_volume_set_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3818703106i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "fog_volume_set_shape" , 3818703106i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&fog_volume),
                    <rendering_server::FogVolumeShape as sys::GodotFfi>::sys_const(&shape),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fog_volume_set_size(&mut self, fog_volume: Rid, size: Vector3) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("fog_volume_set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3227306858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "fog_volume_set_size" , 3227306858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&fog_volume),
                    <Vector3 as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn fog_volume_set_material(&mut self, fog_volume: Rid, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("fog_volume_set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "fog_volume_set_material" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&fog_volume),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn visibility_notifier_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("visibility_notifier_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "visibility_notifier_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn visibility_notifier_set_aabb(&mut self, notifier: Rid, aabb: Aabb) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("visibility_notifier_set_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3696536120i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "visibility_notifier_set_aabb" , 3696536120i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&notifier),
                    <Aabb as sys::GodotFfi>::sys_const(&aabb),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn visibility_notifier_set_callbacks(
            &mut self,
            notifier: Rid,
            enter_callable: Callable,
            exit_callable: Callable,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("visibility_notifier_set_callbacks");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2689735388i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "visibility_notifier_set_callbacks" , 2689735388i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&notifier),
                    <Callable as sys::GodotFfi>::sys_const(&enter_callable),
                    <Callable as sys::GodotFfi>::sys_const(&exit_callable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn occluder_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("occluder_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "occluder_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn occluder_set_mesh(
            &mut self,
            occluder: Rid,
            vertices: PackedVector3Array,
            indices: PackedInt32Array,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("occluder_set_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3854404263i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "occluder_set_mesh" , 3854404263i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <PackedVector3Array as sys::GodotFfi>::sys_const(&vertices),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&indices),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn camera_set_perspective(
            &mut self,
            camera: Rid,
            fovy_degrees: f64,
            z_near: f64,
            z_far: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_perspective");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    157498339i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_perspective" , 157498339i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <f64 as sys::GodotFfi>::sys_const(&fovy_degrees),
                    <f64 as sys::GodotFfi>::sys_const(&z_near),
                    <f64 as sys::GodotFfi>::sys_const(&z_far),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_orthogonal(&mut self, camera: Rid, size: f64, z_near: f64, z_far: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_orthogonal");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    157498339i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_orthogonal" , 157498339i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <f64 as sys::GodotFfi>::sys_const(&size),
                    <f64 as sys::GodotFfi>::sys_const(&z_near),
                    <f64 as sys::GodotFfi>::sys_const(&z_far),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_frustum(
            &mut self,
            camera: Rid,
            size: f64,
            offset: Vector2,
            z_near: f64,
            z_far: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_frustum");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1889878953i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_frustum" , 1889878953i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <f64 as sys::GodotFfi>::sys_const(&size),
                    <Vector2 as sys::GodotFfi>::sys_const(&offset),
                    <f64 as sys::GodotFfi>::sys_const(&z_near),
                    <f64 as sys::GodotFfi>::sys_const(&z_far),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_transform(&mut self, camera: Rid, transform: Transform3D) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3935195649i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_transform" , 3935195649i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_cull_mask(&mut self, camera: Rid, layers: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <i64 as sys::GodotFfi>::sys_const(&layers),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_environment(&mut self, camera: Rid, env: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_environment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_environment" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <Rid as sys::GodotFfi>::sys_const(&env),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_camera_attributes(&mut self, camera: Rid, effects: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_camera_attributes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_camera_attributes" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <Rid as sys::GodotFfi>::sys_const(&effects),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_set_use_vertical_aspect(&mut self, camera: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_set_use_vertical_aspect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_set_use_vertical_aspect" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn viewport_set_use_xr(&mut self, viewport: Rid, use_xr: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_use_xr");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_use_xr" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&use_xr),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_size(&mut self, viewport: Rid, width: i64, height: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4288446313i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_size" , 4288446313i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <i64 as sys::GodotFfi>::sys_const(&width),
                    <i64 as sys::GodotFfi>::sys_const(&height),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_active(&mut self, viewport: Rid, active: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_active" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&active),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_parent_viewport(&mut self, viewport: Rid, parent_viewport: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_parent_viewport");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_parent_viewport" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&parent_viewport),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_attach_to_screen(&mut self, viewport: Rid, rect: Rect2, screen: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_attach_to_screen");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1278520651i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_attach_to_screen" , 1278520651i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <i64 as sys::GodotFfi>::sys_const(&screen),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_render_direct_to_screen(&mut self, viewport: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_render_direct_to_screen");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_render_direct_to_screen" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_canvas_cull_mask(&mut self, viewport: Rid, canvas_cull_mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_canvas_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_canvas_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <i64 as sys::GodotFfi>::sys_const(&canvas_cull_mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_scaling_3d_mode(
            &mut self,
            viewport: Rid,
            scaling_3d_mode: rendering_server::ViewportScaling3DMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_scaling_3d_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2386524376i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_scaling_3d_mode" , 2386524376i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportScaling3DMode as sys::GodotFfi>::sys_const(
                        &scaling_3d_mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_scaling_3d_scale(&mut self, viewport: Rid, scale: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_scaling_3d_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_scaling_3d_scale" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_fsr_sharpness(&mut self, viewport: Rid, sharpness: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_fsr_sharpness");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_fsr_sharpness" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <f64 as sys::GodotFfi>::sys_const(&sharpness),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_texture_mipmap_bias(&mut self, viewport: Rid, mipmap_bias: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_texture_mipmap_bias");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_texture_mipmap_bias" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <f64 as sys::GodotFfi>::sys_const(&mipmap_bias),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_update_mode(
            &mut self,
            viewport: Rid,
            update_mode: rendering_server::ViewportUpdateMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_update_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3161116010i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_update_mode" , 3161116010i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportUpdateMode as sys::GodotFfi>::sys_const(
                        &update_mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_clear_mode(
            &mut self,
            viewport: Rid,
            clear_mode: rendering_server::ViewportClearMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_clear_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3628367896i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_clear_mode" , 3628367896i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportClearMode as sys::GodotFfi>::sys_const(&clear_mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_get_texture(&self, viewport: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_get_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3814569979i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_get_texture" , 3814569979i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&viewport)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn viewport_set_disable_3d(&mut self, viewport: Rid, disable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_disable_3d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_disable_3d" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_disable_2d(&mut self, viewport: Rid, disable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_disable_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_disable_2d" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&disable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_environment_mode(
            &mut self,
            viewport: Rid,
            mode: rendering_server::ViewportEnvironmentMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_environment_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2196892182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_environment_mode" , 2196892182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportEnvironmentMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_attach_camera(&mut self, viewport: Rid, camera: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_attach_camera");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_attach_camera" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&camera),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_scenario(&mut self, viewport: Rid, scenario: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_scenario");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_scenario" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_attach_canvas(&mut self, viewport: Rid, canvas: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_attach_canvas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_attach_canvas" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_remove_canvas(&mut self, viewport: Rid, canvas: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_remove_canvas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_remove_canvas" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_snap_2d_transforms_to_pixel(&mut self, viewport: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_snap_2d_transforms_to_pixel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_snap_2d_transforms_to_pixel" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_snap_2d_vertices_to_pixel(&mut self, viewport: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_snap_2d_vertices_to_pixel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_snap_2d_vertices_to_pixel" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_default_canvas_item_texture_filter(
            &mut self,
            viewport: Rid,
            filter: rendering_server::CanvasItemTextureFilter,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("viewport_set_default_canvas_item_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1155129294i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_default_canvas_item_texture_filter" , 1155129294i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::CanvasItemTextureFilter as sys::GodotFfi>::sys_const(
                        &filter,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_default_canvas_item_texture_repeat(
            &mut self,
            viewport: Rid,
            repeat: rendering_server::CanvasItemTextureRepeat,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("viewport_set_default_canvas_item_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1652956681i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_default_canvas_item_texture_repeat" , 1652956681i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::CanvasItemTextureRepeat as sys::GodotFfi>::sys_const(
                        &repeat,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_canvas_transform(
            &mut self,
            viewport: Rid,
            canvas: Rid,
            offset: Transform2D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_canvas_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3608606053i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_canvas_transform" , 3608606053i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <Transform2D as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_canvas_stacking(
            &mut self,
            viewport: Rid,
            canvas: Rid,
            layer: i64,
            sublayer: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_canvas_stacking");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3713930247i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_canvas_stacking" , 3713930247i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                    <i64 as sys::GodotFfi>::sys_const(&sublayer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_transparent_background(&mut self, viewport: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_transparent_background");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_transparent_background" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_global_canvas_transform(
            &mut self,
            viewport: Rid,
            transform: Transform2D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_global_canvas_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_global_canvas_transform" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_sdf_oversize_and_scale(
            &mut self,
            viewport: Rid,
            oversize: rendering_server::ViewportSDFOversize,
            scale: rendering_server::ViewportSDFScale,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_sdf_oversize_and_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1329198632i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_sdf_oversize_and_scale" , 1329198632i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportSDFOversize as sys::GodotFfi>::sys_const(&oversize),
                    <rendering_server::ViewportSDFScale as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_positional_shadow_atlas_size(
            &mut self,
            viewport: Rid,
            size: i64,
            use_16_bits: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_positional_shadow_atlas_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1904426712i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_positional_shadow_atlas_size" , 1904426712i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <bool as sys::GodotFfi>::sys_const(&use_16_bits),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_positional_shadow_atlas_quadrant_subdivision(
            &mut self,
            viewport: Rid,
            quadrant: i64,
            subdivision: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("viewport_set_positional_shadow_atlas_quadrant_subdivision");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4288446313i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_positional_shadow_atlas_quadrant_subdivision" , 4288446313i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <i64 as sys::GodotFfi>::sys_const(&quadrant),
                    <i64 as sys::GodotFfi>::sys_const(&subdivision),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_msaa_3d(
            &mut self,
            viewport: Rid,
            msaa: rendering_server::ViewportMSAA,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_msaa_3d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3764433340i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_msaa_3d" , 3764433340i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportMSAA as sys::GodotFfi>::sys_const(&msaa),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_msaa_2d(
            &mut self,
            viewport: Rid,
            msaa: rendering_server::ViewportMSAA,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_msaa_2d");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3764433340i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_msaa_2d" , 3764433340i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportMSAA as sys::GodotFfi>::sys_const(&msaa),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_screen_space_aa(
            &mut self,
            viewport: Rid,
            mode: rendering_server::ViewportScreenSpaceAA,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_screen_space_aa");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1447279591i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_screen_space_aa" , 1447279591i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportScreenSpaceAA as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_use_taa(&mut self, viewport: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_use_taa");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_use_taa" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_use_debanding(&mut self, viewport: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_use_debanding");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_use_debanding" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_use_occlusion_culling(&mut self, viewport: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_use_occlusion_culling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_use_occlusion_culling" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_occlusion_rays_per_thread(&mut self, rays_per_thread: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_occlusion_rays_per_thread");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_occlusion_rays_per_thread" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&rays_per_thread)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_occlusion_culling_build_quality(
            &mut self,
            quality: rendering_server::ViewportOcclusionCullingBuildQuality,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("viewport_set_occlusion_culling_build_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2069725696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_occlusion_culling_build_quality" , 2069725696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< rendering_server :: ViewportOcclusionCullingBuildQuality as sys :: GodotFfi > :: sys_const (& quality)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_get_render_info(
            &mut self,
            viewport: Rid,
            type_: rendering_server::ViewportRenderInfoType,
            info: rendering_server::ViewportRenderInfo,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_get_render_info");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2041262392i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_get_render_info" , 2041262392i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportRenderInfoType as sys::GodotFfi>::sys_const(&type_),
                    <rendering_server::ViewportRenderInfo as sys::GodotFfi>::sys_const(&info),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn viewport_set_debug_draw(
            &mut self,
            viewport: Rid,
            draw: rendering_server::ViewportDebugDraw,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_debug_draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2089420930i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_debug_draw" , 2089420930i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportDebugDraw as sys::GodotFfi>::sys_const(&draw),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_measure_render_time(&mut self, viewport: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_measure_render_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_measure_render_time" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_get_measured_render_time_cpu(&self, viewport: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_get_measured_render_time_cpu");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_get_measured_render_time_cpu" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&viewport)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn viewport_get_measured_render_time_gpu(&self, viewport: Rid) -> f64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_get_measured_render_time_gpu");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    866169185i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_get_measured_render_time_gpu" , 866169185i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&viewport)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn viewport_set_vrs_mode(
            &mut self,
            viewport: Rid,
            mode: rendering_server::ViewportVRSMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_vrs_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    398809874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_vrs_mode" , 398809874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <rendering_server::ViewportVRSMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn viewport_set_vrs_texture(&mut self, viewport: Rid, texture: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("viewport_set_vrs_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "viewport_set_vrs_texture" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&viewport),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sky_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sky_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sky_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn sky_set_radiance_size(&mut self, sky: Rid, radiance_size: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sky_set_radiance_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sky_set_radiance_size" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&sky),
                    <i64 as sys::GodotFfi>::sys_const(&radiance_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sky_set_mode(&mut self, sky: Rid, mode: rendering_server::SkyMode) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sky_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3279019937i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sky_set_mode" , 3279019937i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&sky),
                    <rendering_server::SkyMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sky_set_material(&mut self, sky: Rid, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sky_set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sky_set_material" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&sky),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sky_bake_panorama(
            &mut self,
            sky: Rid,
            energy: f64,
            bake_irradiance: bool,
            size: Vector2i,
        ) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sky_bake_panorama");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3875285818i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sky_bake_panorama" , 3875285818i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&sky),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                    <bool as sys::GodotFfi>::sys_const(&bake_irradiance),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn environment_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn environment_set_background(
            &mut self,
            env: Rid,
            bg: rendering_server::EnvironmentBG,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_background");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937328877i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_background" , 3937328877i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <rendering_server::EnvironmentBG as sys::GodotFfi>::sys_const(&bg),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sky(&mut self, env: Rid, sky: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_sky");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sky" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <Rid as sys::GodotFfi>::sys_const(&sky),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sky_custom_fov(&mut self, env: Rid, scale: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_sky_custom_fov");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sky_custom_fov" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sky_orientation(&mut self, env: Rid, orientation: Basis) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_sky_orientation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1735850857i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sky_orientation" , 1735850857i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <Basis as sys::GodotFfi>::sys_const(&orientation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_bg_color(&mut self, env: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_bg_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_bg_color" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_bg_energy(
            &mut self,
            env: Rid,
            multiplier: f64,
            exposure_value: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_bg_energy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2513314492i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_bg_energy" , 2513314492i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <f64 as sys::GodotFfi>::sys_const(&multiplier),
                    <f64 as sys::GodotFfi>::sys_const(&exposure_value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_canvas_max_layer(&mut self, env: Rid, max_layer: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_canvas_max_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_canvas_max_layer" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <i64 as sys::GodotFfi>::sys_const(&max_layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_ambient_light(
            &mut self,
            env: Rid,
            color: Color,
            ambient: rendering_server::EnvironmentAmbientSource,
            energy: f64,
            sky_contibution: f64,
            reflection_source: rendering_server::EnvironmentReflectionSource,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_ambient_light");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    362573166i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_ambient_light" , 362573166i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <rendering_server::EnvironmentAmbientSource as sys::GodotFfi>::sys_const(
                        &ambient,
                    ),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                    <f64 as sys::GodotFfi>::sys_const(&sky_contibution),
                    <rendering_server::EnvironmentReflectionSource as sys::GodotFfi>::sys_const(
                        &reflection_source,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_glow(
            &mut self,
            env: Rid,
            enable: bool,
            levels: PackedFloat32Array,
            intensity: f64,
            strength: f64,
            mix: f64,
            bloom_threshold: f64,
            blend_mode: rendering_server::EnvironmentGlowBlendMode,
            hdr_bleed_threshold: f64,
            hdr_bleed_scale: f64,
            hdr_luminance_cap: f64,
            glow_map_strength: f64,
            glow_map: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_glow");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2421724940i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_glow" , 2421724940i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <PackedFloat32Array as sys::GodotFfi>::sys_const(&levels),
                    <f64 as sys::GodotFfi>::sys_const(&intensity),
                    <f64 as sys::GodotFfi>::sys_const(&strength),
                    <f64 as sys::GodotFfi>::sys_const(&mix),
                    <f64 as sys::GodotFfi>::sys_const(&bloom_threshold),
                    <rendering_server::EnvironmentGlowBlendMode as sys::GodotFfi>::sys_const(
                        &blend_mode,
                    ),
                    <f64 as sys::GodotFfi>::sys_const(&hdr_bleed_threshold),
                    <f64 as sys::GodotFfi>::sys_const(&hdr_bleed_scale),
                    <f64 as sys::GodotFfi>::sys_const(&hdr_luminance_cap),
                    <f64 as sys::GodotFfi>::sys_const(&glow_map_strength),
                    <Rid as sys::GodotFfi>::sys_const(&glow_map),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_tonemap(
            &mut self,
            env: Rid,
            tone_mapper: rendering_server::EnvironmentToneMapper,
            exposure: f64,
            white: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_tonemap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2914312638i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_tonemap" , 2914312638i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <rendering_server::EnvironmentToneMapper as sys::GodotFfi>::sys_const(
                        &tone_mapper,
                    ),
                    <f64 as sys::GodotFfi>::sys_const(&exposure),
                    <f64 as sys::GodotFfi>::sys_const(&white),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_adjustment(
            &mut self,
            env: Rid,
            enable: bool,
            brightness: f64,
            contrast: f64,
            saturation: f64,
            use_1d_color_correction: bool,
            color_correction: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_adjustment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    876799838i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_adjustment" , 876799838i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <f64 as sys::GodotFfi>::sys_const(&brightness),
                    <f64 as sys::GodotFfi>::sys_const(&contrast),
                    <f64 as sys::GodotFfi>::sys_const(&saturation),
                    <bool as sys::GodotFfi>::sys_const(&use_1d_color_correction),
                    <Rid as sys::GodotFfi>::sys_const(&color_correction),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_ssr(
            &mut self,
            env: Rid,
            enable: bool,
            max_steps: i64,
            fade_in: f64,
            fade_out: f64,
            depth_tolerance: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_ssr");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3607294374i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_ssr" , 3607294374i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <i64 as sys::GodotFfi>::sys_const(&max_steps),
                    <f64 as sys::GodotFfi>::sys_const(&fade_in),
                    <f64 as sys::GodotFfi>::sys_const(&fade_out),
                    <f64 as sys::GodotFfi>::sys_const(&depth_tolerance),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_ssao(
            &mut self,
            env: Rid,
            enable: bool,
            radius: f64,
            intensity: f64,
            power: f64,
            detail: f64,
            horizon: f64,
            sharpness: f64,
            light_affect: f64,
            ao_channel_affect: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_ssao");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3994732740i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_ssao" , 3994732740i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                    <f64 as sys::GodotFfi>::sys_const(&intensity),
                    <f64 as sys::GodotFfi>::sys_const(&power),
                    <f64 as sys::GodotFfi>::sys_const(&detail),
                    <f64 as sys::GodotFfi>::sys_const(&horizon),
                    <f64 as sys::GodotFfi>::sys_const(&sharpness),
                    <f64 as sys::GodotFfi>::sys_const(&light_affect),
                    <f64 as sys::GodotFfi>::sys_const(&ao_channel_affect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_fog(
            &mut self,
            env: Rid,
            enable: bool,
            light_color: Color,
            light_energy: f64,
            sun_scatter: f64,
            density: f64,
            height: f64,
            height_density: f64,
            aerial_perspective: f64,
            sky_affect: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_fog");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2793577733i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_fog" , 2793577733i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <Color as sys::GodotFfi>::sys_const(&light_color),
                    <f64 as sys::GodotFfi>::sys_const(&light_energy),
                    <f64 as sys::GodotFfi>::sys_const(&sun_scatter),
                    <f64 as sys::GodotFfi>::sys_const(&density),
                    <f64 as sys::GodotFfi>::sys_const(&height),
                    <f64 as sys::GodotFfi>::sys_const(&height_density),
                    <f64 as sys::GodotFfi>::sys_const(&aerial_perspective),
                    <f64 as sys::GodotFfi>::sys_const(&sky_affect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sdfgi(
            &mut self,
            env: Rid,
            enable: bool,
            cascades: i64,
            min_cell_size: f64,
            y_scale: rendering_server::EnvironmentSDFGIYScale,
            use_occlusion: bool,
            bounce_feedback: f64,
            read_sky: bool,
            energy: f64,
            normal_bias: f64,
            probe_bias: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_sdfgi");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3519144388i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sdfgi" , 3519144388i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <i64 as sys::GodotFfi>::sys_const(&cascades),
                    <f64 as sys::GodotFfi>::sys_const(&min_cell_size),
                    <rendering_server::EnvironmentSDFGIYScale as sys::GodotFfi>::sys_const(
                        &y_scale,
                    ),
                    <bool as sys::GodotFfi>::sys_const(&use_occlusion),
                    <f64 as sys::GodotFfi>::sys_const(&bounce_feedback),
                    <bool as sys::GodotFfi>::sys_const(&read_sky),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                    <f64 as sys::GodotFfi>::sys_const(&normal_bias),
                    <f64 as sys::GodotFfi>::sys_const(&probe_bias),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_volumetric_fog(
            &mut self,
            env: Rid,
            enable: bool,
            density: f64,
            albedo: Color,
            emission: Color,
            emission_energy: f64,
            anisotropy: f64,
            length: f64,
            p_detail_spread: f64,
            gi_inject: f64,
            temporal_reprojection: bool,
            temporal_reprojection_amount: f64,
            ambient_inject: f64,
            sky_affect: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_volumetric_fog");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1553633833i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_volumetric_fog" , 1553633833i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&env),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <f64 as sys::GodotFfi>::sys_const(&density),
                    <Color as sys::GodotFfi>::sys_const(&albedo),
                    <Color as sys::GodotFfi>::sys_const(&emission),
                    <f64 as sys::GodotFfi>::sys_const(&emission_energy),
                    <f64 as sys::GodotFfi>::sys_const(&anisotropy),
                    <f64 as sys::GodotFfi>::sys_const(&length),
                    <f64 as sys::GodotFfi>::sys_const(&p_detail_spread),
                    <f64 as sys::GodotFfi>::sys_const(&gi_inject),
                    <bool as sys::GodotFfi>::sys_const(&temporal_reprojection),
                    <f64 as sys::GodotFfi>::sys_const(&temporal_reprojection_amount),
                    <f64 as sys::GodotFfi>::sys_const(&ambient_inject),
                    <f64 as sys::GodotFfi>::sys_const(&sky_affect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_glow_set_use_bicubic_upscale(&mut self, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_glow_set_use_bicubic_upscale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_glow_set_use_bicubic_upscale" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_ssr_roughness_quality(
            &mut self,
            quality: rendering_server::EnvironmentSSRRoughnessQuality,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_ssr_roughness_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1190026788i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_ssr_roughness_quality" , 1190026788i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::EnvironmentSSRRoughnessQuality as sys::GodotFfi>::sys_const(
                        &quality,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_ssao_quality(
            &mut self,
            quality: rendering_server::EnvironmentSSAOQuality,
            half_size: bool,
            adaptive_target: f64,
            blur_passes: i64,
            fadeout_from: f64,
            fadeout_to: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_ssao_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    189753569i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_ssao_quality" , 189753569i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::EnvironmentSSAOQuality as sys::GodotFfi>::sys_const(
                        &quality,
                    ),
                    <bool as sys::GodotFfi>::sys_const(&half_size),
                    <f64 as sys::GodotFfi>::sys_const(&adaptive_target),
                    <i64 as sys::GodotFfi>::sys_const(&blur_passes),
                    <f64 as sys::GodotFfi>::sys_const(&fadeout_from),
                    <f64 as sys::GodotFfi>::sys_const(&fadeout_to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_ssil_quality(
            &mut self,
            quality: rendering_server::EnvironmentSSILQuality,
            half_size: bool,
            adaptive_target: f64,
            blur_passes: i64,
            fadeout_from: f64,
            fadeout_to: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_ssil_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1713836683i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_ssil_quality" , 1713836683i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::EnvironmentSSILQuality as sys::GodotFfi>::sys_const(
                        &quality,
                    ),
                    <bool as sys::GodotFfi>::sys_const(&half_size),
                    <f64 as sys::GodotFfi>::sys_const(&adaptive_target),
                    <i64 as sys::GodotFfi>::sys_const(&blur_passes),
                    <f64 as sys::GodotFfi>::sys_const(&fadeout_from),
                    <f64 as sys::GodotFfi>::sys_const(&fadeout_to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sdfgi_ray_count(
            &mut self,
            ray_count: rendering_server::EnvironmentSDFGIRayCount,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_sdfgi_ray_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    340137951i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sdfgi_ray_count" , 340137951i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::EnvironmentSDFGIRayCount as sys::GodotFfi>::sys_const(
                        &ray_count,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sdfgi_frames_to_converge(
            &mut self,
            frames: rendering_server::EnvironmentSDFGIFramesToConverge,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_sdfgi_frames_to_converge");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2182444374i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sdfgi_frames_to_converge" , 2182444374i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< rendering_server :: EnvironmentSDFGIFramesToConverge as sys :: GodotFfi > :: sys_const (& frames)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_sdfgi_frames_to_update_light(
            &mut self,
            frames: rendering_server::EnvironmentSDFGIFramesToUpdateLight,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("environment_set_sdfgi_frames_to_update_light");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1251144068i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_sdfgi_frames_to_update_light" , 1251144068i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [< rendering_server :: EnvironmentSDFGIFramesToUpdateLight as sys :: GodotFfi > :: sys_const (& frames)] ;
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_volumetric_fog_volume_size(&mut self, size: i64, depth: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_set_volumetric_fog_volume_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_volumetric_fog_volume_size" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&depth),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_set_volumetric_fog_filter_active(&mut self, active: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("environment_set_volumetric_fog_filter_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_set_volumetric_fog_filter_active" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&active)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn environment_bake_panorama(
            &mut self,
            environment: Rid,
            bake_irradiance: bool,
            size: Vector2i,
        ) -> Option<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("environment_bake_panorama");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2452908646i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "environment_bake_panorama" , 2452908646i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&environment),
                    <bool as sys::GodotFfi>::sys_const(&bake_irradiance),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<Image>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_space_roughness_limiter_set_active(
            &mut self,
            enable: bool,
            amount: f64,
            limit: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("screen_space_roughness_limiter_set_active");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    916716790i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "screen_space_roughness_limiter_set_active" , 916716790i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <f64 as sys::GodotFfi>::sys_const(&amount),
                    <f64 as sys::GodotFfi>::sys_const(&limit),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sub_surface_scattering_set_quality(
            &mut self,
            quality: rendering_server::SubSurfaceScatteringQuality,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sub_surface_scattering_set_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    64571803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sub_surface_scattering_set_quality" , 64571803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::SubSurfaceScatteringQuality as sys::GodotFfi>::sys_const(
                        &quality,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sub_surface_scattering_set_scale(&mut self, scale: f64, depth_scale: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("sub_surface_scattering_set_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1017552074i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "sub_surface_scattering_set_scale" , 1017552074i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                    <f64 as sys::GodotFfi>::sys_const(&depth_scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_attributes_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_attributes_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_attributes_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn camera_attributes_set_dof_blur_quality(
            &mut self,
            quality: rendering_server::DOFBlurQuality,
            use_jitter: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_attributes_set_dof_blur_quality");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2220136795i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_attributes_set_dof_blur_quality" , 2220136795i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_server::DOFBlurQuality as sys::GodotFfi>::sys_const(&quality),
                    <bool as sys::GodotFfi>::sys_const(&use_jitter),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_attributes_set_dof_blur_bokeh_shape(
            &mut self,
            shape: rendering_server::DOFBokehShape,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_attributes_set_dof_blur_bokeh_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1205058394i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_attributes_set_dof_blur_bokeh_shape" , 1205058394i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<rendering_server::DOFBokehShape as sys::GodotFfi>::sys_const(&shape)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_attributes_set_dof_blur(
            &mut self,
            camera_attributes: Rid,
            far_enable: bool,
            far_distance: f64,
            far_transition: f64,
            near_enable: bool,
            near_distance: f64,
            near_transition: f64,
            amount: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_attributes_set_dof_blur");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    316272616i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_attributes_set_dof_blur" , 316272616i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera_attributes),
                    <bool as sys::GodotFfi>::sys_const(&far_enable),
                    <f64 as sys::GodotFfi>::sys_const(&far_distance),
                    <f64 as sys::GodotFfi>::sys_const(&far_transition),
                    <bool as sys::GodotFfi>::sys_const(&near_enable),
                    <f64 as sys::GodotFfi>::sys_const(&near_distance),
                    <f64 as sys::GodotFfi>::sys_const(&near_transition),
                    <f64 as sys::GodotFfi>::sys_const(&amount),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_attributes_set_exposure(
            &mut self,
            camera_attributes: Rid,
            multiplier: f64,
            normalization: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_attributes_set_exposure");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2513314492i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_attributes_set_exposure" , 2513314492i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera_attributes),
                    <f64 as sys::GodotFfi>::sys_const(&multiplier),
                    <f64 as sys::GodotFfi>::sys_const(&normalization),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn camera_attributes_set_auto_exposure(
            &mut self,
            camera_attributes: Rid,
            enable: bool,
            min_sensitivity: f64,
            max_sensitivity: f64,
            speed: f64,
            scale: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("camera_attributes_set_auto_exposure");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4266986332i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "camera_attributes_set_auto_exposure" , 4266986332i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&camera_attributes),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <f64 as sys::GodotFfi>::sys_const(&min_sensitivity),
                    <f64 as sys::GodotFfi>::sys_const(&max_sensitivity),
                    <f64 as sys::GodotFfi>::sys_const(&speed),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn scenario_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("scenario_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "scenario_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn scenario_set_environment(&mut self, scenario: Rid, environment: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("scenario_set_environment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "scenario_set_environment" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                    <Rid as sys::GodotFfi>::sys_const(&environment),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn scenario_set_fallback_environment(&mut self, scenario: Rid, environment: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("scenario_set_fallback_environment");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "scenario_set_fallback_environment" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                    <Rid as sys::GodotFfi>::sys_const(&environment),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn scenario_set_camera_attributes(&mut self, scenario: Rid, effects: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("scenario_set_camera_attributes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "scenario_set_camera_attributes" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                    <Rid as sys::GodotFfi>::sys_const(&effects),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_create2(&mut self, base: Rid, scenario: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_create2");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    746547085i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_create2" , 746547085i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&base),
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instance_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instance_set_base(&mut self, instance: Rid, base: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_base");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_base" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&base),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_scenario(&mut self, instance: Rid, scenario: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_scenario");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_scenario" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_layer_mask(&mut self, instance: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_layer_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_layer_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_pivot_data(
            &mut self,
            instance: Rid,
            sorting_offset: f64,
            use_aabb_center: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_pivot_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1280615259i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_pivot_data" , 1280615259i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <f64 as sys::GodotFfi>::sys_const(&sorting_offset),
                    <bool as sys::GodotFfi>::sys_const(&use_aabb_center),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_transform(&mut self, instance: Rid, transform: Transform3D) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3935195649i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_transform" , 3935195649i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Transform3D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_attach_object_instance_id(&mut self, instance: Rid, id: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_attach_object_instance_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_attach_object_instance_id" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <i64 as sys::GodotFfi>::sys_const(&id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_blend_shape_weight(&mut self, instance: Rid, shape: i64, weight: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_blend_shape_weight");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1892459533i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_blend_shape_weight" , 1892459533i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <i64 as sys::GodotFfi>::sys_const(&shape),
                    <f64 as sys::GodotFfi>::sys_const(&weight),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_surface_override_material(
            &mut self,
            instance: Rid,
            surface: i64,
            material: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_surface_override_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2310537182i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_surface_override_material" , 2310537182i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <i64 as sys::GodotFfi>::sys_const(&surface),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_visible(&mut self, instance: Rid, visible: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_visible" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <bool as sys::GodotFfi>::sys_const(&visible),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_transparency(&mut self, instance: Rid, transparency: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_transparency");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_transparency" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <f64 as sys::GodotFfi>::sys_const(&transparency),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_custom_aabb(&mut self, instance: Rid, aabb: Aabb) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_custom_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3696536120i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_custom_aabb" , 3696536120i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Aabb as sys::GodotFfi>::sys_const(&aabb),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_attach_skeleton(&mut self, instance: Rid, skeleton: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_attach_skeleton");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_attach_skeleton" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&skeleton),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_extra_visibility_margin(&mut self, instance: Rid, margin: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_extra_visibility_margin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_extra_visibility_margin" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <f64 as sys::GodotFfi>::sys_const(&margin),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_visibility_parent(&mut self, instance: Rid, parent: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_visibility_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_visibility_parent" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&parent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_set_ignore_culling(&mut self, instance: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_set_ignore_culling");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_set_ignore_culling" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_flag(
            &mut self,
            instance: Rid,
            flag: rendering_server::InstanceFlags,
            enabled: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_flag");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1014989537i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_flag" , 1014989537i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <rendering_server::InstanceFlags as sys::GodotFfi>::sys_const(&flag),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_cast_shadows_setting(
            &mut self,
            instance: Rid,
            shadow_casting_setting: rendering_server::ShadowCastingSetting,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_cast_shadows_setting");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3768836020i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_cast_shadows_setting" , 3768836020i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <rendering_server::ShadowCastingSetting as sys::GodotFfi>::sys_const(
                        &shadow_casting_setting,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_material_override(&mut self, instance: Rid, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_material_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_material_override" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_material_overlay(&mut self, instance: Rid, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_material_overlay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_material_overlay" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_visibility_range(
            &mut self,
            instance: Rid,
            min: f64,
            max: f64,
            min_margin: f64,
            max_margin: f64,
            fade_mode: rendering_server::VisibilityRangeFadeMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_visibility_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4263925858i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_visibility_range" , 4263925858i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <f64 as sys::GodotFfi>::sys_const(&min),
                    <f64 as sys::GodotFfi>::sys_const(&max),
                    <f64 as sys::GodotFfi>::sys_const(&min_margin),
                    <f64 as sys::GodotFfi>::sys_const(&max_margin),
                    <rendering_server::VisibilityRangeFadeMode as sys::GodotFfi>::sys_const(
                        &fade_mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_lightmap(
            &mut self,
            instance: Rid,
            lightmap: Rid,
            lightmap_uv_scale: Rect2,
            lightmap_slice: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_lightmap");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    536974962i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_lightmap" , 536974962i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <Rid as sys::GodotFfi>::sys_const(&lightmap),
                    <Rect2 as sys::GodotFfi>::sys_const(&lightmap_uv_scale),
                    <i64 as sys::GodotFfi>::sys_const(&lightmap_slice),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_lod_bias(&mut self, instance: Rid, lod_bias: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_lod_bias");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_lod_bias" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <f64 as sys::GodotFfi>::sys_const(&lod_bias),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_set_shader_parameter(
            &mut self,
            instance: Rid,
            parameter: StringName,
            value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_set_shader_parameter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3477296213i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_set_shader_parameter" , 3477296213i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <StringName as sys::GodotFfi>::sys_const(&parameter),
                    <Variant as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn instance_geometry_get_shader_parameter(
            &self,
            instance: Rid,
            parameter: StringName,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_get_shader_parameter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2621281810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_get_shader_parameter" , 2621281810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <StringName as sys::GodotFfi>::sys_const(&parameter),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instance_geometry_get_shader_parameter_default_value(
            &self,
            instance: Rid,
            parameter: StringName,
        ) -> Variant {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name =
                    StringName::from("instance_geometry_get_shader_parameter_default_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2621281810i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_get_shader_parameter_default_value" , 2621281810i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&instance),
                    <StringName as sys::GodotFfi>::sys_const(&parameter),
                ];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instance_geometry_get_shader_parameter_list(
            &self,
            instance: Rid,
        ) -> Array<Dictionary> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instance_geometry_get_shader_parameter_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2684255073i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instance_geometry_get_shader_parameter_list" , 2684255073i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&instance)];
                let __args_ptr = __args.as_ptr();
                <Array<Dictionary> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instances_cull_aabb(&self, aabb: Aabb, scenario: Rid) -> PackedInt64Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instances_cull_aabb");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2031554939i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instances_cull_aabb" , 2031554939i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Aabb as sys::GodotFfi>::sys_const(&aabb),
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instances_cull_ray(
            &self,
            from: Vector3,
            to: Vector3,
            scenario: Rid,
        ) -> PackedInt64Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instances_cull_ray");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3388524336i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instances_cull_ray" , 3388524336i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector3 as sys::GodotFfi>::sys_const(&from),
                    <Vector3 as sys::GodotFfi>::sys_const(&to),
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn instances_cull_convex(
            &self,
            convex: Array<Plane>,
            scenario: Rid,
        ) -> PackedInt64Array {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("instances_cull_convex");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690700105i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "instances_cull_convex" , 3690700105i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Plane> as sys::GodotFfi>::sys_const(&convex),
                    <Rid as sys::GodotFfi>::sys_const(&scenario),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn bake_render_uv2(
            &mut self,
            base: Rid,
            material_overrides: Array<Rid>,
            image_size: Vector2i,
        ) -> Array<Gd<Image>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("bake_render_uv2");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1904608558i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "bake_render_uv2" , 1904608558i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&base),
                    <Array<Rid> as sys::GodotFfi>::sys_const(&material_overrides),
                    <Vector2i as sys::GodotFfi>::sys_const(&image_size),
                ];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Image>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_set_item_mirroring(&mut self, canvas: Rid, item: Rid, mirroring: Vector2) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_set_item_mirroring");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2343975398i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_set_item_mirroring" , 2343975398i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Vector2 as sys::GodotFfi>::sys_const(&mirroring),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_set_modulate(&mut self, canvas: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_set_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_set_modulate" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_set_disable_scale(&mut self, disable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_set_disable_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_set_disable_scale" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&disable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_texture_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_texture_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_texture_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_texture_set_channel(
            &mut self,
            canvas_texture: Rid,
            channel: rendering_server::CanvasTextureChannel,
            texture: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_texture_set_channel");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3822119138i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_texture_set_channel" , 3822119138i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_texture),
                    <rendering_server::CanvasTextureChannel as sys::GodotFfi>::sys_const(&channel),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_texture_set_shading_parameters(
            &mut self,
            canvas_texture: Rid,
            base_color: Color,
            shininess: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_texture_set_shading_parameters");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2124967469i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_texture_set_shading_parameters" , 2124967469i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_texture),
                    <Color as sys::GodotFfi>::sys_const(&base_color),
                    <f64 as sys::GodotFfi>::sys_const(&shininess),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_texture_set_texture_filter(
            &mut self,
            canvas_texture: Rid,
            filter: rendering_server::CanvasItemTextureFilter,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_texture_set_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1155129294i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_texture_set_texture_filter" , 1155129294i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_texture),
                    <rendering_server::CanvasItemTextureFilter as sys::GodotFfi>::sys_const(
                        &filter,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_texture_set_texture_repeat(
            &mut self,
            canvas_texture: Rid,
            repeat: rendering_server::CanvasItemTextureRepeat,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_texture_set_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1652956681i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_texture_set_texture_repeat" , 1652956681i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&canvas_texture),
                    <rendering_server::CanvasItemTextureRepeat as sys::GodotFfi>::sys_const(
                        &repeat,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_item_set_parent(&mut self, item: Rid, parent: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_parent" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rid as sys::GodotFfi>::sys_const(&parent),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_default_texture_filter(
            &mut self,
            item: Rid,
            filter: rendering_server::CanvasItemTextureFilter,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_default_texture_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1155129294i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_default_texture_filter" , 1155129294i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <rendering_server::CanvasItemTextureFilter as sys::GodotFfi>::sys_const(
                        &filter,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_default_texture_repeat(
            &mut self,
            item: Rid,
            repeat: rendering_server::CanvasItemTextureRepeat,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_default_texture_repeat");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1652956681i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_default_texture_repeat" , 1652956681i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <rendering_server::CanvasItemTextureRepeat as sys::GodotFfi>::sys_const(
                        &repeat,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_visible(&mut self, item: Rid, visible: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_visible");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_visible" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&visible),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_light_mask(&mut self, item: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_light_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_light_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_visibility_layer(&mut self, item: Rid, visibility_layer: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_visibility_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_visibility_layer" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <i64 as sys::GodotFfi>::sys_const(&visibility_layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_transform(&mut self, item: Rid, transform: Transform2D) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_transform" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_clip(&mut self, item: Rid, clip: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_clip");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_clip" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&clip),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_distance_field_mode(&mut self, item: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_distance_field_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_distance_field_mode" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_custom_rect(
            &mut self,
            item: Rid,
            use_custom_rect: bool,
            rect: Rect2,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_custom_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2180266943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_custom_rect" , 2180266943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&use_custom_rect),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_modulate(&mut self, item: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_modulate" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_self_modulate(&mut self, item: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_self_modulate");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_self_modulate" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_draw_behind_parent(&mut self, item: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_draw_behind_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_draw_behind_parent" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_line(
            &mut self,
            item: Rid,
            from: Vector2,
            to: Vector2,
            color: Color,
            width: f64,
            antialiased: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_line");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2843922985i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_line" , 2843922985i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Vector2 as sys::GodotFfi>::sys_const(&from),
                    <Vector2 as sys::GodotFfi>::sys_const(&to),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <bool as sys::GodotFfi>::sys_const(&antialiased),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_polyline(
            &mut self,
            item: Rid,
            points: PackedVector2Array,
            colors: PackedColorArray,
            width: f64,
            antialiased: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_polyline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3438017257i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_polyline" , 3438017257i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <f64 as sys::GodotFfi>::sys_const(&width),
                    <bool as sys::GodotFfi>::sys_const(&antialiased),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_rect(&mut self, item: Rid, rect: Rect2, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    934531857i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_rect" , 934531857i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_circle(
            &mut self,
            item: Rid,
            pos: Vector2,
            radius: f64,
            color: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_circle");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2439351960i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_circle" , 2439351960i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Vector2 as sys::GodotFfi>::sys_const(&pos),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_texture_rect(
            &mut self,
            item: Rid,
            rect: Rect2,
            texture: Rid,
            tile: bool,
            modulate: Color,
            transpose: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_texture_rect");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3205360868i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_texture_rect" , 3205360868i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <bool as sys::GodotFfi>::sys_const(&tile),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_msdf_texture_rect_region(
            &mut self,
            item: Rid,
            rect: Rect2,
            texture: Rid,
            src_rect: Rect2,
            modulate: Color,
            outline_size: i64,
            px_range: f64,
            scale: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_msdf_texture_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    349157222i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_msdf_texture_rect_region" , 349157222i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <i64 as sys::GodotFfi>::sys_const(&outline_size),
                    <f64 as sys::GodotFfi>::sys_const(&px_range),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_lcd_texture_rect_region(
            &mut self,
            item: Rid,
            rect: Rect2,
            texture: Rid,
            src_rect: Rect2,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_lcd_texture_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    359793297i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_lcd_texture_rect_region" , 359793297i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_texture_rect_region(
            &mut self,
            item: Rid,
            rect: Rect2,
            texture: Rid,
            src_rect: Rect2,
            modulate: Color,
            transpose: bool,
            clip_uv: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_texture_rect_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2782979504i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_texture_rect_region" , 2782979504i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Rect2 as sys::GodotFfi>::sys_const(&src_rect),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <bool as sys::GodotFfi>::sys_const(&transpose),
                    <bool as sys::GodotFfi>::sys_const(&clip_uv),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_nine_patch(
            &mut self,
            item: Rid,
            rect: Rect2,
            source: Rect2,
            texture: Rid,
            topleft: Vector2,
            bottomright: Vector2,
            x_axis_mode: rendering_server::NinePatchAxisMode,
            y_axis_mode: rendering_server::NinePatchAxisMode,
            draw_center: bool,
            modulate: Color,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_nine_patch");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    904428547i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_nine_patch" , 904428547i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                    <Rect2 as sys::GodotFfi>::sys_const(&source),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Vector2 as sys::GodotFfi>::sys_const(&topleft),
                    <Vector2 as sys::GodotFfi>::sys_const(&bottomright),
                    <rendering_server::NinePatchAxisMode as sys::GodotFfi>::sys_const(&x_axis_mode),
                    <rendering_server::NinePatchAxisMode as sys::GodotFfi>::sys_const(&y_axis_mode),
                    <bool as sys::GodotFfi>::sys_const(&draw_center),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_primitive(
            &mut self,
            item: Rid,
            points: PackedVector2Array,
            colors: PackedColorArray,
            uvs: PackedVector2Array,
            texture: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_primitive");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3731601077i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_primitive" , 3731601077i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_polygon(
            &mut self,
            item: Rid,
            points: PackedVector2Array,
            colors: PackedColorArray,
            uvs: PackedVector2Array,
            texture: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2907936855i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_polygon" , 2907936855i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_triangle_array(
            &mut self,
            item: Rid,
            indices: PackedInt32Array,
            points: PackedVector2Array,
            colors: PackedColorArray,
            uvs: PackedVector2Array,
            bones: PackedInt32Array,
            weights: PackedFloat32Array,
            texture: Rid,
            count: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_triangle_array");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    749685193i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_triangle_array" , 749685193i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&indices),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&points),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&colors),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&uvs),
                    <PackedInt32Array as sys::GodotFfi>::sys_const(&bones),
                    <PackedFloat32Array as sys::GodotFfi>::sys_const(&weights),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <i64 as sys::GodotFfi>::sys_const(&count),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_mesh(
            &mut self,
            item: Rid,
            mesh: Rid,
            transform: Transform2D,
            modulate: Color,
            texture: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3548053052i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_mesh" , 3548053052i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                    <Color as sys::GodotFfi>::sys_const(&modulate),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_multimesh(&mut self, item: Rid, mesh: Rid, texture: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_multimesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1541595251i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_multimesh" , 1541595251i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rid as sys::GodotFfi>::sys_const(&mesh),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_particles(&mut self, item: Rid, particles: Rid, texture: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_particles");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2575754278i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_particles" , 2575754278i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rid as sys::GodotFfi>::sys_const(&particles),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_set_transform(&mut self, item: Rid, transform: Transform2D) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_set_transform" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_clip_ignore(&mut self, item: Rid, ignore: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_clip_ignore");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_clip_ignore" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&ignore),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_add_animation_slice(
            &mut self,
            item: Rid,
            animation_length: f64,
            slice_begin: f64,
            slice_end: f64,
            offset: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_add_animation_slice");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4107531031i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_add_animation_slice" , 4107531031i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <f64 as sys::GodotFfi>::sys_const(&animation_length),
                    <f64 as sys::GodotFfi>::sys_const(&slice_begin),
                    <f64 as sys::GodotFfi>::sys_const(&slice_end),
                    <f64 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_sort_children_by_y(&mut self, item: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_sort_children_by_y");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_sort_children_by_y" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_z_index(&mut self, item: Rid, z_index: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_z_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_z_index" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <i64 as sys::GodotFfi>::sys_const(&z_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_z_as_relative_to_parent(&mut self, item: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_z_as_relative_to_parent");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_z_as_relative_to_parent" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_copy_to_backbuffer(
            &mut self,
            item: Rid,
            enabled: bool,
            rect: Rect2,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_copy_to_backbuffer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2429202503i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_copy_to_backbuffer" , 2429202503i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_clear(&mut self, item: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_clear" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&item)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_draw_index(&mut self, item: Rid, index: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_draw_index");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_draw_index" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_material(&mut self, item: Rid, material: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_material" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <Rid as sys::GodotFfi>::sys_const(&material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_use_parent_material(&mut self, item: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_use_parent_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_use_parent_material" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_visibility_notifier(
            &mut self,
            item: Rid,
            enable: bool,
            area: Rect2,
            enter_callable: Callable,
            exit_callable: Callable,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_visibility_notifier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3568945579i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_visibility_notifier" , 3568945579i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                    <Rect2 as sys::GodotFfi>::sys_const(&area),
                    <Callable as sys::GodotFfi>::sys_const(&enter_callable),
                    <Callable as sys::GodotFfi>::sys_const(&exit_callable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_item_set_canvas_group_mode(
            &mut self,
            item: Rid,
            mode: rendering_server::CanvasGroupMode,
            clear_margin: f64,
            fit_empty: bool,
            fit_margin: f64,
            blur_mipmaps: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_item_set_canvas_group_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1568036344i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_item_set_canvas_group_mode" , 1568036344i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&item),
                    <rendering_server::CanvasGroupMode as sys::GodotFfi>::sys_const(&mode),
                    <f64 as sys::GodotFfi>::sys_const(&clear_margin),
                    <bool as sys::GodotFfi>::sys_const(&fit_empty),
                    <f64 as sys::GodotFfi>::sys_const(&fit_margin),
                    <bool as sys::GodotFfi>::sys_const(&blur_mipmaps),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_light_attach_to_canvas(&mut self, light: Rid, canvas: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_attach_to_canvas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_attach_to_canvas" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_enabled(&mut self, light: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_enabled" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_texture_scale(&mut self, light: Rid, scale: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_texture_scale");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_texture_scale" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <f64 as sys::GodotFfi>::sys_const(&scale),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_transform(&mut self, light: Rid, transform: Transform2D) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_transform" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_texture(&mut self, light: Rid, texture: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_texture" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_texture_offset(&mut self, light: Rid, offset: Vector2) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_texture_offset");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3201125042i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_texture_offset" , 3201125042i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Vector2 as sys::GodotFfi>::sys_const(&offset),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_color(&mut self, light: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_color" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_height(&mut self, light: Rid, height: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_height" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <f64 as sys::GodotFfi>::sys_const(&height),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_energy(&mut self, light: Rid, energy: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_energy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_energy" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <f64 as sys::GodotFfi>::sys_const(&energy),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_z_range(&mut self, light: Rid, min_z: i64, max_z: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_z_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4288446313i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_z_range" , 4288446313i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <i64 as sys::GodotFfi>::sys_const(&min_z),
                    <i64 as sys::GodotFfi>::sys_const(&max_z),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_layer_range(&mut self, light: Rid, min_layer: i64, max_layer: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_layer_range");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4288446313i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_layer_range" , 4288446313i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <i64 as sys::GodotFfi>::sys_const(&min_layer),
                    <i64 as sys::GodotFfi>::sys_const(&max_layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_item_cull_mask(&mut self, light: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_item_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_item_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_item_shadow_cull_mask(&mut self, light: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_item_shadow_cull_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_item_shadow_cull_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_mode(
            &mut self,
            light: Rid,
            mode: rendering_server::CanvasLightMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2957564891i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_mode" , 2957564891i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::CanvasLightMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_shadow_enabled(&mut self, light: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_shadow_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_shadow_enabled" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_shadow_filter(
            &mut self,
            light: Rid,
            filter: rendering_server::CanvasLightShadowFilter,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_shadow_filter");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    393119659i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_shadow_filter" , 393119659i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::CanvasLightShadowFilter as sys::GodotFfi>::sys_const(
                        &filter,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_shadow_color(&mut self, light: Rid, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_shadow_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2948539648i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_shadow_color" , 2948539648i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_shadow_smooth(&mut self, light: Rid, smooth: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_shadow_smooth");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1794382983i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_shadow_smooth" , 1794382983i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <f64 as sys::GodotFfi>::sys_const(&smooth),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_set_blend_mode(
            &mut self,
            light: Rid,
            mode: rendering_server::CanvasLightBlendMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_set_blend_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    804895945i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_set_blend_mode" , 804895945i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&light),
                    <rendering_server::CanvasLightBlendMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_occluder_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_light_occluder_attach_to_canvas(&mut self, occluder: Rid, canvas: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_attach_to_canvas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_attach_to_canvas" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <Rid as sys::GodotFfi>::sys_const(&canvas),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_occluder_set_enabled(&mut self, occluder: Rid, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_set_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_set_enabled" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <bool as sys::GodotFfi>::sys_const(&enabled),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_occluder_set_polygon(&mut self, occluder: Rid, polygon: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_set_polygon");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    395945892i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_set_polygon" , 395945892i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <Rid as sys::GodotFfi>::sys_const(&polygon),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_occluder_set_as_sdf_collision(&mut self, occluder: Rid, enable: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_set_as_sdf_collision");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1265174801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_set_as_sdf_collision" , 1265174801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <bool as sys::GodotFfi>::sys_const(&enable),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_occluder_set_transform(
            &mut self,
            occluder: Rid,
            transform: Transform2D,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_set_transform");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1246044741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_set_transform" , 1246044741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <Transform2D as sys::GodotFfi>::sys_const(&transform),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_light_occluder_set_light_mask(&mut self, occluder: Rid, mask: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_light_occluder_set_light_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3411492887i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_light_occluder_set_light_mask" , 3411492887i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_occluder_polygon_create(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_occluder_polygon_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_occluder_polygon_create" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn canvas_occluder_polygon_set_shape(
            &mut self,
            occluder_polygon: Rid,
            shape: PackedVector2Array,
            closed: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_occluder_polygon_set_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2103882027i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_occluder_polygon_set_shape" , 2103882027i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder_polygon),
                    <PackedVector2Array as sys::GodotFfi>::sys_const(&shape),
                    <bool as sys::GodotFfi>::sys_const(&closed),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_occluder_polygon_set_cull_mode(
            &mut self,
            occluder_polygon: Rid,
            mode: rendering_server::CanvasOccluderPolygonCullMode,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_occluder_polygon_set_cull_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1839404663i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_occluder_polygon_set_cull_mode" , 1839404663i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&occluder_polygon),
                    <rendering_server::CanvasOccluderPolygonCullMode as sys::GodotFfi>::sys_const(
                        &mode,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn canvas_set_shadow_texture_size(&mut self, size: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("canvas_set_shadow_texture_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "canvas_set_shadow_texture_size" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_shader_parameter_add(
            &mut self,
            name: StringName,
            type_: rendering_server::GlobalShaderParameterType,
            default_value: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_add");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    463390080i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_add" , 463390080i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&name),
                    <rendering_server::GlobalShaderParameterType as sys::GodotFfi>::sys_const(
                        &type_,
                    ),
                    <Variant as sys::GodotFfi>::sys_const(&default_value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_shader_parameter_remove(&mut self, name: StringName) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_remove");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3304788590i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_remove" , 3304788590i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn global_shader_parameter_get_list(&self) -> PackedStringArray {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_get_list");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1139954409i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_get_list" , 1139954409i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedStringArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_shader_parameter_set(&mut self, name: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_set" , 3776071444i64);
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
        pub fn global_shader_parameter_set_override(&mut self, name: StringName, value: Variant) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_set_override");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_set_override" , 3776071444i64);
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
        pub fn global_shader_parameter_get(&self, name: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2760726917i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_get" , 2760726917i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn global_shader_parameter_get_type(
            &self,
            name: StringName,
        ) -> rendering_server::GlobalShaderParameterType {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("global_shader_parameter_get_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1601414142i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "global_shader_parameter_get_type" , 1601414142i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                < rendering_server :: GlobalShaderParameterType as sys :: GodotFfi > :: from_sys_init_default (| return_ptr | { __call_fn (__method_bind , self . object_ptr , __args_ptr , return_ptr) ; })
            }
        }
        pub fn free_rid(&mut self, rid: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("free_rid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "free_rid" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&rid)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn request_frame_drawn_callback(&mut self, callable: Callable) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("request_frame_drawn_callback");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1611583062i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "request_frame_drawn_callback" , 1611583062i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Callable as sys::GodotFfi>::sys_const(&callable)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_changed(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("has_changed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "has_changed" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_rendering_info(&mut self, info: rendering_server::RenderingInfo) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_rendering_info");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3763192241i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_rendering_info" , 3763192241i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_server::RenderingInfo as sys::GodotFfi>::sys_const(&info)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_video_adapter_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_video_adapter_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_video_adapter_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_video_adapter_vendor(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_video_adapter_vendor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_video_adapter_vendor" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_video_adapter_type(&self) -> rendering_device::DeviceType {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_video_adapter_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3099547011i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_video_adapter_type" , 3099547011i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <rendering_device::DeviceType as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn get_video_adapter_api_version(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_video_adapter_api_version");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_video_adapter_api_version" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn make_sphere_mesh(&mut self, latitudes: i64, longitudes: i64, radius: f64) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("make_sphere_mesh");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2251015897i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "make_sphere_mesh" , 2251015897i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&latitudes),
                    <i64 as sys::GodotFfi>::sys_const(&longitudes),
                    <f64 as sys::GodotFfi>::sys_const(&radius),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_test_cube(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_test_cube");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_test_cube" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_test_texture(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_test_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_test_texture" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_white_texture(&mut self) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_white_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    529393457i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_white_texture" , 529393457i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_boot_image(
            &mut self,
            image: Gd<Image>,
            color: Color,
            scale: bool,
            use_filter: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("set_boot_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2244367877i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "set_boot_image" , 2244367877i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Image> as AsArg>::as_arg_ptr(&image),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <bool as sys::GodotFfi>::sys_const(&scale),
                    <bool as sys::GodotFfi>::sys_const(&use_filter),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_default_clear_color(&mut self) -> Color {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_default_clear_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3200896285i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_default_clear_color" , 3200896285i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_default_clear_color(&mut self, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("set_default_clear_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2920490490i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "set_default_clear_color" , 2920490490i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Color as sys::GodotFfi>::sys_const(&color)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn has_feature(&self, feature: rendering_server::Features) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("has_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    598462696i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "has_feature" , 598462696i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_server::Features as sys::GodotFfi>::sys_const(
                    &feature,
                )];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_os_feature(&self, feature: GodotString) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("has_os_feature");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3927539163i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "has_os_feature" , 3927539163i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&feature)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_debug_generate_wireframes(&mut self, generate: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("set_debug_generate_wireframes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "set_debug_generate_wireframes" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&generate)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_render_loop_enabled(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("is_render_loop_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "is_render_loop_enabled" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_render_loop_enabled(&mut self, enabled: bool) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("set_render_loop_enabled");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "set_render_loop_enabled" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&enabled)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_frame_setup_time_cpu(&self) -> f64 {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_frame_setup_time_cpu");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1740695150i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_frame_setup_time_cpu" , 1740695150i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn force_sync(&mut self) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("force_sync");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "force_sync" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn force_draw(&mut self, swap_buffers: bool, frame_step: f64) {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("force_draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    899045543i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "force_draw" , 899045543i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <bool as sys::GodotFfi>::sys_const(&swap_buffers),
                    <f64 as sys::GodotFfi>::sys_const(&frame_step),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_rendering_device(&self) -> Option<Gd<RenderingDevice>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("get_rendering_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1405107940i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "get_rendering_device" , 1405107940i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<RenderingDevice>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_local_rendering_device(&self) -> Option<Gd<RenderingDevice>> {
            unsafe {
                let __class_name = StringName::from("RenderingServer");
                let __method_name = StringName::from("create_local_rendering_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1405107940i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingServer" , "create_local_rendering_device" , 1405107940i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<RenderingDevice>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub const NO_INDEX_ARRAY: i32 = -1i32;
        pub const ARRAY_WEIGHTS_SIZE: i32 = 4i32;
        pub const CANVAS_ITEM_Z_MIN: i32 = -4096i32;
        pub const CANVAS_ITEM_Z_MAX: i32 = 4096i32;
        pub const MAX_GLOW_LEVELS: i32 = 7i32;
        pub const MAX_CURSORS: i32 = 8i32;
        pub const MAX_2D_DIRECTIONAL_LIGHTS: i32 = 8i32;
        pub const MATERIAL_RENDER_PRIORITY_MIN: i32 = -128i32;
        pub const MATERIAL_RENDER_PRIORITY_MAX: i32 = 127i32;
        pub const ARRAY_CUSTOM_COUNT: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_POSITION: i32 = 1i32;
        pub const PARTICLES_EMIT_FLAG_ROTATION_SCALE: i32 = 2i32;
        pub const PARTICLES_EMIT_FLAG_VELOCITY: i32 = 4i32;
        pub const PARTICLES_EMIT_FLAG_COLOR: i32 = 8i32;
        pub const PARTICLES_EMIT_FLAG_CUSTOM: i32 = 16i32;
    }
    impl crate::obj::GodotClass for RenderingServer {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "RenderingServer";
    }
    impl crate::obj::EngineClass for RenderingServer {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for RenderingServer {}
    impl std::ops::Deref for RenderingServer {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for RenderingServer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_RenderingServer {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::RenderingServer> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureLayeredType {
    ord: i32,
}
impl TextureLayeredType {
    pub const TEXTURE_LAYERED_2D_ARRAY: Self = Self { ord: 0 };
    pub const TEXTURE_LAYERED_CUBEMAP: Self = Self { ord: 1 };
    pub const TEXTURE_LAYERED_CUBEMAP_ARRAY: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for TextureLayeredType {
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
unsafe impl sys::GodotFfi for TextureLayeredType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CubeMapLayer {
    ord: i32,
}
impl CubeMapLayer {
    pub const CUBEMAP_LAYER_LEFT: Self = Self { ord: 0 };
    pub const CUBEMAP_LAYER_RIGHT: Self = Self { ord: 1 };
    pub const CUBEMAP_LAYER_BOTTOM: Self = Self { ord: 2 };
    pub const CUBEMAP_LAYER_TOP: Self = Self { ord: 3 };
    pub const CUBEMAP_LAYER_FRONT: Self = Self { ord: 4 };
    pub const CUBEMAP_LAYER_BACK: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for CubeMapLayer {
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
unsafe impl sys::GodotFfi for CubeMapLayer {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShaderMode {
    ord: i32,
}
impl ShaderMode {
    pub const SHADER_SPATIAL: Self = Self { ord: 0 };
    pub const SHADER_CANVAS_ITEM: Self = Self { ord: 1 };
    pub const SHADER_PARTICLES: Self = Self { ord: 2 };
    pub const SHADER_SKY: Self = Self { ord: 3 };
    pub const SHADER_FOG: Self = Self { ord: 4 };
    pub const SHADER_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for ShaderMode {
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
unsafe impl sys::GodotFfi for ShaderMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ArrayType {
    ord: i32,
}
impl ArrayType {
    pub const ARRAY_VERTEX: Self = Self { ord: 0 };
    pub const ARRAY_NORMAL: Self = Self { ord: 1 };
    pub const ARRAY_TANGENT: Self = Self { ord: 2 };
    pub const ARRAY_COLOR: Self = Self { ord: 3 };
    pub const ARRAY_TEX_UV: Self = Self { ord: 4 };
    pub const ARRAY_TEX_UV2: Self = Self { ord: 5 };
    pub const ARRAY_CUSTOM0: Self = Self { ord: 6 };
    pub const ARRAY_CUSTOM1: Self = Self { ord: 7 };
    pub const ARRAY_CUSTOM2: Self = Self { ord: 8 };
    pub const ARRAY_CUSTOM3: Self = Self { ord: 9 };
    pub const ARRAY_BONES: Self = Self { ord: 10 };
    pub const ARRAY_WEIGHTS: Self = Self { ord: 11 };
    pub const ARRAY_INDEX: Self = Self { ord: 12 };
    pub const ARRAY_MAX: Self = Self { ord: 13 };
}
impl crate::obj::EngineEnum for ArrayType {
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
            | ord @ 13i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ArrayType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ArrayCustomFormat {
    ord: i32,
}
impl ArrayCustomFormat {
    pub const ARRAY_CUSTOM_RGBA8_UNORM: Self = Self { ord: 0 };
    pub const ARRAY_CUSTOM_RGBA8_SNORM: Self = Self { ord: 1 };
    pub const ARRAY_CUSTOM_RG_HALF: Self = Self { ord: 2 };
    pub const ARRAY_CUSTOM_RGBA_HALF: Self = Self { ord: 3 };
    pub const ARRAY_CUSTOM_R_FLOAT: Self = Self { ord: 4 };
    pub const ARRAY_CUSTOM_RG_FLOAT: Self = Self { ord: 5 };
    pub const ARRAY_CUSTOM_RGB_FLOAT: Self = Self { ord: 6 };
    pub const ARRAY_CUSTOM_RGBA_FLOAT: Self = Self { ord: 7 };
    pub const ARRAY_CUSTOM_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for ArrayCustomFormat {
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
            | ord @ 8i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ArrayCustomFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct ArrayFormat {
    ord: i32,
}
impl ArrayFormat {
    pub const ARRAY_FORMAT_VERTEX: Self = Self { ord: 1 };
    pub const ARRAY_FORMAT_NORMAL: Self = Self { ord: 2 };
    pub const ARRAY_FORMAT_TANGENT: Self = Self { ord: 4 };
    pub const ARRAY_FORMAT_COLOR: Self = Self { ord: 8 };
    pub const ARRAY_FORMAT_TEX_UV: Self = Self { ord: 16 };
    pub const ARRAY_FORMAT_TEX_UV2: Self = Self { ord: 32 };
    pub const ARRAY_FORMAT_CUSTOM0: Self = Self { ord: 64 };
    pub const ARRAY_FORMAT_CUSTOM1: Self = Self { ord: 128 };
    pub const ARRAY_FORMAT_CUSTOM2: Self = Self { ord: 256 };
    pub const ARRAY_FORMAT_CUSTOM3: Self = Self { ord: 512 };
    pub const ARRAY_FORMAT_BONES: Self = Self { ord: 1024 };
    pub const ARRAY_FORMAT_WEIGHTS: Self = Self { ord: 2048 };
    pub const ARRAY_FORMAT_INDEX: Self = Self { ord: 4096 };
    pub const ARRAY_FORMAT_BLEND_SHAPE_MASK: Self = Self { ord: 7 };
    pub const ARRAY_FORMAT_CUSTOM_BASE: Self = Self { ord: 13 };
    pub const ARRAY_FORMAT_CUSTOM_BITS: Self = Self { ord: 3 };
    pub const ARRAY_FORMAT_CUSTOM0_SHIFT: Self = Self { ord: 13 };
    pub const ARRAY_FORMAT_CUSTOM1_SHIFT: Self = Self { ord: 16 };
    pub const ARRAY_FORMAT_CUSTOM2_SHIFT: Self = Self { ord: 19 };
    pub const ARRAY_FORMAT_CUSTOM3_SHIFT: Self = Self { ord: 22 };
    pub const ARRAY_FORMAT_CUSTOM_MASK: Self = Self { ord: 7 };
    pub const ARRAY_COMPRESS_FLAGS_BASE: Self = Self { ord: 25 };
    pub const ARRAY_FLAG_USE_2D_VERTICES: Self = Self { ord: 33554432 };
    pub const ARRAY_FLAG_USE_DYNAMIC_UPDATE: Self = Self { ord: 67108864 };
    pub const ARRAY_FLAG_USE_8_BONE_WEIGHTS: Self = Self { ord: 134217728 };
    pub const ARRAY_FLAG_USES_EMPTY_VERTEX_ARRAY: Self = Self { ord: 268435456 };
}
impl crate::obj::EngineEnum for ArrayFormat {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 7i32
            | ord @ 8i32
            | ord @ 13i32
            | ord @ 16i32
            | ord @ 19i32
            | ord @ 22i32
            | ord @ 25i32
            | ord @ 32i32
            | ord @ 64i32
            | ord @ 128i32
            | ord @ 256i32
            | ord @ 512i32
            | ord @ 1024i32
            | ord @ 2048i32
            | ord @ 4096i32
            | ord @ 33554432i32
            | ord @ 67108864i32
            | ord @ 134217728i32
            | ord @ 268435456i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ArrayFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for ArrayFormat {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PrimitiveType {
    ord: i32,
}
impl PrimitiveType {
    pub const PRIMITIVE_POINTS: Self = Self { ord: 0 };
    pub const PRIMITIVE_LINES: Self = Self { ord: 1 };
    pub const PRIMITIVE_LINE_STRIP: Self = Self { ord: 2 };
    pub const PRIMITIVE_TRIANGLES: Self = Self { ord: 3 };
    pub const PRIMITIVE_TRIANGLE_STRIP: Self = Self { ord: 4 };
    pub const PRIMITIVE_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for PrimitiveType {
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
unsafe impl sys::GodotFfi for PrimitiveType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlendShapeMode {
    ord: i32,
}
impl BlendShapeMode {
    pub const BLEND_SHAPE_MODE_NORMALIZED: Self = Self { ord: 0 };
    pub const BLEND_SHAPE_MODE_RELATIVE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for BlendShapeMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for BlendShapeMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MultimeshTransformFormat {
    ord: i32,
}
impl MultimeshTransformFormat {
    pub const MULTIMESH_TRANSFORM_2D: Self = Self { ord: 0 };
    pub const MULTIMESH_TRANSFORM_3D: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for MultimeshTransformFormat {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for MultimeshTransformFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightProjectorFilter {
    ord: i32,
}
impl LightProjectorFilter {
    pub const LIGHT_PROJECTOR_FILTER_NEAREST: Self = Self { ord: 0 };
    pub const LIGHT_PROJECTOR_FILTER_LINEAR: Self = Self { ord: 1 };
    pub const LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS: Self = Self { ord: 2 };
    pub const LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS: Self = Self { ord: 3 };
    pub const LIGHT_PROJECTOR_FILTER_NEAREST_MIPMAPS_ANISOTROPIC: Self = Self { ord: 4 };
    pub const LIGHT_PROJECTOR_FILTER_LINEAR_MIPMAPS_ANISOTROPIC: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for LightProjectorFilter {
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
unsafe impl sys::GodotFfi for LightProjectorFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightType {
    ord: i32,
}
impl LightType {
    pub const LIGHT_DIRECTIONAL: Self = Self { ord: 0 };
    pub const LIGHT_OMNI: Self = Self { ord: 1 };
    pub const LIGHT_SPOT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LightType {
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
unsafe impl sys::GodotFfi for LightType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightParam {
    ord: i32,
}
impl LightParam {
    pub const LIGHT_PARAM_ENERGY: Self = Self { ord: 0 };
    pub const LIGHT_PARAM_INDIRECT_ENERGY: Self = Self { ord: 1 };
    pub const LIGHT_PARAM_VOLUMETRIC_FOG_ENERGY: Self = Self { ord: 2 };
    pub const LIGHT_PARAM_SPECULAR: Self = Self { ord: 3 };
    pub const LIGHT_PARAM_RANGE: Self = Self { ord: 4 };
    pub const LIGHT_PARAM_SIZE: Self = Self { ord: 5 };
    pub const LIGHT_PARAM_ATTENUATION: Self = Self { ord: 6 };
    pub const LIGHT_PARAM_SPOT_ANGLE: Self = Self { ord: 7 };
    pub const LIGHT_PARAM_SPOT_ATTENUATION: Self = Self { ord: 8 };
    pub const LIGHT_PARAM_SHADOW_MAX_DISTANCE: Self = Self { ord: 9 };
    pub const LIGHT_PARAM_SHADOW_SPLIT_1_OFFSET: Self = Self { ord: 10 };
    pub const LIGHT_PARAM_SHADOW_SPLIT_2_OFFSET: Self = Self { ord: 11 };
    pub const LIGHT_PARAM_SHADOW_SPLIT_3_OFFSET: Self = Self { ord: 12 };
    pub const LIGHT_PARAM_SHADOW_FADE_START: Self = Self { ord: 13 };
    pub const LIGHT_PARAM_SHADOW_NORMAL_BIAS: Self = Self { ord: 14 };
    pub const LIGHT_PARAM_SHADOW_BIAS: Self = Self { ord: 15 };
    pub const LIGHT_PARAM_SHADOW_PANCAKE_SIZE: Self = Self { ord: 16 };
    pub const LIGHT_PARAM_SHADOW_OPACITY: Self = Self { ord: 17 };
    pub const LIGHT_PARAM_SHADOW_BLUR: Self = Self { ord: 18 };
    pub const LIGHT_PARAM_TRANSMITTANCE_BIAS: Self = Self { ord: 19 };
    pub const LIGHT_PARAM_MAX: Self = Self { ord: 21 };
}
impl crate::obj::EngineEnum for LightParam {
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
            | ord @ 21i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for LightParam {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightBakeMode {
    ord: i32,
}
impl LightBakeMode {
    pub const LIGHT_BAKE_DISABLED: Self = Self { ord: 0 };
    pub const LIGHT_BAKE_STATIC: Self = Self { ord: 1 };
    pub const LIGHT_BAKE_DYNAMIC: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LightBakeMode {
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
unsafe impl sys::GodotFfi for LightBakeMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightOmniShadowMode {
    ord: i32,
}
impl LightOmniShadowMode {
    pub const LIGHT_OMNI_SHADOW_DUAL_PARABOLOID: Self = Self { ord: 0 };
    pub const LIGHT_OMNI_SHADOW_CUBE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for LightOmniShadowMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for LightOmniShadowMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightDirectionalShadowMode {
    ord: i32,
}
impl LightDirectionalShadowMode {
    pub const LIGHT_DIRECTIONAL_SHADOW_ORTHOGONAL: Self = Self { ord: 0 };
    pub const LIGHT_DIRECTIONAL_SHADOW_PARALLEL_2_SPLITS: Self = Self { ord: 1 };
    pub const LIGHT_DIRECTIONAL_SHADOW_PARALLEL_4_SPLITS: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LightDirectionalShadowMode {
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
unsafe impl sys::GodotFfi for LightDirectionalShadowMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LightDirectionalSkyMode {
    ord: i32,
}
impl LightDirectionalSkyMode {
    pub const LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_AND_SKY: Self = Self { ord: 0 };
    pub const LIGHT_DIRECTIONAL_SKY_MODE_LIGHT_ONLY: Self = Self { ord: 1 };
    pub const LIGHT_DIRECTIONAL_SKY_MODE_SKY_ONLY: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for LightDirectionalSkyMode {
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
unsafe impl sys::GodotFfi for LightDirectionalSkyMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShadowQuality {
    ord: i32,
}
impl ShadowQuality {
    pub const SHADOW_QUALITY_HARD: Self = Self { ord: 0 };
    pub const SHADOW_QUALITY_SOFT_VERY_LOW: Self = Self { ord: 1 };
    pub const SHADOW_QUALITY_SOFT_LOW: Self = Self { ord: 2 };
    pub const SHADOW_QUALITY_SOFT_MEDIUM: Self = Self { ord: 3 };
    pub const SHADOW_QUALITY_SOFT_HIGH: Self = Self { ord: 4 };
    pub const SHADOW_QUALITY_SOFT_ULTRA: Self = Self { ord: 5 };
    pub const SHADOW_QUALITY_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for ShadowQuality {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ShadowQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ReflectionProbeUpdateMode {
    ord: i32,
}
impl ReflectionProbeUpdateMode {
    pub const REFLECTION_PROBE_UPDATE_ONCE: Self = Self { ord: 0 };
    pub const REFLECTION_PROBE_UPDATE_ALWAYS: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for ReflectionProbeUpdateMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ReflectionProbeUpdateMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ReflectionProbeAmbientMode {
    ord: i32,
}
impl ReflectionProbeAmbientMode {
    pub const REFLECTION_PROBE_AMBIENT_DISABLED: Self = Self { ord: 0 };
    pub const REFLECTION_PROBE_AMBIENT_ENVIRONMENT: Self = Self { ord: 1 };
    pub const REFLECTION_PROBE_AMBIENT_COLOR: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ReflectionProbeAmbientMode {
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
unsafe impl sys::GodotFfi for ReflectionProbeAmbientMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DecalTexture {
    ord: i32,
}
impl DecalTexture {
    pub const DECAL_TEXTURE_ALBEDO: Self = Self { ord: 0 };
    pub const DECAL_TEXTURE_NORMAL: Self = Self { ord: 1 };
    pub const DECAL_TEXTURE_ORM: Self = Self { ord: 2 };
    pub const DECAL_TEXTURE_EMISSION: Self = Self { ord: 3 };
    pub const DECAL_TEXTURE_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for DecalTexture {
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
unsafe impl sys::GodotFfi for DecalTexture {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DecalFilter {
    ord: i32,
}
impl DecalFilter {
    pub const DECAL_FILTER_NEAREST: Self = Self { ord: 0 };
    pub const DECAL_FILTER_LINEAR: Self = Self { ord: 1 };
    pub const DECAL_FILTER_NEAREST_MIPMAPS: Self = Self { ord: 2 };
    pub const DECAL_FILTER_LINEAR_MIPMAPS: Self = Self { ord: 3 };
    pub const DECAL_FILTER_NEAREST_MIPMAPS_ANISOTROPIC: Self = Self { ord: 4 };
    pub const DECAL_FILTER_LINEAR_MIPMAPS_ANISOTROPIC: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for DecalFilter {
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
unsafe impl sys::GodotFfi for DecalFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VoxelGIQuality {
    ord: i32,
}
impl VoxelGIQuality {
    pub const VOXEL_GI_QUALITY_LOW: Self = Self { ord: 0 };
    pub const VOXEL_GI_QUALITY_HIGH: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for VoxelGIQuality {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for VoxelGIQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ParticlesMode {
    ord: i32,
}
impl ParticlesMode {
    pub const PARTICLES_MODE_2D: Self = Self { ord: 0 };
    pub const PARTICLES_MODE_3D: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for ParticlesMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ParticlesMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ParticlesTransformAlign {
    ord: i32,
}
impl ParticlesTransformAlign {
    pub const PARTICLES_TRANSFORM_ALIGN_DISABLED: Self = Self { ord: 0 };
    pub const PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD: Self = Self { ord: 1 };
    pub const PARTICLES_TRANSFORM_ALIGN_Y_TO_VELOCITY: Self = Self { ord: 2 };
    pub const PARTICLES_TRANSFORM_ALIGN_Z_BILLBOARD_Y_TO_VELOCITY: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ParticlesTransformAlign {
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
unsafe impl sys::GodotFfi for ParticlesTransformAlign {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ParticlesDrawOrder {
    ord: i32,
}
impl ParticlesDrawOrder {
    pub const PARTICLES_DRAW_ORDER_INDEX: Self = Self { ord: 0 };
    pub const PARTICLES_DRAW_ORDER_LIFETIME: Self = Self { ord: 1 };
    pub const PARTICLES_DRAW_ORDER_REVERSE_LIFETIME: Self = Self { ord: 2 };
    pub const PARTICLES_DRAW_ORDER_VIEW_DEPTH: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ParticlesDrawOrder {
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
unsafe impl sys::GodotFfi for ParticlesDrawOrder {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ParticlesCollisionType {
    ord: i32,
}
impl ParticlesCollisionType {
    pub const PARTICLES_COLLISION_TYPE_SPHERE_ATTRACT: Self = Self { ord: 0 };
    pub const PARTICLES_COLLISION_TYPE_BOX_ATTRACT: Self = Self { ord: 1 };
    pub const PARTICLES_COLLISION_TYPE_VECTOR_FIELD_ATTRACT: Self = Self { ord: 2 };
    pub const PARTICLES_COLLISION_TYPE_SPHERE_COLLIDE: Self = Self { ord: 3 };
    pub const PARTICLES_COLLISION_TYPE_BOX_COLLIDE: Self = Self { ord: 4 };
    pub const PARTICLES_COLLISION_TYPE_SDF_COLLIDE: Self = Self { ord: 5 };
    pub const PARTICLES_COLLISION_TYPE_HEIGHTFIELD_COLLIDE: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for ParticlesCollisionType {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ParticlesCollisionType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ParticlesCollisionHeightfieldResolution {
    ord: i32,
}
impl ParticlesCollisionHeightfieldResolution {
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_256: Self = Self { ord: 0 };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_512: Self = Self { ord: 1 };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_1024: Self = Self { ord: 2 };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_2048: Self = Self { ord: 3 };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_4096: Self = Self { ord: 4 };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_8192: Self = Self { ord: 5 };
    pub const PARTICLES_COLLISION_HEIGHTFIELD_RESOLUTION_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for ParticlesCollisionHeightfieldResolution {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ParticlesCollisionHeightfieldResolution {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FogVolumeShape {
    ord: i32,
}
impl FogVolumeShape {
    pub const FOG_VOLUME_SHAPE_ELLIPSOID: Self = Self { ord: 0 };
    pub const FOG_VOLUME_SHAPE_CONE: Self = Self { ord: 1 };
    pub const FOG_VOLUME_SHAPE_CYLINDER: Self = Self { ord: 2 };
    pub const FOG_VOLUME_SHAPE_BOX: Self = Self { ord: 3 };
    pub const FOG_VOLUME_SHAPE_WORLD: Self = Self { ord: 4 };
    pub const FOG_VOLUME_SHAPE_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for FogVolumeShape {
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
unsafe impl sys::GodotFfi for FogVolumeShape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportScaling3DMode {
    ord: i32,
}
impl ViewportScaling3DMode {
    pub const VIEWPORT_SCALING_3D_MODE_BILINEAR: Self = Self { ord: 0 };
    pub const VIEWPORT_SCALING_3D_MODE_FSR: Self = Self { ord: 1 };
    pub const VIEWPORT_SCALING_3D_MODE_MAX: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ViewportScaling3DMode {
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
unsafe impl sys::GodotFfi for ViewportScaling3DMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportUpdateMode {
    ord: i32,
}
impl ViewportUpdateMode {
    pub const VIEWPORT_UPDATE_DISABLED: Self = Self { ord: 0 };
    pub const VIEWPORT_UPDATE_ONCE: Self = Self { ord: 1 };
    pub const VIEWPORT_UPDATE_WHEN_VISIBLE: Self = Self { ord: 2 };
    pub const VIEWPORT_UPDATE_WHEN_PARENT_VISIBLE: Self = Self { ord: 3 };
    pub const VIEWPORT_UPDATE_ALWAYS: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for ViewportUpdateMode {
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
unsafe impl sys::GodotFfi for ViewportUpdateMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportClearMode {
    ord: i32,
}
impl ViewportClearMode {
    pub const VIEWPORT_CLEAR_ALWAYS: Self = Self { ord: 0 };
    pub const VIEWPORT_CLEAR_NEVER: Self = Self { ord: 1 };
    pub const VIEWPORT_CLEAR_ONLY_NEXT_FRAME: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ViewportClearMode {
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
unsafe impl sys::GodotFfi for ViewportClearMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportEnvironmentMode {
    ord: i32,
}
impl ViewportEnvironmentMode {
    pub const VIEWPORT_ENVIRONMENT_DISABLED: Self = Self { ord: 0 };
    pub const VIEWPORT_ENVIRONMENT_ENABLED: Self = Self { ord: 1 };
    pub const VIEWPORT_ENVIRONMENT_INHERIT: Self = Self { ord: 2 };
    pub const VIEWPORT_ENVIRONMENT_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ViewportEnvironmentMode {
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
unsafe impl sys::GodotFfi for ViewportEnvironmentMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportSDFOversize {
    ord: i32,
}
impl ViewportSDFOversize {
    pub const VIEWPORT_SDF_OVERSIZE_100_PERCENT: Self = Self { ord: 0 };
    pub const VIEWPORT_SDF_OVERSIZE_120_PERCENT: Self = Self { ord: 1 };
    pub const VIEWPORT_SDF_OVERSIZE_150_PERCENT: Self = Self { ord: 2 };
    pub const VIEWPORT_SDF_OVERSIZE_200_PERCENT: Self = Self { ord: 3 };
    pub const VIEWPORT_SDF_OVERSIZE_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for ViewportSDFOversize {
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
unsafe impl sys::GodotFfi for ViewportSDFOversize {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportSDFScale {
    ord: i32,
}
impl ViewportSDFScale {
    pub const VIEWPORT_SDF_SCALE_100_PERCENT: Self = Self { ord: 0 };
    pub const VIEWPORT_SDF_SCALE_50_PERCENT: Self = Self { ord: 1 };
    pub const VIEWPORT_SDF_SCALE_25_PERCENT: Self = Self { ord: 2 };
    pub const VIEWPORT_SDF_SCALE_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ViewportSDFScale {
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
unsafe impl sys::GodotFfi for ViewportSDFScale {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportMSAA {
    ord: i32,
}
impl ViewportMSAA {
    pub const VIEWPORT_MSAA_DISABLED: Self = Self { ord: 0 };
    pub const VIEWPORT_MSAA_2X: Self = Self { ord: 1 };
    pub const VIEWPORT_MSAA_4X: Self = Self { ord: 2 };
    pub const VIEWPORT_MSAA_8X: Self = Self { ord: 3 };
    pub const VIEWPORT_MSAA_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for ViewportMSAA {
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
unsafe impl sys::GodotFfi for ViewportMSAA {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportScreenSpaceAA {
    ord: i32,
}
impl ViewportScreenSpaceAA {
    pub const VIEWPORT_SCREEN_SPACE_AA_DISABLED: Self = Self { ord: 0 };
    pub const VIEWPORT_SCREEN_SPACE_AA_FXAA: Self = Self { ord: 1 };
    pub const VIEWPORT_SCREEN_SPACE_AA_MAX: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ViewportScreenSpaceAA {
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
unsafe impl sys::GodotFfi for ViewportScreenSpaceAA {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportOcclusionCullingBuildQuality {
    ord: i32,
}
impl ViewportOcclusionCullingBuildQuality {
    pub const VIEWPORT_OCCLUSION_BUILD_QUALITY_LOW: Self = Self { ord: 0 };
    pub const VIEWPORT_OCCLUSION_BUILD_QUALITY_MEDIUM: Self = Self { ord: 1 };
    pub const VIEWPORT_OCCLUSION_BUILD_QUALITY_HIGH: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ViewportOcclusionCullingBuildQuality {
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
unsafe impl sys::GodotFfi for ViewportOcclusionCullingBuildQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportRenderInfo {
    ord: i32,
}
impl ViewportRenderInfo {
    pub const VIEWPORT_RENDER_INFO_OBJECTS_IN_FRAME: Self = Self { ord: 0 };
    pub const VIEWPORT_RENDER_INFO_PRIMITIVES_IN_FRAME: Self = Self { ord: 1 };
    pub const VIEWPORT_RENDER_INFO_DRAW_CALLS_IN_FRAME: Self = Self { ord: 2 };
    pub const VIEWPORT_RENDER_INFO_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ViewportRenderInfo {
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
unsafe impl sys::GodotFfi for ViewportRenderInfo {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportRenderInfoType {
    ord: i32,
}
impl ViewportRenderInfoType {
    pub const VIEWPORT_RENDER_INFO_TYPE_VISIBLE: Self = Self { ord: 0 };
    pub const VIEWPORT_RENDER_INFO_TYPE_SHADOW: Self = Self { ord: 1 };
    pub const VIEWPORT_RENDER_INFO_TYPE_MAX: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for ViewportRenderInfoType {
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
unsafe impl sys::GodotFfi for ViewportRenderInfoType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportDebugDraw {
    ord: i32,
}
impl ViewportDebugDraw {
    pub const VIEWPORT_DEBUG_DRAW_DISABLED: Self = Self { ord: 0 };
    pub const VIEWPORT_DEBUG_DRAW_UNSHADED: Self = Self { ord: 1 };
    pub const VIEWPORT_DEBUG_DRAW_LIGHTING: Self = Self { ord: 2 };
    pub const VIEWPORT_DEBUG_DRAW_OVERDRAW: Self = Self { ord: 3 };
    pub const VIEWPORT_DEBUG_DRAW_WIREFRAME: Self = Self { ord: 4 };
    pub const VIEWPORT_DEBUG_DRAW_NORMAL_BUFFER: Self = Self { ord: 5 };
    pub const VIEWPORT_DEBUG_DRAW_VOXEL_GI_ALBEDO: Self = Self { ord: 6 };
    pub const VIEWPORT_DEBUG_DRAW_VOXEL_GI_LIGHTING: Self = Self { ord: 7 };
    pub const VIEWPORT_DEBUG_DRAW_VOXEL_GI_EMISSION: Self = Self { ord: 8 };
    pub const VIEWPORT_DEBUG_DRAW_SHADOW_ATLAS: Self = Self { ord: 9 };
    pub const VIEWPORT_DEBUG_DRAW_DIRECTIONAL_SHADOW_ATLAS: Self = Self { ord: 10 };
    pub const VIEWPORT_DEBUG_DRAW_SCENE_LUMINANCE: Self = Self { ord: 11 };
    pub const VIEWPORT_DEBUG_DRAW_SSAO: Self = Self { ord: 12 };
    pub const VIEWPORT_DEBUG_DRAW_SSIL: Self = Self { ord: 13 };
    pub const VIEWPORT_DEBUG_DRAW_PSSM_SPLITS: Self = Self { ord: 14 };
    pub const VIEWPORT_DEBUG_DRAW_DECAL_ATLAS: Self = Self { ord: 15 };
    pub const VIEWPORT_DEBUG_DRAW_SDFGI: Self = Self { ord: 16 };
    pub const VIEWPORT_DEBUG_DRAW_SDFGI_PROBES: Self = Self { ord: 17 };
    pub const VIEWPORT_DEBUG_DRAW_GI_BUFFER: Self = Self { ord: 18 };
    pub const VIEWPORT_DEBUG_DRAW_DISABLE_LOD: Self = Self { ord: 19 };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_OMNI_LIGHTS: Self = Self { ord: 20 };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_SPOT_LIGHTS: Self = Self { ord: 21 };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_DECALS: Self = Self { ord: 22 };
    pub const VIEWPORT_DEBUG_DRAW_CLUSTER_REFLECTION_PROBES: Self = Self { ord: 23 };
    pub const VIEWPORT_DEBUG_DRAW_OCCLUDERS: Self = Self { ord: 24 };
    pub const VIEWPORT_DEBUG_DRAW_MOTION_VECTORS: Self = Self { ord: 25 };
}
impl crate::obj::EngineEnum for ViewportDebugDraw {
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
            | ord @ 25i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ViewportDebugDraw {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ViewportVRSMode {
    ord: i32,
}
impl ViewportVRSMode {
    pub const VIEWPORT_VRS_DISABLED: Self = Self { ord: 0 };
    pub const VIEWPORT_VRS_TEXTURE: Self = Self { ord: 1 };
    pub const VIEWPORT_VRS_XR: Self = Self { ord: 2 };
    pub const VIEWPORT_VRS_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ViewportVRSMode {
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
unsafe impl sys::GodotFfi for ViewportVRSMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SkyMode {
    ord: i32,
}
impl SkyMode {
    pub const SKY_MODE_AUTOMATIC: Self = Self { ord: 0 };
    pub const SKY_MODE_QUALITY: Self = Self { ord: 1 };
    pub const SKY_MODE_INCREMENTAL: Self = Self { ord: 2 };
    pub const SKY_MODE_REALTIME: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for SkyMode {
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
unsafe impl sys::GodotFfi for SkyMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentBG {
    ord: i32,
}
impl EnvironmentBG {
    pub const ENV_BG_CLEAR_COLOR: Self = Self { ord: 0 };
    pub const ENV_BG_COLOR: Self = Self { ord: 1 };
    pub const ENV_BG_SKY: Self = Self { ord: 2 };
    pub const ENV_BG_CANVAS: Self = Self { ord: 3 };
    pub const ENV_BG_KEEP: Self = Self { ord: 4 };
    pub const ENV_BG_CAMERA_FEED: Self = Self { ord: 5 };
    pub const ENV_BG_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for EnvironmentBG {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for EnvironmentBG {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentAmbientSource {
    ord: i32,
}
impl EnvironmentAmbientSource {
    pub const ENV_AMBIENT_SOURCE_BG: Self = Self { ord: 0 };
    pub const ENV_AMBIENT_SOURCE_DISABLED: Self = Self { ord: 1 };
    pub const ENV_AMBIENT_SOURCE_COLOR: Self = Self { ord: 2 };
    pub const ENV_AMBIENT_SOURCE_SKY: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for EnvironmentAmbientSource {
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
unsafe impl sys::GodotFfi for EnvironmentAmbientSource {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentReflectionSource {
    ord: i32,
}
impl EnvironmentReflectionSource {
    pub const ENV_REFLECTION_SOURCE_BG: Self = Self { ord: 0 };
    pub const ENV_REFLECTION_SOURCE_DISABLED: Self = Self { ord: 1 };
    pub const ENV_REFLECTION_SOURCE_SKY: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for EnvironmentReflectionSource {
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
unsafe impl sys::GodotFfi for EnvironmentReflectionSource {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentGlowBlendMode {
    ord: i32,
}
impl EnvironmentGlowBlendMode {
    pub const ENV_GLOW_BLEND_MODE_ADDITIVE: Self = Self { ord: 0 };
    pub const ENV_GLOW_BLEND_MODE_SCREEN: Self = Self { ord: 1 };
    pub const ENV_GLOW_BLEND_MODE_SOFTLIGHT: Self = Self { ord: 2 };
    pub const ENV_GLOW_BLEND_MODE_REPLACE: Self = Self { ord: 3 };
    pub const ENV_GLOW_BLEND_MODE_MIX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for EnvironmentGlowBlendMode {
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
unsafe impl sys::GodotFfi for EnvironmentGlowBlendMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentToneMapper {
    ord: i32,
}
impl EnvironmentToneMapper {
    pub const ENV_TONE_MAPPER_LINEAR: Self = Self { ord: 0 };
    pub const ENV_TONE_MAPPER_REINHARD: Self = Self { ord: 1 };
    pub const ENV_TONE_MAPPER_FILMIC: Self = Self { ord: 2 };
    pub const ENV_TONE_MAPPER_ACES: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for EnvironmentToneMapper {
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
unsafe impl sys::GodotFfi for EnvironmentToneMapper {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSSRRoughnessQuality {
    ord: i32,
}
impl EnvironmentSSRRoughnessQuality {
    pub const ENV_SSR_ROUGHNESS_QUALITY_DISABLED: Self = Self { ord: 0 };
    pub const ENV_SSR_ROUGHNESS_QUALITY_LOW: Self = Self { ord: 1 };
    pub const ENV_SSR_ROUGHNESS_QUALITY_MEDIUM: Self = Self { ord: 2 };
    pub const ENV_SSR_ROUGHNESS_QUALITY_HIGH: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for EnvironmentSSRRoughnessQuality {
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
unsafe impl sys::GodotFfi for EnvironmentSSRRoughnessQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSSAOQuality {
    ord: i32,
}
impl EnvironmentSSAOQuality {
    pub const ENV_SSAO_QUALITY_VERY_LOW: Self = Self { ord: 0 };
    pub const ENV_SSAO_QUALITY_LOW: Self = Self { ord: 1 };
    pub const ENV_SSAO_QUALITY_MEDIUM: Self = Self { ord: 2 };
    pub const ENV_SSAO_QUALITY_HIGH: Self = Self { ord: 3 };
    pub const ENV_SSAO_QUALITY_ULTRA: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for EnvironmentSSAOQuality {
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
unsafe impl sys::GodotFfi for EnvironmentSSAOQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSSILQuality {
    ord: i32,
}
impl EnvironmentSSILQuality {
    pub const ENV_SSIL_QUALITY_VERY_LOW: Self = Self { ord: 0 };
    pub const ENV_SSIL_QUALITY_LOW: Self = Self { ord: 1 };
    pub const ENV_SSIL_QUALITY_MEDIUM: Self = Self { ord: 2 };
    pub const ENV_SSIL_QUALITY_HIGH: Self = Self { ord: 3 };
    pub const ENV_SSIL_QUALITY_ULTRA: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for EnvironmentSSILQuality {
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
unsafe impl sys::GodotFfi for EnvironmentSSILQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSDFGIYScale {
    ord: i32,
}
impl EnvironmentSDFGIYScale {
    pub const ENV_SDFGI_Y_SCALE_50_PERCENT: Self = Self { ord: 0 };
    pub const ENV_SDFGI_Y_SCALE_75_PERCENT: Self = Self { ord: 1 };
    pub const ENV_SDFGI_Y_SCALE_100_PERCENT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for EnvironmentSDFGIYScale {
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
unsafe impl sys::GodotFfi for EnvironmentSDFGIYScale {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSDFGIRayCount {
    ord: i32,
}
impl EnvironmentSDFGIRayCount {
    pub const ENV_SDFGI_RAY_COUNT_4: Self = Self { ord: 0 };
    pub const ENV_SDFGI_RAY_COUNT_8: Self = Self { ord: 1 };
    pub const ENV_SDFGI_RAY_COUNT_16: Self = Self { ord: 2 };
    pub const ENV_SDFGI_RAY_COUNT_32: Self = Self { ord: 3 };
    pub const ENV_SDFGI_RAY_COUNT_64: Self = Self { ord: 4 };
    pub const ENV_SDFGI_RAY_COUNT_96: Self = Self { ord: 5 };
    pub const ENV_SDFGI_RAY_COUNT_128: Self = Self { ord: 6 };
    pub const ENV_SDFGI_RAY_COUNT_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for EnvironmentSDFGIRayCount {
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
unsafe impl sys::GodotFfi for EnvironmentSDFGIRayCount {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSDFGIFramesToConverge {
    ord: i32,
}
impl EnvironmentSDFGIFramesToConverge {
    pub const ENV_SDFGI_CONVERGE_IN_5_FRAMES: Self = Self { ord: 0 };
    pub const ENV_SDFGI_CONVERGE_IN_10_FRAMES: Self = Self { ord: 1 };
    pub const ENV_SDFGI_CONVERGE_IN_15_FRAMES: Self = Self { ord: 2 };
    pub const ENV_SDFGI_CONVERGE_IN_20_FRAMES: Self = Self { ord: 3 };
    pub const ENV_SDFGI_CONVERGE_IN_25_FRAMES: Self = Self { ord: 4 };
    pub const ENV_SDFGI_CONVERGE_IN_30_FRAMES: Self = Self { ord: 5 };
    pub const ENV_SDFGI_CONVERGE_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for EnvironmentSDFGIFramesToConverge {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 6i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for EnvironmentSDFGIFramesToConverge {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct EnvironmentSDFGIFramesToUpdateLight {
    ord: i32,
}
impl EnvironmentSDFGIFramesToUpdateLight {
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_1_FRAME: Self = Self { ord: 0 };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_2_FRAMES: Self = Self { ord: 1 };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_4_FRAMES: Self = Self { ord: 2 };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_8_FRAMES: Self = Self { ord: 3 };
    pub const ENV_SDFGI_UPDATE_LIGHT_IN_16_FRAMES: Self = Self { ord: 4 };
    pub const ENV_SDFGI_UPDATE_LIGHT_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for EnvironmentSDFGIFramesToUpdateLight {
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
unsafe impl sys::GodotFfi for EnvironmentSDFGIFramesToUpdateLight {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SubSurfaceScatteringQuality {
    ord: i32,
}
impl SubSurfaceScatteringQuality {
    pub const SUB_SURFACE_SCATTERING_QUALITY_DISABLED: Self = Self { ord: 0 };
    pub const SUB_SURFACE_SCATTERING_QUALITY_LOW: Self = Self { ord: 1 };
    pub const SUB_SURFACE_SCATTERING_QUALITY_MEDIUM: Self = Self { ord: 2 };
    pub const SUB_SURFACE_SCATTERING_QUALITY_HIGH: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for SubSurfaceScatteringQuality {
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
unsafe impl sys::GodotFfi for SubSurfaceScatteringQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DOFBokehShape {
    ord: i32,
}
impl DOFBokehShape {
    pub const DOF_BOKEH_BOX: Self = Self { ord: 0 };
    pub const DOF_BOKEH_HEXAGON: Self = Self { ord: 1 };
    pub const DOF_BOKEH_CIRCLE: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for DOFBokehShape {
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
unsafe impl sys::GodotFfi for DOFBokehShape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DOFBlurQuality {
    ord: i32,
}
impl DOFBlurQuality {
    pub const DOF_BLUR_QUALITY_VERY_LOW: Self = Self { ord: 0 };
    pub const DOF_BLUR_QUALITY_LOW: Self = Self { ord: 1 };
    pub const DOF_BLUR_QUALITY_MEDIUM: Self = Self { ord: 2 };
    pub const DOF_BLUR_QUALITY_HIGH: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for DOFBlurQuality {
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
unsafe impl sys::GodotFfi for DOFBlurQuality {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct InstanceType {
    ord: i32,
}
impl InstanceType {
    pub const INSTANCE_NONE: Self = Self { ord: 0 };
    pub const INSTANCE_MESH: Self = Self { ord: 1 };
    pub const INSTANCE_MULTIMESH: Self = Self { ord: 2 };
    pub const INSTANCE_PARTICLES: Self = Self { ord: 3 };
    pub const INSTANCE_PARTICLES_COLLISION: Self = Self { ord: 4 };
    pub const INSTANCE_LIGHT: Self = Self { ord: 5 };
    pub const INSTANCE_REFLECTION_PROBE: Self = Self { ord: 6 };
    pub const INSTANCE_DECAL: Self = Self { ord: 7 };
    pub const INSTANCE_VOXEL_GI: Self = Self { ord: 8 };
    pub const INSTANCE_LIGHTMAP: Self = Self { ord: 9 };
    pub const INSTANCE_OCCLUDER: Self = Self { ord: 10 };
    pub const INSTANCE_VISIBLITY_NOTIFIER: Self = Self { ord: 11 };
    pub const INSTANCE_FOG_VOLUME: Self = Self { ord: 12 };
    pub const INSTANCE_MAX: Self = Self { ord: 13 };
    pub const INSTANCE_GEOMETRY_MASK: Self = Self { ord: 14 };
}
impl crate::obj::EngineEnum for InstanceType {
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
            | ord @ 14i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for InstanceType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct InstanceFlags {
    ord: i32,
}
impl InstanceFlags {
    pub const INSTANCE_FLAG_USE_BAKED_LIGHT: Self = Self { ord: 0 };
    pub const INSTANCE_FLAG_USE_DYNAMIC_GI: Self = Self { ord: 1 };
    pub const INSTANCE_FLAG_DRAW_NEXT_FRAME_IF_VISIBLE: Self = Self { ord: 2 };
    pub const INSTANCE_FLAG_IGNORE_OCCLUSION_CULLING: Self = Self { ord: 3 };
    pub const INSTANCE_FLAG_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for InstanceFlags {
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
unsafe impl sys::GodotFfi for InstanceFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShadowCastingSetting {
    ord: i32,
}
impl ShadowCastingSetting {
    pub const SHADOW_CASTING_SETTING_OFF: Self = Self { ord: 0 };
    pub const SHADOW_CASTING_SETTING_ON: Self = Self { ord: 1 };
    pub const SHADOW_CASTING_SETTING_DOUBLE_SIDED: Self = Self { ord: 2 };
    pub const SHADOW_CASTING_SETTING_SHADOWS_ONLY: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for ShadowCastingSetting {
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
unsafe impl sys::GodotFfi for ShadowCastingSetting {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VisibilityRangeFadeMode {
    ord: i32,
}
impl VisibilityRangeFadeMode {
    pub const VISIBILITY_RANGE_FADE_DISABLED: Self = Self { ord: 0 };
    pub const VISIBILITY_RANGE_FADE_SELF: Self = Self { ord: 1 };
    pub const VISIBILITY_RANGE_FADE_DEPENDENCIES: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for VisibilityRangeFadeMode {
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
unsafe impl sys::GodotFfi for VisibilityRangeFadeMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BakeChannels {
    ord: i32,
}
impl BakeChannels {
    pub const BAKE_CHANNEL_ALBEDO_ALPHA: Self = Self { ord: 0 };
    pub const BAKE_CHANNEL_NORMAL: Self = Self { ord: 1 };
    pub const BAKE_CHANNEL_ORM: Self = Self { ord: 2 };
    pub const BAKE_CHANNEL_EMISSION: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for BakeChannels {
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
unsafe impl sys::GodotFfi for BakeChannels {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasTextureChannel {
    ord: i32,
}
impl CanvasTextureChannel {
    pub const CANVAS_TEXTURE_CHANNEL_DIFFUSE: Self = Self { ord: 0 };
    pub const CANVAS_TEXTURE_CHANNEL_NORMAL: Self = Self { ord: 1 };
    pub const CANVAS_TEXTURE_CHANNEL_SPECULAR: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for CanvasTextureChannel {
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
unsafe impl sys::GodotFfi for CanvasTextureChannel {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NinePatchAxisMode {
    ord: i32,
}
impl NinePatchAxisMode {
    pub const NINE_PATCH_STRETCH: Self = Self { ord: 0 };
    pub const NINE_PATCH_TILE: Self = Self { ord: 1 };
    pub const NINE_PATCH_TILE_FIT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for NinePatchAxisMode {
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
unsafe impl sys::GodotFfi for NinePatchAxisMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasItemTextureFilter {
    ord: i32,
}
impl CanvasItemTextureFilter {
    pub const CANVAS_ITEM_TEXTURE_FILTER_DEFAULT: Self = Self { ord: 0 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_NEAREST: Self = Self { ord: 1 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_LINEAR: Self = Self { ord: 2 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS: Self = Self { ord: 3 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS: Self = Self { ord: 4 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_NEAREST_WITH_MIPMAPS_ANISOTROPIC: Self = Self { ord: 5 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_LINEAR_WITH_MIPMAPS_ANISOTROPIC: Self = Self { ord: 6 };
    pub const CANVAS_ITEM_TEXTURE_FILTER_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for CanvasItemTextureFilter {
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
unsafe impl sys::GodotFfi for CanvasItemTextureFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasItemTextureRepeat {
    ord: i32,
}
impl CanvasItemTextureRepeat {
    pub const CANVAS_ITEM_TEXTURE_REPEAT_DEFAULT: Self = Self { ord: 0 };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_DISABLED: Self = Self { ord: 1 };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_ENABLED: Self = Self { ord: 2 };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_MIRROR: Self = Self { ord: 3 };
    pub const CANVAS_ITEM_TEXTURE_REPEAT_MAX: Self = Self { ord: 4 };
}
impl crate::obj::EngineEnum for CanvasItemTextureRepeat {
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
unsafe impl sys::GodotFfi for CanvasItemTextureRepeat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasGroupMode {
    ord: i32,
}
impl CanvasGroupMode {
    pub const CANVAS_GROUP_MODE_DISABLED: Self = Self { ord: 0 };
    pub const CANVAS_GROUP_MODE_CLIP_ONLY: Self = Self { ord: 1 };
    pub const CANVAS_GROUP_MODE_CLIP_AND_DRAW: Self = Self { ord: 2 };
    pub const CANVAS_GROUP_MODE_TRANSPARENT: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for CanvasGroupMode {
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
unsafe impl sys::GodotFfi for CanvasGroupMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasLightMode {
    ord: i32,
}
impl CanvasLightMode {
    pub const CANVAS_LIGHT_MODE_POINT: Self = Self { ord: 0 };
    pub const CANVAS_LIGHT_MODE_DIRECTIONAL: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for CanvasLightMode {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CanvasLightMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasLightBlendMode {
    ord: i32,
}
impl CanvasLightBlendMode {
    pub const CANVAS_LIGHT_BLEND_MODE_ADD: Self = Self { ord: 0 };
    pub const CANVAS_LIGHT_BLEND_MODE_SUB: Self = Self { ord: 1 };
    pub const CANVAS_LIGHT_BLEND_MODE_MIX: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for CanvasLightBlendMode {
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
unsafe impl sys::GodotFfi for CanvasLightBlendMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasLightShadowFilter {
    ord: i32,
}
impl CanvasLightShadowFilter {
    pub const CANVAS_LIGHT_FILTER_NONE: Self = Self { ord: 0 };
    pub const CANVAS_LIGHT_FILTER_PCF5: Self = Self { ord: 1 };
    pub const CANVAS_LIGHT_FILTER_PCF13: Self = Self { ord: 2 };
    pub const CANVAS_LIGHT_FILTER_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for CanvasLightShadowFilter {
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
unsafe impl sys::GodotFfi for CanvasLightShadowFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CanvasOccluderPolygonCullMode {
    ord: i32,
}
impl CanvasOccluderPolygonCullMode {
    pub const CANVAS_OCCLUDER_POLYGON_CULL_DISABLED: Self = Self { ord: 0 };
    pub const CANVAS_OCCLUDER_POLYGON_CULL_CLOCKWISE: Self = Self { ord: 1 };
    pub const CANVAS_OCCLUDER_POLYGON_CULL_COUNTER_CLOCKWISE: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for CanvasOccluderPolygonCullMode {
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
unsafe impl sys::GodotFfi for CanvasOccluderPolygonCullMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct GlobalShaderParameterType {
    ord: i32,
}
impl GlobalShaderParameterType {
    pub const GLOBAL_VAR_TYPE_BOOL: Self = Self { ord: 0 };
    pub const GLOBAL_VAR_TYPE_BVEC2: Self = Self { ord: 1 };
    pub const GLOBAL_VAR_TYPE_BVEC3: Self = Self { ord: 2 };
    pub const GLOBAL_VAR_TYPE_BVEC4: Self = Self { ord: 3 };
    pub const GLOBAL_VAR_TYPE_INT: Self = Self { ord: 4 };
    pub const GLOBAL_VAR_TYPE_IVEC2: Self = Self { ord: 5 };
    pub const GLOBAL_VAR_TYPE_IVEC3: Self = Self { ord: 6 };
    pub const GLOBAL_VAR_TYPE_IVEC4: Self = Self { ord: 7 };
    pub const GLOBAL_VAR_TYPE_RECT2I: Self = Self { ord: 8 };
    pub const GLOBAL_VAR_TYPE_UINT: Self = Self { ord: 9 };
    pub const GLOBAL_VAR_TYPE_UVEC2: Self = Self { ord: 10 };
    pub const GLOBAL_VAR_TYPE_UVEC3: Self = Self { ord: 11 };
    pub const GLOBAL_VAR_TYPE_UVEC4: Self = Self { ord: 12 };
    pub const GLOBAL_VAR_TYPE_FLOAT: Self = Self { ord: 13 };
    pub const GLOBAL_VAR_TYPE_VEC2: Self = Self { ord: 14 };
    pub const GLOBAL_VAR_TYPE_VEC3: Self = Self { ord: 15 };
    pub const GLOBAL_VAR_TYPE_VEC4: Self = Self { ord: 16 };
    pub const GLOBAL_VAR_TYPE_COLOR: Self = Self { ord: 17 };
    pub const GLOBAL_VAR_TYPE_RECT2: Self = Self { ord: 18 };
    pub const GLOBAL_VAR_TYPE_MAT2: Self = Self { ord: 19 };
    pub const GLOBAL_VAR_TYPE_MAT3: Self = Self { ord: 20 };
    pub const GLOBAL_VAR_TYPE_MAT4: Self = Self { ord: 21 };
    pub const GLOBAL_VAR_TYPE_TRANSFORM_2D: Self = Self { ord: 22 };
    pub const GLOBAL_VAR_TYPE_TRANSFORM: Self = Self { ord: 23 };
    pub const GLOBAL_VAR_TYPE_SAMPLER2D: Self = Self { ord: 24 };
    pub const GLOBAL_VAR_TYPE_SAMPLER2DARRAY: Self = Self { ord: 25 };
    pub const GLOBAL_VAR_TYPE_SAMPLER3D: Self = Self { ord: 26 };
    pub const GLOBAL_VAR_TYPE_SAMPLERCUBE: Self = Self { ord: 27 };
    pub const GLOBAL_VAR_TYPE_MAX: Self = Self { ord: 28 };
}
impl crate::obj::EngineEnum for GlobalShaderParameterType {
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
            | ord @ 28i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for GlobalShaderParameterType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct RenderingInfo {
    ord: i32,
}
impl RenderingInfo {
    pub const RENDERING_INFO_TOTAL_OBJECTS_IN_FRAME: Self = Self { ord: 0 };
    pub const RENDERING_INFO_TOTAL_PRIMITIVES_IN_FRAME: Self = Self { ord: 1 };
    pub const RENDERING_INFO_TOTAL_DRAW_CALLS_IN_FRAME: Self = Self { ord: 2 };
    pub const RENDERING_INFO_TEXTURE_MEM_USED: Self = Self { ord: 3 };
    pub const RENDERING_INFO_BUFFER_MEM_USED: Self = Self { ord: 4 };
    pub const RENDERING_INFO_VIDEO_MEM_USED: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for RenderingInfo {
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
unsafe impl sys::GodotFfi for RenderingInfo {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Features {
    ord: i32,
}
impl Features {
    pub const FEATURE_SHADERS: Self = Self { ord: 0 };
    pub const FEATURE_MULTITHREADED: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for Features {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32 | ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Features {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
