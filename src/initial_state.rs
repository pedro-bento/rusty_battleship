use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use std::vec::Vec;

use async_trait::async_trait;
use tokio::task::JoinHandle;

use super::battle_state;
use super::config;
use super::ship;
use super::state;

type ServerHandle = JoinHandle<mini_redis::Result<()>>;

pub struct InitialState {
    board_lines: Vec<(Point, Point)>,
    ships: Vec<ship::Ship>,

    ships_t: [ship::ShipType; 5],
    curr_ship_index: usize,
    curr_ship: Option<ship::Ship>,

    server_handle: Option<ServerHandle>,
}

impl InitialState {
    pub async fn new(server_handle: Option<ServerHandle>) -> InitialState {
        let ships_t = [
            ship::ShipType::Carrier,
            ship::ShipType::Battleship,
            ship::ShipType::Destroyer,
            ship::ShipType::Submarine,
            ship::ShipType::PatrolBoat,
        ];

        InitialState {
            board_lines: InitialState::generate_board_lines(),
            ships: Vec::new(),
            ships_t: ships_t,
            curr_ship_index: 1,
            curr_ship: Some(ship::Ship::new(ships_t[0])),
            server_handle: server_handle,
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

    fn get_next_ship(&mut self) -> Option<ship::Ship> {
        if self.curr_ship_index >= self.ships_t.len() {
            return None;
        }

        let result = Some(ship::Ship::new(self.ships_t[self.curr_ship_index]));
        self.curr_ship_index += 1;
        return result;
    }

    fn is_valid_ship(&self) -> bool {
        if self.curr_ship.is_none() {
            return false;
        }

        // does the current ship overlap already existing ships?
        for curr_ship_point in self.curr_ship.as_ref().unwrap().body.iter() {
            for ship in self.ships.iter() {
                for ship_point in ship.body.iter() {
                    if *curr_ship_point == *ship_point {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

#[async_trait(?Send)]
impl state::State for InitialState {
    async fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        next_state: &mut Option<state::NextState>,
    ) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    next_state.replace(state::NextState::Quit);
                    return;
                }

                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Return) => {
                        if self.is_valid_ship() {
                            self.ships.push(self.curr_ship.as_ref().unwrap().clone());
                            self.curr_ship = self.get_next_ship();
                        }

                        // as placed all ships.
                        // returns next state 'battle_state'.
                        if self.curr_ship.is_none() {
                            let rcv: String;
                            let snd: String;

                            if self.server_handle.is_some() {
                                rcv = "player2".to_string();
                                snd = "player1".to_string();
                            } else {
                                rcv = "player1".to_string();
                                snd = "player2".to_string();
                            }

                            next_state.replace(state::NextState::Update(Box::new(
                                battle_state::BattleState::new(
                                    self.board_lines.clone(),
                                    self.ships.clone(),
                                    self.server_handle.take(),
                                    rcv,
                                    snd,
                                )
                                .await
                                .unwrap(),
                            )));

                            return;
                        }
                    }

                    Some(Keycode::W) | Some(Keycode::Up) => {
                        if self.curr_ship.is_some() {
                            self.curr_ship.as_mut().unwrap().move_xy(&Point::new(0, -1));
                        }
                    }

                    Some(Keycode::S) | Some(Keycode::Down) => {
                        if self.curr_ship.is_some() {
                            self.curr_ship.as_mut().unwrap().move_xy(&Point::new(0, 1));
                        }
                    }

                    Some(Keycode::A) | Some(Keycode::Left) => {
                        if self.curr_ship.is_some() {
                            self.curr_ship.as_mut().unwrap().move_xy(&Point::new(-1, 0));
                        }
                    }

                    Some(Keycode::D) | Some(Keycode::Right) => {
                        if self.curr_ship.is_some() {
                            self.curr_ship.as_mut().unwrap().move_xy(&Point::new(1, 0));
                        }
                    }

                    Some(Keycode::Q) => {
                        if self.curr_ship.is_some() {
                            self.curr_ship.as_mut().unwrap().rotate();
                        }
                    }

                    Some(Keycode::E) => {
                        if self.curr_ship.is_some() {
                            self.curr_ship.as_mut().unwrap().rotate();
                        }
                    }

                    Some(Keycode::Escape) => {
                        next_state.replace(state::NextState::Quit);
                        return;
                    }

                    _ => {
                        println!("<InitialState> unused key: {}", keycode.unwrap());
                    }
                },

                _ => {}
            }
        }

        next_state.replace(state::NextState::Continue);
    }

    async fn draw(&self, canvas: &mut Canvas<Window>) {
        // draw board lines.
        canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
        for (p1, p2) in self.board_lines.iter() {
            canvas.draw_line(*p1, *p2).unwrap()
        }

        // draw ships.
        canvas.set_draw_color(Color::RGBA(0, 255, 0, 200));

        let x_offset: i32 = (config::WINDOW_WIDTH as i32 - config::WINDOW_HEIGHT as i32) / 2;
        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let x_interval: i32 = min_wh / 10;
        let y_interval: i32 = min_wh / 10;

        let mut cached_rects: Vec<Rect> = Vec::new();

        // cache current ship.
        if self.curr_ship.is_some() {
            for point in self.curr_ship.as_ref().unwrap().body.iter() {
                let rect = Rect::new(
                    point.x * x_interval + x_offset,
                    point.y * y_interval,
                    x_interval as u32,
                    y_interval as u32,
                );
                cached_rects.push(rect);
            }
        }

        // cache board ships.
        for ship in self.ships.iter() {
            for point in ship.body.iter() {
                let rect = Rect::new(
                    point.x * x_interval + x_offset,
                    point.y * y_interval,
                    x_interval as u32,
                    y_interval as u32,
                );
                cached_rects.push(rect);
            }
        }

        // bash draw.
        canvas.fill_rects(&cached_rects[..]).unwrap();
        canvas.draw_rects(&cached_rects[..]).unwrap();
    }
}
