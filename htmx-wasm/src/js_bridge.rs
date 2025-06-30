use wasm_bindgen::prelude::*;
use std::collections::HashMap;

#[wasm_bindgen]
pub struct JSExtensionBridge {
    extensions: HashMap<String, JsValue>,
}

#[wasm_bindgen]
impl JSExtensionBridge {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JSExtensionBridge {
        JSExtensionBridge {
            extensions: HashMap::new(),
        }
    }
    
    pub fn register_extension(&mut self, name: &str, extension: JsValue) {
        self.extensions.insert(name.to_string(), extension.clone());
        
        if let Ok(init_fn) = js_sys::Reflect::get(&extension, &JsValue::from_str("init")) {
            if init_fn.is_function() {
                let func = init_fn.dyn_into::<js_sys::Function>().unwrap();
                let api = self.create_api_object();
                let _ = func.call1(&extension, &api);
            }
        }
    }
    
    pub fn call_extension_hook(&self, ext_name: &str, hook: &str, args: &JsValue) -> Result<JsValue, JsValue> {
        let extension = self.extensions.get(ext_name).ok_or("Extension not found")?;
        let hook_fn = js_sys::Reflect::get(extension, &JsValue::from_str(hook))?;
        
        if hook_fn.is_function() {
            let func = hook_fn.dyn_into::<js_sys::Function>()?;
            func.call1(extension, args)
        } else {
            Ok(JsValue::UNDEFINED)
        }
    }
    
    pub fn get_extension_selectors(&self, ext_name: &str) -> Result<Vec<String>, JsValue> {
        let extension = self.extensions.get(ext_name).ok_or("Extension not found")?;
        
        if let Ok(selectors_fn) = js_sys::Reflect::get(extension, &JsValue::from_str("getSelectors")) {
            if selectors_fn.is_function() {
                let func = selectors_fn.dyn_into::<js_sys::Function>()?;
                let result = func.call0(extension)?;
                
                if let Ok(array) = result.dyn_into::<js_sys::Array>() {
                    let mut selectors = Vec::new();
                    for i in 0..array.length() {
                        if let Some(selector) = array.get(i).as_string() {
                            selectors.push(selector);
                        }
                    }
                    return Ok(selectors);
                }
            }
        }
        
        Ok(Vec::new())
    }
    
    fn create_api_object(&self) -> JsValue {
        let api = js_sys::Object::new();
        
        let find_fn = js_sys::Function::new_with_args("selector", 
            "return document.querySelector(selector);"
        );
        js_sys::Reflect::set(&api, &JsValue::from_str("find"), &find_fn).unwrap();
        
        let find_all_fn = js_sys::Function::new_with_args("selector", 
            "return Array.from(document.querySelectorAll(selector));"
        );
        js_sys::Reflect::set(&api, &JsValue::from_str("findAll"), &find_all_fn).unwrap();
        
        let trigger_fn = js_sys::Function::new_with_args("element,eventName,detail", 
            "const event = new CustomEvent(eventName, { detail }); element.dispatchEvent(event);"
        );
        js_sys::Reflect::set(&api, &JsValue::from_str("trigger"), &trigger_fn).unwrap();
        
        JsValue::from(api)
    }
    
    pub fn has_extension(&self, name: &str) -> bool {
        self.extensions.contains_key(name)
    }
    
    pub fn remove_extension(&mut self, name: &str) -> bool {
        self.extensions.remove(name).is_some()
    }
}
