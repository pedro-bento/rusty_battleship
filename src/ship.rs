use sdl2::rect::Point;

use super::config;

#[derive(PartialEq, Eq, Copy, Clone)]
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

impl Clone for Ship {
  fn clone(&self) -> Ship {
    Ship {
      body: self.body.clone(),
    }
  }
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

    // is inside map borders?
    fn is_valid_move(&self, dxy: &Point) -> bool {
        for p in self.body.iter() {
            let new_x = p.x + dxy.x;
            let new_y = p.y + dxy.y;

            if new_x < 0
                || new_x >= config::BOARD_LENGTH as i32
                || new_y < 0
                || new_y >= config::BOARD_LENGTH as i32
            {
                return false;
            }
        }

        return true;
    }

    pub fn move_xy(&mut self, dxy: &Point) {
        let new_point = |p: &Point| Point::new(p.x + dxy.x, p.y + dxy.y);

        if self.is_valid_move(dxy) {
            self.body = self.body.iter().map(|p| new_point(p)).collect();
        }
    }
}
