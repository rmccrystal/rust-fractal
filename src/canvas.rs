use pixel_canvas::{Canvas, Color};
use pixel_canvas::input::MouseState;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn create(width: usize, height: usize, algorithm: fn(f64, f64, i32) -> Option<i32>) {
    let canvas = Canvas::new(width, height)
        .title("Fractal")
        .state(MouseState::new())
        .input(MouseState::handle_input);

    let x_min = Arc::new(Mutex::new(-2.5f64));
    let x_max = Arc::new(Mutex::new(1.5f64));
    let y_min = Arc::new(Mutex::new(-1.5f64));
    let y_max = Arc::new(Mutex::new(1.5f64));



    canvas.render(move |_mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                if (y > 5 && y < 10) && x == 5 {
                    *pixel = Color {
                        r: 0,
                        g: 255,
                        b: 0,
                    };
                    continue;
                }
                let point = screen_to_point(width,
                                            height,
                                            (x, y),
                                            *x_min.lock().unwrap(),
                                            *x_max.lock().unwrap(),
                                            *y_min.lock().unwrap(),
                                            *y_max.lock().unwrap());
                *pixel = match algorithm(point.0, point.1, 60) {
                    Some(iter) => Color {
                        r: min((iter * 10) as u8, 255),
                        g: 0,
                        b: 0,
                    },
                    None => {
                        Color {
                            r: 255,
                            g: 255,
                            b: 255,
                        }
                    }
                }
            }
        }
    });
}

fn point_to_screen(screen_width: usize,
                   screen_height: usize,
                   point: (f64, f64),
                   x_min: f64,
                   x_max: f64,
                   y_min: f64,
                   y_max: f64) -> (usize, usize) {
    (
        ((screen_width as f64 / (x_max - x_min)) * (point.0 - x_min).round()) as usize,
        ((screen_height as f64 / (y_max - y_min)) * (point.1 - y_min).round()) as usize,
    )
}

fn screen_to_point(screen_width: usize,
                   screen_height: usize,
                   point: (usize, usize),
                   x_min: f64,
                   x_max: f64,
                   y_min: f64,
                   y_max: f64) -> (f64, f64) {
    (
        ((point.0 as f64 / (screen_width as f64 / (x_max - x_min))) + x_min) as f64,
        ((point.1 as f64 / (screen_height as f64 / (y_max - y_min))) + y_min) as f64,
    )
}
