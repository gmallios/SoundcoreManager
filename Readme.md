<img src=".github/splash.png" alt="Tauri" />

<h3 align="center">
 A desktop companion app for your Soundcore devices
</h3>
<br>

# Downloads

- [Windows Latest](https://nightly.link/gmallios/SoundcoreManager/workflows/push/master/SoundcoreManager-windows-latest.exe.zip) binary or check the Releases section for an installer(Not recomended since the app is constantly updated).

# Features

- See charging status and battery level
- Set ANC Modes
- Adjust EQ

# Supported Devices

| Model ID &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; | Name &nbsp; &nbsp; &nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; | Features Supported &nbsp; &nbsp; &nbsp;&nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; &nbsp; |
| ---------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A3951                                                                                                      | Liberty Air 2 Pro                                                                       | All                                                                                                                                                                  |
| A3025                                                                                                      | Life Q20                                                                                | Untested                                                                                                                                                             |
| A3027                                                                                                      | Life Q35                                                                                | All/In-Progress                                                                                                                                                      |
| A3028                                                                                                      | Life Q30                                                                                | All/In-Progress                                                                                                                                                      |
| A3029                                                                                                      | Life Tune                                                                               | All/In-Progress                                                                                                                                                      |
| A3033                                                                                                      | Live 2 Neo                                                                              | Untested                                                                                                                                                             |
| A3040                                                                                                      | Space Q45                                                                               | None/In-Progress                                                                                                                                                     |
| A3935                                                                                                      | Life A2 NC                                                                              | Untested                                                                                                                                                             |
| A3931                                                                                                      | Life Dot 2                                                                              | Untested                                                                                                                                                             |
| A3992                                                                                                      | Soundcore A3i                                                                           | None/In-Progress                                                                                                                                                     |

### Note: Untested devices have similar implementation as a known working model.

<br>

# Build Instructions

### Requirements

- Rust
- NodeJS

#### Install deps

```
yarn
```

#### To run in debug mode

```
yarn tauri dev
```

#### To build and create an installer

```
yarn tauri build
```

## Directory structure

##### Only important files/folders are listed

    `|-- bluetooth-lib/                  # Custom Bluetooth(non-BLE) library (Rust)
     |    `-- iobluetooth                # macOS sub-library
     |    |     |-- src/
     |    |          |-- server.rs       # macOS Daemon (gRPC used for communication)
     |    `-- src/
     |          |-- win32                # Win32 Implementation
     |          |-- macos                # macOS Implementation
     |
    `|-- soundcore-lib/                  # Soundcore library (Rust)
     |    |-- examples/                  # Examples on how to use the lib
     |    |-- statics.rs                 # Commands and constant values (e.g. EQ Indexes)
     |    `-- src/
     |          |-- devices              # Device implementations (e.g. A3951Device.rs)
     |
    `|-- src-tauri/                      # Tauri Backend (Rust)
    `|-- src/                            # Tauri Frontend (React) 

# License

[MIT](https://choosealicense.com/licenses/mit/)
