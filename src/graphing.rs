use macroquad::prelude::*;

#[derive(Default, Debug)]
pub struct Point {
    pub label: Option<String>,
    pub x: f32,
    pub y: f32,
    pub col1: Color,
    pub col2: Color,
    pub shape: PointShape,
    pub size: f32,
    pub value: f32,
}

impl Point {
    pub fn with_value(value: f32) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn set_coordinates(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn set_colors(mut self, inner: Color, outer: Color) -> Self {
        self.col1 = inner;
        self.col2 = outer;
        self
    }

    pub fn set_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn set_shape(mut self, shape: PointShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn set_value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }
}

#[derive(Default, Debug)]
pub struct Graph {
    pub name: String,
    pub data: Vec<Point>,
    line_thickness: f32,
    line_type: LineType,
    color: Color,
}

impl Graph {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn set_data(mut self, data: impl IntoIterator<Item = Point>) -> Self {
        self.data = data.into_iter().collect();
        self
    }

    pub fn set_look(mut self, line_thickness: f32, line_type: LineType, color: Color) -> Self {
        self.line_type = line_type;
        self.line_thickness = line_thickness;
        self.color = color;
        self
    }
}

#[derive(Default, Debug)]
pub struct Axis {
    pub name: String,
    pub range: u64,
    pub color: Color,
}

impl Axis {
    pub fn new(name: impl Into<String>, range: u64, color: Color) -> Self {
        Self {
            name: name.into(),
            range,
            color,
        }
    }
}

#[derive(Default, Debug)]
pub struct Canvas {
    pub title: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub x_axis: Axis,
    pub y_axis: Axis,
    pub graphs: Vec<Graph>,
    pub bg: Color,
    pub fg: Color,
    pub border_thickness: f32,
}

impl Canvas {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn add_graph(mut self, data: Graph) -> Self {
        self.graphs.push(data);
        self
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

    pub fn set_colors(mut self, bg: Color, fg: Color) -> Self {
        self.bg = bg;
        self.fg = fg;
        self
    }

    pub fn set_border_thickness(mut self, thickness: f32) -> Self {
        self.border_thickness = thickness;
        self
    }

    pub fn add_axes(mut self, x_axis: Axis, y_axis: Axis) -> Self {
        self.x_axis = x_axis;
        self.y_axis = y_axis;
        self
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.w, self.h, self.bg);
        draw_rectangle_lines(
            self.x,
            self.y,
            self.w,
            self.h,
            self.border_thickness,
            self.fg,
        );
        draw_text_ex(
            &self.title,
            (self.x + self.w) / 2.0,
            self.y + 16.0,
            TextParams {
                font_size: 22,
                ..Default::default()
            },
        );

        draw_line(
            self.x + 10.0,
            self.y + self.h - 20.0,
            self.x + self.w - measure_text(&self.x_axis.name, None, 20, 1.0).width - 14.0,
            self.y + self.h - 20.0,
            2.0,
            self.x_axis.color,
        );
        draw_line(
            self.x + self.w - measure_text(&self.x_axis.name, None, 20, 1.0).width - 14.0,
            self.y + self.h - 10.0,
            self.x + self.w - measure_text(&self.x_axis.name, None, 20, 1.0).width - 14.0,
            self.y + self.h - 30.0,
            2.0,
            self.x_axis.color,
        );
        draw_text_ex(
            &self.x_axis.name,
            self.x + self.w - measure_text(&self.x_axis.name, None, 20, 1.0).width - 7.0,
            self.y + self.h - 17.0,
            TextParams {
                color: self.x_axis.color,
                font_size: 20,
                ..Default::default()
            },
        );

        let mut prev_point_y = 0.0;
        let mut point1x = 0.0;
        let mut point2x = 0.0;
        let mut point1y = 0.0;
        let mut point2y = 0.0;
        let mut write_to_second = false;
        let x_scaling =
            (self.x + self.w - measure_text(&self.x_axis.name, None, 20, 1.0).width - 34.0)
                / self.x_axis.range as f32;
        let y_scaling = (self.y + self.h - 20.0) * 0.95 / self.y_axis.range as f32;

        for graph in &self.graphs {
            for point in &graph.data {
                let y_offset = if ((prev_point_y - point.y) > 5.0) || ((point.y - prev_point_y) < 5.0) { 15.0 } else { 0.0 };
                draw_text_ex(
                    &point.label.clone().unwrap_or("".to_string()),
                    self.x + 10.0 + (point.x * x_scaling) + 5.0,
                    self.y + self.h - 20.0 - (point.y * y_scaling) - y_offset,
                    TextParams {
                        color: point.col2,
                        font_size: 15,
                        ..Default::default()
                    },
                );
                if !write_to_second {
                    point1x = point.x;
                    point1y = point.y;
                } else {
                    point2x = point.x;
                    point2y = point.y;
                }
                draw_line(
                    self.x + 10.0 + (point1x * x_scaling),
                    self.y + self.h - 20.0 - (point1y * y_scaling),
                    self.x + 10.0 + (point2x * x_scaling),
                    self.y + self.h - 20.0 - (point2y * y_scaling),
                    graph.line_thickness,
                    graph.color,
                );

                match point.shape {
                    PointShape::Circle => draw_circle(
                        self.x + 10.0 + (point.x * x_scaling),
                        self.y + self.h - 20.0 - (point.y * y_scaling),
                        point.size,
                        point.col1,
                    ),
                    PointShape::Dot => {}
                    PointShape::Square => {}
                    PointShape::Triangle => {}
                }

                write_to_second = !write_to_second;
                prev_point_y = point.y;
            }
        }
    }
}

#[derive(Default, Debug)]
pub enum PointShape {
    #[default]
    Circle,
    Dot,
    Square,
    Triangle,
}

#[derive(Default, Debug)]
pub enum LineType {
    #[default]
    Solid,
    Dashed,
    Dotted,
}
