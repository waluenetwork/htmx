#!/usr/bin/env python3
"""
Simple SSE server for testing HTMX WASM SSE extension
"""
import asyncio
import json
import time
from datetime import datetime
from aiohttp import web, web_response
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class HTMXSSEServer:
    def __init__(self):
        self.clients = set()
        self.event_count = 0
    
    async def handle_sse_connection(self, request):
        response = web_response.StreamResponse(
            status=200,
            reason='OK',
            headers={
                'Content-Type': 'text/event-stream',
                'Cache-Control': 'no-cache',
                'Connection': 'keep-alive',
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Headers': 'Cache-Control'
            }
        )
        
        await response.prepare(request)
        
        self.clients.add(response)
        logger.info(f"SSE client connected. Total clients: {len(self.clients)}")
        
        try:
            welcome_data = {
                "content": "<div class='sse-message'>Connected to SSE test server!</div>",
                "target": "#sse-messages",
                "swap": "beforeend"
            }
            
            await response.write(f"event: message\n".encode())
            await response.write(f"data: {json.dumps(welcome_data)}\n\n".encode())
            
            while True:
                await asyncio.sleep(5)  # Send event every 5 seconds
                
                if response in self.clients:
                    self.event_count += 1
                    timestamp = datetime.now().strftime("%H:%M:%S")
                    
                    event_data = {
                        "content": f"""
                            <div class='sse-event' id='event-{self.event_count}'>
                                <span class='timestamp'>[{timestamp}]</span>
                                <span class='content'>SSE Event #{self.event_count}</span>
                            </div>
                        """,
                        "target": "#sse-messages",
                        "swap": "beforeend"
                    }
                    
                    try:
                        await response.write(f"event: message\n".encode())
                        await response.write(f"data: {json.dumps(event_data)}\n\n".encode())
                    except Exception as e:
                        logger.error(f"Error sending SSE event: {e}")
                        break
                else:
                    break
                    
        except asyncio.CancelledError:
            pass
        except Exception as e:
            logger.error(f"SSE connection error: {e}")
        finally:
            self.clients.discard(response)
            logger.info(f"SSE client disconnected. Total clients: {len(self.clients)}")
        
        return response
    
    async def handle_trigger_event(self, request):
        """Endpoint to trigger custom events"""
        data = await request.json()
        event_type = data.get('event', 'custom')
        message = data.get('message', 'Custom event triggered')
        
        self.event_count += 1
        timestamp = datetime.now().strftime("%H:%M:%S")
        
        event_data = {
            "content": f"""
                <div class='custom-event' id='custom-{self.event_count}'>
                    <span class='timestamp'>[{timestamp}]</span>
                    <span class='event-type'>[{event_type}]</span>
                    <span class='content'>{message}</span>
                </div>
            """,
            "target": "#sse-messages",
            "swap": "beforeend"
        }
        
        disconnected = set()
        for client in self.clients.copy():
            try:
                await client.write(f"event: {event_type}\n".encode())
                await client.write(f"data: {json.dumps(event_data)}\n\n".encode())
            except Exception as e:
                logger.error(f"Error sending custom event: {e}")
                disconnected.add(client)
        
        for client in disconnected:
            self.clients.discard(client)
        
        return web.json_response({"status": "sent", "clients": len(self.clients)})
    
    async def handle_cors_preflight(self, request):
        """Handle CORS preflight requests"""
        return web.Response(
            headers={
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Methods': 'GET, POST, OPTIONS',
                'Access-Control-Allow-Headers': 'Content-Type',
            }
        )

def create_app():
    server = HTMXSSEServer()
    app = web.Application()
    
    app.router.add_get('/events', server.handle_sse_connection)
    
    app.router.add_post('/trigger', server.handle_trigger_event)
    app.router.add_options('/trigger', server.handle_cors_preflight)
    
    app.router.add_static('/', path='.', name='static')
    
    return app

async def main():
    app = create_app()
    
    logger.info("Starting HTMX SSE test server on http://localhost:8766")
    
    runner = web.AppRunner(app)
    await runner.setup()
    
    site = web.TCPSite(runner, 'localhost', 8766)
    await site.start()
    
    await asyncio.Future()  # Run forever

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
