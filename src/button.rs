use macroquad::prelude::*;

#[derive(Default)]
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
    pub bg: Color,
    /// The background color of the Button when hovered
    pub hover_col: Color,
    /// The font size of the Button text
    pub text_size: f32,
    /// The text displayed on the Button
    pub text: String,
    /// The alignment of the Buttons text
    pub text_alignment: TextAlignment,
    /// The Buttons border thickness
    pub border_thickness: f32,
    /// The Buttons border color
    pub border_col: Color,
    /// The Buttons trigger mode
    pub trigger: TriggerMode,
    is_clicked: bool,
}

#[derive(Default)]
pub struct ButtonBuilder {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    text_col: Color,
    bg: Color,
    hover_col: Color,
    text_size: f32,
    border_thickness: f32,
    border_col: Color,
    text: String,
    text_alignment: TextAlignment,
    trigger: TriggerMode,
}

impl Button {
    /// Creates a Builder that can be used to build a button by giving it colors, text, position,
    /// etc.
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
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
            match self.trigger {
                TriggerMode::OnPress => return macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left),
                TriggerMode::OnRelease => return macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Left)
            }
        }
        self.is_clicked = false;
        false
    }

    pub fn is_right_clicked(&mut self) -> bool {
        if self.is_hovered() {
            match self.trigger {
                TriggerMode::OnPress => return macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Right),
                TriggerMode::OnRelease => return macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Right)
            }
        }
        self.is_clicked = false;
        false
    }

    /// This draws the button
    pub fn update(&mut self) {
        match self.is_hovered() {
            true => {
                match self.is_clicked {
                    true => {
                    },
                    false => {
                        draw_rectangle(self.x, self.y, self.w, self.h, self.hover_col);
                    }
                }
            },
            false => {
                draw_rectangle(self.x, self.y, self.w, self.h, self.bg);
            }
        }
        draw_text_ex(&self.text, self.x, self.y, TextParams {
            color: self.text_col,
            ..Default::default()
        });
        if self.border_thickness > 0.0 {
            draw_rectangle_lines(self.x, self.y, self.w, self.h, self.border_thickness, self.border_col);
        }
    }
}

impl ButtonBuilder {
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

    pub fn set_text_size(mut self, size: f32) -> Self {
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
        self.bg = col;
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
        self.trigger = mode;
        self
    }

    pub fn build(self) -> Button {
        Button {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            text_col: self.text_col,
            hover_col: self.hover_col,
            bg: self.bg,
            text: self.text,
            text_size: self.text_size,
            text_alignment: self.text_alignment,
            border_thickness: self.border_thickness,
            border_col: self.border_col,
            is_clicked: false,
            trigger: self.trigger,
        }
    }
}

#[derive(Default)]
pub enum TextAlignment {
    #[default]
    Center,
    Left,
    Right
}

#[derive(Default)]
pub enum TriggerMode {
    #[default]
    OnRelease,
    OnPress,
}
