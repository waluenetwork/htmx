use wasm_bindgen::prelude::*;

#[cfg(feature = "ultra-minimal")]
mod ultra_minimal_core;

#[cfg(feature = "ultra-minimal")]
pub use ultra_minimal_core::*;

#[cfg(not(feature = "ultra-minimal"))]
use web_sys::Element;
#[cfg(not(feature = "ultra-minimal"))]
use std::collections::HashMap;

#[cfg(not(feature = "ultra-minimal"))]
mod core;
#[cfg(not(feature = "ultra-minimal"))]
mod extensions;
#[cfg(not(feature = "ultra-minimal"))]
mod js_bridge;

#[cfg(not(feature = "ultra-minimal"))]
pub use core::*;
#[cfg(not(feature = "ultra-minimal"))]
pub use extensions::*;
#[cfg(not(feature = "ultra-minimal"))]
pub use js_bridge::*;

#[cfg(not(feature = "ultra-minimal"))]
#[wasm_bindgen]
pub struct HtmxWasm {
    core: HtmxCore,
    extension_registry: ExtensionRegistry,
    js_extensions: HashMap<String, JsValue>,
}

#[cfg(not(feature = "ultra-minimal"))]
#[wasm_bindgen]
impl HtmxWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmxWasm {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();
        
        let instance = HtmxWasm {
            core: HtmxCore::new(),
            extension_registry: ExtensionRegistry::new(),
            js_extensions: HashMap::new(),
        };
        
        instance.scan_dom();
        
        instance
    }
    
    pub fn scan_dom(&self) {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Ok(elements) = document.query_selector_all("[hx-get], [hx-post], [hx-put], [hx-delete], [hx-patch]") {
                    for i in 0..elements.length() {
                        if let Some(element) = elements.get(i) {
                            if let Ok(el) = element.dyn_into::<Element>() {
                                let _ = self.core.process_element(&el);
                            }
                        }
                    }
                }
            }
        }
    }
    
    pub fn is_initialized(&self) -> bool {
        true
    }
    
    pub fn process_element(&mut self, element: &Element) -> Result<(), JsValue> {
        self.core.process_element(element)
    }
    
    pub fn enable_extension(&mut self, name: &str) -> Result<(), JsValue> {
        self.extension_registry.enable_extension(name)
    }
    
    pub fn is_extension_enabled(&self, name: &str) -> bool {
        self.extension_registry.is_enabled(name)
    }
    
    pub fn register_js_extension(&mut self, name: &str, extension: JsValue) {
        self.js_extensions.insert(name.to_string(), extension);
    }
    
    pub fn has_js_extension(&self, name: &str) -> bool {
        self.js_extensions.contains_key(name)
    }
    
    pub fn parse_element_config(&self, element: &Element) -> Result<ElementConfig, JsValue> {
        self.core.parse_element_config(element)
    }
    
    pub fn serialize_form(&self, form: &web_sys::HtmlFormElement) -> Result<String, JsValue> {
        self.core.serialize_form(form)
    }
    
    pub fn collect_form_data(&self, form: &web_sys::HtmlFormElement) -> Result<web_sys::FormData, JsValue> {
        self.core.collect_form_data(form)
    }
    
    pub fn has_pending_requests(&self) -> bool {
        self.core.has_pending_requests()
    }
    
    pub fn has_websocket_connection(&self, url: &str) -> bool {
        self.extension_registry.has_websocket_connection(url)
    }
    
    pub fn has_sse_connection(&self, url: &str) -> bool {
        self.extension_registry.has_sse_connection(url)
    }
    
    pub fn process_websocket_message(&mut self, message: &str, element: &Element) -> Result<(), JsValue> {
        self.extension_registry.process_websocket_message(message, element)
    }
    
    pub fn process_sse_event(&mut self, event_data: &str, element: &Element) -> Result<(), JsValue> {
        self.extension_registry.process_sse_event(event_data, element)
    }
    
    pub fn simulate_websocket_disconnect(&mut self, url: &str) -> Result<(), JsValue> {
        self.extension_registry.simulate_websocket_disconnect(url)
    }
    
    pub fn has_pending_websocket_messages(&self, url: &str) -> bool {
        self.extension_registry.has_pending_websocket_messages(url)
    }
    
    pub fn call_js_extension_hook(&self, ext_name: &str, hook: &str, args: &JsValue) -> Result<JsValue, JsValue> {
        if let Some(extension) = self.js_extensions.get(ext_name) {
            let hook_fn = js_sys::Reflect::get(extension, &JsValue::from_str(hook))?;
            if let Ok(func) = hook_fn.dyn_into::<js_sys::Function>() {
                func.call1(extension, args)
            } else {
                Ok(JsValue::UNDEFINED)
            }
        } else {
            Err(JsValue::from_str("Extension not found"))
        }
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
