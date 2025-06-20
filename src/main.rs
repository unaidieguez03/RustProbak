use macroquad::prelude::*;

#[macroquad::main("InputKeys")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;
let mut a = 1.0;
        let (j,k) = (screen_width()/2.0, screen_height()/2.0);
         print!("{} {}", j, k);loop {
        clear_background(LIGHTGRAY);
        
        a+=0.01; 
        // Llamar a la función async y obtener el resultado
        let (new_x, new_y) = get_inputs(x, y).await;
        x = new_x;
        y = new_y;
        draw_circle(j+a, k+a, 15.0, BLUE);
        draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await;
    }
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

