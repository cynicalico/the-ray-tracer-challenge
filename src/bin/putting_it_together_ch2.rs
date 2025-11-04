use trtc;

struct Environment {
    gravity: trtc::Tuple,
    wind: trtc::Tuple,
}

struct Projectile {
    position: trtc::Tuple,
    velocity: trtc::Tuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn main() -> Result<(), std::io::Error> {
    let environment = Environment {
        gravity: trtc::vector(0.0, -0.1, 0.0),
        wind: trtc::vector(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: trtc::point(0.0, 1.0, 0.0),
        velocity: trtc::normalize(&trtc::vector(1.0, 1.8, 0.0)) * 11.25,
    };

    let mut canvas = trtc::Canvas::new(900, 550);
    let red = trtc::Color::new(1.0, 0.0, 0.0);

    loop {
        tick(&environment, &mut projectile);
        if projectile.position.y < 0.0 {
            break;
        }

        let x = projectile.position.x.floor();
        let y = projectile.position.y.floor();
        if (x < 0.0 || x >= canvas.width as f64) || (y < 0.0 || y >= canvas.height as f64) {
            continue;
        }

        let flip_y = canvas.height - 1 - (y as usize);
        canvas[(x as usize, flip_y)] = red;
    }

    canvas.save_png("out.png".into())
}
