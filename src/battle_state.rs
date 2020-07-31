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
use tokio::task::JoinHandle;

use super::chat;
use super::config;
use super::ship;
use super::state;
use super::stats_state;

type ServerHandle = JoinHandle<mini_redis::Result<()>>;

pub struct BattleState {
    board_lines: Vec<(Point, Point)>,
    my_ship_points: Arc<Mutex<Vec<Point>>>,
    my_shot: Point,

    opponent_hit_shots: Arc<Mutex<Vec<Point>>>,
    opponent_miss_shots: Arc<Mutex<Vec<Point>>>,

    my_hit_shots: Arc<Mutex<Vec<Point>>>,
    my_miss_shots: Arc<Mutex<Vec<Point>>>,

    chat: Arc<Mutex<chat::Chat>>,

    server_handle: Option<ServerHandle>,

    is_quit: Arc<Mutex<bool>>,
    is_init: bool,
    is_send_shot: Arc<Mutex<bool>>,
    is_recieve_shot: Arc<Mutex<bool>>,
}

impl BattleState {
    pub async fn new(
        board_lines: Vec<(Point, Point)>,
        my_ships: Vec<ship::Ship>,
        addr: String,
        server_handle: Option<ServerHandle>,
        receive_channel_key: String,
        send_channel_key: String,
    ) -> Result<BattleState> {
        let mut my_ship_points: Vec<Point> = Vec::new();
        for ship in my_ships.iter() {
            for ship_point in ship.body.iter() {
                my_ship_points.push(*ship_point);
            }
        }

        Ok(BattleState {
            board_lines: board_lines,
            my_ship_points: Arc::new(Mutex::new(my_ship_points)),
            my_shot: Point::new(
                (config::BOARD_LENGTH / 2) as i32,
                (config::BOARD_LENGTH / 2) as i32,
            ),

            opponent_hit_shots: Arc::new(Mutex::new(Vec::new())),
            opponent_miss_shots: Arc::new(Mutex::new(Vec::new())),

            my_hit_shots: Arc::new(Mutex::new(Vec::new())),
            my_miss_shots: Arc::new(Mutex::new(Vec::new())),

            chat: Arc::new(Mutex::new(
                chat::Chat::new(addr, receive_channel_key, send_channel_key).await?,
            )),

            server_handle: server_handle,

            is_quit: Arc::new(Mutex::new(false)),
            // this is replaced on first iteration.
            is_init: false,
            is_send_shot: Arc::new(Mutex::new(false)),
            is_recieve_shot: Arc::new(Mutex::new(false)),
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

    async fn rcv_snd(&mut self) {
        let chat = self.chat.clone();
        let is_quit = self.is_quit.clone();
        let is_send_shot = self.is_send_shot.clone();
        let opponent_miss_shots = self.opponent_miss_shots.clone();
        let opponent_hit_shots = self.opponent_hit_shots.clone();
        let my_ship_points = self.my_ship_points.clone();

        tokio::spawn(async move {
            let mut chat = chat.lock().await;
            if let Ok(Some(msg)) = chat.receive().await {
                // parse the recieved shot.
                let msg_str = format!("{:?}", msg.content)
                    .strip_prefix("b\"")
                    .unwrap()
                    .to_string()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_string();

                let words: Vec<&str> = msg_str.split(" ").collect();
                let x: i32 = words[0].parse().unwrap();
                let y: i32 = words[1].parse().unwrap();

                let shot = Point::new(x, y);

                // verify hit.
                let my_ship_points = my_ship_points.lock().await;

                let mut is_hit = false;
                for ship_point in my_ship_points.iter() {
                    if shot == *ship_point {
                        is_hit = true;
                        break;
                    }
                }

                // Cache value and send STAT.
                if is_hit {
                    let mut opponent_hit_shots = opponent_hit_shots.lock().await;

                    // make sure is not repeated.
                    if !opponent_hit_shots.contains(&shot) {
                        opponent_hit_shots.push(shot);
                    }

                    // make sure is not game over.
                    if my_ship_points.len() == opponent_hit_shots.len() {
                        let mut is_quit = is_quit.lock().await;
                        *is_quit = true;

                        let _ = chat.send("GAMEOVER".into()).await;
                    } else {
                        let _ = chat.send("HIT".into()).await;
                    }
                } else {
                    let mut opponent_miss_shots = opponent_miss_shots.lock().await;

                    // make sure is not repeated.
                    if !opponent_miss_shots.contains(&shot) {
                        opponent_miss_shots.push(shot);
                    }

                    let _ = chat.send("MISS".into()).await;
                }

                drop(my_ship_points);

                // we can send a shot now.
                let mut is_send_shot = is_send_shot.lock().await;
                *is_send_shot = true;
            }
        });
    }

    async fn snd_rcv(&mut self, msg: String) {
        let chat = self.chat.clone();
        let is_quit = self.is_quit.clone();
        let is_recieve_shot = self.is_recieve_shot.clone();
        let my_miss_shots = self.my_miss_shots.clone();
        let my_hit_shots = self.my_hit_shots.clone();

        tokio::spawn(async move {
            let msg2 = msg.clone();

            let mut chat = chat.lock().await;
            let _ = chat.send(msg.into()).await;

            let words: Vec<&str> = msg2.split(" ").collect();
            let x: i32 = words[0].parse().unwrap();
            let y: i32 = words[1].parse().unwrap();
            let shot = Point::new(x, y);

            // recieve stat.
            if let Ok(Some(msg)) = chat.receive().await {
                // no neeed to strip because we're using 'contains'
                let msg_str = format!("{:?}", msg.content);

                if msg_str.contains("HIT") {
                    let mut my_hit_shots = my_hit_shots.lock().await;
                    if !my_hit_shots.contains(&shot) {
                        my_hit_shots.push(shot);
                    }
                } else if msg_str.contains("MISS") {
                    let mut my_miss_shots = my_miss_shots.lock().await;
                    if !my_miss_shots.contains(&shot) {
                        my_miss_shots.push(shot);
                    }
                } else if msg_str.contains("GAMEOVER") {
                    let mut is_quit = is_quit.lock().await;
                    *is_quit = true;
                }

                // we can recieve a shot now.
                let mut is_recieve_shot = is_recieve_shot.lock().await;
                *is_recieve_shot = true;
            }
        });
    }

    async fn update(&mut self) {
        let is_send_shot = self.is_send_shot.clone();
        let is_recieve_shot = self.is_recieve_shot.clone();

        let mut is_send_shot = is_send_shot.lock().await;
        let mut is_recieve_shot = is_recieve_shot.lock().await;

        // make sure we're ready.
        if !self.is_init {
            self.is_init = true;
            match self.server_handle {
                Some(_) => {
                    *is_send_shot = true;
                    *is_recieve_shot = false;
                }

                None => {
                    *is_send_shot = false;
                    *is_recieve_shot = true;
                }
            }
        }

        if *is_recieve_shot {
            *is_recieve_shot = false;

            drop(is_send_shot);
            drop(is_recieve_shot);

            self.rcv_snd().await;
        }
    }

    async fn draw_shots(
        &self,
        canvas: &mut Canvas<Window>,
        color: Color,
        shots: &Arc<Mutex<Vec<Point>>>,
    ) {
        let x_offset: i32 = (config::WINDOW_WIDTH as i32 - config::WINDOW_HEIGHT as i32) / 2;
        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let x_interval: i32 = min_wh / 10;
        let y_interval: i32 = min_wh / 10;

        canvas.set_draw_color(color);

        let mut cache: Vec<Rect> = Vec::new();

        let shots = shots.clone();
        let shots = shots.lock().await;

        for point in shots.iter() {
            let rect = Rect::new(
                point.x * x_interval + x_offset,
                point.y * y_interval,
                x_interval as u32,
                y_interval as u32,
            );
            cache.push(rect);
        }

        drop(shots);

        canvas.fill_rects(&cache[..]).unwrap();
        canvas.draw_rects(&cache[..]).unwrap();
    }
}

#[async_trait(?Send)]
impl state::State for BattleState {
    async fn handle_events(
        &mut self,
        event_pump: &mut EventPump,
        next_state: &mut Option<state::NextState>,
    ) {
        self.update().await;

        let is_quit = self.is_quit.clone();
        let is_quit = is_quit.lock().await;
        if *is_quit {
            let opponent_hit_shots = self.opponent_hit_shots.clone();
            let opponent_hit_shots = opponent_hit_shots.lock().await;

            let opponent_miss_shots = self.opponent_miss_shots.clone();
            let opponent_miss_shots = opponent_miss_shots.lock().await;

            let my_hit_shots = self.my_hit_shots.clone();
            let my_hit_shots = my_hit_shots.lock().await;

            let my_miss_shots = self.my_miss_shots.clone();
            let my_miss_shots = my_miss_shots.lock().await;

            next_state.replace(state::NextState::Update(
                Box::new(stats_state::StatsState::new(
                    opponent_hit_shots.clone(),
                    opponent_miss_shots.clone(),
                    my_hit_shots.clone(),
                    my_miss_shots.clone(),
                ))
            ));
            return;
        }

        drop(is_quit);

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

                    Some(Keycode::Return) => {
                        let is_send_shot = self.is_send_shot.clone();
                        let mut is_send_shot = is_send_shot.lock().await;

                        if *is_send_shot {
                            *is_send_shot = false;
                            self.snd_rcv(format!("{} {}", self.my_shot.x, self.my_shot.y))
                                .await;
                        }

                        drop(is_send_shot);
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

    async fn draw(&self, canvas: &mut Canvas<Window>) {
        let x_offset: i32 = (config::WINDOW_WIDTH as i32 - config::WINDOW_HEIGHT as i32) / 2;
        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let x_interval: i32 = min_wh / 10;
        let y_interval: i32 = min_wh / 10;

        // draw board lines.
        canvas.set_draw_color(Color::RGBA(0, 255, 0, 255));

        for (p1, p2) in self.board_lines.iter() {
            canvas.draw_line(*p1, *p2).unwrap()
        }

        // draw all cached shots
        self.draw_shots(
            canvas,
            Color::RGBA(0, 0, 255, 30),
            &self.opponent_miss_shots,
        )
        .await;
        self.draw_shots(canvas, Color::RGBA(255, 0, 0, 30), &self.opponent_hit_shots)
            .await;
        self.draw_shots(canvas, Color::RGBA(0, 0, 255, 255), &self.my_miss_shots)
            .await;
        self.draw_shots(canvas, Color::RGBA(255, 0, 0, 255), &self.my_hit_shots)
            .await;

        // draw my shot.
        let is_send_shot = self.is_send_shot.clone();
        let is_send_shot = is_send_shot.lock().await;

        if *is_send_shot {
            canvas.set_draw_color(Color::RGBA(0, 255, 0, 150));
        } else {
            canvas.set_draw_color(Color::RGBA(255, 255, 255, 50));
        }

        drop(is_send_shot);

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
