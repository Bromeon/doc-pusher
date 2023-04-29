#![doc = "Sidecar module for class [`TileSet`][crate::engine::TileSet].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSet` enums](https://docs.godotengine.org/en/stable/classes/class_tileset.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `TileSet.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`tile_set`][crate::engine::tile_set]: sidecar module with related enum/flag types\n* [`TileSetVirtual`][crate::engine::TileSetVirtual]: virtual methods\n\n\nSee also [Godot docs for `TileSet`](https://docs.godotengine.org/en/stable/classes/class_tileset.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct TileSet {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`TileSet`][crate::engine::TileSet].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSet` methods](https://docs.godotengine.org/en/stable/classes/class_tileset.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait TileSetVirtual:
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
    impl TileSet {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("TileSet");
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
        pub fn get_next_source_id(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_next_source_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_next_source_id" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_source(
            &mut self,
            source: Gd<TileSetSource>,
            atlas_source_id_override: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    276991387i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_source" , 276991387i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<TileSetSource> as AsArg>::as_arg_ptr(&source),
                    <i64 as sys::GodotFfi>::sys_const(&atlas_source_id_override),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_source(&mut self, source_id: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_source" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&source_id)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_source_id(&mut self, source_id: i64, new_source_id: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_source_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_source_id" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_id),
                    <i64 as sys::GodotFfi>::sys_const(&new_source_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_source_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_source_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_source_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_source_id(&self, index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_source_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_source_id" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_source(&self, source_id: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("has_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "has_source" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&source_id)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_source(&self, source_id: i64) -> Option<Gd<TileSetSource>> {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_source");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1763540252i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_source" , 1763540252i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&source_id)];
                let __args_ptr = __args.as_ptr();
                <Gd<TileSetSource>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_shape(&mut self, shape: tile_set::TileShape) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_tile_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2131427112i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_tile_shape" , 2131427112i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tile_set::TileShape as sys::GodotFfi>::sys_const(&shape)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_shape(&self) -> tile_set::TileShape {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_tile_shape");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    716918169i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_tile_shape" , 716918169i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <tile_set::TileShape as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_layout(&mut self, layout: tile_set::TileLayout) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_tile_layout");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1071216679i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_tile_layout" , 1071216679i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tile_set::TileLayout as sys::GodotFfi>::sys_const(&layout)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_layout(&self) -> tile_set::TileLayout {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_tile_layout");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    194628839i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_tile_layout" , 194628839i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <tile_set::TileLayout as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_offset_axis(&mut self, alignment: tile_set::TileOffsetAxis) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_tile_offset_axis");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3300198521i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_tile_offset_axis" , 3300198521i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<tile_set::TileOffsetAxis as sys::GodotFfi>::sys_const(
                    &alignment,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_offset_axis(&self) -> tile_set::TileOffsetAxis {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_tile_offset_axis");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    762494114i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_tile_offset_axis" , 762494114i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <tile_set::TileOffsetAxis as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_size(&mut self, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_tile_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_tile_size" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_size(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_tile_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_tile_size" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_uv_clipping(&mut self, uv_clipping: bool) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_uv_clipping");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_uv_clipping" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&uv_clipping)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn is_uv_clipping(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("is_uv_clipping");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "is_uv_clipping" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_occlusion_layers_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_occlusion_layers_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_occlusion_layers_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_occlusion_layer(&mut self, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_occlusion_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1025054187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_occlusion_layer" , 1025054187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&to_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_occlusion_layer(&mut self, layer_index: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("move_occlusion_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "move_occlusion_layer" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_occlusion_layer(&mut self, layer_index: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_occlusion_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_occlusion_layer" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_occlusion_layer_light_mask(&mut self, layer_index: i64, light_mask: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_occlusion_layer_light_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_occlusion_layer_light_mask" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&light_mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_occlusion_layer_light_mask(&self, layer_index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_occlusion_layer_light_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_occlusion_layer_light_mask" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_occlusion_layer_sdf_collision(&mut self, layer_index: i64, sdf_collision: bool) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_occlusion_layer_sdf_collision");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    300928843i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_occlusion_layer_sdf_collision" , 300928843i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <bool as sys::GodotFfi>::sys_const(&sdf_collision),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_occlusion_layer_sdf_collision(&self, layer_index: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_occlusion_layer_sdf_collision");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_occlusion_layer_sdf_collision" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_physics_layers_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_physics_layers_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_physics_layers_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_physics_layer(&mut self, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_physics_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1025054187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_physics_layer" , 1025054187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&to_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_physics_layer(&mut self, layer_index: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("move_physics_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "move_physics_layer" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_physics_layer(&mut self, layer_index: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_physics_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_physics_layer" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_physics_layer_collision_layer(&mut self, layer_index: i64, layer: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_physics_layer_collision_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_physics_layer_collision_layer" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&layer),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_physics_layer_collision_layer(&self, layer_index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_physics_layer_collision_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_physics_layer_collision_layer" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_physics_layer_collision_mask(&mut self, layer_index: i64, mask: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_physics_layer_collision_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_physics_layer_collision_mask" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&mask),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_physics_layer_collision_mask(&self, layer_index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_physics_layer_collision_mask");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_physics_layer_collision_mask" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_physics_layer_physics_material(
            &mut self,
            layer_index: i64,
            physics_material: Gd<PhysicsMaterial>,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_physics_layer_physics_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1018687357i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_physics_layer_physics_material" , 1018687357i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <Gd<PhysicsMaterial> as AsArg>::as_arg_ptr(&physics_material),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_physics_layer_physics_material(
            &self,
            layer_index: i64,
        ) -> Option<Gd<PhysicsMaterial>> {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_physics_layer_physics_material");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    788318639i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_physics_layer_physics_material" , 788318639i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <Gd<PhysicsMaterial>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_terrain_sets_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_terrain_sets_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_terrain_sets_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_terrain_set(&mut self, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_terrain_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1025054187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_terrain_set" , 1025054187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&to_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_terrain_set(&mut self, terrain_set: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("move_terrain_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "move_terrain_set" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_terrain_set(&mut self, terrain_set: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_terrain_set");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_terrain_set" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&terrain_set)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_terrain_set_mode(&mut self, terrain_set: i64, mode: tile_set::TerrainMode) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_terrain_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3943003916i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_terrain_set_mode" , 3943003916i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <tile_set::TerrainMode as sys::GodotFfi>::sys_const(&mode),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_terrain_set_mode(&self, terrain_set: i64) -> tile_set::TerrainMode {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_terrain_set_mode");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2084469411i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_terrain_set_mode" , 2084469411i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&terrain_set)];
                let __args_ptr = __args.as_ptr();
                <tile_set::TerrainMode as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_terrains_count(&self, terrain_set: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_terrains_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_terrains_count" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&terrain_set)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_terrain(&mut self, terrain_set: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_terrain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3023605688i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_terrain" , 3023605688i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_terrain(&mut self, terrain_set: i64, terrain_index: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("move_terrain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1649997291i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "move_terrain" , 1649997291i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&terrain_index),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_terrain(&mut self, terrain_set: i64, terrain_index: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_terrain");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_terrain" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&terrain_index),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_terrain_name(
            &mut self,
            terrain_set: i64,
            terrain_index: i64,
            name: GodotString,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_terrain_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2285447957i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_terrain_name" , 2285447957i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&terrain_index),
                    <GodotString as sys::GodotFfi>::sys_const(&name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_terrain_name(&self, terrain_set: i64, terrain_index: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_terrain_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1391810591i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_terrain_name" , 1391810591i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&terrain_index),
                ];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_terrain_color(&mut self, terrain_set: i64, terrain_index: i64, color: Color) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_terrain_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3733378741i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_terrain_color" , 3733378741i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&terrain_index),
                    <Color as sys::GodotFfi>::sys_const(&color),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_terrain_color(&self, terrain_set: i64, terrain_index: i64) -> Color {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_terrain_color");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2165839948i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_terrain_color" , 2165839948i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&terrain_set),
                    <i64 as sys::GodotFfi>::sys_const(&terrain_index),
                ];
                let __args_ptr = __args.as_ptr();
                <Color as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_navigation_layers_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_navigation_layers_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_navigation_layers_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_navigation_layer(&mut self, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_navigation_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1025054187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_navigation_layer" , 1025054187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&to_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_navigation_layer(&mut self, layer_index: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("move_navigation_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "move_navigation_layer" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_navigation_layer(&mut self, layer_index: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_navigation_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_navigation_layer" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_navigation_layer_layers(&mut self, layer_index: i64, layers: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_navigation_layer_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_navigation_layer_layers" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&layers),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_navigation_layer_layers(&self, layer_index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_navigation_layer_layers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    923996154i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_navigation_layer_layers" , 923996154i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_navigation_layer_layer_value(
            &mut self,
            layer_index: i64,
            layer_number: i64,
            value: bool,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_navigation_layer_layer_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1383440665i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_navigation_layer_layer_value" , 1383440665i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&layer_number),
                    <bool as sys::GodotFfi>::sys_const(&value),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_navigation_layer_layer_value(
            &self,
            layer_index: i64,
            layer_number: i64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_navigation_layer_layer_value");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2522259332i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_navigation_layer_layer_value" , 2522259332i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&layer_number),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_custom_data_layers_count(&self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_custom_data_layers_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3905245786i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_custom_data_layers_count" , 3905245786i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn add_custom_data_layer(&mut self, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_custom_data_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1025054187i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_custom_data_layer" , 1025054187i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&to_position)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_custom_data_layer(&mut self, layer_index: i64, to_position: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("move_custom_data_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "move_custom_data_layer" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <i64 as sys::GodotFfi>::sys_const(&to_position),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_custom_data_layer(&mut self, layer_index: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_custom_data_layer");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_custom_data_layer" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_custom_data_layer_by_name(&self, layer_name: GodotString) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_custom_data_layer_by_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321353865i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_custom_data_layer_by_name" , 1321353865i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&layer_name)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_custom_data_layer_name(&mut self, layer_index: i64, layer_name: GodotString) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_custom_data_layer_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    501894301i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_custom_data_layer_name" , 501894301i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <GodotString as sys::GodotFfi>::sys_const(&layer_name),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_custom_data_layer_name(&self, layer_index: i64) -> GodotString {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_custom_data_layer_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    844755477i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_custom_data_layer_name" , 844755477i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_custom_data_layer_type(&mut self, layer_index: i64, layer_type: VariantType) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_custom_data_layer_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3492912874i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_custom_data_layer_type" , 3492912874i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&layer_index),
                    <VariantType as sys::GodotFfi>::sys_const(&layer_type),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_custom_data_layer_type(&self, layer_index: i64) -> VariantType {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_custom_data_layer_type");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2990820875i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_custom_data_layer_type" , 2990820875i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&layer_index)];
                let __args_ptr = __args.as_ptr();
                <VariantType as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_source_level_tile_proxy(&mut self, source_from: i64, source_to: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_source_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3937882851i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_source_level_tile_proxy" , 3937882851i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <i64 as sys::GodotFfi>::sys_const(&source_to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_source_level_tile_proxy(&mut self, source_from: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_source_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3744713108i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_source_level_tile_proxy" , 3744713108i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&source_from)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_source_level_tile_proxy(&mut self, source_from: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("has_source_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3067735520i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "has_source_level_tile_proxy" , 3067735520i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&source_from)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_source_level_tile_proxy(&mut self, source_from: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_source_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_source_level_tile_proxy" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&source_from)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_coords_level_tile_proxy(
            &mut self,
            p_source_from: i64,
            coords_from: Vector2i,
            source_to: i64,
            coords_to: Vector2i,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_coords_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1769939278i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_coords_level_tile_proxy" , 1769939278i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&p_source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                    <i64 as sys::GodotFfi>::sys_const(&source_to),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_coords_level_tile_proxy(
            &mut self,
            source_from: i64,
            coords_from: Vector2i,
        ) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_coords_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2856536371i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_coords_level_tile_proxy" , 2856536371i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                ];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_coords_level_tile_proxy(
            &mut self,
            source_from: i64,
            coords_from: Vector2i,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("has_coords_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3957903770i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "has_coords_level_tile_proxy" , 3957903770i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_coords_level_tile_proxy(&mut self, source_from: i64, coords_from: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_coords_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2311374912i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_coords_level_tile_proxy" , 2311374912i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_alternative_level_tile_proxy(
            &mut self,
            source_from: i64,
            coords_from: Vector2i,
            alternative_from: i64,
            source_to: i64,
            coords_to: Vector2i,
            alternative_to: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("set_alternative_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3862385460i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "set_alternative_level_tile_proxy" , 3862385460i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_from),
                    <i64 as sys::GodotFfi>::sys_const(&source_to),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_to),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_to),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_alternative_level_tile_proxy(
            &mut self,
            source_from: i64,
            coords_from: Vector2i,
            alternative_from: i64,
        ) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_alternative_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2303761075i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_alternative_level_tile_proxy" , 2303761075i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_from),
                ];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_alternative_level_tile_proxy(
            &mut self,
            source_from: i64,
            coords_from: Vector2i,
            alternative_from: i64,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("has_alternative_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    180086755i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "has_alternative_level_tile_proxy" , 180086755i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_from),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_alternative_level_tile_proxy(
            &mut self,
            source_from: i64,
            coords_from: Vector2i,
            alternative_from: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_alternative_level_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2328951467i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_alternative_level_tile_proxy" , 2328951467i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_from),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn map_tile_proxy(
            &self,
            source_from: i64,
            coords_from: Vector2i,
            alternative_from: i64,
        ) -> VariantArray {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("map_tile_proxy");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4267935328i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "map_tile_proxy" , 4267935328i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <i64 as sys::GodotFfi>::sys_const(&source_from),
                    <Vector2i as sys::GodotFfi>::sys_const(&coords_from),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_from),
                ];
                let __args_ptr = __args.as_ptr();
                <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn cleanup_invalid_tile_proxies(&mut self) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("cleanup_invalid_tile_proxies");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "cleanup_invalid_tile_proxies" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn clear_tile_proxies(&mut self) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("clear_tile_proxies");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3218959716i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "clear_tile_proxies" , 3218959716i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn add_pattern(&mut self, pattern: Gd<TileMapPattern>, index: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("add_pattern");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3009264082i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "add_pattern" , 3009264082i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<TileMapPattern> as AsArg>::as_arg_ptr(&pattern),
                    <i64 as sys::GodotFfi>::sys_const(&index),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_pattern(&mut self, index: i64) -> Option<Gd<TileMapPattern>> {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_pattern");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4207737510i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_pattern" , 4207737510i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                <Gd<TileMapPattern>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_pattern(&mut self, index: i64) {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("remove_pattern");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "remove_pattern" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&index)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_patterns_count(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSet");
                let __method_name = StringName::from("get_patterns_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSet" , "get_patterns_count" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for TileSet {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "TileSet";
    }
    impl crate::obj::EngineClass for TileSet {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for TileSet {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for TileSet {}
    impl crate::obj::Inherits<crate::engine::Object> for TileSet {}
    impl std::ops::Deref for TileSet {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for TileSet {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_TileSet {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::TileSet> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TileShape {
    ord: i32,
}
impl TileShape {
    pub const TILE_SHAPE_SQUARE: Self = Self { ord: 0 };
    pub const TILE_SHAPE_ISOMETRIC: Self = Self { ord: 1 };
    pub const TILE_SHAPE_HALF_OFFSET_SQUARE: Self = Self { ord: 2 };
    pub const TILE_SHAPE_HEXAGON: Self = Self { ord: 3 };
}
impl crate::obj::EngineEnum for TileShape {
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
unsafe impl sys::GodotFfi for TileShape {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TileLayout {
    ord: i32,
}
impl TileLayout {
    pub const TILE_LAYOUT_STACKED: Self = Self { ord: 0 };
    pub const TILE_LAYOUT_STACKED_OFFSET: Self = Self { ord: 1 };
    pub const TILE_LAYOUT_STAIRS_RIGHT: Self = Self { ord: 2 };
    pub const TILE_LAYOUT_STAIRS_DOWN: Self = Self { ord: 3 };
    pub const TILE_LAYOUT_DIAMOND_RIGHT: Self = Self { ord: 4 };
    pub const TILE_LAYOUT_DIAMOND_DOWN: Self = Self { ord: 5 };
}
impl crate::obj::EngineEnum for TileLayout {
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
unsafe impl sys::GodotFfi for TileLayout {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TileOffsetAxis {
    ord: i32,
}
impl TileOffsetAxis {
    pub const TILE_OFFSET_AXIS_HORIZONTAL: Self = Self { ord: 0 };
    pub const TILE_OFFSET_AXIS_VERTICAL: Self = Self { ord: 1 };
}
impl crate::obj::EngineEnum for TileOffsetAxis {
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
unsafe impl sys::GodotFfi for TileOffsetAxis {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct CellNeighbor {
    ord: i32,
}
impl CellNeighbor {
    pub const CELL_NEIGHBOR_RIGHT_SIDE: Self = Self { ord: 0 };
    pub const CELL_NEIGHBOR_RIGHT_CORNER: Self = Self { ord: 1 };
    pub const CELL_NEIGHBOR_BOTTOM_RIGHT_SIDE: Self = Self { ord: 2 };
    pub const CELL_NEIGHBOR_BOTTOM_RIGHT_CORNER: Self = Self { ord: 3 };
    pub const CELL_NEIGHBOR_BOTTOM_SIDE: Self = Self { ord: 4 };
    pub const CELL_NEIGHBOR_BOTTOM_CORNER: Self = Self { ord: 5 };
    pub const CELL_NEIGHBOR_BOTTOM_LEFT_SIDE: Self = Self { ord: 6 };
    pub const CELL_NEIGHBOR_BOTTOM_LEFT_CORNER: Self = Self { ord: 7 };
    pub const CELL_NEIGHBOR_LEFT_SIDE: Self = Self { ord: 8 };
    pub const CELL_NEIGHBOR_LEFT_CORNER: Self = Self { ord: 9 };
    pub const CELL_NEIGHBOR_TOP_LEFT_SIDE: Self = Self { ord: 10 };
    pub const CELL_NEIGHBOR_TOP_LEFT_CORNER: Self = Self { ord: 11 };
    pub const CELL_NEIGHBOR_TOP_SIDE: Self = Self { ord: 12 };
    pub const CELL_NEIGHBOR_TOP_CORNER: Self = Self { ord: 13 };
    pub const CELL_NEIGHBOR_TOP_RIGHT_SIDE: Self = Self { ord: 14 };
    pub const CELL_NEIGHBOR_TOP_RIGHT_CORNER: Self = Self { ord: 15 };
}
impl crate::obj::EngineEnum for CellNeighbor {
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
            | ord @ 15i32 => Some(Self { ord }),
            _ => None,
        }
    }
    fn ord(self) -> i32 {
        self.ord
    }
}
unsafe impl sys::GodotFfi for CellNeighbor {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TerrainMode {
    ord: i32,
}
impl TerrainMode {
    pub const TERRAIN_MODE_MATCH_CORNERS_AND_SIDES: Self = Self { ord: 0 };
    pub const TERRAIN_MODE_MATCH_CORNERS: Self = Self { ord: 1 };
    pub const TERRAIN_MODE_MATCH_SIDES: Self = Self { ord: 2 };
}
impl crate::obj::EngineEnum for TerrainMode {
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
unsafe impl sys::GodotFfi for TerrainMode {
    sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
}
