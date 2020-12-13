use crate::files;
use std::num;

struct Point(i64, i64);

struct Ship {
    position: Point,
    angle: i32
}

impl Ship {
    fn new(position: Option<Point>, angle: Option<i32>) -> Ship {
        Ship {
            position: position.unwrap_or_else(|| Point (0,0)),
            angle: angle.unwrap_or_else(|| 90),
        }
    }

    fn move_ship(&mut self, instruction: &str) {
        let cmd = &instruction[0..1];
        let distance = (&instruction[1..]).parse::<i64>().expect(
            &format!("Couldn't parse {} as number", &instruction[1..]));

        // println!("Command {}, distance {}", cmd, distance);

        match cmd {
            "N" => self.position.1 += distance,
            "S" => self.position.1 -= distance,
            "W" => self.position.0 -= distance,
            "E" => self.position.0 += distance,
            "L" => {
                self.angle = (self.angle - distance as i32);
                if self.angle < 0 {self.angle+= 360}
            },
            "R" => self.move_ship(&format!("L{}", 360 - distance)),
            "F" => {
                match self.angle {
                    90 => self.position.0 += distance,
                    180 => self.position.1 -= distance,
                    270 => self.position.0 -= distance,
                    0 => self.position.1 += distance,
                    _ => panic!("Illegal angle {}", self.angle)
                }
            },
            _ => panic!("Unknown command {}", instruction)
        }

    }

}

pub fn task1() {
    let data = get_input_data();
    let lines = files::str_to_lines(data);

    let mut ship = Ship::new(
        None,None
    );

    for line in &lines {
        ship.move_ship(&line);
    }

    println!("Position of ship: {}, {}; Manhatten distance: {}",
             ship.position.0,
             ship.position.1,
             ship.position.0.abs() + ship.position.1.abs())
}

/// Reads the file into the binary
fn get_input_data() -> &'static str {
    return include_str!("input.txt");
}