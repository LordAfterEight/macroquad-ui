use macroquad::prelude::*;

#[derive(Default)]
pub struct Button {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub text_col: Color,
    pub bg: Color,
    pub hover_col: Color,
    pub text_size: f32,
    pub text: String,
    pub text_alignment: TextAlignment,
    pub border_thickness: f32,
    pub border_col: Color,
    is_clicked: bool,
}

#[derive(Default)]
pub struct ButtonBuilder {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub text_col: Color,
    pub bg: Color,
    pub hover_col: Color,
    pub text_size: f32,
    pub border_thickness: f32,
    pub border_col: Color,
    pub text: String,
    pub text_alignment: TextAlignment,
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
        if self.is_hovered()
            && macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
            return true;
        }
        self.is_clicked = false;
        false
    }

    pub fn is_right_clicked(&mut self) -> bool {
        if self.is_hovered()
            && macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Right) {
            return true;
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
