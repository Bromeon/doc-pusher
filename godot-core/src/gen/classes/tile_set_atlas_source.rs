#![doc = "Sidecar module for class [`TileSetAtlasSource`][crate::engine::TileSetAtlasSource].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `TileSetAtlasSource` enums](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `TileSetAtlasSource.`\n\nInherits [`TileSetSource`][crate::engine::TileSetSource].\n\nRelated symbols:\n\n* [`TileSetAtlasSourceVirtual`][crate::engine::TileSetAtlasSourceVirtual]: virtual methods\n\n\nSee also [Godot docs for `TileSetAtlasSource`](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct TileSetAtlasSource {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`TileSetAtlasSource`][crate::engine::TileSetAtlasSource].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `TileSetAtlasSource` methods](https://docs.godotengine.org/en/stable/classes/class_tilesetatlassource.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait TileSetAtlasSourceVirtual:
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
    impl TileSetAtlasSource {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
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
        pub fn set_texture(&mut self, texture: Gd<Texture2D>) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4051416890i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_texture" , 4051416890i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Gd<Texture2D> as AsArg>::as_arg_ptr(&texture)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture(&self) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635182373i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_texture" , 3635182373i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_margins(&mut self, margins: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_margins");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_margins" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&margins)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_margins(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_margins");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_margins" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_separation(&mut self, separation: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_separation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_separation" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&separation)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_separation(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_separation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_separation" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_texture_region_size(&mut self, texture_region_size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_texture_region_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_texture_region_size" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&texture_region_size)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_region_size(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_texture_region_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_texture_region_size" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_use_texture_padding(&mut self, use_texture_padding: bool) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_use_texture_padding");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_use_texture_padding" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&use_texture_padding)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_use_texture_padding(&self) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_use_texture_padding");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    36873697i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_use_texture_padding" , 36873697i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_tile(&mut self, atlas_coords: Vector2i, size: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("create_tile");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1583819816i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "create_tile" , 1583819816i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn remove_tile(&mut self, atlas_coords: Vector2i) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("remove_tile");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1130785943i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "remove_tile" , 1130785943i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn move_tile_in_atlas(
            &mut self,
            atlas_coords: Vector2i,
            new_atlas_coords: Vector2i,
            new_size: Vector2i,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("move_tile_in_atlas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1375626516i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "move_tile_in_atlas" , 1375626516i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <Vector2i as sys::GodotFfi>::sys_const(&new_atlas_coords),
                    <Vector2i as sys::GodotFfi>::sys_const(&new_size),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_size_in_atlas(&self, atlas_coords: Vector2i) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_size_in_atlas");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3050897911i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_size_in_atlas" , 3050897911i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn has_room_for_tile(
            &self,
            atlas_coords: Vector2i,
            size: Vector2i,
            animation_columns: i64,
            animation_separation: Vector2i,
            frames_count: i64,
            ignored_tile: Vector2i,
        ) -> bool {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("has_room_for_tile");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4182444377i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "has_room_for_tile" , 4182444377i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <Vector2i as sys::GodotFfi>::sys_const(&size),
                    <i64 as sys::GodotFfi>::sys_const(&animation_columns),
                    <Vector2i as sys::GodotFfi>::sys_const(&animation_separation),
                    <i64 as sys::GodotFfi>::sys_const(&frames_count),
                    <Vector2i as sys::GodotFfi>::sys_const(&ignored_tile),
                ];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_tiles_to_be_removed_on_change(
            &mut self,
            texture: Gd<Texture2D>,
            margins: Vector2i,
            separation: Vector2i,
            texture_region_size: Vector2i,
        ) -> PackedVector2Array {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tiles_to_be_removed_on_change");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1240378054i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tiles_to_be_removed_on_change" , 1240378054i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Gd<Texture2D> as AsArg>::as_arg_ptr(&texture),
                    <Vector2i as sys::GodotFfi>::sys_const(&margins),
                    <Vector2i as sys::GodotFfi>::sys_const(&separation),
                    <Vector2i as sys::GodotFfi>::sys_const(&texture_region_size),
                ];
                let __args_ptr = __args.as_ptr();
                <PackedVector2Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_tile_at_coords(&self, atlas_coords: Vector2i) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_at_coords");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3050897911i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_at_coords" , 3050897911i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_animation_columns(&mut self, atlas_coords: Vector2i, frame_columns: i64) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_tile_animation_columns");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3200960707i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_tile_animation_columns" , 3200960707i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&frame_columns),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_animation_columns(&self, atlas_coords: Vector2i) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_animation_columns");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2485466453i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_animation_columns" , 2485466453i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_animation_separation(
            &mut self,
            atlas_coords: Vector2i,
            separation: Vector2i,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_tile_animation_separation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1941061099i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_tile_animation_separation" , 1941061099i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <Vector2i as sys::GodotFfi>::sys_const(&separation),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_animation_separation(&self, atlas_coords: Vector2i) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_animation_separation");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3050897911i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_animation_separation" , 3050897911i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_animation_speed(&mut self, atlas_coords: Vector2i, speed: f64) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_tile_animation_speed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2262553149i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_tile_animation_speed" , 2262553149i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <f64 as sys::GodotFfi>::sys_const(&speed),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_animation_speed(&self, atlas_coords: Vector2i) -> f64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_animation_speed");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    719993801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_animation_speed" , 719993801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_animation_frames_count(
            &mut self,
            atlas_coords: Vector2i,
            frames_count: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_tile_animation_frames_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3200960707i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_tile_animation_frames_count" , 3200960707i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&frames_count),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_animation_frames_count(&self, atlas_coords: Vector2i) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_animation_frames_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2485466453i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_animation_frames_count" , 2485466453i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_tile_animation_frame_duration(
            &mut self,
            atlas_coords: Vector2i,
            frame_index: i64,
            duration: f64,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_tile_animation_frame_duration");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2843487787i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_tile_animation_frame_duration" , 2843487787i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&frame_index),
                    <f64 as sys::GodotFfi>::sys_const(&duration),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_tile_animation_frame_duration(
            &self,
            atlas_coords: Vector2i,
            frame_index: i64,
        ) -> f64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_animation_frame_duration");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1802448425i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_animation_frame_duration" , 1802448425i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&frame_index),
                ];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_tile_animation_total_duration(&self, atlas_coords: Vector2i) -> f64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_animation_total_duration");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    719993801i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_animation_total_duration" , 719993801i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn create_alternative_tile(
            &mut self,
            atlas_coords: Vector2i,
            alternative_id_override: i64,
        ) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("create_alternative_tile");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3531100812i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "create_alternative_tile" , 3531100812i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_id_override),
                ];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn remove_alternative_tile(&mut self, atlas_coords: Vector2i, alternative_tile: i64) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("remove_alternative_tile");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3200960707i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "remove_alternative_tile" , 3200960707i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_tile),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn set_alternative_tile_id(
            &mut self,
            atlas_coords: Vector2i,
            alternative_tile: i64,
            new_id: i64,
        ) {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("set_alternative_tile_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1499785778i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "set_alternative_tile_id" , 1499785778i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_tile),
                    <i64 as sys::GodotFfi>::sys_const(&new_id),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_next_alternative_tile_id(&self, atlas_coords: Vector2i) -> i64 {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_next_alternative_tile_id");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2485466453i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_next_alternative_tile_id" , 2485466453i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Vector2i as sys::GodotFfi>::sys_const(&atlas_coords)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_tile_data(
            &self,
            atlas_coords: Vector2i,
            alternative_tile: i64,
        ) -> Option<Gd<TileData>> {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3534028207i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_data" , 3534028207i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&alternative_tile),
                ];
                let __args_ptr = __args.as_ptr();
                <Gd<TileData>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_atlas_grid_size(&self) -> Vector2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_atlas_grid_size");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3690982128i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_atlas_grid_size" , 3690982128i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Vector2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_tile_texture_region(&self, atlas_coords: Vector2i, frame: i64) -> Rect2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_tile_texture_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1321423751i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_tile_texture_region" , 1321423751i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&frame),
                ];
                let __args_ptr = __args.as_ptr();
                <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_runtime_texture(&self) -> Option<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_runtime_texture");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3635182373i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_runtime_texture" , 3635182373i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Gd<Texture2D>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_runtime_tile_texture_region(
            &self,
            atlas_coords: Vector2i,
            frame: i64,
        ) -> Rect2i {
            unsafe {
                let __class_name = StringName::from("TileSetAtlasSource");
                let __method_name = StringName::from("get_runtime_tile_texture_region");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    104874263i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "TileSetAtlasSource" , "get_runtime_tile_texture_region" , 104874263i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <Vector2i as sys::GodotFfi>::sys_const(&atlas_coords),
                    <i64 as sys::GodotFfi>::sys_const(&frame),
                ];
                let __args_ptr = __args.as_ptr();
                <Rect2i as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for TileSetAtlasSource {
        type Base = crate::engine::TileSetSource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "TileSetAtlasSource";
    }
    impl crate::obj::EngineClass for TileSetAtlasSource {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::TileSetSource> for TileSetAtlasSource {}
    impl crate::obj::Inherits<crate::engine::Resource> for TileSetAtlasSource {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for TileSetAtlasSource {}
    impl crate::obj::Inherits<crate::engine::Object> for TileSetAtlasSource {}
    impl std::ops::Deref for TileSetAtlasSource {
        type Target = crate::engine::TileSetSource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for TileSetAtlasSource {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_TileSetAtlasSource {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::TileSetAtlasSource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::TileSetSource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
