use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::CustomEvent;
use htmx_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_full_request_cycle() {
    let mut htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let button = document.create_element("button").unwrap();
    button.set_attribute("hx-get", "/api/test").unwrap();
    button.set_attribute("hx-target", "#result").unwrap();
    button.set_attribute("hx-trigger", "click").unwrap();
    
    let target = document.create_element("div").unwrap();
    target.set_attribute("id", "result").unwrap();
    
    document.body().unwrap().append_child(&button).unwrap();
    document.body().unwrap().append_child(&target).unwrap();
    
    htmx.process_element(&button).unwrap();
    
    let click_event = document.create_event("MouseEvent").unwrap();
    click_event.init_event("click");
    button.dispatch_event(&click_event).unwrap();
    
    assert!(htmx.has_pending_requests());
}

#[wasm_bindgen_test]
fn test_websocket_full_integration() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let ws_container = document.create_element("div").unwrap();
    ws_container.set_attribute("ws-connect", "ws://localhost:8080/test").unwrap();
    
    let form = document.create_element("form").unwrap();
    form.set_attribute("ws-send", "").unwrap();
    
    let input = document.create_element("input").unwrap();
    input.set_attribute("name", "message").unwrap();
    input.set_attribute("value", "test message").unwrap();
    
    form.append_child(&input).unwrap();
    ws_container.append_child(&form).unwrap();
    document.body().unwrap().append_child(&ws_container).unwrap();
    
    htmx.process_element(&ws_container).unwrap();
    htmx.process_element(&form).unwrap();
    
    let submit_event = document.create_event("Event").unwrap();
    submit_event.init_event("submit");
    form.dispatch_event(&submit_event).unwrap();
    
    assert!(htmx.has_websocket_connection("ws://localhost:8080/test"));
}

#[wasm_bindgen_test]
fn test_sse_full_integration() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("sse").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let sse_container = document.create_element("div").unwrap();
    sse_container.set_attribute("sse-connect", "http://localhost:8080/events").unwrap();
    sse_container.set_attribute("sse-swap", "message").unwrap();
    
    let target = document.create_element("div").unwrap();
    target.set_attribute("id", "sse-target").unwrap();
    
    sse_container.append_child(&target).unwrap();
    document.body().unwrap().append_child(&sse_container).unwrap();
    
    htmx.process_element(&sse_container).unwrap();
    
    let mock_event = CustomEvent::new("sse-message").unwrap();
    let event_data = js_sys::Object::new();
    js_sys::Reflect::set(&event_data, &"data".into(), &"<p>SSE test message</p>".into()).unwrap();
    
    let custom_event_init = web_sys::CustomEventInit::new();
    custom_event_init.set_detail(&event_data);
    
    let sse_event = CustomEvent::new_with_event_init_dict("sse-message", &custom_event_init).unwrap();
    sse_container.dispatch_event(&sse_event.into()).unwrap();
    
    assert!(htmx.has_sse_connection("http://localhost:8080/events"));
}

#[wasm_bindgen_test]
fn test_javascript_extension_integration() {
    let mut htmx = HtmxWasm::new();
    
    let js_ext = js_sys::Object::new();
    js_sys::Reflect::set(&js_ext, &"name".into(), &"test-extension".into()).unwrap();
    
    let init_fn = js_sys::Function::new_no_args("console.log('Extension initialized')");
    js_sys::Reflect::set(&js_ext, &"init".into(), &init_fn).unwrap();
    
    let transform_fn = js_sys::Function::new_with_args("text, xhr, elt", "return text.toUpperCase()");
    js_sys::Reflect::set(&js_ext, &"transformResponse".into(), &transform_fn).unwrap();
    
    htmx.register_js_extension("test", js_ext.into());
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    let transformed = htmx.call_js_extension_hook("test", "transformResponse", &"hello world".into());
    assert!(transformed.is_ok());
}

#[wasm_bindgen_test]
fn test_mixed_extension_architecture() {
    let mut htmx = HtmxWasm::new();
    
    htmx.enable_extension("ws").unwrap();
    htmx.enable_extension("sse").unwrap();
    
    let js_ext = js_sys::Object::new();
    js_sys::Reflect::set(&js_ext, &"name".into(), &"js-test".into()).unwrap();
    htmx.register_js_extension("js-test", js_ext.into());
    
    assert!(htmx.is_extension_enabled("ws"));
    assert!(htmx.is_extension_enabled("sse"));
    assert!(htmx.has_js_extension("js-test"));
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let element = document.create_element("div").unwrap();
    element.set_attribute("hx-get", "/api/test").unwrap();
    element.set_attribute("ws-connect", "ws://localhost:8080/test").unwrap();
    element.set_attribute("sse-connect", "http://localhost:8080/events").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Mixed extension processing should work");
}

#[wasm_bindgen_test]
fn test_error_handling() {
    let mut htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let element = document.create_element("div").unwrap();
    element.set_attribute("hx-get", "invalid-url").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Should handle invalid URLs gracefully");
    
    let unknown_ext_result = htmx.enable_extension("nonexistent");
    assert!(unknown_ext_result.is_err(), "Should fail for unknown extensions");
}

#[wasm_bindgen_test]
fn test_event_propagation() {
    let mut htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let parent = document.create_element("div").unwrap();
    let child = document.create_element("button").unwrap();
    
    child.set_attribute("hx-get", "/api/test").unwrap();
    child.set_attribute("hx-trigger", "click").unwrap();
    
    parent.append_child(&child).unwrap();
    document.body().unwrap().append_child(&parent).unwrap();
    
    htmx.process_element(&child).unwrap();
    
    let click_event = document.create_event("MouseEvent").unwrap();
    click_event.init_event("click");
    
    let result = child.dispatch_event(&click_event);
    assert!(result.is_ok());
}

#[wasm_bindgen_test]
fn test_form_data_collection() {
    let htmx = HtmxWasm::new();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let form = document.create_element("form").unwrap();
    form.set_attribute("hx-post", "/api/submit").unwrap();
    
    let text_input = document.create_element("input").unwrap();
    text_input.set_attribute("type", "text").unwrap();
    text_input.set_attribute("name", "username").unwrap();
    text_input.set_attribute("value", "testuser").unwrap();
    
    let checkbox = document.create_element("input").unwrap();
    checkbox.set_attribute("type", "checkbox").unwrap();
    checkbox.set_attribute("name", "subscribe").unwrap();
    checkbox.set_attribute("checked", "").unwrap();
    
    form.append_child(&text_input).unwrap();
    form.append_child(&checkbox).unwrap();
    
    let form_element = form.dyn_into::<web_sys::HtmlFormElement>().unwrap();
    let form_data = htmx.collect_form_data(&form_element).unwrap();
    
    let serialized = htmx.serialize_form(&form_element).unwrap();
    assert!(serialized.contains("username=testuser"));
    assert!(serialized.contains("subscribe=on"));
}
