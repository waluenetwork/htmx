use wasm_bindgen_test::*;
use htmx_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_real_websocket_connection() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    element.set_attribute("ws-connect", "ws://localhost:8083/ws").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Real WebSocket connection should be established");
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    assert!(htmx.has_websocket_connection("ws://localhost:8083/ws"));
}

#[wasm_bindgen_test]
async fn test_real_sse_connection() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("sse").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    element.set_attribute("sse-connect", "http://localhost:8082/events").unwrap();
    element.set_attribute("sse-swap", "message").unwrap();
    
    let result = htmx.process_element(&element);
    assert!(result.is_ok(), "Real SSE connection should be established");
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    assert!(htmx.has_sse_connection("http://localhost:8082/events"));
}

#[wasm_bindgen_test]
async fn test_websocket_message_handling() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let container = document.create_element("div").unwrap();
    container.set_attribute("ws-connect", "ws://localhost:8083/ws").unwrap();
    
    let target = document.create_element("div").unwrap();
    target.set_attribute("id", "ws-target").unwrap();
    
    document.body().unwrap().append_child(&container).unwrap();
    document.body().unwrap().append_child(&target).unwrap();
    
    htmx.process_element(&container).unwrap();
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    let test_message = "{\"content\": \"<p>WebSocket test message</p>\", \"target\": \"#ws-target\"}";
    htmx.process_websocket_message(test_message, &container).unwrap();
    
    let target_content = target.inner_html();
    assert!(target_content.contains("WebSocket test message"));
}

#[wasm_bindgen_test]
async fn test_sse_event_handling() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("sse").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let container = document.create_element("div").unwrap();
    container.set_attribute("sse-connect", "http://localhost:8082/events").unwrap();
    container.set_attribute("sse-swap", "message").unwrap();
    
    let target = document.create_element("div").unwrap();
    target.set_attribute("id", "sse-target").unwrap();
    
    container.append_child(&target).unwrap();
    document.body().unwrap().append_child(&container).unwrap();
    
    htmx.process_element(&container).unwrap();
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    let event_data = "<p>SSE test event</p>";
    htmx.process_sse_event(event_data, &container).unwrap();
    
    let target_content = target.inner_html();
    assert!(target_content.contains("SSE test event"));
}

#[wasm_bindgen_test]
async fn test_websocket_reconnection() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    element.set_attribute("ws-connect", "ws://localhost:8083/ws").unwrap();
    htmx.process_element(&element).unwrap();
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    htmx.simulate_websocket_disconnect("ws://localhost:8083/ws").unwrap();
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 3000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    assert!(htmx.has_websocket_connection("ws://localhost:8083/ws"));
}

#[wasm_bindgen_test]
async fn test_websocket_form_submission() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let container = document.create_element("div").unwrap();
    container.set_attribute("ws-connect", "ws://localhost:8083/ws").unwrap();
    
    let form = document.create_element("form").unwrap();
    form.set_attribute("ws-send", "").unwrap();
    
    let input = document.create_element("input").unwrap();
    input.set_attribute("name", "message").unwrap();
    input.set_attribute("value", "test message").unwrap();
    
    form.append_child(&input).unwrap();
    container.append_child(&form).unwrap();
    document.body().unwrap().append_child(&container).unwrap();
    
    htmx.process_element(&container).unwrap();
    htmx.process_element(&form).unwrap();
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 1000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    let submit_event = document.create_event("Event").unwrap();
    submit_event.init_event("submit");
    form.dispatch_event(&submit_event).unwrap();
    
    assert!(htmx.has_pending_websocket_messages("ws://localhost:8083/ws"));
}

#[wasm_bindgen_test]
async fn test_connection_error_handling() {
    let mut htmx = HtmxWasm::new();
    htmx.enable_extension("ws").unwrap();
    htmx.enable_extension("sse").unwrap();
    
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    let ws_element = document.create_element("div").unwrap();
    ws_element.set_attribute("ws-connect", "ws://unreachable:9999/ws").unwrap();
    
    let result = htmx.process_element(&ws_element);
    assert!(result.is_ok(), "Should handle unreachable WebSocket gracefully");
    
    let sse_element = document.create_element("div").unwrap();
    sse_element.set_attribute("sse-connect", "http://unreachable:9999/events").unwrap();
    
    let result = htmx.process_element(&sse_element);
    assert!(result.is_ok(), "Should handle unreachable SSE gracefully");
    
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        web_sys::window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 2000).unwrap();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    
    assert!(!htmx.has_websocket_connection("ws://unreachable:9999/ws"));
    assert!(!htmx.has_sse_connection("http://unreachable:9999/events"));
}
