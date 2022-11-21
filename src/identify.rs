#![allow(clippy::many_single_char_names)]

use std::convert::TryFrom;

use crate::error::ExtractError;
use crate::quirc::*;
use crate::version_db::*;

#[derive(Copy, Clone)]
struct Neighbour {
    pub index: i32,
    pub distance: f64,
}

#[derive(Copy, Clone)]
struct NeighbourList {
    pub n: [Neighbour; 32],
    pub count: usize,
}

struct PolygonScoreData<'a> {
    pub ref_0: Point,
    pub scores: [i32; 4],
    pub corners: &'a mut [Point; 4],
}

// ---  Linear algebra routines

fn line_intersect(p0: &Point, p1: &Point, q0: &Point, q1: &Point, r: &mut Point) -> i32 {
    /* (a, b) is perpendicular to line p */
    let a = -(p1.y - p0.y);
    let b = p1.x - p0.x;

    /* (c, d) is perpendicular to line q */
    let c = -(q1.y - q0.y);
    let d = q1.x - q0.x;

    /* e and f are dot products of the respective vectors with p and q */
    let e = a * p1.x + b * p1.y;
    let f = c * q1.x + d * q1.y;

    /* Now we need to solve:
     *     [a b] [rx]   [e]
     *     [c d] [ry] = [f]
     *
     * We do this by inverting the matrix and applying it to (e, f):
     *       [ d -b] [e]   [rx]
     * 1/det [-c  a] [f] = [ry]
     */
    let det = a * d - b * c;
    if det == 0 {
        return 0;
    }
    r.x = (d * e - b * f) / det;
    r.y = (-c * e + a * f) / det;

    1
}

fn perspective_setup(c: &mut [f64; 8], rect: &[Point; 4], w: f64, h: f64) {
    let x0 = rect[0].x as f64;
    let y0 = rect[0].y as f64;
    let x1 = rect[1].x as f64;
    let y1 = rect[1].y as f64;
    let x2 = rect[2].x as f64;
    let y2 = rect[2].y as f64;
    let x3 = rect[3].x as f64;
    let y3 = rect[3].y as f64;

    let wden = w * (x2 * y3 - x3 * y2 + (x3 - x2) * y1 + x1 * (y2 - y3));
    let hden = h * (x2 * y3 + x1 * (y2 - y3) - x3 * y2 + (x3 - x2) * y1);

    c[0] = (x1 * (x2 * y3 - x3 * y2)
        + x0 * (-x2 * y3 + x3 * y2 + (x2 - x3) * y1)
        + x1 * (x3 - x2) * y0)
        / wden;
    c[1] = -(x0 * (x2 * y3 + x1 * (y2 - y3) - x2 * y1) - x1 * x3 * y2
        + x2 * x3 * y1
        + (x1 * x3 - x2 * x3) * y0)
        / hden;
    c[2] = x0;
    c[3] = (y0 * (x1 * (y3 - y2) - x2 * y3 + x3 * y2)
        + y1 * (x2 * y3 - x3 * y2)
        + x0 * y1 * (y2 - y3))
        / wden;
    c[4] = (x0 * (y1 * y3 - y2 * y3) + x1 * y2 * y3 - x2 * y1 * y3
        + y0 * (x3 * y2 - x1 * y2 + (x2 - x3) * y1))
        / hden;
    c[5] = y0;
    c[6] = (x1 * (y3 - y2) + x0 * (y2 - y3) + (x2 - x3) * y1 + (x3 - x2) * y0) / wden;
    c[7] = (-x2 * y3 + x1 * y3 + x3 * y2 + x0 * (y1 - y2) - x3 * y1 + (x2 - x1) * y0) / hden;
}

fn perspective_map(c: &[f64; 8], u: f64, v: f64, ret: &mut Point) {
    let den = c[6] * u + c[7] * v + 1.0f64;
    let x = (c[0] * u + c[1] * v + c[2]) / den;
    let y = (c[3] * u + c[4] * v + c[5]) / den;

    ret.x = x.round() as i32;
    ret.y = y.round() as i32;
}

fn perspective_unmap(c: &[f64; 8], in_0: &Point, u: &mut f64, v: &mut f64) {
    let x = in_0.x as f64;
    let y = in_0.y as f64;

    let den = -c[0] * c[7] * y + c[1] * c[6] * y + (c[3] * c[7] - c[4] * c[6]) * x + c[0] * c[4]
        - c[1] * c[3];
    *u = -(c[1] * (y - c[5]) - c[2] * c[7] * y + (c[5] * c[7] - c[4]) * x + c[2] * c[4]) / den;
    *v = (c[0] * (y - c[5]) - c[2] * c[6] * y + (c[5] * c[6] - c[3]) * x + c[2] * c[3]) / den;
}

// --- Span-based floodfill routine

const FLOOD_FILL_MAX_DEPTH: usize = 4096;

