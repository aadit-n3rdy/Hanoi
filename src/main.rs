struct Bar {
    discs: Vec<u8>,
    error: String
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let COUNT:u8 = args[1].parse().unwrap();
    
    let mut bars: [Bar; 3] = [
        Bar {
            discs: Vec::new(),
            error: "".to_string()
        },
        Bar {
            discs: Vec::new(),
            error: "".to_string()
        },
        Bar {
            discs: Vec::new(),
            error: "".to_string()
        }
    ];
    for i in  0..COUNT{
        bars[0].discs.push(i);
    }

    move_through(& mut bars, 0, 2, COUNT);

    println!("Done!! Final Positions:");
    for i in 0..bars[2].discs.len() {
        println!("{}", i);
    }
}

fn move_through(mut bars: &mut [Bar; 3], from: usize, to: usize, from_count: u8) -> &mut [Bar; 3] { 
    for i in bars.iter() {
        if i.error != "".to_string() {
            panic!(i.error.clone());
        }
    }

    if bars[to].discs.len() > 0 {
        if bars[from].discs[(from_count - 1) as usize] > bars[to].discs[0] {
            panic!("Larger disk on smaller disk at bar ".to_string() + &to.to_string());
        } 
    }
    
    
    if from_count == 1 {
        if bars[to].discs.len() == 0 || 
        bars[to].discs[0] > bars[from].discs[0] {
            bars[to].discs.insert(0, bars[from].discs[0]);
            bars[from].discs.remove(0);
            println!("Moving disc {} from {} to {}", bars[to].discs[0], from, to);
            return bars;
        }
        else {
            bars[to].error = "Larger disk on smaller disk at bar ".to_string() + &to.to_string();
            return bars;
        }
    }
    else {
        let mut through :usize = 0;
        
        for i in 0..3 {
            if i!= from && i!= to {
                through = i;
            }
        }
        
        bars = move_through(bars, from.clone(), through.clone(), from_count - 1);
        
        
        bars = move_through(bars, from.clone(), to.clone(), 1);
        
        bars = move_through(bars, through.clone(), to.clone(), from_count - 1);
        
    }
    return bars;
    
}