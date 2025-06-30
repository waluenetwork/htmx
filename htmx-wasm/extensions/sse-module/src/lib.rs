use wasm_bindgen::prelude::*;
use web_sys::{EventSource, MessageEvent, Element};
use std::collections::HashMap;

#[wasm_bindgen]
pub struct SSEExtensionModule {
    event_sources: HashMap<String, EventSource>,
    reconnect_delays: HashMap<String, u32>,
}

#[wasm_bindgen]
impl SSEExtensionModule {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SSEExtensionModule {
        console_error_panic_hook::set_once();
        
        SSEExtensionModule {
            event_sources: HashMap::new(),
            reconnect_delays: HashMap::new(),
        }
    }
    
    pub fn register_with_core(&self, core: &JsValue) -> Result<(), JsValue> {
        let extension_def = js_sys::Object::new();
        
        js_sys::Reflect::set(&extension_def, &JsValue::from_str("name"), &JsValue::from_str("sse"))?;
        js_sys::Reflect::set(&extension_def, &JsValue::from_str("wasmNative"), &JsValue::from_bool(true))?;
        
        let selectors = js_sys::Array::new();
        selectors.push(&JsValue::from_str("[sse-connect]"));
        selectors.push(&JsValue::from_str("[sse-swap]"));
        js_sys::Reflect::set(&extension_def, &JsValue::from_str("selectors"), &selectors)?;
        
        if let Ok(register_fn) = js_sys::Reflect::get(core, &JsValue::from_str("registerExtension")) {
            if register_fn.is_function() {
                let func = register_fn.dyn_into::<js_sys::Function>()?;
                func.call2(core, &JsValue::from_str("sse"), &extension_def)?;
            }
        }
        
        Ok(())
    }
    
    pub fn create_event_source(&mut self, url: &str, element: &Element) -> Result<(), JsValue> {
        let event_source = EventSource::new(url)?;
        
        let element_clone = element.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(data) = event.data().dyn_into::<js_sys::JsString>() {
                let data_str = data.as_string().unwrap();
                let _ = Self::process_message(&data_str, &element_clone);
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        event_source.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        let onopen_callback = Closure::wrap(Box::new(move |_event: JsValue| {
            web_sys::console::log_1(&"Modular SSE connection opened".into());
        }) as Box<dyn FnMut(JsValue)>);
        
        event_source.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        let onerror_callback = Closure::wrap(Box::new(move |_event: JsValue| {
            web_sys::console::error_1(&"Modular SSE connection error".into());
        }) as Box<dyn FnMut(JsValue)>);
        
        event_source.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        
        self.event_sources.insert(url.to_string(), event_source);
        Ok(())
    }
    
    fn process_message(message: &str, element: &Element) -> Result<(), JsValue> {
        if let Ok(json) = js_sys::JSON::parse(message) {
            if let Ok(content) = js_sys::Reflect::get(&json, &JsValue::from_str("content")) {
                if let Some(content_str) = content.as_string() {
                    if let Ok(target_attr) = js_sys::Reflect::get(&json, &JsValue::from_str("target")) {
                        if let Some(target_selector) = target_attr.as_string() {
                            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                                if let Ok(Some(target)) = document.query_selector(&target_selector) {
                                    target.set_inner_html(&content_str);
                                    return Ok(());
                                }
                            }
                        }
                    }
                    element.set_inner_html(&content_str);
                }
            }
        } else {
            element.set_inner_html(message);
        }
        Ok(())
    }
    
    pub fn close_event_source(&mut self, url: &str) -> Result<(), JsValue> {
        if let Some(event_source) = self.event_sources.remove(url) {
            event_source.close();
        }
        Ok(())
    }
    
    pub fn get_ready_state(&self, url: &str) -> u16 {
        if let Some(event_source) = self.event_sources.get(url) {
            event_source.ready_state()
        } else {
            EventSource::CLOSED
        }
    }
}
