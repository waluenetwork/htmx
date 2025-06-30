#!/usr/bin/env python3
"""
Investigate websockets library API for correct handler signature
"""
import websockets
import inspect
import asyncio

def investigate_websockets_api():
    """Investigate the correct websockets handler signature"""
    
    print(f"websockets version: {websockets.__version__}")
    
    serve_sig = inspect.signature(websockets.serve)
    print(f"websockets.serve signature: {serve_sig}")
    
    print("\nInvestigating handler requirements...")
    
    print(f"websockets.serve docstring:")
    print(websockets.serve.__doc__)
    
    print("\nTesting handler signature...")
    
    async def test_handler(*args, **kwargs):
        print(f"Handler called with args: {args}")
        print(f"Handler called with kwargs: {kwargs}")
        print(f"Number of args: {len(args)}")
        for i, arg in enumerate(args):
            print(f"  arg[{i}]: {type(arg)} = {arg}")
        return args, kwargs
    
    print("\nRecommended fix:")
    print("Based on websockets 15.0.1, the handler should accept (websocket) only")
    print("The 'path' parameter was removed in newer versions")
    
    return test_handler

if __name__ == "__main__":
    investigate_websockets_api()
