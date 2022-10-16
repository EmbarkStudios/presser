<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 no-emphasis-as-heading -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🗜 presser`

**Utilities to help make copying data around into raw, possibly-uninitialized buffers easier and safer.**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/presser.svg)](https://crates.io/crates/presser)
[![Published Docs](https://docs.rs/presser/badge.svg)](https://docs.rs/presser)
[![Git Docs](https://docs.rs/presser/badge.svg)](https://embarkstudios.github.io/presser/presser/index.html)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/presser/status.svg)](https://deps.rs/repo/github/EmbarkStudios/presser)
</div>

`presser` can help you when copying data into raw buffers. One primary use-case is copying data into
graphics-api-allocated buffers which will then be accessed by the GPU. Common methods for doing this
right now in Rust can often invoke UB in subtle and hard-to-see ways. For example, viewing a GPU allocated
but uninitialized buffer as an `&mut [u8]` **is instantly undefined behavior**, and `transmute`ing even a
`T: Copy` type which has *any padding bytes in its layout* as a `&[u8]` to be the source of a copy is
**also instantly undefined behavior**, in both cases because it is *invalid* to create a reference to an invalid
value (and uninitialized memory is an invalid `u8`), *even if* your code never actually accesses that memory.
This immediately makes what seems like the most straightforward way to copy data into buffers unsound 😬.

`presser` helps with this by allowing you to view raw allocated memory of some size as a "`Slab`" of memory and then
provides *safe, valid* ways to copy data into that memory.

See more in [the docs](https://docs.rs/presser).

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
