use std::path::PathBuf;
use std::time::Instant;

use quircs::*;

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
        info.file_count,
        info.id_count,
        info.decode_count,
        info.id_count - info.decode_count,
    );
    if info.id_count != 0 {
        print!(
            ", {}% success rate",
            (info.decode_count * 100 + info.id_count / 2) / info.id_count,
        );
    }
    println!();
    println!(
        "Total time [load: {}, identify: {}, total: {}]",
        info.load_time, info.identify_time, info.total_time,
    );
    if info.file_count != 0 {
        println!(
            "Average time [load: {}, identify: {}, total: {}]",
            info.load_time.wrapping_div(info.file_count as u128),
            info.identify_time.wrapping_div(info.file_count as u128),
            info.total_time.wrapping_div(info.file_count as u128),
        );
    }
}

fn add_result(sum: &mut ResultInfo, inf: &mut ResultInfo) {
    sum.file_count += inf.file_count;
    sum.id_count += inf.id_count;
    sum.decode_count += inf.decode_count;
    sum.load_time = sum.load_time.wrapping_add(inf.load_time);
    sum.identify_time = sum.identify_time.wrapping_add(inf.identify_time);
    sum.total_time = sum.total_time.wrapping_add(inf.total_time);
}

fn scan_file(decoder: &mut Quirc, opts: &Opts, path: &str, info: &mut ResultInfo) -> i32 {
    let path = PathBuf::from(path);
    let start = Instant::now();
    let total_start = start;

    let img = image::open(&path)
        .expect("failed to open image")
        .into_luma8();

    info.load_time = start.elapsed().as_millis();

    let start = Instant::now();

    let res: Vec<_> = decoder
        .identify(img.width() as usize, img.height() as usize, &img)
        .collect::<Result<_, _>>()
        .unwrap();

    info.identify_time = start.elapsed().as_millis();
    info.id_count = decoder.count();

    for code in &res {
        if code.decode().is_ok() {
            info.decode_count += 1
        }
    }

    info.total_time = total_start.elapsed().as_millis();

    println!(
        "  {:<30}  {:<5} {:<5} {:<5} {:<5} {:<5}",
        path.file_name().unwrap().to_string_lossy(),
        info.load_time,
        info.identify_time,
        info.total_time,
        info.id_count,
        info.decode_count,
    );

    if opts.cell_dump || opts.verbose {
        for code in &res {
            if opts.cell_dump {
                dump_cells(code);
                println!();
            }

            if opts.verbose {
                match code.decode() {
                    Ok(data) => {
                        println!("\n  Decode successful:");
                        dump_data(&data);
                        println!();
                    }
                    Err(err) => {
                        println!("  ERROR: {err}\n");
                    }
                }
            }
        }
    }

    info.file_count = 1;
    1
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
    println!("    Version: {}", data.version);
    println!("    ECC level: {:?}", data.ecc_level);
    println!("    Mask: {}", data.mask);
    println!("    Data type: {:?}", data.data_type);
    println!("    Length: {}", data.payload.len());
    println!("    Payload: {:?}", std::str::from_utf8(&data.payload));
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

    println!("quircs test program");
    println!("Library version: {}\n", version());

    let opts = Opts {
        verbose: true,
        cell_dump: false,
    };

    let res = run_tests(&opts, &args);
    std::process::exit(res);
}
