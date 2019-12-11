use crate::Positions::{Left, Center, Right};
const COUNT: u8 = 3;

enum Positions
{
    Left,
    Right,
    Center,
}

struct Bar
{
    discs: Vec<u8>
}

fn main()
{
    let mut bars: [Bar; 3] = [
        Bar {
            discs: Vec::new()
        },
        Bar {
            discs: Vec::new()
        },
        Bar {
            discs: Vec::new()
        }
    ];

    for i in 0..COUNT - 1 {
        bars[0].discs.push(i.clone());
    }

    move_through(& mut bars, 0, 2);
}

fn move_through(bars: &mut [Bar; 3], from: u8, to: u8) {
    if bars[from as usize].discs.len() == 1 {
        if (bars[to as usize].discs.len() == 0) || (bars[to as usize].discs.last() > bars[from as usize].discs.last()) {
            bars[to as usize].discs.push(bars[from as usize].discs[0]);
        }
    }
}