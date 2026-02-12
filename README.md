# Foolhammer Mod Manager

Foolhammer Mod Manager is a desktop application designed to simplify mod management for Total War games. This is a straightforward, no-frills mod manager that does one thing well: manages your mods.

## ✨ Features

- 🗂️ Organize and manage Total War game mods
- 🔄 Steam Workshop integration via Steamworks SDK
- 🎨 Modern, responsive UI built with Vue 3 and Tailwind CSS
- 🌍 Internationalization support (i18n)
- 🖥️ Cross-platform desktop support (Windows, Linux)
- 🐧 First-party Linux support

## 🎯 Supported Games

Currently supported:
- **Total War: Warhammer III**

> **Note:** PRs for additional Total War titles are welcome!

## 📋 Prerequisites

Before you begin, ensure you have the following installed:
- [Node.js](https://nodejs.org/) (>=24.13.0)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) and Cargo
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform

## 🚀 Getting Started

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

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/)
- Based on [tauri-nuxt-boilerplate](https://github.com/tpkee/tauri-nuxt-boilerplate)
- Uses [RPFM Library](https://github.com/Frodo45127/rpfm) for pack file management
- Steam integration via [steamworks-rs](https://github.com/Thinkofname/steamworks-rs)

## 🐛 Issues

If you encounter any issues or have suggestions, please [open an issue](https://github.com/tpkee/foolhammer-mod-manager/issues) on GitHub.

---

**Note:** This project is in active development. Features and documentation may change.
