# quircs

> QR Scanner in Rust. Ported from [quirc](https://github.com/dlbeer/quirc).


## Example 

```
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


