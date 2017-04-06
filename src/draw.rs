#![allow(dead_code)]

use lcd;

static max_x: u32 = 480;
static max_y: u32 = 272;

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

pub fn draw_line(x1:u32, y1:u32, x2:u32, y2:u32, color:u16) {
    let mut x1 = x1 as i32;
    let mut x2 = x2 as i32;
    let mut y1 = y1 as i32;
    let mut y2 = y2 as i32;
    
    let draw_p = |x: i32, y: i32| {
        draw_pixel(x as u32, y as u32, color);
    };
    
    const ACURR: i32 = 100000;
    let width = x2 - x1;
    let height = y2 - y1;
    let ratio = ACURR*width/height;
    
    if ratio.abs() > ACURR {
        // one point per x step
        if x2 < x1 { 
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }
        for x in x1..x2+1 {
            let off_x = x - x1;
            let off_y = ACURR*off_x/ratio;
            draw_p(x1 + off_x, y1 + off_y);
        }
    } else {
        // one point per y step
        if y2 < y1 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }
        for y in y1..y2+1 {
            let off_y = y - y1;
            let off_x = off_y*ratio/ACURR;
            draw_p(x1 + off_x, y1 + off_y);
        }
    }
}

fn swap(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

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
    if x + width >= max_x || x >= max_x || y + height >= max_y || y >= max_y {
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

pub fn fill_rectangle(x: u32, y: u32, width: u32, height: u32, color: u16) -> bool {
    //(x, y is upper left, according to coordinate system)
    if x + width >= max_x || x >= max_x || y + height >= max_y || y >= max_y {
        return false;
    }

    for x in x..x + width + 1 {
        for y in y..y + width + 1 {
            draw_pixel(x, y, color);
        }
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
