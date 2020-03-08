mod utils;

use std::{cmp};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn render() {
    let canvas = web_sys::window().unwrap()
        .document().unwrap()
        .get_element_by_id("app").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;
    
    canvas.set_height(WIDTH as u32);
    canvas.set_width(HEIGHT as u32);

    let context = canvas
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    let mut pixels: [u8; WIDTH * HEIGHT * 4] = [0 as u8; WIDTH * HEIGHT * 4];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let base: usize = ((y as usize) * (WIDTH as usize) + (x as usize)) * 4;
            pixels[base + 0] = (y + 128) as u8;
            pixels[base + 1] = (x - 128) as u8;
            pixels[base + 2] = cmp::max(255 - x - y, 0) as u8;
            pixels[base + 3] = 255 as u8;
        }
    }

    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(&mut pixels[..]),
        WIDTH as u32,
        HEIGHT as u32
    ).unwrap()    
        .dyn_into::<web_sys::ImageData>().unwrap();

    context.put_image_data(&image_data, 0.0, 0.0);
}
