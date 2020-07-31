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

use super::config;
use super::state;

pub struct StatsState {
    board_lines: Vec<(Point, Point)>,

    opponent_hit_shots: Vec<Point>,
    opponent_miss_shots: Vec<Point>,

    my_hit_shots: Vec<Point>,
    my_miss_shots: Vec<Point>,
}

impl StatsState {
    pub fn new(
        opponent_hit_shots: Vec<Point>,
        opponent_miss_shots: Vec<Point>,
        my_hit_shots: Vec<Point>,
        my_miss_shots: Vec<Point>,
    ) -> StatsState {
        StatsState {
            board_lines: StatsState::generate_board_lines(),
            
            opponent_hit_shots: opponent_hit_shots,
            opponent_miss_shots: opponent_miss_shots,

            my_hit_shots: my_hit_shots,
            my_miss_shots: my_miss_shots,
        }
    }

    fn generate_board_lines() -> Vec<(Point, Point)> {
        let mut line_points: Vec<(Point, Point)> =
            Vec::with_capacity(2 * ((config::BOARD_LENGTH + 1) * 2) as usize);

        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let offset: i32 = min_wh / 2;
        let y_offset: i32 = offset / 2;

        let x_interval: i32 = offset / 10;
        let y_interval: i32 = offset / 10;

        for i in 0..=config::BOARD_LENGTH {
            // left vertical lines.
            line_points.push((
                Point::new(x_interval * i as i32 + x_interval, y_offset),
                Point::new(
                    x_interval * i as i32 + x_interval,
                    config::WINDOW_HEIGHT as i32 - y_offset,
                ),
            ));

            // left horizontal lines.
            line_points.push((
                Point::new(x_interval, y_interval * i as i32 + y_offset),
                Point::new(
                    offset + x_interval,
                    y_interval * i as i32 + y_offset,
                ),
            ));

            // right vertical lines.
            line_points.push((
                Point::new(x_interval * i as i32 + config::WINDOW_WIDTH as i32 - offset - x_interval, y_offset),
                Point::new(
                    x_interval * i as i32 + config::WINDOW_WIDTH as i32 - offset - x_interval,
                    config::WINDOW_HEIGHT as i32 - y_offset,
                ),
            ));

            // right horizontal lines.
            line_points.push((
                Point::new(config::WINDOW_WIDTH as i32 - offset - x_interval, y_interval * i as i32 + y_offset),
                Point::new(
                    config::WINDOW_WIDTH as i32 - x_interval,
                    y_interval * i as i32 + y_offset,
                ),
            ));
        }

        line_points
    }

    fn draw_shots_left(
        &self,
        canvas: &mut Canvas<Window>,
        color: Color,
        shots: &Vec<Point>,
    ) {
        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let offset: i32 = min_wh / 2;
        let y_offset: i32 = offset / 2;

        let x_interval: i32 = offset / 10;
        let y_interval: i32 = offset / 10;

        canvas.set_draw_color(color);

        let mut cache: Vec<Rect> = Vec::new();

        for point in shots.iter() {
            let rect = Rect::new(
                point.x * x_interval + x_interval,
                point.y * y_interval + y_offset,
                x_interval as u32,
                y_interval as u32,
            );
            cache.push(rect);
        }

        canvas.fill_rects(&cache[..]).unwrap();
        canvas.draw_rects(&cache[..]).unwrap();
    }

    fn draw_shots_right(
        &self,
        canvas: &mut Canvas<Window>,
        color: Color,
        shots: &Vec<Point>,
    ) {
        let min_wh: i32 = std::cmp::min(config::WINDOW_WIDTH as i32, config::WINDOW_HEIGHT as i32);

        let offset: i32 = min_wh / 2;
        let y_offset: i32 = offset / 2;

        let x_interval: i32 = offset / 10;
        let y_interval: i32 = offset / 10;

        canvas.set_draw_color(color);

        let mut cache: Vec<Rect> = Vec::new();

        for point in shots.iter() {
            let rect = Rect::new(
                point.x * x_interval + config::WINDOW_WIDTH as i32 - offset - x_interval,
                point.y * y_interval + y_offset,
                x_interval as u32,
                y_interval as u32,
            );
            cache.push(rect);
        }

        canvas.fill_rects(&cache[..]).unwrap();
        canvas.draw_rects(&cache[..]).unwrap();
    }
}

#[async_trait(?Send)]
impl state::State for StatsState {
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
                        
                    }

                    Some(Keycode::W) | Some(Keycode::Up) => {
                        
                    }

                    Some(Keycode::S) | Some(Keycode::Down) => {
                        
                    }

                    Some(Keycode::A) | Some(Keycode::Left) => {
                        
                    }

                    Some(Keycode::D) | Some(Keycode::Right) => {
                        
                    }

                    Some(Keycode::Q) => {
                        
                    }

                    Some(Keycode::E) => {
                        
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

        self.draw_shots_left(canvas, Color::RGBA(0, 255, 0, 200), &self.my_hit_shots);
        self.draw_shots_left(canvas, Color::RGBA(0, 0, 255, 200), &self.my_miss_shots);

        self.draw_shots_right(canvas, Color::RGBA(255, 0, 0, 200), &self.opponent_hit_shots);
        self.draw_shots_right(canvas, Color::RGBA(0, 0, 255, 200), &self.opponent_miss_shots);
    }
}
