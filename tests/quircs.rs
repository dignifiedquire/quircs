use quircs::Quirc;

#[test]
fn empty_jpeg() {
    empty("jpeg");
}

#[test]
fn empty_png() {
    empty("png");
}

fn empty(ext: &str) {
    let mut q = Quirc::default();

    let empty_image = image::open(&format!("./tests/data/1x1.{}", ext))
        .unwrap()
        .into_luma();

    q.identify(
        empty_image.width() as usize,
        empty_image.height() as usize,
        &empty_image,
    );
    assert_eq!(q.count(), 0);
}

#[test]
fn two_qr_codes_small_jpeg() {
    two_qr_codes_small("jpeg");
}

#[test]
fn two_qr_codes_small_png() {
    two_qr_codes_small("png");
}

fn two_qr_codes_small(ext: &str) {
    let mut q = Quirc::default();
    let image = image::open(&format!("./tests/data/Hello+World.{}", ext))
        .unwrap()
        .into_luma();

    let res: Vec<_> = q
        .identify(image.width() as usize, image.height() as usize, &image)
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(res.len(), 2);

    let data = res[0].decode().unwrap();
    assert_eq!(data.version, 1);
    assert_eq!(data.ecc_level, quircs::EccLevel::H);
    assert_eq!(data.mask, 1);
    assert_eq!(data.data_type, Some(quircs::DataType::Byte));
    assert_eq!(data.eci, Some(quircs::Eci::Utf8));
    assert_eq!(data.payload, b"Hello");

    let data = res[1].decode().unwrap();
    assert_eq!(data.version, 1);
    assert_eq!(data.ecc_level, quircs::EccLevel::H);
    assert_eq!(data.mask, 3);
    assert_eq!(data.data_type, Some(quircs::DataType::Byte));
    assert_eq!(data.eci, Some(quircs::Eci::Utf8));
    assert_eq!(data.payload, b"World");
}

#[test]
fn two_qr_codes_large_jpeg() {
    two_qr_codes_large("jpeg");
}

#[test]
fn two_qr_codes_large_png() {
    two_qr_codes_large("png");
}

fn two_qr_codes_large(ext: &str) {
    let mut q = Quirc::default();
    let image = image::open(&format!("./tests/data/big_image_with_two_qrcodes.{}", ext))
        .unwrap()
        .into_luma();

    let res: Vec<_> = q
        .identify(image.width() as usize, image.height() as usize, &image)
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(res.len(), 2);

    let data = res[0].decode().unwrap();
    assert_eq!(data.version, 4);
    assert_eq!(data.ecc_level, quircs::EccLevel::M);
    assert_eq!(data.mask, 2);
    assert_eq!(data.data_type, Some(quircs::DataType::Byte));
    assert_eq!(data.eci, None);
    assert_eq!(data.payload, b"from javascript");

    let data = res[1].decode().unwrap();
    assert_eq!(data.version, 4);
    assert_eq!(data.ecc_level, quircs::EccLevel::M);
    assert_eq!(data.mask, 2);
    assert_eq!(data.data_type, Some(quircs::DataType::Byte));
    assert_eq!(data.eci, None);
    assert_eq!(data.payload, b"here comes qr!");
}

#[test]
fn generated_png() {
    use quircs::{DataType, EccLevel};
    use std::collections::HashMap;
    use std::path::PathBuf;

    let mut q = Quirc::default();

    let mut mode_to_data: HashMap<DataType, &'static [u8]> = HashMap::new();
    mode_to_data.insert(DataType::Numeric, b"42");
    mode_to_data.insert(DataType::Alpha, b"AC-42");
    mode_to_data.insert(DataType::Byte, b"aA1234");
    mode_to_data.insert(DataType::Kanji, &[0x93, 0x5f, 0xe4, 0xaa]); // 点茗 in Shift-JIS

    for version in quircs::VERSION_MIN..=quircs::VERSION_MAX {
        for ecc_level in &[EccLevel::M, EccLevel::L, EccLevel::H, EccLevel::Q] {
            for mode in &[
                DataType::Numeric,
                DataType::Alpha,
                DataType::Byte,
                DataType::Kanji,
            ] {
                let filename = PathBuf::from(format!(
                    "./tests/data/generated/version={:2},level={:?},mode={}.png",
                    version,
                    ecc_level,
                    mode.to_string().to_uppercase()
                ));

                println!("-- parsing {}", filename.display());

                if !filename.exists() {
                    println!("  skipping: missing file");
                    continue;
                }

                // Known failures, same on node-quirc
                if version == 23 && *ecc_level == EccLevel::Q && *mode == DataType::Numeric
                    || version == 23 && *ecc_level == EccLevel::Q && *mode == DataType::Numeric
                    || version == 23 && *ecc_level == EccLevel::Q && *mode == DataType::Kanji
                    || version == 34 && *ecc_level == EccLevel::L && *mode == DataType::Alpha
                    || version == 34 && *ecc_level == EccLevel::L && *mode == DataType::Byte
                    || version == 36 && *ecc_level == EccLevel::M && *mode == DataType::Alpha
                    || version == 36 && *ecc_level == EccLevel::M && *mode == DataType::Byte
                {
                    println!("  skipping: known failure");
                    continue;
                }

                let image = image::open(&filename)
                    .expect("failed to open image")
                    .into_luma();

                let res: Vec<_> = q
                    .identify(image.width() as usize, image.height() as usize, &image)
                    .collect::<Result<_, _>>()
                    .unwrap();
                assert_eq!(res.len(), 1);

                let data = res[0].decode().expect("failed to decode");
                assert_eq!(data.version, version);
                assert_eq!(data.ecc_level, *ecc_level);
                assert_eq!(data.data_type, Some(*mode));
                assert_eq!(
                    &data.payload,
                    mode_to_data.get(mode).expect("missing data for mode")
                );
            }
        }
    }
}
