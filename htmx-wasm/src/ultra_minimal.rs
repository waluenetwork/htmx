use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HtmxUltraMinimal {
    initialized: bool,
}

#[wasm_bindgen]
impl HtmxUltraMinimal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmxUltraMinimal {
        HtmxUltraMinimal { initialized: true }
    }
    
    pub fn process_element(&self, element: &web_sys::Element) -> Result<(), JsValue> {
        if let Some(hx_get) = element.get_attribute("hx-get") {
            web_sys::console::log_1(&format!("Processing hx-get: {}", hx_get).into());
        }
        Ok(())
    }
    
    pub fn get_version(&self) -> String {
        "0.1.0-ultra".to_string()
    }
}
