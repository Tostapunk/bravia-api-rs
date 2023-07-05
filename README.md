<img src="https://github-production-user-asset-6210df.s3.amazonaws.com/25140297/249759823-97931ae4-d01f-4085-9d59-afc117cb320a.png" alt="logo" width="100"/>

# bravia-api-rs
[<img alt="github" src="https://img.shields.io/badge/github-tostapunk/bravia--api--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20"/>](https://github.com/Tostapunk/bravia-api-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bravia_api?style=for-the-badge&logo=rust&color=fc8d62" height="20"/>](https://crates.io/crates/bravia_api)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bravia__api-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20"/>](https://docs.rs/bravia_api)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Tostapunk/bravia-api-rs/ci.yml?style=for-the-badge" height="20"/>](https://github.com/Tostapunk/bravia-api-rs/actions?query=branch%3Amain)\
Rust wrapper for Sony Bravia APIs.\
This project is **unofficial** and not related in any way to Sony.

## Usage
Add the following to your Cargo.toml:
```
[dependencies]
bravia_api = "0.1"
```
Then you can use it like this:
```
let bravia = Bravia::new("ADDRESS", Some("PASSWORD")).await?;

bravia.service_name().api_name().await?;
```
where:
* "ADDRESS" is the address of your server
* "PASSWORD" is optional and only needed when the authentication level is not "None"

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.