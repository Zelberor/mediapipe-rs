# mediapipe-rs

Simple rust bindings for [mediapipe](https://mediapipe.dev/).

Forked from: https://github.com/angular-rust/ux-mediapipe 🙏

And heavily based on this example: https://github.com/asprecic/mediapipe-qt-integration-example 🤌

## requirements

- [bazel](https://bazel.build/install)

## setup

Mediapipe is a framework for building AI-powered computer vision applications. It provides high level libraries exposing some of its solutions for common problems. This package makes some of these solutions available in Rust. In order to use it we must build a custom mediapipe C++ library.

Clone the modified Mediapipe repo.

```shell
git clone https://github.com/julesyoungberg/mediapipe.git
cd mediapipe
```

Build & install the mediagraph library.

### mac os

```shell
bazel build --define MEDIAPIPE_DISABLE_GPU=1 mediapipe:libmediagraph.dylib
sudo cp bazel-bin/mediapipe/libmediagraph.dylib /usr/local/lib/libmediagraph.dylib
cp mediapipe/mediagraph.h /usr/local/include/mediagraph.h
```

### linux (untested)

```shell
bazel build --define MEDIAPIPE_DISABLE_GPU=1 mediapipe:mediagraph
cp bazel-bin/mediapipe/libmediagraph.so /usr/local/lib/libmediagraph.so
cp mediapipe/mediagraph.h /usr/local/include/mediagraph.h
```

## examples

Examples are located in the `./examples` directory. Run `face_mesh.rs` with

```shell
cargo run --release --example face_mesh
```

## usage

Add the following to your dependencies list in `Cargo.toml`:

```toml
mediapipe = { git = "https://github.com/julesyoungberg/mediapipe-rs" }
```

Mediapipe relies on tflite files which must be available at `./mediapipe/modules/`. The easiest way to do this is

```shell
mkdir mediapipe
cp -R ../mediapipe/mediapipe/modules ./mediapipe/modules
```

The path to mediapipe may be different depending on where you have cloned it to.
