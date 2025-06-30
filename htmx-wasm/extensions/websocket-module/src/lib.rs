use wasm_bindgen::prelude::*;
use web_sys::{WebSocket, MessageEvent, CloseEvent, ErrorEvent, Element};
use std::collections::HashMap;

#[wasm_bindgen]
pub struct WebSocketExtensionModule {
    connections: HashMap<String, WebSocket>,
    reconnect_delays: HashMap<String, u32>,
    message_queues: HashMap<String, Vec<String>>,
}

#[wasm_bindgen]
impl WebSocketExtensionModule {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WebSocketExtensionModule {
        console_error_panic_hook::set_once();
        
        WebSocketExtensionModule {
            connections: HashMap::new(),
            reconnect_delays: HashMap::new(),
            message_queues: HashMap::new(),
        }
    }
    
    pub fn register_with_core(&self, core: &JsValue) -> Result<(), JsValue> {
        let extension_def = js_sys::Object::new();
        
        js_sys::Reflect::set(&extension_def, &JsValue::from_str("name"), &JsValue::from_str("ws"))?;
        js_sys::Reflect::set(&extension_def, &JsValue::from_str("wasmNative"), &JsValue::from_bool(true))?;
        
        let selectors = js_sys::Array::new();
        selectors.push(&JsValue::from_str("[ws-connect]"));
        selectors.push(&JsValue::from_str("[ws-send]"));
        js_sys::Reflect::set(&extension_def, &JsValue::from_str("selectors"), &selectors)?;
        
        if let Ok(register_fn) = js_sys::Reflect::get(core, &JsValue::from_str("registerExtension")) {
            if register_fn.is_function() {
                let func = register_fn.dyn_into::<js_sys::Function>()?;
                func.call2(core, &JsValue::from_str("ws"), &extension_def)?;
            }
        }
        
        Ok(())
    }
    
    pub fn create_connection(&mut self, url: &str, element: &Element) -> Result<(), JsValue> {
        let socket = WebSocket::new(url)?;
        
        let onopen_callback = Closure::wrap(Box::new(move |_event: JsValue| {
            web_sys::console::log_1(&"Modular WebSocket connection opened".into());
        }) as Box<dyn FnMut(JsValue)>);
        
        socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        
        let element_clone = element.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Ok(message) = event.data().dyn_into::<js_sys::JsString>() {
                let message_str = message.as_string().unwrap();
                let _ = Self::process_message(&message_str, &element_clone);
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        
        socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
        
        let url_clone = url.to_string();
        let onclose_callback = Closure::wrap(Box::new(move |event: CloseEvent| {
            web_sys::console::log_2(&"Modular WebSocket closed:".into(), &event.code().into());
        }) as Box<dyn FnMut(CloseEvent)>);
        
        socket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();
        
        self.connections.insert(url.to_string(), socket);
        Ok(())
    }
    
    fn process_message(message: &str, element: &Element) -> Result<(), JsValue> {
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
    
    pub fn send_message(&self, url: &str, message: &str) -> Result<(), JsValue> {
        if let Some(socket) = self.connections.get(url) {
            socket.send_with_str(message)?;
        }
        Ok(())
    }
    
    pub fn close_connection(&mut self, url: &str) -> Result<(), JsValue> {
        if let Some(socket) = self.connections.remove(url) {
            socket.close()?;
        }
        Ok(())
    }
    
    pub fn get_connection_state(&self, url: &str) -> u16 {
        if let Some(socket) = self.connections.get(url) {
            socket.ready_state()
        } else {
            WebSocket::CLOSED
        }
    }
}
