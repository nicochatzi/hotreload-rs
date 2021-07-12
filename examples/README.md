# Hot reloading example

This example has a lib containing a few functions used in an app. With every change in the
library the app should reload the functions it uses in the lib on the fly.

## Usage

This requires [cargo watch](https://github.com/watchexec/cargo-watch).

In one shell run:
```sh
cd examples/app
cargo watch -c -x run
```

In another shell run:
```sh
cd examples/lib
cargo watch -c -x build
```

Every time files in `lib/` are changed the app should hotreload the methods it uses from lib.
Every time files in `app/` are changed the app is recompiled and rerun.