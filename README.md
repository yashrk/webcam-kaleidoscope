# webcam-kaleidoscope
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
