# EskomSePush API

This Rust library is an unofficial library to the [EskomSePush](https://sepush.co.za) API 

## TODO

- [x] Improve docs
- [x] Add enum types where relevant
- [x] Improve error types
- [x] Add examples
- [x] Allow option for getting API key from environment variables
- [ ] Features to control sync and async functionality
- [ ] Add unit tests
- [x] Improve status struct to allow for more flexible structure for future proofing (Towns/cities with different loadshedding schedule might be added in the future)
- [x] Add helper functions
- [ ] Restucture code based on this article by Gitlab crate maintainers [Designing Rust bindings for REST APIs](https://plume.benboeckel.net/~/JustAnotherBlog/designing-rust-bindings-for-rest-ap-is)

## Breaking Changes

 * You can now just build the URL without using one of the preconfigured http clients and use your preferred one instead
 * The preconfigured clients are now all locked behind features
 * A [`ureq`](https://crates.io/crates/ureq) client has been added but requires the `ureq` feature to be enabled
 * There are response handlers available for both `ureq` and `reqwest` http clients
 * There are builders for each URL endpoint so you just need to use what you need

## Examples

You can view the [examples here](https://github.com/ZimboPro/eskom-se-push-api/tree/master/examples)

## Contributions

Contributions and PR's are welcome.
