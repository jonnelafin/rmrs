use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = (450.0*1.1) as usize;
const HEIGHT: usize = 450;


fn main() {
	let mut player_pos = HEIGHT as f64 / 2.0;
	let player_speed = 20.0;
	let mut ai_pos = HEIGHT as u32 / 2;

	let mut ball_y = HEIGHT as f64 / 2.0;
	let mut ball_x = WIDTH as f64 / 2.0;
	let mut ball_speed = 10.0;
	let mut vel_y = 0.0;
	let mut vel_x = -ball_speed;
	
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut paint_mode = false;
    let mut paint_cool = 0.0;

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


    window.set_background_color(0, 0, 0);
	let mut time = 10;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let w = WIDTH as f64;
        let h = HEIGHT as f64;
    	//ai_pos = ( (HEIGHT as u32 / 2 ) as f64 + ((time as f64)/10.0).sin() * ((HEIGHT as f64)*0.8 - (HEIGHT as f64)*0.2) / 2.0) as u32;
		let jitter = rand::thread_rng().gen_range(10, 101) as f64 / 10.0 + 50.0;
		ai_pos = ball_y as u32 + jitter as u32;
		if window.is_key_down(Key::Space){
			if(paint_cool < 0.001){
				paint_cool = 20.0;
				paint_mode = !paint_mode;
			}
		}
		paint_cool -= 1.0;
		if window.is_key_down(Key::Up) {
			player_pos -= player_speed;
			player_pos = player_pos.max(0.0);
		}
		if window.is_key_down(Key::Down) {
			player_pos += player_speed;
			player_pos = player_pos.min(HEIGHT as f64);
		}


		if ball_y > (HEIGHT as f64)-6.0{
			vel_y = -ball_speed;
		}
		if ball_y < 6.0{
			vel_y = ball_speed;
		}
		
		if ball_x > (WIDTH as f64)-6.0{
			vel_x = -ball_speed;
		}
		if ball_x < 6.0{
			vel_x = ball_speed;
		}
		
		if (ball_y - (player_pos as f64)).abs() < 106.0 && ball_x-6.0 < 62.0{
			vel_x = -vel_x;
			let diff = ball_y - (player_pos as f64);
			let r = rand::thread_rng().gen_range(10, 101) as f64 / 7.0;
			vel_y = r * (diff/70.0);
		}
		if (ball_y - (ai_pos as f64)).abs() < (WIDTH as f64)-46.0 && ball_x-6.0 > (WIDTH as f64)-67.0{
			vel_x = -vel_x;
			let diff = ball_y - (ai_pos as f64);
			let r = rand::thread_rng().gen_range(10, 101) as f64 / 7.0;
			vel_y = r * (diff/70.0);
		}
		ball_x += vel_x;
		ball_y += vel_y;
        for i in 0..buffer.len() {
        	let y = i / HEIGHT;
        	let x = i % WIDTH;
            buffer[i] = ( (x as u32 + y as u32) as f32  % (time as f32 /2.0) ) as u32;//i as u32 + time ^ 10;

			
       		let vx = x;
       		let vy = y;
       		let mut po = 0.0;
       		if paint_mode{
       			po = ((vx^vy) as f64).rem_euclid((buffer.len() as f64));
       		}
       		else{
       			po = (i+i)/*((vy as f64 * h) + (vx as f64))*/ as f64 % (buffer.len() as f64);
       			//po = ((vy^vx) as f64).rem_euclid((buffer.len() as f64));
       		}
       		//let po = vx+vy;
        	if po < buffer.len() as f64{
        		buffer[i] = (buffer[po as usize] as f64 * 0.25 + 2.5) as u32;
        	}

            let mut ppos = player_pos as f64; //% (HEIGHT as f64);
            if ppos < 1.0{
            	ppos = 1.0;
            }
            if ( (y as f64)-(ppos) ).abs() < 100.0{
            	if (x as f64) > 40.0 && (x as f64) < 61.0{
            		buffer[i] = 0x00ffffff;
            	}
            }
            if ( (y as f64)-(ai_pos as f64) ).abs() < 100.0{
            	if (x as f64) < (w-40.0) && (x as f64) > (w-61.0){
            		buffer[i] = 0x00ffffff;
            	}
            }
            if ( (y as f64)-(ball_y as f64) ).abs() < 12.0{
            	if ( (x as f64)-(ball_x as f64) ).abs() < 12.0{
            		buffer[i] = 0x00ff_00_00;
            	}
            }
        }

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        time += 1;
    }
}
