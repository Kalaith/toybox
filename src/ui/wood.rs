use macroquad::prelude::*;

pub(crate) fn draw_wood_cube(center: Vec3, size: Vec3, seed: usize) {
    let base = wood_tone(seed);
    draw_cube(center, size, None, base);
    draw_cube_wires(center, size, Color::new(0.12, 0.075, 0.035, 0.72));

    let long_x = size.x >= size.z;
    let grain_count = if long_x {
        (size.z / 0.28).ceil() as usize
    } else {
        (size.x / 0.28).ceil() as usize
    }
    .clamp(1, 5);

    for index in 0..grain_count {
        let offset = (index as f32 + 0.5) / grain_count as f32 - 0.5;
        let color = Color::new(
            (base.r - 0.06).max(0.0),
            (base.g - 0.045).max(0.0),
            (base.b - 0.025).max(0.0),
            0.62,
        );
        if long_x {
            draw_cube(
                center + vec3(0.0, size.y * 0.52, offset * size.z),
                vec3(size.x * 0.92, 0.012, 0.018),
                None,
                color,
            );
        } else {
            draw_cube(
                center + vec3(offset * size.x, size.y * 0.52, 0.0),
                vec3(0.018, 0.012, size.z * 0.92),
                None,
                color,
            );
        }
    }
}

pub(crate) fn draw_dark_trim(center: Vec3, size: Vec3) {
    draw_cube(center, size, None, Color::new(0.13, 0.080, 0.040, 1.0));
    draw_cube_wires(center, size, Color::new(0.35, 0.22, 0.10, 0.58));
}

pub(crate) fn wood_tone(seed: usize) -> Color {
    const TONES: [Color; 6] = [
        Color::new(0.30, 0.18, 0.09, 1.0),
        Color::new(0.36, 0.22, 0.11, 1.0),
        Color::new(0.42, 0.27, 0.14, 1.0),
        Color::new(0.25, 0.15, 0.08, 1.0),
        Color::new(0.48, 0.32, 0.17, 1.0),
        Color::new(0.33, 0.20, 0.10, 1.0),
    ];
    TONES[seed % TONES.len()]
}
