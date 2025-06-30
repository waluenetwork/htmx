#!/usr/bin/env python3
"""
Real server integration tests for HTMX WASM extensions
Tests WebSocket and SSE extensions with actual running servers
"""
import asyncio
import websockets
import json
import requests
import time
import logging
from datetime import datetime

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def get_ports():
    """Read ports from available_ports.txt"""
    ports = {}
    try:
        with open('available_ports.txt', 'r') as f:
            for line in f:
                if '=' in line:
                    key, value = line.strip().split('=', 1)
                    ports[key] = int(value)
    except FileNotFoundError:
        ports = {'WS_PORT': 48811, 'SSE_PORT': 38691, 'HTTP_PORT': 45947}
    return ports

class RealServerIntegrationTests:
    def __init__(self):
        self.ports = get_ports()
        self.ws_url = f"ws://localhost:{self.ports['WS_PORT']}"
        self.sse_url = f"http://localhost:{self.ports['SSE_PORT']}"
        self.test_results = []
    
    async def test_websocket_connection(self):
        """Test WebSocket connection and message exchange"""
        logger.info("Testing WebSocket connection...")
        
        try:
            async with websockets.connect(self.ws_url) as websocket:
                welcome_msg = await asyncio.wait_for(websocket.recv(), timeout=5.0)
                welcome_data = json.loads(welcome_msg)
                
                assert 'content' in welcome_data
                assert 'target' in welcome_data
                assert welcome_data['target'] == '#messages'
                
                test_message = {"message": "Integration test message"}
                await websocket.send(json.dumps(test_message))
                
                
                self.test_results.append({
                    'test': 'websocket_connection',
                    'status': 'PASS',
                    'details': 'WebSocket connection established and welcome message received'
                })
                logger.info("✅ WebSocket connection test PASSED")
                
        except Exception as e:
            self.test_results.append({
                'test': 'websocket_connection',
                'status': 'FAIL',
                'error': str(e)
            })
            logger.error(f"❌ WebSocket connection test FAILED: {e}")
    
    def test_sse_connection(self):
        """Test SSE connection and event stream"""
        logger.info("Testing SSE connection...")
        
        try:
            response = requests.get(f"{self.sse_url}/events", stream=True, timeout=10)
            
            if response.status_code != 200:
                raise Exception(f"SSE endpoint returned status {response.status_code}")
            
            events_received = 0
            for line in response.iter_lines(decode_unicode=True):
                if line.startswith('data: '):
                    event_data = line[6:]  # Remove 'data: ' prefix
                    try:
                        parsed_data = json.loads(event_data)
                        assert 'content' in parsed_data
                        assert 'target' in parsed_data
                        events_received += 1
                        
                        if events_received >= 2:  # Test first 2 events
                            break
                    except json.JSONDecodeError:
                        continue
            
            if events_received >= 2:
                self.test_results.append({
                    'test': 'sse_connection',
                    'status': 'PASS',
                    'details': f'SSE connection established, received {events_received} events'
                })
                logger.info("✅ SSE connection test PASSED")
            else:
                raise Exception(f"Only received {events_received} events, expected at least 2")
                
        except Exception as e:
            self.test_results.append({
                'test': 'sse_connection',
                'status': 'FAIL',
                'error': str(e)
            })
            logger.error(f"❌ SSE connection test FAILED: {e}")
    
    def test_sse_health_endpoint(self):
        """Test SSE server health endpoint"""
        logger.info("Testing SSE health endpoint...")
        
        try:
            response = requests.get(f"{self.sse_url}/health", timeout=5)
            
            if response.status_code == 200:
                health_data = response.json()
                assert health_data['status'] == 'ok'
                assert health_data['service'] == 'sse-server'
                
                self.test_results.append({
                    'test': 'sse_health',
                    'status': 'PASS',
                    'details': 'SSE health endpoint responding correctly'
                })
                logger.info("✅ SSE health test PASSED")
            else:
                raise Exception(f"Health endpoint returned status {response.status_code}")
                
        except Exception as e:
            self.test_results.append({
                'test': 'sse_health',
                'status': 'FAIL',
                'error': str(e)
            })
            logger.error(f"❌ SSE health test FAILED: {e}")
    
    async def test_websocket_message_format(self):
        """Test WebSocket message format compliance with HTMX protocol"""
        logger.info("Testing WebSocket message format...")
        
        try:
            async with websockets.connect(self.ws_url) as websocket:
                welcome_msg = await asyncio.wait_for(websocket.recv(), timeout=5.0)
                welcome_data = json.loads(welcome_msg)
                
                required_fields = ['content', 'target', 'swap']
                for field in required_fields:
                    if field not in welcome_data:
                        if field == 'swap':
                            continue  # swap is optional
                        raise Exception(f"Missing required field: {field}")
                
                content = welcome_data['content']
                if not content.strip().startswith('<'):
                    raise Exception("Content is not HTML format")
                
                self.test_results.append({
                    'test': 'websocket_message_format',
                    'status': 'PASS',
                    'details': 'WebSocket messages follow HTMX protocol format'
                })
                logger.info("✅ WebSocket message format test PASSED")
                
        except Exception as e:
            self.test_results.append({
                'test': 'websocket_message_format',
                'status': 'FAIL',
                'error': str(e)
            })
            logger.error(f"❌ WebSocket message format test FAILED: {e}")
    
    def test_sse_message_format(self):
        """Test SSE message format compliance with HTMX protocol"""
        logger.info("Testing SSE message format...")
        
        try:
            response = requests.get(f"{self.sse_url}/events", stream=True, timeout=10)
            
            for line in response.iter_lines(decode_unicode=True):
                if line.startswith('data: '):
                    event_data = line[6:]
                    parsed_data = json.loads(event_data)
                    
                    required_fields = ['content', 'target']
                    for field in required_fields:
                        if field not in parsed_data:
                            raise Exception(f"Missing required field: {field}")
                    
                    content = parsed_data['content']
                    if not content.strip().startswith('<'):
                        raise Exception("Content is not HTML format")
                    
                    self.test_results.append({
                        'test': 'sse_message_format',
                        'status': 'PASS',
                        'details': 'SSE messages follow HTMX protocol format'
                    })
                    logger.info("✅ SSE message format test PASSED")
                    break
                    
        except Exception as e:
            self.test_results.append({
                'test': 'sse_message_format',
                'status': 'FAIL',
                'error': str(e)
            })
            logger.error(f"❌ SSE message format test FAILED: {e}")
    
    async def run_all_tests(self):
        """Run all integration tests"""
        logger.info("Starting real server integration tests...")
        
        await self.test_websocket_connection()
        await self.test_websocket_message_format()
        
        self.test_sse_health_endpoint()
        self.test_sse_connection()
        self.test_sse_message_format()
        
        total_tests = len(self.test_results)
        passed_tests = len([t for t in self.test_results if t['status'] == 'PASS'])
        failed_tests = total_tests - passed_tests
        
        logger.info(f"\n=== INTEGRATION TEST SUMMARY ===")
        logger.info(f"Total tests: {total_tests}")
        logger.info(f"Passed: {passed_tests}")
        logger.info(f"Failed: {failed_tests}")
        logger.info(f"Success rate: {(passed_tests/total_tests)*100:.1f}%")
        
        results = {
            'timestamp': datetime.now().isoformat(),
            'summary': {
                'total': total_tests,
                'passed': passed_tests,
                'failed': failed_tests,
                'success_rate': (passed_tests/total_tests)*100
            },
            'tests': self.test_results,
            'server_ports': self.ports
        }
        
        with open('real_server_integration_results.json', 'w') as f:
            json.dump(results, f, indent=2)
        
        logger.info("Results saved to real_server_integration_results.json")
        
        return passed_tests == total_tests

async def main():
    """Main test runner"""
    tester = RealServerIntegrationTests()
    success = await tester.run_all_tests()
    
    if success:
        logger.info("🎉 All integration tests PASSED!")
        return 0
    else:
        logger.error("💥 Some integration tests FAILED!")
        return 1

if __name__ == "__main__":
    import sys
    try:
        result = asyncio.run(main())
        sys.exit(result)
    except KeyboardInterrupt:
        logger.info("Tests interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Test runner error: {e}")
        sys.exit(1)
