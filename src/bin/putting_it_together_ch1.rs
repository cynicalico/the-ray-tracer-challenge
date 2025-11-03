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

fn main() {
    let environment = Environment {
        gravity: trtc::vector(0.0, -0.1, 0.0),
        wind: trtc::vector(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: trtc::point(0.0, 1.0, 0.0),
        velocity: trtc::vector(1.0, 1.8, 0.0),
    };

    while projectile.position.y > 0.0 {
        tick(&environment, &mut projectile);
        println!("{:?}", projectile.position);
    }
}
