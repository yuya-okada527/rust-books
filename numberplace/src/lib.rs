mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const N: usize = 81;

fn is_valid(result: [u8; N], p: usize, v: u8) -> bool {
    let y = p / 9;
    let x = p % 9;
    for i in 0..9 {
        if result[9 * i + x] == v || result[9 * y + i] == v {
            return false;
        }
    }
    let block_y = y / 3;
    let block_x = x / 3;
    for i in 0..3 {
        for j in 0..3 {
            if result[9 * (3 * block_y + i) + (3 * block_x + j)] == v {
                return false;
            }
        }
    }
    return true;
}

#[wasm_bindgen]
pub fn solve(problem: Vec<u8>) -> Vec<u8> {
    solve_inner(problem)
}

fn solve_inner(problem: Vec<u8>) -> Vec<u8> {
    // TODO
}
