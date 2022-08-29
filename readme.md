## Compile to get the executed file
---
- For x86_64 windows
    1. sudo apt-get install gcc-mingw-w64-x86-64 
    2. rustup target add x86_64-pc-windows-gnu
    3. cargo build --target x86_64-pc-windows-gnu -r
- For x86_64 linux
    1. cargo build -r