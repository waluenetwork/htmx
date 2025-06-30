#!/usr/bin/env python3
"""
Production server for htmx WASM examples
Serves static files and provides WebSocket/SSE endpoints
Usage: python3 production-server.py
Then open: http://localhost:8080/single-bundle-fixed.html
"""

import asyncio
import json
import time
import os
from http.server import HTTPServer, SimpleHTTPRequestHandler
from socketserver import ThreadingMixIn
import threading
import websockets
import logging
from urllib.parse import urlparse, parse_qs

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class ProductionHTTPRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.getcwd(), **kwargs)
    
    def guess_type(self, path):
        """Override to add proper MIME type for WASM files"""
        result = super().guess_type(path)
        if str(path).endswith('.wasm'):
            return 'application/wasm'
        return result
    
    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type, Authorization, HX-Request, HX-Target, HX-Current-URL')
        super().end_headers()
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()
    
    def do_GET(self):
        if self.path == '/events':
            self.handle_sse()
        elif self.path.startswith('/api/'):
            self.handle_api_request()
        elif self.path == '/trigger-event':
            self.handle_trigger_event()
        else:
            super().do_GET()
    
    def do_POST(self):
        if self.path.startswith('/api/'):
            self.handle_api_request()
        elif self.path == '/trigger-event':
            self.handle_trigger_event()
        else:
            self.send_response(404)
            self.end_headers()
    
    def handle_sse(self):
        """Handle Server-Sent Events"""
        self.send_response(200)
        self.send_header('Content-Type', 'text/event-stream')
        self.send_header('Cache-Control', 'no-cache')
        self.send_header('Connection', 'keep-alive')
        self.end_headers()
        
        self.wfile.write('data: <div class="notification">✅ SSE connection established</div>\n\n'.encode('utf-8'))
        self.wfile.flush()
        
        for i in range(10):
            time.sleep(2)
            timestamp = time.strftime("%H:%M:%S")
            event_data = f'data: <div class="notification">📡 SSE update {i+1} at {timestamp}</div>\n\n'
            try:
                self.wfile.write(event_data.encode('utf-8'))
                self.wfile.flush()
            except:
                break
    
    def handle_trigger_event(self):
        """Handle manual event trigger"""
        timestamp = time.strftime("%H:%M:%S")
        response = f'<div class="notification">🎯 Manual event triggered at {timestamp}</div>'
        
        self.send_response(200)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(response.encode('utf-8'))
    
    def handle_api_request(self):
        """Handle API requests"""
        content_length = int(self.headers.get('Content-Length', 0))
        post_data = self.rfile.read(content_length) if content_length > 0 else b''
        
        if self.path == '/api/data':
            response = '''
            <div>
                <h3>✅ API Data Loaded Successfully</h3>
                <ul>
                    <li><strong>Item 1:</strong> Sample data from WASM htmx</li>
                    <li><strong>Item 2:</strong> WebSocket and SSE extensions active</li>
                    <li><strong>Item 3:</strong> Performance optimized Rust implementation</li>
                </ul>
                <p><em>Loaded at {}</em></p>
            </div>
            '''.format(time.strftime('%Y-%m-%d %H:%M:%S'))
        elif self.path == '/api/test':
            response = '<p>✅ API test successful - HTMX WASM is working!</p>'
        elif self.path == '/api/slow-data':
            time.sleep(1)
            response = f'<div><h4>🐌 Slow Data Response</h4><p>Data loaded at {time.strftime("%H:%M:%S")}</p><p>This request took 1 second to process.</p></div>'
        else:
            response = '<p>❌ Unknown API endpoint</p>'
        
        self.send_response(200)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(response.encode('utf-8'))

class ThreadingHTTPServer(ThreadingMixIn, HTTPServer):
    pass

async def websocket_handler(websocket):
    logger.info(f"WebSocket connection established from {websocket.remote_address}")
    
    try:
        welcome_msg = '<div class="notification">🔌 WebSocket connected successfully!</div>'
        await websocket.send(welcome_msg)
        
        async for message in websocket:
            logger.info(f"Received WebSocket message: {message}")
            
            try:
                if 'message=' in message:
                    user_message = message.split('message=')[1].split('&')[0]
                    import urllib.parse
                    user_message = urllib.parse.unquote_plus(user_message)
                else:
                    user_message = message
                
                timestamp = time.strftime("%H:%M:%S")
                response = f'<div class="notification">📨 Echo: "{user_message}" at {timestamp}</div>'
                await websocket.send(response)
                
            except Exception as e:
                logger.error(f"Error processing message: {e}")
                error_response = f'<div class="notification">❌ Error: {str(e)}</div>'
                await websocket.send(error_response)
                
    except websockets.exceptions.ConnectionClosed:
        logger.info("WebSocket connection closed")
    except Exception as e:
        logger.error(f"WebSocket error: {e}")

def start_websocket_server():
    """Start WebSocket server on port 8081"""
    async def run_server():
        start_server = websockets.serve(websocket_handler, "localhost", 8081)
        logger.info("🔌 WebSocket server starting on ws://localhost:8081")
        await start_server
        await asyncio.Future()
    
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(run_server())

def start_http_server():
    """Start HTTP server on port 8080"""
    server = ThreadingHTTPServer(('localhost', 8080), ProductionHTTPRequestHandler)
    logger.info("🌐 HTTP server starting on http://localhost:8080")
    logger.info("📁 Serving files from current directory")
    logger.info("🔗 Open http://localhost:8080/single-bundle-fixed.html to test")
    server.serve_forever()

if __name__ == '__main__':
    print("🚀 Starting HTMX WASM Production Server")
    print("=" * 60)
    print("📍 HTTP Server: http://localhost:8080")
    print("📍 WebSocket Server: ws://localhost:8081")
    print("📍 SSE Endpoint: http://localhost:8080/events")
    print("📍 API Endpoints: http://localhost:8080/api/*")
    print("=" * 60)
    print("🔗 Test URLs:")
    print("   Single Bundle: http://localhost:8080/single-bundle-fixed.html")
    print("   Minimal Bundle: http://localhost:8080/minimal-bundle-fixed.html")
    print("=" * 60)
    
    ws_thread = threading.Thread(target=start_websocket_server, daemon=True)
    ws_thread.start()
    
    try:
        start_http_server()
    except KeyboardInterrupt:
        logger.info("🛑 Servers shutting down...")
