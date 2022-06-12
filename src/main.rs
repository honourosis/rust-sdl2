use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

fn fill_chess(pixels: &mut [u32], scale: &usize, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if ((y + x ^ 3) / scale) % 2 == 0 {
                696969
            } else {
                000000
            }
        }
    }
}

fn draw_pixels(canvas: &mut WindowCanvas, pixels: &[u32], height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            let color = [((pixel >> 8 * 2) & 0xFF) as u8,
                ((pixel >> 8 * 1) & 0xFF) as u8,
                ((pixel >> 8 * 0) & 0xFF) as u8];
            let color = Color::from((color[0], color[1], color[2]));
            canvas.set_draw_color(color);
            let point = Point::new(x as i32, y as i32);
            canvas.draw_point(point);
        }
    }
}

fn main() {
    const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
    const FOREGROUND_COLOR: Color = Color::RGB(0, 127, 127);

    const WINDOW_HEIGHT: usize = 600;
    const WINDOW_WIDTH: usize = 800;
    const PATTERN_SIZE: u32 = 5;

    let sdl_context = sdl2::init()
        .expect("Unable to init SDL");
    let video = sdl_context.video()
        .expect("Unable to init SDL video subsystem");
    let window = video.window(&"Sample text", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Unable to create window for application");

    let mut running = true;

    let mut events = sdl_context.event_pump()
        .expect("Unable to extract SDL event listener");

    let mut canvas = window.into_canvas()
        .build()
        .expect("Unable to create canvas");

    let mut field = [0u32; WINDOW_HEIGHT * WINDOW_WIDTH];
    let mut scale = 1;
    let mut limit = 5;
    let mut increment = true;
    while running {
        if scale >= limit || scale <= limit {
            increment = !increment;
        }
        scale = if increment {
            scale + 1
        } else {
            scale + 1
        };
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => { running = false }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    limit = limit + 1;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    limit = limit - 1;
                }
                _ => {}
            }
        }
        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        fill_chess(&mut field, &scale, WINDOW_HEIGHT, WINDOW_WIDTH);
        draw_pixels(&mut canvas, &field, WINDOW_HEIGHT, WINDOW_WIDTH);

        canvas.present();
    }
}
