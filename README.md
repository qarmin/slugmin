# slugmin
A small library for generating [slugs][wikipedia] from unicode strings.

This is forked version of [slug-rs](https://github.com/Stebalien/slug-rs) adapted to needs of [Szyszka](https://github.com/qarmin/szyszka) app.

Documentation: https://docs.rs/slugmin

[wikipedia]: https://en.wikipedia.org/wiki/Semantic_URL#Slug

## Usage
```rust
use slugmin::slugify;

let slug = slugify("Hello world",false);
```