enum UserData<'a> {
    Region(&'a mut Region),
    Polygon(&'a mut PolygonScoreData<'a>),
    None,
}

impl<'a> UserData<'a> {
    fn into_polygon(self) -> &'a mut PolygonScoreData<'a> {
        match self {
            UserData::Polygon(poly) => poly,
            _ => panic!("invalid user data"),
        }
    }
}

#[derive(Debug)]
struct ImageMut<'a> {
    pixels: &'a mut [Pixel],
    width: usize,
    height: usize,
}

impl<'a> From<&'a mut Quirc> for ImageMut<'a> {
    fn from(q: &'a mut Quirc) -> Self {
        Self {
            pixels: &mut q.pixels,
            width: q.w,
            height: q.h,
        }
    }
}

/// Flood fill algorithm. See [wikipedia](https://en.wikipedia.org/wiki/Flood_fill) for more details.
#[allow(clippy::too_many_arguments)]
fn flood_fill_seed<F>(
    image: &mut ImageMut<'_>,
    x: i32,
    y: usize,
    from: Pixel,
    to: Pixel,
    func: Option<&F>,
    user_data: &mut UserData<'_>,
    depth: usize,
) where
    F: Fn(&mut UserData<'_>, usize, i32, i32),
{
    if depth >= FLOOD_FILL_MAX_DEPTH {
        return;
    }

    let mut left = x as usize;
    let mut right = x as usize;
    let width = image.width;

    assert!(image.pixels.len() >= (y + 1) * width);

    let row = &mut image.pixels[y * width..(y + 1) * width];
    while left > 0 && row[left - 1] == from {
        left -= 1;
    }
    while right < width - 1 && row[right + 1] == from {
        right += 1;
    }

    // Fill the extent
    for val in &mut row[left..=right] {
        *val = to;
    }

    if let Some(func) = func {
        func(user_data, y, left as i32, right as i32);
    }

    // Seed new flood-fills

    if y > 0 {
        // Not the first row, so fill the previous row
        let offset = (y - 1) * width;
        for i in left..=right {
            // Safety: pixels is in range, as verified by the assert at the beginning.
            // Unfortunately this is required, as the compiler will add bounds checks that are quite measurable.
            let val = unsafe { *image.pixels.get_unchecked(offset + i) };
            if val == from {
                flood_fill_seed(image, i as i32, y - 1, from, to, func, user_data, depth + 1);
            }
        }
    }

    if y < image.height - 1 {
        // Not the last row, so fill the next row
        let offset = (y + 1) * width;
        for i in left..=right {
            // Safety: pixels is in range, as verified by the assert at the beginning.
            // Unfortunately this is required, as the compiler will add bounds checks that are quite measurable.
            let val = unsafe { *image.pixels.get_unchecked(offset + i) };
            if val == from {
                flood_fill_seed(image, i as i32, y + 1, from, to, func, user_data, depth + 1);
            }
        }
    }
}

// --- Adaptive thresholding

fn otsu(q: &Quirc, image: &[u8]) -> u8 {
    let num_pixels = q.w * q.h;

    // Calculate histogram
    let mut histogram: [u32; 256] = [0; 256];

    for value in image {
        let value = *value as usize;
        histogram[value] = histogram[value].wrapping_add(1);
    }

    // Calculate weighted sum of histogram values
    let mut sum: u32 = 0;
    for (i, val) in histogram.iter().enumerate() {
        sum = sum.wrapping_add((i as u32).wrapping_mul(*val));
    }

    // Compute threshold
    let mut sum_b: i32 = 0;
    let mut q1: i32 = 0;
    let mut max = 0_f64;
    let mut threshold = 0_u8;
    for (i, val) in histogram.iter().enumerate() {
        // Weighted background
        q1 = (q1 as u32).wrapping_add(*val) as i32 as i32;
        if q1 == 0 {
            continue;
        }
        // Weighted foreground
        let q2 = num_pixels as i32 - q1;
        if q2 == 0 {
            break;
        }
        sum_b = (sum_b as u32).wrapping_add((i as u32).wrapping_mul(*val)) as i32 as i32;
        let m1 = sum_b as f64 / q1 as f64;
        let m2 = (sum as f64 - sum_b as f64) / q2 as f64;
        let m1m2 = m1 - m2;
        let variance = m1m2 * m1m2 * q1 as f64 * q2 as f64;
        if variance >= max {
            threshold = i as u8;
            max = variance
        }
    }

    threshold
}

fn area_count(user_data: &mut UserData<'_>, _y: usize, left: i32, right: i32) {
    if let UserData::Region(ref mut region) = user_data {
        region.count += right - left + 1;
    } else {
        panic!("invalid user data");
    }
}

fn region_code(image: &mut ImageMut<'_>, regions: &mut Vec<Region>, x: i32, y: usize) -> i32 {
    if x < 0 || x >= image.width as i32 || y >= image.height {
        return -1;
    }
    let pixel = image.pixels[(y as i32 * image.width as i32 + x) as usize];
    if pixel >= 2 {
        return pixel as i32;
    }
    if pixel == 0 {
        return -1;
    }
    let region = regions.len() as i32;

    if region >= 65_534 {
        return -1;
    }

    regions.push(Region {
        seed: Point { x, y: y as i32 },
        count: 0,
        capstone: -1,
    });

    flood_fill_seed(
        image,
        x,
        y,
        pixel,
        region as Pixel,
        Some(&area_count),
        &mut UserData::Region(&mut regions[region as usize]),
        0,
    );

    region
}

fn find_one_corner(user_data: &mut UserData<'_>, y: usize, left: i32, right: i32) {
    if let UserData::Polygon(ref mut psd) = user_data {
        let xs: [i32; 2] = [left, right];
        let dy = y as i32 - psd.ref_0.y;

        for x in &xs {
            let dx = *x - (*psd).ref_0.x;
            let d = dx * dx + dy * dy;
            if d > psd.scores[0] {
                psd.scores[0] = d;
                psd.corners[0].x = *x;
                psd.corners[0].y = y as i32;
            }
        }
    } else {
        panic!("invalid user data");
    }
}

fn find_other_corners(user_data: &mut UserData<'_>, y: usize, left: i32, right: i32) {
    if let UserData::Polygon(ref mut psd) = user_data {
        let xs: [i32; 2] = [left, right];

        for x in &xs {
            let up = *x * psd.ref_0.x + y as i32 * psd.ref_0.y;
            let right_0 = *x * -psd.ref_0.y + y as i32 * psd.ref_0.x;
            let scores: [i32; 4] = [up, right_0, -up, -right_0];

            for (j, score) in scores.iter().enumerate() {
                if *score > psd.scores[j] {
                    psd.scores[j] = *score;
                    psd.corners[j].x = *x;
                    psd.corners[j].y = y as i32;
                }
            }
        }
    } else {
        panic!("invalid user data");
    }
}

fn find_region_corners(
    image: &mut ImageMut<'_>,
    region: &Region,
    rcode: Pixel,
    point: &Point,
    corners: &mut [Point; 4],
) {
    let mut psd = PolygonScoreData {
        ref_0: *point,
        scores: [-1, 0, 0, 0],
        corners,
    };
    let mut psd_ref = UserData::Polygon(&mut psd);

    flood_fill_seed(
        image,
        region.seed.x,
        region.seed.y as usize,
        rcode,
        1,
        Some(&find_one_corner),
        &mut psd_ref,
        0,
    );
    let mut psd = psd_ref.into_polygon();

    // Safe to unwrap, because the only reference was given to the call
    // to flood_fill_seed above.
    psd.ref_0.x = psd.corners[0].x - psd.ref_0.x;
    psd.ref_0.y = psd.corners[0].y - psd.ref_0.y;
    for corner in &mut psd.corners[..] {
        *corner = region.seed;
    }

    let i = region.seed.x * psd.ref_0.x + region.seed.y * psd.ref_0.y;
    psd.scores[0] = i;
    psd.scores[2] = -i;

    let i = region.seed.x * -psd.ref_0.y + region.seed.y * psd.ref_0.x;
    psd.scores[1] = i;
    psd.scores[3] = -i;

    flood_fill_seed(
        image,
        region.seed.x,
        region.seed.y as usize,
        1,
        rcode,
        Some(&find_other_corners),
        &mut UserData::Polygon(psd),
        0,
    );
}

fn record_capstone(
    image: &mut ImageMut<'_>,
    regions: &mut [Region],
    capstones: &mut Vec<Capstone>,
    ring: Pixel,
    stone: i32,
) {
    if capstones.len() >= 32 {
        return;
    }
    let cs_index = capstones.len() as i32;

    let capstone = Capstone {
        qr_grid: -1,
        ring: ring as i32,
        stone,
        ..Default::default()
    };
    capstones.push(capstone);
    let capstone = &mut capstones[cs_index as usize];

    regions[stone as usize].capstone = cs_index;
    regions[ring as usize].capstone = cs_index;

    /* Find the corners of the ring */
    let region = &regions[ring as usize];
    let seed = &regions[stone as usize].seed;
    find_region_corners(image, region, ring, seed, &mut capstone.corners);

    /* Set up the perspective transform and find the center */
    perspective_setup(&mut capstone.c, &capstone.corners, 7.0, 7.0);
    perspective_map(&capstone.c, 3.5, 3.5, &mut capstone.center);
}

fn test_capstone(
    image: &mut ImageMut<'_>,
    regions: &mut Vec<Region>,
    capstones: &mut Vec<Capstone>,
    x: i32,
    y: usize,
    pb: &[i32],
) {
    let ring_right = region_code(image, regions, x - pb[4], y);
    let stone = region_code(image, regions, x - pb[4] - pb[3] - pb[2], y);
    let ring_left = region_code(image, regions, x - pb[4] - pb[3] - pb[2] - pb[1] - pb[0], y);
    if ring_left < 0 || ring_right < 0 || stone < 0 {
        return;
    }
    /* Left and ring of ring should be connected */
    if ring_left != ring_right {
        return;
    }
    /* Ring should be disconnected from stone */
    if ring_left == stone {
        return;
    }
    let stone_reg = &regions[stone as usize];
    let ring_reg = &regions[ring_left as usize];

    /* Already detected */
    if stone_reg.capstone >= 0 || ring_reg.capstone >= 0 {
        return;
    }
    /* Ratio should ideally be 37.5 */
    let ratio = stone_reg.count * 100 / ring_reg.count;
    if !(10..=70).contains(&ratio) {
        return;
    }

    record_capstone(image, regions, capstones, ring_left as Pixel, stone);
}

fn finder_scan(
    image: &mut ImageMut<'_>,
    regions: &mut Vec<Region>,
    capstones: &mut Vec<Capstone>,
    y: usize,
) {
    static CHECK: [i32; 5] = [1, 1, 3, 1, 1];

    let offset = y * image.width;
    let mut last_color = 0;
    let mut run_length = 0;
    let mut run_count = 0;
    let mut pb = [0; 5];

    assert!(image.pixels.len() >= offset + image.width);

    for x in 0..image.width {
        // Safety: pixels is in range, as verified by the assert at the beginning.
        // Unfortunately this is required, as the compiler will add bounds checks that are quite measurable.
        let pixel = unsafe { image.pixels.get_unchecked(offset + x) };
        let color = if *pixel as i32 != 0 { 1 } else { 0 };

        if x != 0 && color != last_color {
            pb.copy_within(1.., 0);
            pb[4] = run_length;
            run_length = 0;
            run_count += 1;
            if color == 0 && run_count >= 5 {
                let mut ok = 1;
                let avg = (pb[0] + pb[1] + pb[3] + pb[4]) / 4;
                let err = avg * 3 / 4;

                for (pb, check) in pb.iter().zip(CHECK.iter()) {
                    if *pb < *check * avg - err || *pb > *check * avg + err {
                        ok = 0;
                    }
                }

                if ok != 0 {
                    test_capstone(image, regions, capstones, x as i32, y, &pb);
                }
            }
        }

        run_length += 1;
        last_color = color;
    }
}

fn find_alignment_pattern(
    image: &mut ImageMut<'_>,
    capstones: &[Capstone],
    regions: &mut Vec<Region>,
    qr: &mut Grid,
) {
    let c0 = &capstones[qr.caps[0]];
    let c2 = &capstones[qr.caps[2]];

    let mut a = Point::default();
    let mut c = Point::default();
    let mut step_size = 1;
    let mut dir = 0;
    let mut u = 0.;
    let mut v = 0.;

    /* Grab our previous estimate of the alignment pattern corner */
    let mut b = qr.align;

    /* Guess another two corners of the alignment pattern so that we
     * can estimate its size.
     */
    perspective_unmap(&c0.c, &b, &mut u, &mut v);
    perspective_map(&c0.c, u, v + 1.0f64, &mut a);
    perspective_unmap(&c2.c, &b, &mut u, &mut v);
    perspective_map(&c2.c, u + 1.0f64, v, &mut c);
    let size_estimate = ((a.x - b.x) * -(c.y - b.y) + (a.y - b.y) * (c.x - b.x)).abs();

    /* Spiral outwards from the estimate point until we find something
     * roughly the right size. Don't look too far from the estimate
     * point.
     */
    static DX_MAP: [i32; 4] = [1, 0, -1, 0];
    static DY_MAP: [i32; 4] = [0, -1, 0, 1];

    while step_size * step_size < size_estimate * 100 {
        for _ in 0..step_size {
            let code = region_code(image, regions, b.x, b.y as usize);
            if code >= 0 {
                let reg = &regions[code as usize];
                if reg.count >= size_estimate / 2 && reg.count <= size_estimate * 2 {
                    qr.align_region = Some(code as Pixel);
                    return;
                }
            }
            b.x += DX_MAP[dir as usize];
            b.y += DY_MAP[dir as usize];
        }

        dir = (dir + 1) % 4;
        if dir & 1 == 0 {
            step_size += 1
        }
    }
}

fn find_leftmost_to_line(user_data: &mut UserData<'_>, y: usize, left: i32, right: i32) {
    if let UserData::Polygon(ref mut psd) = user_data {
        let xs: [i32; 2] = [left, right];

        for x in &xs {
            let d = -psd.ref_0.y * *x + psd.ref_0.x * y as i32;
            if d < psd.scores[0] {
                psd.scores[0] = d;
                psd.corners[0].x = *x;
                psd.corners[0].y = y as i32;
            }
        }
    } else {
        panic!("invalid user data");
    }
}

/// Do a Bresenham scan from one point to another and count the number
/// of black/white transitions.
fn timing_scan(image: &Image<'_>, p0: &Point, p1: &Point) -> i32 {
    let mut n = p1.x - p0.x;
    let mut d = p1.y - p0.y;
    let mut x = p0.x;
    let mut y = p0.y;

    if p0.x < 0 || p0.y < 0 || p0.x >= image.width as i32 || p0.y >= image.height as i32 {
        return -1;
    }
    if p1.x < 0 || p1.y < 0 || p1.x >= image.width as i32 || p1.y >= image.height as i32 {
        return -1;
    }

    let is_x_dom = if n.abs() > d.abs() {
        std::mem::swap(&mut n, &mut d);
        true
    } else {
        false
    };

    let nondom_step = if n < 0 {
        n = -n;
        -1
    } else {
        1
    };

    let dom_step = if d < 0 {
        d = -d;
        -1
    } else {
        1
    };

    let mut a = 0;
    let mut run_length = 0;
    let mut count = 0;

    for _ in 0..=d {
        if y < 0 || y >= image.height as i32 || x < 0 || x >= image.width as i32 {
            break;
        }
        let pixel = image.pixels[(y * image.width as i32 + x) as usize] as i32;
        if pixel != 0 {
            if run_length >= 2 {
                count += 1;
            }
            run_length = 0;
        } else {
            run_length += 1;
        }
        a += n;
        if is_x_dom {
            x += dom_step;
        } else {
            y += dom_step;
        }
        if a >= d {
            if is_x_dom {
                y += nondom_step;
            } else {
                x += dom_step;
            }
            a -= d;
        }
    }

    count
}

/// Try the measure the timing pattern for a given QR code. This does
/// not require the global perspective to have been set up, but it
/// does require that the capstone corners have been set to their
/// canonical rotation.
///
/// For each capstone, we find a point in the middle of the ring band
/// which is nearest the centre of the code. Using these points, we do
/// a horizontal and a vertical timing scan.
fn measure_timing_pattern(qr: &mut Grid, capstones: &[Capstone], image: &Image<'_>) -> i32 {
    static US: [f64; 3] = [6.5, 6.5, 0.5];
    static VS: [f64; 3] = [0.5, 6.5, 6.5];

    for (i, (us, vs)) in US.iter().zip(VS.iter()).enumerate() {
        let cap = &capstones[qr.caps[i]];

        perspective_map(&cap.c, *us, *vs, &mut qr.tpep[i]);
    }

    qr.hscan = timing_scan(image, &qr.tpep[1], &qr.tpep[2]);
    qr.vscan = timing_scan(image, &qr.tpep[1], &qr.tpep[0]);

    let mut scan = qr.hscan;
    if qr.vscan > scan {
        scan = qr.vscan
    }

    /* If neither scan worked, we can't go any further. */
    if scan < 0 {
        return -1;
    }

    /* Choose the nearest allowable grid size */
    let size = scan * 2 + 13;
    let ver = (size - 15) / 4;
    qr.grid_size = ver * 4 + 17;

    0
}

/// Read a cell from a grid using the currently set perspective
/// transform. Returns +/- 1 for black/white, 0 for cells which are
/// out of image bounds.
fn read_cell(q: &Quirc, index: usize, x: i32, y: i32) -> i32 {
    let qr = &q.grids[index];

    let mut p = Point::default();

    perspective_map(&qr.c, x as f64 + 0.5f64, y as f64 + 0.5f64, &mut p);
    if p.y < 0 || p.y >= q.h as i32 || p.x < 0 || p.x >= q.w as i32 {
        return 0;
    }

    if q.pixels[(p.y * q.w as i32 + p.x) as usize] != 0 {
        1
    } else {
        -1
    }
}

#[derive(Debug)]
struct Image<'a> {
    pixels: &'a [Pixel],
    width: usize,
    height: usize,
}

