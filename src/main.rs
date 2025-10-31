use macroquad::prelude::*;

mod button;
mod graphing;

fn window_conf() -> macroquad::window::Conf {
    Conf {
        window_title: "".to_string(),
        window_width: 1920 / 3,
        window_height: 1080 / 3,
        high_dpi: false,
        fullscreen: false,
        sample_count: 4,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut button = crate::button::Button::default()
        .set_position(100.0, 100.0)
        .set_size(150.0, 50.0)
        .set_text("Click Me!")
        .set_text_size(30)
        .set_text_color(Color::new(0.1, 0.1, 0.1, 1.0))
        .set_bg_color(Color::new(0.9, 0.9, 0.9, 1.0))
        .set_hover_color(Color::new(0.85, 0.85, 0.85, 1.0))
        .set_press_color(Color::new(0.6, 0.6, 0.6, 1.0))
        .set_trigger_mode(button::TriggerMode::OnRelease)
        .set_trigger_button(MouseButton::Left)
        .inside_shadow_size(4)
        .set_border(2.0, DARKGRAY);

    loop {
        clear_background(Color::new(0.2, 0.2, 0.2, 1.0));
        button.update();
        next_frame().await;
    }
}
