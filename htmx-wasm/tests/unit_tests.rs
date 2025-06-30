use htmx_wasm::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htmx_core_creation() {
        let core = htmx_wasm::HtmxCore::new();
        assert!(!core.has_pending_requests());
    }

    #[test]
    fn test_extension_registry_creation() {
        let registry = htmx_wasm::ExtensionRegistry::new();
        assert!(!registry.has_websocket_connection("test-url"));
        assert!(!registry.has_sse_connection("test-url"));
    }

    #[test]
    fn test_websocket_extension_creation() {
        let ws_ext = htmx_wasm::WebSocketExtension::new();
        assert_eq!(ws_ext.name(), "ws");
        assert!(ws_ext.selectors().contains(&"[ws-connect]"));
    }

    #[test]
    fn test_sse_extension_creation() {
        let sse_ext = htmx_wasm::SSEExtension::new();
        assert_eq!(sse_ext.name(), "sse");
        assert!(sse_ext.selectors().contains(&"[sse-connect]"));
    }

    #[test]
    fn test_js_extension_bridge_creation() {
        let _bridge = htmx_wasm::JSExtensionBridge::new();
    }

    #[test]
    fn test_element_config_creation() {
        let config = htmx_wasm::ElementConfig::new(
            "GET".to_string(),
            "/test".to_string(),
            "click".to_string(),
            "innerHTML".to_string(),
            "#target".to_string(),
        );
        
        assert_eq!(config.method(), "GET");
        assert_eq!(config.url(), "/test");
        assert_eq!(config.trigger(), "click");
        assert_eq!(config.swap(), "innerHTML");
        assert_eq!(config.target(), "#target");
    }

    #[test]
    fn test_htmx_config_defaults() {
        let config = htmx_wasm::HtmxConfig::default();
        assert!(config.history_enabled);
        assert_eq!(config.default_swap_style, "innerHTML");
        assert_eq!(config.default_swap_delay, 0);
        assert_eq!(config.default_settle_delay, 20);
    }
}
