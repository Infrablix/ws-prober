# WebSocket Probe Service

A lightweight HTTP service that tests WebSocket endpoint availability. The service attempts to establish WebSocket connections to provided URLs and reports their accessibility.

## Purpose

While Prometheus Blackbox Exporter provides various network probing capabilities (HTTP, TCP, ICMP, etc.), it currently lacks native WebSocket probing functionality. This service fills that gap by providing a dedicated WebSocket probe that can be integrated into your Prometheus monitoring stack via HTTP prober configuration.

## Features

- Supports both plain WebSocket (ws://) and secure WebSocket (wss://) connections
- REST API endpoint for connection testing
- JSON responses with detailed error reporting
- TLS/SSL support for secure WebSocket connections

## API Endpoint

### GET `/probe-ws`

Tests if a WebSocket endpoint is accessible.

#### Query Parameters

- `url` (required): The WebSocket URL to test (e.g., ws://example.com/socket or wss://example.com/secure-socket)

#### Response Format

```json
{
    "status": "success|failed",
    "error": "error message" // null if status is success
}