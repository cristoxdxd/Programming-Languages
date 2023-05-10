#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Robot {
    fn new(x: isize, y: isize, direction: Direction) -> Self {
        Robot { x, y, direction }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn advance(&mut self) {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
    }

    fn instructions(&mut self, instructions: &str) {
        for instruction in instructions.chars() {
            match instruction {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => panic!("Invalid instruction"),
            }
        }
    }

    fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    fn direction(&self) -> &Direction {
        &self.direction
    }
}

fn main() {
    let mut robot = Robot::new(7, 3, Direction::North);
    robot.instructions("RAALAL");
    assert_eq!(robot.position(), (9, 4));
    assert_eq!(robot.direction(), &Direction::West);
}