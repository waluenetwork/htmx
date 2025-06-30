#!/usr/bin/env python3
"""
Dynamic SSE server for testing HTMX WASM SSE extension
Uses port from available_ports.txt
"""
import asyncio
import json
import logging
import sys
from datetime import datetime
from http.server import HTTPServer, BaseHTTPRequestHandler
import threading
import time

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def get_port():
    try:
        with open('available_ports.txt', 'r') as f:
            for line in f:
                if line.startswith('SSE_PORT='):
                    return int(line.split('=')[1].strip())
    except FileNotFoundError:
        return 38691  # fallback port
    return 38691

class SSEHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/events':
            self.send_response(200)
            self.send_header('Content-Type', 'text/event-stream')
            self.send_header('Cache-Control', 'no-cache')
            self.send_header('Connection', 'keep-alive')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            
            initial_data = {
                "content": "<div class='notification'>SSE connection established!</div>",
                "target": "#notifications",
                "swap": "beforeend"
            }
            self.wfile.write(f"data: {json.dumps(initial_data)}\n\n".encode())
            self.wfile.flush()
            
            for i in range(10):
                time.sleep(2)
                timestamp = datetime.now().strftime("%H:%M:%S")
                update_data = {
                    "content": f"""
                        <div class='notification' id='notif-{i+1}'>
                            <span class='timestamp'>[{timestamp}]</span>
                            <span class='content'>SSE Update #{i+1}</span>
                        </div>
                    """,
                    "target": "#notifications",
                    "swap": "beforeend"
                }
                try:
                    self.wfile.write(f"data: {json.dumps(update_data)}\n\n".encode())
                    self.wfile.flush()
                except BrokenPipeError:
                    logger.info("Client disconnected")
                    break
                    
        elif self.path == '/health':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            response = {"status": "ok", "service": "sse-server"}
            self.wfile.write(json.dumps(response).encode())
        else:
            self.send_response(404)
            self.end_headers()
    
    def log_message(self, format, *args):
        logger.info(f"SSE Server: {format % args}")

def run_server():
    port = get_port()
    server = HTTPServer(('localhost', port), SSEHandler)
    logger.info(f"Starting HTMX SSE test server on http://localhost:{port}")
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
    except Exception as e:
        logger.error(f"Server error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    run_server()
