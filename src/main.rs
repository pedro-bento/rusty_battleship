use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

const WINDOW_TITLE: &str = "Rusty Battleship";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const BOARD_LENGTH: u32 = 10;

#[derive(PartialEq, Eq)]
enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
    Submarine,
    PatrolBoat,
}

struct Ship {
    body: [Option<Point>; 5],
}

impl Ship {
    fn new(ship_type: ShipType) -> Ship {
        let body = if ship_type == ShipType::Carrier {
            [
                Some(Point::new(0, 0)),
                Some(Point::new(1, 0)),
                Some(Point::new(2, 0)),
                Some(Point::new(3, 0)),
                Some(Point::new(4, 0)),
            ]
        } else if ship_type == ShipType::Battleship {
            [
                Some(Point::new(0, 0)),
                Some(Point::new(1, 0)),
                Some(Point::new(2, 0)),
                Some(Point::new(3, 0)),
                None,
            ]
        } else if ship_type == ShipType::Destroyer {
            [
                Some(Point::new(0, 0)),
                Some(Point::new(1, 0)),
                Some(Point::new(2, 0)),
                None,
                None,
            ]
        } else if ship_type == ShipType::Submarine {
            [
                Some(Point::new(0, 0)),
                Some(Point::new(1, 0)),
                Some(Point::new(2, 0)),
                None,
                None,
            ]
        } else if ship_type == ShipType::PatrolBoat {
            [
                Some(Point::new(0, 0)),
                Some(Point::new(1, 0)),
                None,
                None,
                None,
            ]
        } else {
            [None, None, None, None, None]
        };

        Ship { body: body }
    }

    fn move_xy(&mut self, dxy: Point) {
        for point in self.body.iter_mut() {
            if point.is_some() {
                let px = point.unwrap().x;
                let py = point.unwrap().y;

                point.replace(Point::new(
                    std::cmp::min(std::cmp::max(px + dxy.x, 0), BOARD_LENGTH as i32),
                    std::cmp::min(std::cmp::max(py + dxy.y, 0), BOARD_LENGTH as i32),
                ));
            }
        }
    }
}

struct Game {
    board_lines: [Point; 44],
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ships: [Ship; 5],
}

impl Game {
    fn new() -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut battleship = Ship::new(ShipType::Battleship);
        battleship.move_xy(Point::new(1, 2));

        let mut destroyer = Ship::new(ShipType::Destroyer);
        destroyer.move_xy(Point::new(2, 4));

        let mut submarine = Ship::new(ShipType::Submarine);
        submarine.move_xy(Point::new(3, 6));

        let mut patroalboat = Ship::new(ShipType::PatrolBoat);
        patroalboat.move_xy(Point::new(4, 8));

        Game {
            board_lines: Game::__get_board_lines(),
            canvas: window.into_canvas().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
            ships: [
                Ship::new(ShipType::Carrier),
                battleship,
                destroyer,
                submarine,
                patroalboat,
            ],
        }
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn handle_events(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
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

    fn draw(&mut self) {
        self.draw_board_lines();
        self.draw_ships();
    }

    fn render(&mut self) {
        self.canvas.present();
    }

    fn draw_ships(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));

        let x_offset: i32 = (WINDOW_WIDTH as i32 - WINDOW_HEIGHT as i32) / 2;
        let min_wh: i32 = std::cmp::min(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

        let x_interval: i32 = min_wh / 10;
        let y_interval: i32 = min_wh / 10;

        for ship in self.ships.iter() {
            for point in ship.body.iter() {
                match point {
                    Some(point) => {
                        let cell = Rect::new(
                            point.x * x_interval + x_offset,
                            point.y * y_interval,
                            x_interval as u32,
                            y_interval as u32,
                        );
                        self.canvas.fill_rect(cell).unwrap();
                        self.canvas.draw_rect(cell).unwrap();
                    }

                    None => {}
                }
            }
        }
    }

    fn draw_board_lines(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));

        for index in (0..self.board_lines.len()).step_by(2) {
            self.canvas
                .draw_line(self.board_lines[index], self.board_lines[index + 1])
                .unwrap();
        }
    }

    fn __get_board_lines() -> [Point; 44] {
        let mut line_points = [Point::new(0, 0); 44];

        let x_offset: i32 = (WINDOW_WIDTH as i32 - WINDOW_HEIGHT as i32) / 2;
        let min_wh: i32 = std::cmp::min(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

        let x_interval: i32 = min_wh / 10;
        let y_interval: i32 = min_wh / 10;

        let mut index = 0;
        for i in 0..=10 {
            // vertical lines
            line_points[index] = Point::new(x_interval * i + x_offset, 0);
            index += 1;

            line_points[index] = Point::new(x_interval * i + x_offset, WINDOW_HEIGHT as i32);
            index += 1;

            // horizontal lines
            line_points[index] = Point::new(x_offset, y_interval * i);
            index += 1;

            line_points[index] = Point::new(WINDOW_WIDTH as i32 - x_offset, y_interval * i);
            index += 1;
        }

        line_points
    }
}

fn main() {
    let mut game = Game::new();

    'game_loop: loop {
        game.clear();

        if game.handle_events() {
            break 'game_loop;
        }

        game.draw();

        game.render();
    }
}
