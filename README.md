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

## Start a Rocket-based web application using Tera template

* Template
* 404 pages

```
rocket-starter --tera2 hello
cd hello
cargo test
cargo run
```

## Start a Rocket-based web application using Tera template

* Template
* 404 pages
* Rocket.toml file with custom configuation

```
rocket-starter --tera1 hello
cd hello
cargo test
cargo run
```



## Release


* Update the version number in Cargo.toml to 0.3.0

```
git commit
```

```
cargo publish
```

```
git tag -a 0.3.0 -m 0.3.0
git push --tags
```
