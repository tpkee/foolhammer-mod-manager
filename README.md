<p align="center">
  <img src="src-tauri/icons/Square150x150Logo.png"   />
</p>

# Foolhammer Mod Manager

A straightforward mod manager for Total War games.

## Table of Contents

- [Features](#features)
- [Planned Features](#planned-features)
- [Supported Games](#supported-games)
- [Installation](#installation)
  - [Linux](#linux)
- [Prerequisites](#prerequisites)
- [Contributing](#getting-started)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Issues](#issues)

## Features

- Profiles
- Mod groups
- First-party Linux support

## Planned Features

- Profile export (pack files + profile JSON)
- Windows support
- Steamworks integration (subscribe/unsubscribe)

## Supported Games

- **Total War: Warhammer III**

To add another title: add its defaults in `src-tauri/src/defaults/games.rs`, a game icon under `public/images/games/`, and a translation entry in the locale file(s).

## Installation

### Linux
<details>
  <summary>Arch Linux (AUR)</summary>

  [![AUR version](https://img.shields.io/aur/version/foolhammer-mod-manager-bin?style=flat&label=foolhammer-mod-manager-bin)](https://aur.archlinux.org/packages/foolhammer-mod-manager-bin)

  The package [foolhammer-mod-manager-bin](https://aur.archlinux.org/packages/foolhammer-mod-manager-bin) is available on the AUR.

  You can install it using your preferred AUR helper, for example:

  ```bash
  # Using yay
  yay -S foolhammer-mod-manager-bin

  # Using paru
  paru -S foolhammer-mod-manager-bin
  ```

  Please see [the Arch Wiki](https://wiki.archlinux.org/title/Arch_User_Repository#Installing_and_upgrading_packages) for more information on installing AUR packages.

</details>

<details>
  <summary>Debian</summary>
  Download the deb from the release page
</details>

<details>
  <summary>Fedora</summary>
  Download the rpm from the release page
</details>

<details>
  <summary>Build from source</summary>

### Installation
You *must* have the [prerequisites](#prerequisites) first.

```bash
git clone https://github.com/tpkee/foolhammer-mod-manager.git
cd foolhammer-mod-manager
pnpm install
pnpm build
```
</details>

## Prerequisites

Before you begin, ensure you have the following installed:
- [Node.js](https://nodejs.org/) (>=24.13.0)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) and Cargo
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

## Getting Started

### Installation

1. Clone the repository:
```bash
git clone https://github.com/tpkee/foolhammer-mod-manager.git
cd foolhammer-mod-manager
```

2. Install dependencies:
```bash
pnpm install
```

### Development

Run the app:
```bash
pnpm dev
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

> **Note:** No AI slop pls 

## License

This project is licensed under the GPL3 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Uses [RPFM Library](https://github.com/Frodo45127/rpfm) for pack file management
- Steam integration via [steamworks-rs](https://github.com/Thinkofname/steamworks-rs)

## Issues

If you encounter any issues or have suggestions, please [open an issue](https://github.com/tpkee/foolhammer-mod-manager/issues) on GitHub.

---

**Note:** This project is in active development. Features and documentation may change.
