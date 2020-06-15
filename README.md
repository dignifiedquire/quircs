<h1 align="center">quircs</h1>
<div align="center">
 <strong>
   QR Scanner in Rust.
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/quircs">
    <img src="https://img.shields.io/crates/v/quircs.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/quircs">
    <img src="https://img.shields.io/crates/d/quircs.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/quircs">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- CI -->
  <a href="https://github.com/dignifiedquire/quircs/actions">
    <img src="https://github.com/dignifiedquire/quircs/workflows/CI/badge.svg"
      alt="CI status" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/quircs">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/dignifiedquire/quircs/releases">
      Releases
    </a>
  </h3>
</div>

<br/>

> Ported from [quirc](https://github.com/dlbeer/quirc).


## Example 

```rust
// open the image from disk
let img = image::open("tests/data/Hello+World.png").unwrap();

// convert to gray scale
let img_gray = img.into_luma();

// create a decoder
let mut decoder = quircs::Quirc::default();

// identify all qr codes
let codes = decoder.identify(img_gray.width() as usize, img_gray.height() as usize, &img_gray);

for code in codes {
    let code = code.expect("failed to extract qr code");
    let decoded = code.decode().expect("failed to decode qr code");
    println!("qrcode: {}", std::str::from_utf8(&decoded.payload).unwrap());
}
```

## CLI Example

```
$ cargo build --release --example qrtest
$ qrtest <path-to-image>
```


