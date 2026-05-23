# dbus-media-track
dbus-media-track scans the Media Player and provides information, intended for use as a service.

## Features
### Service mode:
- Event-driven MPRIS updates — Uses D-Bus signal subscriptions instead of polling. `PropertiesProxy::receive_properties_changed()` provides a __real-time stream__ of MPRIS events with zero unnecessary background load and instant reaction to track, playback, and metadata changes.

- If the Media Player was killed (`SIGKILL -9`), then the signal stream will be stuck in standby mode.

- Additional stability without `Restart=always` in service-managers.

## Args
> Note: `spotify` is used in the examples.

### `--service` or `-s`
- Enables a stream/daemon mode.
- Must be the first argument when called.
### `--player` or `-p`
- Parsing a specific instance. Expects an argument, e.x.: `--player spotify`.
- Must be the second argument when calling or the first if `--service` is not used.
### `--title`
- Returns the found value: "__title__".
```rust
title: Some(
        "Master Of Puppets", // <- this
    )
```

### `--artist`
- Returns the found value: "__artist__".
```rust
artist: Some(
        [
            "Metallica", // <-
        ],
    ),
```

### `--album`
- Returns the found value: "__album__".
```rust
album: Some(
        "Master Of Puppets (Remastered)", // <-
    ),
```

### `--art_url`
- Returns the found value: "__art_url__".
```rust
art_url: Some(
        "https://art.example.com", // <-
    ),
```

### `--status`
- Displays the __PlaybackStatus__ variant returned by a D-Bus method.
```bash
dict entry(
         string "PlaybackStatus"
         variant string "Paused" # <-
      )
```
> Note: if the PlaybackStatus != Playing, an empty string is returned.

### Quotes (`" "`)
- Separator or add-on between received information.
```bash
dbus-media-track --player spotify "Song:" --artist "—" --title
Song: Metallica — Master Of Puppets
^^^^^		    ^
```
