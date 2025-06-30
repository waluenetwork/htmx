use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Event, CustomEvent, CustomEventInit, Window, HtmlFormElement, FormData};

pub struct HtmxCore {
    _config: HtmxConfig,
    pending_requests: std::cell::RefCell<u32>,
}

#[wasm_bindgen]
pub struct ElementConfig {
    method: String,
    url: String,
    trigger: String,
    swap: String,
    target: String,
}

#[wasm_bindgen]
impl ElementConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(method: String, url: String, trigger: String, swap: String, target: String) -> ElementConfig {
        ElementConfig {
            method,
            url,
            trigger,
            swap,
            target,
        }
    }
    
    pub fn method(&self) -> String { self.method.clone() }
    pub fn url(&self) -> String { self.url.clone() }
    pub fn trigger(&self) -> String { self.trigger.clone() }
    pub fn swap(&self) -> String { self.swap.clone() }
    pub fn target(&self) -> String { self.target.clone() }
}

pub struct HtmxConfig {
    pub history_enabled: bool,
    pub default_swap_style: String,
    pub default_swap_delay: u32,
    pub default_settle_delay: u32,
}

impl Default for HtmxConfig {
    fn default() -> Self {
        HtmxConfig {
            history_enabled: true,
            default_swap_style: "innerHTML".to_string(),
            default_swap_delay: 0,
            default_settle_delay: 20,
        }
    }
}

impl HtmxCore {
    pub fn new() -> Self {
        HtmxCore {
            _config: HtmxConfig::default(),
            pending_requests: std::cell::RefCell::new(0),
        }
    }
    
    pub fn process_element(&self, element: &Element) -> Result<(), JsValue> {
        self.process_hx_attributes(element)?;
        Ok(())
    }
    
    fn process_hx_attributes(&self, element: &Element) -> Result<(), JsValue> {
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
        
        let closure = Closure::wrap(Box::new(move |event: Event| {
            event.prevent_default();
            let _ = HtmxCore::make_request(&element_clone, &verb_clone, &url_clone);
        }) as Box<dyn FnMut(Event)>);
        
        element.add_event_listener_with_callback(&trigger, closure.as_ref().unchecked_ref())?;
        closure.forget();
        
        Ok(())
    }
    
    fn make_request(element: &Element, verb: &str, url: &str) -> Result<(), JsValue> {
        let xhr = web_sys::XmlHttpRequest::new()?;
        xhr.open(verb, url)?;
        xhr.set_request_header("HX-Request", "true")?;
        
        let element_clone = element.clone();
        let onload_callback = Closure::wrap(Box::new(move |_event: Event| {
            if let Ok(xhr) = _event.target().unwrap().dyn_into::<web_sys::XmlHttpRequest>() {
                if xhr.status().unwrap() == 200 {
                    if let Ok(response_text) = xhr.response_text() {
                        if let Some(text) = response_text {
                            let _ = HtmxCore::swap_content(&element_clone, &text);
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(Event)>);
        
        xhr.set_onload(Some(onload_callback.as_ref().unchecked_ref()));
        onload_callback.forget();
        
        xhr.send()?;
        Ok(())
    }
    
    pub fn parse_element_config(&self, element: &Element) -> Result<ElementConfig, JsValue> {
        let mut method = String::new();
        let mut url = String::new();
        
        let verbs = ["get", "post", "put", "delete", "patch"];
        for verb in &verbs {
            let attr_name = format!("hx-{}", verb);
            if let Some(attr_url) = element.get_attribute(&attr_name) {
                method = verb.to_uppercase();
                url = attr_url;
                break;
            }
        }
        
        let trigger = element.get_attribute("hx-trigger").unwrap_or_else(|| {
            match element.tag_name().as_str() {
                "FORM" => "submit".to_string(),
                "INPUT" | "TEXTAREA" | "SELECT" => "change".to_string(),
                _ => "click".to_string(),
            }
        });
        
        let swap = element.get_attribute("hx-swap").unwrap_or_else(|| "innerHTML".to_string());
        let target = element.get_attribute("hx-target").unwrap_or_else(|| "".to_string());
        
        Ok(ElementConfig {
            method,
            url,
            trigger,
            swap,
            target,
        })
    }
    
    pub fn serialize_form(&self, form: &HtmlFormElement) -> Result<String, JsValue> {
        let _form_data = FormData::new()?;
        let mut result = Vec::new();
        
        let inputs = form.query_selector_all("input, textarea, select")?;
        for i in 0..inputs.length() {
            if let Some(node) = inputs.get(i) {
                if let Ok(input) = node.dyn_into::<web_sys::HtmlInputElement>() {
                    if let Some(name) = input.get_attribute("name") {
                        let value = input.value();
                        let encoded_name = js_sys::encode_uri_component(&name);
                        let encoded_value = js_sys::encode_uri_component(&value);
                        result.push(format!("{}={}", encoded_name, encoded_value));
                    }
                }
            }
        }
        
        Ok(result.join("&"))
    }
    
    pub fn collect_form_data(&self, form: &HtmlFormElement) -> Result<FormData, JsValue> {
        FormData::new_with_form(form)
    }
    
    pub fn has_pending_requests(&self) -> bool {
        *self.pending_requests.borrow() > 0
    }
    
    fn swap_content(element: &Element, content: &str) -> Result<(), JsValue> {
        let target = if let Some(target_selector) = element.get_attribute("hx-target") {
            HtmxCore::get_document()?.query_selector(&target_selector)?.unwrap_or_else(|| element.clone())
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
    
    pub fn trigger_event(&self, element: &Element, event_name: &str, detail: &JsValue) -> Result<(), JsValue> {
        let event_init = CustomEventInit::new();
        event_init.set_detail(detail);
        
        let event = CustomEvent::new_with_event_init_dict(event_name, &event_init)?;
        element.dispatch_event(&event)?;
        
        Ok(())
    }
    
    pub fn find(&self, selector: &str) -> Option<Element> {
        HtmxCore::get_document().ok()?.query_selector(selector).ok()?
    }
    
    pub fn find_all(&self, selector: &str) -> Vec<Element> {
        let document = match HtmxCore::get_document() {
            Ok(doc) => doc,
            Err(_) => return Vec::new(),
        };
        
        let node_list = match document.query_selector_all(selector) {
            Ok(list) => list,
            Err(_) => return Vec::new(),
        };
        
        let mut elements = Vec::new();
        for i in 0..node_list.length() {
            if let Some(node) = node_list.get(i) {
                if let Ok(element) = node.dyn_into::<Element>() {
                    elements.push(element);
                }
            }
        }
        
        elements
    }
    
    fn get_document() -> Result<Document, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No global window exists"))?;
        window.document().ok_or_else(|| JsValue::from_str("Should have a document on window"))
    }
    
    fn _get_window() -> Result<Window, JsValue> {
        web_sys::window().ok_or_else(|| JsValue::from_str("No global window exists"))
    }
}
