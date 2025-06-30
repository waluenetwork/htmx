use wasm_bindgen::prelude::*;
use web_sys::{Element, DocumentFragment};

#[cfg(feature = "websocket")]
pub mod websocket;
#[cfg(feature = "sse")]
pub mod sse;

#[cfg(feature = "websocket")]
pub use websocket::WebSocketExtension;
#[cfg(feature = "sse")]
pub use sse::SSEExtension;

pub struct HtmxApi {
    pub find: fn(&str) -> Option<Element>,
    pub find_all: fn(&str) -> Vec<Element>,
    pub trigger_event: fn(&Element, &str, &JsValue) -> Result<(), JsValue>,
}

pub trait HtmxExtension {
    fn name(&self) -> &'static str;
    fn selectors(&self) -> Vec<&'static str>;
    fn init(&mut self, api: &HtmxApi) -> Result<(), JsValue>;
    fn on_event(&mut self, event: &str, element: &Element, detail: &JsValue) -> Result<bool, JsValue>;
    fn transform_response(&self, text: &str, element: &Element) -> Result<String, JsValue>;
    fn handle_swap(&self, swap_style: &str, target: &Element, fragment: &DocumentFragment) -> Result<bool, JsValue>;
    
    fn has_connection(&self, _url: &str) -> bool { false }
    fn process_message(&self, _message: &str, _element: &Element) -> Result<(), JsValue> { Ok(()) }
    fn process_event(&self, _data: &str, _element: &Element) -> Result<(), JsValue> { Ok(()) }
    fn simulate_disconnect(&self, _url: &str) -> Result<(), JsValue> { Ok(()) }
    fn has_pending_messages(&self, _url: &str) -> bool { false }
}

pub struct ExtensionRegistry {
    #[cfg(feature = "websocket")]
    websocket: Option<Box<dyn HtmxExtension>>,
    #[cfg(feature = "sse")]
    sse: Option<Box<dyn HtmxExtension>>,
    enabled_extensions: std::collections::HashSet<String>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        ExtensionRegistry {
            #[cfg(feature = "websocket")]
            websocket: None,
            #[cfg(feature = "sse")]
            sse: None,
            enabled_extensions: std::collections::HashSet::new(),
        }
    }
    
    pub fn enable_extension(&mut self, name: &str) -> Result<(), JsValue> {
        match name {
            #[cfg(feature = "websocket")]
            "ws" => {
                self.websocket = Some(Box::new(WebSocketExtension::new()));
                self.enabled_extensions.insert(name.to_string());
                Ok(())
            },
            #[cfg(feature = "sse")]
            "sse" => {
                self.sse = Some(Box::new(SSEExtension::new()));
                self.enabled_extensions.insert(name.to_string());
                Ok(())
            },
            _ => Err(JsValue::from_str(&format!("Unknown extension: {}", name))),
        }
    }
    
    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled_extensions.contains(name)
    }
    
    pub fn has_websocket_connection(&self, _url: &str) -> bool {
        #[cfg(feature = "websocket")]
        {
            if let Some(ref ws_ext) = self.websocket {
                return ws_ext.has_connection(_url);
            }
        }
        false
    }
    
    pub fn process_websocket_message(&self, message: &str, element: &Element) -> Result<(), JsValue> {
        #[cfg(feature = "websocket")]
        {
            if let Some(ref ws_ext) = self.websocket {
                return ws_ext.process_message(message, element);
            }
        }
        Ok(())
    }
    
    pub fn simulate_websocket_disconnect(&self, url: &str) -> Result<(), JsValue> {
        #[cfg(feature = "websocket")]
        {
            if let Some(ref ws_ext) = self.websocket {
                return ws_ext.simulate_disconnect(url);
            }
        }
        Ok(())
    }
    
    pub fn has_pending_websocket_messages(&self, _url: &str) -> bool {
        #[cfg(feature = "websocket")]
        {
            if let Some(ref ws_ext) = self.websocket {
                return ws_ext.has_pending_messages(_url);
            }
        }
        false
    }
    
    pub fn has_sse_connection(&self, _url: &str) -> bool {
        #[cfg(feature = "sse")]
        {
            if let Some(ref sse_ext) = self.sse {
                return sse_ext.has_connection(_url);
            }
        }
        false
    }
    
    pub fn process_sse_event(&self, data: &str, element: &Element) -> Result<(), JsValue> {
        #[cfg(feature = "sse")]
        {
            if let Some(ref sse_ext) = self.sse {
                return sse_ext.process_event(data, element);
            }
        }
        Ok(())
    }
    
    pub fn get_selectors(&self) -> Vec<String> {
        let selectors = Vec::new();
        
        #[cfg(feature = "websocket")]
        if let Some(ref ext) = self.websocket {
            // selectors.extend(ext.selectors().iter().map(|s| s.to_string()));
        }
        
        #[cfg(feature = "sse")]
        if let Some(ref ext) = self.sse {
            // selectors.extend(ext.selectors().iter().map(|s| s.to_string()));
        }
        
        selectors
    }
}
