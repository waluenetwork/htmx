/* tslint:disable */
/* eslint-disable */
export class ElementConfig {
  free(): void;
  constructor(method: string, url: string, trigger: string, swap: string, target: string);
  method(): string;
  url(): string;
  trigger(): string;
  swap(): string;
  target(): string;
}
export class HtmxWasm {
  free(): void;
  constructor();
  scan_dom(): void;
  is_initialized(): boolean;
  process_element(element: Element): void;
  enable_extension(name: string): void;
  is_extension_enabled(name: string): boolean;
  register_js_extension(name: string, extension: any): void;
  has_js_extension(name: string): boolean;
  parse_element_config(element: Element): ElementConfig;
  serialize_form(form: HTMLFormElement): string;
  collect_form_data(form: HTMLFormElement): FormData;
  has_pending_requests(): boolean;
  has_websocket_connection(url: string): boolean;
  has_sse_connection(url: string): boolean;
  process_websocket_message(message: string, element: Element): void;
  process_sse_event(event_data: string, element: Element): void;
  simulate_websocket_disconnect(url: string): void;
  has_pending_websocket_messages(url: string): boolean;
  call_js_extension_hook(ext_name: string, hook: string, args: any): any;
  trigger_event(element: Element, event_name: string, detail: any): void;
  find(selector: string): Element | undefined;
  find_all(selector: string): Element[];
}
export class JSExtensionBridge {
  free(): void;
  constructor();
  register_extension(name: string, extension: any): void;
  call_extension_hook(ext_name: string, hook: string, args: any): any;
  get_extension_selectors(ext_name: string): string[];
  has_extension(name: string): boolean;
  remove_extension(name: string): boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_elementconfig_free: (a: number, b: number) => void;
  readonly elementconfig_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly elementconfig_method: (a: number, b: number) => void;
  readonly elementconfig_url: (a: number, b: number) => void;
  readonly elementconfig_trigger: (a: number, b: number) => void;
  readonly elementconfig_swap: (a: number, b: number) => void;
  readonly elementconfig_target: (a: number, b: number) => void;
  readonly __wbg_jsextensionbridge_free: (a: number, b: number) => void;
  readonly jsextensionbridge_new: () => number;
  readonly jsextensionbridge_register_extension: (a: number, b: number, c: number, d: number) => void;
  readonly jsextensionbridge_call_extension_hook: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly jsextensionbridge_get_extension_selectors: (a: number, b: number, c: number, d: number) => void;
  readonly jsextensionbridge_has_extension: (a: number, b: number, c: number) => number;
  readonly jsextensionbridge_remove_extension: (a: number, b: number, c: number) => number;
  readonly __wbg_htmxwasm_free: (a: number, b: number) => void;
  readonly htmxwasm_new: () => number;
  readonly htmxwasm_scan_dom: (a: number) => void;
  readonly htmxwasm_is_initialized: (a: number) => number;
  readonly htmxwasm_process_element: (a: number, b: number, c: number) => void;
  readonly htmxwasm_enable_extension: (a: number, b: number, c: number, d: number) => void;
  readonly htmxwasm_is_extension_enabled: (a: number, b: number, c: number) => number;
  readonly htmxwasm_register_js_extension: (a: number, b: number, c: number, d: number) => void;
  readonly htmxwasm_has_js_extension: (a: number, b: number, c: number) => number;
  readonly htmxwasm_parse_element_config: (a: number, b: number, c: number) => void;
  readonly htmxwasm_serialize_form: (a: number, b: number, c: number) => void;
  readonly htmxwasm_collect_form_data: (a: number, b: number, c: number) => void;
  readonly htmxwasm_has_pending_requests: (a: number) => number;
  readonly htmxwasm_has_websocket_connection: (a: number, b: number, c: number) => number;
  readonly htmxwasm_has_sse_connection: (a: number, b: number, c: number) => number;
  readonly htmxwasm_process_websocket_message: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly htmxwasm_process_sse_event: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly htmxwasm_simulate_websocket_disconnect: (a: number, b: number, c: number, d: number) => void;
  readonly htmxwasm_has_pending_websocket_messages: (a: number, b: number, c: number) => number;
  readonly htmxwasm_call_js_extension_hook: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly htmxwasm_trigger_event: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly htmxwasm_find: (a: number, b: number, c: number) => number;
  readonly htmxwasm_find_all: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_export_0: (a: number) => void;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_2: (a: number, b: number) => number;
  readonly __wbindgen_export_3: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_5: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
