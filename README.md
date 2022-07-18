# Rust AzureAD Exploitation Framework (`RAADEF`)

[![Language](https://img.shields.io/badge/Lang-Rust-blue.svg)](https://www.https://www.rust-lang.org/.org)
[![License](https://img.shields.io/badge/License-Apache%202.0-red.svg)](https://opensource.org/licenses/Apache-2.0)

## An extensible `Rust`-based exploitation framework designed to assist red teamers and security professionals in assessing `AzureAD` environments.

`RAADEF` aims at streamlining and simplifying the process of auditing/attacking `AzureAD` environments.

`Rust` was selected as programming language for `RAADEF` due to its great performance, tooling, and active community.

Currently, `RAADEF` is limited to [this set of features](#Features). Having said that, *ideally*, this framework will grow and embed additional features and attack vectors thanks to the [community contributions](#Contributing) - special focus on the [roadmap](#Roadmap).

## Features

- Fine-tuning of attacks settings.
- Password brute forcing.
- Password spraying.
- Supported authentication endpoints are:
    - https://login.microsoft.com/common/oauth2/token

## Roadmap

- [ ] *Beautify* the `console`/`file` output (e.g., `progress bar`, `colors`, silence `reqwest`).
- [ ] Implement `pause` and `resume` options! 🤩
- [x] ~~Implement support for additional authentication endpoints.~~
- [ ] Implement support for cycling through `resource principals`.
- [ ] Implement support for lockout detection -> `lockout` and `force` flags.
- [x] ~~Implement support for requests `delay`.~~
- [ ] Improve the logic around the endpoint `HashMap`/`CLI parser`, e.g., try to get away with using the `pub enum Resource` and fetch options direcly from the `HashMap keys` instead.
- [x] ~~Restructure the code -> more modularity por favor! 🌯~~

## Requirements

- [cargo](https://github.com/rust-lang/cargo)
- [rustup](https://www.rust-lang.org/tools/install)

## Installation

1. Clone/download the repository:

    ```powershell
    git clone https://github.com/aress31/raadef
    cd raadef
    ```

3. Compile/run `raadef` with:

    ```powershell
    cargo build --release
    .\target\release\raadef.exe
    ```

    ```powershell
    cargo run --
    ```

## Usage

```powershell
.\target\release\raadef.exe --help
```

## Sponsor 💓

If you want to support this project and appreciate the time invested in developping, maintening and extending it; consider donating toward my next (cup of coffee ☕/lamborghini 🚗) - as **a lot** of my **personal time** went into creating this project. 😪

It is easy, all you got to do is press the `Sponsor` button at the top of this page or alternatively [click this link](https://github.com/sponsors/aress31). 😁

## Reporting Issues

Found a bug 🐛? I would love to squash it!

Please report all issues on the GitHub [issues tracker](https://github.com/aress31/raadef/issues).

## Contributing

You would like to contribute to better this project? 🤩

Please submit all `PRs` on the GitHub [pull requests tracker](https://github.com/aress31/raadef/pulls).

## Acknowledgements

Give to Caesar what belongs to Caesar:

- [Spray365](https://github.com/MarkoH17/Spray365)
- [msspray.py](https://github.com/SecurityRiskAdvisors/msspray)

## License

`RAADEF` is primarily distributed under the terms of the `Apache License (Version 2.0)`.

See [LICENSE](./LICENSE) for details.