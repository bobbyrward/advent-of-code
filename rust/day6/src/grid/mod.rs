use std::cmp::max;

#[derive(Clone, Copy)]
pub enum LightOp {
    TurnOn,
    TurnOff,
    Toggle,
}

pub struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Coord {
        Coord{x: x, y: y}
    }
}

pub struct LightCommand {
    op: LightOp,
    start: Coord,
    stop: Coord,
}

impl LightCommand {
    pub fn new(op: LightOp, start: Coord, stop: Coord) -> LightCommand {
        LightCommand {
            op: op,
            start: start,
            stop: stop,
        }
    }
}

trait ApplyLightOp {
    fn apply(&mut self, op: LightOp);
}

impl ApplyLightOp for u8 {
    fn apply(&mut self, op: LightOp) {
        match op {
            LightOp::TurnOn => *self += 1,
            LightOp::Toggle => *self += 2,
            LightOp::TurnOff => *self = max(1, *self) - 1,
        };
    }
}

impl ApplyLightOp for bool {
    fn apply(&mut self, op: LightOp) {
        match op {
            LightOp::TurnOn => *self = true,
            LightOp::TurnOff => *self = false,
            LightOp::Toggle => *self = !*self,
        };
    }
}

pub struct LightGrid<T> {
    pub lights: Vec<T>,
    width: u32,
}

impl <T: Clone + ApplyLightOp> LightGrid<T> {
    pub fn new(initial_value: T, width: u32, height: u32) -> LightGrid<T> {
        LightGrid{
            lights: vec![initial_value; (width * height) as usize],
            width: width,
        }
    }

    fn process_row_command(&mut self, row: u32, start: u32, stop: u32, op: LightOp) {
        let light_start = row * self.width + start;
        let light_stop = row * self.width + stop;

        for idx in (light_start..light_stop) {
            self.lights[idx as usize].apply(op);
        }
    }

    pub fn process_command(&mut self, command: &LightCommand) {
        for row in (command.start.y..command.stop.y+1) {
            self.process_row_command(row, command.start.x, command.stop.x+1, command.op);
        }
    }
}

#[test]
fn test_create() {
    let mut grid = LightGrid::new(false, 10, 10);

    grid.process_command(&LightCommand::new(
        LightOp::TurnOn,
        Coord::new(0, 0),
        Coord::new(9, 9),
    ));

    assert_eq!(
        vec![true; 10*10],
        grid.lights
    );

    grid.process_command(&LightCommand::new(
        LightOp::TurnOff,
        Coord::new(0, 0),
        Coord::new(9, 9),
    ));

    assert_eq!(
        vec![false; 10*10],
        grid.lights
    );

    grid.process_command(&LightCommand::new(
        LightOp::Toggle,
        Coord::new(0, 0),
        Coord::new(9, 9),
    ));

    assert_eq!(
        vec![true; 10*10],
        grid.lights
    );

    grid.process_command(&LightCommand::new(
        LightOp::Toggle,
        Coord::new(0, 1),
        Coord::new(9, 1),
    ));

    assert_eq!(
        vec![
            true, true, true, true, true, true, true, true, true, true,
            false, false, false, false, false, false, false, false, false, false,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true,
        ],
        grid.lights
    );
}
