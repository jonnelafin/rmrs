use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 900;
const HEIGHT: usize = 900;


fn main() {
	let mut player_pos = WIDTH as u32 / 2;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "RMRS - ESC to exit",
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
    //window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    window.set_background_color(0, 0, 20);
	let mut time = 10;
    while window.is_open() && !window.is_key_down(Key::Escape) {
    	player_pos = ( (WIDTH as u32 / 2 ) as f64 + ((time as f64)/10.0).sin() * (HEIGHT as f64) / 2.0) as u32;
        for i in 0..buffer.len() {
        	let y = i / WIDTH;
        	let x = i % HEIGHT;
            buffer[i] = ( (x as u32 + y as u32) as f32  % (time as f32 /2.0) ) as u32;//i as u32 + time ^ 10;
            if ( (y as f64)-(player_pos as f64) ).abs() < 100.0{
            	if (x as f64) > 40.0 && (x as f64) < 61.0{
            		buffer[i] = 0x00ffffff;
            	}
            }
        }

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        time += 1;
    }
}
