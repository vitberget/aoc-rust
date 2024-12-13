use itertools::Itertools as _;

#[derive(Debug)]
pub struct XY { pub x: i128, pub y: i128 }

impl From<&str> for XY {
    fn from(line: &str) -> Self {
        let mut parts = line.split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|word| word[2..].parse::<i128>().unwrap());

        let x = parts.next().unwrap();
        let y = parts.next().unwrap();

        XY { x, y }
    }
}

#[derive(Debug)]
pub struct Machine {
    pub button_a: XY,
    pub button_b: XY,
    pub prize: XY,
}

#[derive(Debug, PartialEq)]
pub struct ButtonPresses {
    pub button_a: i128,
    pub button_b: i128
}

impl ButtonPresses {
    pub fn total_button_presses(&self) -> i128 {
        self.button_a * 3 + self.button_b
    }
}

impl TryFrom<&Machine> for ButtonPresses {
    type Error = String;

    fn try_from(machine: &Machine) -> Result<Self, Self::Error> {
        let a = &machine.button_a;
        let b = &machine.button_b;
        let p = &machine.prize;

        let dividend: i128 = a.x * p.y - a.y * p.x;
        let divisor: i128 = a.x * b.y - a.y * b.x;

        if dividend % divisor == 0 {
            let button_b = dividend / divisor;
            let dividend = p.x - button_b * b.x;
            let divisor = a.x;
            if dividend % divisor == 0 {
                let button_a = dividend / divisor;
                Ok(ButtonPresses { button_a, button_b })
            } else {
                Err("Second part not whole number".to_string())
            }
        } else {
            Err("First part not whole number".to_string())
        }

    }
}

#[derive(Debug)]
pub struct Arcade {
    pub machines: Vec<Machine>
}

impl Arcade {
    pub(crate) fn adjust_conversion_error(&mut self, conversion_error: i128) {
        self.machines.iter_mut()
            .for_each(|machine| {
                machine.prize.x += conversion_error;
                machine.prize.y += conversion_error;
            })
    }
    pub(crate) fn press_buttons(&self) -> Vec<Option<ButtonPresses>> {
        self.machines
            .iter()
            .map(|machine| machine.try_into().ok())
            .collect()
    }
}

impl From<&str> for Arcade {
    fn from(text: &str) -> Self {
        let machines: Vec<Machine> = text.lines()
            .chunks(4)
            .into_iter()
            .map(|mut lines| {
                let button_a = lines.next().unwrap();
                let button_b = lines.next().unwrap();
                let prize = lines.next().unwrap();

                Machine { 
                    button_a: button_a.into(),
                    button_b: button_b.into(),
                    prize: prize.into()
                }
            }).collect();

        Arcade { machines }
    }
}
