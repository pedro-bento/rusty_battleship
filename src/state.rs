use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub trait State {
    fn handle_events(&mut self, event_pump: &mut EventPump) -> bool; // return false when quit.
    fn draw(&self, canvas: &mut Canvas<Window>);
}
