use rand::Rng;
use rand::rngs::ThreadRng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

fn fill_with_check_pattern(pixels: &mut [u32], scale: &usize, height: usize, width: usize, rng: &mut ThreadRng) {
    for y in 0..height {
        for x in 0..width {
            pixels[y * width + x] = if (x / scale + y / scale) % 2 == 0 {
                get_random_color(rng)
            } else {
                compose_color(20, 20, 20)
            }
        }
    }
}

fn split_rgb(color: u32) -> (u8, u8, u8) {
    (((color >> 8 * 2) & 0xFF) as u8,
     ((color >> 8 * 1) & 0xFF) as u8,
     ((color >> 8 * 0) & 0xFF) as u8)
}

fn compose_color(r: u32, g: u32, b: u32) -> u32 {
    let mut rgb = r;
    rgb = (rgb << 8) + g;
    rgb = (rgb << 8) + b;
    rgb as u32
}

fn get_random_color(rng: &mut ThreadRng) -> u32 {
    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);
    compose_color(r, g, b)
}

fn draw_pixels(canvas: &mut WindowCanvas, pixels: &[u32], height: usize, width: usize, scale: usize) {
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            let color = split_rgb(pixel);
            let color = Color::from(color);
            canvas.set_draw_color(color);
            let rect = Rect::new((x * scale) as i32, (y * scale) as i32, scale as u32, scale as u32);
            canvas.draw_rect(rect);
        }
    }
}

fn main() {
    const BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
    const FOREGROUND_COLOR: Color = Color::RGB(0, 127, 127);

    const WINDOW_HEIGHT: usize = 600;
    const WINDOW_WIDTH: usize = 800;
    const WINDOW_SCALE: usize = 4;

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
        .present_vsync()
        .accelerated()
        .build()
        .expect("Unable to create canvas");

    let mut field = [0u32; WINDOW_HEIGHT * WINDOW_WIDTH / WINDOW_SCALE];
    let mut rng = rand::thread_rng();

    let mut upper_limit = 25;
    let mut lower_limit = 10;
    let mut scale = lower_limit;
    let mut increment = true;
    while running {
        println!("Scale: {}; Upper limit: {} | Lower limit: {}", scale, upper_limit, lower_limit);
        if scale >= upper_limit && increment {
            increment = false;
        } else if scale <= lower_limit && !increment {
            increment = true;
        }
        scale = if increment {
            scale + 1
        } else {
            scale - 1
        };
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => { running = false }
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    upper_limit = upper_limit + 1;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if !(upper_limit - 1 <= lower_limit) {
                        upper_limit = upper_limit - 1;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    if !(lower_limit + 1 >= upper_limit) {
                        lower_limit = lower_limit + 1;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if !(lower_limit <= 1) {
                        lower_limit = lower_limit - 1;
                    }
                }
                _ => {}
            }
        }

        fill_with_check_pattern(&mut field, &scale, WINDOW_HEIGHT / WINDOW_SCALE, WINDOW_WIDTH / WINDOW_SCALE, &mut rng);

        canvas.set_draw_color(BACKGROUND_COLOR);
        canvas.clear();

        draw_pixels(&mut canvas, &field, WINDOW_HEIGHT / WINDOW_SCALE, WINDOW_WIDTH / WINDOW_SCALE, WINDOW_SCALE);

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}
