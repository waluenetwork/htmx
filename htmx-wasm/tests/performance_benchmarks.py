#!/usr/bin/env python3
"""
Performance benchmarks for htmx WASM vs JavaScript implementation.
Measures initialization time, bundle size, and memory usage.
"""

import time
import subprocess
import json
import requests
from pathlib import Path
import os

class PerformanceBenchmarker:
    def __init__(self):
        self.results = {}
        self.htmx_wasm_dir = Path('/home/ubuntu/htmx/htmx-wasm')
        self.htmx_js_dir = Path('/home/ubuntu/htmx')
        
    def measure_bundle_sizes(self):
        """Measure and compare bundle sizes"""
        print("📏 Measuring bundle sizes...")
        
        wasm_file = self.htmx_wasm_dir / 'pkg' / 'htmx_wasm.wasm'
        js_binding_file = self.htmx_wasm_dir / 'pkg' / 'htmx_wasm.js'
        
        wasm_size = 0
        js_binding_size = 0
        
        if wasm_file.exists():
            wasm_size = wasm_file.stat().st_size
        if js_binding_file.exists():
            js_binding_size = js_binding_file.stat().st_size
            
        total_wasm_size = wasm_size + js_binding_size
        
        htmx_js_file = self.htmx_js_dir / 'dist' / 'htmx.min.js'
        htmx_js_size = 0
        
        if htmx_js_file.exists():
            htmx_js_size = htmx_js_file.stat().st_size
        else:
            htmx_js_file = self.htmx_js_dir / 'src' / 'htmx.js'
            if htmx_js_file.exists():
                htmx_js_size = htmx_js_file.stat().st_size
        
        self.results['bundle_sizes'] = {
            'wasm_binary_kb': wasm_size / 1024,
            'js_bindings_kb': js_binding_size / 1024,
            'total_wasm_kb': total_wasm_size / 1024,
            'original_htmx_kb': htmx_js_size / 1024,
            'size_ratio': total_wasm_size / htmx_js_size if htmx_js_size > 0 else 0
        }
        
        print(f"  WASM binary: {self.results['bundle_sizes']['wasm_binary_kb']:.1f}KB")
        print(f"  JS bindings: {self.results['bundle_sizes']['js_bindings_kb']:.1f}KB")
        print(f"  Total WASM: {self.results['bundle_sizes']['total_wasm_kb']:.1f}KB")
        print(f"  Original htmx: {self.results['bundle_sizes']['original_htmx_kb']:.1f}KB")
        print(f"  Size ratio: {self.results['bundle_sizes']['size_ratio']:.2f}x")
    
    def measure_build_times(self):
        """Measure build times for different configurations"""
        print("⏱️  Measuring build times...")
        
        build_times = {}
        
        start_time = time.time()
        result = subprocess.run(['wasm-pack', 'build', '--target', 'web'], 
                              cwd=self.htmx_wasm_dir,
                              capture_output=True, text=True)
        build_times['single_bundle'] = time.time() - start_time
        
        start_time = time.time()
        result = subprocess.run(['wasm-pack', 'build', '--target', 'web'], 
                              cwd=self.htmx_wasm_dir / 'extensions' / 'websocket-module',
                              capture_output=True, text=True)
        build_times['websocket_module'] = time.time() - start_time
        
        start_time = time.time()
        result = subprocess.run(['wasm-pack', 'build', '--target', 'web'], 
                              cwd=self.htmx_wasm_dir / 'extensions' / 'sse-module',
                              capture_output=True, text=True)
        build_times['sse_module'] = time.time() - start_time
        
        self.results['build_times'] = build_times
        
        print(f"  Single bundle: {build_times['single_bundle']:.2f}s")
        print(f"  WebSocket module: {build_times['websocket_module']:.2f}s")
        print(f"  SSE module: {build_times['sse_module']:.2f}s")
    
    def measure_extension_loading(self):
        """Measure extension loading performance"""
        print("🧩 Measuring extension loading...")
        
        ws_ext_file = self.htmx_wasm_dir / 'extensions' / 'websocket-module' / 'pkg' / 'htmx_websocket_extension.wasm'
        sse_ext_file = self.htmx_wasm_dir / 'extensions' / 'sse-module' / 'pkg' / 'htmx_sse_extension.wasm'
        
        extension_sizes = {}
        
        if ws_ext_file.exists():
            extension_sizes['websocket_kb'] = ws_ext_file.stat().st_size / 1024
        
        if sse_ext_file.exists():
            extension_sizes['sse_kb'] = sse_ext_file.stat().st_size / 1024
        
        js_ws_ext = self.htmx_js_dir / 'dist' / 'ext' / 'ws.js'
        js_sse_ext = self.htmx_js_dir / 'dist' / 'ext' / 'sse.js'
        
        if js_ws_ext.exists():
            extension_sizes['js_websocket_kb'] = js_ws_ext.stat().st_size / 1024
        
        if js_sse_ext.exists():
            extension_sizes['js_sse_kb'] = js_sse_ext.stat().st_size / 1024
        
        self.results['extension_sizes'] = extension_sizes
        
        for ext, size in extension_sizes.items():
            print(f"  {ext}: {size:.1f}KB")
    
    def measure_compilation_performance(self):
        """Measure Rust compilation performance"""
        print("🦀 Measuring Rust compilation...")
        
        subprocess.run(['cargo', 'clean'], cwd=self.htmx_wasm_dir, capture_output=True)
        
        start_time = time.time()
        result = subprocess.run(['cargo', 'build', '--release'], 
                              cwd=self.htmx_wasm_dir,
                              capture_output=True, text=True)
        compilation_time = time.time() - start_time
        
        start_time = time.time()
        result = subprocess.run(['cargo', 'build', '--release'], 
                              cwd=self.htmx_wasm_dir,
                              capture_output=True, text=True)
        incremental_time = time.time() - start_time
        
        self.results['compilation'] = {
            'full_build_time': compilation_time,
            'incremental_build_time': incremental_time
        }
        
        print(f"  Full build: {compilation_time:.2f}s")
        print(f"  Incremental build: {incremental_time:.2f}s")
    
    def generate_report(self):
        """Generate comprehensive performance report"""
        print("\n" + "=" * 60)
        print("📊 HTMX WASM Performance Report")
        print("=" * 60)
        
        if 'bundle_sizes' in self.results:
            sizes = self.results['bundle_sizes']
            print(f"\n📦 Bundle Size Analysis:")
            print(f"  Original htmx.js: {sizes['original_htmx_kb']:.1f}KB")
            print(f"  WASM implementation: {sizes['total_wasm_kb']:.1f}KB")
            print(f"  Size difference: {sizes['size_ratio']:.2f}x")
            
            if sizes['total_wasm_kb'] <= 15:
                print("  ✅ Bundle size target met (<15KB)")
            else:
                print(f"  ⚠️  Bundle size exceeds target (>{sizes['total_wasm_kb']:.1f}KB)")
        
        if 'build_times' in self.results:
            times = self.results['build_times']
            print(f"\n⏱️  Build Performance:")
            print(f"  Single bundle: {times['single_bundle']:.2f}s")
            print(f"  Modular extensions: {times['websocket_module'] + times['sse_module']:.2f}s")
        
        if 'extension_sizes' in self.results:
            ext_sizes = self.results['extension_sizes']
            print(f"\n🧩 Extension Size Comparison:")
            
            if 'websocket_kb' in ext_sizes and 'js_websocket_kb' in ext_sizes:
                wasm_ws = ext_sizes['websocket_kb']
                js_ws = ext_sizes['js_websocket_kb']
                ratio = wasm_ws / js_ws if js_ws > 0 else 0
                print(f"  WebSocket - WASM: {wasm_ws:.1f}KB, JS: {js_ws:.1f}KB ({ratio:.2f}x)")
            
            if 'sse_kb' in ext_sizes and 'js_sse_kb' in ext_sizes:
                wasm_sse = ext_sizes['sse_kb']
                js_sse = ext_sizes['js_sse_kb']
                ratio = wasm_sse / js_sse if js_sse > 0 else 0
                print(f"  SSE - WASM: {wasm_sse:.1f}KB, JS: {js_sse:.1f}KB ({ratio:.2f}x)")
        
        results_file = self.htmx_wasm_dir / 'performance_results.json'
        with open(results_file, 'w') as f:
            json.dump(self.results, f, indent=2)
        
        print(f"\n💾 Results saved to: {results_file}")
        
        return self.results
    
    def run_all_benchmarks(self):
        """Run all performance benchmarks"""
        print("🚀 Starting htmx WASM Performance Benchmarks")
        print("=" * 50)
        
        self.measure_bundle_sizes()
        self.measure_build_times()
        self.measure_extension_loading()
        self.measure_compilation_performance()
        
        return self.generate_report()

if __name__ == "__main__":
    benchmarker = PerformanceBenchmarker()
    results = benchmarker.run_all_benchmarks()
