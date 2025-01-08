# `roguelike`

[![Build Status][ci-shield]][ci-url]
[![Rust][rust-shield]][rust-url]
[![MIT License][license-shield]][license-url]

Tiny hobby / free-time roguelike game pet-project. 🔮🧝🏻🪄

```
    ____                         __    _ __
   / __ \____  ____ ___  _____  / /   (_) /_____
  / /_/ / __ \/ __ `/ / / / _ \/ /   / / //_/ _ \
 / _, _/ /_/ / /_/ / /_/ /  __/ /___/ / ,< /  __/
/_/ |_|\____/\__, /\__,_/\___/_____/_/_/|_|\___/
            /____/
```

## Arch Linux (+ EndeavourOS) Dependencies

- https://archlinux.org/packages/extra/x86_64/sdl2/
- https://archlinux.org/packages/extra/x86_64/sdl2_image/

```
yay -S sdl2 sdl2_image
```

## Ubuntu Dependencies

```
sudo apt install libsdl2-dev libsdl2-image-dev
```

## MacOS Dependencies

Tested on MBP 14" (Nov 2023), M3 Pro, 36 GB RAM, Sonoma 14.7.1.

```
brew install sdl2
brew install sdl2_image

brew link sdl2
brew link sdl2_image

export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
```

## Assets

### `./assets/orc`

Source: [click here](https://craftpix.net/freebies/free-top-down-orc-game-character-pixel-art).

### `./assets/dungeon`

Source: [click here](https://pixel-poem.itch.io/dungeon-assetpuck).

## License (except assets)

[MIT](./LICENSE.md)

[ci-shield]: https://img.shields.io/github/actions/workflow/status/resurtm/roguelike/ci.yml?style=for-the-badge
[ci-url]: https://github.com/resurtm/roguelike/actions/workflows/ci.yml
[rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org/
[license-shield]: https://img.shields.io/github/license/resurtm/roguelike?style=for-the-badge
[license-url]: https://github.com/resurtm/roguelike/blob/main/LICENSE.md
