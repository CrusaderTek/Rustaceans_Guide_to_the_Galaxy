Cargo_mobile2 is simple and easy to get examples compiled and ran directly on android. However, the platforms/examples it uses have a decently steep learning curve to modify and write in, if you are just starting out your programming journey.

# Complete the Prerequisites
https://github.com/CrusaderTek/Rustaceans_Guide_to_the_Galaxy/blob/main/prerequisits/prerequisites.md

# Intstall cargo_mobile2
these steps are from the readme on https://github.com/tauri-apps/cargo-mobile2
```sh
cargo install --git https://github.com/tauri-apps/cargo-mobile2
```
to update whenever
```sh
  cargo mobile update
```
# Set your ANDROID_HOME and NDK_HOME variables
linux (remember to change the path to the location of your SDK and NDK dir)
```sh
export ANDROID_HOME=/PATH/TO/YOUR/SDK/DIRECTORY
```

```sh
export NDK_HOME=/PATH/TO/YOUR/NDK/DIRECTORY
```
# Start a project
```sh
mkdir mobile_test
cd mobile_test
cargo mobile init
```
follow the prompts and choose your template project
# run your project
test on connected tablet:
```sh
cargo android run
```
run on desktop:
```sh
cargo run
```
build apk: (it should tell you the location it built to after compiling)
```sh
cargo android apk build
```
build aab:
```sh
cargo android aab build
```

