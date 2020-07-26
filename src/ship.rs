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

    fn is_valid_rotate(&self) -> bool {
        let mid_point = self.body.get(self.body.len() / 2).unwrap();

        for p in self.body.iter() {
            let aux_x = p.x - mid_point.x;
            let aux_y = p.y - mid_point.y;

            let aux_new_x = -aux_y;
            let aux_new_y = -aux_x;

            let new_x = aux_new_x + mid_point.x;
            let new_y = aux_new_y + mid_point.y;

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

    // just left rotation for now.
    // if needed implement new_point_rotR (just swap the '-' sign from x to y coord).
    pub fn rotate(&mut self) {
        let mid_point = self.body.get(self.body.len() / 2).unwrap();

        let new_point_rotl = |p: &Point| {
            let aux_x = p.x - mid_point.x;
            let aux_y = p.y - mid_point.y;

            let aux_new_x = -aux_y;
            let aux_new_y = -aux_x;

            let new_x = aux_new_x + mid_point.x;
            let new_y = aux_new_y + mid_point.y;

            return Point::new(new_x, new_y);
        };

        if self.is_valid_rotate() {
          self.body = self.body.iter().map(|p| new_point_rotl(p)).collect();
        }
    }
}
