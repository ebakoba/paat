# Paat

### Overview

Paat is a bot for finding spots on ferries moving between Hiiumaa, Saaremaa and mainland Estonia. Even when the ferries are sold out people often change their tickets or opt to go at an earlier time. Therefore, continues polling of the available spots can lead to a ferry ticket at the desired time. Paat helps to simplify it by polling after every 30 seconds, removing the need to keep refreshing the ferry website.

Paat consists of two parts: `paat-core` which implements logic for polling, `paat-cli` which is the user facing CLI application. In future there is a plan to add more user facing applications such `paat-tui` and `paat-desktop`.

### Usage

```bash
paat-cli
```

![Paat Usage GIF](assets/paat-usage.gif)

Oh, and it will play sound too🎵!

### Installation

#### Executables

Compiled executables can be downloaded under [Release section](https://github.com/ebakoba/paat/releases/).

#### Using Cargo

```bash
cargo install paat-cli
```

#### From source

```bash
git clone https://github.com/ebakoba/paat
cd paat/paat-cli
cargo install --path .
```

### Limitations

Paat is written agains internal API of [praamid.ee](praamid.ee). Changes at the internal API can break functionality of Paat at any moment.

Currently Paat can only poll spots for small vehicles.
