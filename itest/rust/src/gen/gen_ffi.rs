use godot::builtin::*;
use godot::obj::InstanceId;
#[derive(godot :: bind :: GodotClass)]
#[class(init)]
struct GenFfi {}
#[allow(clippy::bool_comparison)]
#[godot::bind::godot_api]
impl GenFfi {
    #[func]
    fn return_i64(&self) -> i64 {
        -922337203685477580
    }
    #[func]
    fn accept_i64(&self, i: i64) -> bool {
        i == -922337203685477580
    }
    #[func]
    fn mirror_i64(&self, i: i64) -> i64 {
        i
    }
    #[func]
    fn return_i32(&self) -> i32 {
        -2147483648
    }
    #[func]
    fn accept_i32(&self, i: i32) -> bool {
        i == -2147483648
    }
    #[func]
    fn mirror_i32(&self, i: i32) -> i32 {
        i
    }
    #[func]
    fn return_u32(&self) -> u32 {
        4294967295
    }
    #[func]
    fn accept_u32(&self, i: u32) -> bool {
        i == 4294967295
    }
    #[func]
    fn mirror_u32(&self, i: u32) -> u32 {
        i
    }
    #[func]
    fn return_i16(&self) -> i16 {
        -32767
    }
    #[func]
    fn accept_i16(&self, i: i16) -> bool {
        i == -32767
    }
    #[func]
    fn mirror_i16(&self, i: i16) -> i16 {
        i
    }
    #[func]
    fn return_u16(&self) -> u16 {
        65535
    }
    #[func]
    fn accept_u16(&self, i: u16) -> bool {
        i == 65535
    }
    #[func]
    fn mirror_u16(&self, i: u16) -> u16 {
        i
    }
    #[func]
    fn return_i8(&self) -> i8 {
        -128
    }
    #[func]
    fn accept_i8(&self, i: i8) -> bool {
        i == -128
    }
    #[func]
    fn mirror_i8(&self, i: i8) -> i8 {
        i
    }
    #[func]
    fn return_u8(&self) -> u8 {
        255
    }
    #[func]
    fn accept_u8(&self, i: u8) -> bool {
        i == 255
    }
    #[func]
    fn mirror_u8(&self, i: u8) -> u8 {
        i
    }
    #[func]
    fn return_f32(&self) -> f32 {
        12.5
    }
    #[func]
    fn accept_f32(&self, i: f32) -> bool {
        i == 12.5
    }
    #[func]
    fn mirror_f32(&self, i: f32) -> f32 {
        i
    }
    #[func]
    fn return_f64(&self) -> f64 {
        127.83156478
    }
    #[func]
    fn accept_f64(&self, i: f64) -> bool {
        i == 127.83156478
    }
    #[func]
    fn mirror_f64(&self, i: f64) -> f64 {
        i
    }
    #[func]
    fn return_bool(&self) -> bool {
        true
    }
    #[func]
    fn accept_bool(&self, i: bool) -> bool {
        i == true
    }
    #[func]
    fn mirror_bool(&self, i: bool) -> bool {
        i
    }
    #[func]
    fn return_color(&self) -> Color {
        Color::from_rgba(0.7, 0.5, 0.3, 0.2)
    }
    #[func]
    fn accept_color(&self, i: Color) -> bool {
        i == Color::from_rgba(0.7, 0.5, 0.3, 0.2)
    }
    #[func]
    fn mirror_color(&self, i: Color) -> Color {
        i
    }
    #[func]
    fn return_godotstring(&self) -> GodotString {
        "hello".into()
    }
    #[func]
    fn accept_godotstring(&self, i: GodotString) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_godotstring(&self, i: GodotString) -> GodotString {
        i
    }
    #[func]
    fn return_stringname(&self) -> StringName {
        "hello".into()
    }
    #[func]
    fn accept_stringname(&self, i: StringName) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_stringname(&self, i: StringName) -> StringName {
        i
    }
    #[func]
    fn return_nodepath(&self) -> NodePath {
        "hello".into()
    }
    #[func]
    fn accept_nodepath(&self, i: NodePath) -> bool {
        i == "hello".into()
    }
    #[func]
    fn mirror_nodepath(&self, i: NodePath) -> NodePath {
        i
    }
    #[func]
    fn return_vector2(&self) -> Vector2 {
        Vector2::new(12.5, -3.5)
    }
    #[func]
    fn accept_vector2(&self, i: Vector2) -> bool {
        i == Vector2::new(12.5, -3.5)
    }
    #[func]
    fn mirror_vector2(&self, i: Vector2) -> Vector2 {
        i
    }
    #[func]
    fn return_vector3(&self) -> Vector3 {
        Vector3::new(117.5, 100.0, -323.25)
    }
    #[func]
    fn accept_vector3(&self, i: Vector3) -> bool {
        i == Vector3::new(117.5, 100.0, -323.25)
    }
    #[func]
    fn mirror_vector3(&self, i: Vector3) -> Vector3 {
        i
    }
    #[func]
    fn return_vector4(&self) -> Vector4 {
        Vector4::new(-18.5, 24.75, -1.25, 777.875)
    }
    #[func]
    fn accept_vector4(&self, i: Vector4) -> bool {
        i == Vector4::new(-18.5, 24.75, -1.25, 777.875)
    }
    #[func]
    fn mirror_vector4(&self, i: Vector4) -> Vector4 {
        i
    }
    #[func]
    fn return_vector2i(&self) -> Vector2i {
        Vector2i::new(-2147483648, 2147483647)
    }
    #[func]
    fn accept_vector2i(&self, i: Vector2i) -> bool {
        i == Vector2i::new(-2147483648, 2147483647)
    }
    #[func]
    fn mirror_vector2i(&self, i: Vector2i) -> Vector2i {
        i
    }
    #[func]
    fn return_vector3i(&self) -> Vector3i {
        Vector3i::new(-1, -2147483648, 2147483647)
    }
    #[func]
    fn accept_vector3i(&self, i: Vector3i) -> bool {
        i == Vector3i::new(-1, -2147483648, 2147483647)
    }
    #[func]
    fn mirror_vector3i(&self, i: Vector3i) -> Vector3i {
        i
    }
    #[func]
    fn return_variantarray(&self) -> VariantArray {
        varray![-7, "godot", false, Vector2i::new(-77, 88)]
    }
    #[func]
    fn accept_variantarray(&self, i: VariantArray) -> bool {
        i == varray![-7, "godot", false, Vector2i::new(-77, 88)]
    }
    #[func]
    fn mirror_variantarray(&self, i: VariantArray) -> VariantArray {
        i
    }
    #[func]
    fn return_dictionary(&self) -> Dictionary {
        dict! { "key" : 83 , (- 3) : Vector2 :: new (1.0 , 2.0) , 0.03 : true }
    }
    #[func]
    fn accept_dictionary(&self, i: Dictionary) -> bool {
        i == dict! { "key" : 83 , (- 3) : Vector2 :: new (1.0 , 2.0) , 0.03 : true }
    }
    #[func]
    fn mirror_dictionary(&self, i: Dictionary) -> Dictionary {
        i
    }
    #[func]
    fn return_instanceid(&self) -> InstanceId {
        InstanceId::from_nonzero(0xFFFFFFFFFFFFFFF)
    }
    #[func]
    fn accept_instanceid(&self, i: InstanceId) -> bool {
        i == InstanceId::from_nonzero(0xFFFFFFFFFFFFFFF)
    }
    #[func]
    fn mirror_instanceid(&self, i: InstanceId) -> InstanceId {
        i
    }
    #[func]
    fn return_variant(&self) -> Variant {
        123i64.to_variant()
    }
    #[func]
    fn accept_variant(&self, i: Variant) -> bool {
        i == 123i64.to_variant()
    }
    #[func]
    fn mirror_variant(&self, i: Variant) -> Variant {
        i
    }
}
