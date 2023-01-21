
# Soundcore Manager

A desktop companion app to control and monitor your Soundcore earbuds.

## Features
- See charging status and battery level 
- Set ANC Modes
- Adjust EQ
- Cross-Platform. NOTE: Windows works. macOS is experimental. No Linux support yet.


## Instructions

#### Install deps
```
npm install
```

#### To build and create an installer

```
  npm run tauri build
```

#### To run dev build

```
  npm run tauri dev
```

## Supported Devices (Not guaranteed)

| Model ID | Name              | Features Supported |
|----------|-------------------|--------------------|
| A3951    |Liberty Air 2 Pro  | All (Tested)       |
| A3027    |Life Q35           | Most/In-Progress   |
| A3033    |Live 2 Neo         | Untested           |
| A3025    |Life Q20           | Untested           |
| A3935    |Life A2 NC         | Untested           |
| A3931    |Life Dot 2         | Untested           |
| A3029    |Life Tune          | Untested           |

Note: Untested devices have similar implementation as a known working model.

## Roadmap
- Move device polling from Frontend to a background thread (Tauri-Event Based?)

## License

[MIT](https://choosealicense.com/licenses/mit/)

