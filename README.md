# ux-mediapipe

Rust and mediapipe.

Mediapipe is a framework for building AI-powered computer vision and augmented reality applications. It provides high level libraries exposing some of its solutions for common problems. This package makes some of these solutions available in Rust. In order to use it we must build a C++ library that provides an interface to the mediapipe 'engine'.

Figuring all this out has been a challenge. I have made these forks and instructions to help me in the future.

## requirements

- bazel
- clang

## setup

Clone the modified Mediapipe repo:

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

## usage

Add the following to your dependencies list in `Cargo.toml`:

```toml
ux-mediapipe = { git = "https://github.com/julesyoungberg/ux-mediapipe" }
```

## binding generation

The binding generation should be automated. Currently it is a combination of manual commands and manual file editing...not ideal.
