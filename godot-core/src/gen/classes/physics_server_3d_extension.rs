#![doc = "Sidecar module for class [`PhysicsServer3DExtension`][crate::engine::PhysicsServer3DExtension].\n\nDefines related flag and enum types. In GDScript, those are nested under the class scope.\n\nSee also [Godot docs for `PhysicsServer3DExtension` enums](https://docs.godotengine.org/en/stable/classes/class_physicsserver3dextension.html#enumerations).\n\n"]
use crate::builtin::*;
use crate::engine::notify::*;
use crate::engine::*;
use crate::obj::{AsArg, Gd};
use godot_ffi as sys;
use sys::GodotFfi as _;
pub(super) mod re_export {
    use super::*;
    #[doc = "Godot class `PhysicsServer3DExtension.`\n\nInherits [`PhysicsServer3D`][crate::engine::PhysicsServer3D].\n\nRelated symbols:\n\n* [`PhysicsServer3DExtensionVirtual`][crate::engine::PhysicsServer3DExtensionVirtual]: virtual methods\n\n\nSee also [Godot docs for `PhysicsServer3DExtension`](https://docs.godotengine.org/en/stable/classes/class_physicsserver3dextension.html).\n\n"]
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct PhysicsServer3DExtension {
        object_ptr: sys::GDExtensionObjectPtr,
    }
    #[doc = "Virtual methods for class [`PhysicsServer3DExtension`][crate::engine::PhysicsServer3DExtension].\n\nThese methods represent constructors (`init`) or callbacks invoked by the engine.\n\nSee also [Godot docs for `PhysicsServer3DExtension` methods](https://docs.godotengine.org/en/stable/classes/class_physicsserver3dextension.html#methods).\n\n"]
    #[allow(unused_variables)]
    #[allow(clippy::unimplemented)]
    pub trait PhysicsServer3DExtensionVirtual:
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
        fn world_boundary_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn separation_ray_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn sphere_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn box_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn capsule_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn cylinder_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn convex_polygon_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn concave_polygon_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn heightmap_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn custom_shape_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn shape_set_data(&mut self, shape: Rid, data: Variant) {
            unimplemented!()
        }
        fn shape_set_custom_solver_bias(&mut self, shape: Rid, bias: f64) {
            unimplemented!()
        }
        fn shape_set_margin(&mut self, shape: Rid, margin: f64) {
            unimplemented!()
        }
        fn shape_get_margin(&self, shape: Rid) -> f64 {
            unimplemented!()
        }
        fn shape_get_type(&self, shape: Rid) -> physics_server_3d::ShapeType {
            unimplemented!()
        }
        fn shape_get_data(&self, shape: Rid) -> Variant {
            unimplemented!()
        }
        fn shape_get_custom_solver_bias(&self, shape: Rid) -> f64 {
            unimplemented!()
        }
        fn space_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn space_set_active(&mut self, space: Rid, active: bool) {
            unimplemented!()
        }
        fn space_is_active(&self, space: Rid) -> bool {
            unimplemented!()
        }
        fn space_set_param(
            &mut self,
            space: Rid,
            param: physics_server_3d::SpaceParameter,
            value: f64,
        ) {
            unimplemented!()
        }
        fn space_get_param(&self, space: Rid, param: physics_server_3d::SpaceParameter) -> f64 {
            unimplemented!()
        }
        fn space_get_direct_state(&mut self, space: Rid) -> Option<Gd<PhysicsDirectSpaceState3D>> {
            unimplemented!()
        }
        fn space_set_debug_contacts(&mut self, space: Rid, max_contacts: i64) {
            unimplemented!()
        }
        fn space_get_contacts(&self, space: Rid) -> PackedVector3Array {
            unimplemented!()
        }
        fn space_get_contact_count(&self, space: Rid) -> i64 {
            unimplemented!()
        }
        fn area_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn area_set_space(&mut self, area: Rid, space: Rid) {
            unimplemented!()
        }
        fn area_get_space(&self, area: Rid) -> Rid {
            unimplemented!()
        }
        fn area_add_shape(
            &mut self,
            area: Rid,
            shape: Rid,
            transform: Transform3D,
            disabled: bool,
        ) {
            unimplemented!()
        }
        fn area_set_shape(&mut self, area: Rid, shape_idx: i64, shape: Rid) {
            unimplemented!()
        }
        fn area_set_shape_transform(&mut self, area: Rid, shape_idx: i64, transform: Transform3D) {
            unimplemented!()
        }
        fn area_set_shape_disabled(&mut self, area: Rid, shape_idx: i64, disabled: bool) {
            unimplemented!()
        }
        fn area_get_shape_count(&self, area: Rid) -> i64 {
            unimplemented!()
        }
        fn area_get_shape(&self, area: Rid, shape_idx: i64) -> Rid {
            unimplemented!()
        }
        fn area_get_shape_transform(&self, area: Rid, shape_idx: i64) -> Transform3D {
            unimplemented!()
        }
        fn area_remove_shape(&mut self, area: Rid, shape_idx: i64) {
            unimplemented!()
        }
        fn area_clear_shapes(&mut self, area: Rid) {
            unimplemented!()
        }
        fn area_attach_object_instance_id(&mut self, area: Rid, id: i64) {
            unimplemented!()
        }
        fn area_get_object_instance_id(&self, area: Rid) -> i64 {
            unimplemented!()
        }
        fn area_set_param(
            &mut self,
            area: Rid,
            param: physics_server_3d::AreaParameter,
            value: Variant,
        ) {
            unimplemented!()
        }
        fn area_set_transform(&mut self, area: Rid, transform: Transform3D) {
            unimplemented!()
        }
        fn area_get_param(&self, area: Rid, param: physics_server_3d::AreaParameter) -> Variant {
            unimplemented!()
        }
        fn area_get_transform(&self, area: Rid) -> Transform3D {
            unimplemented!()
        }
        fn area_set_collision_layer(&mut self, area: Rid, layer: i64) {
            unimplemented!()
        }
        fn area_get_collision_layer(&self, area: Rid) -> i64 {
            unimplemented!()
        }
        fn area_set_collision_mask(&mut self, area: Rid, mask: i64) {
            unimplemented!()
        }
        fn area_get_collision_mask(&self, area: Rid) -> i64 {
            unimplemented!()
        }
        fn area_set_monitorable(&mut self, area: Rid, monitorable: bool) {
            unimplemented!()
        }
        fn area_set_ray_pickable(&mut self, area: Rid, enable: bool) {
            unimplemented!()
        }
        fn area_set_monitor_callback(&mut self, area: Rid, callback: Callable) {
            unimplemented!()
        }
        fn area_set_area_monitor_callback(&mut self, area: Rid, callback: Callable) {
            unimplemented!()
        }
        fn body_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn body_set_space(&mut self, body: Rid, space: Rid) {
            unimplemented!()
        }
        fn body_get_space(&self, body: Rid) -> Rid {
            unimplemented!()
        }
        fn body_set_mode(&mut self, body: Rid, mode: physics_server_3d::BodyMode) {
            unimplemented!()
        }
        fn body_get_mode(&self, body: Rid) -> physics_server_3d::BodyMode {
            unimplemented!()
        }
        fn body_add_shape(
            &mut self,
            body: Rid,
            shape: Rid,
            transform: Transform3D,
            disabled: bool,
        ) {
            unimplemented!()
        }
        fn body_set_shape(&mut self, body: Rid, shape_idx: i64, shape: Rid) {
            unimplemented!()
        }
        fn body_set_shape_transform(&mut self, body: Rid, shape_idx: i64, transform: Transform3D) {
            unimplemented!()
        }
        fn body_set_shape_disabled(&mut self, body: Rid, shape_idx: i64, disabled: bool) {
            unimplemented!()
        }
        fn body_get_shape_count(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn body_get_shape(&self, body: Rid, shape_idx: i64) -> Rid {
            unimplemented!()
        }
        fn body_get_shape_transform(&self, body: Rid, shape_idx: i64) -> Transform3D {
            unimplemented!()
        }
        fn body_remove_shape(&mut self, body: Rid, shape_idx: i64) {
            unimplemented!()
        }
        fn body_clear_shapes(&mut self, body: Rid) {
            unimplemented!()
        }
        fn body_attach_object_instance_id(&mut self, body: Rid, id: i64) {
            unimplemented!()
        }
        fn body_get_object_instance_id(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn body_set_enable_continuous_collision_detection(&mut self, body: Rid, enable: bool) {
            unimplemented!()
        }
        fn body_is_continuous_collision_detection_enabled(&self, body: Rid) -> bool {
            unimplemented!()
        }
        fn body_set_collision_layer(&mut self, body: Rid, layer: i64) {
            unimplemented!()
        }
        fn body_get_collision_layer(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn body_set_collision_mask(&mut self, body: Rid, mask: i64) {
            unimplemented!()
        }
        fn body_get_collision_mask(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn body_set_collision_priority(&mut self, body: Rid, priority: f64) {
            unimplemented!()
        }
        fn body_get_collision_priority(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn body_set_user_flags(&mut self, body: Rid, flags: i64) {
            unimplemented!()
        }
        fn body_get_user_flags(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn body_set_param(
            &mut self,
            body: Rid,
            param: physics_server_3d::BodyParameter,
            value: Variant,
        ) {
            unimplemented!()
        }
        fn body_get_param(&self, body: Rid, param: physics_server_3d::BodyParameter) -> Variant {
            unimplemented!()
        }
        fn body_reset_mass_properties(&mut self, body: Rid) {
            unimplemented!()
        }
        fn body_set_state(
            &mut self,
            body: Rid,
            state: physics_server_3d::BodyState,
            value: Variant,
        ) {
            unimplemented!()
        }
        fn body_get_state(&self, body: Rid, state: physics_server_3d::BodyState) -> Variant {
            unimplemented!()
        }
        fn body_apply_central_impulse(&mut self, body: Rid, impulse: Vector3) {
            unimplemented!()
        }
        fn body_apply_impulse(&mut self, body: Rid, impulse: Vector3, position: Vector3) {
            unimplemented!()
        }
        fn body_apply_torque_impulse(&mut self, body: Rid, impulse: Vector3) {
            unimplemented!()
        }
        fn body_apply_central_force(&mut self, body: Rid, force: Vector3) {
            unimplemented!()
        }
        fn body_apply_force(&mut self, body: Rid, force: Vector3, position: Vector3) {
            unimplemented!()
        }
        fn body_apply_torque(&mut self, body: Rid, torque: Vector3) {
            unimplemented!()
        }
        fn body_add_constant_central_force(&mut self, body: Rid, force: Vector3) {
            unimplemented!()
        }
        fn body_add_constant_force(&mut self, body: Rid, force: Vector3, position: Vector3) {
            unimplemented!()
        }
        fn body_add_constant_torque(&mut self, body: Rid, torque: Vector3) {
            unimplemented!()
        }
        fn body_set_constant_force(&mut self, body: Rid, force: Vector3) {
            unimplemented!()
        }
        fn body_get_constant_force(&self, body: Rid) -> Vector3 {
            unimplemented!()
        }
        fn body_set_constant_torque(&mut self, body: Rid, torque: Vector3) {
            unimplemented!()
        }
        fn body_get_constant_torque(&self, body: Rid) -> Vector3 {
            unimplemented!()
        }
        fn body_set_axis_velocity(&mut self, body: Rid, axis_velocity: Vector3) {
            unimplemented!()
        }
        fn body_set_axis_lock(&mut self, body: Rid, axis: physics_server_3d::BodyAxis, lock: bool) {
            unimplemented!()
        }
        fn body_is_axis_locked(&self, body: Rid, axis: physics_server_3d::BodyAxis) -> bool {
            unimplemented!()
        }
        fn body_add_collision_exception(&mut self, body: Rid, excepted_body: Rid) {
            unimplemented!()
        }
        fn body_remove_collision_exception(&mut self, body: Rid, excepted_body: Rid) {
            unimplemented!()
        }
        fn body_get_collision_exceptions(&self, body: Rid) -> Array<Rid> {
            unimplemented!()
        }
        fn body_set_max_contacts_reported(&mut self, body: Rid, amount: i64) {
            unimplemented!()
        }
        fn body_get_max_contacts_reported(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn body_set_contacts_reported_depth_threshold(&mut self, body: Rid, threshold: f64) {
            unimplemented!()
        }
        fn body_get_contacts_reported_depth_threshold(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn body_set_omit_force_integration(&mut self, body: Rid, enable: bool) {
            unimplemented!()
        }
        fn body_is_omitting_force_integration(&self, body: Rid) -> bool {
            unimplemented!()
        }
        fn body_set_state_sync_callback(&mut self, body: Rid, callable: Callable) {
            unimplemented!()
        }
        fn body_set_force_integration_callback(
            &mut self,
            body: Rid,
            callable: Callable,
            userdata: Variant,
        ) {
            unimplemented!()
        }
        fn body_set_ray_pickable(&mut self, body: Rid, enable: bool) {
            unimplemented!()
        }
        fn body_get_direct_state(&mut self, body: Rid) -> Option<Gd<PhysicsDirectBodyState3D>> {
            unimplemented!()
        }
        fn soft_body_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn soft_body_update_rendering_server(
            &mut self,
            body: Rid,
            rendering_server_handler: Gd<PhysicsServer3DRenderingServerHandler>,
        ) {
            unimplemented!()
        }
        fn soft_body_set_space(&mut self, body: Rid, space: Rid) {
            unimplemented!()
        }
        fn soft_body_get_space(&self, body: Rid) -> Rid {
            unimplemented!()
        }
        fn soft_body_set_ray_pickable(&mut self, body: Rid, enable: bool) {
            unimplemented!()
        }
        fn soft_body_set_collision_layer(&mut self, body: Rid, layer: i64) {
            unimplemented!()
        }
        fn soft_body_get_collision_layer(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn soft_body_set_collision_mask(&mut self, body: Rid, mask: i64) {
            unimplemented!()
        }
        fn soft_body_get_collision_mask(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn soft_body_add_collision_exception(&mut self, body: Rid, body_b: Rid) {
            unimplemented!()
        }
        fn soft_body_remove_collision_exception(&mut self, body: Rid, body_b: Rid) {
            unimplemented!()
        }
        fn soft_body_get_collision_exceptions(&self, body: Rid) -> Array<Rid> {
            unimplemented!()
        }
        fn soft_body_set_state(
            &mut self,
            body: Rid,
            state: physics_server_3d::BodyState,
            variant: Variant,
        ) {
            unimplemented!()
        }
        fn soft_body_get_state(&self, body: Rid, state: physics_server_3d::BodyState) -> Variant {
            unimplemented!()
        }
        fn soft_body_set_transform(&mut self, body: Rid, transform: Transform3D) {
            unimplemented!()
        }
        fn soft_body_set_simulation_precision(&mut self, body: Rid, simulation_precision: i64) {
            unimplemented!()
        }
        fn soft_body_get_simulation_precision(&self, body: Rid) -> i64 {
            unimplemented!()
        }
        fn soft_body_set_total_mass(&mut self, body: Rid, total_mass: f64) {
            unimplemented!()
        }
        fn soft_body_get_total_mass(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn soft_body_set_linear_stiffness(&mut self, body: Rid, linear_stiffness: f64) {
            unimplemented!()
        }
        fn soft_body_get_linear_stiffness(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn soft_body_set_pressure_coefficient(&mut self, body: Rid, pressure_coefficient: f64) {
            unimplemented!()
        }
        fn soft_body_get_pressure_coefficient(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn soft_body_set_damping_coefficient(&mut self, body: Rid, damping_coefficient: f64) {
            unimplemented!()
        }
        fn soft_body_get_damping_coefficient(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn soft_body_set_drag_coefficient(&mut self, body: Rid, drag_coefficient: f64) {
            unimplemented!()
        }
        fn soft_body_get_drag_coefficient(&self, body: Rid) -> f64 {
            unimplemented!()
        }
        fn soft_body_set_mesh(&mut self, body: Rid, mesh: Rid) {
            unimplemented!()
        }
        fn soft_body_get_bounds(&self, body: Rid) -> Aabb {
            unimplemented!()
        }
        fn soft_body_move_point(&mut self, body: Rid, point_index: i64, global_position: Vector3) {
            unimplemented!()
        }
        fn soft_body_get_point_global_position(&self, body: Rid, point_index: i64) -> Vector3 {
            unimplemented!()
        }
        fn soft_body_remove_all_pinned_points(&mut self, body: Rid) {
            unimplemented!()
        }
        fn soft_body_pin_point(&mut self, body: Rid, point_index: i64, pin: bool) {
            unimplemented!()
        }
        fn soft_body_is_point_pinned(&self, body: Rid, point_index: i64) -> bool {
            unimplemented!()
        }
        fn joint_create(&mut self) -> Rid {
            unimplemented!()
        }
        fn joint_clear(&mut self, joint: Rid) {
            unimplemented!()
        }
        fn joint_make_pin(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_A: Vector3,
            body_B: Rid,
            local_B: Vector3,
        ) {
            unimplemented!()
        }
        fn pin_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::PinJointParam,
            value: f64,
        ) {
            unimplemented!()
        }
        fn pin_joint_get_param(&self, joint: Rid, param: physics_server_3d::PinJointParam) -> f64 {
            unimplemented!()
        }
        fn pin_joint_set_local_a(&mut self, joint: Rid, local_A: Vector3) {
            unimplemented!()
        }
        fn pin_joint_get_local_a(&self, joint: Rid) -> Vector3 {
            unimplemented!()
        }
        fn pin_joint_set_local_b(&mut self, joint: Rid, local_B: Vector3) {
            unimplemented!()
        }
        fn pin_joint_get_local_b(&self, joint: Rid) -> Vector3 {
            unimplemented!()
        }
        fn joint_make_hinge(
            &mut self,
            joint: Rid,
            body_A: Rid,
            hinge_A: Transform3D,
            body_B: Rid,
            hinge_B: Transform3D,
        ) {
            unimplemented!()
        }
        fn joint_make_hinge_simple(
            &mut self,
            joint: Rid,
            body_A: Rid,
            pivot_A: Vector3,
            axis_A: Vector3,
            body_B: Rid,
            pivot_B: Vector3,
            axis_B: Vector3,
        ) {
            unimplemented!()
        }
        fn hinge_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::HingeJointParam,
            value: f64,
        ) {
            unimplemented!()
        }
        fn hinge_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::HingeJointParam,
        ) -> f64 {
            unimplemented!()
        }
        fn hinge_joint_set_flag(
            &mut self,
            joint: Rid,
            flag: physics_server_3d::HingeJointFlag,
            enabled: bool,
        ) {
            unimplemented!()
        }
        fn hinge_joint_get_flag(
            &self,
            joint: Rid,
            flag: physics_server_3d::HingeJointFlag,
        ) -> bool {
            unimplemented!()
        }
        fn joint_make_slider(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_ref_A: Transform3D,
            body_B: Rid,
            local_ref_B: Transform3D,
        ) {
            unimplemented!()
        }
        fn slider_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::SliderJointParam,
            value: f64,
        ) {
            unimplemented!()
        }
        fn slider_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::SliderJointParam,
        ) -> f64 {
            unimplemented!()
        }
        fn joint_make_cone_twist(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_ref_A: Transform3D,
            body_B: Rid,
            local_ref_B: Transform3D,
        ) {
            unimplemented!()
        }
        fn cone_twist_joint_set_param(
            &mut self,
            joint: Rid,
            param: physics_server_3d::ConeTwistJointParam,
            value: f64,
        ) {
            unimplemented!()
        }
        fn cone_twist_joint_get_param(
            &self,
            joint: Rid,
            param: physics_server_3d::ConeTwistJointParam,
        ) -> f64 {
            unimplemented!()
        }
        fn joint_make_generic_6dof(
            &mut self,
            joint: Rid,
            body_A: Rid,
            local_ref_A: Transform3D,
            body_B: Rid,
            local_ref_B: Transform3D,
        ) {
            unimplemented!()
        }
        fn generic_6dof_joint_set_param(
            &mut self,
            joint: Rid,
            axis: Vector3Axis,
            param: physics_server_3d::G6DOFJointAxisParam,
            value: f64,
        ) {
            unimplemented!()
        }
        fn generic_6dof_joint_get_param(
            &self,
            joint: Rid,
            axis: Vector3Axis,
            param: physics_server_3d::G6DOFJointAxisParam,
        ) -> f64 {
            unimplemented!()
        }
        fn generic_6dof_joint_set_flag(
            &mut self,
            joint: Rid,
            axis: Vector3Axis,
            flag: physics_server_3d::G6DOFJointAxisFlag,
            enable: bool,
        ) {
            unimplemented!()
        }
        fn generic_6dof_joint_get_flag(
            &self,
            joint: Rid,
            axis: Vector3Axis,
            flag: physics_server_3d::G6DOFJointAxisFlag,
        ) -> bool {
            unimplemented!()
        }
        fn joint_get_type(&self, joint: Rid) -> physics_server_3d::JointType {
            unimplemented!()
        }
        fn joint_set_solver_priority(&mut self, joint: Rid, priority: i64) {
            unimplemented!()
        }
        fn joint_get_solver_priority(&self, joint: Rid) -> i64 {
            unimplemented!()
        }
        fn joint_disable_collisions_between_bodies(&mut self, joint: Rid, disable: bool) {
            unimplemented!()
        }
        fn joint_is_disabled_collisions_between_bodies(&self, joint: Rid) -> bool {
            unimplemented!()
        }
        fn free_rid(&mut self, rid: Rid) {
            unimplemented!()
        }
        fn set_active(&mut self, active: bool) {
            unimplemented!()
        }
        fn init_ext(&mut self) {
            unimplemented!()
        }
        fn step(&mut self, step: f64) {
            unimplemented!()
        }
        fn sync(&mut self) {
            unimplemented!()
        }
        fn flush_queries(&mut self) {
            unimplemented!()
        }
        fn end_sync(&mut self) {
            unimplemented!()
        }
        fn finish(&mut self) {
            unimplemented!()
        }
        fn is_flushing_queries(&self) -> bool {
            unimplemented!()
        }
        fn get_process_info(&mut self, process_info: physics_server_3d::ProcessInfo) -> i64 {
            unimplemented!()
        }
    }
    impl PhysicsServer3DExtension {
        #[must_use]
        pub fn new_alloc() -> Gd<Self> {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3DExtension");
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
        pub fn body_test_motion_is_excluding_body(&self, body: Rid) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3DExtension");
                let __method_name = StringName::from("body_test_motion_is_excluding_body");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    4155700596i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3DExtension" , "body_test_motion_is_excluding_body" , 4155700596i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<Rid as sys::GodotFfi>::sys_const(&body)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
        pub fn body_test_motion_is_excluding_object(&self, object: i64) -> bool {
            unsafe {
                let __class_name = StringName::from("PhysicsServer3DExtension");
                let __method_name = StringName::from("body_test_motion_is_excluding_object");
                let __method_bind = sys::interface_fn!(classdb_get_method_bind)(
                    __class_name.string_sys(),
                    __method_name.string_sys(),
                    1116898809i64,
                );
                assert ! (! __method_bind . is_null () , "failed to load method {}::{} (hash {}) -- possible Godot/gdext version mismatch" , "PhysicsServer3DExtension" , "body_test_motion_is_excluding_object" , 1116898809i64);
                let __call_fn = sys::interface_fn!(object_method_bind_ptrcall);
                let __args = [<i64 as sys::GodotFfi>::sys_const(&object)];
                let __args_ptr = __args.as_ptr();
                <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                    __call_fn(__method_bind, self.object_ptr, __args_ptr, return_ptr);
                })
            }
        }
    }
    impl crate::obj::GodotClass for PhysicsServer3DExtension {
        type Base = crate::engine::PhysicsServer3D;
        type Declarer = crate::obj::dom::EngineDomain;
        type Mem = crate::obj::mem::ManualMemory;
        const CLASS_NAME: &'static str = "PhysicsServer3DExtension";
    }
    impl crate::obj::EngineClass for PhysicsServer3DExtension {
        fn as_object_ptr(&self) -> sys::GDExtensionObjectPtr {
            self.object_ptr
        }
        fn as_type_ptr(&self) -> sys::GDExtensionTypePtr {
            std::ptr::addr_of!(self.object_ptr) as sys::GDExtensionTypePtr
        }
    }
    impl crate::obj::Inherits<crate::engine::PhysicsServer3D> for PhysicsServer3DExtension {}
    impl crate::obj::Inherits<crate::engine::Object> for PhysicsServer3DExtension {}
    impl std::ops::Deref for PhysicsServer3DExtension {
        type Target = crate::engine::PhysicsServer3D;
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&Self, &Self::Target>(self) }
        }
    }
    impl std::ops::DerefMut for PhysicsServer3DExtension {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe { std::mem::transmute::<&mut Self, &mut Self::Target>(self) }
        }
    }
    #[macro_export]
    #[allow(non_snake_case)]
    macro_rules! inherits_transitive_PhysicsServer3DExtension {
        ($ Class : ident) => {
            impl ::godot::obj::Inherits<::godot::engine::PhysicsServer3DExtension> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::PhysicsServer3D> for $Class {}
            impl ::godot::obj::Inherits<::godot::engine::Object> for $Class {}
        };
    }
}
