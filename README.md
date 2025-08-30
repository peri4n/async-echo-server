# Async Echo Server

A simple asynchronous TCP echo server built with Rust and Tokio.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Starting the Server

To start the echo server:

```bash
cargo run
```

The server will start listening on `127.0.0.1:3000` and accept incoming connections.

### Testing with Telnet

Once the server is running, you can test it using telnet:

```bash
telnet 127.0.0.1 3000
```

- Type any message and press Enter - the server will echo it back
- Type `quit` to disconnect from the server
- The server will continue running and accept new connections

### Example Session

```
$ telnet 127.0.0.1 3000
Trying 127.0.0.1...
Connected to 127.0.0.1.
Escape character is '^]'.
Hello, world!
Hello, world!
This is a test
This is a test
quit
Connection closed by foreign host.
```

## Features

- Handles multiple concurrent connections
- Echoes back any input received
- Graceful client disconnection with "quit" command
- Built with async/await using Tokio