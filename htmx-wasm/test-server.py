#!/usr/bin/env python3
"""
Test server for htmx WASM integration testing
Provides WebSocket and SSE endpoints for real server testing
"""

import asyncio
import json
import time
from http.server import HTTPServer, SimpleHTTPRequestHandler
from socketserver import ThreadingMixIn
import threading
import websockets
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class CORSHTTPRequestHandler(SimpleHTTPRequestHandler):
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
        else:
            super().do_GET()
    
    def do_POST(self):
        if self.path.startswith('/api/'):
            self.handle_api_request()
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
        
        self.wfile.write(b'data: <p>SSE connection established</p>\n\n')
        self.wfile.flush()
        
        for i in range(5):
            time.sleep(1)
            event_data = f'data: <p>SSE event {i+1} at {time.strftime("%H:%M:%S")}</p>\n\n'
            try:
                self.wfile.write(event_data.encode())
                self.wfile.flush()
            except:
                break
    
    def handle_api_request(self):
        """Handle API requests"""
        content_length = int(self.headers.get('Content-Length', 0))
        post_data = self.rfile.read(content_length) if content_length > 0 else b''
        
        if self.path == '/api/test':
            response = json.dumps({'message': 'API test response', 'status': 'success'})
        elif self.path == '/api/data':
            response = json.dumps({
                'items': [
                    {'name': 'Item 1', 'value': 'Value A', 'type': 'String'},
                    {'name': 'Item 2', 'value': '42', 'type': 'Number'},
                    {'name': 'Item 3', 'value': 'true', 'type': 'Boolean'}
                ],
                'timestamp': time.strftime('%Y-%m-%d %H:%M:%S')
            })
        elif self.path == '/api/slow-data':
            time.sleep(2)  # Simulate slow response
            response = f'<p>Slow data loaded at {time.strftime("%H:%M:%S")}</p>'
        else:
            response = json.dumps({'message': 'Unknown API endpoint', 'status': 'error'})
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json' if not response.startswith('<') else 'text/html')
        self.end_headers()
        self.wfile.write(response.encode())

class ThreadingHTTPServer(ThreadingMixIn, HTTPServer):
    pass

async def websocket_handler(websocket):
    logger.info(f"WebSocket connection established from {websocket.remote_address}")
    
    try:
        welcome_msg = {
            'content': '<p>WebSocket connection established</p>',
            'target': '#ws-target'
        }
        await websocket.send(json.dumps(welcome_msg))
        
        async for message in websocket:
            logger.info(f"Received WebSocket message: {message}")
            
            try:
                data = json.loads(message)
                response = {
                    'content': f'<p>Echo: {data.get("message", "No message")} at {time.strftime("%H:%M:%S")}</p>',
                    'target': '#ws-target'
                }
                await websocket.send(json.dumps(response))
            except json.JSONDecodeError:
                response = {
                    'content': f'<p>Received: {message} at {time.strftime("%H:%M:%S")}</p>',
                    'target': '#ws-target'
                }
                await websocket.send(json.dumps(response))
                
    except websockets.exceptions.ConnectionClosed:
        logger.info("WebSocket connection closed")
    except Exception as e:
        logger.error(f"WebSocket error: {e}")

def start_websocket_server():
    """Start WebSocket server on port 8083"""
    async def run_server():
        start_server = websockets.serve(websocket_handler, "localhost", 8083)
        logger.info("WebSocket server starting on ws://localhost:8083")
        await start_server
        await asyncio.Future()  # Run forever
    
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(run_server())

def start_http_server():
    """Start HTTP server on port 8082"""
    server = ThreadingHTTPServer(('localhost', 8082), CORSHTTPRequestHandler)
    logger.info("HTTP server starting on http://localhost:8082")
    server.serve_forever()

if __name__ == '__main__':
    ws_thread = threading.Thread(target=start_websocket_server, daemon=True)
    ws_thread.start()
    
    try:
        start_http_server()
    except KeyboardInterrupt:
        logger.info("Servers shutting down...")
