fn main() {
    let mut canvas = trtc::Canvas::new(200, 200);
    let mut shape = trtc::Sphere::default();

    shape.transform = trtc::Matrix4::eye()
        .scale(0.5, 1.0, 1.0)
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let ray_origin = trtc::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas.width as f64;

    for y in 0..canvas.height {
        let world_y = (wall_size / 2.0) - pixel_size * y as f64;

        for x in 0..canvas.height {
            let world_x = -(wall_size / 2.0) + pixel_size * x as f64;

            let position = trtc::point(world_x, world_y, wall_z);

            let r = trtc::Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = trtc::intersect(&shape, &r);

            if trtc::hit(&xs).is_some() {
                canvas[(x, y)] = trtc::Color::new(1.0, 0.0, 0.0);
            }
        }
    }

    canvas
        .save_png("out.png".into())
        .expect("Failed to save png");
}
