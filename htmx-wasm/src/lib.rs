use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event};
use std::collections::HashMap;

mod core;
mod extensions;
mod js_bridge;

pub use core::*;
pub use extensions::*;
pub use js_bridge::*;

#[wasm_bindgen]
pub struct HtmxWasm {
    core: HtmxCore,
    extension_registry: ExtensionRegistry,
    js_extensions: HashMap<String, JsValue>,
}

#[wasm_bindgen]
impl HtmxWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmxWasm {
        console_error_panic_hook::set_once();
        
        HtmxWasm {
            core: HtmxCore::new(),
            extension_registry: ExtensionRegistry::new(),
            js_extensions: HashMap::new(),
        }
    }
    
    pub fn process_element(&mut self, element: &Element) -> Result<(), JsValue> {
        self.core.process_element(element)
    }
    
    pub fn enable_extension(&mut self, name: &str) -> Result<(), JsValue> {
        self.extension_registry.enable_extension(name)
    }
    
    pub fn register_js_extension(&mut self, name: &str, extension: JsValue) {
        self.js_extensions.insert(name.to_string(), extension);
    }
    
    pub fn trigger_event(&self, element: &Element, event_name: &str, detail: &JsValue) -> Result<(), JsValue> {
        self.core.trigger_event(element, event_name, detail)
    }
    
    pub fn find(&self, selector: &str) -> Option<Element> {
        self.core.find(selector)
    }
    
    pub fn find_all(&self, selector: &str) -> Vec<Element> {
        self.core.find_all(selector)
    }
}
