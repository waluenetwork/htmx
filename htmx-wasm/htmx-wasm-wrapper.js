import init, { HtmxWasm } from './pkg/htmx_wasm.js';

class HtmxWasmWrapper {
    constructor() {
        this.wasmCore = null;
        this.jsExtensions = new Map();
        this.initialized = false;
        this.config = {
            historyEnabled: true,
            defaultSwapStyle: 'innerHTML',
            defaultSwapDelay: 0,
            defaultSettleDelay: 20
        };
    }
    
    async init() {
        await init();
        this.wasmCore = new HtmxWasm();
        this.initialized = true;
        
        this.setupHtmxAPI();
        this.setupEventListeners();
        
        console.log('HTMX WASM initialized successfully');
    }
    
    setupHtmxAPI() {
        const self = this;
        
        window.htmx = {
            defineExtension: (name, extension) => this.defineExtension(name, extension),
            process: (elt) => this.process(elt),
            find: (selector) => this.wasmCore.find(selector),
            findAll: (selector) => this.wasmCore.find_all(selector),
            trigger: (element, eventName, detail) => this.wasmCore.trigger_event(element, eventName, detail || {}),
            config: this.config,
            
            on: (eventName, handler) => {
                document.addEventListener(eventName, handler);
            },
            
            off: (eventName, handler) => {
                document.removeEventListener(eventName, handler);
            },
            
            ajax: (verb, path, options) => {
                return this.makeAjaxRequest(verb, path, options);
            },
            
            swap: (target, content, swapSpec) => {
                return this.swapContent(target, content, swapSpec);
            },
            
            version: '2.0.0-wasm'
        };
        
        this.processExistingElements();
    }
    
    setupEventListeners() {
        document.addEventListener('DOMContentLoaded', () => {
            this.processExistingElements();
        });
        
        const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
                mutation.addedNodes.forEach((node) => {
                    if (node.nodeType === Node.ELEMENT_NODE) {
                        this.process(node);
                    }
                });
            });
        });
        
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    }
    
    processExistingElements() {
        const elements = document.querySelectorAll('[hx-get], [hx-post], [hx-put], [hx-delete], [hx-patch], [ws-connect], [sse-connect]');
        elements.forEach(element => this.process(element));
    }
    
    process(element) {
        if (!this.initialized) return;
        
        try {
            this.wasmCore.process_element(element);
            this.processJSExtensions(element);
        } catch (error) {
            console.error('Error processing element:', error);
        }
    }
    
    processJSExtensions(element) {
        this.jsExtensions.forEach((extension, name) => {
            if (extension.getSelectors) {
                const selectors = extension.getSelectors();
                if (selectors.some(selector => element.matches(selector))) {
                    if (extension.onEvent) {
                        extension.onEvent('htmx:load', { detail: { elt: element } });
                    }
                }
            }
        });
    }
    
    defineExtension(name, extension) {
        if (extension.wasmNative) {
            this.wasmCore.enable_extension(name);
        } else {
            this.jsExtensions.set(name, extension);
            this.wasmCore.register_js_extension(name, extension);
            
            if (extension.init) {
                extension.init(this.createAPI());
            }
        }
        
        console.log(`Extension '${name}' registered successfully`);
    }
    
    createAPI() {
        return {
            find: (selector) => document.querySelector(selector),
            findAll: (selector) => Array.from(document.querySelectorAll(selector)),
            trigger: (element, eventName, detail) => {
                const event = new CustomEvent(eventName, { detail });
                element.dispatchEvent(event);
            },
            swap: (target, content, swapSpec) => this.swapContent(target, content, swapSpec),
            ajax: (verb, path, options) => this.makeAjaxRequest(verb, path, options),
            config: this.config
        };
    }
    
    async loadWasmExtension(name, url) {
        try {
            const module = await import(url);
            const extension = new module.default();
            extension.register_with_core(this.wasmCore);
            console.log(`WASM extension '${name}' loaded successfully`);
        } catch (error) {
            console.error(`Failed to load WASM extension '${name}':`, error);
        }
    }
    
    makeAjaxRequest(verb, path, options = {}) {
        return new Promise((resolve, reject) => {
            const xhr = new XMLHttpRequest();
            xhr.open(verb.toUpperCase(), path);
            
            xhr.setRequestHeader('HX-Request', 'true');
            if (options.headers) {
                Object.entries(options.headers).forEach(([key, value]) => {
                    xhr.setRequestHeader(key, value);
                });
            }
            
            xhr.onload = () => {
                if (xhr.status >= 200 && xhr.status < 300) {
                    resolve({
                        status: xhr.status,
                        responseText: xhr.responseText,
                        xhr: xhr
                    });
                } else {
                    reject(new Error(`HTTP ${xhr.status}: ${xhr.statusText}`));
                }
            };
            
            xhr.onerror = () => reject(new Error('Network error'));
            
            if (options.data) {
                xhr.send(options.data);
            } else {
                xhr.send();
            }
        });
    }
    
    swapContent(target, content, swapSpec = 'innerHTML') {
        switch (swapSpec) {
            case 'innerHTML':
                target.innerHTML = content;
                break;
            case 'outerHTML':
                target.outerHTML = content;
                break;
            case 'beforebegin':
                target.insertAdjacentHTML('beforebegin', content);
                break;
            case 'afterbegin':
                target.insertAdjacentHTML('afterbegin', content);
                break;
            case 'beforeend':
                target.insertAdjacentHTML('beforeend', content);
                break;
            case 'afterend':
                target.insertAdjacentHTML('afterend', content);
                break;
            default:
                target.innerHTML = content;
        }
    }
}

const htmxWasm = new HtmxWasmWrapper();
export default htmxWasm;
