# PHTS NP-01: librespot

This is a fork of [ashthespy/librespot → `vollibrespot` branch][ashthespy/librespot] which is a an outdated fork of original [librespot] library.

Dependency of [NP-01_Vollibrespot] and used by [PHTS NP-01].

Changes:

- add more values (album release date and track number) to metadata
- fix JSON parsing errors in some cases

## Development

Clone this repo to [NP-01_Vollibrespot] and uncomment proper lines in [`Cargo.toml`](https://github.com/phts/NP-01_Vollibrespot/blob/master/Cargo.toml) file.

[librespot]: https://github.com/librespot-org/librespot
[ashthespy/librespot]: https://github.com/ashthespy/librespot/tree/vollibrespot
[NP-01_Vollibrespot]: https://github.com/phts/NP-01_Vollibrespot
[phts np-01]: https://tsaryk.com/NP-01
