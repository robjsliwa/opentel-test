# Introduction

This repo is a simple project to test OpenTelemetry SDKs across languages and transports for tracing and metrics. Since this is a test the number of servers is a lot more than it would ever be needed to generate sidekick nicknames.

There are two servers:

- SidekicknameServer - is a simple Nodejs webserver that listens to GET REST API call on /name and returns names of sidekicks.

- nameserver - is Rust based server that listens to GRPC connection for a request for the nickname and then it calls SidekicknameServer on http://localhost:8000/name.

Client - is Go based client that call name server via GRPC to get sideckick name.

Once you call using client you can open Jaeger UI on http://localhost:16686/ and you should be able to see traces and spans as shown below.
