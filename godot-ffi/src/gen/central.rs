use crate::{
    ffi_methods, GDExtensionConstTypePtr, GDExtensionTypePtr, GDExtensionVariantPtr, GodotFfi,
};
pub mod types {
    pub type OpaqueNil = crate::opaque::Opaque<0usize>;
    pub type OpaqueBool = crate::opaque::Opaque<1usize>;
    pub type OpaqueInt = crate::opaque::Opaque<8usize>;
    pub type OpaqueFloat = crate::opaque::Opaque<8usize>;
    pub type OpaqueString = crate::opaque::Opaque<8usize>;
    pub type OpaqueVector2 = crate::opaque::Opaque<8usize>;
    pub type OpaqueVector2i = crate::opaque::Opaque<8usize>;
    pub type OpaqueRect2 = crate::opaque::Opaque<16usize>;
    pub type OpaqueRect2i = crate::opaque::Opaque<16usize>;
    pub type OpaqueVector3 = crate::opaque::Opaque<12usize>;
    pub type OpaqueVector3i = crate::opaque::Opaque<12usize>;
    pub type OpaqueTransform2D = crate::opaque::Opaque<24usize>;
    pub type OpaqueVector4 = crate::opaque::Opaque<16usize>;
    pub type OpaqueVector4i = crate::opaque::Opaque<16usize>;
    pub type OpaquePlane = crate::opaque::Opaque<16usize>;
    pub type OpaqueQuaternion = crate::opaque::Opaque<16usize>;
    pub type OpaqueAabb = crate::opaque::Opaque<24usize>;
    pub type OpaqueBasis = crate::opaque::Opaque<36usize>;
    pub type OpaqueTransform3D = crate::opaque::Opaque<48usize>;
    pub type OpaqueProjection = crate::opaque::Opaque<64usize>;
    pub type OpaqueColor = crate::opaque::Opaque<16usize>;
    pub type OpaqueStringName = crate::opaque::Opaque<8usize>;
    pub type OpaqueNodePath = crate::opaque::Opaque<8usize>;
    pub type OpaqueRid = crate::opaque::Opaque<8usize>;
    pub type OpaqueObject = crate::opaque::Opaque<8usize>;
    pub type OpaqueCallable = crate::opaque::Opaque<16usize>;
    pub type OpaqueSignal = crate::opaque::Opaque<16usize>;
    pub type OpaqueDictionary = crate::opaque::Opaque<8usize>;
    pub type OpaqueArray = crate::opaque::Opaque<8usize>;
    pub type OpaquePackedByteArray = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedInt32Array = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedInt64Array = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedFloat32Array = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedFloat64Array = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedStringArray = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedVector2Array = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedVector3Array = crate::opaque::Opaque<16usize>;
    pub type OpaquePackedColorArray = crate::opaque::Opaque<16usize>;
    pub type OpaqueVariant = crate::opaque::Opaque<24usize>;
}
pub struct GlobalMethodTable {
    pub bool_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub bool_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub int_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub int_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub float_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub float_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub string_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub string_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub string_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub string_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub string_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_from_string_name:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_from_node_path:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub vector2_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub vector2_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub vector2_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector2_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector2_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2_from_vector2i:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2_from_x_y: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2i_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub vector2i_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub vector2i_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector2i_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector2i_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2i_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2i_from_vector2:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector2i_from_x_y: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub rect2_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub rect2_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub rect2_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2_from_rect2i: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2_from_position_size:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2_from_x_y_width_height:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2i_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub rect2i_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub rect2i_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub rect2i_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2i_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2i_from_rect2: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2i_from_position_size:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rect2i_from_x_y_width_height:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub vector3_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub vector3_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector3_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector3_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3_from_vector3i:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3_from_x_y_z:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3i_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub vector3i_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub vector3i_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector3i_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector3i_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3i_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3i_from_vector3:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector3i_from_x_y_z:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_2d_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub transform_2d_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub transform_2d_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub transform_2d_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_2d_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_2d_from_rotation_position:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_2d_from_rotation_scale_skew_position:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_2d_from_x_axis_y_axis_origin:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub vector4_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub vector4_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector4_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector4_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4_from_vector4i:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4_from_x_y_z_w:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4i_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub vector4i_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub vector4i_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector4i_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub vector4i_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4i_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4i_from_vector4:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub vector4i_from_x_y_z_w:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub plane_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub plane_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub plane_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_from_normal: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_from_normal_d:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_from_normal_point:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_from_point1_point2_point3:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub plane_from_a_b_c_d:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub quaternion_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub quaternion_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub quaternion_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub quaternion_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub quaternion_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub quaternion_from_basis:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub quaternion_from_axis_angle:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub quaternion_from_arc_from_arc_to:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub quaternion_from_x_y_z_w:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub aabb_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub aabb_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub aabb_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub aabb_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub aabb_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub aabb_from_position_size:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub basis_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub basis_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub basis_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub basis_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub basis_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub basis_from_quaternion:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub basis_from_axis_angle:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub basis_from_x_axis_y_axis_z_axis:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_3d_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub transform_3d_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub transform_3d_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub transform_3d_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_3d_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_3d_from_basis_origin:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_3d_from_x_axis_y_axis_z_axis_origin:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub transform_3d_from_projection:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub projection_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub projection_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub projection_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub projection_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub projection_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub projection_from_transform_3d:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub projection_from_x_axis_y_axis_z_axis_w_axis:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub color_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub color_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub color_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_from_from_alpha:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_from_r_g_b: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_from_r_g_b_a:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_from_code: unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub color_from_code_alpha:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_name_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub string_name_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub string_name_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub string_name_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub string_name_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_name_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_name_from_string:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub string_name_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub node_path_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub node_path_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub node_path_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub node_path_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub node_path_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub node_path_from_string:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub node_path_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub rid_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub rid_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub rid_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub rid_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub rid_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub rid_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub object_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub object_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub callable_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub callable_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub callable_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub callable_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub callable_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub callable_from_object_method:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub callable_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub signal_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub signal_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub signal_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub signal_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub signal_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub signal_from_object_signal:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub signal_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub dictionary_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub dictionary_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub dictionary_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub dictionary_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub dictionary_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub dictionary_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub array_to_variant: unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub array_from_variant: unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub array_operator_less:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_base_type_class_name_script:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_byte_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_int32_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_int64_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_float32_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_float64_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_string_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_vector2_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_vector3_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_from_packed_color_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_byte_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_byte_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_byte_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_byte_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_byte_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_byte_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_byte_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_int32_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_int32_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_int32_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_int32_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_int32_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_int32_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_int32_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_int64_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_int64_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_int64_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_int64_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_int64_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_int64_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_int64_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_float32_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_float32_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_float32_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_float32_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_float32_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_float32_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_float32_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_float64_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_float64_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_float64_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_float64_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_float64_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_float64_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_float64_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_string_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_string_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_string_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_string_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_string_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_string_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_string_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_vector2_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_vector2_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_vector2_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_vector2_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_vector2_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_vector2_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_vector2_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_vector3_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_vector3_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_vector3_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_vector3_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_vector3_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_vector3_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_vector3_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
    pub packed_color_array_to_variant:
        unsafe extern "C" fn(GDExtensionVariantPtr, GDExtensionTypePtr),
    pub packed_color_array_from_variant:
        unsafe extern "C" fn(GDExtensionTypePtr, GDExtensionVariantPtr),
    pub packed_color_array_operator_equal:
        unsafe extern "C" fn(GDExtensionConstTypePtr, GDExtensionConstTypePtr, GDExtensionTypePtr),
    pub packed_color_array_construct_default:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_color_array_construct_copy:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_color_array_from_array:
        unsafe extern "C" fn(GDExtensionTypePtr, *const GDExtensionConstTypePtr),
    pub packed_color_array_destroy: unsafe extern "C" fn(GDExtensionTypePtr),
}
impl GlobalMethodTable {
    pub(crate) unsafe fn new(interface: &crate::GDExtensionInterface) -> Self {
        Self {
            bool_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BOOL)
                    .expect("failed to load GDExtension function `bool_to_variant`")
            },
            bool_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BOOL)
                    .expect("failed to load GDExtension function `bool_from_variant`")
            },
            int_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_INT)
                    .expect("failed to load GDExtension function `int_to_variant`")
            },
            int_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_INT)
                    .expect("failed to load GDExtension function `int_from_variant`")
            },
            float_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_FLOAT)
                    .expect("failed to load GDExtension function `float_to_variant`")
            },
            float_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_FLOAT)
                    .expect("failed to load GDExtension function `float_from_variant`")
            },
            string_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING)
                    .expect("failed to load GDExtension function `string_to_variant`")
            },
            string_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING)
                    .expect("failed to load GDExtension function `string_from_variant`")
            },
            string_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING,
                )
                .expect("failed to load GDExtension function `string_operator_equal`")
            },
            string_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING,
                )
                .expect("failed to load GDExtension function `string_operator_less`")
            },
            string_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 0i32)
                    .expect("failed to load GDExtension function `string_construct_default`")
            },
            string_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 1i32)
                    .expect("failed to load GDExtension function `string_construct_copy`")
            },
            string_from_string_name: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 2i32)
                    .expect("failed to load GDExtension function `string_from_string_name`")
            },
            string_from_node_path: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING, 3i32)
                    .expect("failed to load GDExtension function `string_from_node_path`")
            },
            string_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING).unwrap()
            },
            vector2_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2)
                    .expect("failed to load GDExtension function `vector2_to_variant`")
            },
            vector2_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2)
                    .expect("failed to load GDExtension function `vector2_from_variant`")
            },
            vector2_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2,
                )
                .expect("failed to load GDExtension function `vector2_operator_equal`")
            },
            vector2_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2,
                )
                .expect("failed to load GDExtension function `vector2_operator_less`")
            },
            vector2_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 0i32)
                    .expect("failed to load GDExtension function `vector2_construct_default`")
            },
            vector2_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 1i32)
                    .expect("failed to load GDExtension function `vector2_construct_copy`")
            },
            vector2_from_vector2i: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 2i32)
                    .expect("failed to load GDExtension function `vector2_from_vector2i`")
            },
            vector2_from_x_y: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2, 3i32)
                    .expect("failed to load GDExtension function `vector2_from_x_y`")
            },
            vector2i_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I)
                    .expect("failed to load GDExtension function `vector2i_to_variant`")
            },
            vector2i_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I)
                    .expect("failed to load GDExtension function `vector2i_from_variant`")
            },
            vector2i_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I,
                )
                .expect("failed to load GDExtension function `vector2i_operator_equal`")
            },
            vector2i_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I,
                )
                .expect("failed to load GDExtension function `vector2i_operator_less`")
            },
            vector2i_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 0i32)
                    .expect("failed to load GDExtension function `vector2i_construct_default`")
            },
            vector2i_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 1i32)
                    .expect("failed to load GDExtension function `vector2i_construct_copy`")
            },
            vector2i_from_vector2: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 2i32)
                    .expect("failed to load GDExtension function `vector2i_from_vector2`")
            },
            vector2i_from_x_y: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR2I, 3i32)
                    .expect("failed to load GDExtension function `vector2i_from_x_y`")
            },
            rect2_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2)
                    .expect("failed to load GDExtension function `rect2_to_variant`")
            },
            rect2_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2)
                    .expect("failed to load GDExtension function `rect2_from_variant`")
            },
            rect2_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_RECT2,
                    crate::GDEXTENSION_VARIANT_TYPE_RECT2,
                )
                .expect("failed to load GDExtension function `rect2_operator_equal`")
            },
            rect2_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 0i32)
                    .expect("failed to load GDExtension function `rect2_construct_default`")
            },
            rect2_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 1i32)
                    .expect("failed to load GDExtension function `rect2_construct_copy`")
            },
            rect2_from_rect2i: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 2i32)
                    .expect("failed to load GDExtension function `rect2_from_rect2i`")
            },
            rect2_from_position_size: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 3i32)
                    .expect("failed to load GDExtension function `rect2_from_position_size`")
            },
            rect2_from_x_y_width_height: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2, 4i32)
                    .expect("failed to load GDExtension function `rect2_from_x_y_width_height`")
            },
            rect2i_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I)
                    .expect("failed to load GDExtension function `rect2i_to_variant`")
            },
            rect2i_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I)
                    .expect("failed to load GDExtension function `rect2i_from_variant`")
            },
            rect2i_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_RECT2I,
                    crate::GDEXTENSION_VARIANT_TYPE_RECT2I,
                )
                .expect("failed to load GDExtension function `rect2i_operator_equal`")
            },
            rect2i_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 0i32)
                    .expect("failed to load GDExtension function `rect2i_construct_default`")
            },
            rect2i_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 1i32)
                    .expect("failed to load GDExtension function `rect2i_construct_copy`")
            },
            rect2i_from_rect2: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 2i32)
                    .expect("failed to load GDExtension function `rect2i_from_rect2`")
            },
            rect2i_from_position_size: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 3i32)
                    .expect("failed to load GDExtension function `rect2i_from_position_size`")
            },
            rect2i_from_x_y_width_height: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RECT2I, 4i32)
                    .expect("failed to load GDExtension function `rect2i_from_x_y_width_height`")
            },
            vector3_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3)
                    .expect("failed to load GDExtension function `vector3_to_variant`")
            },
            vector3_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3)
                    .expect("failed to load GDExtension function `vector3_from_variant`")
            },
            vector3_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3,
                )
                .expect("failed to load GDExtension function `vector3_operator_equal`")
            },
            vector3_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3,
                )
                .expect("failed to load GDExtension function `vector3_operator_less`")
            },
            vector3_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 0i32)
                    .expect("failed to load GDExtension function `vector3_construct_default`")
            },
            vector3_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 1i32)
                    .expect("failed to load GDExtension function `vector3_construct_copy`")
            },
            vector3_from_vector3i: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 2i32)
                    .expect("failed to load GDExtension function `vector3_from_vector3i`")
            },
            vector3_from_x_y_z: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3, 3i32)
                    .expect("failed to load GDExtension function `vector3_from_x_y_z`")
            },
            vector3i_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I)
                    .expect("failed to load GDExtension function `vector3i_to_variant`")
            },
            vector3i_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I)
                    .expect("failed to load GDExtension function `vector3i_from_variant`")
            },
            vector3i_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I,
                )
                .expect("failed to load GDExtension function `vector3i_operator_equal`")
            },
            vector3i_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I,
                )
                .expect("failed to load GDExtension function `vector3i_operator_less`")
            },
            vector3i_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 0i32)
                    .expect("failed to load GDExtension function `vector3i_construct_default`")
            },
            vector3i_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 1i32)
                    .expect("failed to load GDExtension function `vector3i_construct_copy`")
            },
            vector3i_from_vector3: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 2i32)
                    .expect("failed to load GDExtension function `vector3i_from_vector3`")
            },
            vector3i_from_x_y_z: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR3I, 3i32)
                    .expect("failed to load GDExtension function `vector3i_from_x_y_z`")
            },
            transform_2d_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D)
                    .expect("failed to load GDExtension function `transform_2d_to_variant`")
            },
            transform_2d_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D)
                    .expect("failed to load GDExtension function `transform_2d_from_variant`")
            },
            transform_2d_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D,
                    crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D,
                )
                .expect("failed to load GDExtension function `transform_2d_operator_equal`")
            },
            transform_2d_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 0i32)
                    .expect("failed to load GDExtension function `transform_2d_construct_default`")
            },
            transform_2d_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 1i32)
                    .expect("failed to load GDExtension function `transform_2d_construct_copy`")
            },
            transform_2d_from_rotation_position: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 2i32).expect(
                    "failed to load GDExtension function `transform_2d_from_rotation_position`",
                )
            },
            transform_2d_from_rotation_scale_skew_position: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn (crate :: GDEXTENSION_VARIANT_TYPE_TRANSFORM2D , 3i32) . expect ("failed to load GDExtension function `transform_2d_from_rotation_scale_skew_position`")
            },
            transform_2d_from_x_axis_y_axis_origin: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM2D, 4i32).expect(
                    "failed to load GDExtension function `transform_2d_from_x_axis_y_axis_origin`",
                )
            },
            vector4_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4)
                    .expect("failed to load GDExtension function `vector4_to_variant`")
            },
            vector4_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4)
                    .expect("failed to load GDExtension function `vector4_from_variant`")
            },
            vector4_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4,
                )
                .expect("failed to load GDExtension function `vector4_operator_equal`")
            },
            vector4_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4,
                )
                .expect("failed to load GDExtension function `vector4_operator_less`")
            },
            vector4_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 0i32)
                    .expect("failed to load GDExtension function `vector4_construct_default`")
            },
            vector4_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 1i32)
                    .expect("failed to load GDExtension function `vector4_construct_copy`")
            },
            vector4_from_vector4i: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 2i32)
                    .expect("failed to load GDExtension function `vector4_from_vector4i`")
            },
            vector4_from_x_y_z_w: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4, 3i32)
                    .expect("failed to load GDExtension function `vector4_from_x_y_z_w`")
            },
            vector4i_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I)
                    .expect("failed to load GDExtension function `vector4i_to_variant`")
            },
            vector4i_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I)
                    .expect("failed to load GDExtension function `vector4i_from_variant`")
            },
            vector4i_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I,
                )
                .expect("failed to load GDExtension function `vector4i_operator_equal`")
            },
            vector4i_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I,
                    crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I,
                )
                .expect("failed to load GDExtension function `vector4i_operator_less`")
            },
            vector4i_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 0i32)
                    .expect("failed to load GDExtension function `vector4i_construct_default`")
            },
            vector4i_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 1i32)
                    .expect("failed to load GDExtension function `vector4i_construct_copy`")
            },
            vector4i_from_vector4: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 2i32)
                    .expect("failed to load GDExtension function `vector4i_from_vector4`")
            },
            vector4i_from_x_y_z_w: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_VECTOR4I, 3i32)
                    .expect("failed to load GDExtension function `vector4i_from_x_y_z_w`")
            },
            plane_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE)
                    .expect("failed to load GDExtension function `plane_to_variant`")
            },
            plane_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE)
                    .expect("failed to load GDExtension function `plane_from_variant`")
            },
            plane_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PLANE,
                    crate::GDEXTENSION_VARIANT_TYPE_PLANE,
                )
                .expect("failed to load GDExtension function `plane_operator_equal`")
            },
            plane_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 0i32)
                    .expect("failed to load GDExtension function `plane_construct_default`")
            },
            plane_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 1i32)
                    .expect("failed to load GDExtension function `plane_construct_copy`")
            },
            plane_from_normal: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 2i32)
                    .expect("failed to load GDExtension function `plane_from_normal`")
            },
            plane_from_normal_d: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 3i32)
                    .expect("failed to load GDExtension function `plane_from_normal_d`")
            },
            plane_from_normal_point: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 4i32)
                    .expect("failed to load GDExtension function `plane_from_normal_point`")
            },
            plane_from_point1_point2_point3: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 5i32)
                    .expect("failed to load GDExtension function `plane_from_point1_point2_point3`")
            },
            plane_from_a_b_c_d: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PLANE, 6i32)
                    .expect("failed to load GDExtension function `plane_from_a_b_c_d`")
            },
            quaternion_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION)
                    .expect("failed to load GDExtension function `quaternion_to_variant`")
            },
            quaternion_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION)
                    .expect("failed to load GDExtension function `quaternion_from_variant`")
            },
            quaternion_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_QUATERNION,
                    crate::GDEXTENSION_VARIANT_TYPE_QUATERNION,
                )
                .expect("failed to load GDExtension function `quaternion_operator_equal`")
            },
            quaternion_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 0i32)
                    .expect("failed to load GDExtension function `quaternion_construct_default`")
            },
            quaternion_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 1i32)
                    .expect("failed to load GDExtension function `quaternion_construct_copy`")
            },
            quaternion_from_basis: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 2i32)
                    .expect("failed to load GDExtension function `quaternion_from_basis`")
            },
            quaternion_from_axis_angle: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 3i32)
                    .expect("failed to load GDExtension function `quaternion_from_axis_angle`")
            },
            quaternion_from_arc_from_arc_to: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 4i32)
                    .expect("failed to load GDExtension function `quaternion_from_arc_from_arc_to`")
            },
            quaternion_from_x_y_z_w: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_QUATERNION, 5i32)
                    .expect("failed to load GDExtension function `quaternion_from_x_y_z_w`")
            },
            aabb_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB)
                    .expect("failed to load GDExtension function `aabb_to_variant`")
            },
            aabb_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB)
                    .expect("failed to load GDExtension function `aabb_from_variant`")
            },
            aabb_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_AABB,
                    crate::GDEXTENSION_VARIANT_TYPE_AABB,
                )
                .expect("failed to load GDExtension function `aabb_operator_equal`")
            },
            aabb_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB, 0i32)
                    .expect("failed to load GDExtension function `aabb_construct_default`")
            },
            aabb_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB, 1i32)
                    .expect("failed to load GDExtension function `aabb_construct_copy`")
            },
            aabb_from_position_size: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_AABB, 2i32)
                    .expect("failed to load GDExtension function `aabb_from_position_size`")
            },
            basis_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS)
                    .expect("failed to load GDExtension function `basis_to_variant`")
            },
            basis_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS)
                    .expect("failed to load GDExtension function `basis_from_variant`")
            },
            basis_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_BASIS,
                    crate::GDEXTENSION_VARIANT_TYPE_BASIS,
                )
                .expect("failed to load GDExtension function `basis_operator_equal`")
            },
            basis_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 0i32)
                    .expect("failed to load GDExtension function `basis_construct_default`")
            },
            basis_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 1i32)
                    .expect("failed to load GDExtension function `basis_construct_copy`")
            },
            basis_from_quaternion: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 2i32)
                    .expect("failed to load GDExtension function `basis_from_quaternion`")
            },
            basis_from_axis_angle: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 3i32)
                    .expect("failed to load GDExtension function `basis_from_axis_angle`")
            },
            basis_from_x_axis_y_axis_z_axis: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_BASIS, 4i32)
                    .expect("failed to load GDExtension function `basis_from_x_axis_y_axis_z_axis`")
            },
            transform_3d_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D)
                    .expect("failed to load GDExtension function `transform_3d_to_variant`")
            },
            transform_3d_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D)
                    .expect("failed to load GDExtension function `transform_3d_from_variant`")
            },
            transform_3d_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D,
                    crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D,
                )
                .expect("failed to load GDExtension function `transform_3d_operator_equal`")
            },
            transform_3d_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 0i32)
                    .expect("failed to load GDExtension function `transform_3d_construct_default`")
            },
            transform_3d_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 1i32)
                    .expect("failed to load GDExtension function `transform_3d_construct_copy`")
            },
            transform_3d_from_basis_origin: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 2i32)
                    .expect("failed to load GDExtension function `transform_3d_from_basis_origin`")
            },
            transform_3d_from_x_axis_y_axis_z_axis_origin: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn (crate :: GDEXTENSION_VARIANT_TYPE_TRANSFORM3D , 3i32) . expect ("failed to load GDExtension function `transform_3d_from_x_axis_y_axis_z_axis_origin`")
            },
            transform_3d_from_projection: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_TRANSFORM3D, 4i32)
                    .expect("failed to load GDExtension function `transform_3d_from_projection`")
            },
            projection_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION)
                    .expect("failed to load GDExtension function `projection_to_variant`")
            },
            projection_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION)
                    .expect("failed to load GDExtension function `projection_from_variant`")
            },
            projection_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PROJECTION,
                    crate::GDEXTENSION_VARIANT_TYPE_PROJECTION,
                )
                .expect("failed to load GDExtension function `projection_operator_equal`")
            },
            projection_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 0i32)
                    .expect("failed to load GDExtension function `projection_construct_default`")
            },
            projection_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 1i32)
                    .expect("failed to load GDExtension function `projection_construct_copy`")
            },
            projection_from_transform_3d: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PROJECTION, 2i32)
                    .expect("failed to load GDExtension function `projection_from_transform_3d`")
            },
            projection_from_x_axis_y_axis_z_axis_w_axis: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn (crate :: GDEXTENSION_VARIANT_TYPE_PROJECTION , 3i32) . expect ("failed to load GDExtension function `projection_from_x_axis_y_axis_z_axis_w_axis`")
            },
            color_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR)
                    .expect("failed to load GDExtension function `color_to_variant`")
            },
            color_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR)
                    .expect("failed to load GDExtension function `color_from_variant`")
            },
            color_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_COLOR,
                    crate::GDEXTENSION_VARIANT_TYPE_COLOR,
                )
                .expect("failed to load GDExtension function `color_operator_equal`")
            },
            color_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 0i32)
                    .expect("failed to load GDExtension function `color_construct_default`")
            },
            color_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 1i32)
                    .expect("failed to load GDExtension function `color_construct_copy`")
            },
            color_from_from_alpha: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 2i32)
                    .expect("failed to load GDExtension function `color_from_from_alpha`")
            },
            color_from_r_g_b: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 3i32)
                    .expect("failed to load GDExtension function `color_from_r_g_b`")
            },
            color_from_r_g_b_a: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 4i32)
                    .expect("failed to load GDExtension function `color_from_r_g_b_a`")
            },
            color_from_code: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 5i32)
                    .expect("failed to load GDExtension function `color_from_code`")
            },
            color_from_code_alpha: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_COLOR, 6i32)
                    .expect("failed to load GDExtension function `color_from_code_alpha`")
            },
            string_name_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                    .expect("failed to load GDExtension function `string_name_to_variant`")
            },
            string_name_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME)
                    .expect("failed to load GDExtension function `string_name_from_variant`")
            },
            string_name_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME,
                )
                .expect("failed to load GDExtension function `string_name_operator_equal`")
            },
            string_name_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME,
                    crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME,
                )
                .expect("failed to load GDExtension function `string_name_operator_less`")
            },
            string_name_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, 0i32)
                    .expect("failed to load GDExtension function `string_name_construct_default`")
            },
            string_name_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, 1i32)
                    .expect("failed to load GDExtension function `string_name_construct_copy`")
            },
            string_name_from_string: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME, 2i32)
                    .expect("failed to load GDExtension function `string_name_from_string`")
            },
            string_name_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_STRING_NAME).unwrap()
            },
            node_path_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH)
                    .expect("failed to load GDExtension function `node_path_to_variant`")
            },
            node_path_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH)
                    .expect("failed to load GDExtension function `node_path_from_variant`")
            },
            node_path_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH,
                    crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH,
                )
                .expect("failed to load GDExtension function `node_path_operator_equal`")
            },
            node_path_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, 0i32)
                    .expect("failed to load GDExtension function `node_path_construct_default`")
            },
            node_path_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, 1i32)
                    .expect("failed to load GDExtension function `node_path_construct_copy`")
            },
            node_path_from_string: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH, 2i32)
                    .expect("failed to load GDExtension function `node_path_from_string`")
            },
            node_path_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_NODE_PATH).unwrap()
            },
            rid_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RID)
                    .expect("failed to load GDExtension function `rid_to_variant`")
            },
            rid_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RID)
                    .expect("failed to load GDExtension function `rid_from_variant`")
            },
            rid_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_RID,
                    crate::GDEXTENSION_VARIANT_TYPE_RID,
                )
                .expect("failed to load GDExtension function `rid_operator_equal`")
            },
            rid_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_RID,
                    crate::GDEXTENSION_VARIANT_TYPE_RID,
                )
                .expect("failed to load GDExtension function `rid_operator_less`")
            },
            rid_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RID, 0i32)
                    .expect("failed to load GDExtension function `rid_construct_default`")
            },
            rid_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_RID, 1i32)
                    .expect("failed to load GDExtension function `rid_construct_copy`")
            },
            object_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_OBJECT)
                    .expect("failed to load GDExtension function `object_to_variant`")
            },
            object_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_OBJECT)
                    .expect("failed to load GDExtension function `object_from_variant`")
            },
            callable_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE)
                    .expect("failed to load GDExtension function `callable_to_variant`")
            },
            callable_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE)
                    .expect("failed to load GDExtension function `callable_from_variant`")
            },
            callable_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_CALLABLE,
                    crate::GDEXTENSION_VARIANT_TYPE_CALLABLE,
                )
                .expect("failed to load GDExtension function `callable_operator_equal`")
            },
            callable_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, 0i32)
                    .expect("failed to load GDExtension function `callable_construct_default`")
            },
            callable_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, 1i32)
                    .expect("failed to load GDExtension function `callable_construct_copy`")
            },
            callable_from_object_method: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE, 2i32)
                    .expect("failed to load GDExtension function `callable_from_object_method`")
            },
            callable_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_CALLABLE).unwrap()
            },
            signal_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL)
                    .expect("failed to load GDExtension function `signal_to_variant`")
            },
            signal_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL)
                    .expect("failed to load GDExtension function `signal_from_variant`")
            },
            signal_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_SIGNAL,
                    crate::GDEXTENSION_VARIANT_TYPE_SIGNAL,
                )
                .expect("failed to load GDExtension function `signal_operator_equal`")
            },
            signal_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, 0i32)
                    .expect("failed to load GDExtension function `signal_construct_default`")
            },
            signal_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, 1i32)
                    .expect("failed to load GDExtension function `signal_construct_copy`")
            },
            signal_from_object_signal: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL, 2i32)
                    .expect("failed to load GDExtension function `signal_from_object_signal`")
            },
            signal_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_SIGNAL).unwrap()
            },
            dictionary_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY)
                    .expect("failed to load GDExtension function `dictionary_to_variant`")
            },
            dictionary_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY)
                    .expect("failed to load GDExtension function `dictionary_from_variant`")
            },
            dictionary_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY,
                    crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY,
                )
                .expect("failed to load GDExtension function `dictionary_operator_equal`")
            },
            dictionary_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, 0i32)
                    .expect("failed to load GDExtension function `dictionary_construct_default`")
            },
            dictionary_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY, 1i32)
                    .expect("failed to load GDExtension function `dictionary_construct_copy`")
            },
            dictionary_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_DICTIONARY).unwrap()
            },
            array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                    .expect("failed to load GDExtension function `array_to_variant`")
            },
            array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY)
                    .expect("failed to load GDExtension function `array_from_variant`")
            },
            array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_ARRAY,
                )
                .expect("failed to load GDExtension function `array_operator_equal`")
            },
            array_operator_less: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_LESS,
                    crate::GDEXTENSION_VARIANT_TYPE_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_ARRAY,
                )
                .expect("failed to load GDExtension function `array_operator_less`")
            },
            array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 0i32)
                    .expect("failed to load GDExtension function `array_construct_default`")
            },
            array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 1i32)
                    .expect("failed to load GDExtension function `array_construct_copy`")
            },
            array_from_base_type_class_name_script: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 2i32).expect(
                    "failed to load GDExtension function `array_from_base_type_class_name_script`",
                )
            },
            array_from_packed_byte_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 3i32)
                    .expect("failed to load GDExtension function `array_from_packed_byte_array`")
            },
            array_from_packed_int32_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 4i32)
                    .expect("failed to load GDExtension function `array_from_packed_int32_array`")
            },
            array_from_packed_int64_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 5i32)
                    .expect("failed to load GDExtension function `array_from_packed_int64_array`")
            },
            array_from_packed_float32_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 6i32)
                    .expect("failed to load GDExtension function `array_from_packed_float32_array`")
            },
            array_from_packed_float64_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 7i32)
                    .expect("failed to load GDExtension function `array_from_packed_float64_array`")
            },
            array_from_packed_string_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 8i32)
                    .expect("failed to load GDExtension function `array_from_packed_string_array`")
            },
            array_from_packed_vector2_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 9i32)
                    .expect("failed to load GDExtension function `array_from_packed_vector2_array`")
            },
            array_from_packed_vector3_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 10i32)
                    .expect("failed to load GDExtension function `array_from_packed_vector3_array`")
            },
            array_from_packed_color_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY, 11i32)
                    .expect("failed to load GDExtension function `array_from_packed_color_array`")
            },
            array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_ARRAY).unwrap()
            },
            packed_byte_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY)
                    .expect("failed to load GDExtension function `packed_byte_array_to_variant`")
            },
            packed_byte_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY)
                    .expect("failed to load GDExtension function `packed_byte_array_from_variant`")
            },
            packed_byte_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_byte_array_operator_equal`")
            },
            packed_byte_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_byte_array_construct_default`",
                )
            },
            packed_byte_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_byte_array_construct_copy`",
                )
            },
            packed_byte_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_byte_array_from_array`")
            },
            packed_byte_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_BYTE_ARRAY).unwrap()
            },
            packed_int32_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY)
                    .expect("failed to load GDExtension function `packed_int32_array_to_variant`")
            },
            packed_int32_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY)
                    .expect("failed to load GDExtension function `packed_int32_array_from_variant`")
            },
            packed_int32_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_int32_array_operator_equal`")
            },
            packed_int32_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_int32_array_construct_default`",
                )
            },
            packed_int32_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_int32_array_construct_copy`",
                )
            },
            packed_int32_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_int32_array_from_array`")
            },
            packed_int32_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT32_ARRAY).unwrap()
            },
            packed_int64_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY)
                    .expect("failed to load GDExtension function `packed_int64_array_to_variant`")
            },
            packed_int64_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY)
                    .expect("failed to load GDExtension function `packed_int64_array_from_variant`")
            },
            packed_int64_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_int64_array_operator_equal`")
            },
            packed_int64_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_int64_array_construct_default`",
                )
            },
            packed_int64_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_int64_array_construct_copy`",
                )
            },
            packed_int64_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_int64_array_from_array`")
            },
            packed_int64_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_INT64_ARRAY).unwrap()
            },
            packed_float32_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY)
                    .expect("failed to load GDExtension function `packed_float32_array_to_variant`")
            },
            packed_float32_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY).expect(
                    "failed to load GDExtension function `packed_float32_array_from_variant`",
                )
            },
            packed_float32_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_float32_array_operator_equal`")
            },
            packed_float32_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_float32_array_construct_default`",
                )
            },
            packed_float32_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_float32_array_construct_copy`",
                )
            },
            packed_float32_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_float32_array_from_array`")
            },
            packed_float32_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT32_ARRAY).unwrap()
            },
            packed_float64_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY)
                    .expect("failed to load GDExtension function `packed_float64_array_to_variant`")
            },
            packed_float64_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY).expect(
                    "failed to load GDExtension function `packed_float64_array_from_variant`",
                )
            },
            packed_float64_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_float64_array_operator_equal`")
            },
            packed_float64_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_float64_array_construct_default`",
                )
            },
            packed_float64_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_float64_array_construct_copy`",
                )
            },
            packed_float64_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_float64_array_from_array`")
            },
            packed_float64_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_FLOAT64_ARRAY).unwrap()
            },
            packed_string_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY)
                    .expect("failed to load GDExtension function `packed_string_array_to_variant`")
            },
            packed_string_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY).expect(
                    "failed to load GDExtension function `packed_string_array_from_variant`",
                )
            },
            packed_string_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_string_array_operator_equal`")
            },
            packed_string_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_string_array_construct_default`",
                )
            },
            packed_string_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_string_array_construct_copy`",
                )
            },
            packed_string_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_string_array_from_array`")
            },
            packed_string_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_STRING_ARRAY).unwrap()
            },
            packed_vector2_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY)
                    .expect("failed to load GDExtension function `packed_vector2_array_to_variant`")
            },
            packed_vector2_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY).expect(
                    "failed to load GDExtension function `packed_vector2_array_from_variant`",
                )
            },
            packed_vector2_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_vector2_array_operator_equal`")
            },
            packed_vector2_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_vector2_array_construct_default`",
                )
            },
            packed_vector2_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_vector2_array_construct_copy`",
                )
            },
            packed_vector2_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_vector2_array_from_array`")
            },
            packed_vector2_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR2_ARRAY).unwrap()
            },
            packed_vector3_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY)
                    .expect("failed to load GDExtension function `packed_vector3_array_to_variant`")
            },
            packed_vector3_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY).expect(
                    "failed to load GDExtension function `packed_vector3_array_from_variant`",
                )
            },
            packed_vector3_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_vector3_array_operator_equal`")
            },
            packed_vector3_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_vector3_array_construct_default`",
                )
            },
            packed_vector3_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_vector3_array_construct_copy`",
                )
            },
            packed_vector3_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_vector3_array_from_array`")
            },
            packed_vector3_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_VECTOR3_ARRAY).unwrap()
            },
            packed_color_array_to_variant: {
                let ctor_fn = interface.get_variant_from_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY)
                    .expect("failed to load GDExtension function `packed_color_array_to_variant`")
            },
            packed_color_array_from_variant: {
                let ctor_fn = interface.get_variant_to_type_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY)
                    .expect("failed to load GDExtension function `packed_color_array_from_variant`")
            },
            packed_color_array_operator_equal: {
                let op_finder = interface.variant_get_ptr_operator_evaluator.unwrap();
                op_finder(
                    crate::GDEXTENSION_VARIANT_OP_EQUAL,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY,
                    crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY,
                )
                .expect("failed to load GDExtension function `packed_color_array_operator_equal`")
            },
            packed_color_array_construct_default: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, 0i32).expect(
                    "failed to load GDExtension function `packed_color_array_construct_default`",
                )
            },
            packed_color_array_construct_copy: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, 1i32).expect(
                    "failed to load GDExtension function `packed_color_array_construct_copy`",
                )
            },
            packed_color_array_from_array: {
                let ctor_fn = interface.variant_get_ptr_constructor.unwrap();
                ctor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY, 2i32)
                    .expect("failed to load GDExtension function `packed_color_array_from_array`")
            },
            packed_color_array_destroy: {
                let dtor_fn = interface.variant_get_ptr_destructor.unwrap();
                dtor_fn(crate::GDEXTENSION_VARIANT_TYPE_PACKED_COLOR_ARRAY).unwrap()
            },
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(i32)]
pub enum VariantType {
    Nil = 0,
    Bool = 1,
    Int = 2,
    Float = 3,
    String = 4,
    Vector2 = 5,
    Vector2i = 6,
    Rect2 = 7,
    Rect2i = 8,
    Vector3 = 9,
    Vector3i = 10,
    Transform2D = 11,
    Vector4 = 12,
    Vector4i = 13,
    Plane = 14,
    Quaternion = 15,
    Aabb = 16,
    Basis = 17,
    Transform3D = 18,
    Projection = 19,
    Color = 20,
    StringName = 21,
    NodePath = 22,
    Rid = 23,
    Object = 24,
    Callable = 25,
    Signal = 26,
    Dictionary = 27,
    Array = 28,
    PackedByteArray = 29,
    PackedInt32Array = 30,
    PackedInt64Array = 31,
    PackedFloat32Array = 32,
    PackedFloat64Array = 33,
    PackedStringArray = 34,
    PackedVector2Array = 35,
    PackedVector3Array = 36,
    PackedColorArray = 37,
}
impl VariantType {
    #[doc(hidden)]
    pub fn from_sys(enumerator: crate::GDExtensionVariantType) -> Self {
        match enumerator {
            0 => Self::Nil,
            1 => Self::Bool,
            2 => Self::Int,
            3 => Self::Float,
            4 => Self::String,
            5 => Self::Vector2,
            6 => Self::Vector2i,
            7 => Self::Rect2,
            8 => Self::Rect2i,
            9 => Self::Vector3,
            10 => Self::Vector3i,
            11 => Self::Transform2D,
            12 => Self::Vector4,
            13 => Self::Vector4i,
            14 => Self::Plane,
            15 => Self::Quaternion,
            16 => Self::Aabb,
            17 => Self::Basis,
            18 => Self::Transform3D,
            19 => Self::Projection,
            20 => Self::Color,
            21 => Self::StringName,
            22 => Self::NodePath,
            23 => Self::Rid,
            24 => Self::Object,
            25 => Self::Callable,
            26 => Self::Signal,
            27 => Self::Dictionary,
            28 => Self::Array,
            29 => Self::PackedByteArray,
            30 => Self::PackedInt32Array,
            31 => Self::PackedInt64Array,
            32 => Self::PackedFloat32Array,
            33 => Self::PackedFloat64Array,
            34 => Self::PackedStringArray,
            35 => Self::PackedVector2Array,
            36 => Self::PackedVector3Array,
            37 => Self::PackedColorArray,
            _ => unreachable!("invalid variant type {}", enumerator),
        }
    }
    #[doc(hidden)]
    pub fn sys(self) -> crate::GDExtensionVariantType {
        self as _
    }
}
unsafe impl GodotFfi for VariantType {
    ffi_methods! { type GDExtensionTypePtr = * mut Self ; .. }
}
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[repr(i32)]
pub enum VariantOperator {
    Equal = 0,
    NotEqual = 1,
    Less = 2,
    LessEqual = 3,
    Greater = 4,
    GreaterEqual = 5,
    Add = 6,
    Subtract = 7,
    Multiply = 8,
    Divide = 9,
    Negate = 10,
    Positive = 11,
    Module = 12,
    Power = 13,
    ShiftLeft = 14,
    ShiftRight = 15,
    BitAnd = 16,
    BitOr = 17,
    BitXor = 18,
    BitNegate = 19,
    And = 20,
    Or = 21,
    Xor = 22,
    Not = 23,
    In = 24,
}
impl VariantOperator {
    #[doc(hidden)]
    pub fn from_sys(enumerator: crate::GDExtensionVariantOperator) -> Self {
        match enumerator {
            0 => Self::Equal,
            1 => Self::NotEqual,
            2 => Self::Less,
            3 => Self::LessEqual,
            4 => Self::Greater,
            5 => Self::GreaterEqual,
            6 => Self::Add,
            7 => Self::Subtract,
            8 => Self::Multiply,
            9 => Self::Divide,
            10 => Self::Negate,
            11 => Self::Positive,
            12 => Self::Module,
            13 => Self::Power,
            14 => Self::ShiftLeft,
            15 => Self::ShiftRight,
            16 => Self::BitAnd,
            17 => Self::BitOr,
            18 => Self::BitXor,
            19 => Self::BitNegate,
            20 => Self::And,
            21 => Self::Or,
            22 => Self::Xor,
            23 => Self::Not,
            24 => Self::In,
            _ => unreachable!("invalid variant operator {}", enumerator),
        }
    }
    #[doc(hidden)]
    pub fn sys(self) -> crate::GDExtensionVariantOperator {
        self as _
    }
}
unsafe impl GodotFfi for VariantOperator {
    ffi_methods! { type GDExtensionTypePtr = * mut Self ; .. }
}
