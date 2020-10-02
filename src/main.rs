use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Fractal - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X2,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to Open Window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    window.set_background_color(0, 0, 20);
	let mut time = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..buffer.len() {
            buffer[i] = i as u32 + time ^ 10;
        }

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        time += 1;
    }
}
