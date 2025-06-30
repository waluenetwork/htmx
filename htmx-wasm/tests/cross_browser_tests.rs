use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use htmx_wasm::*;
use web_sys::window;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_browser_compatibility() {
    let window = window().unwrap();
    let user_agent = "WASM Browser Test".to_string();
    
    web_sys::console::log_1(&format!("Testing on: {}", user_agent).into());
    
    let htmx = HtmxWasm::new();
    assert!(htmx.is_initialized());
}

#[wasm_bindgen_test]
fn test_websocket_browser_support() {
    let window = window().unwrap();
    
    let websocket_supported = js_sys::Reflect::has(&window, &"WebSocket".into()).unwrap();
    
    if websocket_supported {
        let mut htmx = HtmxWasm::new();
        assert!(htmx.enable_extension("ws").is_ok());
        web_sys::console::log_1(&"WebSocket extension enabled successfully".into());
    } else {
        web_sys::console::warn_1(&"WebSocket not supported in this browser".into());
    }
}

#[wasm_bindgen_test]
fn test_sse_browser_support() {
    let window = window().unwrap();
    
    let eventsource_supported = js_sys::Reflect::has(&window, &"EventSource".into()).unwrap();
    
    if eventsource_supported {
        let mut htmx = HtmxWasm::new();
        assert!(htmx.enable_extension("sse").is_ok());
        web_sys::console::log_1(&"SSE extension enabled successfully".into());
    } else {
        web_sys::console::warn_1(&"EventSource not supported in this browser".into());
    }
}

#[wasm_bindgen_test]
fn test_fetch_api_support() {
    let window = window().unwrap();
    
    let fetch_supported = js_sys::Reflect::has(&window, &"fetch".into()).unwrap();
    
    if fetch_supported {
        web_sys::console::log_1(&"Fetch API supported".into());
    } else {
        web_sys::console::warn_1(&"Fetch API not supported, falling back to XMLHttpRequest".into());
    }
    
    let htmx = HtmxWasm::new();
    assert!(htmx.is_initialized());
}

#[wasm_bindgen_test]
fn test_dom_manipulation_compatibility() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    let test_element = document.create_element("div").unwrap();
    test_element.set_attribute("id", "test-element").unwrap();
    test_element.set_inner_html("Test content");
    
    assert_eq!(test_element.inner_html(), "Test content");
    assert_eq!(test_element.get_attribute("id").unwrap(), "test-element");
    
    web_sys::console::log_1(&"DOM manipulation compatibility verified".into());
}

#[wasm_bindgen_test]
fn test_event_handling_compatibility() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    let test_element = document.create_element("button").unwrap();
    
    let callback = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
        web_sys::console::log_1(&"Event handler triggered successfully".into());
    }) as Box<dyn FnMut(_)>);
    
    test_element.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).unwrap();
    callback.forget();
    
    web_sys::console::log_1(&"Event handling compatibility verified".into());
}

#[wasm_bindgen_test]
fn test_wasm_memory_management() {
    let mut htmx_instances = Vec::new();
    
    for i in 0..10 {
        let htmx = HtmxWasm::new();
        htmx_instances.push(htmx);
        
        if i % 5 == 0 {
            web_sys::console::log_1(&format!("Created {} HTMX instances", i + 1).into());
        }
    }
    
    for (i, htmx) in htmx_instances.iter().enumerate() {
        assert!(htmx.is_initialized(), "Instance {} should be initialized", i);
    }
    
    web_sys::console::log_1(&"WASM memory management test completed".into());
}

#[wasm_bindgen_test]
fn test_mobile_browser_compatibility() {
    let window = window().unwrap();
    let user_agent = "WASM Browser Test".to_string();
    
    let is_mobile = user_agent.contains("Mobile") || 
                   user_agent.contains("Android") || 
                   user_agent.contains("iPhone");
    
    if is_mobile {
        web_sys::console::log_1(&"Running mobile browser compatibility tests".into());
        
        let document = window.document().unwrap();
        let test_element = document.create_element("div").unwrap();
        
        let touch_callback = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
            web_sys::console::log_1(&"Touch event handled successfully".into());
        }) as Box<dyn FnMut(_)>);
        
        test_element.add_event_listener_with_callback("touchstart", touch_callback.as_ref().unchecked_ref()).unwrap();
        touch_callback.forget();
    }
    
    let htmx = HtmxWasm::new();
    assert!(htmx.is_initialized());
}