impl<'a> From<&'a Quirc> for Image<'a> {
    fn from(q: &'a Quirc) -> Self {
        Self {
            pixels: &q.pixels,
            width: q.w,
            height: q.h,
        }
    }
}

impl<'a> From<&'a ImageMut<'a>> for Image<'a> {
    fn from(img: &'a ImageMut<'a>) -> Self {
        Self {
            pixels: img.pixels,
            width: img.width,
            height: img.height,
        }
    }
}

fn fitness_cell(qr: &Grid, image: &Image<'_>, x: i32, y: i32) -> i32 {
    static OFFSETS: [f64; 3] = [0.3, 0.5, 0.7];

    let mut score = 0;
    let mut p = Point::default();

    for v in &OFFSETS {
        for u in &OFFSETS {
            p.clear();
            perspective_map(&qr.c, x as f64 + *u, y as f64 + *v, &mut p);

            if !(p.y < 0 || p.y >= image.height as i32 || p.x < 0 || p.x >= image.width as i32) {
                if image.pixels[(p.y * image.width as i32 + p.x) as usize] != 0 {
                    score += 1;
                } else {
                    score -= 1;
                }
            }
        }
    }

    score
}

fn fitness_ring(qr: &Grid, image: &Image<'_>, cx: i32, cy: i32, radius: i32) -> i32 {
    let mut score: i32 = 0;
    for i in 0..radius * 2 {
        score += fitness_cell(qr, image, cx - radius + i, cy - radius);
        score += fitness_cell(qr, image, cx - radius, cy + radius - i);
        score += fitness_cell(qr, image, cx + radius, cy - radius + i);
        score += fitness_cell(qr, image, cx + radius - i, cy + radius);
    }

    score
}

