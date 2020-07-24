use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_TITLE: &str = "Rusty Battleship";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
        // clear canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,

                _ => {}
            }
        }
        
        draw_board_lines(&mut canvas);

        canvas.present();
    }
}

fn draw_board_lines(canvas: &mut Canvas<Window>) {
    // line color
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    // draw lines.
    let x_offset: i32 = (WINDOW_WIDTH as i32 - WINDOW_HEIGHT as i32) / 2;
    let min_wh: i32 = std::cmp::min(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

    let x_interval: i32 = min_wh / 10;
    let y_interval: i32 = min_wh / 10;

    for i in 0..=10 {
        // vertical lines
        canvas.draw_line(
            Point::new(x_interval * i + x_offset, 0), 
            Point::new(x_interval * i + x_offset, WINDOW_HEIGHT as i32))
            .unwrap();
        
        // horizontal lines
        canvas.draw_line(
            Point::new(x_offset, y_interval * i), 
            Point::new(WINDOW_WIDTH as i32 - x_offset, y_interval * i))
            .unwrap();
    }
}