use trtc;

#[test]
fn test_creating_a_canvas() {
    let c = trtc::Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);

    for y in 0..c.height {
        for x in 0..c.width {
            assert_eq!(c[(x, y)], trtc::Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn test_writing_pixels_to_a_canvas() {
    let mut c = trtc::Canvas::new(10, 20);
    let red = trtc::Color::new(1.0, 0.0, 0.0);

    c[(2, 3)] = red;

    assert_eq!(c[(2, 3)], red);
}
