**Most of this assumes you are developing on an debian based linux distro(...probably ubuntu). (I may get around to writing windows commands later... just use linux its much easier)

//TODO
INSTALL RUST
CARGO android build tools (cargo_apk, cargo_mobie2, xbuild, cargo-ndk)
Optional:
INSTALL HELIX
//TODO

# Enable Developer Options on your Android device
This will enable you to run apps directly on your android device while testing with "Cargo apk run --lib" (or "Cargo Android run" if using cargo_mobile2"

Settings > System > Developer options > USB debugging

on your android device go to your settings (swipe down from the top twice and select the gear icon.)
type USB Degugging into your search bar, select it, and toggle it on.

*on some devices you may need to enable developer options first. Google your device and enable developer options and it should give you a process to enable this. It usually involves tapping a section in about a rediculous amount of times.

# Install Android Software Development Kit and NDK
you got options:

You can install android studio https://developer.android.com/studio  . 

you can install from command line 
```sh
sudo snap install androidsdk
```

Once you have the sdk you can use the commandline tools to get versions of the sdk that you need. INFO: https://developer.android.com/tools#environment-variables

navigate to your sdk directory and navigate to the bin directory and run this code to add android-35 required by the egui example.
```sh
./sdkmanager "platforms;android-35"
```
if you installed with snap on ubuntu you should be able to run (you can also look at adding sdkmanager to your PATH)
```sh
sdkmanager "platforms;android-35"
```
if you run into issues running/compiling once then failing on subsequent runs you may need to change permissions in the sdk directory. Sometimes the files can be owned by root on linux. (look into CHMOD and CHOWN commands if you are unfamiliar)

Download and extract your NDK of choice from https://developer.android.com/ndk/downloads/
I placed this folder inside my sdk folder for ease of use.
I recommend getting the LTS (long term stable) version but I have tested most examples with android-ndk-r25b.

*you will have to run these commands alot throughout your Journey through the Galaxy so might as well add them to .bashrc or PATH or your startup script of choice. remember to change the path to your sdk and ndk directories
```sh
export ANDROID_HOME=/PATH/TO/YOUR/SDK/DIRECTORY
```

```sh
export NDK_HOME=/PATH/TO/YOUR/NDK/DIRECTORY
```
