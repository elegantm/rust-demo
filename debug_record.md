
failed to run custom build command for `openssl-sys v0.9.55`

https://github.com/sfackler/rust-openssl/issues/1021
ubuntu 解决办法：sudo apt install pkg-config

未尝试解决办法：
---------------------------
Cargo.toml

[dependencies]
openssl = { version = "0.10", features = ["vendored"] }
---------------------------