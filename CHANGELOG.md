# Unreleased

# 0.6.3 - 2019-09-20

* [Removed usage of mutable global state from `#[auto_enum]` for forward compatibility.][60] See also [rust-lang/rust#63831].

[60]: https://github.com/taiki-e/auto_enums/pull/60
[rust-lang/rust#63831]: https://github.com/rust-lang/rust/pull/63831

# 0.6.2 - 2019-09-08

* Fixed links to generated code.

# 0.6.1 - 2019-09-08

* Improved documentation.

# 0.6.0 - 2019-09-07

* [Added `"unstable"` crate feature to separate unstable features from stable features.][56] When using features that depend on unstable APIs, the `"unstable"` feature must be explicitly enabled.

* Improved compile time.

* Renamed `#[rec]` to `#[nested]`.

* [Removed `marker(name)` option in favor of `marker = name`.][55]

* [Removed `never` option in argument position in favor of `#[enum_derive]` attribute.][48]

* [Removed `"proc_macro"` crate feature.][54]

* Added `"ops"` crate feature, and made `[std|core]::ops`'s `Deref`, `DerefMut`, `Index`, `IndexMut`, and `RangeBounds` traits optional.

* Added `"convert"` crate feature, and made `[std|core]::convert`'s `AsRef` and `AsMut` traits optional.

* Added `"generator_trait"` crate feature, and made `[std|core]::ops::Generator` traits optional. *(nightly-only)*

* Added `"fn_traits"` crate feature, and made `Fn`, `FnMut`, and `FnOnce` traits optional. *(nightly-only)*

* Added `"trusted_len"` crate feature, and made `[std|core]::iter::TrustedLen` traits optional. *(nightly-only)*

* Improved error messages.

(There are no changes since the 0.6.0-alpha.3 release.)

# 0.6.0-alpha.3 - 2019-09-06

* [Added `"unstable"` crate feature to separate unstable features from stable features.][56] When using features that depend on unstable APIs, the `"unstable"` feature must be explicitly enabled.

[56]: https://github.com/taiki-e/auto_enums/pull/56

# 0.6.0-alpha.2 - 2019-08-30

* [Removed `marker(name)` option in favor of `marker = name`.][55]

* [Removed `"proc_macro"` crate feature.][54]

[54]: https://github.com/taiki-e/auto_enums/pull/54
[55]: https://github.com/taiki-e/auto_enums/pull/55

# 0.6.0-alpha.1 - 2019-08-24

* Renamed `#[rec]` to `#[nested]`.

* [Removed `never` option in argument position in favor of `#[enum_derive]` attribute.][48]

* Improved compile time.

* Added `"ops"` crate feature, and made `[std|core]::ops`'s `Deref`, `DerefMut`, `Index`, `IndexMut`, and `RangeBounds` traits optional.

* Added `"convert"` crate feature, and made `[std|core]::convert`'s `AsRef` and `AsMut` traits optional.

* Added `"generator_trait"` crate feature, and made `[std|core]::ops::Generator` traits optional. *(nightly-only)*

* Added `"fn_traits"` crate feature, and made `Fn`, `FnMut`, and `FnOnce` traits optional. *(nightly-only)*

* Added `"trusted_len"` crate feature, and made `[std|core]::iter::TrustedLen` traits optional. *(nightly-only)*

* Improved error messages.

[48]: https://github.com/taiki-e/auto_enums/pull/48

# 0.5.10 - 2019-08-15

* Updated `proc-macro2`, `syn`, and `quote` to 1.0.

* Updated `derive_utils` to 0.9. This improves the error message.

* Added some generated code examples.

# 0.5.9 - 2019-07-07

* Updated to support `futures-preview` 0.3.0-alpha.17.

* Added some generated code examples.

# 0.5.8 - 2019-05-22

* Added support for `futures::io::{AsyncSeek, AsyncBufRead}`.

# 0.5.7 - 2019-05-12

* Updated to new nightly. `iovec` stabilized. `#[enum_derive]` automatically detects the rustc version and supports `Read::read_vectored` and `Write::write_vectored` as the part of `Read` and `Write`.

* Supported for latest `futures` 0.3.0-alpha.16.

# 0.5.6 - 2019-04-16

* Updated to new nightly. `futures_api` stabilized.

# 0.5.5 - 2019-03-29

* Fixed trait support in `"type_analysis"` feature.

# 0.5.4 - 2019-03-14

* Fixed the problem that `"failed to resolve: use of undeclared type or module"` (E0433) error is shown when one or more compilation errors occur when multiple `#[auto_enum]` attributes are used.

* Improved the error message of `#[enum_derive]` attribute.

* Updated minimum `derive_utils` version to 0.7.0. This improves the error message.

# 0.5.3 - 2019-03-13

* Greatly improved the error message of `#[auto_enum]` attribute.

# 0.5.2 - 2019-03-10

* Added some generated code examples.

* Added `"iovec"` crate feature. This supports the unstable `iovec` feature ([rust-lang/rust#58452](https://github.com/rust-lang/rust/issues/58452)).

* Updated minimum `syn` version to 0.15.29. This fixes some warnings.

# 0.5.1 - 2019-03-03

* Fixed examples and some sentence in README.md.

# 0.5.0 - 2019-03-03

* Transition to Rust 2018. With this change, the minimum required version will go up to Rust 1.31.

* Reduced the feature of `"std"` crate feature. The current `"std"` crate feature only determines whether to enable `std` library's traits (e.g., `std::io::Read`) support. `"std"` crate feature is enabled by default, but you can reduce compile time by disabling this feature.

* Fixed problem where "macro attributes must be placed before `#[derive]`" error occurred when `#[enum_derive]` attribute was used with other attributes.

* No longer need `#[macro_use] extern crate auto_enums;`. You can use `#[auto_enum]` attribute by `use auto_enums::auto_enum;`.

* Removed `"unstable"` crate feature.

# 0.4.1 - 2019-02-21

* Updated to new nightly.

* Added some generated code examples.

* Updated minimum `derive_utils` version to 0.6.3.

* Updated minimum `syn` version to 0.15.22.

* Updated minimum `smallvec` version to 0.6.9.

# 0.4.0 - 2019-01-30

* Added support for `?` operator in functions and closures.

* Added support for `[core|std]::ops::Generator`.

# 0.3.8 - 2019-01-26

* Updated minimum `derive_utils` version to 0.6.1.

* Updated minimum `smallvec` version to 0.6.8.

# 0.3.7 - 2019-01-26

* Fixed bug of closure support.

# 0.3.6 - 2019-01-19

* Parentheses and type ascription can now be searched recursively.

# 0.3.5 - 2019-01-09

* Improved performance of `#[auto_enum]` attribute.

* Updated minimum `derive_utils` version to 0.6.0.

# 0.3.4 - 2019-01-06

* Added support for `futures::{AsyncRead, AsyncWrite}`.

# 0.3.3 - 2019-01-04

* Updated minimum `derive_utils` version to 0.5.4.

# 0.3.2 - 2018-12-27

* Improved error messages.

* Updated minimum `derive_utils` version to 0.5.3.

# 0.3.1 - 2018-12-26

* Updated minimum `derive_utils` version to 0.5.1. This includes support to stable Pin API.

# 0.3.0 - 2018-12-24

* Added support for `break` in loop. This includes support for nested loops and labeled `break`.

* Changed `#[enum_derive(Error)]` implementation. [The code generated by the new implementation](docs/supported_traits/std/error.md).

* Removed `"error_cause"` crate feature.

* Updated minimum `derive_utils` version to 0.5.0.

# 0.2.1 - 2018-12-22

* Updated minimum `derive_utils` version to 0.4.0.

# 0.2.0 - 2018-12-20

* Added support for `return` in function and closure.

* Added `"fmt"` crate feature, and made `[std|core]::fmt`'s traits other than `Debug`, `Display` and `Write` optional.

# 0.1.3 - 2018-12-15

* Changed `#[enum_derive(Error)]` implementation. In Rust 1.33, `Error::cause` is deprecated. In the new implementation, `Error::cause` is optional for Rust 1.33 and later. In versions less than 1.33, `Error::cause` is always implemented.

# 0.1.2 - 2018-12-15

* Moved features of derive/utils to [derive_utils](https://github.com/taiki-e/derive_utils).

* Aligned version number of `auto_enumerate` and `auto_enums`.

# 0.1.1 - 2018-12-13

* Renamed from `auto_enumerate` to `auto_enums`.

# 0.1.0 - 2018-12-09

Initial release
