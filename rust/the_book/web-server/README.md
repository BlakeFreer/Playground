# Web Server

Followed the tutorial from <https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html>.

## Description

A simple multithreaded TCP server backend. Responds with a simple HTML page when accessed. This response can be delayed for 5 seconds by requesting the `/sleep` URI. This demonstrates the multithreaded behaviour because other tabs can still load while one thread is sleeping.

## Usage

Start the server with

```bash
cargo run
```

Then, in a web browser, go to `127.0.0.1:7878` or `127.0.0.1:7878/sleep`.