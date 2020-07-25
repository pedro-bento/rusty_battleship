use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::vec::Vec;

use super::config;
use super::ship;
use super::state;

pub struct InitialState {
  board_lines: Vec<(Point, Point)>,
  ships: [ship::Ship; 5],
}

impl InitialState {
  pub fn new() -> InitialState {
      let mut battleship = ship::Ship::new(ship::ShipType::Battleship);
      battleship.move_xy(Point::new(1, 2));

      let mut destroyer = ship::Ship::new(ship::ShipType::Destroyer);
      destroyer.move_xy(Point::new(2, 4));

      let mut submarine = ship::Ship::new(ship::ShipType::Submarine);
      submarine.move_xy(Point::new(3, 6));

      let mut patroalboat = ship::Ship::new(ship::ShipType::PatrolBoat);
      patroalboat.move_xy(Point::new(4, 8));

      InitialState {
        board_lines: InitialState::generate_board_lines(),
          ships: [
              ship::Ship::new(ship::ShipType::Carrier),
              battleship,
              destroyer,
              submarine,
              patroalboat,
          ],
      }
  }

  fn generate_board_lines() -> Vec<(Point, Point)> {
    let mut line_points: Vec<(Point, Point)> =
        Vec::with_capacity(((config::BOARD_LENGTH + 1) * 2) as usize);

    let x_offset: i32 = (config::WINDOW_WIDTH as i32 - config::WINDOW_HEIGHT as i32) / 2;
    let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

    let x_interval: i32 = min_wh / 10;
    let y_interval: i32 = min_wh / 10;

    for i in 0..=config::BOARD_LENGTH {
        // vertical lines.
        line_points.push((
            Point::new(x_interval * i as i32 + x_offset, 0),
            Point::new(
                x_interval * i as i32 + x_offset,
                config::WINDOW_HEIGHT as i32,
            ),
        ));

        // horizontal lines.
        line_points.push((
            Point::new(x_offset, y_interval * i as i32),
            Point::new(
                config::WINDOW_WIDTH as i32 - x_offset,
                y_interval * i as i32,
            ),
        ));
    }

    line_points
  }
}

impl state::State for InitialState {
  fn handle_events(&mut self, event_pump: &mut EventPump) -> bool {
      for event in event_pump.poll_iter() {
          match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => return true,

              _ => {}
          }
      }

      false
  }

  fn draw(&self, canvas: &mut Canvas<Window>) {
      // draw board lines.
      canvas.set_draw_color(Color::RGB(0, 255, 0));
      for (p1, p2) in self.board_lines.iter() {
        canvas.draw_line(*p1, *p2).unwrap()
      }

      // draw ships.
      canvas.set_draw_color(Color::RGB(0, 255, 0));

      let x_offset: i32 = (config::WINDOW_WIDTH as i32 - config::WINDOW_HEIGHT as i32) / 2;
      let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

      let x_interval: i32 = min_wh / 10;
      let y_interval: i32 = min_wh / 10;

      for ship in self.ships.iter() {
          for point in ship.body.iter() {
              let cell = Rect::new(
                  point.x * x_interval + x_offset,
                  point.y * y_interval,
                  x_interval as u32,
                  y_interval as u32,
              );
              canvas.fill_rect(cell).unwrap();
              canvas.draw_rect(cell).unwrap();
          }
      } 
  }
}
