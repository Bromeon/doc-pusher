#![doc = "Sidecar module for class [`RenderingDevice`][crate::engine::RenderingDevice].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `RenderingDevice` enums](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `RenderingDevice.`\n\nInherits [`Object`][crate::engine::Object].\n\nRelated symbols:\n\n* [`rendering_device`][crate::engine::rendering_device]: sidecar module with related enum/flag types\n* [`RenderingDeviceVirtual`][crate::engine::RenderingDeviceVirtual]: virtual methods\n\n\nSee also [Godot docs for `RenderingDevice`](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct RenderingDevice {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`RenderingDevice`][crate::engine::RenderingDevice].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `RenderingDevice` methods](https://docs.godotengine.org/en/stable/classes/class_renderingdevice.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait RenderingDeviceVirtual:
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
    impl RenderingDevice {
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
        pub fn texture_create(
            &mut self,
            format: Gd<RdTextureFormat>,
            view: Gd<RdTextureView>,
            data: PackedByteArray,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3011278298i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_create" , 3011278298i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<RdTextureFormat> as AsArg>::as_arg_ptr(&format),
                    <Gd<RdTextureView> as AsArg>::as_arg_ptr(&view),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_create_shared(&mut self, view: Gd<RdTextureView>, with_texture: Rid) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_create_shared");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3178156134i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_create_shared" , 3178156134i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<RdTextureView> as AsArg>::as_arg_ptr(&view),
                    <Rid as sys::GodotFfi>::sys_const(&with_texture),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_create_shared_from_slice(
            &mut self,
            view: Gd<RdTextureView>,
            with_texture: Rid,
            layer: i64,
            mipmap: i64,
            mipmaps: i64,
            slice_type: rendering_device::TextureSliceType,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_create_shared_from_slice");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    864132525i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_create_shared_from_slice" , 864132525i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<RdTextureView> as AsArg>::as_arg_ptr(&view),
                    <Rid as sys::GodotFfi>::sys_const(&with_texture),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                    <i64 as sys::GodotFfi>::sys_const(&mipmap),
                    <i64 as sys::GodotFfi>::sys_const(&mipmaps),
                    <rendering_device::TextureSliceType as sys::GodotFfi>::sys_const(&slice_type),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_update(
            &mut self,
            texture: Rid,
            layer: i64,
            data: PackedByteArray,
            post_barrier: rendering_device::BarrierMask,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2736912341i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_update" , 2736912341i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&post_barrier),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_get_data(&mut self, texture: Rid, layer: i64) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_get_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1859412099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_get_data" , 1859412099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_is_format_supported_for_usage(
            &self,
            format: rendering_device::DataFormat,
            usage_flags: rendering_device::TextureUsageBits,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_is_format_supported_for_usage");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2592520478i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_is_format_supported_for_usage" , 2592520478i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_device::DataFormat as sys::GodotFfi>::sys_const(&format),
                    <rendering_device::TextureUsageBits as sys::GodotFfi>::sys_const(&usage_flags),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_is_shared(&mut self, texture: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_is_shared");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_is_shared" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&texture)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_is_valid(&mut self, texture: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_is_valid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_is_valid" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&texture)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_copy(
            &mut self,
            from_texture: Rid,
            to_texture: Rid,
            from_pos: Vector3,
            to_pos: Vector3,
            size: Vector3,
            src_mipmap: i64,
            dst_mipmap: i64,
            src_layer: i64,
            dst_layer: i64,
            post_barrier: rendering_device::BarrierMask,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_copy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3741367532i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_copy" , 3741367532i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&from_texture),
                    <Rid as sys::GodotFfi>::sys_const(&to_texture),
                    <Vector3 as sys::GodotFfi>::sys_const(&from_pos),
                    <Vector3 as sys::GodotFfi>::sys_const(&to_pos),
                    <Vector3 as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&src_mipmap),
                    <i64 as sys::GodotFfi>::sys_const(&dst_mipmap),
                    <i64 as sys::GodotFfi>::sys_const(&src_layer),
                    <i64 as sys::GodotFfi>::sys_const(&dst_layer),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&post_barrier),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_clear(
            &mut self,
            texture: Rid,
            color: Color,
            base_mipmap: i64,
            mipmap_count: i64,
            base_layer: i64,
            layer_count: i64,
            post_barrier: rendering_device::BarrierMask,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3423681478i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_clear" , 3423681478i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&texture),
                    <Color as sys::GodotFfi>::sys_const(&color),
                    <i64 as sys::GodotFfi>::sys_const(&base_mipmap),
                    <i64 as sys::GodotFfi>::sys_const(&mipmap_count),
                    <i64 as sys::GodotFfi>::sys_const(&base_layer),
                    <i64 as sys::GodotFfi>::sys_const(&layer_count),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&post_barrier),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_resolve_multisample(
            &mut self,
            from_texture: Rid,
            to_texture: Rid,
            post_barrier: rendering_device::BarrierMask,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_resolve_multisample");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2126834943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_resolve_multisample" , 2126834943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&from_texture),
                    <Rid as sys::GodotFfi>::sys_const(&to_texture),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&post_barrier),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_format_create(
            &mut self,
            attachments: Array<Gd<RdAttachmentFormat>>,
            view_count: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_format_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2635475316i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_format_create" , 2635475316i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Gd<RdAttachmentFormat>> as sys::GodotFfi>::sys_const(&attachments),
                    <i64 as sys::GodotFfi>::sys_const(&view_count),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_format_create_multipass(
            &mut self,
            attachments: Array<Gd<RdAttachmentFormat>>,
            passes: Array<Gd<RdFramebufferPass>>,
            view_count: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_format_create_multipass");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1992489524i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_format_create_multipass" , 1992489524i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Gd<RdAttachmentFormat>> as sys::GodotFfi>::sys_const(&attachments),
                    <Array<Gd<RdFramebufferPass>> as sys::GodotFfi>::sys_const(&passes),
                    <i64 as sys::GodotFfi>::sys_const(&view_count),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_format_create_empty(
            &mut self,
            samples: rendering_device::TextureSamples,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_format_create_empty");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    555930169i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_format_create_empty" , 555930169i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args =
                    [<rendering_device::TextureSamples as sys::GodotFfi>::sys_const(&samples)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_format_get_texture_samples(
            &mut self,
            format: i64,
            render_pass: i64,
        ) -> rendering_device::TextureSamples {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_format_get_texture_samples");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1036806638i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_format_get_texture_samples" , 1036806638i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&format),
                    <i64 as sys::GodotFfi>::sys_const(&render_pass),
                ];
                let __args_ptr = __args.as_ptr();
                <rendering_device::TextureSamples as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn framebuffer_create(
            &mut self,
            textures: Array<Rid>,
            validate_with_format: i64,
            view_count: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1884747791i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_create" , 1884747791i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Rid> as sys::GodotFfi>::sys_const(&textures),
                    <i64 as sys::GodotFfi>::sys_const(&validate_with_format),
                    <i64 as sys::GodotFfi>::sys_const(&view_count),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_create_multipass(
            &mut self,
            textures: Array<Rid>,
            passes: Array<Gd<RdFramebufferPass>>,
            validate_with_format: i64,
            view_count: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_create_multipass");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    452534725i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_create_multipass" , 452534725i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Rid> as sys::GodotFfi>::sys_const(&textures),
                    <Array<Gd<RdFramebufferPass>> as sys::GodotFfi>::sys_const(&passes),
                    <i64 as sys::GodotFfi>::sys_const(&validate_with_format),
                    <i64 as sys::GodotFfi>::sys_const(&view_count),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_create_empty(
            &mut self,
            size: Vector2i,
            samples: rendering_device::TextureSamples,
            validate_with_format: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_create_empty");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    382373098i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_create_empty" , 382373098i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <rendering_device::TextureSamples as sys::GodotFfi>::sys_const(&samples),
                    <i64 as sys::GodotFfi>::sys_const(&validate_with_format),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_get_format(&mut self, framebuffer: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_get_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3917799429i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_get_format" , 3917799429i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&framebuffer)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn framebuffer_is_valid(&self, framebuffer: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("framebuffer_is_valid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "framebuffer_is_valid" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&framebuffer)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn sampler_create(&mut self, state: Gd<RdSamplerState>) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("sampler_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2327892535i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "sampler_create" , 2327892535i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<RdSamplerState> as AsArg>::as_arg_ptr(&state)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn vertex_buffer_create(
            &mut self,
            size_bytes: i64,
            data: PackedByteArray,
            use_as_storage: bool,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("vertex_buffer_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3491282828i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "vertex_buffer_create" , 3491282828i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                    <bool as sys::GodotFfi>::sys_const(&use_as_storage),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn vertex_format_create(
            &mut self,
            vertex_descriptions: Array<Gd<RdVertexAttribute>>,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("vertex_format_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1242678479i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "vertex_format_create" , 1242678479i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<RdVertexAttribute>> as sys::GodotFfi>::sys_const(
                    &vertex_descriptions,
                )];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn vertex_array_create(
            &mut self,
            vertex_count: i64,
            vertex_format: i64,
            src_buffers: Array<Rid>,
            offsets: PackedInt64Array,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("vertex_array_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3137892244i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "vertex_array_create" , 3137892244i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&vertex_count),
                    <i64 as sys::GodotFfi>::sys_const(&vertex_format),
                    <Array<Rid> as sys::GodotFfi>::sys_const(&src_buffers),
                    <PackedInt64Array as sys::GodotFfi>::sys_const(&offsets),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn index_buffer_create(
            &mut self,
            size_indices: i64,
            format: rendering_device::IndexBufferFormat,
            data: PackedByteArray,
            use_restart_indices: bool,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("index_buffer_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    975915977i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "index_buffer_create" , 975915977i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size_indices),
                    <rendering_device::IndexBufferFormat as sys::GodotFfi>::sys_const(&format),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                    <bool as sys::GodotFfi>::sys_const(&use_restart_indices),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn index_array_create(
            &mut self,
            index_buffer: Rid,
            index_offset: i64,
            index_count: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("index_array_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2256026069i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "index_array_create" , 2256026069i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&index_buffer),
                    <i64 as sys::GodotFfi>::sys_const(&index_offset),
                    <i64 as sys::GodotFfi>::sys_const(&index_count),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_compile_spirv_from_source(
            &mut self,
            shader_source: Gd<RdShaderSource>,
            allow_cache: bool,
        ) -> Option<Gd<RdShaderSpirv>> {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("shader_compile_spirv_from_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3459523685i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "shader_compile_spirv_from_source" , 3459523685i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<RdShaderSource> as AsArg>::as_arg_ptr(&shader_source),
                    <bool as sys::GodotFfi>::sys_const(&allow_cache),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<RdShaderSpirv>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_compile_binary_from_spirv(
            &mut self,
            spirv_data: Gd<RdShaderSpirv>,
            name: GodotString,
        ) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("shader_compile_binary_from_spirv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1395027180i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "shader_compile_binary_from_spirv" , 1395027180i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<RdShaderSpirv> as AsArg>::as_arg_ptr(&spirv_data),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_create_from_spirv(
            &mut self,
            spirv_data: Gd<RdShaderSpirv>,
            name: GodotString,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("shader_create_from_spirv");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3297482566i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "shader_create_from_spirv" , 3297482566i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<RdShaderSpirv> as AsArg>::as_arg_ptr(&spirv_data),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_create_from_bytecode(&mut self, binary_data: PackedByteArray) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("shader_create_from_bytecode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3049171473i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "shader_create_from_bytecode" , 3049171473i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&binary_data)];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn shader_get_vertex_input_attribute_mask(&mut self, shader: Rid) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("shader_get_vertex_input_attribute_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3917799429i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "shader_get_vertex_input_attribute_mask" , 3917799429i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&shader)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn uniform_buffer_create(&mut self, size_bytes: i64, data: PackedByteArray) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("uniform_buffer_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1453158401i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "uniform_buffer_create" , 1453158401i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn storage_buffer_create(
            &mut self,
            size_bytes: i64,
            data: PackedByteArray,
            usage: rendering_device::StorageBufferUsage,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("storage_buffer_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1173156076i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "storage_buffer_create" , 1173156076i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                    <rendering_device::StorageBufferUsage as sys::GodotFfi>::sys_const(&usage),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn texture_buffer_create(
            &mut self,
            size_bytes: i64,
            format: rendering_device::DataFormat,
            data: PackedByteArray,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("texture_buffer_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2344087557i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "texture_buffer_create" , 2344087557i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                    <rendering_device::DataFormat as sys::GodotFfi>::sys_const(&format),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn uniform_set_create(
            &mut self,
            uniforms: Array<Gd<RdUniform>>,
            shader: Rid,
            shader_set: i64,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("uniform_set_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2280795797i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "uniform_set_create" , 2280795797i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Array<Gd<RdUniform>> as sys::GodotFfi>::sys_const(&uniforms),
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <i64 as sys::GodotFfi>::sys_const(&shader_set),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn uniform_set_is_valid(&mut self, uniform_set: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("uniform_set_is_valid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "uniform_set_is_valid" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&uniform_set)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn buffer_update(
            &mut self,
            buffer: Rid,
            offset: i64,
            size_bytes: i64,
            data: PackedByteArray,
            post_barrier: rendering_device::BarrierMask,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("buffer_update");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    652628289i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "buffer_update" , 652628289i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&buffer),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&data),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&post_barrier),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn buffer_clear(
            &mut self,
            buffer: Rid,
            offset: i64,
            size_bytes: i64,
            post_barrier: rendering_device::BarrierMask,
        ) -> global::Error {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("buffer_clear");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1645170096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "buffer_clear" , 1645170096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&buffer),
                    <i64 as sys::GodotFfi>::sys_const(&offset),
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&post_barrier),
                ];
                let __args_ptr = __args.as_ptr();
                <global::Error as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn buffer_get_data(
            &mut self,
            buffer: Rid,
            offset_bytes: i64,
            size_bytes: i64,
        ) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("buffer_get_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    125363422i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "buffer_get_data" , 125363422i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&buffer),
                    <i64 as sys::GodotFfi>::sys_const(&offset_bytes),
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn render_pipeline_create(
            &mut self,
            shader: Rid,
            framebuffer_format: i64,
            vertex_format: i64,
            primitive: rendering_device::RenderPrimitive,
            rasterization_state: Gd<RdPipelineRasterizationState>,
            multisample_state: Gd<RdPipelineMultisampleState>,
            stencil_state: Gd<RdPipelineDepthStencilState>,
            color_blend_state: Gd<RdPipelineColorBlendState>,
            dynamic_state_flags: rendering_device::PipelineDynamicStateFlags,
            for_render_pass: i64,
            specialization_constants: Array<Gd<RdPipelineSpecializationConstant>>,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("render_pipeline_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2911419500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "render_pipeline_create" , 2911419500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <i64 as sys::GodotFfi>::sys_const(&framebuffer_format),
                    <i64 as sys::GodotFfi>::sys_const(&vertex_format),
                    <rendering_device::RenderPrimitive as sys::GodotFfi>::sys_const(&primitive),
                    <Gd<RdPipelineRasterizationState> as AsArg>::as_arg_ptr(&rasterization_state),
                    <Gd<RdPipelineMultisampleState> as AsArg>::as_arg_ptr(&multisample_state),
                    <Gd<RdPipelineDepthStencilState> as AsArg>::as_arg_ptr(&stencil_state),
                    <Gd<RdPipelineColorBlendState> as AsArg>::as_arg_ptr(&color_blend_state),
                    <rendering_device::PipelineDynamicStateFlags as sys::GodotFfi>::sys_const(
                        &dynamic_state_flags,
                    ),
                    <i64 as sys::GodotFfi>::sys_const(&for_render_pass),
                    <Array<Gd<RdPipelineSpecializationConstant>> as sys::GodotFfi>::sys_const(
                        &specialization_constants,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn render_pipeline_is_valid(&mut self, render_pipeline: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("render_pipeline_is_valid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "render_pipeline_is_valid" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&render_pipeline)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn compute_pipeline_create(
            &mut self,
            shader: Rid,
            specialization_constants: Array<Gd<RdPipelineSpecializationConstant>>,
        ) -> Rid {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_pipeline_create");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    403593840i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_pipeline_create" , 403593840i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&shader),
                    <Array<Gd<RdPipelineSpecializationConstant>> as sys::GodotFfi>::sys_const(
                        &specialization_constants,
                    ),
                ];
                let __args_ptr = __args.as_ptr();
                <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn compute_pipeline_is_valid(&mut self, compute_pieline: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_pipeline_is_valid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3521089500i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_pipeline_is_valid" , 3521089500i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&compute_pieline)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_width(&self, screen: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("screen_get_width");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1591665591i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "screen_get_width" , 1591665591i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_height(&self, screen: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("screen_get_height");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1591665591i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "screen_get_height" , 1591665591i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&screen)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn screen_get_framebuffer_format(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("screen_get_framebuffer_format");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "screen_get_framebuffer_format" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_list_begin_for_screen(&mut self, screen: i64, clear_color: Color) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_begin_for_screen");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3988079995i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_begin_for_screen" , 3988079995i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&screen),
                    <Color as sys::GodotFfi>::sys_const(&clear_color),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_list_begin(
            &mut self,
            framebuffer: Rid,
            initial_color_action: rendering_device::InitialAction,
            final_color_action: rendering_device::FinalAction,
            initial_depth_action: rendering_device::InitialAction,
            final_depth_action: rendering_device::FinalAction,
            clear_color_values: PackedColorArray,
            clear_depth: f64,
            clear_stencil: i64,
            region: Rect2,
            storage_textures: VariantArray,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_begin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4252992020i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_begin" , 4252992020i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&framebuffer),
                    <rendering_device::InitialAction as sys::GodotFfi>::sys_const(
                        &initial_color_action,
                    ),
                    <rendering_device::FinalAction as sys::GodotFfi>::sys_const(
                        &final_color_action,
                    ),
                    <rendering_device::InitialAction as sys::GodotFfi>::sys_const(
                        &initial_depth_action,
                    ),
                    <rendering_device::FinalAction as sys::GodotFfi>::sys_const(
                        &final_depth_action,
                    ),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&clear_color_values),
                    <f64 as sys::GodotFfi>::sys_const(&clear_depth),
                    <i64 as sys::GodotFfi>::sys_const(&clear_stencil),
                    <Rect2 as sys::GodotFfi>::sys_const(&region),
                    <VariantArray as sys::GodotFfi>::sys_const(&storage_textures),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_list_begin_split(
            &mut self,
            framebuffer: Rid,
            splits: i64,
            initial_color_action: rendering_device::InitialAction,
            final_color_action: rendering_device::FinalAction,
            initial_depth_action: rendering_device::InitialAction,
            final_depth_action: rendering_device::FinalAction,
            clear_color_values: PackedColorArray,
            clear_depth: f64,
            clear_stencil: i64,
            region: Rect2,
            storage_textures: Array<Rid>,
        ) -> PackedInt64Array {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_begin_split");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    832527510i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_begin_split" , 832527510i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&framebuffer),
                    <i64 as sys::GodotFfi>::sys_const(&splits),
                    <rendering_device::InitialAction as sys::GodotFfi>::sys_const(
                        &initial_color_action,
                    ),
                    <rendering_device::FinalAction as sys::GodotFfi>::sys_const(
                        &final_color_action,
                    ),
                    <rendering_device::InitialAction as sys::GodotFfi>::sys_const(
                        &initial_depth_action,
                    ),
                    <rendering_device::FinalAction as sys::GodotFfi>::sys_const(
                        &final_depth_action,
                    ),
                    <PackedColorArray as sys::GodotFfi>::sys_const(&clear_color_values),
                    <f64 as sys::GodotFfi>::sys_const(&clear_depth),
                    <i64 as sys::GodotFfi>::sys_const(&clear_stencil),
                    <Rect2 as sys::GodotFfi>::sys_const(&region),
                    <Array<Rid> as sys::GodotFfi>::sys_const(&storage_textures),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_list_set_blend_constants(&mut self, draw_list: i64, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_set_blend_constants");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2878471219i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_set_blend_constants" , 2878471219i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_bind_render_pipeline(&mut self, draw_list: i64, render_pipeline: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_bind_render_pipeline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4040184819i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_bind_render_pipeline" , 4040184819i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <Rid as sys::GodotFfi>::sys_const(&render_pipeline),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_bind_uniform_set(
            &mut self,
            draw_list: i64,
            uniform_set: Rid,
            set_index: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_bind_uniform_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    749655778i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_bind_uniform_set" , 749655778i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <Rid as sys::GodotFfi>::sys_const(&uniform_set),
                    <i64 as sys::GodotFfi>::sys_const(&set_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_bind_vertex_array(&mut self, draw_list: i64, vertex_array: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_bind_vertex_array");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4040184819i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_bind_vertex_array" , 4040184819i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <Rid as sys::GodotFfi>::sys_const(&vertex_array),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_bind_index_array(&mut self, draw_list: i64, index_array: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_bind_index_array");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4040184819i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_bind_index_array" , 4040184819i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <Rid as sys::GodotFfi>::sys_const(&index_array),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_set_push_constant(
            &mut self,
            draw_list: i64,
            buffer: PackedByteArray,
            size_bytes: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_set_push_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2772371345i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_set_push_constant" , 2772371345i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&buffer),
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_draw(
            &mut self,
            draw_list: i64,
            use_indices: bool,
            instances: i64,
            procedural_vertex_count: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_draw");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3710874499i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_draw" , 3710874499i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <bool as sys::GodotFfi>::sys_const(&use_indices),
                    <i64 as sys::GodotFfi>::sys_const(&instances),
                    <i64 as sys::GodotFfi>::sys_const(&procedural_vertex_count),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_enable_scissor(&mut self, draw_list: i64, rect: Rect2) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_enable_scissor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    338791288i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_enable_scissor" , 338791288i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&draw_list),
                    <Rect2 as sys::GodotFfi>::sys_const(&rect),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_disable_scissor(&mut self, draw_list: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_disable_scissor");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_disable_scissor" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&draw_list)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_list_switch_to_next_pass(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_switch_to_next_pass");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_switch_to_next_pass" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_list_switch_to_next_pass_split(&mut self, splits: i64) -> PackedInt64Array {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_switch_to_next_pass_split");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2865087369i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_switch_to_next_pass_split" , 2865087369i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&splits)];
                let __args_ptr = __args.as_ptr();
                <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn draw_list_end(&mut self, post_barrier: rendering_device::BarrierMask) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_list_end");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    422991495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_list_end" , 422991495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_device::BarrierMask as sys::GodotFfi>::sys_const(
                    &post_barrier,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_list_begin(&mut self, allow_draw_overlap: bool) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_begin");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    968564752i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_begin" , 968564752i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&allow_draw_overlap)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn compute_list_bind_compute_pipeline(
            &mut self,
            compute_list: i64,
            compute_pipeline: Rid,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_bind_compute_pipeline");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4040184819i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_bind_compute_pipeline" , 4040184819i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&compute_list),
                    <Rid as sys::GodotFfi>::sys_const(&compute_pipeline),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_list_set_push_constant(
            &mut self,
            compute_list: i64,
            buffer: PackedByteArray,
            size_bytes: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_set_push_constant");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2772371345i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_set_push_constant" , 2772371345i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&compute_list),
                    <PackedByteArray as sys::GodotFfi>::sys_const(&buffer),
                    <i64 as sys::GodotFfi>::sys_const(&size_bytes),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_list_bind_uniform_set(
            &mut self,
            compute_list: i64,
            uniform_set: Rid,
            set_index: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_bind_uniform_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    749655778i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_bind_uniform_set" , 749655778i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&compute_list),
                    <Rid as sys::GodotFfi>::sys_const(&uniform_set),
                    <i64 as sys::GodotFfi>::sys_const(&set_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_list_dispatch(
            &mut self,
            compute_list: i64,
            x_groups: i64,
            y_groups: i64,
            z_groups: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_dispatch");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4275841770i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_dispatch" , 4275841770i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&compute_list),
                    <i64 as sys::GodotFfi>::sys_const(&x_groups),
                    <i64 as sys::GodotFfi>::sys_const(&y_groups),
                    <i64 as sys::GodotFfi>::sys_const(&z_groups),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_list_add_barrier(&mut self, compute_list: i64) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_add_barrier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_add_barrier" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&compute_list)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn compute_list_end(&mut self, post_barrier: rendering_device::BarrierMask) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("compute_list_end");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    422991495i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "compute_list_end" , 422991495i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_device::BarrierMask as sys::GodotFfi>::sys_const(
                    &post_barrier,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn free_rid(&mut self, rid: Rid) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("free_rid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2722037293i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "free_rid" , 2722037293i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&rid)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn capture_timestamp(&mut self, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("capture_timestamp");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "capture_timestamp" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_captured_timestamps_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_captured_timestamps_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_captured_timestamps_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_captured_timestamps_frame(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_captured_timestamps_frame");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_captured_timestamps_frame" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_captured_timestamp_gpu_time(&self, index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_captured_timestamp_gpu_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_captured_timestamp_gpu_time" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_captured_timestamp_cpu_time(&self, index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_captured_timestamp_cpu_time");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_captured_timestamp_cpu_time" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_captured_timestamp_name(&self, index: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_captured_timestamp_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_captured_timestamp_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn limit_get(&self, limit: rendering_device::Limit) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("limit_get");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1559202131i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "limit_get" , 1559202131i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_device::Limit as sys::GodotFfi>::sys_const(
                    &limit,
                )];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_frame_delay(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_frame_delay");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_frame_delay" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn submit(&mut self) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("submit");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "submit" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn sync(&mut self) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("sync");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "sync" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn barrier(
            &mut self,
            from: rendering_device::BarrierMask,
            to: rendering_device::BarrierMask,
        ) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("barrier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    266666049i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "barrier" , 266666049i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&from),
                    <rendering_device::BarrierMask as sys::GodotFfi>::sys_const(&to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn full_barrier(&mut self) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("full_barrier");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "full_barrier" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn create_local_device(&mut self) -> Option<Gd<RenderingDevice>> {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("create_local_device");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2846302423i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "create_local_device" , 2846302423i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<RenderingDevice>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_resource_name(&mut self, id: Rid, name: GodotString) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("set_resource_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2726140452i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "set_resource_name" , 2726140452i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Rid as sys::GodotFfi>::sys_const(&id),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_command_begin_label(&mut self, name: GodotString, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_command_begin_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1636512886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_command_begin_label" , 1636512886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_command_insert_label(&mut self, name: GodotString, color: Color) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_command_insert_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1636512886i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_command_insert_label" , 1636512886i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn draw_command_end_label(&mut self) {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("draw_command_end_label");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "draw_command_end_label" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_device_vendor_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_device_vendor_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_device_vendor_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_device_name(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_device_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_device_name" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_device_pipeline_cache_uuid(&self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_device_pipeline_cache_uuid");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    201670096i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_device_pipeline_cache_uuid" , 201670096i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_memory_usage(&self, type_: rendering_device::MemoryType) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_memory_usage");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    251690689i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_memory_usage" , 251690689i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<rendering_device::MemoryType as sys::GodotFfi>::sys_const(
                    &type_,
                )];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_driver_resource(
            &mut self,
            resource: rendering_device::DriverResource,
            rid: Rid,
            index: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("RenderingDevice");
                let __method_name = StringName::from("get_driver_resource");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    501815484i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "RenderingDevice" , "get_driver_resource" , 501815484i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <rendering_device::DriverResource as sys::GodotFfi>::sys_const(&resource),
                    <Rid as sys::GodotFfi>::sys_const(&rid),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub const INVALID_ID: i32 = -1i32;
        pub const INVALID_FORMAT_ID: i32 = -1i32;
    }
    impl crate::obj::GodotClass for RenderingDevice {
        type Base = crate::engine::Object;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "RenderingDevice";
    }
    impl crate::obj::EngineClass for RenderingDevice {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Object> for RenderingDevice {}
    impl std::ops::Deref for RenderingDevice {
        type Target = crate::engine::Object;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for RenderingDevice {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_RenderingDevice {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::RenderingDevice> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DeviceType {
    ord: i32,
}
impl DeviceType {
    pub const DEVICE_TYPE_OTHER: Self = Self { ord: 0 };
    pub const DEVICE_TYPE_INTEGRATED_GPU: Self = Self { ord: 1 };
    pub const DEVICE_TYPE_DISCRETE_GPU: Self = Self { ord: 2 };
    pub const DEVICE_TYPE_VIRTUAL_GPU: Self = Self { ord: 3 };
    pub const DEVICE_TYPE_CPU: Self = Self { ord: 4 };
    pub const DEVICE_TYPE_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for DeviceType {
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
unsafe impl sys::GodotFfi for DeviceType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DriverResource {
    ord: i32,
}
impl DriverResource {
    pub const DRIVER_RESOURCE_VULKAN_DEVICE: Self = Self { ord: 0 };
    pub const DRIVER_RESOURCE_VULKAN_PHYSICAL_DEVICE: Self = Self { ord: 1 };
    pub const DRIVER_RESOURCE_VULKAN_INSTANCE: Self = Self { ord: 2 };
    pub const DRIVER_RESOURCE_VULKAN_QUEUE: Self = Self { ord: 3 };
    pub const DRIVER_RESOURCE_VULKAN_QUEUE_FAMILY_INDEX: Self = Self { ord: 4 };
    pub const DRIVER_RESOURCE_VULKAN_IMAGE: Self = Self { ord: 5 };
    pub const DRIVER_RESOURCE_VULKAN_IMAGE_VIEW: Self = Self { ord: 6 };
    pub const DRIVER_RESOURCE_VULKAN_IMAGE_NATIVE_TEXTURE_FORMAT: Self = Self { ord: 7 };
    pub const DRIVER_RESOURCE_VULKAN_SAMPLER: Self = Self { ord: 8 };
    pub const DRIVER_RESOURCE_VULKAN_DESCRIPTOR_SET: Self = Self { ord: 9 };
    pub const DRIVER_RESOURCE_VULKAN_BUFFER: Self = Self { ord: 10 };
    pub const DRIVER_RESOURCE_VULKAN_COMPUTE_PIPELINE: Self = Self { ord: 11 };
    pub const DRIVER_RESOURCE_VULKAN_RENDER_PIPELINE: Self = Self { ord: 12 };
}
impl crate::obj::EngineEnum for DriverResource {
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
            | ord @ 12i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for DriverResource {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct DataFormat {
    ord: i32,
}
impl DataFormat {
    pub const DATA_FORMAT_R4G4_UNORM_PACK8: Self = Self { ord: 0 };
    pub const DATA_FORMAT_R4G4B4A4_UNORM_PACK16: Self = Self { ord: 1 };
    pub const DATA_FORMAT_B4G4R4A4_UNORM_PACK16: Self = Self { ord: 2 };
    pub const DATA_FORMAT_R5G6B5_UNORM_PACK16: Self = Self { ord: 3 };
    pub const DATA_FORMAT_B5G6R5_UNORM_PACK16: Self = Self { ord: 4 };
    pub const DATA_FORMAT_R5G5B5A1_UNORM_PACK16: Self = Self { ord: 5 };
    pub const DATA_FORMAT_B5G5R5A1_UNORM_PACK16: Self = Self { ord: 6 };
    pub const DATA_FORMAT_A1R5G5B5_UNORM_PACK16: Self = Self { ord: 7 };
    pub const DATA_FORMAT_R8_UNORM: Self = Self { ord: 8 };
    pub const DATA_FORMAT_R8_SNORM: Self = Self { ord: 9 };
    pub const DATA_FORMAT_R8_USCALED: Self = Self { ord: 10 };
    pub const DATA_FORMAT_R8_SSCALED: Self = Self { ord: 11 };
    pub const DATA_FORMAT_R8_UINT: Self = Self { ord: 12 };
    pub const DATA_FORMAT_R8_SINT: Self = Self { ord: 13 };
    pub const DATA_FORMAT_R8_SRGB: Self = Self { ord: 14 };
    pub const DATA_FORMAT_R8G8_UNORM: Self = Self { ord: 15 };
    pub const DATA_FORMAT_R8G8_SNORM: Self = Self { ord: 16 };
    pub const DATA_FORMAT_R8G8_USCALED: Self = Self { ord: 17 };
    pub const DATA_FORMAT_R8G8_SSCALED: Self = Self { ord: 18 };
    pub const DATA_FORMAT_R8G8_UINT: Self = Self { ord: 19 };
    pub const DATA_FORMAT_R8G8_SINT: Self = Self { ord: 20 };
    pub const DATA_FORMAT_R8G8_SRGB: Self = Self { ord: 21 };
    pub const DATA_FORMAT_R8G8B8_UNORM: Self = Self { ord: 22 };
    pub const DATA_FORMAT_R8G8B8_SNORM: Self = Self { ord: 23 };
    pub const DATA_FORMAT_R8G8B8_USCALED: Self = Self { ord: 24 };
    pub const DATA_FORMAT_R8G8B8_SSCALED: Self = Self { ord: 25 };
    pub const DATA_FORMAT_R8G8B8_UINT: Self = Self { ord: 26 };
    pub const DATA_FORMAT_R8G8B8_SINT: Self = Self { ord: 27 };
    pub const DATA_FORMAT_R8G8B8_SRGB: Self = Self { ord: 28 };
    pub const DATA_FORMAT_B8G8R8_UNORM: Self = Self { ord: 29 };
    pub const DATA_FORMAT_B8G8R8_SNORM: Self = Self { ord: 30 };
    pub const DATA_FORMAT_B8G8R8_USCALED: Self = Self { ord: 31 };
    pub const DATA_FORMAT_B8G8R8_SSCALED: Self = Self { ord: 32 };
    pub const DATA_FORMAT_B8G8R8_UINT: Self = Self { ord: 33 };
    pub const DATA_FORMAT_B8G8R8_SINT: Self = Self { ord: 34 };
    pub const DATA_FORMAT_B8G8R8_SRGB: Self = Self { ord: 35 };
    pub const DATA_FORMAT_R8G8B8A8_UNORM: Self = Self { ord: 36 };
    pub const DATA_FORMAT_R8G8B8A8_SNORM: Self = Self { ord: 37 };
    pub const DATA_FORMAT_R8G8B8A8_USCALED: Self = Self { ord: 38 };
    pub const DATA_FORMAT_R8G8B8A8_SSCALED: Self = Self { ord: 39 };
    pub const DATA_FORMAT_R8G8B8A8_UINT: Self = Self { ord: 40 };
    pub const DATA_FORMAT_R8G8B8A8_SINT: Self = Self { ord: 41 };
    pub const DATA_FORMAT_R8G8B8A8_SRGB: Self = Self { ord: 42 };
    pub const DATA_FORMAT_B8G8R8A8_UNORM: Self = Self { ord: 43 };
    pub const DATA_FORMAT_B8G8R8A8_SNORM: Self = Self { ord: 44 };
    pub const DATA_FORMAT_B8G8R8A8_USCALED: Self = Self { ord: 45 };
    pub const DATA_FORMAT_B8G8R8A8_SSCALED: Self = Self { ord: 46 };
    pub const DATA_FORMAT_B8G8R8A8_UINT: Self = Self { ord: 47 };
    pub const DATA_FORMAT_B8G8R8A8_SINT: Self = Self { ord: 48 };
    pub const DATA_FORMAT_B8G8R8A8_SRGB: Self = Self { ord: 49 };
    pub const DATA_FORMAT_A8B8G8R8_UNORM_PACK32: Self = Self { ord: 50 };
    pub const DATA_FORMAT_A8B8G8R8_SNORM_PACK32: Self = Self { ord: 51 };
    pub const DATA_FORMAT_A8B8G8R8_USCALED_PACK32: Self = Self { ord: 52 };
    pub const DATA_FORMAT_A8B8G8R8_SSCALED_PACK32: Self = Self { ord: 53 };
    pub const DATA_FORMAT_A8B8G8R8_UINT_PACK32: Self = Self { ord: 54 };
    pub const DATA_FORMAT_A8B8G8R8_SINT_PACK32: Self = Self { ord: 55 };
    pub const DATA_FORMAT_A8B8G8R8_SRGB_PACK32: Self = Self { ord: 56 };
    pub const DATA_FORMAT_A2R10G10B10_UNORM_PACK32: Self = Self { ord: 57 };
    pub const DATA_FORMAT_A2R10G10B10_SNORM_PACK32: Self = Self { ord: 58 };
    pub const DATA_FORMAT_A2R10G10B10_USCALED_PACK32: Self = Self { ord: 59 };
    pub const DATA_FORMAT_A2R10G10B10_SSCALED_PACK32: Self = Self { ord: 60 };
    pub const DATA_FORMAT_A2R10G10B10_UINT_PACK32: Self = Self { ord: 61 };
    pub const DATA_FORMAT_A2R10G10B10_SINT_PACK32: Self = Self { ord: 62 };
    pub const DATA_FORMAT_A2B10G10R10_UNORM_PACK32: Self = Self { ord: 63 };
    pub const DATA_FORMAT_A2B10G10R10_SNORM_PACK32: Self = Self { ord: 64 };
    pub const DATA_FORMAT_A2B10G10R10_USCALED_PACK32: Self = Self { ord: 65 };
    pub const DATA_FORMAT_A2B10G10R10_SSCALED_PACK32: Self = Self { ord: 66 };
    pub const DATA_FORMAT_A2B10G10R10_UINT_PACK32: Self = Self { ord: 67 };
    pub const DATA_FORMAT_A2B10G10R10_SINT_PACK32: Self = Self { ord: 68 };
    pub const DATA_FORMAT_R16_UNORM: Self = Self { ord: 69 };
    pub const DATA_FORMAT_R16_SNORM: Self = Self { ord: 70 };
    pub const DATA_FORMAT_R16_USCALED: Self = Self { ord: 71 };
    pub const DATA_FORMAT_R16_SSCALED: Self = Self { ord: 72 };
    pub const DATA_FORMAT_R16_UINT: Self = Self { ord: 73 };
    pub const DATA_FORMAT_R16_SINT: Self = Self { ord: 74 };
    pub const DATA_FORMAT_R16_SFLOAT: Self = Self { ord: 75 };
    pub const DATA_FORMAT_R16G16_UNORM: Self = Self { ord: 76 };
    pub const DATA_FORMAT_R16G16_SNORM: Self = Self { ord: 77 };
    pub const DATA_FORMAT_R16G16_USCALED: Self = Self { ord: 78 };
    pub const DATA_FORMAT_R16G16_SSCALED: Self = Self { ord: 79 };
    pub const DATA_FORMAT_R16G16_UINT: Self = Self { ord: 80 };
    pub const DATA_FORMAT_R16G16_SINT: Self = Self { ord: 81 };
    pub const DATA_FORMAT_R16G16_SFLOAT: Self = Self { ord: 82 };
    pub const DATA_FORMAT_R16G16B16_UNORM: Self = Self { ord: 83 };
    pub const DATA_FORMAT_R16G16B16_SNORM: Self = Self { ord: 84 };
    pub const DATA_FORMAT_R16G16B16_USCALED: Self = Self { ord: 85 };
    pub const DATA_FORMAT_R16G16B16_SSCALED: Self = Self { ord: 86 };
    pub const DATA_FORMAT_R16G16B16_UINT: Self = Self { ord: 87 };
    pub const DATA_FORMAT_R16G16B16_SINT: Self = Self { ord: 88 };
    pub const DATA_FORMAT_R16G16B16_SFLOAT: Self = Self { ord: 89 };
    pub const DATA_FORMAT_R16G16B16A16_UNORM: Self = Self { ord: 90 };
    pub const DATA_FORMAT_R16G16B16A16_SNORM: Self = Self { ord: 91 };
    pub const DATA_FORMAT_R16G16B16A16_USCALED: Self = Self { ord: 92 };
    pub const DATA_FORMAT_R16G16B16A16_SSCALED: Self = Self { ord: 93 };
    pub const DATA_FORMAT_R16G16B16A16_UINT: Self = Self { ord: 94 };
    pub const DATA_FORMAT_R16G16B16A16_SINT: Self = Self { ord: 95 };
    pub const DATA_FORMAT_R16G16B16A16_SFLOAT: Self = Self { ord: 96 };
    pub const DATA_FORMAT_R32_UINT: Self = Self { ord: 97 };
    pub const DATA_FORMAT_R32_SINT: Self = Self { ord: 98 };
    pub const DATA_FORMAT_R32_SFLOAT: Self = Self { ord: 99 };
    pub const DATA_FORMAT_R32G32_UINT: Self = Self { ord: 100 };
    pub const DATA_FORMAT_R32G32_SINT: Self = Self { ord: 101 };
    pub const DATA_FORMAT_R32G32_SFLOAT: Self = Self { ord: 102 };
    pub const DATA_FORMAT_R32G32B32_UINT: Self = Self { ord: 103 };
    pub const DATA_FORMAT_R32G32B32_SINT: Self = Self { ord: 104 };
    pub const DATA_FORMAT_R32G32B32_SFLOAT: Self = Self { ord: 105 };
    pub const DATA_FORMAT_R32G32B32A32_UINT: Self = Self { ord: 106 };
    pub const DATA_FORMAT_R32G32B32A32_SINT: Self = Self { ord: 107 };
    pub const DATA_FORMAT_R32G32B32A32_SFLOAT: Self = Self { ord: 108 };
    pub const DATA_FORMAT_R64_UINT: Self = Self { ord: 109 };
    pub const DATA_FORMAT_R64_SINT: Self = Self { ord: 110 };
    pub const DATA_FORMAT_R64_SFLOAT: Self = Self { ord: 111 };
    pub const DATA_FORMAT_R64G64_UINT: Self = Self { ord: 112 };
    pub const DATA_FORMAT_R64G64_SINT: Self = Self { ord: 113 };
    pub const DATA_FORMAT_R64G64_SFLOAT: Self = Self { ord: 114 };
    pub const DATA_FORMAT_R64G64B64_UINT: Self = Self { ord: 115 };
    pub const DATA_FORMAT_R64G64B64_SINT: Self = Self { ord: 116 };
    pub const DATA_FORMAT_R64G64B64_SFLOAT: Self = Self { ord: 117 };
    pub const DATA_FORMAT_R64G64B64A64_UINT: Self = Self { ord: 118 };
    pub const DATA_FORMAT_R64G64B64A64_SINT: Self = Self { ord: 119 };
    pub const DATA_FORMAT_R64G64B64A64_SFLOAT: Self = Self { ord: 120 };
    pub const DATA_FORMAT_B10G11R11_UFLOAT_PACK32: Self = Self { ord: 121 };
    pub const DATA_FORMAT_E5B9G9R9_UFLOAT_PACK32: Self = Self { ord: 122 };
    pub const DATA_FORMAT_D16_UNORM: Self = Self { ord: 123 };
    pub const DATA_FORMAT_X8_D24_UNORM_PACK32: Self = Self { ord: 124 };
    pub const DATA_FORMAT_D32_SFLOAT: Self = Self { ord: 125 };
    pub const DATA_FORMAT_S8_UINT: Self = Self { ord: 126 };
    pub const DATA_FORMAT_D16_UNORM_S8_UINT: Self = Self { ord: 127 };
    pub const DATA_FORMAT_D24_UNORM_S8_UINT: Self = Self { ord: 128 };
    pub const DATA_FORMAT_D32_SFLOAT_S8_UINT: Self = Self { ord: 129 };
    pub const DATA_FORMAT_BC1_RGB_UNORM_BLOCK: Self = Self { ord: 130 };
    pub const DATA_FORMAT_BC1_RGB_SRGB_BLOCK: Self = Self { ord: 131 };
    pub const DATA_FORMAT_BC1_RGBA_UNORM_BLOCK: Self = Self { ord: 132 };
    pub const DATA_FORMAT_BC1_RGBA_SRGB_BLOCK: Self = Self { ord: 133 };
    pub const DATA_FORMAT_BC2_UNORM_BLOCK: Self = Self { ord: 134 };
    pub const DATA_FORMAT_BC2_SRGB_BLOCK: Self = Self { ord: 135 };
    pub const DATA_FORMAT_BC3_UNORM_BLOCK: Self = Self { ord: 136 };
    pub const DATA_FORMAT_BC3_SRGB_BLOCK: Self = Self { ord: 137 };
    pub const DATA_FORMAT_BC4_UNORM_BLOCK: Self = Self { ord: 138 };
    pub const DATA_FORMAT_BC4_SNORM_BLOCK: Self = Self { ord: 139 };
    pub const DATA_FORMAT_BC5_UNORM_BLOCK: Self = Self { ord: 140 };
    pub const DATA_FORMAT_BC5_SNORM_BLOCK: Self = Self { ord: 141 };
    pub const DATA_FORMAT_BC6H_UFLOAT_BLOCK: Self = Self { ord: 142 };
    pub const DATA_FORMAT_BC6H_SFLOAT_BLOCK: Self = Self { ord: 143 };
    pub const DATA_FORMAT_BC7_UNORM_BLOCK: Self = Self { ord: 144 };
    pub const DATA_FORMAT_BC7_SRGB_BLOCK: Self = Self { ord: 145 };
    pub const DATA_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: Self = Self { ord: 146 };
    pub const DATA_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: Self = Self { ord: 147 };
    pub const DATA_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self { ord: 148 };
    pub const DATA_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self { ord: 149 };
    pub const DATA_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self { ord: 150 };
    pub const DATA_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self { ord: 151 };
    pub const DATA_FORMAT_EAC_R11_UNORM_BLOCK: Self = Self { ord: 152 };
    pub const DATA_FORMAT_EAC_R11_SNORM_BLOCK: Self = Self { ord: 153 };
    pub const DATA_FORMAT_EAC_R11G11_UNORM_BLOCK: Self = Self { ord: 154 };
    pub const DATA_FORMAT_EAC_R11G11_SNORM_BLOCK: Self = Self { ord: 155 };
    pub const DATA_FORMAT_ASTC_4x4_UNORM_BLOCK: Self = Self { ord: 156 };
    pub const DATA_FORMAT_ASTC_4x4_SRGB_BLOCK: Self = Self { ord: 157 };
    pub const DATA_FORMAT_ASTC_5x4_UNORM_BLOCK: Self = Self { ord: 158 };
    pub const DATA_FORMAT_ASTC_5x4_SRGB_BLOCK: Self = Self { ord: 159 };
    pub const DATA_FORMAT_ASTC_5x5_UNORM_BLOCK: Self = Self { ord: 160 };
    pub const DATA_FORMAT_ASTC_5x5_SRGB_BLOCK: Self = Self { ord: 161 };
    pub const DATA_FORMAT_ASTC_6x5_UNORM_BLOCK: Self = Self { ord: 162 };
    pub const DATA_FORMAT_ASTC_6x5_SRGB_BLOCK: Self = Self { ord: 163 };
    pub const DATA_FORMAT_ASTC_6x6_UNORM_BLOCK: Self = Self { ord: 164 };
    pub const DATA_FORMAT_ASTC_6x6_SRGB_BLOCK: Self = Self { ord: 165 };
    pub const DATA_FORMAT_ASTC_8x5_UNORM_BLOCK: Self = Self { ord: 166 };
    pub const DATA_FORMAT_ASTC_8x5_SRGB_BLOCK: Self = Self { ord: 167 };
    pub const DATA_FORMAT_ASTC_8x6_UNORM_BLOCK: Self = Self { ord: 168 };
    pub const DATA_FORMAT_ASTC_8x6_SRGB_BLOCK: Self = Self { ord: 169 };
    pub const DATA_FORMAT_ASTC_8x8_UNORM_BLOCK: Self = Self { ord: 170 };
    pub const DATA_FORMAT_ASTC_8x8_SRGB_BLOCK: Self = Self { ord: 171 };
    pub const DATA_FORMAT_ASTC_10x5_UNORM_BLOCK: Self = Self { ord: 172 };
    pub const DATA_FORMAT_ASTC_10x5_SRGB_BLOCK: Self = Self { ord: 173 };
    pub const DATA_FORMAT_ASTC_10x6_UNORM_BLOCK: Self = Self { ord: 174 };
    pub const DATA_FORMAT_ASTC_10x6_SRGB_BLOCK: Self = Self { ord: 175 };
    pub const DATA_FORMAT_ASTC_10x8_UNORM_BLOCK: Self = Self { ord: 176 };
    pub const DATA_FORMAT_ASTC_10x8_SRGB_BLOCK: Self = Self { ord: 177 };
    pub const DATA_FORMAT_ASTC_10x10_UNORM_BLOCK: Self = Self { ord: 178 };
    pub const DATA_FORMAT_ASTC_10x10_SRGB_BLOCK: Self = Self { ord: 179 };
    pub const DATA_FORMAT_ASTC_12x10_UNORM_BLOCK: Self = Self { ord: 180 };
    pub const DATA_FORMAT_ASTC_12x10_SRGB_BLOCK: Self = Self { ord: 181 };
    pub const DATA_FORMAT_ASTC_12x12_UNORM_BLOCK: Self = Self { ord: 182 };
    pub const DATA_FORMAT_ASTC_12x12_SRGB_BLOCK: Self = Self { ord: 183 };
    pub const DATA_FORMAT_G8B8G8R8_422_UNORM: Self = Self { ord: 184 };
    pub const DATA_FORMAT_B8G8R8G8_422_UNORM: Self = Self { ord: 185 };
    pub const DATA_FORMAT_G8_B8_R8_3PLANE_420_UNORM: Self = Self { ord: 186 };
    pub const DATA_FORMAT_G8_B8R8_2PLANE_420_UNORM: Self = Self { ord: 187 };
    pub const DATA_FORMAT_G8_B8_R8_3PLANE_422_UNORM: Self = Self { ord: 188 };
    pub const DATA_FORMAT_G8_B8R8_2PLANE_422_UNORM: Self = Self { ord: 189 };
    pub const DATA_FORMAT_G8_B8_R8_3PLANE_444_UNORM: Self = Self { ord: 190 };
    pub const DATA_FORMAT_R10X6_UNORM_PACK16: Self = Self { ord: 191 };
    pub const DATA_FORMAT_R10X6G10X6_UNORM_2PACK16: Self = Self { ord: 192 };
    pub const DATA_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self { ord: 193 };
    pub const DATA_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self { ord: 194 };
    pub const DATA_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self { ord: 195 };
    pub const DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self { ord: 196 };
    pub const DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self { ord: 197 };
    pub const DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self { ord: 198 };
    pub const DATA_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self { ord: 199 };
    pub const DATA_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self { ord: 200 };
    pub const DATA_FORMAT_R12X4_UNORM_PACK16: Self = Self { ord: 201 };
    pub const DATA_FORMAT_R12X4G12X4_UNORM_2PACK16: Self = Self { ord: 202 };
    pub const DATA_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self { ord: 203 };
    pub const DATA_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self { ord: 204 };
    pub const DATA_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self { ord: 205 };
    pub const DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self { ord: 206 };
    pub const DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self { ord: 207 };
    pub const DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self { ord: 208 };
    pub const DATA_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self { ord: 209 };
    pub const DATA_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self { ord: 210 };
    pub const DATA_FORMAT_G16B16G16R16_422_UNORM: Self = Self { ord: 211 };
    pub const DATA_FORMAT_B16G16R16G16_422_UNORM: Self = Self { ord: 212 };
    pub const DATA_FORMAT_G16_B16_R16_3PLANE_420_UNORM: Self = Self { ord: 213 };
    pub const DATA_FORMAT_G16_B16R16_2PLANE_420_UNORM: Self = Self { ord: 214 };
    pub const DATA_FORMAT_G16_B16_R16_3PLANE_422_UNORM: Self = Self { ord: 215 };
    pub const DATA_FORMAT_G16_B16R16_2PLANE_422_UNORM: Self = Self { ord: 216 };
    pub const DATA_FORMAT_G16_B16_R16_3PLANE_444_UNORM: Self = Self { ord: 217 };
    pub const DATA_FORMAT_MAX: Self = Self { ord: 218 };
}
impl crate::obj::EngineEnum for DataFormat {
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
            | ord @ 33i32
            | ord @ 34i32
            | ord @ 35i32
            | ord @ 36i32
            | ord @ 37i32
            | ord @ 38i32
            | ord @ 39i32
            | ord @ 40i32
            | ord @ 41i32
            | ord @ 42i32
            | ord @ 43i32
            | ord @ 44i32
            | ord @ 45i32
            | ord @ 46i32
            | ord @ 47i32
            | ord @ 48i32
            | ord @ 49i32
            | ord @ 50i32
            | ord @ 51i32
            | ord @ 52i32
            | ord @ 53i32
            | ord @ 54i32
            | ord @ 55i32
            | ord @ 56i32
            | ord @ 57i32
            | ord @ 58i32
            | ord @ 59i32
            | ord @ 60i32
            | ord @ 61i32
            | ord @ 62i32
            | ord @ 63i32
            | ord @ 64i32
            | ord @ 65i32
            | ord @ 66i32
            | ord @ 67i32
            | ord @ 68i32
            | ord @ 69i32
            | ord @ 70i32
            | ord @ 71i32
            | ord @ 72i32
            | ord @ 73i32
            | ord @ 74i32
            | ord @ 75i32
            | ord @ 76i32
            | ord @ 77i32
            | ord @ 78i32
            | ord @ 79i32
            | ord @ 80i32
            | ord @ 81i32
            | ord @ 82i32
            | ord @ 83i32
            | ord @ 84i32
            | ord @ 85i32
            | ord @ 86i32
            | ord @ 87i32
            | ord @ 88i32
            | ord @ 89i32
            | ord @ 90i32
            | ord @ 91i32
            | ord @ 92i32
            | ord @ 93i32
            | ord @ 94i32
            | ord @ 95i32
            | ord @ 96i32
            | ord @ 97i32
            | ord @ 98i32
            | ord @ 99i32
            | ord @ 100i32
            | ord @ 101i32
            | ord @ 102i32
            | ord @ 103i32
            | ord @ 104i32
            | ord @ 105i32
            | ord @ 106i32
            | ord @ 107i32
            | ord @ 108i32
            | ord @ 109i32
            | ord @ 110i32
            | ord @ 111i32
            | ord @ 112i32
            | ord @ 113i32
            | ord @ 114i32
            | ord @ 115i32
            | ord @ 116i32
            | ord @ 117i32
            | ord @ 118i32
            | ord @ 119i32
            | ord @ 120i32
            | ord @ 121i32
            | ord @ 122i32
            | ord @ 123i32
            | ord @ 124i32
            | ord @ 125i32
            | ord @ 126i32
            | ord @ 127i32
            | ord @ 128i32
            | ord @ 129i32
            | ord @ 130i32
            | ord @ 131i32
            | ord @ 132i32
            | ord @ 133i32
            | ord @ 134i32
            | ord @ 135i32
            | ord @ 136i32
            | ord @ 137i32
            | ord @ 138i32
            | ord @ 139i32
            | ord @ 140i32
            | ord @ 141i32
            | ord @ 142i32
            | ord @ 143i32
            | ord @ 144i32
            | ord @ 145i32
            | ord @ 146i32
            | ord @ 147i32
            | ord @ 148i32
            | ord @ 149i32
            | ord @ 150i32
            | ord @ 151i32
            | ord @ 152i32
            | ord @ 153i32
            | ord @ 154i32
            | ord @ 155i32
            | ord @ 156i32
            | ord @ 157i32
            | ord @ 158i32
            | ord @ 159i32
            | ord @ 160i32
            | ord @ 161i32
            | ord @ 162i32
            | ord @ 163i32
            | ord @ 164i32
            | ord @ 165i32
            | ord @ 166i32
            | ord @ 167i32
            | ord @ 168i32
            | ord @ 169i32
            | ord @ 170i32
            | ord @ 171i32
            | ord @ 172i32
            | ord @ 173i32
            | ord @ 174i32
            | ord @ 175i32
            | ord @ 176i32
            | ord @ 177i32
            | ord @ 178i32
            | ord @ 179i32
            | ord @ 180i32
            | ord @ 181i32
            | ord @ 182i32
            | ord @ 183i32
            | ord @ 184i32
            | ord @ 185i32
            | ord @ 186i32
            | ord @ 187i32
            | ord @ 188i32
            | ord @ 189i32
            | ord @ 190i32
            | ord @ 191i32
            | ord @ 192i32
            | ord @ 193i32
            | ord @ 194i32
            | ord @ 195i32
            | ord @ 196i32
            | ord @ 197i32
            | ord @ 198i32
            | ord @ 199i32
            | ord @ 200i32
            | ord @ 201i32
            | ord @ 202i32
            | ord @ 203i32
            | ord @ 204i32
            | ord @ 205i32
            | ord @ 206i32
            | ord @ 207i32
            | ord @ 208i32
            | ord @ 209i32
            | ord @ 210i32
            | ord @ 211i32
            | ord @ 212i32
            | ord @ 213i32
            | ord @ 214i32
            | ord @ 215i32
            | ord @ 216i32
            | ord @ 217i32
            | ord @ 218i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for DataFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct BarrierMask {
    ord: i32,
}
impl BarrierMask {
    pub const BARRIER_MASK_RASTER: Self = Self { ord: 1 };
    pub const BARRIER_MASK_COMPUTE: Self = Self { ord: 2 };
    pub const BARRIER_MASK_TRANSFER: Self = Self { ord: 4 };
    pub const BARRIER_MASK_ALL_BARRIERS: Self = Self { ord: 7 };
    pub const BARRIER_MASK_NO_BARRIER: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for BarrierMask {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 7i32 | ord @ 8i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for BarrierMask {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for BarrierMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureType {
    ord: i32,
}
impl TextureType {
    pub const TEXTURE_TYPE_1D: Self = Self { ord: 0 };
    pub const TEXTURE_TYPE_2D: Self = Self { ord: 1 };
    pub const TEXTURE_TYPE_3D: Self = Self { ord: 2 };
    pub const TEXTURE_TYPE_CUBE: Self = Self { ord: 3 };
    pub const TEXTURE_TYPE_1D_ARRAY: Self = Self { ord: 4 };
    pub const TEXTURE_TYPE_2D_ARRAY: Self = Self { ord: 5 };
    pub const TEXTURE_TYPE_CUBE_ARRAY: Self = Self { ord: 6 };
    pub const TEXTURE_TYPE_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for TextureType {
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
unsafe impl sys::GodotFfi for TextureType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureSamples {
    ord: i32,
}
impl TextureSamples {
    pub const TEXTURE_SAMPLES_1: Self = Self { ord: 0 };
    pub const TEXTURE_SAMPLES_2: Self = Self { ord: 1 };
    pub const TEXTURE_SAMPLES_4: Self = Self { ord: 2 };
    pub const TEXTURE_SAMPLES_8: Self = Self { ord: 3 };
    pub const TEXTURE_SAMPLES_16: Self = Self { ord: 4 };
    pub const TEXTURE_SAMPLES_32: Self = Self { ord: 5 };
    pub const TEXTURE_SAMPLES_64: Self = Self { ord: 6 };
    pub const TEXTURE_SAMPLES_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for TextureSamples {
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
unsafe impl sys::GodotFfi for TextureSamples {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct TextureUsageBits {
    ord: i32,
}
impl TextureUsageBits {
    pub const TEXTURE_USAGE_SAMPLING_BIT: Self = Self { ord: 1 };
    pub const TEXTURE_USAGE_COLOR_ATTACHMENT_BIT: Self = Self { ord: 2 };
    pub const TEXTURE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: Self = Self { ord: 4 };
    pub const TEXTURE_USAGE_STORAGE_BIT: Self = Self { ord: 8 };
    pub const TEXTURE_USAGE_STORAGE_ATOMIC_BIT: Self = Self { ord: 16 };
    pub const TEXTURE_USAGE_CPU_READ_BIT: Self = Self { ord: 32 };
    pub const TEXTURE_USAGE_CAN_UPDATE_BIT: Self = Self { ord: 64 };
    pub const TEXTURE_USAGE_CAN_COPY_FROM_BIT: Self = Self { ord: 128 };
    pub const TEXTURE_USAGE_CAN_COPY_TO_BIT: Self = Self { ord: 256 };
    pub const TEXTURE_USAGE_INPUT_ATTACHMENT_BIT: Self = Self { ord: 512 };
}
impl crate::obj::EngineEnum for TextureUsageBits {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32
            | ord @ 2i32
            | ord @ 4i32
            | ord @ 8i32
            | ord @ 16i32
            | ord @ 32i32
            | ord @ 64i32
            | ord @ 128i32
            | ord @ 256i32
            | ord @ 512i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for TextureUsageBits {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for TextureUsageBits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureSwizzle {
    ord: i32,
}
impl TextureSwizzle {
    pub const TEXTURE_SWIZZLE_IDENTITY: Self = Self { ord: 0 };
    pub const TEXTURE_SWIZZLE_ZERO: Self = Self { ord: 1 };
    pub const TEXTURE_SWIZZLE_ONE: Self = Self { ord: 2 };
    pub const TEXTURE_SWIZZLE_R: Self = Self { ord: 3 };
    pub const TEXTURE_SWIZZLE_G: Self = Self { ord: 4 };
    pub const TEXTURE_SWIZZLE_B: Self = Self { ord: 5 };
    pub const TEXTURE_SWIZZLE_A: Self = Self { ord: 6 };
    pub const TEXTURE_SWIZZLE_MAX: Self = Self { ord: 7 };
}
impl crate::obj::EngineEnum for TextureSwizzle {
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
unsafe impl sys::GodotFfi for TextureSwizzle {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TextureSliceType {
    ord: i32,
}
impl TextureSliceType {
    pub const TEXTURE_SLICE_2D: Self = Self { ord: 0 };
    pub const TEXTURE_SLICE_CUBEMAP: Self = Self { ord: 1 };
    pub const TEXTURE_SLICE_3D: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for TextureSliceType {
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
unsafe impl sys::GodotFfi for TextureSliceType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SamplerFilter {
    ord: i32,
}
impl SamplerFilter {
    pub const SAMPLER_FILTER_NEAREST: Self = Self { ord: 0 };
    pub const SAMPLER_FILTER_LINEAR: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for SamplerFilter {
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
unsafe impl sys::GodotFfi for SamplerFilter {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SamplerRepeatMode {
    ord: i32,
}
impl SamplerRepeatMode {
    pub const SAMPLER_REPEAT_MODE_REPEAT: Self = Self { ord: 0 };
    pub const SAMPLER_REPEAT_MODE_MIRRORED_REPEAT: Self = Self { ord: 1 };
    pub const SAMPLER_REPEAT_MODE_CLAMP_TO_EDGE: Self = Self { ord: 2 };
    pub const SAMPLER_REPEAT_MODE_CLAMP_TO_BORDER: Self = Self { ord: 3 };
    pub const SAMPLER_REPEAT_MODE_MIRROR_CLAMP_TO_EDGE: Self = Self { ord: 4 };
    pub const SAMPLER_REPEAT_MODE_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for SamplerRepeatMode {
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
unsafe impl sys::GodotFfi for SamplerRepeatMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct SamplerBorderColor {
    ord: i32,
}
impl SamplerBorderColor {
    pub const SAMPLER_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: Self = Self { ord: 0 };
    pub const SAMPLER_BORDER_COLOR_INT_TRANSPARENT_BLACK: Self = Self { ord: 1 };
    pub const SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_BLACK: Self = Self { ord: 2 };
    pub const SAMPLER_BORDER_COLOR_INT_OPAQUE_BLACK: Self = Self { ord: 3 };
    pub const SAMPLER_BORDER_COLOR_FLOAT_OPAQUE_WHITE: Self = Self { ord: 4 };
    pub const SAMPLER_BORDER_COLOR_INT_OPAQUE_WHITE: Self = Self { ord: 5 };
    pub const SAMPLER_BORDER_COLOR_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for SamplerBorderColor {
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
unsafe impl sys::GodotFfi for SamplerBorderColor {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct VertexFrequency {
    ord: i32,
}
impl VertexFrequency {
    pub const VERTEX_FREQUENCY_VERTEX: Self = Self { ord: 0 };
    pub const VERTEX_FREQUENCY_INSTANCE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for VertexFrequency {
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
unsafe impl sys::GodotFfi for VertexFrequency {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct IndexBufferFormat {
    ord: i32,
}
impl IndexBufferFormat {
    pub const INDEX_BUFFER_FORMAT_UINT16: Self = Self { ord: 0 };
    pub const INDEX_BUFFER_FORMAT_UINT32: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for IndexBufferFormat {
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
unsafe impl sys::GodotFfi for IndexBufferFormat {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct StorageBufferUsage {
    ord: i32,
}
impl StorageBufferUsage {
    pub const STORAGE_BUFFER_USAGE_DISPATCH_INDIRECT: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for StorageBufferUsage {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for StorageBufferUsage {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for StorageBufferUsage {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct UniformType {
    ord: i32,
}
impl UniformType {
    pub const UNIFORM_TYPE_SAMPLER: Self = Self { ord: 0 };
    pub const UNIFORM_TYPE_SAMPLER_WITH_TEXTURE: Self = Self { ord: 1 };
    pub const UNIFORM_TYPE_TEXTURE: Self = Self { ord: 2 };
    pub const UNIFORM_TYPE_IMAGE: Self = Self { ord: 3 };
    pub const UNIFORM_TYPE_TEXTURE_BUFFER: Self = Self { ord: 4 };
    pub const UNIFORM_TYPE_SAMPLER_WITH_TEXTURE_BUFFER: Self = Self { ord: 5 };
    pub const UNIFORM_TYPE_IMAGE_BUFFER: Self = Self { ord: 6 };
    pub const UNIFORM_TYPE_UNIFORM_BUFFER: Self = Self { ord: 7 };
    pub const UNIFORM_TYPE_STORAGE_BUFFER: Self = Self { ord: 8 };
    pub const UNIFORM_TYPE_INPUT_ATTACHMENT: Self = Self { ord: 9 };
    pub const UNIFORM_TYPE_MAX: Self = Self { ord: 10 };
}
impl crate::obj::EngineEnum for UniformType {
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
unsafe impl sys::GodotFfi for UniformType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct RenderPrimitive {
    ord: i32,
}
impl RenderPrimitive {
    pub const RENDER_PRIMITIVE_POINTS: Self = Self { ord: 0 };
    pub const RENDER_PRIMITIVE_LINES: Self = Self { ord: 1 };
    pub const RENDER_PRIMITIVE_LINES_WITH_ADJACENCY: Self = Self { ord: 2 };
    pub const RENDER_PRIMITIVE_LINESTRIPS: Self = Self { ord: 3 };
    pub const RENDER_PRIMITIVE_LINESTRIPS_WITH_ADJACENCY: Self = Self { ord: 4 };
    pub const RENDER_PRIMITIVE_TRIANGLES: Self = Self { ord: 5 };
    pub const RENDER_PRIMITIVE_TRIANGLES_WITH_ADJACENCY: Self = Self { ord: 6 };
    pub const RENDER_PRIMITIVE_TRIANGLE_STRIPS: Self = Self { ord: 7 };
    pub const RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_AJACENCY: Self = Self { ord: 8 };
    pub const RENDER_PRIMITIVE_TRIANGLE_STRIPS_WITH_RESTART_INDEX: Self = Self { ord: 9 };
    pub const RENDER_PRIMITIVE_TESSELATION_PATCH: Self = Self { ord: 10 };
    pub const RENDER_PRIMITIVE_MAX: Self = Self { ord: 11 };
}
impl crate::obj::EngineEnum for RenderPrimitive {
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
            | ord @ 11i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for RenderPrimitive {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PolygonCullMode {
    ord: i32,
}
impl PolygonCullMode {
    pub const POLYGON_CULL_DISABLED: Self = Self { ord: 0 };
    pub const POLYGON_CULL_FRONT: Self = Self { ord: 1 };
    pub const POLYGON_CULL_BACK: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for PolygonCullMode {
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
unsafe impl sys::GodotFfi for PolygonCullMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PolygonFrontFace {
    ord: i32,
}
impl PolygonFrontFace {
    pub const POLYGON_FRONT_FACE_CLOCKWISE: Self = Self { ord: 0 };
    pub const POLYGON_FRONT_FACE_COUNTER_CLOCKWISE: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for PolygonFrontFace {
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
unsafe impl sys::GodotFfi for PolygonFrontFace {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct StencilOperation {
    ord: i32,
}
impl StencilOperation {
    pub const STENCIL_OP_KEEP: Self = Self { ord: 0 };
    pub const STENCIL_OP_ZERO: Self = Self { ord: 1 };
    pub const STENCIL_OP_REPLACE: Self = Self { ord: 2 };
    pub const STENCIL_OP_INCREMENT_AND_CLAMP: Self = Self { ord: 3 };
    pub const STENCIL_OP_DECREMENT_AND_CLAMP: Self = Self { ord: 4 };
    pub const STENCIL_OP_INVERT: Self = Self { ord: 5 };
    pub const STENCIL_OP_INCREMENT_AND_WRAP: Self = Self { ord: 6 };
    pub const STENCIL_OP_DECREMENT_AND_WRAP: Self = Self { ord: 7 };
    pub const STENCIL_OP_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for StencilOperation {
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
unsafe impl sys::GodotFfi for StencilOperation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CompareOperator {
    ord: i32,
}
impl CompareOperator {
    pub const COMPARE_OP_NEVER: Self = Self { ord: 0 };
    pub const COMPARE_OP_LESS: Self = Self { ord: 1 };
    pub const COMPARE_OP_EQUAL: Self = Self { ord: 2 };
    pub const COMPARE_OP_LESS_OR_EQUAL: Self = Self { ord: 3 };
    pub const COMPARE_OP_GREATER: Self = Self { ord: 4 };
    pub const COMPARE_OP_NOT_EQUAL: Self = Self { ord: 5 };
    pub const COMPARE_OP_GREATER_OR_EQUAL: Self = Self { ord: 6 };
    pub const COMPARE_OP_ALWAYS: Self = Self { ord: 7 };
    pub const COMPARE_OP_MAX: Self = Self { ord: 8 };
}
impl crate::obj::EngineEnum for CompareOperator {
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
unsafe impl sys::GodotFfi for CompareOperator {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct LogicOperation {
    ord: i32,
}
impl LogicOperation {
    pub const LOGIC_OP_CLEAR: Self = Self { ord: 0 };
    pub const LOGIC_OP_AND: Self = Self { ord: 1 };
    pub const LOGIC_OP_AND_REVERSE: Self = Self { ord: 2 };
    pub const LOGIC_OP_COPY: Self = Self { ord: 3 };
    pub const LOGIC_OP_AND_INVERTED: Self = Self { ord: 4 };
    pub const LOGIC_OP_NO_OP: Self = Self { ord: 5 };
    pub const LOGIC_OP_XOR: Self = Self { ord: 6 };
    pub const LOGIC_OP_OR: Self = Self { ord: 7 };
    pub const LOGIC_OP_NOR: Self = Self { ord: 8 };
    pub const LOGIC_OP_EQUIVALENT: Self = Self { ord: 9 };
    pub const LOGIC_OP_INVERT: Self = Self { ord: 10 };
    pub const LOGIC_OP_OR_REVERSE: Self = Self { ord: 11 };
    pub const LOGIC_OP_COPY_INVERTED: Self = Self { ord: 12 };
    pub const LOGIC_OP_OR_INVERTED: Self = Self { ord: 13 };
    pub const LOGIC_OP_NAND: Self = Self { ord: 14 };
    pub const LOGIC_OP_SET: Self = Self { ord: 15 };
    pub const LOGIC_OP_MAX: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for LogicOperation {
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
            | ord @ 16i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for LogicOperation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlendFactor {
    ord: i32,
}
impl BlendFactor {
    pub const BLEND_FACTOR_ZERO: Self = Self { ord: 0 };
    pub const BLEND_FACTOR_ONE: Self = Self { ord: 1 };
    pub const BLEND_FACTOR_SRC_COLOR: Self = Self { ord: 2 };
    pub const BLEND_FACTOR_ONE_MINUS_SRC_COLOR: Self = Self { ord: 3 };
    pub const BLEND_FACTOR_DST_COLOR: Self = Self { ord: 4 };
    pub const BLEND_FACTOR_ONE_MINUS_DST_COLOR: Self = Self { ord: 5 };
    pub const BLEND_FACTOR_SRC_ALPHA: Self = Self { ord: 6 };
    pub const BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: Self = Self { ord: 7 };
    pub const BLEND_FACTOR_DST_ALPHA: Self = Self { ord: 8 };
    pub const BLEND_FACTOR_ONE_MINUS_DST_ALPHA: Self = Self { ord: 9 };
    pub const BLEND_FACTOR_CONSTANT_COLOR: Self = Self { ord: 10 };
    pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: Self = Self { ord: 11 };
    pub const BLEND_FACTOR_CONSTANT_ALPHA: Self = Self { ord: 12 };
    pub const BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: Self = Self { ord: 13 };
    pub const BLEND_FACTOR_SRC_ALPHA_SATURATE: Self = Self { ord: 14 };
    pub const BLEND_FACTOR_SRC1_COLOR: Self = Self { ord: 15 };
    pub const BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: Self = Self { ord: 16 };
    pub const BLEND_FACTOR_SRC1_ALPHA: Self = Self { ord: 17 };
    pub const BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: Self = Self { ord: 18 };
    pub const BLEND_FACTOR_MAX: Self = Self { ord: 19 };
}
impl crate::obj::EngineEnum for BlendFactor {
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
            | ord @ 19i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for BlendFactor {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BlendOperation {
    ord: i32,
}
impl BlendOperation {
    pub const BLEND_OP_ADD: Self = Self { ord: 0 };
    pub const BLEND_OP_SUBTRACT: Self = Self { ord: 1 };
    pub const BLEND_OP_REVERSE_SUBTRACT: Self = Self { ord: 2 };
    pub const BLEND_OP_MINIMUM: Self = Self { ord: 3 };
    pub const BLEND_OP_MAXIMUM: Self = Self { ord: 4 };
    pub const BLEND_OP_MAX: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for BlendOperation {
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
unsafe impl sys::GodotFfi for BlendOperation {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct PipelineDynamicStateFlags {
    ord: i32,
}
impl PipelineDynamicStateFlags {
    pub const DYNAMIC_STATE_LINE_WIDTH: Self = Self { ord: 1 };
    pub const DYNAMIC_STATE_DEPTH_BIAS: Self = Self { ord: 2 };
    pub const DYNAMIC_STATE_BLEND_CONSTANTS: Self = Self { ord: 4 };
    pub const DYNAMIC_STATE_DEPTH_BOUNDS: Self = Self { ord: 8 };
    pub const DYNAMIC_STATE_STENCIL_COMPARE_MASK: Self = Self { ord: 16 };
    pub const DYNAMIC_STATE_STENCIL_WRITE_MASK: Self = Self { ord: 32 };
    pub const DYNAMIC_STATE_STENCIL_REFERENCE: Self = Self { ord: 64 };
}
impl crate::obj::EngineEnum for PipelineDynamicStateFlags {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 1i32
            | ord @ 2i32
            | ord @ 4i32
            | ord @ 8i32
            | ord @ 16i32
            | ord @ 32i32
            | ord @ 64i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for PipelineDynamicStateFlags {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
impl std::ops::BitOr for PipelineDynamicStateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            ord: self.ord | rhs.ord,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct InitialAction {
    ord: i32,
}
impl InitialAction {
    pub const INITIAL_ACTION_CLEAR: Self = Self { ord: 0 };
    pub const INITIAL_ACTION_CLEAR_REGION: Self = Self { ord: 1 };
    pub const INITIAL_ACTION_CLEAR_REGION_CONTINUE: Self = Self { ord: 2 };
    pub const INITIAL_ACTION_KEEP: Self = Self { ord: 3 };
    pub const INITIAL_ACTION_DROP: Self = Self { ord: 4 };
    pub const INITIAL_ACTION_CONTINUE: Self = Self { ord: 5 };
    pub const INITIAL_ACTION_MAX: Self = Self { ord: 6 };
}
impl crate::obj::EngineEnum for InitialAction {
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
unsafe impl sys::GodotFfi for InitialAction {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct FinalAction {
    ord: i32,
}
impl FinalAction {
    pub const FINAL_ACTION_READ: Self = Self { ord: 0 };
    pub const FINAL_ACTION_DISCARD: Self = Self { ord: 1 };
    pub const FINAL_ACTION_CONTINUE: Self = Self { ord: 2 };
    pub const FINAL_ACTION_MAX: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for FinalAction {
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
unsafe impl sys::GodotFfi for FinalAction {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShaderStage {
    ord: i32,
}
impl ShaderStage {
    pub const SHADER_STAGE_VERTEX: Self = Self { ord: 0 };
    pub const SHADER_STAGE_FRAGMENT: Self = Self { ord: 1 };
    pub const SHADER_STAGE_TESSELATION_CONTROL: Self = Self { ord: 2 };
    pub const SHADER_STAGE_TESSELATION_EVALUATION: Self = Self { ord: 3 };
    pub const SHADER_STAGE_COMPUTE: Self = Self { ord: 4 };
    pub const SHADER_STAGE_MAX: Self = Self { ord: 5 };
    pub const SHADER_STAGE_VERTEX_BIT: Self = Self { ord: 1 };
    pub const SHADER_STAGE_FRAGMENT_BIT: Self = Self { ord: 2 };
    pub const SHADER_STAGE_TESSELATION_CONTROL_BIT: Self = Self { ord: 4 };
    pub const SHADER_STAGE_TESSELATION_EVALUATION_BIT: Self = Self { ord: 8 };
    pub const SHADER_STAGE_COMPUTE_BIT: Self = Self { ord: 16 };
}
impl crate::obj::EngineEnum for ShaderStage {
    fn try_from_ord(ord: i32) -> Option<Self> {
        match ord {
            ord @ 0i32
            | ord @ 1i32
            | ord @ 2i32
            | ord @ 3i32
            | ord @ 4i32
            | ord @ 5i32
            | ord @ 8i32
            | ord @ 16i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for ShaderStage {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ShaderLanguage {
    ord: i32,
}
impl ShaderLanguage {
    pub const SHADER_LANGUAGE_GLSL: Self = Self { ord: 0 };
    pub const SHADER_LANGUAGE_HLSL: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for ShaderLanguage {
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
unsafe impl sys::GodotFfi for ShaderLanguage {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct PipelineSpecializationConstantType {
    ord: i32,
}
impl PipelineSpecializationConstantType {
    pub const PIPELINE_SPECIALIZATION_CONSTANT_TYPE_BOOL: Self = Self { ord: 0 };
    pub const PIPELINE_SPECIALIZATION_CONSTANT_TYPE_INT: Self = Self { ord: 1 };
    pub const PIPELINE_SPECIALIZATION_CONSTANT_TYPE_FLOAT: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for PipelineSpecializationConstantType {
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
unsafe impl sys::GodotFfi for PipelineSpecializationConstantType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Limit {
    ord: i32,
}
impl Limit {
    pub const LIMIT_MAX_BOUND_UNIFORM_SETS: Self = Self { ord: 0 };
    pub const LIMIT_MAX_FRAMEBUFFER_COLOR_ATTACHMENTS: Self = Self { ord: 1 };
    pub const LIMIT_MAX_TEXTURES_PER_UNIFORM_SET: Self = Self { ord: 2 };
    pub const LIMIT_MAX_SAMPLERS_PER_UNIFORM_SET: Self = Self { ord: 3 };
    pub const LIMIT_MAX_STORAGE_BUFFERS_PER_UNIFORM_SET: Self = Self { ord: 4 };
    pub const LIMIT_MAX_STORAGE_IMAGES_PER_UNIFORM_SET: Self = Self { ord: 5 };
    pub const LIMIT_MAX_UNIFORM_BUFFERS_PER_UNIFORM_SET: Self = Self { ord: 6 };
    pub const LIMIT_MAX_DRAW_INDEXED_INDEX: Self = Self { ord: 7 };
    pub const LIMIT_MAX_FRAMEBUFFER_HEIGHT: Self = Self { ord: 8 };
    pub const LIMIT_MAX_FRAMEBUFFER_WIDTH: Self = Self { ord: 9 };
    pub const LIMIT_MAX_TEXTURE_ARRAY_LAYERS: Self = Self { ord: 10 };
    pub const LIMIT_MAX_TEXTURE_SIZE_1D: Self = Self { ord: 11 };
    pub const LIMIT_MAX_TEXTURE_SIZE_2D: Self = Self { ord: 12 };
    pub const LIMIT_MAX_TEXTURE_SIZE_3D: Self = Self { ord: 13 };
    pub const LIMIT_MAX_TEXTURE_SIZE_CUBE: Self = Self { ord: 14 };
    pub const LIMIT_MAX_TEXTURES_PER_SHADER_STAGE: Self = Self { ord: 15 };
    pub const LIMIT_MAX_SAMPLERS_PER_SHADER_STAGE: Self = Self { ord: 16 };
    pub const LIMIT_MAX_STORAGE_BUFFERS_PER_SHADER_STAGE: Self = Self { ord: 17 };
    pub const LIMIT_MAX_STORAGE_IMAGES_PER_SHADER_STAGE: Self = Self { ord: 18 };
    pub const LIMIT_MAX_UNIFORM_BUFFERS_PER_SHADER_STAGE: Self = Self { ord: 19 };
    pub const LIMIT_MAX_PUSH_CONSTANT_SIZE: Self = Self { ord: 20 };
    pub const LIMIT_MAX_UNIFORM_BUFFER_SIZE: Self = Self { ord: 21 };
    pub const LIMIT_MAX_VERTEX_INPUT_ATTRIBUTE_OFFSET: Self = Self { ord: 22 };
    pub const LIMIT_MAX_VERTEX_INPUT_ATTRIBUTES: Self = Self { ord: 23 };
    pub const LIMIT_MAX_VERTEX_INPUT_BINDINGS: Self = Self { ord: 24 };
    pub const LIMIT_MAX_VERTEX_INPUT_BINDING_STRIDE: Self = Self { ord: 25 };
    pub const LIMIT_MIN_UNIFORM_BUFFER_OFFSET_ALIGNMENT: Self = Self { ord: 26 };
    pub const LIMIT_MAX_COMPUTE_SHARED_MEMORY_SIZE: Self = Self { ord: 27 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_X: Self = Self { ord: 28 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Y: Self = Self { ord: 29 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_COUNT_Z: Self = Self { ord: 30 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_INVOCATIONS: Self = Self { ord: 31 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_X: Self = Self { ord: 32 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Y: Self = Self { ord: 33 };
    pub const LIMIT_MAX_COMPUTE_WORKGROUP_SIZE_Z: Self = Self { ord: 34 };
    pub const LIMIT_MAX_VIEWPORT_DIMENSIONS_X: Self = Self { ord: 35 };
    pub const LIMIT_MAX_VIEWPORT_DIMENSIONS_Y: Self = Self { ord: 36 };
}
impl crate::obj::EngineEnum for Limit {
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
            | ord @ 33i32
            | ord @ 34i32
            | ord @ 35i32
            | ord @ 36i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for Limit {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct MemoryType {
    ord: i32,
}
impl MemoryType {
    pub const MEMORY_TEXTURES: Self = Self { ord: 0 };
    pub const MEMORY_BUFFERS: Self = Self { ord: 1 };
    pub const MEMORY_TOTAL: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for MemoryType {
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
unsafe impl sys::GodotFfi for MemoryType {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