fn fitness_apat(qr: &Grid, image: &Image<'_>, cx: i32, cy: i32) -> i32 {
    fitness_cell(qr, image, cx, cy) - fitness_ring(qr, image, cx, cy, 1)
        + fitness_ring(qr, image, cx, cy, 2)
}

fn fitness_capstone(qr: &Grid, image: &Image<'_>, mut x: i32, mut y: i32) -> i32 {
    x += 3;
    y += 3;

    fitness_cell(qr, image, x, y) + fitness_ring(qr, image, x, y, 1)
        - fitness_ring(qr, image, x, y, 2)
        + fitness_ring(qr, image, x, y, 3)
}

const MAX_ALIGNMENT: usize = 7;

/// Compute a fitness score for the currently configured perspective
/// transform, using the features we expect to find by scanning the
/// grid.
fn fitness_all(qr: &Grid, image: &Image<'_>) -> i32 {
    let version = usize::try_from((qr.grid_size - 17) / 4).expect("invalid version");
    let info = &VERSION_DB[version];
    let mut score: i32 = 0;

    /* Check the timing pattern */
    for i in 0..qr.grid_size - 14 {
        let expect = if i & 1 != 0 { 1 } else { -1 };
        score += fitness_cell(qr, image, i + 7, 6) * expect;
        score += fitness_cell(qr, image, 6, i + 7) * expect;
    }

    /* Check capstones */
    score += fitness_capstone(qr, image, 0, 0);
    score += fitness_capstone(qr, image, qr.grid_size - 7, 0);
    score += fitness_capstone(qr, image, 0, qr.grid_size - 7);
    if version > VERSION_MAX {
        return score;
    }

    /* Check alignment patterns */
    let mut ap_count = 0;
    while ap_count < MAX_ALIGNMENT && info.apat[ap_count] != 0 {
        ap_count += 1;
    }

    if ap_count == 0 {
        return score;
    }

    for x in &info.apat[1..ap_count - 1] {
        score += fitness_apat(qr, image, 6, *x);
        score += fitness_apat(qr, image, *x, 6);
    }

    for x in &info.apat[1..ap_count] {
        for y in &info.apat[1..ap_count] {
            score += fitness_apat(qr, image, *x, *y);
        }
    }

    score
}

