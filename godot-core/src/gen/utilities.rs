#![doc = r" Global utility functions."]
#![doc = r""]
#![doc = r" A list of global-scope built-in functions."]
#![doc = r" For global enums and constants, check out the [`global` module][crate::engine::global]."]
#![doc = r""]
#![doc = r" See also [Godot docs for `@GlobalScope`](https://docs.godotengine.org/en/stable/classes/class_@globalscope.html#methods)."]
use crate::builtin::*;
use crate::engine::Object;
use crate::obj::Gd;
use godot_ffi as sys;
use sys::GodotFfi as _;
pub fn sin(angle_rad: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("sin");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&angle_rad)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn cos(angle_rad: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("cos");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&angle_rad)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn tan(angle_rad: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("tan");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&angle_rad)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn sinh(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("sinh");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn cosh(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("cosh");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn tanh(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("tanh");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn asin(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("asin");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn acos(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("acos");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn atan(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("atan");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn atan2(y: f64, x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("atan2");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&y),
            <f64 as sys::GodotFfi>::sys_const(&x),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn sqrt(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("sqrt");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn fmod(x: f64, y: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("fmod");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&x),
            <f64 as sys::GodotFfi>::sys_const(&y),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn fposmod(x: f64, y: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("fposmod");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&x),
            <f64 as sys::GodotFfi>::sys_const(&y),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn posmod(x: i64, y: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("posmod");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3133453818i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <i64 as sys::GodotFfi>::sys_const(&x),
            <i64 as sys::GodotFfi>::sys_const(&y),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn floor(x: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("floor");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4776452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn floorf(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("floorf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn floori(x: f64) -> i64 {
    unsafe {
        let __function_name = StringName::from("floori");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2780425386i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn ceil(x: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("ceil");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4776452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn ceilf(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("ceilf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn ceili(x: f64) -> i64 {
    unsafe {
        let __function_name = StringName::from("ceili");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2780425386i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn round(x: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("round");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4776452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn roundf(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("roundf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn roundi(x: f64) -> i64 {
    unsafe {
        let __function_name = StringName::from("roundi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2780425386i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn abs(x: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("abs");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4776452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn absf(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("absf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn absi(x: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("absi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2157319888i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn sign(x: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("sign");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4776452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn signf(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("signf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn signi(x: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("signi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2157319888i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn snapped(x: Variant, step: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("snapped");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            459914704i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <Variant as sys::GodotFfi>::sys_const(&x),
            <Variant as sys::GodotFfi>::sys_const(&step),
        ];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn snappedf(x: f64, step: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("snappedf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&x),
            <f64 as sys::GodotFfi>::sys_const(&step),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn snappedi(x: f64, step: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("snappedi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3570758393i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&x),
            <i64 as sys::GodotFfi>::sys_const(&step),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn pow(base: f64, exp: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("pow");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&base),
            <f64 as sys::GodotFfi>::sys_const(&exp),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn log(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("log");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn exp(x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("exp");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_nan(x: f64) -> bool {
    unsafe {
        let __function_name = StringName::from("is_nan");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3569215213i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_inf(x: f64) -> bool {
    unsafe {
        let __function_name = StringName::from("is_inf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3569215213i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_equal_approx(a: f64, b: f64) -> bool {
    unsafe {
        let __function_name = StringName::from("is_equal_approx");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1400789633i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&a),
            <f64 as sys::GodotFfi>::sys_const(&b),
        ];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_zero_approx(x: f64) -> bool {
    unsafe {
        let __function_name = StringName::from("is_zero_approx");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3569215213i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_finite(x: f64) -> bool {
    unsafe {
        let __function_name = StringName::from("is_finite");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3569215213i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn ease(x: f64, curve: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("ease");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&x),
            <f64 as sys::GodotFfi>::sys_const(&curve),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn step_decimals(x: f64) -> i64 {
    unsafe {
        let __function_name = StringName::from("step_decimals");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2780425386i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&x)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn lerp(from: Variant, to: Variant, weight: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("lerp");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3389874542i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <Variant as sys::GodotFfi>::sys_const(&from),
            <Variant as sys::GodotFfi>::sys_const(&to),
            <Variant as sys::GodotFfi>::sys_const(&weight),
        ];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn lerpf(from: f64, to: f64, weight: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("lerpf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&weight),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn cubic_interpolate(from: f64, to: f64, pre: f64, post: f64, weight: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("cubic_interpolate");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1090965791i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&pre),
            <f64 as sys::GodotFfi>::sys_const(&post),
            <f64 as sys::GodotFfi>::sys_const(&weight),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn cubic_interpolate_angle(from: f64, to: f64, pre: f64, post: f64, weight: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("cubic_interpolate_angle");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1090965791i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&pre),
            <f64 as sys::GodotFfi>::sys_const(&post),
            <f64 as sys::GodotFfi>::sys_const(&weight),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn cubic_interpolate_in_time(
    from: f64,
    to: f64,
    pre: f64,
    post: f64,
    weight: f64,
    to_t: f64,
    pre_t: f64,
    post_t: f64,
) -> f64 {
    unsafe {
        let __function_name = StringName::from("cubic_interpolate_in_time");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            388121036i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&pre),
            <f64 as sys::GodotFfi>::sys_const(&post),
            <f64 as sys::GodotFfi>::sys_const(&weight),
            <f64 as sys::GodotFfi>::sys_const(&to_t),
            <f64 as sys::GodotFfi>::sys_const(&pre_t),
            <f64 as sys::GodotFfi>::sys_const(&post_t),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn cubic_interpolate_angle_in_time(
    from: f64,
    to: f64,
    pre: f64,
    post: f64,
    weight: f64,
    to_t: f64,
    pre_t: f64,
    post_t: f64,
) -> f64 {
    unsafe {
        let __function_name = StringName::from("cubic_interpolate_angle_in_time");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            388121036i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&pre),
            <f64 as sys::GodotFfi>::sys_const(&post),
            <f64 as sys::GodotFfi>::sys_const(&weight),
            <f64 as sys::GodotFfi>::sys_const(&to_t),
            <f64 as sys::GodotFfi>::sys_const(&pre_t),
            <f64 as sys::GodotFfi>::sys_const(&post_t),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn bezier_interpolate(start: f64, control_1: f64, control_2: f64, end: f64, t: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("bezier_interpolate");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1090965791i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&start),
            <f64 as sys::GodotFfi>::sys_const(&control_1),
            <f64 as sys::GodotFfi>::sys_const(&control_2),
            <f64 as sys::GodotFfi>::sys_const(&end),
            <f64 as sys::GodotFfi>::sys_const(&t),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn bezier_derivative(start: f64, control_1: f64, control_2: f64, end: f64, t: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("bezier_derivative");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1090965791i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&start),
            <f64 as sys::GodotFfi>::sys_const(&control_1),
            <f64 as sys::GodotFfi>::sys_const(&control_2),
            <f64 as sys::GodotFfi>::sys_const(&end),
            <f64 as sys::GodotFfi>::sys_const(&t),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn lerp_angle(from: f64, to: f64, weight: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("lerp_angle");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&weight),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn inverse_lerp(from: f64, to: f64, weight: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("inverse_lerp");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&weight),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn remap(value: f64, istart: f64, istop: f64, ostart: f64, ostop: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("remap");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1090965791i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&value),
            <f64 as sys::GodotFfi>::sys_const(&istart),
            <f64 as sys::GodotFfi>::sys_const(&istop),
            <f64 as sys::GodotFfi>::sys_const(&ostart),
            <f64 as sys::GodotFfi>::sys_const(&ostop),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn smoothstep(from: f64, to: f64, x: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("smoothstep");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&x),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn move_toward(from: f64, to: f64, delta: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("move_toward");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
            <f64 as sys::GodotFfi>::sys_const(&delta),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn deg_to_rad(deg: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("deg_to_rad");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&deg)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn rad_to_deg(rad: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("rad_to_deg");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&rad)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn linear_to_db(lin: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("linear_to_db");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&lin)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn db_to_linear(db: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("db_to_linear");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2140049587i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<f64 as sys::GodotFfi>::sys_const(&db)];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn wrap(value: Variant, min: Variant, max: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("wrap");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3389874542i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <Variant as sys::GodotFfi>::sys_const(&value),
            <Variant as sys::GodotFfi>::sys_const(&min),
            <Variant as sys::GodotFfi>::sys_const(&max),
        ];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn wrapi(value: i64, min: i64, max: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("wrapi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            650295447i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <i64 as sys::GodotFfi>::sys_const(&value),
            <i64 as sys::GodotFfi>::sys_const(&min),
            <i64 as sys::GodotFfi>::sys_const(&max),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn wrapf(value: f64, min: f64, max: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("wrapf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&value),
            <f64 as sys::GodotFfi>::sys_const(&min),
            <f64 as sys::GodotFfi>::sys_const(&max),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn max(arg1: Variant, arg2: Variant, varargs: &[Variant]) -> Variant {
    unsafe {
        let __function_name = StringName::from("max");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3896050336i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [
            <Variant as ToVariant>::to_variant(&arg1),
            <Variant as ToVariant>::to_variant(&arg2),
        ];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let variant = Variant::from_sys_init_default(|return_ptr| {
            let mut __err = sys::default_call_error();
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
            if __err.error != sys::GDEXTENSION_CALL_OK {
                let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __arg_types.extend(varargs.iter().map(Variant::get_type));
                let __vararg_str = varargs
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                sys::panic_call_error(
                    &__err,
                    &format!("max({arg1:?}, {arg2:?}; {__vararg_str})"),
                    &__arg_types,
                );
            }
        });
        variant
    }
}
pub fn maxi(a: i64, b: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("maxi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3133453818i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <i64 as sys::GodotFfi>::sys_const(&a),
            <i64 as sys::GodotFfi>::sys_const(&b),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn maxf(a: f64, b: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("maxf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&a),
            <f64 as sys::GodotFfi>::sys_const(&b),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn min(arg1: Variant, arg2: Variant, varargs: &[Variant]) -> Variant {
    unsafe {
        let __function_name = StringName::from("min");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3896050336i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [
            <Variant as ToVariant>::to_variant(&arg1),
            <Variant as ToVariant>::to_variant(&arg2),
        ];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let variant = Variant::from_sys_init_default(|return_ptr| {
            let mut __err = sys::default_call_error();
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
            if __err.error != sys::GDEXTENSION_CALL_OK {
                let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __arg_types.extend(varargs.iter().map(Variant::get_type));
                let __vararg_str = varargs
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                sys::panic_call_error(
                    &__err,
                    &format!("min({arg1:?}, {arg2:?}; {__vararg_str})"),
                    &__arg_types,
                );
            }
        });
        variant
    }
}
pub fn mini(a: i64, b: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("mini");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3133453818i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <i64 as sys::GodotFfi>::sys_const(&a),
            <i64 as sys::GodotFfi>::sys_const(&b),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn minf(a: f64, b: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("minf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&a),
            <f64 as sys::GodotFfi>::sys_const(&b),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn clamp(value: Variant, min: Variant, max: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("clamp");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3389874542i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <Variant as sys::GodotFfi>::sys_const(&value),
            <Variant as sys::GodotFfi>::sys_const(&min),
            <Variant as sys::GodotFfi>::sys_const(&max),
        ];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn clampi(value: i64, min: i64, max: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("clampi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            650295447i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <i64 as sys::GodotFfi>::sys_const(&value),
            <i64 as sys::GodotFfi>::sys_const(&min),
            <i64 as sys::GodotFfi>::sys_const(&max),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn clampf(value: f64, min: f64, max: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("clampf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            998901048i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&value),
            <f64 as sys::GodotFfi>::sys_const(&min),
            <f64 as sys::GodotFfi>::sys_const(&max),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn nearest_po2(value: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("nearest_po2");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2157319888i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&value)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn pingpong(value: f64, length: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("pingpong");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&value),
            <f64 as sys::GodotFfi>::sys_const(&length),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn randomize() {
    unsafe {
        let __function_name = StringName::from("randomize");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1691721052i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [];
        let __args_ptr = __args.as_ptr();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
    }
}
pub fn randi() -> i64 {
    unsafe {
        let __function_name = StringName::from("randi");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            701202648i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn randf() -> f64 {
    unsafe {
        let __function_name = StringName::from("randf");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2086227845i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn randi_range(from: i64, to: i64) -> i64 {
    unsafe {
        let __function_name = StringName::from("randi_range");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3133453818i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <i64 as sys::GodotFfi>::sys_const(&from),
            <i64 as sys::GodotFfi>::sys_const(&to),
        ];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn randf_range(from: f64, to: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("randf_range");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&from),
            <f64 as sys::GodotFfi>::sys_const(&to),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn randfn(mean: f64, deviation: f64) -> f64 {
    unsafe {
        let __function_name = StringName::from("randfn");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            92296394i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <f64 as sys::GodotFfi>::sys_const(&mean),
            <f64 as sys::GodotFfi>::sys_const(&deviation),
        ];
        let __args_ptr = __args.as_ptr();
        <f64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn seed(base: i64) {
    unsafe {
        let __function_name = StringName::from("seed");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            382931173i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&base)];
        let __args_ptr = __args.as_ptr();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
    }
}
pub fn rand_from_seed(seed: i64) -> PackedInt64Array {
    unsafe {
        let __function_name = StringName::from("rand_from_seed");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1391063685i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&seed)];
        let __args_ptr = __args.as_ptr();
        <PackedInt64Array as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn weakref(obj: Variant) -> Variant {
    unsafe {
        let __function_name = StringName::from("weakref");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4776452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&obj)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn typeof_(variable: Variant) -> i64 {
    unsafe {
        let __function_name = StringName::from("typeof");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            326422594i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&variable)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn str(arg1: Variant, varargs: &[Variant]) -> GodotString {
    unsafe {
        let __function_name = StringName::from("str");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            32569176i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let variant = Variant::from_sys_init_default(|return_ptr| {
            let mut __err = sys::default_call_error();
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
            if __err.error != sys::GDEXTENSION_CALL_OK {
                let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
                __arg_types.extend(varargs.iter().map(Variant::get_type));
                let __vararg_str = varargs
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                sys::panic_call_error(
                    &__err,
                    &format!("str({arg1:?}; {__vararg_str})"),
                    &__arg_types,
                );
            }
        });
        variant.to()
    }
}
pub fn error_string(error: i64) -> GodotString {
    unsafe {
        let __function_name = StringName::from("error_string");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            942708242i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&error)];
        let __args_ptr = __args.as_ptr();
        <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn print(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("print");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("print({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn print_rich(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("print_rich");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("print_rich({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn printerr(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("printerr");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("printerr({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn printt(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("printt");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("printt({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn prints(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("prints");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("prints({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn printraw(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("printraw");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("printraw({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn print_verbose(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("print_verbose");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("print_verbose({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn push_error(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("push_error");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("push_error({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn push_warning(arg1: Variant, varargs: &[Variant]) {
    unsafe {
        let __function_name = StringName::from("push_warning");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2648703342i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __explicit_args = [<Variant as ToVariant>::to_variant(&arg1)];
        let mut __args = Vec::with_capacity(__explicit_args.len() + varargs.len());
        __args.extend(__explicit_args.iter().map(Variant::sys_const));
        __args.extend(varargs.iter().map(Variant::sys_const));
        let __args_ptr = __args.as_ptr();
        let mut __err = sys::default_call_error();
        let return_ptr = std::ptr::null_mut();
        __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        if __err.error != sys::GDEXTENSION_CALL_OK {
            let mut __arg_types = Vec::with_capacity(__explicit_args.len() + varargs.len());
            __arg_types.extend(varargs.iter().map(Variant::get_type));
            let __vararg_str = varargs
                .iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(", ");
            sys::panic_call_error(
                &__err,
                &format!("push_warning({arg1:?}; {__vararg_str})"),
                &__arg_types,
            );
        }
    }
}
pub fn var_to_str(variable: Variant) -> GodotString {
    unsafe {
        let __function_name = StringName::from("var_to_str");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            866625479i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&variable)];
        let __args_ptr = __args.as_ptr();
        <GodotString as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn str_to_var(string: GodotString) -> Variant {
    unsafe {
        let __function_name = StringName::from("str_to_var");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1891498491i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<GodotString as sys::GodotFfi>::sys_const(&string)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn var_to_bytes(variable: Variant) -> PackedByteArray {
    unsafe {
        let __function_name = StringName::from("var_to_bytes");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2947269930i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&variable)];
        let __args_ptr = __args.as_ptr();
        <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn bytes_to_var(bytes: PackedByteArray) -> Variant {
    unsafe {
        let __function_name = StringName::from("bytes_to_var");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4249819452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&bytes)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn var_to_bytes_with_objects(variable: Variant) -> PackedByteArray {
    unsafe {
        let __function_name = StringName::from("var_to_bytes_with_objects");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2947269930i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&variable)];
        let __args_ptr = __args.as_ptr();
        <PackedByteArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn bytes_to_var_with_objects(bytes: PackedByteArray) -> Variant {
    unsafe {
        let __function_name = StringName::from("bytes_to_var_with_objects");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            4249819452i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<PackedByteArray as sys::GodotFfi>::sys_const(&bytes)];
        let __args_ptr = __args.as_ptr();
        <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn hash(variable: Variant) -> i64 {
    unsafe {
        let __function_name = StringName::from("hash");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            326422594i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&variable)];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn instance_from_id(instance_id: i64) -> Option<Gd<Object>> {
    unsafe {
        let __function_name = StringName::from("instance_from_id");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1156694636i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&instance_id)];
        let __args_ptr = __args.as_ptr();
        <Gd<Object>>::from_sys_init_opt(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_instance_id_valid(id: i64) -> bool {
    unsafe {
        let __function_name = StringName::from("is_instance_id_valid");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            2232439758i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&id)];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_instance_valid(instance: Variant) -> bool {
    unsafe {
        let __function_name = StringName::from("is_instance_valid");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            996128841i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<Variant as sys::GodotFfi>::sys_const(&instance)];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn rid_allocate_id() -> i64 {
    unsafe {
        let __function_name = StringName::from("rid_allocate_id");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            701202648i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [];
        let __args_ptr = __args.as_ptr();
        <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn rid_from_int64(base: i64) -> Rid {
    unsafe {
        let __function_name = StringName::from("rid_from_int64");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            3426892196i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [<i64 as sys::GodotFfi>::sys_const(&base)];
        let __args_ptr = __args.as_ptr();
        <Rid as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
pub fn is_same(a: Variant, b: Variant) -> bool {
    unsafe {
        let __function_name = StringName::from("is_same");
        let __call_fn = sys::interface_fn!(variant_get_ptr_utility_function)(
            __function_name.string_sys(),
            1409423524i64,
        );
        let __call_fn = __call_fn.unwrap_unchecked();
        let __args = [
            <Variant as sys::GodotFfi>::sys_const(&a),
            <Variant as sys::GodotFfi>::sys_const(&b),
        ];
        let __args_ptr = __args.as_ptr();
        <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
            __call_fn(return_ptr, __args_ptr, __args.len() as i32);
        })
    }
}
