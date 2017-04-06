#![allow(dead_code)]

use lcd;

static max_x: u32 = 480;
static max_y: u32 = 272;

//pub use self::color::Color;
//pub use self::init::init;

use board::ltdc::Ltdc;
use embedded::interfaces::gpio::OutputPin;
use core::ptr;

//mod init;
//mod color;

fn draw_pixel(x: u32, y: u32, color: u16) {
    assert!(x < 480);
    assert!(y < 272);

    // layer 2
    let addr: u32 = 0xC000_0000 + (480 * 272 * 2);
    let pixel = u32::from(y) * 480 + u32::from(x);
    let pixel_color = (addr + pixel * 2) as *mut u16;

    unsafe { ptr::write_volatile(pixel_color, color) };
}

fn draw_line(x1: u32, y1: u32, x2: u32, y2: u32, color: u16) {}

// fn draw_rectangle_not_so_pretty(x1:u32, y1:u32, x2:u32, y2:u32, color:u16) {

//     if x1 < x2 {
//         let x_small = x1;
//         let x_big = x2;
//     } else {
//         let x_small = x2;
//         let x_big = x1;
//     }
//     if y1 < y2 {
//         let y_small = y1;
//         let y_big = y2;
//     } else {
//         let y_small = y2;
//         let y_big = y1;
//     }

//     for i in x_small..x_big {
//         draw_pixel(i, y_small, color);
//         draw_pixel(i, y_big, color);
//     }
//     for i in y_small..y_big {
//         draw_pixel(x_small, i, color);
//         draw_pixel(x_big, i, color);
//     }
// }

pub fn draw_rectangle(x: u32, y: u32, width: u32, height: u32, color: u16) -> bool {
    //(x, y is upper left, according to coordinate system)
    if x + width > max_x || x > max_x || y + height > max_y || y > max_y {
        return false;
    }

    for i in x..x + width + 1 {
        draw_pixel(i, y, color);
        draw_pixel(i, y + height, color);
    }
    for i in y..y + height + 1 {
        draw_pixel(x, i, color);
        draw_pixel(x + width, i, color);
    }
    true
}

pub fn convert_color_to_u16(color: lcd::Color) -> u16 {
    let alpha: u16 = (color.alpha as u16 / 255) << 15;
    let red: u16 = (color.red as u16 / 8) << 10;
    let green: u16 = (color.green as u16 / 8) << 5;
    let blue: u16 = color.blue as u16 / 8;

    alpha | red | green | blue
}
