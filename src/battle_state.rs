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
use mini_redis::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::chat;
use super::config;
use super::ship;
use super::state;

pub struct BattleState {
    board_lines: Vec<(Point, Point)>,
    my_ships: Vec<ship::Ship>,
    my_shot: Point,

    chat: Arc<Mutex<chat::Chat>>,
}

impl BattleState {
    pub async fn new(
        board_lines: Vec<(Point, Point)>,
        my_ships: Vec<ship::Ship>,
    ) -> Result<BattleState> {
        Ok(BattleState {
            board_lines: board_lines,
            my_ships: my_ships,
            my_shot: Point::new(
                (config::BOARD_LENGTH / 2) as i32,
                (config::BOARD_LENGTH / 2) as i32,
            ),

            // TODO:make sure to swap channels for different players
            chat: Arc::new(Mutex::new(
                chat::Chat::new(&"127.0.0.1:6379".to_string(), "1", "1").await?,
            )),
        })
    }

    fn is_valid_shot_move(&self, dxy: &Point) -> bool {
        if self.my_shot.x + dxy.x < 0
            || self.my_shot.x + dxy.x >= config::BOARD_LENGTH as i32
            || self.my_shot.y + dxy.y < 0
            || self.my_shot.y + dxy.y >= config::BOARD_LENGTH as i32
        {
            return false;
        }

        return true;
    }

    fn move_shot(&mut self, dxy: &Point) {
        if self.is_valid_shot_move(dxy) {
            self.my_shot.x += dxy.x;
            self.my_shot.y += dxy.y;
        }
    }
}

#[async_trait(?Send)]
impl state::State for BattleState {
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
                    Some(Keycode::Escape) => {
                        next_state.replace(state::NextState::Quit);
                        return;
                    }

                    // TODO:
                    Some(Keycode::Return) => {
                        // send shot info across network
                        // socket.write("<pox.x> <pos.y>")
                        // socket.recv()
                        //    should only continue playing after oppenent as sent shot.

                        let chat = self.chat.clone();
                        tokio::spawn(async move {
                            let mut chat = chat.lock().await;
                            let _ = chat.send("SHOT".into()).await;
                        });
                    }

                    // JUST FOR TEST
                    Some(Keycode::R) => {
                        let chat = self.chat.clone();
                        tokio::spawn(async move {
                            let mut chat = chat.lock().await;
                            if let Ok(Some(msg)) = chat.receive().await {
                                println!("{:?}", msg.content);
                            }
                        });
                    }

                    Some(Keycode::W) | Some(Keycode::Up) => {
                        self.move_shot(&Point::new(0, -1));
                    }

                    Some(Keycode::S) | Some(Keycode::Down) => {
                        self.move_shot(&Point::new(0, 1));
                    }

                    Some(Keycode::A) | Some(Keycode::Left) => {
                        self.move_shot(&Point::new(-1, 0));
                    }

                    Some(Keycode::D) | Some(Keycode::Right) => {
                        self.move_shot(&Point::new(1, 0));
                    }

                    _ => {
                        println!("<BattleState> unused key: {}", keycode.unwrap());
                    }
                },

                _ => {}
            }
        }

        next_state.replace(state::NextState::Continue);
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        // draw board lines.
        canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));
        for (p1, p2) in self.board_lines.iter() {
            canvas.draw_line(*p1, *p2).unwrap()
        }

        // draw my shot.
        canvas.set_draw_color(Color::RGBA(10, 255, 0, 150));

        let x_offset: i32 = (config::WINDOW_WIDTH as i32 - config::WINDOW_HEIGHT as i32) / 2;
        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let x_interval: i32 = min_wh / 10;
        let y_interval: i32 = min_wh / 10;

        let rect = Rect::new(
            self.my_shot.x * x_interval + x_offset,
            self.my_shot.y * y_interval,
            x_interval as u32,
            y_interval as u32,
        );

        canvas.fill_rect(rect).unwrap();
        canvas.draw_rect(rect).unwrap();
    }
}
