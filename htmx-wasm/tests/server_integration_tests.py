#!/usr/bin/env python3
"""
Real server integration tests for htmx WASM implementation.
Tests WebSocket and SSE functionality with the running test server.
"""

import asyncio
import websockets
import requests
import json
import time
import subprocess
import sys
from pathlib import Path

class ServerIntegrationTester:
    def __init__(self):
        self.base_url = "http://localhost:8082"
        self.ws_url = "ws://localhost:8083/ws"
        self.test_results = []
        
    def log_test(self, test_name, success, message=""):
        result = "✅ PASS" if success else "❌ FAIL"
        print(f"{result}: {test_name}")
        if message:
            print(f"   {message}")
        self.test_results.append({
            "test": test_name,
            "success": success,
            "message": message
        })
    
    def test_http_endpoints(self):
        """Test basic HTTP endpoints"""
        try:
            response = requests.get(f"{self.base_url}/")
            self.log_test("HTTP main page", response.status_code == 200)
            
            response = requests.get(f"{self.base_url}/api/test")
            self.log_test("HTTP API endpoint", 
                         response.status_code == 200 and "message" in response.json())
            
            response = requests.options(f"{self.base_url}/api/test")
            has_cors = "Access-Control-Allow-Origin" in response.headers
            self.log_test("CORS headers present", has_cors)
            
        except Exception as e:
            self.log_test("HTTP endpoints", False, str(e))
    
    def test_sse_endpoint(self):
        """Test Server-Sent Events endpoint"""
        try:
            response = requests.get(f"{self.base_url}/events", stream=True, timeout=10)
            self.log_test("SSE connection", response.status_code == 200)
            
            content_type = response.headers.get('content-type', '')
            has_sse_headers = 'text/event-stream' in content_type
            self.log_test("SSE headers correct", has_sse_headers)
            
            lines = []
            try:
                for line in response.iter_lines(decode_unicode=True, chunk_size=1):
                    if line:
                        lines.append(line)
                        if len(lines) >= 2:  # Get at least 2 data lines
                            break
                
                has_data = any(line.startswith('data:') for line in lines)
                self.log_test("SSE data received", has_data, f"Received {len(lines)} lines")
                
            except requests.exceptions.ReadTimeout:
                has_data = any(line.startswith('data:') for line in lines)
                self.log_test("SSE data received", has_data, f"Timeout but got {len(lines)} lines")
            
        except Exception as e:
            self.log_test("SSE endpoint", False, str(e))
    
    async def test_websocket_endpoint(self):
        """Test WebSocket endpoint"""
        try:
            async with websockets.connect(self.ws_url) as websocket:
                self.log_test("WebSocket connection", True)
                
                test_message = {"type": "test", "data": "hello"}
                await websocket.send(json.dumps(test_message))
                
                response = await asyncio.wait_for(websocket.recv(), timeout=5.0)
                response_data = json.loads(response)
                
                has_response = "content" in response_data
                self.log_test("WebSocket message exchange", has_response)
                
        except Exception as e:
            self.log_test("WebSocket endpoint", False, str(e))
    
    def test_wasm_build(self):
        """Test WASM build process"""
        try:
            result = subprocess.run(['wasm-pack', '--version'], 
                                  capture_output=True, text=True)
            self.log_test("wasm-pack available", result.returncode == 0)
            
            build_result = subprocess.run(['wasm-pack', 'build', '--target', 'web'], 
                                        cwd='/home/ubuntu/htmx/htmx-wasm',
                                        capture_output=True, text=True)
            self.log_test("WASM build successful", build_result.returncode == 0)
            
            pkg_dir = Path('/home/ubuntu/htmx/htmx-wasm/pkg')
            wasm_file = pkg_dir / 'htmx_wasm_bg.wasm'  # Correct WASM filename
            js_file = pkg_dir / 'htmx_wasm.js'
            
            self.log_test("WASM file generated", wasm_file.exists())
            self.log_test("JS bindings generated", js_file.exists())
            
            if wasm_file.exists():
                wasm_size = wasm_file.stat().st_size
                size_kb = wasm_size / 1024
                self.log_test(f"WASM bundle size ({size_kb:.1f}KB)", 
                             size_kb < 50, f"Target: <50KB, Actual: {size_kb:.1f}KB")
            
        except Exception as e:
            self.log_test("WASM build process", False, str(e))
    
    def test_extension_modules(self):
        """Test modular extension builds"""
        try:
            ws_result = subprocess.run(['wasm-pack', 'build', '--target', 'web'], 
                                     cwd='/home/ubuntu/htmx/htmx-wasm/extensions/websocket-module',
                                     capture_output=True, text=True)
            self.log_test("WebSocket module build", ws_result.returncode == 0)
            
            sse_result = subprocess.run(['wasm-pack', 'build', '--target', 'web'], 
                                      cwd='/home/ubuntu/htmx/htmx-wasm/extensions/sse-module',
                                      capture_output=True, text=True)
            self.log_test("SSE module build", sse_result.returncode == 0)
            
        except Exception as e:
            self.log_test("Extension module builds", False, str(e))
    
    def run_all_tests(self):
        """Run all integration tests"""
        print("🚀 Starting htmx WASM Server Integration Tests")
        print("=" * 50)
        
        print("\n📡 Testing HTTP endpoints...")
        self.test_http_endpoints()
        
        print("\n📺 Testing Server-Sent Events...")
        self.test_sse_endpoint()
        
        print("\n🔌 Testing WebSocket...")
        asyncio.run(self.test_websocket_endpoint())
        
        print("\n🔧 Testing WASM build process...")
        self.test_wasm_build()
        
        print("\n🧩 Testing extension modules...")
        self.test_extension_modules()
        
        print("\n" + "=" * 50)
        print("📊 Test Summary")
        print("=" * 50)
        
        passed = sum(1 for result in self.test_results if result["success"])
        total = len(self.test_results)
        
        print(f"Total tests: {total}")
        print(f"Passed: {passed}")
        print(f"Failed: {total - passed}")
        print(f"Success rate: {(passed/total)*100:.1f}%")
        
        if passed == total:
            print("\n🎉 All tests passed!")
            return True
        else:
            print(f"\n⚠️  {total - passed} tests failed")
            return False

if __name__ == "__main__":
    tester = ServerIntegrationTester()
    success = tester.run_all_tests()
    sys.exit(0 if success else 1)
