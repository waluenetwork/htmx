#!/usr/bin/env python3
"""
Fix WebSocket handler signature issue
"""
import re

def fix_websocket_handler():
    """Fix the WebSocket handler method signature"""
    
    with open('test-websocket-server-dynamic.py', 'r') as f:
        content = f.read()
    
    print("Checking websockets library version...")
    import websockets
    print(f"websockets version: {websockets.__version__}")
    
    
    old_main = '''async def main():
    port = get_port()
    server = HTMXWebSocketServer()
    
    logger.info(f"Starting HTMX WebSocket test server on ws://localhost:{port}")
    
    async def websocket_handler(websocket, path):
        await server.handle_client(websocket, path)
    
    start_server = websockets.serve(websocket_handler, "localhost", port)
    
    await start_server
    await asyncio.Future()  # Run forever'''
    
    new_main = '''async def main():
    port = get_port()
    server = HTMXWebSocketServer()
    
    logger.info(f"Starting HTMX WebSocket test server on ws://localhost:{port}")
    
    start_server = websockets.serve(
        lambda websocket, path: server.handle_client(websocket, path),
        "localhost", 
        port
    )
    
    await start_server
    await asyncio.Future()  # Run forever'''
    
    if old_main in content:
        content = content.replace(old_main, new_main)
        print("✅ Applied WebSocket handler fix")
    else:
        print("❌ Could not find exact match for main function")
        return False
    
    with open('test-websocket-server-dynamic.py', 'w') as f:
        f.write(content)
    
    print("✅ WebSocket handler fixed successfully")
    return True

if __name__ == "__main__":
    success = fix_websocket_handler()
    exit(0 if success else 1)
