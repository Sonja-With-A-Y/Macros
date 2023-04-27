# Universal Macros
 Vim style macros, anywhere (except Wayland).

## What does that mean?
It means you can quickly record and play back sequences of keys outside of any particular application. Macros are recorded into one of 26 memory slots, one for each letter, and then played back with that letter.

## State of the project
This program is still being developed, right now it's just complete enough that I find it worthwhile to use, but it's still very rough around the edges.

My current goals are making it more configurable, and setting up proper installation methods.

## Installation
Currently, the only way to use it is to run it from source. This requires cargo so make sure you have that installed.

```
git clone https://github.com/Sonja-With-A-Y/universal-macros ; cd universal-macros
cargo run
```

## Usage
I recommend finding some sort of way to run the program from a hotkey. I use the i3 window manager which makes this simple to do (will add recommendations for this for different operating systems).

After starting the program, tap right control to choose to record, then tap a letter to choose a memory slot. Your key presses will be recorded until you tap right control again. If you chose a, then it will be recorded into "a.txt".

The program will end when you finish recording. To play that macro, start it up again then tap right shift, followed by the letter. The program also terminates after a macro has been played.
