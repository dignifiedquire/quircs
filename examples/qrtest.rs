use quircs::*;
use std::path::PathBuf;

#[derive(Debug, Clone)]
struct ResultInfo {
    file_count: usize,
    id_count: usize,
    decode_count: usize,
    load_time: u128,
    identify_time: u128,
    total_time: u128,
}

#[derive(Default)]
struct Opts {
    verbose: bool,
    cell_dump: bool,
}

fn print_result(name: &str, info: &mut ResultInfo) {
    print!("-------------------------------------------------------------------------------");
    print!(
        "{}: {} files, {} codes, {} decoded ({} failures)",
        name,
        (*info).file_count,
        (*info).id_count,
        (*info).decode_count,
        (*info).id_count - (*info).decode_count,
    );
    if (*info).id_count != 0 {
        print!(
            ", {}% success rate",
            ((*info).decode_count * 100 + (*info).id_count / 2) / (*info).id_count,
        );
    }
    println!();
    println!(
        "Total time [load: {}, identify: {}, total: {}]",
        (*info).load_time,
        (*info).identify_time,
        (*info).total_time,
    );
    if (*info).file_count != 0 {
        println!(
            "Average time [load: {}, identify: {}, total: {}]",
            (*info).load_time.wrapping_div((*info).file_count as u128),
            (*info)
                .identify_time
                .wrapping_div((*info).file_count as u128),
            (*info).total_time.wrapping_div((*info).file_count as u128),
        );
    }
}

fn add_result(mut sum: &mut ResultInfo, inf: &mut ResultInfo) {
    (*sum).file_count += (*inf).file_count;
    (*sum).id_count += (*inf).id_count;
    (*sum).decode_count += (*inf).decode_count;
    (*sum).load_time = (*sum).load_time.wrapping_add((*inf).load_time);
    (*sum).identify_time = (*sum).identify_time.wrapping_add((*inf).identify_time);
    (*sum).total_time = (*sum).total_time.wrapping_add((*inf).total_time);
}

fn load_jpeg(_dec: &mut Quirc, _path: &PathBuf) -> i32 {
    todo!()
}

fn load_png(dec: &mut Quirc, path: &PathBuf) -> i32 {
    let img = image::open(&path)
        .expect("failed to open image")
        .into_luma();
    let width = img.width() as usize;
    let height = img.height() as usize;

    dec.resize(width, height);

    let image_ptr = dec.begin(&mut 0, &mut 0);
    // copy image to the ptr
    for (x, y, px) in img.enumerate_pixels() {
        unsafe {
            *image_ptr.add(y as usize * width as usize + x as usize) = px[0];
        }
    }

    0
}

fn scan_file(decoder: &mut Quirc, opts: &Opts, path: &str, mut info: &mut ResultInfo) -> i32 {
    let path = std::path::PathBuf::from(path);

    use std::time::Instant;

    let start = Instant::now();
    let total_start = start;

    let ret = if path.extension().unwrap() == "jpg" || path.extension().unwrap() == "jpeg" {
        load_jpeg(decoder, &path)
    } else if path.extension().unwrap() == "png" {
        load_png(decoder, &path)
    } else {
        panic!("unsupported extension: {:?}", path.extension());
    };

    (*info).load_time = start.elapsed().as_millis();

    if ret < 0 {
        panic!("{}: load failed", path.display());
    }

    let start = Instant::now();
    decoder.end();
    (*info).identify_time = start.elapsed().as_millis();
    (*info).id_count = decoder.count();

    for i in 0..(*info).id_count as usize {
        let mut code: Code = Code {
            corners: [Point { x: 0, y: 0 }; 4],
            size: 0,
            cell_bitmap: [0; 3917],
        };
        let mut data = Data::default();
        decoder.extract(i, &mut code);
        if quirc_decode(&mut code, &mut data).is_ok() {
            (*info).decode_count += 1
        }
    }

    (*info).total_time = total_start.elapsed().as_millis();

    println!(
        "  {:<30}  {:<5} {:<5} {:<5} {:<5} {:<5}",
        path.file_name().unwrap().to_string_lossy(),
        (*info).load_time,
        (*info).identify_time,
        (*info).total_time,
        (*info).id_count,
        (*info).decode_count,
    );

    if opts.cell_dump || opts.verbose {
        for i in 0..(*info).id_count {
            let mut code_0 = Code {
                corners: [Point { x: 0, y: 0 }; 4],
                size: 0,
                cell_bitmap: [0; 3917],
            };
            decoder.extract(i, &mut code_0);
            if opts.cell_dump {
                dump_cells(&mut code_0);
                println!();
            }

            if opts.verbose {
                let mut data_0 = Data::default();
                if let Err(err) = quirc_decode(&mut code_0, &mut data_0) {
                    println!("  ERROR: {}\n", err);
                } else {
                    println!("\n  Decode successful:");
                    dump_data(&mut data_0);
                    println!();
                }
            }
        }
    }

    (*info).file_count = 1;
    return 1;
}

fn test_scan(decoder: &mut Quirc, opts: &Opts, path: &str, info: &mut ResultInfo) -> i32 {
    scan_file(decoder, opts, path, info)
}

fn run_tests(opts: &Opts, paths: &[String]) -> i32 {
    let mut sum = ResultInfo {
        file_count: 0,
        id_count: 0,
        decode_count: 0,
        load_time: 0,
        identify_time: 0,
        total_time: 0,
    };
    let mut count: i32 = 0;
    let mut decoder = Quirc::new();

    println!("  {:30}  {:^17} {:^11}", "", "Time (ms)", "Count");
    println!(
        "  {:30}  {:5} {:5} {:5} {:5} {:5}",
        "Filename", "Load", "ID", "Total", "ID", "Dec",
    );
    println!("-------------------------------------------------------------------------------");

    for path in paths {
        let mut info: ResultInfo = ResultInfo {
            file_count: 0,
            id_count: 0,
            decode_count: 0,
            load_time: 0,
            identify_time: 0,
            total_time: 0,
        };
        if test_scan(&mut decoder, opts, path, &mut info) > 0 {
            add_result(&mut sum, &mut info);
            count += 1
        }
    }
    if count > 1 {
        print_result("TOTAL", &mut sum);
    }

    0
}

fn dump_data(data: &Data) {
    let data = *data;

    println!("    Version: {}", data.version);
    println!("    ECC level: {:?}", data.ecc_level);
    println!("    Mask: {}", data.mask);
    println!("    Data type: {:?}", data.data_type,);
    println!("    Length: {}", data.payload_len);
    println!(
        "    Payload: {:?}",
        std::str::from_utf8(&data.payload[..data.payload_len as usize])
    );
    println!("    ECI: {:?}", data.eci);
}

fn dump_cells(code: &Code) {
    let code = *code;

    print!("    {} cells, corners:", code.size);
    for u in 0..4 {
        print!(" ({},{})", code.corners[u].x, code.corners[u].y);
    }
    println!();

    for v in 0..code.size {
        print!("    ");
        for u in 0..code.size {
            let p = v * code.size + u;

            if (code.cell_bitmap[(p >> 3) as usize] & (1 << (p & 7))) != 0 {
                print!("[]");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    println!("quirc test program");
    println!("Library version: {}\n", version());

    let opts = Opts {
        verbose: true,
        cell_dump: false,
    };

    let res = run_tests(&opts, &args);
    std::process::exit(res);
}
