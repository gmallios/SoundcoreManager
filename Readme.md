<img src=".github/splash.png" alt="Tauri" />

<h3 align="center">
 A desktop companion app for your Soundcore devices
</h3>
<br>
<h5 align="center">

![Github Actions](https://github.com/gmallios/SoundcoreManager/actions/workflows/push.yml/badge.svg)
[![Donate](https://img.shields.io/badge/Donate-PayPal-green.svg)](https://www.paypal.com/donate/?hosted_button_id=58VZ5TZFZXACJ)

<h5>

# Features

- See charging status and battery level
- Set ANC Modes
- Adjust EQ

# Installing

- Check the [releases section](https://github.com/gmallios/SoundcoreManager/releases) for a stable build or try the latest unstable [Windows](https://nightly.link/gmallios/SoundcoreManager/workflows/push/master/SoundcoreManager-windows-latest.exe.zip)
  standalone binary

# Supported Devices - Mostly Tested

| Model ID | Name              |
| -------- | ----------------- |
| A3951    | Liberty Air 2 Pro |
| A3027    | Life Q35          |
| A3028    | Life Q30          |
| A3029    | Life Tune         |
| A3935    | Life A2 NC        |

# Planned Support - Need testers

| Model ID | Name          |
| -------- | ------------- |
| A3040    | Space Q45     |
| A3025    | Life Q20      |
| A3033    | Live 2 Neo    |
| A3931    | Life Dot 2    |
| A3992    | Soundcore A3i |
| A3993    | Soundcore P3i |

# Build Instructions

### Requirements

- Rust
- Yarn/Node

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
