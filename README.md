# Rocket starter

Start building a [Rocket](https://rocket.rs/)-based web application.

## Install

```
cargo install rocket-starter

```


## Start a simple Rocket-based application

```
rocket-starter --simple hello
cd hello
cargo test
cargo run
```



## Release


* Update the version number in Cargo.toml to 0.2.0

```
git commit
```

```
cargo publish
```

```
git tag -a 0.2.0 -m 0.2.0
git push --tags
```