<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 no-emphasis-as-heading -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

<!--- FIXME: Pick an emoji and name your project! --->
# `🌻 opensource-template`

<!--- FIXME: Write short catchy description/tagline of project --->
**Template for creating new open source repositories that follow the Embark open source guidelines**

<!--- FIXME: Update crate, repo and CI workflow names here! Remove any that are not relevant --->

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/rust-gpu.svg)](https://crates.io/crates/rust-gpu)
[![Docs](https://docs.rs/rust-gpu/badge.svg)](https://docs.rs/rust-gpu)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/rust-gpu/status.svg)](https://deps.rs/repo/github/EmbarkStudios/rust-gpu)
[![Build status](https://github.com/EmbarkStudios/physx-rs/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/physx-rs/actions)
</div>

## TEMPLATE INSTRUCTIONS

1. Create a new repository under EmbarkStudios using this template.
1. **Title:** Change the first line of this README to the name of your project, and replace the sunflower with an emoji that represents your project. 🚨 Your emoji selection is critical.
1. **Badges:** In the badges section above, change the repo name in each URL. If you are creating something other than a Rust crate, remove the crates.io and docs badges (and feel free to add more appropriate ones for your language).
1. **CI:** In `./github/workflows/` rename `rust-ci.yml` (or the appropriate config for your language) to `ci.yml`. And go over it and adapt it to work for your project
1. **CHANGELOG.md:** Change the `$REPO_NAME` in the links at the bottom to the name of the repository, and replace the example template lines with the actual notes for the repository/crate.
1. **release.toml:** in `./release.toml` change the `$REPO_NAME` to the name of the repository
1. **Cleanup:** Remove this section of the README and any unused files (such as configs for other languages) from the repo.

## Contribution

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
