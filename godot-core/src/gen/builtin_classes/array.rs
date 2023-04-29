use crate::builtin::*;
use crate::engine::Object;
use crate::obj::{AsArg, Gd};
use crate::sys::GodotFfi as _;
use godot_ffi as sys;
#[repr(transparent)]
pub struct InnerArray<'a> {
    _outer_lifetime: std::marker::PhantomData<&'a ()>,
    sys_ptr: sys::GDExtensionTypePtr,
}
impl<'a> InnerArray<'a> {
    pub fn from_outer(outer: &VariantArray) -> Self {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn from_outer_typed<T>(outer: &Array<T>) -> Self
    where
        T: crate::builtin::meta::VariantMetadata,
    {
        Self {
            _outer_lifetime: std::marker::PhantomData,
            sys_ptr: outer.sys(),
        }
    }
    pub fn size(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("size");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3173160232i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_empty(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("is_empty");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn clear(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("clear");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn hash(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("hash");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3173160232i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn assign(&mut self, array: VariantArray) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("assign");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2307260970i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<VariantArray as sys::GodotFfi>::sys_const(&array)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn push_back(&mut self, value: Variant) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("push_back");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3316032543i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn push_front(&mut self, value: Variant) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("push_front");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3316032543i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn append(&mut self, value: Variant) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("append");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3316032543i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn append_array(&mut self, array: VariantArray) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("append_array");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2307260970i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<VariantArray as sys::GodotFfi>::sys_const(&array)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn resize(&mut self, size: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("resize");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                848867239i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&size)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn insert(&mut self, position: i64, value: Variant) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("insert");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3176316662i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&position),
                <Variant as sys::GodotFfi>::sys_const(&value),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn remove_at(&mut self, position: i64) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("remove_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2823966027i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&position)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn fill(&mut self, value: Variant) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("fill");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3316032543i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn erase(&mut self, value: Variant) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("erase");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3316032543i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn front(&self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("front");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1460142086i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn back(&self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("back");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1460142086i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn pick_random(&self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("pick_random");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1460142086i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn find(&self, what: Variant, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("find");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2336346817i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Variant as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn rfind(&self, what: Variant, from: i64) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("rfind");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2336346817i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Variant as sys::GodotFfi>::sys_const(&what),
                <i64 as sys::GodotFfi>::sys_const(&from),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn count(&self, value: Variant) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("count");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1481661226i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn has(&self, value: Variant) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("has");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3680194679i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Variant as sys::GodotFfi>::sys_const(&value)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn pop_back(&mut self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("pop_back");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1321915136i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn pop_front(&mut self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("pop_front");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1321915136i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn pop_at(&mut self, position: i64) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("pop_at");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3518259424i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<i64 as sys::GodotFfi>::sys_const(&position)];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn sort(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("sort");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn sort_custom(&mut self, func: Callable) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("sort_custom");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3470848906i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&func)];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn shuffle(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("shuffle");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn bsearch(&self, value: Variant, before: bool) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("bsearch");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3372222236i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Variant as sys::GodotFfi>::sys_const(&value),
                <bool as sys::GodotFfi>::sys_const(&before),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn bsearch_custom(&self, value: Variant, func: Callable, before: bool) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("bsearch_custom");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                161317131i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Variant as sys::GodotFfi>::sys_const(&value),
                <Callable as sys::GodotFfi>::sys_const(&func),
                <bool as sys::GodotFfi>::sys_const(&before),
            ];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn reverse(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("reverse");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn duplicate(&self, deep: bool) -> VariantArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("duplicate");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                636440122i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<bool as sys::GodotFfi>::sys_const(&deep)];
            let __args_ptr = __args.as_ptr();
            <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn slice(&self, begin: i64, end: i64, step: i64, deep: bool) -> VariantArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("slice");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1393718243i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <i64 as sys::GodotFfi>::sys_const(&begin),
                <i64 as sys::GodotFfi>::sys_const(&end),
                <i64 as sys::GodotFfi>::sys_const(&step),
                <bool as sys::GodotFfi>::sys_const(&deep),
            ];
            let __args_ptr = __args.as_ptr();
            <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn filter(&self, method: Callable) -> VariantArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("filter");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4075186556i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&method)];
            let __args_ptr = __args.as_ptr();
            <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn map(&self, method: Callable) -> VariantArray {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("map");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4075186556i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&method)];
            let __args_ptr = __args.as_ptr();
            <VariantArray as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn reduce(&self, method: Callable, accum: Variant) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("reduce");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4272450342i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [
                <Callable as sys::GodotFfi>::sys_const(&method),
                <Variant as sys::GodotFfi>::sys_const(&accum),
            ];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn any(&self, method: Callable) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("any");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4129521963i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&method)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn all(&self, method: Callable) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("all");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                4129521963i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<Callable as sys::GodotFfi>::sys_const(&method)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn max(&self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("max");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1460142086i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn min(&self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("min");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1460142086i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_typed(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("is_typed");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn is_same_typed(&self, array: VariantArray) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("is_same_typed");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                2988181878i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [<VariantArray as sys::GodotFfi>::sys_const(&array)];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_typed_builtin(&self) -> i64 {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("get_typed_builtin");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3173160232i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <i64 as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_typed_class_name(&self) -> StringName {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("get_typed_class_name");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1825232092i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <StringName as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn get_typed_script(&self) -> Variant {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("get_typed_script");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                1460142086i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <Variant as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
    pub fn make_read_only(&mut self) {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("make_read_only");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3218959716i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            let return_ptr = std::ptr::null_mut();
            __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
        }
    }
    pub fn is_read_only(&self) -> bool {
        unsafe {
            let __variant_type = sys::GDEXTENSION_VARIANT_TYPE_ARRAY;
            let __method_name = StringName::from("is_read_only");
            let __call_fn = sys::interface_fn!(variant_get_ptr_builtin_method)(
                __variant_type,
                __method_name.string_sys(),
                3918633141i64,
            );
            let __call_fn = __call_fn.unwrap_unchecked();
            let __args = [];
            let __args_ptr = __args.as_ptr();
            <bool as sys::GodotFfi>::from_sys_init_default(|return_ptr| {
                __call_fn(self.sys_ptr, __args_ptr, return_ptr, __args.len() as i32);
            })
        }
    }
}
