use wasm_bindgen::prelude::*;
use web_sys::Element;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct HtmxCore;

#[wasm_bindgen]
impl HtmxCore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> HtmxCore {
        let instance = HtmxCore;
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
                                let _ = self.process_element(&el);
                            }
                        }
                    }
                }
            }
        }
    }
    
    #[wasm_bindgen(js_name = processElement)]
    pub fn process_element(&self, element: &Element) -> Result<(), JsValue> {
        self.setup_request_handlers(element)
    }
    
    fn setup_request_handlers(&self, element: &Element) -> Result<(), JsValue> {
        let verbs = ["get", "post", "put", "delete", "patch"];
        
        for verb in &verbs {
            let attr_name = format!("hx-{}", verb);
            if let Some(url) = element.get_attribute(&attr_name) {
                self.setup_request_handler(element, verb, &url)?;
            }
        }
        
        Ok(())
    }
    
    fn setup_request_handler(&self, element: &Element, verb: &str, url: &str) -> Result<(), JsValue> {
        let trigger = element.get_attribute("hx-trigger").unwrap_or_else(|| {
            match element.tag_name().as_str() {
                "FORM" => "submit".to_string(),
                "INPUT" | "TEXTAREA" | "SELECT" => "change".to_string(),
                _ => "click".to_string(),
            }
        });
        
        let element_clone = element.clone();
        let verb_clone = verb.to_string();
        let url_clone = url.to_string();
        
        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::Event| {
            event.prevent_default();
            let _ = HtmxCore::make_request(&element_clone, &verb_clone, &url_clone);
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        element.add_event_listener_with_callback(&trigger, closure.as_ref().unchecked_ref())?;
        closure.forget();
        
        Ok(())
    }
    
    fn make_request(element: &Element, verb: &str, url: &str) -> Result<(), JsValue> {
        let xhr = web_sys::XmlHttpRequest::new()?;
        xhr.open(verb, url)?;
        xhr.set_request_header("HX-Request", "true")?;
        
        let element_clone = element.clone();
        let onload_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Ok(xhr) = _event.target().unwrap().dyn_into::<web_sys::XmlHttpRequest>() {
                if xhr.status().unwrap() == 200 {
                    if let Ok(response_text) = xhr.response_text() {
                        if let Some(text) = response_text {
                            let _ = HtmxCore::swap_content(&element_clone, &text);
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(web_sys::Event)>);
        
        xhr.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
        onload_callback.forget();
        
        xhr.send()?;
        Ok(())
    }
    
    fn swap_content(element: &Element, content: &str) -> Result<(), JsValue> {
        let target = if let Some(target_selector) = element.get_attribute("hx-target") {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    document.query_selector(&target_selector)?.unwrap_or_else(|| element.clone())
                } else {
                    element.clone()
                }
            } else {
                element.clone()
            }
        } else {
            element.clone()
        };
        
        let swap_style = element.get_attribute("hx-swap").unwrap_or_else(|| "innerHTML".to_string());
        
        match swap_style.as_str() {
            "innerHTML" => target.set_inner_html(content),
            "outerHTML" => target.set_outer_html(content),
            "beforebegin" => target.insert_adjacent_html("beforebegin", content)?,
            "afterbegin" => target.insert_adjacent_html("afterbegin", content)?,
            "beforeend" => target.insert_adjacent_html("beforeend", content)?,
            "afterend" => target.insert_adjacent_html("afterend", content)?,
            _ => target.set_inner_html(content),
        }
        
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    web_sys::console::log_1(&"HTMX WASM Ultra-Minimal Core loaded".into());
}
