use macroquad::prelude::*;
#[macroquad::main("InputKeys")]
async fn main() {
    let screen_x = screen_width() / 2.0;
    let screen_y = screen_height() / 2.0;
    print!("{},{}\n", screen_x, screen_y);
    let mut angle = 0.0;
let mut rotated_positions=[[screen_x+50.0,screen_x-50.0, screen_x-50.0,screen_x+50.0],
                                                   [screen_y+50.0,screen_y+50.0,screen_y-50.0,screen_y-50.0]];
    let position_init =[[screen_x+50.0,screen_x-50.0, screen_x-50.0,screen_x+50.0],
                                                   [screen_y+50.0,screen_y+50.0,screen_y-50.0,screen_y-50.0]];
            print!("joder: {}, {}, {}, {}\n", rotated_positions[0][0],rotated_positions[0][1],rotated_positions[0][2],rotated_positions[0][3]);
            print!("joder: {}, {}, {}, {}\n", rotated_positions[1][0],rotated_positions[1][1],rotated_positions[1][2],rotated_positions[1][3]);
    loop {
        clear_background(LIGHTGRAY);
        
        // Llamar a la función async y obtener el resultado
        let new_angle = get_angle(angle).await;
        
        if angle!=new_angle {
            angle = new_angle.to_radians();
let rotation_matrix =[[f32::cos(-angle),-f32::sin(-angle)],
                                             [f32::sin(-angle),f32::cos(-angle)]];
            print!("{}, {}\n",angle, new_angle);
            rotated_positions = rotate_positions(rotation_matrix, position_init, (screen_x, screen_y));

            print!("rotated_x: {}, {}, {}, {}\n", rotated_positions[0][0],rotated_positions[0][1],rotated_positions[0][2],rotated_positions[0][3]);
            print!("rotated_y: {}, {}, {}, {}\n", rotated_positions[1][0],rotated_positions[1][1],rotated_positions[1][2],rotated_positions[1][3]);
            print!("--------------------------------------------------------------------------------------\n");
            print!("rotation_x: {}, {}, {}, {}\n", rotation_matrix[0][0],rotation_matrix[0][1],rotation_matrix[1][0],rotation_matrix[1][1]);
            print!("--------------------------------------------------------------------------------------\n");
        }
//        print!("angle:{} , x:{}, y:{}\n", angle, x, y);
        let vec1:Vec2 =create_vector(rotated_positions[0][0], rotated_positions[1][0]);
        let vec2:Vec2 =create_vector(rotated_positions[0][1], rotated_positions[1][1]);
        let vec3:Vec2 =create_vector(rotated_positions[0][2], rotated_positions[1][2]);
        let vec4:Vec2 =create_vector(rotated_positions[0][3], rotated_positions[1][3]);
        draw_triangle(vec1, vec2, vec3, YELLOW);
        draw_triangle(vec3, vec1, vec4, GREEN);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await;
    }
}
fn create_vector(x: f32, y: f32)->Vec2{
    let return_vector = Vec2::new(x, y);
    return_vector
}
fn rotate_positions(
    rotation_matrix: [[f32; 2]; 2],
    positions: [[f32; 4]; 2],
    center: (f32, f32),
) -> [[f32; 4]; 2] {
    let mut centered = [[0.0f32; 4]; 2];
    let mut rotated = [[0.0f32; 4]; 2];

    // Step 1: Subtract center to move points relative to origin
    for i in 0..4 {
        centered[0][i] = positions[0][i] - center.0;
        centered[1][i] = positions[1][i] - center.1;
    }

    // Step 2: Rotate around origin
    for i in 0..2 {
        for j in 0..4 {
            for k in 0..2 {
                rotated[i][j] += rotation_matrix[i][k] * centered[k][j];
            }
        }
    }

    // Step 3: Add center back to get screen coordinates
    for i in 0..4 {
        rotated[0][i] += center.0;
        rotated[1][i] += center.1;
    }

    rotated
}


async fn get_angle(angle:f32) -> f32 {
    let mut angle:f32= angle;
    if is_key_down(KeyCode::Right) {
        angle = 0.0;
    }
    if is_key_down(KeyCode::Left) {
        angle = 180.0;
    }
    if is_key_down(KeyCode::Down) {
        angle = 270.0;
    }
    if is_key_down(KeyCode::Up) {
        angle = 90.0;
    }
    angle
}
