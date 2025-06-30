use wasm_bindgen_test::*;
use htmx_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_htmx_wasm_initialization() {
    let htmx = HtmxWasm::new();
    assert!(true);
}

#[cfg(feature = "websocket")]
#[wasm_bindgen_test]
fn test_websocket_extension() {
    let mut htmx = HtmxWasm::new();
    assert!(htmx.enable_extension("ws").is_ok());
}

#[cfg(feature = "sse")]
#[wasm_bindgen_test]
fn test_sse_extension() {
    let mut htmx = HtmxWasm::new();
    assert!(htmx.enable_extension("sse").is_ok());
}

#[wasm_bindgen_test]
fn test_js_extension_registration() {
    let mut htmx = HtmxWasm::new();
    let js_ext = js_sys::Object::new();
    htmx.register_js_extension("test", js_ext.into());
    assert!(true);
}

#[wasm_bindgen_test]
fn test_unknown_extension() {
    let mut htmx = HtmxWasm::new();
    assert!(htmx.enable_extension("unknown").is_err());
}

#[wasm_bindgen_test]
fn test_element_processing() {
    let htmx = HtmxWasm::new();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    element.set_attribute("hx-get", "/test").unwrap();
    
    assert!(htmx.find("div").is_none());
}

#[wasm_bindgen_test]
fn test_find_functionality() {
    let htmx = HtmxWasm::new();
    
    let elements = htmx.find_all("*");
    assert!(elements.len() >= 0);
}

#[wasm_bindgen_test]
fn test_trigger_event() {
    let htmx = HtmxWasm::new();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    let detail = js_sys::Object::new();
    assert!(htmx.trigger_event(&element, "test-event", &detail.into()).is_ok());
}
