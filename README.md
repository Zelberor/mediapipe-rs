# mediapipe-rs

Simple rust bindings for [mediapipe](https://mediapipe.dev/).

Forked from: https://github.com/angular-rust/ux-mediapipe 🙏

And heavily based on this example: https://github.com/asprecic/mediapipe-qt-integration-example 🤌

## requirements

- [bazel](https://bazel.build/install)

## setup

To run the examples clone this project.

```shell
git clone https://github.com/julesyoungberg/mediapipe-rs.git
```

Mediapipe is a framework for building AI-powered computer vision applications. It provides high level libraries exposing some of its solutions for common problems. This package makes some of these solutions available in Rust. In order to use it we must build a custom mediapipe C++ library.

Clone the modified Mediapipe repo next to `mediapipe-rs` or a project that uses `mediapipe-rs`.

```shell
git clone https://github.com/julesyoungberg/mediapipe.git
cd mediapipe
```

### building mediapipe

Build & install the mediagraph library.

#### mac os

```shell
bazel build --define MEDIAPIPE_DISABLE_GPU=1 mediapipe:libmediagraph.dylib
sudo cp bazel-bin/mediapipe/libmediagraph.dylib /usr/local/lib/libmediagraph.dylib
sudo cp mediapipe/mediagraph.h /usr/local/include/mediagraph.h
```

#### linux (untested)

```shell
bazel build --define MEDIAPIPE_DISABLE_GPU=1 mediapipe:mediagraph
sudo cp bazel-bin/mediapipe/libmediagraph.so /usr/local/lib/libmediagraph.so
sudo cp mediapipe/mediagraph.h /usr/local/include/mediagraph.h
```

### linking

Navigate to the project directory and create a symbolic link to `../mediapipe/mediapipe`.

```shell
cd ../mediapipe-rs # or the path to your project
ln -s ../mediapipe/mediapipe .
```

## usage

Add the following to your dependencies list in `Cargo.toml`:

```toml
mediapipe = { git = "https://github.com/julesyoungberg/mediapipe-rs" }
```

Mediapipe relies on tflite files which must be available at `./mediapipe/modules/`. The easiest way to satisfy this is by creating a symbolic link to mediapipe as explained in the `linking` section above.

## examples

Examples are located in the `./examples` directory. Run `face_mesh.rs` with

```shell
cargo run --release --example face_mesh
```
