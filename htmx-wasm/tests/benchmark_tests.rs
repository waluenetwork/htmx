use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::*;
use std::time::Instant;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
}

#[wasm_bindgen_test]
fn benchmark_wasm_vs_js_initialization() {
    let start_time = now();
    let _htmx_wasm = htmx_wasm::HtmxWasm::new();
    let wasm_init_time = now() - start_time;
    
    console::log_1(&format!("WASM initialization time: {}ms", wasm_init_time).into());
    
    assert!(wasm_init_time < 100.0);
}

#[wasm_bindgen_test]
fn benchmark_element_processing() {
    let mut htmx = htmx_wasm::HtmxWasm::new();
    
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    let start_time = now();
    
    for i in 0..1000 {
        let element = document.create_element("div").unwrap();
        element.set_attribute("hx-get", &format!("/test/{}", i)).unwrap();
        let _ = htmx.process_element(&element);
    }
    
    let processing_time = now() - start_time;
    console::log_1(&format!("WASM processing 1000 elements: {}ms", processing_time).into());
    
    assert!(processing_time < 1000.0);
}

#[wasm_bindgen_test]
fn benchmark_extension_loading() {
    let mut htmx = htmx_wasm::HtmxWasm::new();
    
    let start_time = now();
    
    #[cfg(feature = "websocket")]
    {
        let _ = htmx.enable_extension("ws");
    }
    
    #[cfg(feature = "sse")]
    {
        let _ = htmx.enable_extension("sse");
    }
    
    let extension_loading_time = now() - start_time;
    console::log_1(&format!("WASM extension loading time: {}ms", extension_loading_time).into());
    
    assert!(extension_loading_time < 50.0);
}

#[wasm_bindgen_test]
fn benchmark_dom_queries() {
    let mut htmx = htmx_wasm::HtmxWasm::new();
    
    let start_time = now();
    
    for _ in 0..1000 {
        let _ = htmx.find("body");
        let _ = htmx.find_all("*");
    }
    
    let query_time = now() - start_time;
    console::log_1(&format!("WASM DOM queries (2000 operations): {}ms", query_time).into());
    
    assert!(query_time < 500.0);
}

#[wasm_bindgen_test]
fn benchmark_event_triggering() {
    let mut htmx = htmx_wasm::HtmxWasm::new();
    
    let window = window().unwrap();
    let document = window.document().unwrap();
    let element = document.create_element("div").unwrap();
    
    let start_time = now();
    
    for i in 0..1000 {
        let detail = js_sys::Object::new();
        js_sys::Reflect::set(&detail, &"index".into(), &i.into()).unwrap();
        let _ = htmx.trigger_event(&element, "test-event", &detail.into());
    }
    
    let event_time = now() - start_time;
    console::log_1(&format!("WASM event triggering (1000 events): {}ms", event_time).into());
    
    assert!(event_time < 200.0);
}
