# http_file_server

A simple http file server using rust.

## Features
- Very lightweight as it uses [**tiny_http**](https://github.com/tiny-http/tiny-http) for http server

## Compiling
```
cargo build --release
```

## Reducing size
If you wanted to reduce the size further then you can do
```
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target your_target --release
```
you can compress the executable using [**UPX**](https://github.com/upx/upx)
by doing
```
upx --best --lzma target/release/http_file_server
```