use macroquad::prelude::*;

mod button;
mod graphing;

#[macroquad::main("")]
async fn main() {
    let mut points: Vec<graphing::Point> = Vec::new();

    for i in 0..50 {
        points.push(
            graphing::Point::with_value(i as f32)
                .set_label(&format!("TP {}", i))
                .set_coordinates(1.0 + i as f32, macroquad::rand::gen_range(10.0, 50.0))
                .set_size(2.0)
                .set_colors(
                    Color::new(1.0, 0.2, 0.2, 1.0),
                    Color::new(0.8, 0.8, 0.8, 1.0),
                ),
        );
    }

    let canvas = graphing::Canvas::new("Data")
        .set_position(10.0, 10.0)
        .set_colors(
            Color::new(0.1, 0.1, 0.1, 1.0),
            Color::new(0.8, 0.8, 0.8, 1.0),
        )
        .set_border_thickness(2.0)
        .add_axes(
            graphing::Axis::new("X", points.len() as u64, Color::new(0.4, 0.4, 0.8, 1.0)),
            graphing::Axis::new("Y", 100, Color::new(0.8, 0.4, 0.4, 1.0)),
        )
        .add_graph(
            graphing::Graph::new("A")
                .set_data(points)
                .set_look(1.0, graphing::LineType::Solid),
        )
        .set_size(750.0, 500.0);

    loop {
        canvas.draw();
        next_frame().await;
    }
}
