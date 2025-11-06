Termux is a linux terminal emulator for android. It can be used to compile and run Rust programs on your android device.

This guide is intended to be done on your android device of choice. If you haven't tried it yet android has full keyboard support. Find a bluetooth keyboard or use a dongle to connect a usb keyboard. 

# Download the latest package from f-droid (This is recommended way by the Termux website):
https://f-droid.org/en/packages/com.termux
The leatest non-beta should have a suggested icon next to its version. Click the download APK link.

*there is a termux application available in the Google Play store but it is not advised as it is buggy and still in developement.

# Download Termux x11 *optional: (fully functional x server)
https://github.com/termux/termux-x11/releases/download/nightly/app-universal-debug.apk

# Install Rust
```sh
apt install rust
```
```sh
rustup update
```
optional:
```sh
apt install helix
```
# Setup Storage
You may need to go to your settings page to allow Termux to files and media. You may have to toggle it off and on again for the permission to stick 
to setup links in your home directory to android storage run:
```sh
termux-setup-storage
```
this allows you to access your files on your device from the Termux environment. ( ~/storage/shared , ~/storage/downloads )
more info at: https://wiki.termux.com/wiki/Termux-setup-storage

# Test a rust program
```sh
cargo new mobile_test
cd mobile_test
cargo run
```
your hello world should run. 
Now your set to run any command line programs you write directly from your termux terminal.

# Testing with x11
additional info here: https://github.com/termux/termux-x11

open your termux-x11 app then switch to termux.

to test lets download kitty, a modern gpu rendered terminal
```sh
apt install kitty
```

```sh
termux-x11 :1 -xstartup "dbus-launch --exit-with-session kitty"
```
switch to your x11 app and the kitty terminal should now be running.
Replace kitty with any command that would normally spawn a window on your xserver.

#additional info
You can run xfce4 desktop environment as shown in the termux-x11 readme, but you might need to install it first.
```sh
apt install xfce4
```

you can install various distros of linux with:
```sh
apt install proot-distro
```

You can do so much more... but this should get you started.


