use raylib::prelude::*;
use std::{thread, time::Duration};

const WIDTH: usize = 140;
const HEIGHT: usize = 85;

struct Framebuffer {
    width: usize,
    height: usize,
    buffer: Vec<bool>,
    buffer_next: Vec<bool>,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![false; width * height],
            buffer_next: vec![false; width * height],
        }
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            Some((y as usize) * self.width + (x as usize))
        } else {
            None
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        if let Some(i) = self.index(x, y) {
            self.buffer[i]
        } else {
            false
        }
    }

    fn set(&mut self, x: i32, y: i32, val: bool) {
        if let Some(i) = self.index(x, y) {
            self.buffer[i] = val;
        }
    }

    fn step(&mut self) {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let alive = self.get(x, y);
                let mut neighbors = 0;
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if self.get(x + dx, y + dy) {
                            neighbors += 1;
                        }
                    }
                }
                
                let idx = self.index(x, y).unwrap();
                self.buffer_next[idx] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        std::mem::swap(&mut self.buffer, &mut self.buffer_next);
    }
}

fn draw_framebuffer(d: &mut RaylibDrawHandle, fb: &Framebuffer, scale: i32) {
    d.clear_background(Color::BLACK);

    for y in 0..fb.height as i32 {
        for x in 0..fb.width as i32 {
            if fb.get(x, y) {
                d.draw_rectangle(x * scale, y * scale, scale, scale, Color::WHITE);
            }
        }
    }
}

fn draw_pattern(fb: &mut Framebuffer, pattern: &[(i32, i32)], origin: (i32, i32)) {
    for &(dx, dy) in pattern {
        fb.set(origin.0 + dx, origin.1 + dy, true);
    }
}

// Pulsar (period 3)
fn pulsar_pattern() -> Vec<(i32, i32)> {
    vec![
        (2, 0), (3, 0), (4, 0),     (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5),     (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7),     (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12),  (8, 12), (9, 12), (10, 12),
    ]
}

// Blinker (period 2) 
fn blinker_pattern() -> Vec<(i32, i32)> {
    vec![
        (0, 0), (1, 0), (2, 0),
    ]
}

// Glider 
fn glider_pattern() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ]
}

// Light-weight spaceship (LWSS) 
fn lwss_pattern() -> Vec<(i32, i32)> {
    vec![
        (0, 0), (3, 0),
        (4, 1),
        (0, 2), (4, 2),
        (1, 3), (2, 3), (3, 3), (4, 3),
    ]
}

fn main() {
    let screen_width = 1120;
    let screen_height = 800;
    let scale = 8;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Conway's Game of Life - Multiple Patterns")
        .build();

    rl.set_target_fps(10);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    // FILA SUPERIOR - Pulsars 
    for i in 0..6 {
        draw_pattern(&mut framebuffer, &pulsar_pattern(), (3 + i * 22, 3));
    }


    // Primera fila - Blinkers 
    for i in 0..8 {
        draw_pattern(&mut framebuffer, &blinker_pattern(), (10 + i * 15, 30));
    }
    
    // Segunda fila - Gliders 
    for i in 0..6 {
        draw_pattern(&mut framebuffer, &glider_pattern(), (15 + i * 18, 35));
    }
    
    // Tercera fila - LWSS 
    for i in 0..5 {
        draw_pattern(&mut framebuffer, &lwss_pattern(), (12 + i * 22, 40));
    }
    
    // Cuarta fila - blinkers
    for i in 0..7 {
        draw_pattern(&mut framebuffer, &blinker_pattern(), (8 + i * 17, 45));
    }
    
    // Quinta fila - Combinación de gliders y LWSS
    for i in 0..6 {
        if i % 2 == 0 {
            draw_pattern(&mut framebuffer, &glider_pattern(), (18 + i * 16, 50));
        } else {
            draw_pattern(&mut framebuffer, &lwss_pattern(), (18 + i * 16, 50));
        }
    }
    
    for i in 0..8 {
        draw_pattern(&mut framebuffer, &blinker_pattern(), (6 + i * 16, 55));
    }

    for i in 0..5 {
        draw_pattern(&mut framebuffer, &pulsar_pattern(), (8 + i * 25, 67));
    }

    while !rl.window_should_close() {
        framebuffer.step();

        let mut d = rl.begin_drawing(&thread);
        draw_framebuffer(&mut d, &framebuffer, scale);

        thread::sleep(Duration::from_millis(1));
    }
}