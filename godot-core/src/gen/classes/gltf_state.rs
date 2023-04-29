#![doc = "Sidecar module for class [`GltfState`][crate::engine::GltfState].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `GLTFState` enums](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `GLTFState.`\n\nInherits [`Resource`][crate::engine::Resource].\n\nRelated symbols:\n\n* [`GltfStateVirtual`][crate::engine::GltfStateVirtual]: virtual methods\n\n\nSee also [Godot docs for `GLTFState`](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct GltfState {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`GltfState`][crate::engine::GltfState].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `GLTFState` methods](https://docs.godotengine.org/en/stable/classes/class_gltfstate.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait GltfStateVirtual:
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
    impl GltfState {
        pub fn new() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
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
        pub fn add_used_extension(&mut self, extension_name: GodotString, required: bool) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("add_used_extension");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2678287736i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "add_used_extension" , 2678287736i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <GodotString as sys::GodotFfi>::sys_const(&extension_name),
                    <bool as sys::GodotFfi>::sys_const(&required),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_json(&mut self) -> Dictionary {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_json");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2382534195i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_json" , 2382534195i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Dictionary as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_json(&mut self, json: Dictionary) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_json");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155329257i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_json" , 4155329257i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Dictionary as sys::GodotFfi>::sys_const(&json)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_major_version(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_major_version");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_major_version" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_major_version(&mut self, major_version: i64) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_major_version");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_major_version" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&major_version)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_minor_version(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_minor_version");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_minor_version" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_minor_version(&mut self, minor_version: i64) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_minor_version");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_minor_version" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&minor_version)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_glb_data(&mut self) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_glb_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2115431945i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_glb_data" , 2115431945i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_glb_data(&mut self, glb_data: PackedByteArray) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_glb_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2971499966i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_glb_data" , 2971499966i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&glb_data)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_use_named_skin_binds(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_use_named_skin_binds");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_use_named_skin_binds" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_use_named_skin_binds(&mut self, use_named_skin_binds: bool) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_use_named_skin_binds");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_use_named_skin_binds" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&use_named_skin_binds)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_nodes(&mut self) -> Array<Gd<GltfNode>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_nodes" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfNode>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_nodes(&mut self, nodes: Array<Gd<GltfNode>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_nodes" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfNode>> as sys::GodotFfi>::sys_const(&nodes)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_buffers(&mut self) -> PackedByteArray {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_buffers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_buffers" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_buffers(&mut self, buffers: PackedByteArray) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_buffers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_buffers" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&buffers)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_buffer_views(&mut self) -> Array<Gd<GltfBufferView>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_buffer_views");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_buffer_views" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfBufferView>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_buffer_views(&mut self, buffer_views: Array<Gd<GltfBufferView>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_buffer_views");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_buffer_views" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfBufferView>> as sys::GodotFfi>::sys_const(
                    &buffer_views,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_accessors(&mut self) -> Array<Gd<GltfAccessor>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_accessors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_accessors" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfAccessor>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_accessors(&mut self, accessors: Array<Gd<GltfAccessor>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_accessors");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_accessors" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfAccessor>> as sys::GodotFfi>::sys_const(
                    &accessors,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_meshes(&mut self) -> Array<Gd<GltfMesh>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_meshes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_meshes" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfMesh>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_meshes(&mut self, meshes: Array<Gd<GltfMesh>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_meshes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_meshes" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfMesh>> as sys::GodotFfi>::sys_const(&meshes)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_animation_players_count(&mut self, idx: i64) -> i64 {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_animation_players_count");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3744713108i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_animation_players_count" , 3744713108i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_animation_player(&mut self, idx: i64) -> Option<Gd<AnimationPlayer>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_animation_player");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    925043400i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_animation_player" , 925043400i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <Gd<AnimationPlayer>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_materials(&mut self) -> Array<Gd<Material>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_materials");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_materials" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Material>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_materials(&mut self, materials: Array<Gd<Material>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_materials");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_materials" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<Material>> as sys::GodotFfi>::sys_const(
                    &materials,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_scene_name(&mut self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_scene_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2841200299i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_scene_name" , 2841200299i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_scene_name(&mut self, scene_name: GodotString) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_scene_name");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_scene_name" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&scene_name)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_base_path(&mut self) -> GodotString {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_base_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2841200299i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_base_path" , 2841200299i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_base_path(&mut self, base_path: GodotString) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_base_path");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    83702148i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_base_path" , 83702148i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<GodotString as sys::GodotFfi>::sys_const(&base_path)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_root_nodes(&mut self) -> PackedInt32Array {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_root_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    969006518i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_root_nodes" , 969006518i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <PackedInt32Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_root_nodes(&mut self, root_nodes: PackedInt32Array) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_root_nodes");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3614634198i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_root_nodes" , 3614634198i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<PackedInt32Array as sys::GodotFfi>::sys_const(&root_nodes)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_textures(&mut self) -> Array<Gd<GltfTexture>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_textures");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_textures" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfTexture>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_textures(&mut self, textures: Array<Gd<GltfTexture>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_textures");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_textures" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfTexture>> as sys::GodotFfi>::sys_const(
                    &textures,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_texture_samplers(&mut self) -> Array<Gd<GltfTextureSampler>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_texture_samplers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_texture_samplers" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfTextureSampler>> as sys::GodotFfi>::from_sys_init_default(
                    |return_ptr| {
                        __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                    },
                )
            }
        }
        pub fn set_texture_samplers(&mut self, texture_samplers: Array<Gd<GltfTextureSampler>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_texture_samplers");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_texture_samplers" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfTextureSampler>> as sys::GodotFfi>::sys_const(
                    &texture_samplers,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_images(&mut self) -> Array<Gd<Texture2D>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_images");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_images" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<Texture2D>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_images(&mut self, images: Array<Gd<Texture2D>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_images");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_images" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<Texture2D>> as sys::GodotFfi>::sys_const(&images)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_skins(&mut self) -> Array<Gd<GltfSkin>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_skins");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_skins" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfSkin>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_skins(&mut self, skins: Array<Gd<GltfSkin>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_skins");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_skins" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfSkin>> as sys::GodotFfi>::sys_const(&skins)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_cameras(&mut self) -> Array<Gd<GltfCamera>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_cameras");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_cameras" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfCamera>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_cameras(&mut self, cameras: Array<Gd<GltfCamera>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_cameras");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_cameras" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfCamera>> as sys::GodotFfi>::sys_const(
                    &cameras,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_lights(&mut self) -> Array<Gd<GltfLight>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_lights");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_lights" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfLight>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_lights(&mut self, lights: Array<Gd<GltfLight>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_lights");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_lights" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfLight>> as sys::GodotFfi>::sys_const(&lights)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_unique_names(&mut self) -> Array<GodotString> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_unique_names");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_unique_names" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<GodotString> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_unique_names(&mut self, unique_names: Array<GodotString>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_unique_names");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_unique_names" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<GodotString> as sys::GodotFfi>::sys_const(
                    &unique_names,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_unique_animation_names(&mut self) -> Array<GodotString> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_unique_animation_names");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_unique_animation_names" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<GodotString> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_unique_animation_names(&mut self, unique_animation_names: Array<GodotString>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_unique_animation_names");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_unique_animation_names" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<GodotString> as sys::GodotFfi>::sys_const(
                    &unique_animation_names,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_skeletons(&mut self) -> Array<Gd<GltfSkeleton>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_skeletons");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_skeletons" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfSkeleton>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_skeletons(&mut self, skeletons: Array<Gd<GltfSkeleton>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_skeletons");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_skeletons" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfSkeleton>> as sys::GodotFfi>::sys_const(
                    &skeletons,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_create_animations(&mut self) -> bool {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_create_animations");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2240911060i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_create_animations" , 2240911060i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_create_animations(&mut self, create_animations: bool) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_create_animations");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2586408642i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_create_animations" , 2586408642i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<bool as sys::GodotFfi>::sys_const(&create_animations)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_animations(&mut self) -> Array<Gd<GltfAnimation>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_animations");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2915620761i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_animations" , 2915620761i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <Array<Gd<GltfAnimation>> as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_animations(&mut self, animations: Array<Gd<GltfAnimation>>) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_animations");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    381264803i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_animations" , 381264803i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Array<Gd<GltfAnimation>> as sys::GodotFfi>::sys_const(
                    &animations,
                )];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_scene_node(&mut self, idx: i64) -> Option<Gd<Node>> {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_scene_node");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4253421667i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_scene_node" , 4253421667i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&idx)];
                let __args_ptr = __args.as_ptr();
                <Gd<Node>>::from_sys_init_opt(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn get_additional_data(&mut self, extension_name: StringName) -> Variant {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_additional_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2138907829i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_additional_data" , 2138907829i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<StringName as sys::GodotFfi>::sys_const(&extension_name)];
                let __args_ptr = __args.as_ptr();
                <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_additional_data(
            &mut self,
            extension_name: StringName,
            additional_data: Variant,
        ) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_additional_data");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    3776071444i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_additional_data" , 3776071444i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [
                    <StringName as sys::GodotFfi>::sys_const(&extension_name),
                    <Variant as sys::GodotFfi>::sys_const(&additional_data),
                ];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub fn get_handle_binary_image(&mut self) -> i64 {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("get_handle_binary_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    2455072627i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "get_handle_binary_image" , 2455072627i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [];
                let __args_ptr = __args.as_ptr();
                <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn set_handle_binary_image(&mut self, method: i64) {
            unsafe {
                let __class_name = StringName::from("GLTFState");
                let __method_name = StringName::from("set_handle_binary_image");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1286410249i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "GLTFState" , "set_handle_binary_image" , 1286410249i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&method)];
                let __args_ptr = __args.as_ptr();
                let return_ptr = std::ptr::null_mut();
                __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
            }
        }
        pub const HANDLE_BINARY_DISCARD_TEXTURES: i32 = 0i32;
        pub const HANDLE_BINARY_EXTRACT_TEXTURES: i32 = 1i32;
        pub const HANDLE_BINARY_EMBED_AS_BASISU: i32 = 2i32;
        pub const HANDLE_BINARY_EMBED_AS_UNCOMPRESSED: i32 = 3i32;
    }
    impl crate::obj::GodotClass for GltfState {
        type Base = crate::engine::Resource;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::StaticRefCount;
        const CLASS_NAME: &'static str = "GLTFState";
    }
    impl crate::obj::EngineClass for GltfState {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::Resource> for GltfState {}
    impl crate::obj::Inherits<crate::engine::RefCounted> for GltfState {}
    impl crate::obj::Inherits<crate::engine::Object> for GltfState {}
    impl std::ops::Deref for GltfState {
        type Target = crate::engine::Resource;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for GltfState {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_GltfState {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::GltfState> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Resource> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::RefCounted> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
