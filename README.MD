# Upward⬆

Upward⬆ is my personal utility for updating my Desktop programatically. If you find it useful, feel free to put an issue for any ideas. it fetches all images from Unsplash.

## Installing
```sh
cargo install --git https://github.com/tristanisham/upward.git
```
So far, this program has been only tested on Windows. It should mostly be cross-platform though (with libOpenSSL (Rust, write a good native TLS/SSL lib already) being a known dependency on Linux).

## Using
```sh
        # defaults to 1920  # Defaults to 1080
upward -x <X dimension> -y <Y Dimension> -s <Search terms in string deliminated by spaces>
```
