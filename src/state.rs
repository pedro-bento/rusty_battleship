use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use async_trait::async_trait;

pub enum NextState {
    Update(Box<dyn State>),
    Continue,
    Quit,
}

#[async_trait(?Send)]
pub trait State {
    async fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        next_state: &mut Option<NextState>,
    );
    async fn draw(&self, canvas: &mut Canvas<Window>);
}
