# Rustaceans_Guide_to_the_Galaxy
Guide for fast simple Rust Developement on Android. 

I've spent the better part of 2 months researching and trying to find simple and fast methods for getting rust code to run on android devices. This guide is my attempt to simplify this search for anyone interested in learning rust AND wants to utilize its strengths in android apps or devices.

# Why Rust? 
Well if you have made it this far you know the basics; Fast, Secure, Modern, Compiles to most architecures... 
My reason: Cargo, I didn't want to mess with learning an IDE to actually code simple useful Apps. Cargo simplifies the process without the distraction of extra programs

No IDE? 
Yeah well not entirely true. I use Helix a modal text editor for terminals. And its written in rust so we can stay tribe loyal. Crab People unite!

# The Problem:
Android is not so friendly to new programming langueges like Rust. The examples offered by Android assume you know multiple languages (Java, Kotlin) and your just using rust to speed up computation and gluing it into other languages.

# The Goal:
I am looking for simple fast solutions to developing in rust for Android with minimal distractions like multiple IDE's and long convoluted processes.

# Solutions So Far:
The simpliest: Egui + Eframe + Rust .

Runner up: Cargo_mobile2 (currently barely maintaned by tauri).
It has 5 or so examples that compile if you just want get started, the Bevy examples seem the most stable and user friendly but bevy has a steep learning curve and is very large 1.5+ gigs.

The fastest: Termux.
These people are doing Gods work. Termux is a fully functional linux/bash terminal. You can install rust and helix and start ripping out code that runs right on your Android. If you only need commandline tools on Android this is THE way go to. With its companion X11 app you can even run gui applications (tho this has a steeper learning curve). 



