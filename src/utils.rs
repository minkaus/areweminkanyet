#[macro_export]
macro_rules! js_object {
    ($($name:ident: $value:expr),* $(,)?) => {{
        let c = move || {
            let obj = js_sys::Object::new();
            $(
                js_sys::Reflect::set(
                    &obj,
                    &wasm_bindgen::JsValue::from_str(::core::stringify!($name)),
                    &$value,
                )?;
            )*
            Ok(JsValue::from(obj))
        };
        c()
    }};
}

#[inline]
pub fn get_elapsed_days_since(time: f64) -> usize {
    let delta = js_sys::Date::now() - time;
    (delta / 86_400_000.).floor() as usize
}
