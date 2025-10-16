use macroquad::prelude::*;

mod button;

#[macroquad::main("")]
async fn main() {
    let mut button = crate::button::Button::builder()
        .set_text("Test")
        .set_position(100.0, 100.0)
        .set_size(100.0, 25.0)
        .set_text_color(Color::new(1.0, 1.0, 1.0, 1.0))
        .set_bg_color(Color::new(0.2, 0.2, 0.2, 1.0))
        .set_border(2.0, Color::new(1.0, 0.0, 0.0, 1.0))
        .build();

    loop {
        button.update();
        next_frame().await;
    }
}
