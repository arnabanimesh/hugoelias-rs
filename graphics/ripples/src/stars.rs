#![windows_subsystem = "windows"]

use minifb::{Key, Window, WindowOptions};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const SIZE: usize = WIDTH * HEIGHT;

const DAMPING: f64 = 0.3; // Enter (0,1], 0 means fully damped
const DROP_RATE: f64 = 0.3;
const DELAY: u64 = 10000; // default: 16600 micro seconds  = 1000000/100 frame delay for 100 fps
const RIPPLE_SIZE: u8 = 10; //Enter 1-255

fn main() {
    let mut previous = vec![vec![0.; HEIGHT]; WIDTH];
    let mut current = vec![vec![0.; HEIGHT]; WIDTH];
    let mut buffer = vec![0; SIZE];
    let uniform_x = Uniform::new(1, WIDTH - 1);
    let uniform_y = Uniform::new(1, HEIGHT - 1);
    let mut rng = thread_rng();

    let mut window = Window::new(
        "Ripples - Click and drag to create ripples, ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(DELAY)));

    let mut leftclick = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            // Holding the left mouse button and dragging it outside the window causes it to
            // remain pressed even after releasing. TO rectify it, press the left mouse button
            // inside the window
            if leftclick {
                leftclick = false;
            } else {
                leftclick = window.get_mouse_down(minifb::MouseButton::Left);
            }
            if leftclick {
                previous[x as usize][y as usize] = grayscale(1) as f64;
            }
        }
        if rand::random::<f64>() < DROP_RATE {
            previous[uniform_x.sample(&mut rng)][uniform_y.sample(&mut rng)] =
                grayscale(RIPPLE_SIZE) as f64;
        }
        for x in 1..(WIDTH - 1) {
            for y in 1..(HEIGHT - 1) {
                current[x][y] = ((previous[x - 1][y]
                    + previous[x + 1][y]
                    + previous[x][y - 1]
                    + previous[x][y + 1]
                    + previous[x - 1][y - 1]
                    + previous[x + 1][y + 1]
                    + previous[x + 1][y - 1]
                    + previous[x - 1][y + 1])
                    / 6.)
                    - current[x][y] * 2.;
                current[x][y] *= DAMPING;
                buffer[index(x, y)] = grayscale(current[x][y] as u8);
            }
        }
        let temp = previous;
        previous = current;
        current = temp;

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

/* const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | b as u32
} */
const fn grayscale(color: u8) -> u32 {
    ((color as u32) << 16) | ((color as u32) << 8) | color as u32
}
const fn index(x: usize, y: usize) -> usize {
    y * WIDTH + x
}
