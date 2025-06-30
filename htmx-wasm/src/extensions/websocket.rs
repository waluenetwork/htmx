use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent, Element, DocumentFragment};
use std::collections::HashMap;
use super::{HtmxExtension, HtmxApi};

pub struct WebSocketExtension {
    connections: HashMap<String, WebSocket>,
    _reconnect_delays: HashMap<String, u32>,
    _message_queues: HashMap<String, Vec<String>>,
}

impl WebSocketExtension {
    pub fn new() -> Self {
        WebSocketExtension {
            connections: HashMap::new(),
            _reconnect_delays: HashMap::new(),
            _message_queues: HashMap::new(),
        }
    }
    
    fn create_connection(&mut self, url: &str, element: &Element) -> Result<(), JsValue> {
        let socket = WebSocket::new(url)?;
        
        let onopen_callback = Closure::wrap(Box::new(move |_event: JsValue| {
            web_sys::console::log_1(&"WebSocket connection opened".into());
        }) as Box<dyn FnMut(JsValue)>);
        
        socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        let element_clone = element.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(message) = event.data().dyn_into::<js_sys::JsString>() {
                let message_str = message.as_string().unwrap();
                let _ = Self::process_htmx_message(&message_str, &element_clone);
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        let _url_clone = url.to_string();
        let onclose_callback = Closure::wrap(Box::new(move |event: CloseEvent| {
            web_sys::console::log_2(&"WebSocket closed:".into(), &event.code().into());
        }) as Box<dyn FnMut(CloseEvent)>);
        
        socket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
        
        let onerror_callback = Closure::wrap(Box::new(move |_event: ErrorEvent| {
            web_sys::console::error_1(&"WebSocket error".into());
        }) as Box<dyn FnMut(ErrorEvent)>);
        
        socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
        
        self.connections.insert(url.to_string(), socket);
        Ok(())
    }
    
    fn process_htmx_message(message: &str, element: &Element) -> Result<(), JsValue> {
        if let Ok(json) = js_sys::JSON::parse(message) {
            if let Ok(content) = js_sys::Reflect::get(&json, &JsValue::from_str("content")) {
                if let Some(content_str) = content.as_string() {
                    element.set_inner_html(&content_str);
                }
            }
        } else {
            element.set_inner_html(message);
        }
        Ok(())
    }
    
    fn send_message(&self, url: &str, message: &str) -> Result<(), JsValue> {
        if let Some(socket) = self.connections.get(url) {
            socket.send_with_str(message)?;
        }
        Ok(())
    }
}

impl HtmxExtension for WebSocketExtension {
    fn name(&self) -> &'static str { 
        "ws" 
    }
    
    fn selectors(&self) -> Vec<&'static str> {
        vec!["[ws-connect]", "[ws-send]"]
    }
    
    fn init(&mut self, api: &HtmxApi) -> Result<(), JsValue> {
        let elements = (api.find_all)("[ws-connect]");
        for element in elements {
            if let Some(url) = element.get_attribute("ws-connect") {
                self.create_connection(&url, &element)?;
            }
        }
        Ok(())
    }
    
    fn on_event(&mut self, event: &str, element: &Element, detail: &JsValue) -> Result<bool, JsValue> {
        match event {
            "htmx:beforeRequest" => {
                if element.has_attribute("ws-send") {
                    if let Some(form_data) = js_sys::Reflect::get(detail, &JsValue::from_str("formData")).ok() {
                        let message = js_sys::JSON::stringify(&form_data).unwrap().as_string().unwrap();
                        if let Some(ws_connect) = element.get_attribute("ws-connect") {
                            let _ = self.send_message(&ws_connect, &message);
                        }
                    }
                    return Ok(true);
                }
            },
            _ => {}
        }
        Ok(false)
    }
    
    fn transform_response(&self, text: &str, _element: &Element) -> Result<String, JsValue> {
        Ok(text.to_string())
    }
    
    fn handle_swap(&self, _swap_style: &str, _target: &Element, _fragment: &DocumentFragment) -> Result<bool, JsValue> {
        Ok(false)
    }
}
