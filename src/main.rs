use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

mod battle_state;
mod chat;
mod choose_state;
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
        let sdl_context: sdl2::Sdl = sdl2::init().unwrap();
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
            state: Box::new(choose_state::ChooseState::new()),
            canvas: canvas,
            event_pump: sdl_context.event_pump().unwrap(),    
        }
    }

    async fn run(&mut self) {
        'game_loop: loop {
            // clear
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            let mut next_state: Option<state::NextState> = None;
            let _ = self
                .state
                .handle_events(&mut self.event_pump, &mut next_state)
                .await;

            match next_state {
                Some(state::NextState::Quit) => break 'game_loop,

                Some(state::NextState::Update(new_state)) => {
                    self.state = new_state;
                }

                Some(state::NextState::Continue) => {}

                _ => {}
            }

            self.state.draw(&mut self.canvas).await;

            // render
            self.canvas.present();
        }
    }
}

#[tokio::main]
pub async fn main() {
    let mut game = Game::new();
    let _ = game.run().await;
}
