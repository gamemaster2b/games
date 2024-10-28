use rand::random;

/// generates a random direction as an angle in radians
pub fn get_random_direction(cone: f32) -> f32 {
    let mut direction: f32 = random::<f32>() * 360.;
    'set_cone: loop {
        if direction > (cone / 2.) && direction < 90.
            || direction > 180. + (cone / 2.) && direction < 270.
        {
            direction -= 45.;
        } else if direction > 270. && direction < 360. - (cone / 2.)
            || direction > 90. && direction < 180. - (cone / 2.)
        {
            direction += 45.;
        } else {
            direction = direction.to_radians();
            break 'set_cone;
        }
    }
    direction
}
