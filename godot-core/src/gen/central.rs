use crate::builtin::*;
use crate::engine::Object;
use crate::obj::Gd;
#[allow(dead_code)]
pub enum VariantDispatch {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(GodotString),
    Vector2(Vector2),
    Vector2i(Vector2i),
    Rect2(Rect2),
    Rect2i(Rect2i),
    Vector3(Vector3),
    Vector3i(Vector3i),
    Transform2D(Transform2D),
    Vector4(Vector4),
    Vector4i(Vector4i),
    Plane(Plane),
    Quaternion(Quaternion),
    Aabb(Aabb),
    Basis(Basis),
    Transform3D(Transform3D),
    Projection(Projection),
    Color(Color),
    StringName(StringName),
    NodePath(NodePath),
    Rid(Rid),
    Object(Gd<Object>),
    Callable(Callable),
    Signal(Signal),
    Dictionary(Dictionary),
    Array(VariantArray),
    PackedByteArray(PackedByteArray),
    PackedInt32Array(PackedInt32Array),
    PackedInt64Array(PackedInt64Array),
    PackedFloat32Array(PackedFloat32Array),
    PackedFloat64Array(PackedFloat64Array),
    PackedStringArray(PackedStringArray),
    PackedVector2Array(PackedVector2Array),
    PackedVector3Array(PackedVector3Array),
    PackedColorArray(PackedColorArray),
}
#[cfg(FALSE)]
impl FromVariant for VariantDispatch {
    fn try_from_variant(variant: &Variant) -> Result<Self, VariantConversionError> {
        let dispatch = match variant.get_type() {
            VariantType::Nil => Self::Nil,
            VariantType::Bool => Self::Bool(variant.to::<bool>()),
            VariantType::Int => Self::Int(variant.to::<i64>()),
            VariantType::Float => Self::Float(variant.to::<f64>()),
            VariantType::String => Self::String(variant.to::<GodotString>()),
            VariantType::Vector2 => Self::Vector2(variant.to::<Vector2>()),
            VariantType::Vector2i => Self::Vector2i(variant.to::<Vector2i>()),
            VariantType::Rect2 => Self::Rect2(variant.to::<Rect2>()),
            VariantType::Rect2i => Self::Rect2i(variant.to::<Rect2i>()),
            VariantType::Vector3 => Self::Vector3(variant.to::<Vector3>()),
            VariantType::Vector3i => Self::Vector3i(variant.to::<Vector3i>()),
            VariantType::Transform2D => Self::Transform2D(variant.to::<Transform2D>()),
            VariantType::Vector4 => Self::Vector4(variant.to::<Vector4>()),
            VariantType::Vector4i => Self::Vector4i(variant.to::<Vector4i>()),
            VariantType::Plane => Self::Plane(variant.to::<Plane>()),
            VariantType::Quaternion => Self::Quaternion(variant.to::<Quaternion>()),
            VariantType::Aabb => Self::Aabb(variant.to::<Aabb>()),
            VariantType::Basis => Self::Basis(variant.to::<Basis>()),
            VariantType::Transform3D => Self::Transform3D(variant.to::<Transform3D>()),
            VariantType::Projection => Self::Projection(variant.to::<Projection>()),
            VariantType::Color => Self::Color(variant.to::<Color>()),
            VariantType::StringName => Self::StringName(variant.to::<StringName>()),
            VariantType::NodePath => Self::NodePath(variant.to::<NodePath>()),
            VariantType::Rid => Self::Rid(variant.to::<Rid>()),
            VariantType::Object => Self::Object(variant.to::<Gd<Object>>()),
            VariantType::Callable => Self::Callable(variant.to::<Callable>()),
            VariantType::Signal => Self::Signal(variant.to::<Signal>()),
            VariantType::Dictionary => Self::Dictionary(variant.to::<Dictionary>()),
            VariantType::Array => Self::Array(variant.to::<VariantArray>()),
            VariantType::PackedByteArray => Self::PackedByteArray(variant.to::<PackedByteArray>()),
            VariantType::PackedInt32Array => {
                Self::PackedInt32Array(variant.to::<PackedInt32Array>())
            }
            VariantType::PackedInt64Array => {
                Self::PackedInt64Array(variant.to::<PackedInt64Array>())
            }
            VariantType::PackedFloat32Array => {
                Self::PackedFloat32Array(variant.to::<PackedFloat32Array>())
            }
            VariantType::PackedFloat64Array => {
                Self::PackedFloat64Array(variant.to::<PackedFloat64Array>())
            }
            VariantType::PackedStringArray => {
                Self::PackedStringArray(variant.to::<PackedStringArray>())
            }
            VariantType::PackedVector2Array => {
                Self::PackedVector2Array(variant.to::<PackedVector2Array>())
            }
            VariantType::PackedVector3Array => {
                Self::PackedVector3Array(variant.to::<PackedVector3Array>())
            }
            VariantType::PackedColorArray => {
                Self::PackedColorArray(variant.to::<PackedColorArray>())
            }
        };
        Ok(dispatch)
    }
}
#[doc = r" Global enums and constants."]
#[doc = r""]
#[doc = r" A list of global-scope enumerated constants."]
#[doc = r" For global built-in functions, check out the [`utilities` module][crate::engine::utilities]."]
#[doc = r""]
#[doc = r" See also [Godot docs for `@GlobalScope`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#enumerations)."]
pub mod global {
    use crate::sys;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Side {
        ord: i32,
    }
    impl Side {
        pub const SIDE_LEFT: Self = Self { ord: 0 };
        pub const SIDE_TOP: Self = Self { ord: 1 };
        pub const SIDE_RIGHT: Self = Self { ord: 2 };
        pub const SIDE_BOTTOM: Self = Self { ord: 3 };
    }
    impl crate::obj::EngineEnum for Side {
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
    unsafe impl sys::GodotFfi for Side {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Corner {
        ord: i32,
    }
    impl Corner {
        pub const CORNER_TOP_LEFT: Self = Self { ord: 0 };
        pub const CORNER_TOP_RIGHT: Self = Self { ord: 1 };
        pub const CORNER_BOTTOM_RIGHT: Self = Self { ord: 2 };
        pub const CORNER_BOTTOM_LEFT: Self = Self { ord: 3 };
    }
    impl crate::obj::EngineEnum for Corner {
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
    unsafe impl sys::GodotFfi for Corner {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Orientation {
        ord: i32,
    }
    impl Orientation {
        pub const VERTICAL: Self = Self { ord: 1 };
        pub const HORIZONTAL: Self = Self { ord: 0 };
    }
    impl crate::obj::EngineEnum for Orientation {
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
    unsafe impl sys::GodotFfi for Orientation {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct ClockDirection {
        ord: i32,
    }
    impl ClockDirection {
        pub const CLOCKWISE: Self = Self { ord: 0 };
        pub const COUNTERCLOCKWISE: Self = Self { ord: 1 };
    }
    impl crate::obj::EngineEnum for ClockDirection {
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
    unsafe impl sys::GodotFfi for ClockDirection {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct HorizontalAlignment {
        ord: i32,
    }
    impl HorizontalAlignment {
        pub const HORIZONTAL_ALIGNMENT_LEFT: Self = Self { ord: 0 };
        pub const HORIZONTAL_ALIGNMENT_CENTER: Self = Self { ord: 1 };
        pub const HORIZONTAL_ALIGNMENT_RIGHT: Self = Self { ord: 2 };
        pub const HORIZONTAL_ALIGNMENT_FILL: Self = Self { ord: 3 };
    }
    impl crate::obj::EngineEnum for HorizontalAlignment {
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
    unsafe impl sys::GodotFfi for HorizontalAlignment {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct VerticalAlignment {
        ord: i32,
    }
    impl VerticalAlignment {
        pub const VERTICAL_ALIGNMENT_TOP: Self = Self { ord: 0 };
        pub const VERTICAL_ALIGNMENT_CENTER: Self = Self { ord: 1 };
        pub const VERTICAL_ALIGNMENT_BOTTOM: Self = Self { ord: 2 };
        pub const VERTICAL_ALIGNMENT_FILL: Self = Self { ord: 3 };
    }
    impl crate::obj::EngineEnum for VerticalAlignment {
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
    unsafe impl sys::GodotFfi for VerticalAlignment {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct InlineAlignment {
        ord: i32,
    }
    impl InlineAlignment {
        pub const INLINE_ALIGNMENT_TOP_TO: Self = Self { ord: 0 };
        pub const INLINE_ALIGNMENT_CENTER_TO: Self = Self { ord: 1 };
        pub const INLINE_ALIGNMENT_BASELINE_TO: Self = Self { ord: 3 };
        pub const INLINE_ALIGNMENT_BOTTOM_TO: Self = Self { ord: 2 };
        pub const INLINE_ALIGNMENT_TO_TOP: Self = Self { ord: 0 };
        pub const INLINE_ALIGNMENT_TO_CENTER: Self = Self { ord: 4 };
        pub const INLINE_ALIGNMENT_TO_BASELINE: Self = Self { ord: 8 };
        pub const INLINE_ALIGNMENT_TO_BOTTOM: Self = Self { ord: 12 };
        pub const INLINE_ALIGNMENT_TOP: Self = Self { ord: 0 };
        pub const INLINE_ALIGNMENT_CENTER: Self = Self { ord: 5 };
        pub const INLINE_ALIGNMENT_BOTTOM: Self = Self { ord: 14 };
        pub const INLINE_ALIGNMENT_IMAGE_MASK: Self = Self { ord: 3 };
        pub const INLINE_ALIGNMENT_TEXT_MASK: Self = Self { ord: 12 };
    }
    impl crate::obj::EngineEnum for InlineAlignment {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ 0i32
                | ord @ 1i32
                | ord @ 2i32
                | ord @ 3i32
                | ord @ 4i32
                | ord @ 5i32
                | ord @ 8i32
                | ord @ 12i32
                | ord @ 14i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for InlineAlignment {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct EulerOrder {
        ord: i32,
    }
    impl EulerOrder {
        pub const EULER_ORDER_XYZ: Self = Self { ord: 0 };
        pub const EULER_ORDER_XZY: Self = Self { ord: 1 };
        pub const EULER_ORDER_YXZ: Self = Self { ord: 2 };
        pub const EULER_ORDER_YZX: Self = Self { ord: 3 };
        pub const EULER_ORDER_ZXY: Self = Self { ord: 4 };
        pub const EULER_ORDER_ZYX: Self = Self { ord: 5 };
    }
    impl crate::obj::EngineEnum for EulerOrder {
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
    unsafe impl sys::GodotFfi for EulerOrder {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Key {
        ord: i32,
    }
    impl Key {
        pub const KEY_NONE: Self = Self { ord: 0 };
        pub const KEY_SPECIAL: Self = Self { ord: 4194304 };
        pub const KEY_ESCAPE: Self = Self { ord: 4194305 };
        pub const KEY_TAB: Self = Self { ord: 4194306 };
        pub const KEY_BACKTAB: Self = Self { ord: 4194307 };
        pub const KEY_BACKSPACE: Self = Self { ord: 4194308 };
        pub const KEY_ENTER: Self = Self { ord: 4194309 };
        pub const KEY_KP_ENTER: Self = Self { ord: 4194310 };
        pub const KEY_INSERT: Self = Self { ord: 4194311 };
        pub const KEY_DELETE: Self = Self { ord: 4194312 };
        pub const KEY_PAUSE: Self = Self { ord: 4194313 };
        pub const KEY_PRINT: Self = Self { ord: 4194314 };
        pub const KEY_SYSREQ: Self = Self { ord: 4194315 };
        pub const KEY_CLEAR: Self = Self { ord: 4194316 };
        pub const KEY_HOME: Self = Self { ord: 4194317 };
        pub const KEY_END: Self = Self { ord: 4194318 };
        pub const KEY_LEFT: Self = Self { ord: 4194319 };
        pub const KEY_UP: Self = Self { ord: 4194320 };
        pub const KEY_RIGHT: Self = Self { ord: 4194321 };
        pub const KEY_DOWN: Self = Self { ord: 4194322 };
        pub const KEY_PAGEUP: Self = Self { ord: 4194323 };
        pub const KEY_PAGEDOWN: Self = Self { ord: 4194324 };
        pub const KEY_SHIFT: Self = Self { ord: 4194325 };
        pub const KEY_CTRL: Self = Self { ord: 4194326 };
        pub const KEY_META: Self = Self { ord: 4194327 };
        pub const KEY_ALT: Self = Self { ord: 4194328 };
        pub const KEY_CAPSLOCK: Self = Self { ord: 4194329 };
        pub const KEY_NUMLOCK: Self = Self { ord: 4194330 };
        pub const KEY_SCROLLLOCK: Self = Self { ord: 4194331 };
        pub const KEY_F1: Self = Self { ord: 4194332 };
        pub const KEY_F2: Self = Self { ord: 4194333 };
        pub const KEY_F3: Self = Self { ord: 4194334 };
        pub const KEY_F4: Self = Self { ord: 4194335 };
        pub const KEY_F5: Self = Self { ord: 4194336 };
        pub const KEY_F6: Self = Self { ord: 4194337 };
        pub const KEY_F7: Self = Self { ord: 4194338 };
        pub const KEY_F8: Self = Self { ord: 4194339 };
        pub const KEY_F9: Self = Self { ord: 4194340 };
        pub const KEY_F10: Self = Self { ord: 4194341 };
        pub const KEY_F11: Self = Self { ord: 4194342 };
        pub const KEY_F12: Self = Self { ord: 4194343 };
        pub const KEY_F13: Self = Self { ord: 4194344 };
        pub const KEY_F14: Self = Self { ord: 4194345 };
        pub const KEY_F15: Self = Self { ord: 4194346 };
        pub const KEY_F16: Self = Self { ord: 4194347 };
        pub const KEY_F17: Self = Self { ord: 4194348 };
        pub const KEY_F18: Self = Self { ord: 4194349 };
        pub const KEY_F19: Self = Self { ord: 4194350 };
        pub const KEY_F20: Self = Self { ord: 4194351 };
        pub const KEY_F21: Self = Self { ord: 4194352 };
        pub const KEY_F22: Self = Self { ord: 4194353 };
        pub const KEY_F23: Self = Self { ord: 4194354 };
        pub const KEY_F24: Self = Self { ord: 4194355 };
        pub const KEY_F25: Self = Self { ord: 4194356 };
        pub const KEY_F26: Self = Self { ord: 4194357 };
        pub const KEY_F27: Self = Self { ord: 4194358 };
        pub const KEY_F28: Self = Self { ord: 4194359 };
        pub const KEY_F29: Self = Self { ord: 4194360 };
        pub const KEY_F30: Self = Self { ord: 4194361 };
        pub const KEY_F31: Self = Self { ord: 4194362 };
        pub const KEY_F32: Self = Self { ord: 4194363 };
        pub const KEY_F33: Self = Self { ord: 4194364 };
        pub const KEY_F34: Self = Self { ord: 4194365 };
        pub const KEY_F35: Self = Self { ord: 4194366 };
        pub const KEY_KP_MULTIPLY: Self = Self { ord: 4194433 };
        pub const KEY_KP_DIVIDE: Self = Self { ord: 4194434 };
        pub const KEY_KP_SUBTRACT: Self = Self { ord: 4194435 };
        pub const KEY_KP_PERIOD: Self = Self { ord: 4194436 };
        pub const KEY_KP_ADD: Self = Self { ord: 4194437 };
        pub const KEY_KP_0: Self = Self { ord: 4194438 };
        pub const KEY_KP_1: Self = Self { ord: 4194439 };
        pub const KEY_KP_2: Self = Self { ord: 4194440 };
        pub const KEY_KP_3: Self = Self { ord: 4194441 };
        pub const KEY_KP_4: Self = Self { ord: 4194442 };
        pub const KEY_KP_5: Self = Self { ord: 4194443 };
        pub const KEY_KP_6: Self = Self { ord: 4194444 };
        pub const KEY_KP_7: Self = Self { ord: 4194445 };
        pub const KEY_KP_8: Self = Self { ord: 4194446 };
        pub const KEY_KP_9: Self = Self { ord: 4194447 };
        pub const KEY_MENU: Self = Self { ord: 4194370 };
        pub const KEY_HYPER: Self = Self { ord: 4194371 };
        pub const KEY_HELP: Self = Self { ord: 4194373 };
        pub const KEY_BACK: Self = Self { ord: 4194376 };
        pub const KEY_FORWARD: Self = Self { ord: 4194377 };
        pub const KEY_STOP: Self = Self { ord: 4194378 };
        pub const KEY_REFRESH: Self = Self { ord: 4194379 };
        pub const KEY_VOLUMEDOWN: Self = Self { ord: 4194380 };
        pub const KEY_VOLUMEMUTE: Self = Self { ord: 4194381 };
        pub const KEY_VOLUMEUP: Self = Self { ord: 4194382 };
        pub const KEY_MEDIAPLAY: Self = Self { ord: 4194388 };
        pub const KEY_MEDIASTOP: Self = Self { ord: 4194389 };
        pub const KEY_MEDIAPREVIOUS: Self = Self { ord: 4194390 };
        pub const KEY_MEDIANEXT: Self = Self { ord: 4194391 };
        pub const KEY_MEDIARECORD: Self = Self { ord: 4194392 };
        pub const KEY_HOMEPAGE: Self = Self { ord: 4194393 };
        pub const KEY_FAVORITES: Self = Self { ord: 4194394 };
        pub const KEY_SEARCH: Self = Self { ord: 4194395 };
        pub const KEY_STANDBY: Self = Self { ord: 4194396 };
        pub const KEY_OPENURL: Self = Self { ord: 4194397 };
        pub const KEY_LAUNCHMAIL: Self = Self { ord: 4194398 };
        pub const KEY_LAUNCHMEDIA: Self = Self { ord: 4194399 };
        pub const KEY_LAUNCH0: Self = Self { ord: 4194400 };
        pub const KEY_LAUNCH1: Self = Self { ord: 4194401 };
        pub const KEY_LAUNCH2: Self = Self { ord: 4194402 };
        pub const KEY_LAUNCH3: Self = Self { ord: 4194403 };
        pub const KEY_LAUNCH4: Self = Self { ord: 4194404 };
        pub const KEY_LAUNCH5: Self = Self { ord: 4194405 };
        pub const KEY_LAUNCH6: Self = Self { ord: 4194406 };
        pub const KEY_LAUNCH7: Self = Self { ord: 4194407 };
        pub const KEY_LAUNCH8: Self = Self { ord: 4194408 };
        pub const KEY_LAUNCH9: Self = Self { ord: 4194409 };
        pub const KEY_LAUNCHA: Self = Self { ord: 4194410 };
        pub const KEY_LAUNCHB: Self = Self { ord: 4194411 };
        pub const KEY_LAUNCHC: Self = Self { ord: 4194412 };
        pub const KEY_LAUNCHD: Self = Self { ord: 4194413 };
        pub const KEY_LAUNCHE: Self = Self { ord: 4194414 };
        pub const KEY_LAUNCHF: Self = Self { ord: 4194415 };
        pub const KEY_UNKNOWN: Self = Self { ord: 8388607 };
        pub const KEY_SPACE: Self = Self { ord: 32 };
        pub const KEY_EXCLAM: Self = Self { ord: 33 };
        pub const KEY_QUOTEDBL: Self = Self { ord: 34 };
        pub const KEY_NUMBERSIGN: Self = Self { ord: 35 };
        pub const KEY_DOLLAR: Self = Self { ord: 36 };
        pub const KEY_PERCENT: Self = Self { ord: 37 };
        pub const KEY_AMPERSAND: Self = Self { ord: 38 };
        pub const KEY_APOSTROPHE: Self = Self { ord: 39 };
        pub const KEY_PARENLEFT: Self = Self { ord: 40 };
        pub const KEY_PARENRIGHT: Self = Self { ord: 41 };
        pub const KEY_ASTERISK: Self = Self { ord: 42 };
        pub const KEY_PLUS: Self = Self { ord: 43 };
        pub const KEY_COMMA: Self = Self { ord: 44 };
        pub const KEY_MINUS: Self = Self { ord: 45 };
        pub const KEY_PERIOD: Self = Self { ord: 46 };
        pub const KEY_SLASH: Self = Self { ord: 47 };
        pub const KEY_0: Self = Self { ord: 48 };
        pub const KEY_1: Self = Self { ord: 49 };
        pub const KEY_2: Self = Self { ord: 50 };
        pub const KEY_3: Self = Self { ord: 51 };
        pub const KEY_4: Self = Self { ord: 52 };
        pub const KEY_5: Self = Self { ord: 53 };
        pub const KEY_6: Self = Self { ord: 54 };
        pub const KEY_7: Self = Self { ord: 55 };
        pub const KEY_8: Self = Self { ord: 56 };
        pub const KEY_9: Self = Self { ord: 57 };
        pub const KEY_COLON: Self = Self { ord: 58 };
        pub const KEY_SEMICOLON: Self = Self { ord: 59 };
        pub const KEY_LESS: Self = Self { ord: 60 };
        pub const KEY_EQUAL: Self = Self { ord: 61 };
        pub const KEY_GREATER: Self = Self { ord: 62 };
        pub const KEY_QUESTION: Self = Self { ord: 63 };
        pub const KEY_AT: Self = Self { ord: 64 };
        pub const KEY_A: Self = Self { ord: 65 };
        pub const KEY_B: Self = Self { ord: 66 };
        pub const KEY_C: Self = Self { ord: 67 };
        pub const KEY_D: Self = Self { ord: 68 };
        pub const KEY_E: Self = Self { ord: 69 };
        pub const KEY_F: Self = Self { ord: 70 };
        pub const KEY_G: Self = Self { ord: 71 };
        pub const KEY_H: Self = Self { ord: 72 };
        pub const KEY_I: Self = Self { ord: 73 };
        pub const KEY_J: Self = Self { ord: 74 };
        pub const KEY_K: Self = Self { ord: 75 };
        pub const KEY_L: Self = Self { ord: 76 };
        pub const KEY_M: Self = Self { ord: 77 };
        pub const KEY_N: Self = Self { ord: 78 };
        pub const KEY_O: Self = Self { ord: 79 };
        pub const KEY_P: Self = Self { ord: 80 };
        pub const KEY_Q: Self = Self { ord: 81 };
        pub const KEY_R: Self = Self { ord: 82 };
        pub const KEY_S: Self = Self { ord: 83 };
        pub const KEY_T: Self = Self { ord: 84 };
        pub const KEY_U: Self = Self { ord: 85 };
        pub const KEY_V: Self = Self { ord: 86 };
        pub const KEY_W: Self = Self { ord: 87 };
        pub const KEY_X: Self = Self { ord: 88 };
        pub const KEY_Y: Self = Self { ord: 89 };
        pub const KEY_Z: Self = Self { ord: 90 };
        pub const KEY_BRACKETLEFT: Self = Self { ord: 91 };
        pub const KEY_BACKSLASH: Self = Self { ord: 92 };
        pub const KEY_BRACKETRIGHT: Self = Self { ord: 93 };
        pub const KEY_ASCIICIRCUM: Self = Self { ord: 94 };
        pub const KEY_UNDERSCORE: Self = Self { ord: 95 };
        pub const KEY_QUOTELEFT: Self = Self { ord: 96 };
        pub const KEY_BRACELEFT: Self = Self { ord: 123 };
        pub const KEY_BAR: Self = Self { ord: 124 };
        pub const KEY_BRACERIGHT: Self = Self { ord: 125 };
        pub const KEY_ASCIITILDE: Self = Self { ord: 126 };
        pub const KEY_YEN: Self = Self { ord: 165 };
        pub const KEY_SECTION: Self = Self { ord: 167 };
        pub const KEY_GLOBE: Self = Self { ord: 4194416 };
        pub const KEY_KEYBOARD: Self = Self { ord: 4194417 };
        pub const KEY_JIS_EISU: Self = Self { ord: 4194418 };
        pub const KEY_JIS_KANA: Self = Self { ord: 4194419 };
    }
    impl crate::obj::EngineEnum for Key {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ 0i32
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
                | ord @ 123i32
                | ord @ 124i32
                | ord @ 125i32
                | ord @ 126i32
                | ord @ 165i32
                | ord @ 167i32
                | ord @ 4194304i32
                | ord @ 4194305i32
                | ord @ 4194306i32
                | ord @ 4194307i32
                | ord @ 4194308i32
                | ord @ 4194309i32
                | ord @ 4194310i32
                | ord @ 4194311i32
                | ord @ 4194312i32
                | ord @ 4194313i32
                | ord @ 4194314i32
                | ord @ 4194315i32
                | ord @ 4194316i32
                | ord @ 4194317i32
                | ord @ 4194318i32
                | ord @ 4194319i32
                | ord @ 4194320i32
                | ord @ 4194321i32
                | ord @ 4194322i32
                | ord @ 4194323i32
                | ord @ 4194324i32
                | ord @ 4194325i32
                | ord @ 4194326i32
                | ord @ 4194327i32
                | ord @ 4194328i32
                | ord @ 4194329i32
                | ord @ 4194330i32
                | ord @ 4194331i32
                | ord @ 4194332i32
                | ord @ 4194333i32
                | ord @ 4194334i32
                | ord @ 4194335i32
                | ord @ 4194336i32
                | ord @ 4194337i32
                | ord @ 4194338i32
                | ord @ 4194339i32
                | ord @ 4194340i32
                | ord @ 4194341i32
                | ord @ 4194342i32
                | ord @ 4194343i32
                | ord @ 4194344i32
                | ord @ 4194345i32
                | ord @ 4194346i32
                | ord @ 4194347i32
                | ord @ 4194348i32
                | ord @ 4194349i32
                | ord @ 4194350i32
                | ord @ 4194351i32
                | ord @ 4194352i32
                | ord @ 4194353i32
                | ord @ 4194354i32
                | ord @ 4194355i32
                | ord @ 4194356i32
                | ord @ 4194357i32
                | ord @ 4194358i32
                | ord @ 4194359i32
                | ord @ 4194360i32
                | ord @ 4194361i32
                | ord @ 4194362i32
                | ord @ 4194363i32
                | ord @ 4194364i32
                | ord @ 4194365i32
                | ord @ 4194366i32
                | ord @ 4194370i32
                | ord @ 4194371i32
                | ord @ 4194373i32
                | ord @ 4194376i32
                | ord @ 4194377i32
                | ord @ 4194378i32
                | ord @ 4194379i32
                | ord @ 4194380i32
                | ord @ 4194381i32
                | ord @ 4194382i32
                | ord @ 4194388i32
                | ord @ 4194389i32
                | ord @ 4194390i32
                | ord @ 4194391i32
                | ord @ 4194392i32
                | ord @ 4194393i32
                | ord @ 4194394i32
                | ord @ 4194395i32
                | ord @ 4194396i32
                | ord @ 4194397i32
                | ord @ 4194398i32
                | ord @ 4194399i32
                | ord @ 4194400i32
                | ord @ 4194401i32
                | ord @ 4194402i32
                | ord @ 4194403i32
                | ord @ 4194404i32
                | ord @ 4194405i32
                | ord @ 4194406i32
                | ord @ 4194407i32
                | ord @ 4194408i32
                | ord @ 4194409i32
                | ord @ 4194410i32
                | ord @ 4194411i32
                | ord @ 4194412i32
                | ord @ 4194413i32
                | ord @ 4194414i32
                | ord @ 4194415i32
                | ord @ 4194416i32
                | ord @ 4194417i32
                | ord @ 4194418i32
                | ord @ 4194419i32
                | ord @ 4194433i32
                | ord @ 4194434i32
                | ord @ 4194435i32
                | ord @ 4194436i32
                | ord @ 4194437i32
                | ord @ 4194438i32
                | ord @ 4194439i32
                | ord @ 4194440i32
                | ord @ 4194441i32
                | ord @ 4194442i32
                | ord @ 4194443i32
                | ord @ 4194444i32
                | ord @ 4194445i32
                | ord @ 4194446i32
                | ord @ 4194447i32
                | ord @ 8388607i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for Key {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
    pub struct KeyModifierMask {
        ord: i32,
    }
    impl KeyModifierMask {
        pub const KEY_CODE_MASK: Self = Self { ord: 8388607 };
        pub const KEY_MODIFIER_MASK: Self = Self { ord: 532676608 };
        pub const KEY_MASK_CMD_OR_CTRL: Self = Self { ord: 16777216 };
        pub const KEY_MASK_SHIFT: Self = Self { ord: 33554432 };
        pub const KEY_MASK_ALT: Self = Self { ord: 67108864 };
        pub const KEY_MASK_META: Self = Self { ord: 134217728 };
        pub const KEY_MASK_CTRL: Self = Self { ord: 268435456 };
        pub const KEY_MASK_KPAD: Self = Self { ord: 536870912 };
        pub const KEY_MASK_GROUP_SWITCH: Self = Self { ord: 1073741824 };
    }
    impl crate::obj::EngineEnum for KeyModifierMask {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ 8388607i32
                | ord @ 16777216i32
                | ord @ 33554432i32
                | ord @ 67108864i32
                | ord @ 134217728i32
                | ord @ 268435456i32
                | ord @ 532676608i32
                | ord @ 536870912i32
                | ord @ 1073741824i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for KeyModifierMask {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    impl std::ops::BitOr for KeyModifierMask {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord,
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct MouseButton {
        ord: i32,
    }
    impl MouseButton {
        pub const MOUSE_BUTTON_NONE: Self = Self { ord: 0 };
        pub const MOUSE_BUTTON_LEFT: Self = Self { ord: 1 };
        pub const MOUSE_BUTTON_RIGHT: Self = Self { ord: 2 };
        pub const MOUSE_BUTTON_MIDDLE: Self = Self { ord: 3 };
        pub const MOUSE_BUTTON_WHEEL_UP: Self = Self { ord: 4 };
        pub const MOUSE_BUTTON_WHEEL_DOWN: Self = Self { ord: 5 };
        pub const MOUSE_BUTTON_WHEEL_LEFT: Self = Self { ord: 6 };
        pub const MOUSE_BUTTON_WHEEL_RIGHT: Self = Self { ord: 7 };
        pub const MOUSE_BUTTON_XBUTTON1: Self = Self { ord: 8 };
        pub const MOUSE_BUTTON_XBUTTON2: Self = Self { ord: 9 };
    }
    impl crate::obj::EngineEnum for MouseButton {
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
                | ord @ 9i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for MouseButton {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
    pub struct MouseButtonMask {
        ord: i32,
    }
    impl MouseButtonMask {
        pub const MOUSE_BUTTON_MASK_LEFT: Self = Self { ord: 1 };
        pub const MOUSE_BUTTON_MASK_RIGHT: Self = Self { ord: 2 };
        pub const MOUSE_BUTTON_MASK_MIDDLE: Self = Self { ord: 4 };
        pub const MOUSE_BUTTON_MASK_MB_XBUTTON1: Self = Self { ord: 128 };
        pub const MOUSE_BUTTON_MASK_MB_XBUTTON2: Self = Self { ord: 256 };
    }
    impl crate::obj::EngineEnum for MouseButtonMask {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ 1i32 | ord @ 2i32 | ord @ 4i32 | ord @ 128i32 | ord @ 256i32 => {
                    Some(Self { ord })
                }
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for MouseButtonMask {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    impl std::ops::BitOr for MouseButtonMask {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord,
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct JoyButton {
        ord: i32,
    }
    impl JoyButton {
        pub const JOY_BUTTON_INVALID: Self = Self { ord: -1 };
        pub const JOY_BUTTON_A: Self = Self { ord: 0 };
        pub const JOY_BUTTON_B: Self = Self { ord: 1 };
        pub const JOY_BUTTON_X: Self = Self { ord: 2 };
        pub const JOY_BUTTON_Y: Self = Self { ord: 3 };
        pub const JOY_BUTTON_BACK: Self = Self { ord: 4 };
        pub const JOY_BUTTON_GUIDE: Self = Self { ord: 5 };
        pub const JOY_BUTTON_START: Self = Self { ord: 6 };
        pub const JOY_BUTTON_LEFT_STICK: Self = Self { ord: 7 };
        pub const JOY_BUTTON_RIGHT_STICK: Self = Self { ord: 8 };
        pub const JOY_BUTTON_LEFT_SHOULDER: Self = Self { ord: 9 };
        pub const JOY_BUTTON_RIGHT_SHOULDER: Self = Self { ord: 10 };
        pub const JOY_BUTTON_DPAD_UP: Self = Self { ord: 11 };
        pub const JOY_BUTTON_DPAD_DOWN: Self = Self { ord: 12 };
        pub const JOY_BUTTON_DPAD_LEFT: Self = Self { ord: 13 };
        pub const JOY_BUTTON_DPAD_RIGHT: Self = Self { ord: 14 };
        pub const JOY_BUTTON_MISC1: Self = Self { ord: 15 };
        pub const JOY_BUTTON_PADDLE1: Self = Self { ord: 16 };
        pub const JOY_BUTTON_PADDLE2: Self = Self { ord: 17 };
        pub const JOY_BUTTON_PADDLE3: Self = Self { ord: 18 };
        pub const JOY_BUTTON_PADDLE4: Self = Self { ord: 19 };
        pub const JOY_BUTTON_TOUCHPAD: Self = Self { ord: 20 };
        pub const JOY_BUTTON_SDL_MAX: Self = Self { ord: 21 };
        pub const JOY_BUTTON_MAX: Self = Self { ord: 128 };
    }
    impl crate::obj::EngineEnum for JoyButton {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ -1i32
                | ord @ 0i32
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
                | ord @ 128i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for JoyButton {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct JoyAxis {
        ord: i32,
    }
    impl JoyAxis {
        pub const JOY_AXIS_INVALID: Self = Self { ord: -1 };
        pub const JOY_AXIS_LEFT_X: Self = Self { ord: 0 };
        pub const JOY_AXIS_LEFT_Y: Self = Self { ord: 1 };
        pub const JOY_AXIS_RIGHT_X: Self = Self { ord: 2 };
        pub const JOY_AXIS_RIGHT_Y: Self = Self { ord: 3 };
        pub const JOY_AXIS_TRIGGER_LEFT: Self = Self { ord: 4 };
        pub const JOY_AXIS_TRIGGER_RIGHT: Self = Self { ord: 5 };
        pub const JOY_AXIS_SDL_MAX: Self = Self { ord: 6 };
        pub const JOY_AXIS_MAX: Self = Self { ord: 10 };
    }
    impl crate::obj::EngineEnum for JoyAxis {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ -1i32
                | ord @ 0i32
                | ord @ 1i32
                | ord @ 2i32
                | ord @ 3i32
                | ord @ 4i32
                | ord @ 5i32
                | ord @ 6i32
                | ord @ 10i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for JoyAxis {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct MIDIMessage {
        ord: i32,
    }
    impl MIDIMessage {
        pub const MIDI_MESSAGE_NONE: Self = Self { ord: 0 };
        pub const MIDI_MESSAGE_NOTE_OFF: Self = Self { ord: 8 };
        pub const MIDI_MESSAGE_NOTE_ON: Self = Self { ord: 9 };
        pub const MIDI_MESSAGE_AFTERTOUCH: Self = Self { ord: 10 };
        pub const MIDI_MESSAGE_CONTROL_CHANGE: Self = Self { ord: 11 };
        pub const MIDI_MESSAGE_PROGRAM_CHANGE: Self = Self { ord: 12 };
        pub const MIDI_MESSAGE_CHANNEL_PRESSURE: Self = Self { ord: 13 };
        pub const MIDI_MESSAGE_PITCH_BEND: Self = Self { ord: 14 };
        pub const MIDI_MESSAGE_SYSTEM_EXCLUSIVE: Self = Self { ord: 240 };
        pub const MIDI_MESSAGE_QUARTER_FRAME: Self = Self { ord: 241 };
        pub const MIDI_MESSAGE_SONG_POSITION_POINTER: Self = Self { ord: 242 };
        pub const MIDI_MESSAGE_SONG_SELECT: Self = Self { ord: 243 };
        pub const MIDI_MESSAGE_TUNE_REQUEST: Self = Self { ord: 246 };
        pub const MIDI_MESSAGE_TIMING_CLOCK: Self = Self { ord: 248 };
        pub const MIDI_MESSAGE_START: Self = Self { ord: 250 };
        pub const MIDI_MESSAGE_CONTINUE: Self = Self { ord: 251 };
        pub const MIDI_MESSAGE_STOP: Self = Self { ord: 252 };
        pub const MIDI_MESSAGE_ACTIVE_SENSING: Self = Self { ord: 254 };
        pub const MIDI_MESSAGE_SYSTEM_RESET: Self = Self { ord: 255 };
    }
    impl crate::obj::EngineEnum for MIDIMessage {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ 0i32
                | ord @ 8i32
                | ord @ 9i32
                | ord @ 10i32
                | ord @ 11i32
                | ord @ 12i32
                | ord @ 13i32
                | ord @ 14i32
                | ord @ 240i32
                | ord @ 241i32
                | ord @ 242i32
                | ord @ 243i32
                | ord @ 246i32
                | ord @ 248i32
                | ord @ 250i32
                | ord @ 251i32
                | ord @ 252i32
                | ord @ 254i32
                | ord @ 255i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for MIDIMessage {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct Error {
        ord: i32,
    }
    impl Error {
        pub const OK: Self = Self { ord: 0 };
        pub const FAILED: Self = Self { ord: 1 };
        pub const ERR_UNAVAILABLE: Self = Self { ord: 2 };
        pub const ERR_UNCONFIGURED: Self = Self { ord: 3 };
        pub const ERR_UNAUTHORIZED: Self = Self { ord: 4 };
        pub const ERR_PARAMETER_RANGE_ERROR: Self = Self { ord: 5 };
        pub const ERR_OUT_OF_MEMORY: Self = Self { ord: 6 };
        pub const ERR_FILE_NOT_FOUND: Self = Self { ord: 7 };
        pub const ERR_FILE_BAD_DRIVE: Self = Self { ord: 8 };
        pub const ERR_FILE_BAD_PATH: Self = Self { ord: 9 };
        pub const ERR_FILE_NO_PERMISSION: Self = Self { ord: 10 };
        pub const ERR_FILE_ALREADY_IN_USE: Self = Self { ord: 11 };
        pub const ERR_FILE_CANT_OPEN: Self = Self { ord: 12 };
        pub const ERR_FILE_CANT_WRITE: Self = Self { ord: 13 };
        pub const ERR_FILE_CANT_READ: Self = Self { ord: 14 };
        pub const ERR_FILE_UNRECOGNIZED: Self = Self { ord: 15 };
        pub const ERR_FILE_CORRUPT: Self = Self { ord: 16 };
        pub const ERR_FILE_MISSING_DEPENDENCIES: Self = Self { ord: 17 };
        pub const ERR_FILE_EOF: Self = Self { ord: 18 };
        pub const ERR_CANT_OPEN: Self = Self { ord: 19 };
        pub const ERR_CANT_CREATE: Self = Self { ord: 20 };
        pub const ERR_QUERY_FAILED: Self = Self { ord: 21 };
        pub const ERR_ALREADY_IN_USE: Self = Self { ord: 22 };
        pub const ERR_LOCKED: Self = Self { ord: 23 };
        pub const ERR_TIMEOUT: Self = Self { ord: 24 };
        pub const ERR_CANT_CONNECT: Self = Self { ord: 25 };
        pub const ERR_CANT_RESOLVE: Self = Self { ord: 26 };
        pub const ERR_CONNECTION_ERROR: Self = Self { ord: 27 };
        pub const ERR_CANT_ACQUIRE_RESOURCE: Self = Self { ord: 28 };
        pub const ERR_CANT_FORK: Self = Self { ord: 29 };
        pub const ERR_INVALID_DATA: Self = Self { ord: 30 };
        pub const ERR_INVALID_PARAMETER: Self = Self { ord: 31 };
        pub const ERR_ALREADY_EXISTS: Self = Self { ord: 32 };
        pub const ERR_DOES_NOT_EXIST: Self = Self { ord: 33 };
        pub const ERR_DATABASE_CANT_READ: Self = Self { ord: 34 };
        pub const ERR_DATABASE_CANT_WRITE: Self = Self { ord: 35 };
        pub const ERR_COMPILATION_FAILED: Self = Self { ord: 36 };
        pub const ERR_METHOD_NOT_FOUND: Self = Self { ord: 37 };
        pub const ERR_LINK_FAILED: Self = Self { ord: 38 };
        pub const ERR_SCRIPT_FAILED: Self = Self { ord: 39 };
        pub const ERR_CYCLIC_LINK: Self = Self { ord: 40 };
        pub const ERR_INVALID_DECLARATION: Self = Self { ord: 41 };
        pub const ERR_DUPLICATE_SYMBOL: Self = Self { ord: 42 };
        pub const ERR_PARSE_ERROR: Self = Self { ord: 43 };
        pub const ERR_BUSY: Self = Self { ord: 44 };
        pub const ERR_SKIP: Self = Self { ord: 45 };
        pub const ERR_HELP: Self = Self { ord: 46 };
        pub const ERR_BUG: Self = Self { ord: 47 };
        pub const ERR_PRINTER_ON_FIRE: Self = Self { ord: 48 };
    }
    impl crate::obj::EngineEnum for Error {
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
                | ord @ 48i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for Error {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct PropertyHint {
        ord: i32,
    }
    impl PropertyHint {
        pub const PROPERTY_HINT_NONE: Self = Self { ord: 0 };
        pub const PROPERTY_HINT_RANGE: Self = Self { ord: 1 };
        pub const PROPERTY_HINT_ENUM: Self = Self { ord: 2 };
        pub const PROPERTY_HINT_ENUM_SUGGESTION: Self = Self { ord: 3 };
        pub const PROPERTY_HINT_EXP_EASING: Self = Self { ord: 4 };
        pub const PROPERTY_HINT_LINK: Self = Self { ord: 5 };
        pub const PROPERTY_HINT_FLAGS: Self = Self { ord: 6 };
        pub const PROPERTY_HINT_LAYERS_2D_RENDER: Self = Self { ord: 7 };
        pub const PROPERTY_HINT_LAYERS_2D_PHYSICS: Self = Self { ord: 8 };
        pub const PROPERTY_HINT_LAYERS_2D_NAVIGATION: Self = Self { ord: 9 };
        pub const PROPERTY_HINT_LAYERS_3D_RENDER: Self = Self { ord: 10 };
        pub const PROPERTY_HINT_LAYERS_3D_PHYSICS: Self = Self { ord: 11 };
        pub const PROPERTY_HINT_LAYERS_3D_NAVIGATION: Self = Self { ord: 12 };
        pub const PROPERTY_HINT_FILE: Self = Self { ord: 13 };
        pub const PROPERTY_HINT_DIR: Self = Self { ord: 14 };
        pub const PROPERTY_HINT_GLOBAL_FILE: Self = Self { ord: 15 };
        pub const PROPERTY_HINT_GLOBAL_DIR: Self = Self { ord: 16 };
        pub const PROPERTY_HINT_RESOURCE_TYPE: Self = Self { ord: 17 };
        pub const PROPERTY_HINT_MULTILINE_TEXT: Self = Self { ord: 18 };
        pub const PROPERTY_HINT_EXPRESSION: Self = Self { ord: 19 };
        pub const PROPERTY_HINT_PLACEHOLDER_TEXT: Self = Self { ord: 20 };
        pub const PROPERTY_HINT_COLOR_NO_ALPHA: Self = Self { ord: 21 };
        pub const PROPERTY_HINT_OBJECT_ID: Self = Self { ord: 22 };
        pub const PROPERTY_HINT_TYPE_STRING: Self = Self { ord: 23 };
        pub const PROPERTY_HINT_NODE_PATH_TO_EDITED_NODE: Self = Self { ord: 24 };
        pub const PROPERTY_HINT_OBJECT_TOO_BIG: Self = Self { ord: 25 };
        pub const PROPERTY_HINT_NODE_PATH_VALID_TYPES: Self = Self { ord: 26 };
        pub const PROPERTY_HINT_SAVE_FILE: Self = Self { ord: 27 };
        pub const PROPERTY_HINT_GLOBAL_SAVE_FILE: Self = Self { ord: 28 };
        pub const PROPERTY_HINT_INT_IS_OBJECTID: Self = Self { ord: 29 };
        pub const PROPERTY_HINT_INT_IS_POINTER: Self = Self { ord: 30 };
        pub const PROPERTY_HINT_ARRAY_TYPE: Self = Self { ord: 31 };
        pub const PROPERTY_HINT_LOCALE_ID: Self = Self { ord: 32 };
        pub const PROPERTY_HINT_LOCALIZABLE_STRING: Self = Self { ord: 33 };
        pub const PROPERTY_HINT_NODE_TYPE: Self = Self { ord: 34 };
        pub const PROPERTY_HINT_HIDE_QUATERNION_EDIT: Self = Self { ord: 35 };
        pub const PROPERTY_HINT_PASSWORD: Self = Self { ord: 36 };
        pub const PROPERTY_HINT_MAX: Self = Self { ord: 37 };
    }
    impl crate::obj::EngineEnum for PropertyHint {
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
                | ord @ 37i32 => Some(Self { ord }),
                _ => None,
            }
        }
        fn ord(self) -> i32 {
            self.ord
        }
    }
    unsafe impl sys::GodotFfi for PropertyHint {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
    pub struct PropertyUsageFlags {
        ord: i32,
    }
    impl PropertyUsageFlags {
        pub const PROPERTY_USAGE_NONE: Self = Self { ord: 0 };
        pub const PROPERTY_USAGE_STORAGE: Self = Self { ord: 2 };
        pub const PROPERTY_USAGE_EDITOR: Self = Self { ord: 4 };
        pub const PROPERTY_USAGE_INTERNAL: Self = Self { ord: 8 };
        pub const PROPERTY_USAGE_CHECKABLE: Self = Self { ord: 16 };
        pub const PROPERTY_USAGE_CHECKED: Self = Self { ord: 32 };
        pub const PROPERTY_USAGE_GROUP: Self = Self { ord: 64 };
        pub const PROPERTY_USAGE_CATEGORY: Self = Self { ord: 128 };
        pub const PROPERTY_USAGE_SUBGROUP: Self = Self { ord: 256 };
        pub const PROPERTY_USAGE_CLASS_IS_BITFIELD: Self = Self { ord: 512 };
        pub const PROPERTY_USAGE_NO_INSTANCE_STATE: Self = Self { ord: 1024 };
        pub const PROPERTY_USAGE_RESTART_IF_CHANGED: Self = Self { ord: 2048 };
        pub const PROPERTY_USAGE_SCRIPT_VARIABLE: Self = Self { ord: 4096 };
        pub const PROPERTY_USAGE_STORE_IF_NULL: Self = Self { ord: 8192 };
        pub const PROPERTY_USAGE_UPDATE_ALL_IF_MODIFIED: Self = Self { ord: 16384 };
        pub const PROPERTY_USAGE_SCRIPT_DEFAULT_VALUE: Self = Self { ord: 32768 };
        pub const PROPERTY_USAGE_CLASS_IS_ENUM: Self = Self { ord: 65536 };
        pub const PROPERTY_USAGE_NIL_IS_VARIANT: Self = Self { ord: 131072 };
        pub const PROPERTY_USAGE_ARRAY: Self = Self { ord: 262144 };
        pub const PROPERTY_USAGE_ALWAYS_DUPLICATE: Self = Self { ord: 524288 };
        pub const PROPERTY_USAGE_NEVER_DUPLICATE: Self = Self { ord: 1048576 };
        pub const PROPERTY_USAGE_HIGH_END_GFX: Self = Self { ord: 2097152 };
        pub const PROPERTY_USAGE_NODE_PATH_FROM_SCENE_ROOT: Self = Self { ord: 4194304 };
        pub const PROPERTY_USAGE_RESOURCE_NOT_PERSISTENT: Self = Self { ord: 8388608 };
        pub const PROPERTY_USAGE_KEYING_INCREMENTS: Self = Self { ord: 16777216 };
        pub const PROPERTY_USAGE_DEFERRED_SET_RESOURCE: Self = Self { ord: 33554432 };
        pub const PROPERTY_USAGE_EDITOR_INSTANTIATE_OBJECT: Self = Self { ord: 67108864 };
        pub const PROPERTY_USAGE_EDITOR_BASIC_SETTING: Self = Self { ord: 134217728 };
        pub const PROPERTY_USAGE_READ_ONLY: Self = Self { ord: 268435456 };
        pub const PROPERTY_USAGE_DEFAULT: Self = Self { ord: 6 };
        pub const PROPERTY_USAGE_NO_EDITOR: Self = Self { ord: 2 };
    }
    impl crate::obj::EngineEnum for PropertyUsageFlags {
        fn try_from_ord(ord: i32) -> Option<Self> {
            match ord {
                ord @ 0i32
                | ord @ 2i32
                | ord @ 4i32
                | ord @ 6i32
                | ord @ 8i32
                | ord @ 16i32
                | ord @ 32i32
                | ord @ 64i32
                | ord @ 128i32
                | ord @ 256i32
                | ord @ 512i32
                | ord @ 1024i32
                | ord @ 2048i32
                | ord @ 4096i32
                | ord @ 8192i32
                | ord @ 16384i32
                | ord @ 32768i32
                | ord @ 65536i32
                | ord @ 131072i32
                | ord @ 262144i32
                | ord @ 524288i32
                | ord @ 1048576i32
                | ord @ 2097152i32
                | ord @ 4194304i32
                | ord @ 8388608i32
                | ord @ 16777216i32
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
    unsafe impl sys::GodotFfi for PropertyUsageFlags {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    impl std::ops::BitOr for PropertyUsageFlags {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord,
            }
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
    pub struct MethodFlags {
        ord: i32,
    }
    impl MethodFlags {
        pub const METHOD_FLAG_NORMAL: Self = Self { ord: 1 };
        pub const METHOD_FLAG_EDITOR: Self = Self { ord: 2 };
        pub const METHOD_FLAG_CONST: Self = Self { ord: 4 };
        pub const METHOD_FLAG_VIRTUAL: Self = Self { ord: 8 };
        pub const METHOD_FLAG_VARARG: Self = Self { ord: 16 };
        pub const METHOD_FLAG_STATIC: Self = Self { ord: 32 };
        pub const METHOD_FLAG_OBJECT_CORE: Self = Self { ord: 64 };
        pub const METHOD_FLAGS_DEFAULT: Self = Self { ord: 1 };
    }
    impl crate::obj::EngineEnum for MethodFlags {
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
    unsafe impl sys::GodotFfi for MethodFlags {
        sys::ffi_methods! { type sys :: GDExtensionTypePtr = * mut Self ; .. }
    }
    impl std::ops::BitOr for MethodFlags {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self {
                ord: self.ord | rhs.ord,
            }
        }
    }
}
