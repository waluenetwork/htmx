#!/usr/bin/env python3
"""
Debug WebSocket server connection issues
"""
import asyncio
import websockets
import json
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def get_ws_port():
    try:
        with open('available_ports.txt', 'r') as f:
            for line in f:
                if line.startswith('WS_PORT='):
                    return int(line.split('=')[1].strip())
    except FileNotFoundError:
        return 48811
    return 48811

async def test_websocket_connection():
    """Test WebSocket connection with detailed debugging"""
    port = get_ws_port()
    ws_url = f"ws://localhost:{port}"
    
    logger.info(f"Testing WebSocket connection to {ws_url}")
    
    try:
        async with websockets.connect(ws_url) as websocket:
            logger.info("✅ WebSocket connection established successfully")
            
            try:
                welcome_msg = await asyncio.wait_for(websocket.recv(), timeout=5.0)
                logger.info(f"✅ Received welcome message: {welcome_msg[:100]}...")
                
                try:
                    welcome_data = json.loads(welcome_msg)
                    logger.info(f"✅ Message parsed successfully: {welcome_data}")
                    
                    test_message = {"message": "Debug test message"}
                    await websocket.send(json.dumps(test_message))
                    logger.info("✅ Test message sent successfully")
                    
                    logger.info("🎉 WebSocket connection test PASSED")
                    return True
                    
                except json.JSONDecodeError as e:
                    logger.error(f"❌ Failed to parse welcome message as JSON: {e}")
                    logger.error(f"Raw message: {welcome_msg}")
                    return False
                    
            except asyncio.TimeoutError:
                logger.error("❌ Timeout waiting for welcome message")
                return False
                
    except websockets.exceptions.ConnectionClosedError as e:
        logger.error(f"❌ WebSocket connection closed: {e}")
        return False
    except websockets.exceptions.InvalidStatusCode as e:
        logger.error(f"❌ Invalid status code: {e}")
        return False
    except Exception as e:
        logger.error(f"❌ WebSocket connection failed: {e}")
        return False

async def main():
    success = await test_websocket_connection()
    return 0 if success else 1

if __name__ == "__main__":
    import sys
    try:
        result = asyncio.run(main())
        sys.exit(result)
    except KeyboardInterrupt:
        logger.info("Test interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Test runner error: {e}")
        sys.exit(1)
