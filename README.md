# URS (URL Shortener)

A simple URL shortener written in Rust.

![MIT](https://img.shields.io/github/license/egargo/urs)
[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org)

> [!NOTE]
> This is a simple URL shortener written in Rust cobbled up together in an hour.
> This project is not meant for production use.


## Dependencies

- Rust
- Actix
- Docker


## Setup

```bash
# Clone the repository with SSH
git clone git@github.com:uccp-cdo/urs.git

# or with HTTPS
git clone https://github.com/uccp-cdo/urs.git

# Go to `urs` directory
cd urs

# Build and run `urs`
docker compose up -d

# Test connection
curl http://0.0.0.0:2122/ | jq
```


## Usage

```bash
# Shorten a url
curl -sX POST 'http://0.0.0.0:2122?url=<URL_TO_SHORTEN>'

# Access shorten url
links 'http://0.0.0.0:2122/gKUQL0w'
```


## License

This project is provided under the [MIT license](./LICENSE).
