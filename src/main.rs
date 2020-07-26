use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

mod battle_state;
mod config;
mod initial_state;
mod ship;
mod state;

struct Game {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    state: Box<dyn state::State>,
}

impl Game {
    fn new() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                config::WINDOW_TITLE,
                config::WINDOW_WIDTH,
                config::WINDOW_HEIGHT,
            )
            .position_centered()
            .build()
            .unwrap();
        
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend); 

        Game {
            canvas: canvas,
            event_pump: sdl_context.event_pump().unwrap(),
            state: Box::new(initial_state::InitialState::new()),
        }
    }

    fn run(&mut self) {
        'game_loop: loop {
            // clear
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            match self.state.handle_events(&mut self.event_pump) {
                state::NextState::Quit => break 'game_loop,

                state::NextState::Update(new_state) => {
                    self.state = new_state;
                }

                state::NextState::Continue => {}
            }

            self.state.draw(&mut self.canvas);

            // render
            self.canvas.present();
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}
