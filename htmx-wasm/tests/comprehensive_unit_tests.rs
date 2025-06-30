use wasm_bindgen_test::*;
use htmx_wasm::*;
use web_sys::{Element, HtmlFormElement};
use wasm_bindgen::JsCast;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_htmx_core_initialization() {
    let htmx = HtmxWasm::new();
    assert!(htmx.is_initialized());
}

#[wasm_bindgen_test]
fn test_websocket_extension_comprehensive() {
    let mut htmx = HtmxWasm::new();
    
    assert!(htmx.enable_extension("ws").is_ok());
    
    let mock_element = create_mock_element_with_attrs(&[
        ("ws-connect", "ws://localhost:8080/test"),
        ("id", "test-ws-element")
    ]);
    
    assert!(htmx.process_element(&mock_element).is_ok());
}

#[wasm_bindgen_test]
fn test_sse_extension_comprehensive() {
    let mut htmx = HtmxWasm::new();
    
    assert!(htmx.enable_extension("sse").is_ok());
    
    let mock_element = create_mock_element_with_attrs(&[
        ("sse-connect", "http://localhost:8080/events"),
        ("sse-swap", "message,update"),
        ("id", "test-sse-element")
    ]);
    
    assert!(htmx.process_element(&mock_element).is_ok());
}

#[wasm_bindgen_test]
fn test_http_verb_processing() {
    let htmx = HtmxWasm::new();
    
    let verbs = ["get", "post", "put", "delete", "patch"];
    
    for verb in &verbs {
        let mock_element = create_mock_element_with_attrs(&[
            (&format!("hx-{}", verb), "/api/test"),
            ("hx-trigger", "click"),
            ("hx-target", "#result")
        ]);
        
        let config = htmx.parse_element_config(&mock_element);
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.method(), verb.to_uppercase());
        assert_eq!(config.url(), "/api/test");
        assert_eq!(config.trigger(), "click");
        assert_eq!(config.target(), "#result");
    }
}

#[wasm_bindgen_test]
fn test_form_serialization_comprehensive() {
    let htmx = HtmxWasm::new();
    
    let form = create_mock_form_with_inputs(&[
        ("text", "username", "testuser"),
        ("email", "email", "test@example.com"),
        ("password", "password", "secret123"),
        ("hidden", "csrf_token", "abc123"),
        ("checkbox", "subscribe", "on"),
    ]);
    
    let serialized = htmx.serialize_form(&form);
    assert!(serialized.is_ok());
    
    let result = serialized.unwrap();
    assert!(result.contains("username=testuser"));
    assert!(result.contains("email=test%40example.com"));
    assert!(result.contains("csrf_token=abc123"));
}

#[wasm_bindgen_test]
fn test_swap_strategies() {
    let htmx = HtmxWasm::new();
    
    let swap_strategies = [
        "innerHTML", "outerHTML", "beforebegin", 
        "afterbegin", "beforeend", "afterend"
    ];
    
    for strategy in &swap_strategies {
        let mock_element = create_mock_element_with_attrs(&[
            ("hx-get", "/api/content"),
            ("hx-swap", strategy),
            ("id", "test-target")
        ]);
        
        let config = htmx.parse_element_config(&mock_element);
        assert!(config.is_ok());
        assert_eq!(config.unwrap().swap(), *strategy);
    }
}

#[wasm_bindgen_test]
fn test_trigger_parsing() {
    let htmx = HtmxWasm::new();
    
    let trigger_tests = [
        ("click", "click"),
        ("submit", "submit"),
        ("change", "change"),
        ("keyup", "keyup"),
        ("load", "load"),
        ("revealed", "revealed"),
        ("intersect", "intersect"),
    ];
    
    for (trigger, expected) in &trigger_tests {
        let mock_element = create_mock_element_with_attrs(&[
            ("hx-get", "/api/test"),
            ("hx-trigger", trigger)
        ]);
        
        let config = htmx.parse_element_config(&mock_element);
        assert!(config.is_ok());
        assert_eq!(config.unwrap().trigger(), *expected);
    }
}

#[wasm_bindgen_test]
fn test_javascript_extension_bridge() {
    let mut htmx = HtmxWasm::new();
    
    let js_extension = js_sys::Object::new();
    js_sys::Reflect::set(&js_extension, &"name".into(), &"test-extension".into()).unwrap();
    
    htmx.register_js_extension("test-extension", js_extension.into());
    
    assert!(htmx.has_js_extension("test-extension"));
}

#[wasm_bindgen_test]
fn test_error_handling() {
    let mut htmx = HtmxWasm::new();
    
    assert!(htmx.enable_extension("unknown-extension").is_err());
    
    let empty_element = create_mock_element_with_attrs(&[]);
    let config = htmx.parse_element_config(&empty_element);
    assert!(config.is_ok()); // Should handle gracefully with defaults
}

#[wasm_bindgen_test]
fn test_modular_extension_loading() {
    let mut htmx = HtmxWasm::new();
    
    assert!(htmx.enable_extension("ws").is_ok());
    assert!(htmx.is_extension_enabled("ws"));
    
    assert!(htmx.enable_extension("sse").is_ok());
    assert!(htmx.is_extension_enabled("sse"));
}

#[wasm_bindgen_test]
fn test_performance_metrics() {
    let mut htmx = HtmxWasm::new();
    
    let start = js_sys::Date::now();
    
    for i in 0..100 {
        let mock_element = create_mock_element_with_attrs(&[
            ("hx-get", &format!("/api/test/{}", i)),
            ("hx-trigger", "click"),
            ("id", &format!("element-{}", i))
        ]);
        
        assert!(htmx.process_element(&mock_element).is_ok());
    }
    
    let duration = js_sys::Date::now() - start;
    
    assert!(duration < 100.0);
}

fn create_mock_element_with_attrs(attrs: &[(&str, &str)]) -> Element {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    for (name, value) in attrs {
        element.set_attribute(name, value).unwrap();
    }
    
    element
}

fn create_mock_form_with_inputs(inputs: &[(&str, &str, &str)]) -> HtmlFormElement {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let form = document.create_element("form").unwrap()
        .dyn_into::<HtmlFormElement>().unwrap();
    
    for (input_type, name, value) in inputs {
        let input = document.create_element("input").unwrap();
        input.set_attribute("type", input_type).unwrap();
        input.set_attribute("name", name).unwrap();
        input.set_attribute("value", value).unwrap();
        form.append_child(&input).unwrap();
    }
    
    form
}
