use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
// use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::vec::Vec;

// use super::config;
use super::ship;
use super::state;

pub struct BattleState {
    board_lines: Vec<(Point, Point)>,
    my_ships: Vec<ship::Ship>,
}

impl BattleState {
    pub fn new(board_lines: Vec<(Point, Point)>, my_ships: Vec<ship::Ship>) -> BattleState {
        BattleState {
            board_lines: board_lines,
            my_ships: my_ships,
        }
    }
}

impl state::State for BattleState {
    fn handle_events(&mut self, event_pump: &mut EventPump) -> state::NextState {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return state::NextState::Quit,

                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => return state::NextState::Quit,

                    _ => {
                        println!("<InitialState> unused key: {}", keycode.unwrap());
                    }
                },

                _ => {}
            }
        }

        return state::NextState::Continue;
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        // draw board lines.
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for (p1, p2) in self.board_lines.iter() {
            canvas.draw_line(*p1, *p2).unwrap()
        }
    }
}