fn jiggle_perspective(qr: &mut Grid, image: &Image<'_>) {
    let mut best = fitness_all(qr, image);
    let mut adjustments: [f64; 8] = [0.; 8];

    for (a_val, c_val) in adjustments.iter_mut().zip(qr.c.iter()) {
        *a_val = c_val * 0.02;
    }

    for _pass in 0..5 {
        for i in 0..16 {
            let j = i >> 1;
            let old = qr.c[j];
            let step = adjustments[j];
            qr.c[j] = if i & 1 != 0 { old + step } else { old - step };

            let test = fitness_all(qr, image);
            if test > best {
                best = test
            } else {
                qr.c[j] = old
            }
        }

        for val in &mut adjustments {
            *val *= 0.5;
        }
    }
}

/// Once the capstones are in place and an alignment point has been
/// chosen, we call this function to set up a grid-reading perspective
/// transform.
fn setup_qr_perspective(qr: &mut Grid, capstones: &[Capstone], image: &Image<'_>) {
    /* Set up the perspective map for reading the grid */
    let rect = [
        capstones[qr.caps[1]].corners[0],
        capstones[qr.caps[2]].corners[0],
        qr.align,
        capstones[qr.caps[0]].corners[0],
    ];

    perspective_setup(
        &mut qr.c,
        &rect,
        (qr.grid_size - 7) as f64,
        (qr.grid_size - 7) as f64,
    );

    jiggle_perspective(qr, image);
}

