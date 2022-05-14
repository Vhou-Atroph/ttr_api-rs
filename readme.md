# ttr_api-rs

A library meant to make it easier to access Toontown Rewritten's APIs through Rust. For information about Toontown Rewritten's APIs, check out their documentations [at this repository](https://github.com/ToontownRewritten/api-doc).  

## installation

```toml
[dependencies]
ttr_api = "1.0.0"
```

## usage

Below is an example of using the library to check if a specific cog is invading, in this case The Mingler.

```Rust
use ttr_api::Invasions;
    fn is_cog_invading() {
        let inv = Invasions::Invasion::new(ttr_api::makeclient().unwrap()).unwrap();
        assert_eq!(inv.cog_invading("The Mingler"),true) //Change this to a cog that is currently invading.
    }
```
