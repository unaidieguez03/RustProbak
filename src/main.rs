use macroquad::prelude::*;

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
    let mut angle = 0.0;
         loop {
        clear_background(LIGHTGRAY);
        
        // Llamar a la función async y obtener el resultado
        let new_angle = get_angle(angle).await;

        if(angle!=new_angle){
            x = x * f32::cos(angle) - y *f32::sin(angle); 
            y = x * f32::sin(angle) + y *f32::cos(angle); 
        }
        angle = new_angle;
        print!("angle:{} , x:{}, y:{}\n", angle, x, y);
        let vec1:Vec2 =create_vector(x-100.0, y+100.0);
        let vec2:Vec2 =create_vector(x+100.0, y+100.0);
        let vec3:Vec2 =create_vector(x-100.0, y-100.0);
        let vec4:Vec2 =create_vector(x+100.0, y-100.0);
        draw_triangle(vec1, vec2, vec3, YELLOW);
        draw_triangle(vec3, vec2, vec4, GREEN);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await;
    }
}
fn create_vector(x: f32, y: f32)->Vec2{
    let return_vector = Vec2::new(x, y);
    return_vector
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
async fn get_inputs(x: f32, y: f32) -> (f32, f32) {
    let mut x = x;
    let mut y = y;

    if is_key_down(KeyCode::Right) {
        x += 1.0;
    }
    if is_key_down(KeyCode::Left) {
        x -= 1.0;
    }
    if is_key_down(KeyCode::Down) {
        y += 1.0;
    }
    if is_key_down(KeyCode::Up) {
        y -= 1.0;
    }

    (x, y)
}

