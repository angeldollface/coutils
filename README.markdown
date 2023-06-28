# COUTILS :hammer: :gear:

![GitHub CI](https://github.com/angeldollface/coutils/actions/workflows/rust.yml/badge.svg)

***A set of useful functions for Rust. :hammer: :gear:***

## ABOUT :books:

I found myself re-writing the same functions a million times in different Rust projects. Because this is not efficient and is quite time-consuming, I thought I'd unify these common functions into a library. This is that library. ***Coutils*** is short for ***Co***mmon ***Utilities***. Enjoy. :heart:

## INSTALLATION :inbox_tray:

To use ***Coutils*** in your Rust project, add this line to your project's `Cargo.toml`'s `[dependencies]` section:

```TOML
coutils = { git = "https://github.com/angeldollface/coutils", version = "1.3.0" }
```

## USAGE :hammer:

To understand what functions and entities *Coutils* offers, please make sure you have the [Rust](https://rust-lang.org) and [Git](https://git-scm.org) installed and execute the following steps:

- 1.) Download the source code:

```bash
git clone https://github.com/angeldollface/coutils.git
```

- 2.) Change directory into the root directory of the crate's source code:

```bash
cd coutils
```

- 3.) Generate the documentation and open it in your default browser:

```bash
cargo doc --open
```

## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.

### Version 1.1.0

- Added the `has_item` method.
- Added unit tests.
- Split the code into modules.

### Version 1.2.0

- Split everything into sub-modules.
- Refactored some functions.
- Added some new entities.
- Added a host of functions and entities.
- Added instructions for using and viewing API documentation.

### Version 1.3.0

- Added a function to copy files from `src/test_file.txt` to `dest/test_file.txt`.
- Updated documentation.
- Bumped version number.

## NOTE :scroll:

- *Coutils :hammer: :gear:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.