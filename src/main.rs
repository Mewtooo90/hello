/*
By: <Your Name Here>
Date: 2026-09-22
Program Details: <Program Description Here>
*/

mod ui;
mod utils;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use macroquad::prelude::*;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "hello".to_string(),
        window_width: 1250,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: false,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}



#[macroquad::main(window_conf)]
async fn main() {
    let btn_name = TextButton::new(50.0, 600.0, 150.0, 60.0, "Name", BLUE, GREEN, 30);
        let btn_school = TextButton::new(250.0, 600.0, 150.0, 60.0, "School", BLUE, GREEN, 30);
        let btn_job = TextButton::new(450.0, 600.0, 150.0, 60.0, "Job", BLUE, GREEN, 30);
        let btn_number = TextButton::new(650.0, 600.0, 150.0, 60.0, "Bus Number", BLUE, GREEN, 30);
        let btn_cw = TextButton::new(850.0, 600.0, 150.0, 60.0, "Text", BLUE, GREEN, 30);
        let btn_exit = TextButton::new(1050.0, 600.0, 150.0, 60.0, "Exit", BLUE, GREEN, 30);
 
        let img = StillImage::new(
        "assets/images_name.png",
        100.0,  // width
        200.0,  // height
        200.0,  // x position
        60.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
       
        let mut lbl_out = Label::new("Hello\nWorld", 50.0, 50.0, 60);
lbl_out.with_colors(WHITE, Some(DARKGRAY));
lbl_out.with_fixed_size(400.0, 400.0);
        
    loop {

        clear_background(GRAY);
        draw_grid(50.0, WHITE);

        lbl_out.draw();
        
        
        
        
        
        
        
        if btn_name.click() {lbl_out.set_text("Layne");}
        if btn_school.click() {lbl_out.set_text("BHS");}
        if btn_job.click() {lbl_out.set_text("Krubms");}
        if btn_number.click() {lbl_out.set_text("867-5309");}
        if btn_cw.click() {lbl_out.set_text("Co0nnor");}
        if btn_exit.click() {
            break;
        }

        
        
        
        
        // Do something when the button is clicked
        next_frame().await;
    }
}
