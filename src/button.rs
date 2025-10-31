use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Button {
    /// The Buttons x coordinate in pixels
    pub x: f32,
    /// The Buttons y coordinate in pixels
    pub y: f32,
    /// The Buttons width in pixels
    pub w: f32,
    /// The Buttons height in pixels
    pub h: f32,
    /// The color of the Buttons text
    pub text_col: Color,
    /// The normal background color of the Button
    pub bg_col: Color,
    /// The background color of the Button when hovered
    pub hover_col: Color,
    /// The background color of the Button when pressed
    pub press_col: Color,
    /// The text displayed on the Button
    pub text: String,
    /// The size of the Button text
    pub text_size: u16,
    /// The alignment of the Buttons text
    pub text_alignment: TextAlignment,
    /// The Buttons border thickness
    pub border_thickness: f32,
    /// The Buttons border color
    pub border_col: Color,
    /// The Buttons trigger mode
    pub trigger_mode: TriggerMode,
    /// The mouse button that triggers a Button
    pub trigger_button: MouseButtons,
    is_clicked: bool,
    /// Size of the inside shadow. A value of 0 disables it
    pub inside_shadow_size: u8,
}

#[derive(Default, Debug)]
pub enum TriggerButton {
    #[default]
    LMB,
    RMB,
    MMB,
}

impl Button {
    /// Creates a Builder that can be used to build a button by giving it colors, text, position,
    /// etc.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_hovered(&self) -> bool {
        let (x, y) = macroquad::input::mouse_position();
        if (x >= self.x && x <= self.w + self.x) && (y >= self.y && y <= self.h + self.y) {
            return true;
        }
        false
    }

    pub fn is_left_clicked(&mut self) -> bool {
        if self.is_hovered() {
            match self.trigger_mode {
                TriggerMode::OnPress => {
                    return macroquad::input::is_mouse_button_down(
                        macroquad::input::MouseButton::Left,
                    );
                }
                TriggerMode::OnRelease => {
                    return macroquad::input::is_mouse_button_released(
                        macroquad::input::MouseButton::Left,
                    );
                }
            }
        }
        self.is_clicked = false;
        false
    }

    pub fn is_right_clicked(&mut self) -> bool {
        if self.is_hovered() {
            match self.trigger_mode {
                TriggerMode::OnPress => {
                    return macroquad::input::is_mouse_button_down(
                        macroquad::input::MouseButton::Right,
                    );
                }
                TriggerMode::OnRelease => {
                    return macroquad::input::is_mouse_button_released(
                        macroquad::input::MouseButton::Right,
                    );
                }
            }
        }
        self.is_clicked = false;
        false
    }

    pub fn set_position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_size(mut self, w: f32, h: f32) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn set_text_size(mut self, size: u16) -> Self {
        self.text_size = size;
        self
    }

    pub fn set_text_color(mut self, col: Color) -> Self {
        self.text_col = col;
        self
    }

    pub fn set_hover_color(mut self, col: Color) -> Self {
        self.hover_col = col;
        self
    }

    pub fn set_bg_color(mut self, col: Color) -> Self {
        self.bg_col = col;
        self
    }

    pub fn set_press_color(mut self, col: Color) -> Self {
        self.press_col = col;
        self
    }

    pub fn set_border(mut self, thickness: f32, col: Color) -> Self {
        self.border_thickness = thickness;
        self.border_col = col;
        self
    }

    pub fn set_text_alignment(mut self, align: TextAlignment) -> Self {
        self.text_alignment = align;
        self
    }

    pub fn set_trigger_mode(mut self, mode: TriggerMode) -> Self {
        self.trigger_mode = mode;
        self
    }

    pub fn set_trigger_button(mut self, button: MouseButton) -> Self {
        self.trigger_button = MouseButtons(button);
        self
    }

    pub fn inside_shadow_size(mut self, size: u8) -> Self {
        self.inside_shadow_size = size;
        self
    }

    /// This draws the button
    pub fn update(&mut self) {
        match self.is_hovered() {
            true => match is_mouse_button_down(self.trigger_button.0) {
                true => {
                    draw_rectangle(self.x, self.y, self.w, self.h, self.press_col);
                }
                false => {
                    draw_rectangle(self.x, self.y, self.w, self.h, self.hover_col);
                }
            },
            false => {
                draw_rectangle(self.x, self.y, self.w, self.h, self.bg_col);
            }
        }
        if self.inside_shadow_size > 0 {
            for i in 0..self.inside_shadow_size {
                draw_rectangle_lines(
                    self.x + i as f32,
                    self.y + i as f32,
                    self.w - (i * 2) as f32,
                    self.h - (i * 2) as f32,
                    1.0,
                    Color::new(0.1, 0.1, 0.1, 1.0 - (i as f32 / self.inside_shadow_size as f32)),
                );
            }
        }
        draw_text_ex(
            &self.text,
            self.x + 4.0,
            self.y + (measure_text(&self.text, None, self.text_size, 1.0).height + self.h) / 2.0,
            TextParams {
                font_size: self.text_size,
                color: self.text_col,
                ..Default::default()
            },
        );
        if self.border_thickness > 0.0 {
            draw_rectangle_lines(
                self.x,
                self.y,
                self.w,
                self.h,
                self.border_thickness,
                self.border_col,
            );
        }
    }
}

#[derive(Default, Debug)]
pub enum TextAlignment {
    #[default]
    Center,
    Left,
    Right,
}

#[derive(Default, Debug)]
pub enum TriggerMode {
    #[default]
    OnRelease,
    OnPress,
}

#[derive(Debug)]
pub struct MouseButtons(pub MouseButton);

impl Default for MouseButtons {
    fn default() -> Self {
        Self(MouseButton::Left)
    }
}
