# evelyn-rs
![screenshot](screenshot.png)

## About
**evelyn-rs** is an open source server emulator for the game **Zenless Zone Zero**.

## Features
- Training battles
- A bit of main city logic (changing character and time period)
- NPC spawn and interaction system via EventGraph as intended
- Inventory (currently only W-Engines)
- Player data persistence
- A version-agnostic protocol library, allowing to modify protocol capabilities in most painless way

## Getting started
### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [SurrealDB](https://surrealdb.com/docs/surrealdb/installation)
##### NOTE: to start SurrealDB, use this command: `surreal start -u root -p root surrealkv://evelyn`

### Setup
#### a) building from sources (preferred)
```sh
git clone https://git.xeondev.com/evelyn-rs/evelyn-rs.git
cd evelyn-rs
cargo run --bin evelyn-autopatch-server
cargo run --bin evelyn-sdk-server
cargo run --bin evelyn-dispatch-server
cargo run --bin evelyn-gate-server
cargo run --bin evelyn-game-server
```
#### b) using pre-built binaries
Navigate to the [Releases](https://git.xeondev.com/evelyn-rs/evelyn-rs/releases) page and download the latest release for your platform.<br>
Launch all services: `evelyn-autopatch-server`, `evelyn-sdk-server`, `evelyn-dispatch-server`, `evelyn-gate-server`, `evelyn-game-server`

### Configuration
You should configure each service using their own config files. They're being created in current working directory upon first startup.

### Connecting
You have to get a compatible game client. Currently supported one is `CNBetaWin1.5.0`, you can [get it here](https://git.xeondev.com/xeon/3/raw/branch/3/ZZZ_1.5_beta_reversedrooms.torrent). Next, you have to apply [this patch](https://git.xeondev.com/evelyn-rs/evelyn-Patch/releases), it allows you to connect to local server and replaces RSA encryption keys with custom ones.
##### NOTE: you have to create in-game account, by default, you can do so at http://127.0.0.1:10001/account/register
##### NOTE2: to register an account, you should have `evelyn-sdk-server` up and running!

## Community
[Our Discord Server](https://discord.gg/reversedrooms) is open for everyone who's interested in our projects!

## Support
Your support for this project is greatly appreciated! If you'd like to contribute, feel free to send a tip [via Boosty](https://boosty.to/xeondev/donate)!

## Friendly reminder
The server is in a very early state. Right now, it's NOT recommended to run this on a production environment. Please don't open issues about missing features, I'm well aware of this.

## Sanity
If you want to lose your sanity, consider checking [this](crates/qwer-rpc/src/)
