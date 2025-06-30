#!/usr/bin/env python3
"""
Find available ports for test servers
"""
import socket

def find_free_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('', 0))
        return s.getsockname()[1]

if __name__ == "__main__":
    ws_port = find_free_port()
    sse_port = find_free_port()
    http_port = find_free_port()
    
    print(f"Available ports:")
    print(f"WebSocket: {ws_port}")
    print(f"SSE: {sse_port}")
    print(f"HTTP: {http_port}")
    
    with open('available_ports.txt', 'w') as f:
        f.write(f"WS_PORT={ws_port}\n")
        f.write(f"SSE_PORT={sse_port}\n")
        f.write(f"HTTP_PORT={http_port}\n")
