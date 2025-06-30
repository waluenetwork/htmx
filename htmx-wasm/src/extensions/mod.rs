use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, DocumentFragment};

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
}

pub struct ExtensionRegistry {
    #[cfg(feature = "websocket")]
    websocket: Option<Box<dyn HtmxExtension>>,
    #[cfg(feature = "sse")]
    sse: Option<Box<dyn HtmxExtension>>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        ExtensionRegistry {
            #[cfg(feature = "websocket")]
            websocket: None,
            #[cfg(feature = "sse")]
            sse: None,
        }
    }
    
    pub fn enable_extension(&mut self, name: &str) -> Result<(), JsValue> {
        match name {
            #[cfg(feature = "websocket")]
            "ws" => {
                self.websocket = Some(Box::new(WebSocketExtension::new()));
                Ok(())
            },
            #[cfg(feature = "sse")]
            "sse" => {
                self.sse = Some(Box::new(SSEExtension::new()));
                Ok(())
            },
            _ => Err(JsValue::from_str(&format!("Unknown extension: {}", name))),
        }
    }
    
    pub fn get_selectors(&self) -> Vec<String> {
        let mut selectors = Vec::new();
        
        #[cfg(feature = "websocket")]
        if let Some(ref ext) = self.websocket {
            selectors.extend(ext.selectors().iter().map(|s| s.to_string()));
        }
        
        #[cfg(feature = "sse")]
        if let Some(ref ext) = self.sse {
            selectors.extend(ext.selectors().iter().map(|s| s.to_string()));
        }
        
        selectors
    }
}
