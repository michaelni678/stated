This directory contains code vendored from [`safer_ffi`](https://github.com/getditto/safer_ffi).

## Vendored Code

- [`squote!`](https://github.com/getditto/safer_ffi/blob/96aa942d8a6ac019b774071aa6c3f46c2f3942c5/src/proc_macro/utils/macros.rs#L209) macro.

## Modifications

- Minor style changes.
- Added `parse_squote!` which calls [`syn::parse_quote_spanned!`](https://docs.rs/syn/latest/syn/macro.parse_quote.html).

## License

The license file in this directory **only** applies to the vendored `safer_ffi` code. It does **not** apply to the rest of this project.