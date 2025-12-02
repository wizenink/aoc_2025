use std::error::Error;

enum Direction {
    RIGHT,
    LEFT,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'r' | 'R' => Direction::RIGHT,
            'l' | 'L' => Direction::LEFT,
            _ => panic!("Unknown direction"),
        }
    }
}

struct Rotation {
    direction: Direction,
    amount: i32,
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        Self {
            direction: value.chars().nth(0).unwrap().into(),
            amount: value[1usize..].parse::<i32>().unwrap(),
        }
    }
}

impl From<Rotation> for i32 {
    fn from(value: Rotation) -> Self {
        match value.direction {
            Direction::RIGHT => value.amount,
            Direction::LEFT => -value.amount,
        }
    }
}

struct Dial {
    pub state: i32,
    pub zero_count: i32,
}

impl Dial {
    fn rotate(&mut self, rotation: Rotation) {
        let crosses = match rotation.direction {
            Direction::RIGHT => (self.state + rotation.amount) / 100,
            Direction::LEFT => {
                if self.state == 0 {
                    rotation.amount / 100
                } else if rotation.amount < self.state {
                    0
                } else {
                    1 + (rotation.amount - self.state) / 100
                }
            }
        };

        self.zero_count += crosses;
        let amount: i32 = rotation.into();
        self.state = (self.state + amount).rem_euclid(100);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let mut dial = Dial {
        state: 50,
        zero_count: 0,
    };
    let input: Vec<Rotation> =
        std::fs::read_to_string(args.nth(1).unwrap_or("input.txt".to_owned()))?
            .lines()
            .map(Rotation::from)
            .collect();

    for rotation in input {
        dial.rotate(rotation);
    }

    println!("Password: {}", dial.zero_count);
    Ok(())
}
