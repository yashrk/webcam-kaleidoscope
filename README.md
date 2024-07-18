# webcam-kaleidoscope
## What is it?
A party activity, a moving wallpapar or a simple screensaver... We're not sure.
### What's going on?
Web camera observing something. 
May be a table with small things, your face or fingers, or what you'd like to show.
The application real-time transforms this image like a real kaleidoscope and shows results on the screen. 
It can rotate it fast or slow, change colors or make it quake, depending on used shaders.
Used shaders are ruled with a keyboard.
One can use a standard keyboard and mouse or some mini-keyboard or numpad. 
We used this one: https://aliexpress.ru/item/1005006109748866.html 
For others it may be necessary to develop its own controls module (see `src/controls`).
## What do you need?
Some *nix system, web camera and screen.
## How to configire?
In `config` folder create `local.json` file. Then redefine there parameters to change.
For example, if you want to make application fullscreen and to use our mini keyboard,write here
```
{"fullscreen": true, "keyboard": "mini"}
```
## How to run?
```bash
cargo run
```
## Standard controls
### Keyboard controls

| Key            | Action                 |
|----------------|------------------------|
| `r`            | Start/stop rotation    |
| `m`            | Change mesh            |
| `←`, `→`       | Change fragment shader |
| `↑`, `↓`       | Change vertex shader   |
| `PgUp`, `PgDn` | Change camera height   |
| `ESC`          | Exit                   |
| `9`            | Slow down rotation     |
| `0`            | Speed up rotation      |
| `7`            | Speed up cycle         |
| `8`            | Slow down cycle        |

### Mouse wheel
Change camera height

## Mini keyboad controls
### Wheels

| Action        | Wheel1                | Wheel2             | Wheel3 (big)            |
|---------------|-----------------------|--------------------|-------------------------|
| anticlockwise | Slow down shaders     | Slow down rotation | Camera down             |
| press         | Default shaders speed | Stop rotation      | Default camera position |
| clockwise     | Speed up shaders      | Speed up rotation  | Camera up               |

### Keys

| Col1       | Col2               | Col3                 |
|------------|--------------------|----------------------|
| Next mesh  | Next vertex shader | Next fragment shader |
| default.vs | inner_rotation.vs  | quake.vs             |
| default.fs | test.fs            | rgb_corners.fs       |
| corners.fs | time.fs            | rainbow.fs           |

## Local configuration
To redefine config params put `local.json` file to `config` folder.
