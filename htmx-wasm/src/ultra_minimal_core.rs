use wasm_bindgen::prelude::*;
use web_sys::Element;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct HtmxCore;

#[wasm_bindgen]
impl HtmxCore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmxCore {
        HtmxCore
    }
    
    #[wasm_bindgen(js_name = processElement)]
    pub fn process_element(&self, element: &Element) -> Result<(), JsValue> {
        if let Some(get_attr) = element.get_attribute("hx-get") {
            web_sys::console::log_1(&format!("GET {}", get_attr).into());
        }
        if let Some(post_attr) = element.get_attribute("hx-post") {
            web_sys::console::log_1(&format!("POST {}", post_attr).into());
        }
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"HTMX WASM Ultra-Minimal Core loaded".into());
}
