#![allow(dead_code)]

use board::ltdc::Ltdc;
use core::ptr;
use embedded::interfaces::gpio::OutputPin;
use stm32f7::lcd;
use stm32f7::lcd::Color;
use stm32f7::lcd::TextWriter;

use util::sizes;

fn draw_pixel(x: i32, y: i32, color: Color) -> bool {
    if x < 0 || y < 0 || x > sizes::MAX_X || y > sizes::MAX_Y {
        return false;
    }

    let addr: u32 = 0xC000_0000;
    let pixel = y as u32 * 480 + x as u32;
    let pixel_color = (addr + pixel * 2) as *mut u16;

    unsafe { ptr::write_volatile(pixel_color, color.to_argb4444()) };

    true
}

pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
    let mut x1: i32 = x1;
    let mut x2: i32 = x2;
    let mut y1: i32 = y1;
    let mut y2: i32 = y2;

    const ACURR: i32 = 100000;
    let width = x2 - x1;
    let height = y2 - y1;
    let ratio = ACURR * width / height;

    if ratio.abs() > ACURR {
        // one point per x step
        if x2 < x1 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }
        for x in x1..x2 + 1 {
            let off_x = x - x1;
            let off_y = ACURR * off_x / ratio;
            draw_pixel(x1 + off_x, y1 + off_y, color);
        }
    } else {
        // one point per y step
        if y2 < y1 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }
        for y in y1..y2 + 1 {
            let off_y = y - y1;
            let off_x = off_y * ratio / ACURR;
            draw_pixel(x1 + off_x, y1 + off_y, color);
        }
    }
}

fn swap(a: &mut i32, b: &mut i32) {
    let c = *a;
    *a = *b;
    *b = c;
}

pub fn draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) -> bool {
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

pub fn fill_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) -> bool {
    for x in x..x + width + 1 {
        for y in y..y + width + 1 {
            draw_pixel(x, y, color);
        }
    }

    true
}
