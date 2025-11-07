Most of this folder was taken from https://github.com/emilk/egui/tree/main/examples/hello_android
There are some edits by me.

# STEP 1
complete the prerequisits for Android Studio sdk and ndk
https://github.com/CrusaderTek/Rustaceans_Guide_to_the_Galaxy/blob/main/prerequisits/prerequisites.md

The following are required files for Cargo APK to build your android APK file. You must change the Path to where your files are on your system.

```sh
export ANDROID_HOME=/PATH/TO/YOUR/SDK/DIRECTORY
```

```sh
export NDK_HOME=/PATH/TO/YOUR/NDK/DIRECTORY
```
on my ubuntu machine these files are located at /usr/lib/android-sdk/ and /usr/lib/android-sdk/android-ndk-r25b

# STEP 2 : Hello world example for Android.
Either clone this repo with git and cd to egui_example OR download the src directory and Cargo.toml and copy into an existing cargo project.

Use `cargo-apk` to build and run. Requires a patch to workaround [an upstream bug](https://github.com/rust-mobile/cargo-subcommand/issues/29).

One-time setup: (*This must be done in each project)

```sh
cargo install \
    --git https://github.com/parasyte/cargo-apk.git \
    --rev 282639508eeed7d73f2e1eaeea042da2716436d5 \
    cargo-apk
```

Build and run:

```sh
# Run on android
cargo apk run --lib

```
