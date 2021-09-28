#!/usr/bin/env zsh

cd rust-pkg

name='xlsx-to-csv'

buildAndPack () {
  echo "Building: $1"
  local target=$1
  local humanTarget=$2
  local ext=$3

  cargo build --release --target=$target

  mkdir -p ../builds/$name-$humanTarget
  cp target/$target/release/$name$ext ../builds/$name-$humanTarget

  tar -C ../builds -czvf ../builds/$name-$humanTarget.tar.gz $name-$humanTarget
}

buildAndPack "x86_64-apple-darwin" "macos-x86_64" ""
buildAndPack "aarch64-apple-darwin" "macos-aarch64" ""
buildAndPack "x86_64-pc-windows-gnu" "win64" ".exe"
buildAndPack "x86_64-unknown-linux-musl" "linux" ""
