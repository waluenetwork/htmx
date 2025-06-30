use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub struct HtmxMinimal {
    initialized: bool,
}

#[wasm_bindgen]
impl HtmxMinimal {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmxMinimal {
        console_error_panic_hook::set_once();
        HtmxMinimal { initialized: true }
    }
    
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    pub fn process_element(&self, element: &Element) -> Result<(), JsValue> {
        let hx_get = element.get_attribute("hx-get");
        let hx_post = element.get_attribute("hx-post");
        
        if hx_get.is_some() || hx_post.is_some() {
            web_sys::console::log_1(&"Processing htmx element".into());
            Ok(())
        } else {
            Ok(())
        }
    }
    
    pub fn get_version(&self) -> String {
        "0.1.0-minimal".to_string()
    }
}
