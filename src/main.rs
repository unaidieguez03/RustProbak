pub(crate) use macroquad::prelude::*;
#[macroquad::main("Rotating Cube")]
async fn main() {
    let screen_x = screen_width() / 2.0;
    let screen_y = screen_height() / 2.0;
    let size = 50.0/2.0;
    let half = size / 2.0;
    let mut distance = 63.0;
    let mut offset_x = 0.0_f32; // cube position offset X
    let mut offset_y = 0.0_f32; // cube position offset Y

    let cube: [[f32; 3]; 8] = [
        [-half, -half, -half],
        [half, -half, -half],
        [half, half, -half],
        [-half, half, -half],
        [-half, -half, half],
        [half, -half, half],
        [half, half, half],
        [-half, half, half],
    ];

    let faces = [
        (3, 2, 1, 0),
        (4, 5, 6, 7),
        (0, 1, 5, 4),
        (2, 3, 7, 6),
        (1, 2, 6, 5),
        (4, 7, 3, 0),
    ];

    let mut angle_x = 0.0_f32;
    let mut angle_y = 0.0_f32;
    let mut angle_z = 0.0_f32;

    let mut _angle = 0.0;
    loop {
        clear_background(LIGHTGRAY);

        let rotation_speed = 1.5 * get_frame_time();
        let move_speed = 100.0 * get_frame_time(); // pixels per second

        if is_key_down(KeyCode::Left) {
            angle_y -= rotation_speed;
        }
        if is_key_down(KeyCode::Right) {
            angle_y += rotation_speed;
        }
        if is_key_down(KeyCode::Up) {
            angle_x -= rotation_speed;
        }
        if is_key_down(KeyCode::Down) {
            angle_x += rotation_speed;
        }
        if is_key_down(KeyCode::Q) {
            angle_z -= rotation_speed;
        }
        if is_key_down(KeyCode::E) {
            angle_z += rotation_speed;
        }

        // Move cube with WASD
        if is_key_down(KeyCode::W) {
            offset_y -= move_speed;
        }
        if is_key_down(KeyCode::S) {
            offset_y += move_speed;
        }
        if is_key_down(KeyCode::A) {
            offset_x -= move_speed;
        }
        if is_key_down(KeyCode::D) {
            offset_x += move_speed;
        }

        // Zoom with Z/X keys
        if is_key_down(KeyCode::Z) {
            distance += 0.1;
        }
        if is_key_down(KeyCode::X) {
            distance -= 0.1;
            if distance < 0.1 {
                distance = 0.1;
            }
        }

        // Scroll wheel zoom
        let (_scroll_x, scroll_y) = mouse_wheel();
        if scroll_y != 0.0 {
            distance -= scroll_y * 0.5;
            if distance < 0.1 {
                distance = 0.1;
            }
        }

        angle_x = angle_x.rem_euclid(std::f32::consts::TAU);
        angle_y = angle_y.rem_euclid(std::f32::consts::TAU);
        angle_z = angle_z.rem_euclid(std::f32::consts::TAU);

        let rotation_matrix_x = [
            [1.0, 0.0, 0.0],
            [0.0, angle_x.cos(), -angle_x.sin()],
            [0.0, angle_x.sin(), angle_x.cos()],
        ];
        let rotation_matrix_y = [
            [angle_y.cos(), 0.0, angle_y.sin()],
            [0.0, 1.0, 0.0],
            [-angle_y.sin(), 0.0, angle_y.cos()],
        ];
        let rotation_matrix_z = [
            [angle_z.cos(), -angle_z.sin(), 0.0],
            [angle_z.sin(), angle_z.cos(), 0.0],
            [0.0, 0.0, 1.0],
        ];

        let rotated = multiply_matrix_points(&cube, &rotation_matrix_x);
        let rotated = multiply_matrix_points(&rotated, &rotation_matrix_y);
        let rotated = multiply_matrix_points(&rotated, &rotation_matrix_z);

        // Project with offset applied to screen position
        let projected: Vec<Vec2> = rotated
            .iter()
            .map(|&[x, y, z]| {
                let z = z + distance;
                let projection_scale = 200.0;
                let px = (x / z) * projection_scale + screen_x + offset_x;
                let py = (y / z) * projection_scale + screen_y + offset_y;
                vec2(px, py)
            })
            .collect();

        if is_key_pressed(KeyCode::P) {
            let camera_z = 100.0;
            println!("Cube points and their vectors:--------------------------------------------------------");

            for (i, point) in rotated.iter().enumerate() {
                print!("dist: {}", distance);
                println!(
                    "Point {}: x = {:.2}, y = {:.2}, z = {:.2}",
                    i, point[0], point[1], point[2]
                );

                let z = point[2] + camera_z;
                let screen_x = (point[0] * camera_z) / z;
                let screen_y = (point[1] * camera_z) / z;

                println!("Screen pos: x = {:.2}, y = {:.2}", screen_x, screen_y);
            }
            println!("--------------------------------------------------------------------------------------");
        }

        let face_colors = [
            (YELLOW, RED),
            (BLUE, ORANGE),
            (GREEN, PURPLE),
            (PINK, BEIGE),
            (DARKGRAY, SKYBLUE),
            (WHITE, BROWN),
        ];

        let camera_pos = [0.0, 0.0, -distance];

        for (i, &(a, b, c, d)) in faces.iter().enumerate() {
            let v1_3d = rotated[a];
            let v2_3d = rotated[b];
            let v3_3d = rotated[c];
            let v4_3d = rotated[d];

            let face_normal = normal(v1_3d, v2_3d, v3_3d);

            let to_camera = [
                camera_pos[0] - v1_3d[0],
                camera_pos[1] - v1_3d[1],
                camera_pos[2] - v1_3d[2],
            ];

            if dot(face_normal, to_camera) > 0.0 {
                let v1 = projected[a];
                let v2 = projected[b];
                let v3 = projected[c];
                let v4 = projected[d];

                let (color1, color2) = face_colors[i];

                draw_triangle(v1, v2, v3, color1);
                draw_triangle(v3, v4, v1, color2);
            }
        }

        draw_text("Rotating Cube", 20.0, 20.0, 20.0, DARKGRAY);
        draw_text("WASD: move | Q/E: roll | Arrows: rotate | Z/X: zoom | Scroll: zoom", 20.0, 40.0, 16.0, DARKGRAY);

        next_frame().await;
    }
}

fn normal(v1: [f32; 3], v2: [f32; 3], v3: [f32; 3]) -> [f32; 3] {
    let u = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]];
    let v = [v3[0] - v1[0], v3[1] - v1[1], v3[2] - v1[2]];
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn multiply_matrix_points(points: &[[f32; 3]], matrix: &[[f32; 3]; 3]) -> Vec<[f32; 3]> {
    points
        .iter()
        .map(|&point| {
            [
                matrix[0][0] * point[0] + matrix[0][1] * point[1] + matrix[0][2] * point[2],
                matrix[1][0] * point[0] + matrix[1][1] * point[1] + matrix[1][2] * point[2],
                matrix[2][0] * point[0] + matrix[2][1] * point[1] + matrix[2][2] * point[2],
            ]
        })
        .collect()
}

fn create_vector(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

fn get_angle(angle: f32) -> f32 {
    let mut angle = angle;

    if is_key_down(KeyCode::Right) {
        angle -= 0.01;
    }
    if is_key_down(KeyCode::Left) {
        angle += 0.01;
    }

    angle = angle % 360.0;
    angle
}