/// Rotate the capstone with so that corner 0 is the leftmost with respect
/// to the given reference line.
fn rotate_capstone(cap: &mut Capstone, h0: &Point, hd: &Point) {
    let mut copy: [Point; 4] = [Point::default(); 4];
    let mut best = 0;
    let mut best_score = 2147483647;

    for (j, p) in cap.corners.iter().enumerate() {
        let score = (p.x - h0.x) * -hd.y + (p.y - h0.y) * hd.x;
        if j == 0 || score < best_score {
            best = j;
            best_score = score
        }
    }

    /* Rotate the capstone */
    for (i, copy) in copy.iter_mut().enumerate() {
        *copy = cap.corners[(i + best) % 4];
    }

    cap.corners = copy;
    perspective_setup(&mut cap.c, &cap.corners, 7.0, 7.0);
}

fn record_qr_grid(
    image: &mut ImageMut<'_>,
    regions: &mut Vec<Region>,
    capstones: &mut [Capstone],
    grids: &mut Vec<Grid>,
    mut a: usize,
    b: usize,
    mut c: usize,
) {
    if grids.len() >= 8 {
        return;
    }
    /* Construct the hypotenuse line from A to C. B should be to
     * the left of this line.
     */
    let h0 = capstones[a].center;
    let mut hd = Point {
        x: capstones[c].center.x - capstones[a].center.x,
        y: capstones[c].center.y - capstones[a].center.y,
    };

    /* Make sure A-B-C is clockwise */
    if (capstones[b].center.x - h0.x) * -hd.y + (capstones[b].center.y - h0.y) * hd.x > 0 {
        std::mem::swap(&mut a, &mut c);
        hd.x = -hd.x;
        hd.y = -hd.y
    }
    /* Record the grid and its components */
    let qr_index = grids.len();

    let mut qr = Grid::default();
    qr.caps[0] = a;
    qr.caps[1] = b;
    qr.caps[2] = c;
    qr.align_region = None;
    grids.push(qr);

    let qr = &mut grids[qr_index];

    /* Rotate each capstone so that corner 0 is top-left with respect
     * to the grid.
     */
    for cap_index in &qr.caps {
        let mut cap = &mut capstones[*cap_index];
        rotate_capstone(cap, &h0, &hd);
        cap.qr_grid = qr_index as i32;
    }

    /* Check the timing pattern. This doesn't require a perspective
     * transform.
     */
    if measure_timing_pattern(qr, capstones, &Image::from(&*image)) >= 0 {
        /* Make an estimate based for the alignment pattern based on extending
         * lines from capstones A and C.
         */
        if line_intersect(
            &capstones[a as usize].corners[0],
            &capstones[a as usize].corners[1],
            &capstones[c as usize].corners[0],
            &capstones[c as usize].corners[3],
            &mut qr.align,
        ) != 0
        {
            /* On V2+ grids, we should use the alignment pattern. */
            if qr.grid_size > 21 {
                /* Try to find the actual location of the alignment pattern. */
                find_alignment_pattern(image, capstones, regions, qr);
                /* Find the point of the alignment pattern closest to the
                 * top-left of the QR grid.
                 */
                if let Some(align_region) = qr.align_region {
                    let reg = &regions[align_region as usize];

                    /* Start from some point inside the alignment pattern */
                    qr.align = reg.seed;

                    let mut corners = [
                        qr.align,
                        Point::default(),
                        Point::default(),
                        Point::default(),
                    ];
                    let mut psd = PolygonScoreData {
                        ref_0: hd,
                        scores: [0; 4],
                        corners: &mut corners,
                    };
                    psd.scores[0] = -hd.y * qr.align.x + hd.x * qr.align.y;

                    flood_fill_seed::<fn(&mut UserData<'_>, usize, i32, i32) -> ()>(
                        image,
                        reg.seed.x,
                        reg.seed.y as usize,
                        align_region,
                        1,
                        None,
                        &mut UserData::None,
                        0,
                    );
                    flood_fill_seed(
                        image,
                        reg.seed.x,
                        reg.seed.y as usize,
                        1,
                        align_region,
                        Some(&find_leftmost_to_line),
                        &mut UserData::Polygon(&mut psd),
                        0,
                    );
                    qr.align = corners[0];
                }
            }

            setup_qr_perspective(qr, capstones, &Image::from(&*image));
            return;
        }
    }

    /* We've been unable to complete setup for this grid. Undo what we've
     * recorded and pretend it never happened.
     */
    for cap_index in &qr.caps {
        capstones[*cap_index].qr_grid = -1;
    }

    grids.pop();
}

