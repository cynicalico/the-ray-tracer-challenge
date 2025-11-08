use trtc;

fn main() {
    let mut canvas = trtc::Canvas::new(512, 512);

    let center_x = canvas.width as f64 / 2.0;
    let center_y = canvas.height as f64 / 2.0;
    let radius = canvas.width as f64 / 3.0;

    for i in 0..12 {
        let theta = i as f64 * (std::f64::consts::TAU / 12.0);

        let transform = trtc::Matrix4::eye()
            .translate(radius, 0.0, 0.0)
            .rotate_z(theta)
            .translate(center_x, center_y, 0.0);

        let p = transform * trtc::point(0.0, 0.0, 0.0);

        canvas[(p.x as usize, p.y as usize)] = trtc::Color::new(1.0, 1.0, 1.0);
    }

    canvas
        .save_png("out.png".into())
        .expect("Failed to save png");
}
