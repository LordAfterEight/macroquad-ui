use macroquad::prelude::*;

mod button;

#[macroquad::main("")]
async fn main() {
    let mut button = button::Button::builder()
        .set_text("Test")
        .set_position(100.0, 100.0)
        .set_size(100.0, 25.0)
        .set_text_color(Color::new(1.0, 1.0, 1.0, 1.0))
        .set_bg_color(Color::new(0.1, 0.1, 0.1, 1.0))
        .set_hover_color(Color::new(0.2, 0.2, 0.2, 1.0))
        .set_border(2.0, Color::new(0.2, 0.5, 0.6, 1.0))
        .set_trigger_mode(button::TriggerMode::OnRelease)
        .build();

    loop {
        if button.is_right_clicked() {
            std::process::exit(0);
        }
        button.update();
        next_frame().await;
    }
}
