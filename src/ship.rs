use sdl2::rect::Point;

use super::config;

#[derive(PartialEq, Eq)]
pub enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
    Submarine,
    PatrolBoat,
}

pub struct Ship {
    pub body: Vec<Point>,
}

impl Ship {
    pub fn new(ship_type: ShipType) -> Ship {
        let mut body: Vec<Point> = Vec::with_capacity(5);

        match ship_type {
            ShipType::Carrier => {
                body.push(Point::new(0, 0));
                body.push(Point::new(1, 0));
                body.push(Point::new(2, 0));
                body.push(Point::new(3, 0));
                body.push(Point::new(4, 0));
            }

            ShipType::Battleship => {
                body.push(Point::new(0, 0));
                body.push(Point::new(1, 0));
                body.push(Point::new(2, 0));
                body.push(Point::new(3, 0));
            }

            ShipType::Destroyer | ShipType::Submarine => {
                body.push(Point::new(0, 0));
                body.push(Point::new(1, 0));
                body.push(Point::new(2, 0));
            }

            ShipType::PatrolBoat => {
                body.push(Point::new(0, 0));
                body.push(Point::new(1, 0));
            }
        }

        Ship { body: body }
    }

    pub fn move_xy(&mut self, dxy: Point) {
        let new_point = |p: &Point| {
            Point::new(
                std::cmp::min(std::cmp::max(p.x + dxy.x, 0), config::BOARD_LENGTH as i32),
                std::cmp::min(std::cmp::max(p.y + dxy.y, 0), config::BOARD_LENGTH as i32),
            )
        };

        self.body = self.body.iter().map(|p| new_point(p)).collect();
    }
}
