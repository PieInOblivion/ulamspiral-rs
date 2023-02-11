use bitvec::prelude::*;
use minifb::{Key, Window, WindowOptions};
use std::cmp;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut window = Window::new(
        "Ulam Spiral",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| panic!("{e}"));

    window.limit_update_rate(Some(std::time::Duration::from_millis(8)));

    let mut primesvec: BitVec<u8> = bitvec![u8, Lsb0; 1; WIDTH*HEIGHT];

    let mut lastsize: (usize, usize) = (0, 0);
    let mut windowbuf: Vec<u32> = vec_to_spiral_layout(&primesvec, WIDTH, HEIGHT);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let size = window.get_size();

        if lastsize.0 != size.0 || lastsize.1 != size.1 {
            lastsize = size;

            let largestsidesqrd = cmp::max(size.1, size.0).pow(2);

            if largestsidesqrd > primesvec.len() {
                primesvec = sieve_gen(largestsidesqrd);
            }
            windowbuf = vec_to_spiral_layout(&primesvec, size.0, size.1);
        }

        window
            .update_with_buffer(&windowbuf, size.0, size.1)
            .unwrap();
    }
}

fn sieve_gen(max: usize) -> BitVec<u8> {
    let mut prime_buffer = bitvec![u8, Lsb0; 1; max];

    prime_buffer.set(0, false);
    prime_buffer.set(1, false);

    for p in 2..max / 2 + 1 {
        if prime_buffer[p] {
            let mut multi = p * p;
            while multi < max {
                prime_buffer.set(multi, false);
                multi += p;
            }
        }
    }

    return prime_buffer;
}

fn x_y_to_vec_index(x: usize, y: usize, w: usize) -> usize {
    return (y * w) + x;
}

fn vec_index_to_x_y(index: usize, w: usize) -> (usize, usize) {
    let x: usize = index % w;
    let y: usize = index / w;
    return (x, y);
}

fn vec_to_spiral_layout(primes: &BitVec<u8>, w: usize, h: usize) -> Vec<u32> {
    let mut spiralvec: Vec<u32> = vec![0; w * h];

    let rel_coords = spiral_relative_coords(w as isize, h as isize);

    let mut lastx: isize = 0;
    let mut lasty: isize = 0;
    let mut skip: usize = 0;
    let mut skips: usize = 0;

    for i in 0..rel_coords.len() {
        let curx: isize = (w as isize / 2) + rel_coords[i].0 - 1;
        let cury: isize = (h as isize / 2) + rel_coords[i].1 - 1;

        let curi: usize = x_y_to_vec_index(curx as usize, cury as usize, w);

        let xy = vec_index_to_x_y(curi, w);
        let gap: isize = xy.0 as isize + xy.1 as isize - lastx - lasty;

        lastx = xy.0 as isize;
        lasty = xy.1 as isize;

        if gap.abs() > 1 && i != 0 {
            skips += 1;
            if skips % 2 == 0 {
                skip -= 1;
            }
            skip += gap.abs() as usize + skips;
        }

        spiralvec[curi] = if primes[i + skip] { 0xFFFFFF } else { 0 };
    }

    return spiralvec;
}

fn spiral_relative_coords(x: isize, y: isize) -> Vec<(isize, isize)> {
    let mut relative_coords: Vec<(isize, isize)> = Vec::new();

    let mut x2: isize = 0;
    let mut y2: isize = 0;
    let mut dx: isize = 0;
    let mut dy: isize = -1;

    for _ in 0..cmp::max(x, y).pow(2) {
        if (-x / 2 < x2 && x2 <= x / 2) && (-y / 2 < y2 && y2 <= y / 2) {
            relative_coords.push((x2, y2));
        }
        if x2 == y2 || (x2 < 0 && x2 == -y2) || (x2 > 0 && x2 == 1 - y2) {
            (dx, dy) = (-dy, dx);
        }
        (x2, y2) = (x2 + dx, y2 + dy);
    }

    return relative_coords;
}
