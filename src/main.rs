#![no_std]
#![no_main]
#![feature(collections)]
#![feature(asm)]

#[macro_use]
extern crate collections;
#[macro_use]
extern crate stm32f7_discovery as stm32f7;

extern crate arrayvec;


mod semi_hosting;

// Initialization routines for .data and .bss.
extern crate r0;
use stm32f7::{system_clock, board, embedded, sdram, lcd, i2c, touch};
use embedded::interfaces::gpio::{self, Gpio};

mod forms;
mod draw;
mod util;
mod action;
mod layout;
mod move_things;

use util::bounding_box::BoundingBox;
use collections::Vec;
use collections::VecDeque;
use forms::button::Button;
use forms::label::Label;

use collections::boxed::Box;
use forms::form::Form;
use util::sizes;

use util::layout_funcs::BorderArea;

use move_things::swipe;

use stm32f7::touch::Touch;
//use stm32f7::arrayvec::ArrayVec;
use arrayvec::ArrayVec;

#[inline(never)]
fn main(hw: board::Hardware) -> ! {
    let board::Hardware {
        rcc,
        pwr,
        flash,
        fmc,
        ltdc,
        gpio_a,
        gpio_b,
        gpio_c,
        gpio_d,
        gpio_e,
        gpio_f,
        gpio_g,
        gpio_h,
        gpio_i,
        gpio_j,
        gpio_k,
        i2c_3,
        ..
    } = hw;

    let mut gpio = Gpio::new(gpio_a,
                             gpio_b,
                             gpio_c,
                             gpio_d,
                             gpio_e,
                             gpio_f,
                             gpio_g,
                             gpio_h,
                             gpio_i,
                             gpio_j,
                             gpio_k);

    system_clock::init(rcc, pwr, flash);

    // Enable all gpio ports.
    rcc.ahb1enr
        .update(|r| {
            r.set_gpioaen(true);
            r.set_gpioben(true);
            r.set_gpiocen(true);
            r.set_gpioden(true);
            r.set_gpioeen(true);
            r.set_gpiofen(true);
            r.set_gpiogen(true);
            r.set_gpiohen(true);
            r.set_gpioien(true);
            r.set_gpiojen(true);
            r.set_gpioken(true);
        });

    let led_pin = (gpio::Port::PortI, gpio::Pin::Pin1);
    let mut led = gpio.to_output(led_pin,
                                 gpio::OutputType::PushPull,
                                 gpio::OutputSpeed::Low,
                                 gpio::Resistor::NoPull)
        .expect("led pin already in use");

    led.set(true);

    // Initialize display.
    sdram::init(rcc, fmc, &mut gpio);
    let mut lcd = lcd::init(ltdc, rcc, &mut gpio);
    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear();
    layer_2.clear();

    stm32f7::init_stdout(lcd.layer_2().unwrap());

    //None::<()>.unwrap();


    // Initialize touch on display.
    i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3);
    touch::check_family_id(&mut i2c_3).unwrap();



    /*
    let mut tmp_bb = BoundingBox {
        x: 10,
        y: 10,
        width: 10,
        height: 10,
    };
    let mut tmp_bb2 = BoundingBox {
        x: 10,
        y: 10,
        width: 10,
        height: 10,
    };
    let mut tmp_bb3 = BoundingBox {
        x: 10,
        y: 10,
        width: 10,
        height: 10,
    };
    let mut tmp_bb4 = BoundingBox {
        x: 10,
        y: 10,
        width: 10,
        height: 10,
    };
    let mut tmp_bb5 = BoundingBox {
        x: 10,
        y: 10,
        width: 10,
        height: 10,
    };
    let mut tmp_bb6 = BoundingBox {
        x: 10,
        y: 10,
        width: 10,
        height: 10,
    };

    let mut bl = layout::BorderLayout::new(BoundingBox {
                                               x: 50,
                                               y: 50,
                                               width: 100,
                                               height: 100,
                                           });
    let mut button1 = Box::new(Button::new(tmp_bb, 3));
    let mut button2 = Box::new(Button::new(tmp_bb2, 3));
    let mut button3 = Box::new(Button::new(tmp_bb3, 3));
    let mut button4 = Box::new(Button::new(tmp_bb4, 3));
    let mut button5 = Box::new(Button::new(tmp_bb5, 3));
    let mut button6 = Box::new(Button::new(tmp_bb6, 3));
    bl.add_form(button1, BorderArea::Top);
    bl.add_form(button2, BorderArea::Left);
    bl.add_form(button3, BorderArea::Center);
    bl.add_form(button4, BorderArea::Right);
    bl.add_form(button5, BorderArea::Bottom);
    bl.add_form(button6, BorderArea::Top);

    bl.draw();
    */

    
    // move test
    let mut move_bb_outer = BoundingBox {
        x: 300,
        y: 300,
        width: 200,
        height: 200,
    };
    let mut move_box = layout::MoveBox::new(move_bb_outer, true);

    let mut move_bb_inner = BoundingBox {
        x: 15,
        y: 15,
        width: 50,
        height: 50,
    };
    let mut button = Box::new(Button::new(move_bb_inner, 2));
    button.set_movable(true);

    let mut move_bb_inner2 = BoundingBox {
        x: 75,
        y: 15,
        width: 50,
        height: 50,
    };
    let mut button_other = Box::new(Button::new(move_bb_inner2, 2));
    button_other.set_movable(true);

    move_box.add_form(button);
    move_box.add_form(button_other);
    move_box.draw();
    //--------------------------------
    /*
    let mut bb1 = BoundingBox {
        x: 75,
        y: 15,
        width: 50,
        height: 50,
    };
    let mut button1 = Box::new(Button::new(bb1, 2));

    let mut outer_move_box = layout::MoveBox::new(BoundingBox {
                                                      x: 0,
                                                      y: 0,
                                                      width: sizes::MAX_X,
                                                      height: sizes::MAX_Y,
                                                  },
                                                  false);
    outer_move_box.add_form(Box::new(move_box));
    let mut bb2 = BoundingBox {
        x: 75,
        y: 15,
        width: 50,
        height: 50,
    };
    let mut button2 = Box::new(Button::new(bb2, 2));

    let mut bb3 = BoundingBox {
        x: 75,
        y: 15,
        width: 50,
        height: 50,
    };
    let mut button3 = Box::new(Button::new(bb3, 2));

    /*let bb3 = BoundingBox{x:50, y:50, width:15, height:15};
    let button3 = Button::new(bb3);
    let b3 = Box::new(button3);*/

    let mut el_vec = Vec::new();
    let bb_dummy = BoundingBox{x:0, y: 0, width:480, height:272};
    let mut flow_container = layout::HorizontalLayout{
    bounding_box:BoundingBox{x:10, y:10, width:100, height:100}, elements:el_vec, movable:true, outer_bounding_box:bb_dummy};
    flow_container.add_form(button1);
    flow_container.add_form(button2);
    flow_container.add_form(button3);
    let v = vec![2, 2, 6];
    flow_container.set_proportions(v);
    flow_container.draw_area();*/

    //let color: lcd::Color = lcd::Color::from_hex(0xFFFFFF);
    //draw::fill_rectangle(30, 30, 200, 200, draw::convert_color_to_u16(color));

    let mut touch_history = move_things::swipe::TouchHistory::new();


    let mut last_led_toggle = system_clock::ticks();
    loop {
        let ticks = system_clock::ticks();

        if ticks - last_led_toggle >= 1000 {
            let led_current = led.get();
            led.set(!led_current);
            last_led_toggle = ticks;
        }

        /*for touch in &touch::touches(&mut i2c_3).unwrap() {
            action::walker::walk(&mut move_box, touch.x as i32, touch.y as i32);
        }*/

        //: &Result<ArrayVec<[Touch; 5]>, i2c::Error>
        let touches_result = touch::touches(&mut i2c_3).unwrap();
        //let mut touch_history = swipe::TouchHistory::new();

        let mut input: Vec<(i32, i32)> = Vec::new();
        for i in touches_result {
            input.push((i.x as i32, i.y as i32));
        }
        /*println!("Length of touch result: {}", &input.len());
        for i in &input {
            println!("{} - {}", i.0, i.1);
        }*/

        
        touch_history.update(ticks, input);
        //touch_history.check_for_object_moves(&mut outer_move_box);

        touch_history.check_for_object_moves(&mut move_box);
        

        //let v: VecDeque<u32> = VecDeque::new();
        // check if there was an error or get ArrayVec otherwise for an update
        //let mut touches = ArrayVec::<[_; 16]>::new();
        //let mut touches = ArrayVec::<[_;16]>::new();
        // match touchArrayVec {
        //      Ok(T) => {},
        //      Err() => {},
        // }

    }
}


fn clicked(form: &mut Button) {
    let width = form.get_border_width();
    match width {
        2 => form.set_border_width(5),
        _ => form.set_border_width(2),
    }

    form.clear();
    form.draw();
}

#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static __DATA_LOAD: u32;
        static __DATA_END: u32;
        static mut __DATA_START: u32;
        static mut __BSS_START: u32;
        static mut __BSS_END: u32;
    }

    let data_load = &__DATA_LOAD;
    let data_start = &mut __DATA_START;
    let data_end = &__DATA_END;
    let bss_start = &mut __BSS_START;
    let bss_end = &__BSS_END;

    // initializes the .data section
    //(copy the data segment initializers from flash to RAM)
    r0::init_data(data_start, data_end, data_load);
    // zeroes the .bss section
    r0::zero_bss(bss_start, bss_end);

    stm32f7::heap::init();

    // enable floating point unit
    let scb = stm32f7::cortex_m::peripheral::scb_mut();
    scb.cpacr.modify(|v| v | 0b1111 << 20);

    main(board::hw());
}