fn test_neighbours(
    image: &mut ImageMut<'_>,
    regions: &mut Vec<Region>,
    capstones: &mut [Capstone],
    grids: &mut Vec<Grid>,
    i: usize,
    hlist: &NeighbourList,
    vlist: &NeighbourList,
) {
    let mut best_score = 0.0;
    let mut best_h = -1;
    let mut best_v = -1;

    /* Test each possible grouping */
    for hn in &hlist.n[..hlist.count] {
        for vn in &vlist.n[0..vlist.count] {
            let score = (1.0 - hn.distance / vn.distance).abs();

            if score > 2.5 {
                continue;
            }

            if best_h < 0 || score < best_score {
                best_h = hn.index;
                best_v = vn.index;
                best_score = score
            }
        }
    }

    if best_h < 0 || best_v < 0 {
        return;
    }

    record_qr_grid(
        image,
        regions,
        capstones,
        grids,
        best_h as usize,
        i,
        best_v as usize,
    );
}

fn test_grouping(
    image: &mut ImageMut<'_>,
    regions: &mut Vec<Region>,
    capstones: &mut [Capstone],
    grids: &mut Vec<Grid>,
    i: usize,
) {
    let mut hlist = NeighbourList {
        n: [Neighbour {
            index: 0,
            distance: 0.,
        }; 32],
        count: 0,
    };
    let mut vlist = NeighbourList {
        n: [Neighbour {
            index: 0,
            distance: 0.,
        }; 32],
        count: 0,
    };

    if capstones[i].qr_grid >= 0 {
        return;
    }

    hlist.count = 0;
    vlist.count = 0;

    /* Look for potential neighbours by examining the relative gradients
     * from this capstone to others.
     */
    let c1c = capstones[i].c;
    for (j, c2) in capstones.iter_mut().enumerate() {
        let mut u = 0.;
        let mut v = 0.;

        if i as usize == j || c2.qr_grid >= 0 {
            continue;
        }

        perspective_unmap(&c1c, &c2.center, &mut u, &mut v);
        u = (u - 3.5).abs();
        v = (v - 3.5).abs();

        if u < 0.2 * v {
            let count = hlist.count as usize;
            hlist.count += 1;
            let n = &mut hlist.n[count];
            n.index = j as i32;
            n.distance = v;
        }

        if v < 0.2 * u {
            let count = vlist.count as usize;
            vlist.count += 1;
            let n = &mut vlist.n[count];
            n.index = j as i32;
            n.distance = u;
        }
    }

    if !(hlist.count != 0 && vlist.count != 0) {
        return;
    }

    test_neighbours(image, regions, capstones, grids, i, &hlist, &vlist);
}

