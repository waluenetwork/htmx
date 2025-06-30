#!/usr/bin/env python3
"""
Simple WebSocket server for testing HTMX WASM WebSocket extension
"""
import asyncio
import websockets
import json
import logging
from datetime import datetime

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class HTMXWebSocketServer:
    def __init__(self):
        self.clients = set()
        self.message_count = 0
    
    async def register_client(self, websocket):
        self.clients.add(websocket)
        logger.info(f"Client connected. Total clients: {len(self.clients)}")
        
        welcome_msg = {
            "content": "<div class='message'>Welcome to HTMX WebSocket test server!</div>",
            "target": "#messages",
            "swap": "beforeend"
        }
        await websocket.send(json.dumps(welcome_msg))
    
    async def unregister_client(self, websocket):
        self.clients.discard(websocket)
        logger.info(f"Client disconnected. Total clients: {len(self.clients)}")
    
    async def broadcast_message(self, message, sender_websocket=None):
        if not self.clients:
            return
        
        self.message_count += 1
        timestamp = datetime.now().strftime("%H:%M:%S")
        
        htmx_response = {
            "content": f"""
                <div class='message' id='msg-{self.message_count}'>
                    <span class='timestamp'>[{timestamp}]</span>
                    <span class='content'>{message}</span>
                </div>
            """,
            "target": "#messages",
            "swap": "beforeend"
        }
        
        disconnected = set()
        for client in self.clients:
            if client != sender_websocket:
                try:
                    await client.send(json.dumps(htmx_response))
                except websockets.exceptions.ConnectionClosed:
                    disconnected.add(client)
        
        for client in disconnected:
            self.clients.discard(client)
    
    async def handle_client(self, websocket, path):
        await self.register_client(websocket)
        try:
            async for message in websocket:
                try:
                    data = json.loads(message)
                    if isinstance(data, dict) and 'message' in data:
                        await self.broadcast_message(data['message'], websocket)
                    else:
                        await self.broadcast_message(str(data), websocket)
                except json.JSONDecodeError:
                    await self.broadcast_message(message, websocket)
                    
        except websockets.exceptions.ConnectionClosed:
            pass
        finally:
            await self.unregister_client(websocket)

async def main():
    server = HTMXWebSocketServer()
    
    logger.info("Starting HTMX WebSocket test server on ws://localhost:8767")
    
    start_server = websockets.serve(server.handle_client, "localhost", 8767)
    
    await start_server
    
    await asyncio.Future()  # Run forever

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
