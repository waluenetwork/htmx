use wasm_bindgen::prelude::*;
use web_sys::{EventSource, MessageEvent, Element, DocumentFragment};
use std::collections::HashMap;
use super::{HtmxExtension, HtmxApi};

pub struct SSEExtension {
    event_sources: HashMap<String, EventSource>,
    _reconnect_delays: HashMap<String, u32>,
}

impl SSEExtension {
    pub fn new() -> Self {
        SSEExtension {
            event_sources: HashMap::new(),
            _reconnect_delays: HashMap::new(),
        }
    }
    
    fn create_event_source(&mut self, url: &str, element: &Element) -> Result<(), JsValue> {
        let event_source = EventSource::new(url)?;
        
        let element_clone = element.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(data) = event.data().dyn_into::<js_sys::JsString>() {
                let data_str = data.as_string().unwrap();
                let _ = Self::process_sse_message(&data_str, &element_clone);
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        event_source.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        let onopen_callback = Closure::wrap(Box::new(move |_event: JsValue| {
            web_sys::console::log_1(&"SSE connection opened".into());
        }) as Box<dyn FnMut(JsValue)>);
        
        event_source.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        let onerror_callback = Closure::wrap(Box::new(move |_event: JsValue| {
            web_sys::console::error_1(&"SSE connection error".into());
        }) as Box<dyn FnMut(JsValue)>);
        
        event_source.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        
        self.event_sources.insert(url.to_string(), event_source);
        Ok(())
    }
    
    fn process_sse_message(message: &str, element: &Element) -> Result<(), JsValue> {
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
    
    fn _setup_custom_event_listeners(&self, element: &Element, event_source: &EventSource) -> Result<(), JsValue> {
        let sse_swap = element.get_attribute("sse-swap");
        if let Some(events) = sse_swap {
            for event_name in events.split(',') {
                let event_name = event_name.trim();
                let element_clone = element.clone();
                let event_name_clone = event_name.to_string();
                
                let callback = Closure::wrap(Box::new(move |event: MessageEvent| {
                    if let Ok(data) = event.data().dyn_into::<js_sys::JsString>() {
                        let data_str = data.as_string().unwrap();
                        let _ = Self::process_sse_message(&data_str, &element_clone);
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
                
                event_source.add_event_listener_with_callback(&event_name_clone, callback.as_ref().unchecked_ref())?;
                callback.forget();
            }
        }
        Ok(())
    }
}

impl HtmxExtension for SSEExtension {
    fn name(&self) -> &'static str { 
        "sse" 
    }
    
    fn selectors(&self) -> Vec<&'static str> {
        vec!["[sse-connect]", "[sse-swap]"]
    }
    
    fn init(&mut self, api: &HtmxApi) -> Result<(), JsValue> {
        let elements = (api.find_all)("[sse-connect]");
        for element in elements {
            if let Some(url) = element.get_attribute("sse-connect") {
                self.create_event_source(&url, &element)?;
            }
        }
        Ok(())
    }
    
    fn on_event(&mut self, event: &str, element: &Element, _detail: &JsValue) -> Result<bool, JsValue> {
        match event {
            "htmx:load" => {
                if let Some(url) = element.get_attribute("sse-connect") {
                    if !self.event_sources.contains_key(&url) {
                        let _ = self.create_event_source(&url, element);
                    }
                }
                Ok(true)
            },
            _ => Ok(false)
        }
    }
    
    fn transform_response(&self, text: &str, _element: &Element) -> Result<String, JsValue> {
        Ok(text.to_string())
    }
    
    fn handle_swap(&self, _swap_style: &str, _target: &Element, _fragment: &DocumentFragment) -> Result<bool, JsValue> {
        Ok(false)
    }
}
