#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W
}

#[derive(Debug)]
pub struct Position(pub i32, pub i32);

struct MarsRover {
    position: Position, 
    facing: Direction,
}

trait MarsRoverCommand {
    fn movement(&mut self); 
    fn process(&mut self, text: String); 
    fn turn_left(&mut self); 
    fn turn_right(&mut self); 
    fn set_position(&mut self, x: i32, y: i32, facing: Direction); 
    fn print_position(&mut self); 
}

impl MarsRoverCommand for MarsRover {
    
    fn movement(&mut self) {
        self.position = match self.facing {
            Direction::N => Position(self.position.0, self.position.1 + 1),
            Direction::E => Position(self.position.0 + 1, self.position.1),  
            Direction::S => Position(self.position.0, self.position.1 - 1),
            Direction::W => Position(self.position.0 - 1, self.position.1),
        }; 
    }

    fn process(&mut self, commands: String) {
        for command in commands.chars() {
              
            match command {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                'M' => self.movement(),
                _ => println!("Invalid command")
        };

        }
    }

    fn turn_left(&mut self){
        self.facing = match self.facing {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N
        };  
    }

    fn set_position(&mut self,  x: i32, y: i32, facing: Direction) {
        self.position = Position(x, y);
        self.facing = facing;
    }

    fn print_position(&mut self) {
        println!("{:?} {:?}", self.position, self.facing);
    }
}



fn main() {

    let mut rover = MarsRover {
        position: Position(0, 0),
        facing: Direction::N
    };

    rover.set_position(1, 2,  Direction::N);
    rover.process("LMLMLMLMM".to_string());
    rover.print_position();
    rover.set_position(3, 3,  Direction::E);
    rover.process("MMRMMRMRRM".to_string());
    rover.print_position();
}
