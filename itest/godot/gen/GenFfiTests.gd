# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

extends TestSuite

# Note: GDScript only uses ptrcalls if it has the full type information available at "compile" (parse) time.
# That includes all arguments (including receiver) as well as function signature (parameters + return type).
# Otherwise, GDScript will use varcall. Both are tested below.
# It is thus important that `ffi` is initialized using = for varcalls, and using := for ptrcalls.


func test_varcall_i64():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i64()
	assert_that(ffi.accept_i64(from_rust), "ffi.accept_i64(from_rust)")

	var from_gdscript: Variant = -922337203685477580
	var mirrored: Variant = ffi.mirror_i64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_i32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i32()
	assert_that(ffi.accept_i32(from_rust), "ffi.accept_i32(from_rust)")

	var from_gdscript: Variant = -2147483648
	var mirrored: Variant = ffi.mirror_i32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_u32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_u32()
	assert_that(ffi.accept_u32(from_rust), "ffi.accept_u32(from_rust)")

	var from_gdscript: Variant = 4294967295
	var mirrored: Variant = ffi.mirror_u32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_i16():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i16()
	assert_that(ffi.accept_i16(from_rust), "ffi.accept_i16(from_rust)")

	var from_gdscript: Variant = -32767
	var mirrored: Variant = ffi.mirror_i16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_u16():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_u16()
	assert_that(ffi.accept_u16(from_rust), "ffi.accept_u16(from_rust)")

	var from_gdscript: Variant = 65535
	var mirrored: Variant = ffi.mirror_u16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_i8():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_i8()
	assert_that(ffi.accept_i8(from_rust), "ffi.accept_i8(from_rust)")

	var from_gdscript: Variant = -128
	var mirrored: Variant = ffi.mirror_i8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_u8():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_u8()
	assert_that(ffi.accept_u8(from_rust), "ffi.accept_u8(from_rust)")

	var from_gdscript: Variant = 255
	var mirrored: Variant = ffi.mirror_u8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_f32():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_f32()
	assert_that(ffi.accept_f32(from_rust), "ffi.accept_f32(from_rust)")

	var from_gdscript: Variant = 12.5
	var mirrored: Variant = ffi.mirror_f32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_f64():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_f64()
	assert_that(ffi.accept_f64(from_rust), "ffi.accept_f64(from_rust)")

	var from_gdscript: Variant = 127.83156478
	var mirrored: Variant = ffi.mirror_f64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_bool():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_bool()
	assert_that(ffi.accept_bool(from_rust), "ffi.accept_bool(from_rust)")

	var from_gdscript: Variant = true
	var mirrored: Variant = ffi.mirror_bool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_color():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_color()
	assert_that(ffi.accept_color(from_rust), "ffi.accept_color(from_rust)")

	var from_gdscript: Variant = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Variant = ffi.mirror_color(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_godotstring():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_godotstring()
	assert_that(ffi.accept_godotstring(from_rust), "ffi.accept_godotstring(from_rust)")

	var from_gdscript: Variant = "hello"
	var mirrored: Variant = ffi.mirror_godotstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_stringname():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_stringname()
	assert_that(ffi.accept_stringname(from_rust), "ffi.accept_stringname(from_rust)")

	var from_gdscript: Variant = &"hello"
	var mirrored: Variant = ffi.mirror_stringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_nodepath():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_nodepath()
	assert_that(ffi.accept_nodepath(from_rust), "ffi.accept_nodepath(from_rust)")

	var from_gdscript: Variant = ^"hello"
	var mirrored: Variant = ffi.mirror_nodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector2():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector2()
	assert_that(ffi.accept_vector2(from_rust), "ffi.accept_vector2(from_rust)")

	var from_gdscript: Variant = Vector2(12.5, -3.5)
	var mirrored: Variant = ffi.mirror_vector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector3():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector3()
	assert_that(ffi.accept_vector3(from_rust), "ffi.accept_vector3(from_rust)")

	var from_gdscript: Variant = Vector3(117.5, 100.0, -323.25)
	var mirrored: Variant = ffi.mirror_vector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector4():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector4()
	assert_that(ffi.accept_vector4(from_rust), "ffi.accept_vector4(from_rust)")

	var from_gdscript: Variant = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Variant = ffi.mirror_vector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector2i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector2i()
	assert_that(ffi.accept_vector2i(from_rust), "ffi.accept_vector2i(from_rust)")

	var from_gdscript: Variant = Vector2i(-2147483648, 2147483647)
	var mirrored: Variant = ffi.mirror_vector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_vector3i():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_vector3i()
	assert_that(ffi.accept_vector3i(from_rust), "ffi.accept_vector3i(from_rust)")

	var from_gdscript: Variant = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Variant = ffi.mirror_vector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_variantarray():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_variantarray()
	assert_that(ffi.accept_variantarray(from_rust), "ffi.accept_variantarray(from_rust)")

	var from_gdscript: Variant = [-7, "godot", false, Vector2i(-77, 88)]
	var mirrored: Variant = ffi.mirror_variantarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_dictionary():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_dictionary()
	assert_that(ffi.accept_dictionary(from_rust), "ffi.accept_dictionary(from_rust)")

	var from_gdscript: Variant = {"key": 83, -3: Vector2(1, 2), 0.03: true}
	var mirrored: Variant = ffi.mirror_dictionary(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_instanceid():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_instanceid()
	assert_that(ffi.accept_instanceid(from_rust), "ffi.accept_instanceid(from_rust)")

	var from_gdscript: Variant = -1
	var mirrored: Variant = ffi.mirror_instanceid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_varcall_variant():
	var ffi = GenFfi.new()

	var from_rust: Variant = ffi.return_variant()
	assert_that(ffi.accept_variant(from_rust), "ffi.accept_variant(from_rust)")

	var from_gdscript: Variant = 123
	var mirrored: Variant = ffi.mirror_variant(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")



func test_ptrcall_i64():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i64()
	assert_that(ffi.accept_i64(from_rust), "ffi.accept_i64(from_rust)")

	var from_gdscript: int = -922337203685477580
	var mirrored: int = ffi.mirror_i64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_i32():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i32()
	assert_that(ffi.accept_i32(from_rust), "ffi.accept_i32(from_rust)")

	var from_gdscript: int = -2147483648
	var mirrored: int = ffi.mirror_i32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_u32():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_u32()
	assert_that(ffi.accept_u32(from_rust), "ffi.accept_u32(from_rust)")

	var from_gdscript: int = 4294967295
	var mirrored: int = ffi.mirror_u32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_i16():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i16()
	assert_that(ffi.accept_i16(from_rust), "ffi.accept_i16(from_rust)")

	var from_gdscript: int = -32767
	var mirrored: int = ffi.mirror_i16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_u16():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_u16()
	assert_that(ffi.accept_u16(from_rust), "ffi.accept_u16(from_rust)")

	var from_gdscript: int = 65535
	var mirrored: int = ffi.mirror_u16(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_i8():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_i8()
	assert_that(ffi.accept_i8(from_rust), "ffi.accept_i8(from_rust)")

	var from_gdscript: int = -128
	var mirrored: int = ffi.mirror_i8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_u8():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_u8()
	assert_that(ffi.accept_u8(from_rust), "ffi.accept_u8(from_rust)")

	var from_gdscript: int = 255
	var mirrored: int = ffi.mirror_u8(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_f32():
	var ffi := GenFfi.new()

	var from_rust: float = ffi.return_f32()
	assert_that(ffi.accept_f32(from_rust), "ffi.accept_f32(from_rust)")

	var from_gdscript: float = 12.5
	var mirrored: float = ffi.mirror_f32(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_f64():
	var ffi := GenFfi.new()

	var from_rust: float = ffi.return_f64()
	assert_that(ffi.accept_f64(from_rust), "ffi.accept_f64(from_rust)")

	var from_gdscript: float = 127.83156478
	var mirrored: float = ffi.mirror_f64(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_bool():
	var ffi := GenFfi.new()

	var from_rust: bool = ffi.return_bool()
	assert_that(ffi.accept_bool(from_rust), "ffi.accept_bool(from_rust)")

	var from_gdscript: bool = true
	var mirrored: bool = ffi.mirror_bool(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_color():
	var ffi := GenFfi.new()

	var from_rust: Color = ffi.return_color()
	assert_that(ffi.accept_color(from_rust), "ffi.accept_color(from_rust)")

	var from_gdscript: Color = Color(0.7, 0.5, 0.3, 0.2)
	var mirrored: Color = ffi.mirror_color(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_godotstring():
	var ffi := GenFfi.new()

	var from_rust: String = ffi.return_godotstring()
	assert_that(ffi.accept_godotstring(from_rust), "ffi.accept_godotstring(from_rust)")

	var from_gdscript: String = "hello"
	var mirrored: String = ffi.mirror_godotstring(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_stringname():
	var ffi := GenFfi.new()

	var from_rust: StringName = ffi.return_stringname()
	assert_that(ffi.accept_stringname(from_rust), "ffi.accept_stringname(from_rust)")

	var from_gdscript: StringName = &"hello"
	var mirrored: StringName = ffi.mirror_stringname(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_nodepath():
	var ffi := GenFfi.new()

	var from_rust: NodePath = ffi.return_nodepath()
	assert_that(ffi.accept_nodepath(from_rust), "ffi.accept_nodepath(from_rust)")

	var from_gdscript: NodePath = ^"hello"
	var mirrored: NodePath = ffi.mirror_nodepath(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector2():
	var ffi := GenFfi.new()

	var from_rust: Vector2 = ffi.return_vector2()
	assert_that(ffi.accept_vector2(from_rust), "ffi.accept_vector2(from_rust)")

	var from_gdscript: Vector2 = Vector2(12.5, -3.5)
	var mirrored: Vector2 = ffi.mirror_vector2(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector3():
	var ffi := GenFfi.new()

	var from_rust: Vector3 = ffi.return_vector3()
	assert_that(ffi.accept_vector3(from_rust), "ffi.accept_vector3(from_rust)")

	var from_gdscript: Vector3 = Vector3(117.5, 100.0, -323.25)
	var mirrored: Vector3 = ffi.mirror_vector3(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector4():
	var ffi := GenFfi.new()

	var from_rust: Vector4 = ffi.return_vector4()
	assert_that(ffi.accept_vector4(from_rust), "ffi.accept_vector4(from_rust)")

	var from_gdscript: Vector4 = Vector4(-18.5, 24.75, -1.25, 777.875)
	var mirrored: Vector4 = ffi.mirror_vector4(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector2i():
	var ffi := GenFfi.new()

	var from_rust: Vector2i = ffi.return_vector2i()
	assert_that(ffi.accept_vector2i(from_rust), "ffi.accept_vector2i(from_rust)")

	var from_gdscript: Vector2i = Vector2i(-2147483648, 2147483647)
	var mirrored: Vector2i = ffi.mirror_vector2i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_vector3i():
	var ffi := GenFfi.new()

	var from_rust: Vector3i = ffi.return_vector3i()
	assert_that(ffi.accept_vector3i(from_rust), "ffi.accept_vector3i(from_rust)")

	var from_gdscript: Vector3i = Vector3i(-1, -2147483648, 2147483647)
	var mirrored: Vector3i = ffi.mirror_vector3i(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_variantarray():
	var ffi := GenFfi.new()

	var from_rust: Array = ffi.return_variantarray()
	assert_that(ffi.accept_variantarray(from_rust), "ffi.accept_variantarray(from_rust)")

	var from_gdscript: Array = [-7, "godot", false, Vector2i(-77, 88)]
	var mirrored: Array = ffi.mirror_variantarray(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_dictionary():
	var ffi := GenFfi.new()

	var from_rust: Dictionary = ffi.return_dictionary()
	assert_that(ffi.accept_dictionary(from_rust), "ffi.accept_dictionary(from_rust)")

	var from_gdscript: Dictionary = {"key": 83, -3: Vector2(1, 2), 0.03: true}
	var mirrored: Dictionary = ffi.mirror_dictionary(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_instanceid():
	var ffi := GenFfi.new()

	var from_rust: int = ffi.return_instanceid()
	assert_that(ffi.accept_instanceid(from_rust), "ffi.accept_instanceid(from_rust)")

	var from_gdscript: int = -1
	var mirrored: int = ffi.mirror_instanceid(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

func test_ptrcall_variant():
	var ffi := GenFfi.new()

	var from_rust: Variant = ffi.return_variant()
	assert_that(ffi.accept_variant(from_rust), "ffi.accept_variant(from_rust)")

	var from_gdscript: Variant = 123
	var mirrored: Variant = ffi.mirror_variant(from_gdscript)
	assert_eq(mirrored, from_gdscript, "mirrored == from_gdscript")