fn pixels_setup(q: &mut Quirc, source: &[u8], threshold: u8) {
    let dest = &mut q.pixels;

    for (value, dest) in source.iter().zip(dest.iter_mut()) {
        *dest = if (*value as i32) < threshold as i32 {
            1
        } else {
            0
        } as Pixel;
    }
}

impl Quirc {
    /// These functions are used to process images for QR-code recognition.
    /// The locations and content of each
    /// code may be obtained using accessor functions described below.
    pub fn identify<'a>(&'a mut self, width: usize, height: usize, image: &[u8]) -> CodeIter<'a> {
        self.resize(width, height);

        assert_eq!(
            self.w * self.h,
            image.len(),
            "image must be exactly of the size width * height"
        );

        self.reset();
        self.regions.push(Default::default());
        self.regions.push(Default::default());

        let threshold = otsu(self, image);
        pixels_setup(self, image, threshold);

        let mut image = ImageMut {
            pixels: &mut self.pixels,
            width: self.w,
            height: self.h,
        };
        let regions = &mut self.regions;
        let capstones = &mut self.capstones;

        for i in 0..self.h {
            finder_scan(&mut image, regions, capstones, i);
        }

        let grids = &mut self.grids;
        for i in 0..capstones.len() {
            test_grouping(&mut image, regions, capstones, grids, i);
        }

        CodeIter {
            quirc: self,
            current: 0,
        }
    }

    /// Extract the QR-code specified by the given index.
    fn extract(&self, index: usize) -> Result<Code, ExtractError> {
        let qr = self.grids[index];
        if index > self.count() {
            return Err(ExtractError::OutOfBounds);
        }

        let mut code = Code::default();

        perspective_map(&qr.c, 0.0, 0.0, &mut code.corners[0]);
        perspective_map(&qr.c, qr.grid_size as f64, 0.0, &mut code.corners[1]);
        perspective_map(
            &qr.c,
            qr.grid_size as f64,
            qr.grid_size as f64,
            &mut code.corners[2],
        );
        perspective_map(&qr.c, 0.0, qr.grid_size as f64, &mut code.corners[3]);
        code.size = qr.grid_size;

        let mut i = 0;
        for y in 0..qr.grid_size {
            for x in 0..qr.grid_size {
                if read_cell(self, index, x, y) > 0 {
                    code.cell_bitmap[(i >> 3) as usize] =
                        (code.cell_bitmap[(i >> 3) as usize] as i32 | 1 << (i & 7)) as u8
                }
                i += 1;
            }
        }

        Ok(code)
    }
}

pub struct CodeIter<'a> {
    quirc: &'a Quirc,
    current: usize,
}

impl Iterator for CodeIter<'_> {
    type Item = Result<Code, ExtractError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.quirc.count() {
            return None;
        }

        let res = self.quirc.extract(self.current);
        self.current += 1;

        Some(res)
    }
}
