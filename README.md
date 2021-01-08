# HTTP/1.1 Server
Simple project to learn some Rust basics. Created by following [Learn Rust by Building Real Applications](https://www.udemy.com/course/rust-fundamentals/) course.

## Usage
```
USAGE:
    http_server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --path <default_path>        Path to public directory
    -i, --ip-address <ip_address>    Listen IP address
    -p, --port <port>                Listen port

```

In your browser visit 127.0.0.1:6969 and be amazed! xD

## Potential tasks
- [ ] Implement headers (currently being ignored)
- [ ] Incresing performance (currently single thread). [Thread](https://doc.rust-lang.org/std/thread/) and [Sync](https://doc.rust-lang.org/std/sync/index.html) docs. Entry gate to asynchronous Rust ([async](https://rust-lang.github.io/async-book/), e.g. [Tokio](https://tokio.rs/)).