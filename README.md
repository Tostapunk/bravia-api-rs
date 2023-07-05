<img src="https://github-production-user-asset-6210df.s3.amazonaws.com/25140297/249759823-97931ae4-d01f-4085-9d59-afc117cb320a.png" alt="logo" width="100"/>

# bravia-api-rs
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

bravia.service_name().api_name().await?
```
where:
* "ADDRESS" is the address of your server
* "PASSWORD" is optional and only needed when the authentication level is not "None"

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.