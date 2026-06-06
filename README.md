<h1 align="center"><a href="https://github.com/WilliamVenner/gmpublisher/releases">Download</a></h1>

<p align="center"><img src="https://github.com/WilliamVenner/gmpublisher/assets/14863743/8c514124-7f63-4d20-b6c6-25989c9bf528"/></p>

# gmpublisher

A Workshop publishing utility for Garry's Mod — publish, update, browse, extract, and manage addons without `gmad.exe` or `gmpublish.exe`.

[Download releases](https://github.com/WilliamVenner/gmpublisher/releases)

> **Stability warning — read this first**
>
> gmpublisher has recently been migrated from **Tauri v1 to Tauri v2**. This was a large internal upgrade touching the Rust backend, permissions model, packaging, and minimal frontend API bindings.
>
> **Do not trust the stability of current builds.** Expect crashes, broken dialogs, permission errors, platform-specific packaging issues, and regressions in features that previously worked. Treat every release from this migration onward as experimental until it has been thoroughly tested on your platform.
>
> If you depend on gmpublisher for production publishing workflows, stay on the last pre-migration release until newer builds have been validated for your use case. Report issues on GitHub with your OS, version, and steps to reproduce.

###### Are you a developer? You may also like [VSCode GLua Enhanced](https://github.com/WilliamVenner/vscode-glua-enhanced).

## Tauri v2 upgrade

Recent versions run on [Tauri 2](https://v2.tauri.app/) instead of Tauri 1. Notable changes:

- **Desktop shell:** Tauri 2 with a Rust `lib.rs` entry point, capabilities-based permissions, and the dialog plugin for native file pickers.
- **Linux:** Requires **WebKitGTK 4.1** (`libwebkit2gtk-4.1-dev` when building; `webkit2gtk-4.1` on Arch). CI bundles updated accordingly.
- **Frontend:** Minimal API import changes only (`@tauri-apps/api/core`, `@tauri-apps/plugin-dialog`). No intentional UI redesign.
- **Windows / macOS:** MSI and universal macOS builds need re-validation after the migration. Do not assume installers behave identically to Tauri 1 builds.

The migration guide used as reference: [Upgrade from Tauri 1.0](https://v2.tauri.app/start/migrate/from-tauri-1/).

## Installation

### Windows and macOS

Download the latest build from the [releases page](https://github.com/WilliamVenner/gmpublisher/releases). Verify it works for your workflow before relying on it — see the stability warning above.

### Linux

Use the ZIP from the [releases page](https://github.com/WilliamVenner/gmpublisher/releases), or install from the AUR:

- **Arch Linux:** [`gmpublisher-bin`](https://aur.archlinux.org/packages/gmpublisher-bin) (depends on `webkit2gtk-4.1`)

Other distributions may need additional WebKit/GTK dependencies. Again: treat post-migration builds as untrusted until you have tested them locally.

## Building from source

Prerequisites: [Node.js](https://nodejs.org/), [Rust](https://www.rust-lang.org/) (see `src-tauri/rust-toolchain.toml`), and platform Tauri dependencies.

**Linux (Debian/Ubuntu-based):**

```sh
sudo apt install libwebkit2gtk-4.1-dev build-essential libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

```sh
npm install
npm run tauri dev    # development
npm run tauri build  # production binary / bundle
```

Steam API shared libraries are copied automatically via `src-tauri/copySharedLibs.js` before dev and build.

## Tutorials

- [DanFMN — Fastest Way to Upload a Garry's Mod Addon To Workshop](https://youtu.be/_syLXTFXmgM)
- [DarkFated — GMPublisher Guide in Russian](https://youtu.be/ldjHm85AnYk)

## Features

- Publish and update Workshop items without Valve's publish tools
- Extract, search, and browse GMA files and installed addons
- Bulk download and extract Workshop items and collections
- Upload animated GIFs as Workshop item icons
- Addon size analyzer treemap
- Legacy SteamPipe addons and old GMA versions
- Works without an Internet connection (after initial setup)
- CLI interface
- (Windows) `.gma` file type association for quick extraction

## Languages

![](https://user-images.githubusercontent.com/14863743/115954244-ce459780-a4e7-11eb-9237-92eab7d17814.png) English

![](https://user-images.githubusercontent.com/14863743/115954306-195faa80-a4e8-11eb-8489-07ceca216211.png) French

![](https://user-images.githubusercontent.com/14863743/115954290-03ea8080-a4e8-11eb-86df-9001929981a7.png) German

![](https://user-images.githubusercontent.com/14863743/115957563-18844400-a4fb-11eb-9828-cf76b15c6a48.png) Russian

![](https://user-images.githubusercontent.com/14863743/116080210-ad6c7600-a690-11eb-8c26-33de913e7ad0.png) Polish

![](https://user-images.githubusercontent.com/14863743/115975014-223c9480-a559-11eb-81c4-6a0bfc0fdb9d.png) Turkish

![](https://user-images.githubusercontent.com/14863743/116463612-cfb5ed80-a862-11eb-81f1-fb453cf77da5.png) Portuguese (Brazil)

![](https://user-images.githubusercontent.com/14863743/115976530-d7297e00-a566-11eb-9fe0-113c59ce49ce.png) Spanish

![](https://user-images.githubusercontent.com/14863743/123729167-754e0300-d88c-11eb-9dae-6fb82e0ca0ce.png) Chinese

![](https://user-images.githubusercontent.com/14863743/123729280-9dd5fd00-d88c-11eb-8aee-0360615d4d57.png) Dutch

![](https://github.com/WilliamVenner/gmpublisher/assets/14863743/31a1a199-1427-483c-bf6c-140116e3f445) Korean

![](https://github.com/Blueberryy/gmpublisher/assets/36592509/319e7681-46c4-4a79-9fdc-99db49bd2ccb) Ukrainian

[Want to translate gmpublisher?](https://github.com/WilliamVenner/gmpublisher/tree/master/i18n)

## Requirements

Windows, macOS, or Linux.

Linux users need WebKitGTK 4.1 and related GTK dependencies. Post-migration builds have not been battle-tested across all distros.

## Technical notes

- Heavy use of multithreading; benefits from multi-core CPUs
- **Rust** backend, **Svelte** frontend, **[Tauri 2](https://v2.tauri.app/)** desktop shell (not Electron)
- [steamworks-rs](https://crates.io/crates/steamworks) for Steamworks SDK integration
- Release binary is roughly ~10–15 MB depending on platform

## Media

![Screenshot](https://user-images.githubusercontent.com/14863743/115953601-5f1a7400-a4e4-11eb-831c-d6a924afbf33.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953605-63469180-a4e4-11eb-9f96-90b992cbffc4.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115954341-5b88ec00-a4e8-11eb-8f27-c03d43df165a.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953616-7c4f4280-a4e4-11eb-95c0-add80b1d41bd.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953639-9db02e80-a4e4-11eb-935d-bad41cd30bde.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115958825-00afbe80-a501-11eb-81da-6d53a94eddbf.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953801-845bb200-a4e5-11eb-8fc2-8b142f2be237.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953820-99d0dc00-a4e5-11eb-93a4-36e8b2248e87.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953827-a35a4400-a4e5-11eb-9691-48e520eb9bb1.png)

![Screenshot](https://user-images.githubusercontent.com/14863743/115953670-bb7d9380-a4e4-11eb-8f54-f43fcd153d90.png)

<p align="center"><img src="https://i.imgur.com/Un4akZe.gif"/></p>
