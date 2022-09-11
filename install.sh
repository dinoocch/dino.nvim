#!/usr/bin/env bash

# Based on noib3/nvim-completion install.sh
set -e

# The root of the project.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cargo_build() {
  if ! command -v cargo &>/dev/null; then
    echo "Couldn't find cargo in \$PATH, make sure the Rust toolchain is installed"
    return 1
  fi
  # Nightly is needed to compile (rustup toolchain install nightly) until https://github.com/rust-lang/rust/issues/79524 is merged.
  cargo +nightly build --release &>/dev/null
  return 0
}

copy_stuff() {
  # TODO: extension is `.so` on linux, `.dylib` on macOS and `.dll` on Windows
  library_extension=$(\
    [ -f $SCRIPT_DIR/target/release/libcompleet_client.so ] \
      && echo so \
      || echo dylib \
  )

  # Place the compiled library where Neovim can find it.
  mkdir -p $SCRIPT_DIR/lua
  cp \
    "$SCRIPT_DIR/target/release/libdino_nvim.$library_extension" \
    $SCRIPT_DIR/lua/dino.so
}

cargo_build && copy_stuff
