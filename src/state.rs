use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub enum NextState {
    Update(Box<dyn State>),
    Continue,
    Quit,
}

pub trait State {
    fn handle_events(&mut self, event_pump: &mut EventPump) -> NextState;
    fn draw(&self, canvas: &mut Canvas<Window>);
}
