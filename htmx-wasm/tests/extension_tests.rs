use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use htmx_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_htmx_wasm_initialization() {
    let htmx = HtmxWasm::new();
    assert!(htmx.is_initialized());
}

#[wasm_bindgen_test]
fn test_core_functionality() {
    let mut htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let element = document.create_element("div").unwrap();
    element.set_attribute("hx-get", "/api/test").unwrap();
    element.set_attribute("hx-target", "#result").unwrap();
    element.set_attribute("hx-trigger", "click").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Element processing should succeed");
}

#[cfg(feature = "websocket")]
#[wasm_bindgen_test]
fn test_websocket_extension_enable() {
    let mut htmx = HtmxWasm::new();
    let result = htmx.enable_extension("ws");
    assert!(result.is_ok(), "WebSocket extension should enable successfully");
    assert!(htmx.is_extension_enabled("ws"), "WebSocket extension should be enabled");
}

#[cfg(feature = "sse")]
#[wasm_bindgen_test]
fn test_sse_extension_enable() {
    let mut htmx = HtmxWasm::new();
    let result = htmx.enable_extension("sse");
    assert!(result.is_ok(), "SSE extension should enable successfully");
    assert!(htmx.is_extension_enabled("sse"), "SSE extension should be enabled");
}

#[wasm_bindgen_test]
fn test_unknown_extension_fails() {
    let mut htmx = HtmxWasm::new();
    let result = htmx.enable_extension("nonexistent");
    assert!(result.is_err(), "Unknown extension should fail to enable");
}

#[wasm_bindgen_test]
fn test_js_extension_registration() {
    let mut htmx = HtmxWasm::new();
    let js_ext = js_sys::Object::new();
    
    js_sys::Reflect::set(&js_ext, &"name".into(), &"test-extension".into()).unwrap();
    js_sys::Reflect::set(&js_ext, &"init".into(), &js_sys::Function::new_no_args("")).unwrap();
    
    htmx.register_js_extension("test", js_ext.into());
    assert!(htmx.has_js_extension("test"), "JS extension should be registered");
}

#[wasm_bindgen_test]
fn test_http_verb_attributes() {
    let mut htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let verbs = ["get", "post", "put", "delete", "patch"];
    
    for verb in verbs.iter() {
        let element = document.create_element("div").unwrap();
        element.set_attribute(&format!("hx-{}", verb), "/api/test").unwrap();
        
        let result = htmx.process_element(&element);
        assert!(result.is_ok(), "HTTP {} verb should be processed successfully", verb);
    }
}

#[wasm_bindgen_test]
fn test_attribute_parsing() {
    let htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    element.set_attribute("hx-post", "/api/data").unwrap();
    element.set_attribute("hx-trigger", "click").unwrap();
    element.set_attribute("hx-swap", "innerHTML").unwrap();
    element.set_attribute("hx-target", "#result").unwrap();
    
    let config = htmx.parse_element_config(&element).unwrap();
    assert_eq!(config.method(), "POST");
    assert_eq!(config.url(), "/api/data");
    assert_eq!(config.trigger(), "click");
    assert_eq!(config.swap(), "innerHTML");
    assert_eq!(config.target(), "#result");
}

#[wasm_bindgen_test]
fn test_form_serialization() {
    let htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let form = document.create_element("form").unwrap();
    
    let input1 = document.create_element("input").unwrap();
    input1.set_attribute("name", "username").unwrap();
    input1.set_attribute("value", "testuser").unwrap();
    
    let input2 = document.create_element("input").unwrap();
    input2.set_attribute("name", "email").unwrap();
    input2.set_attribute("value", "test@example.com").unwrap();
    
    form.append_child(&input1).unwrap();
    form.append_child(&input2).unwrap();
    
    let serialized = htmx.serialize_form(&form.dyn_into::<web_sys::HtmlFormElement>().unwrap());
    assert!(serialized.is_ok());
    let data = serialized.unwrap();
    assert!(data.contains("username=testuser"));
    assert!(data.contains("email=test%40example.com"));
}

#[cfg(feature = "websocket")]
#[wasm_bindgen_test]
fn test_websocket_connection_setup() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    element.set_attribute("ws-connect", "ws://localhost:8080/test").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "WebSocket connection setup should succeed");
}

#[cfg(feature = "sse")]
#[wasm_bindgen_test]
fn test_sse_connection_setup() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("sse").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    element.set_attribute("sse-connect", "http://localhost:8080/events").unwrap();
    element.set_attribute("sse-swap", "message").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "SSE connection setup should succeed");
}

#[wasm_bindgen_test]
fn test_multiple_extensions_enabled() {
    let mut htmx = HtmxWasm::new();
    
    #[cfg(feature = "websocket")]
    {
        assert!(htmx.enable_extension("ws").is_ok());
        assert!(htmx.is_extension_enabled("ws"));
    }
    
    #[cfg(feature = "sse")]
    {
        assert!(htmx.enable_extension("sse").is_ok());
        assert!(htmx.is_extension_enabled("sse"));
    }
}

#[wasm_bindgen_test]
fn test_event_handling() {
    let htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    let detail = js_sys::Object::new();
    js_sys::Reflect::set(&detail, &"test".into(), &"value".into()).unwrap();
    
    let result = htmx.trigger_event(&element, "htmx:test", &detail.into());
    assert!(result.is_ok(), "Event triggering should succeed");
}

#[wasm_bindgen_test]
fn test_dom_queries() {
    let htmx = HtmxWasm::new();
    
    let body = htmx.find("body");
    assert!(body.is_some(), "Should find body element");
    
    let all_elements = htmx.find_all("*");
    assert!(all_elements.len() > 0, "Should find multiple elements");
}

#[wasm_bindgen_test]
fn test_error_handling() {
    let mut htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let element = document.create_element("div").unwrap();
    element.set_attribute("hx-get", "").unwrap(); // Empty URL
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Should handle empty URLs gracefully");
    
    element.set_attribute("hx-trigger", "invalid-trigger-syntax").unwrap();
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Should handle malformed triggers gracefully");
}
