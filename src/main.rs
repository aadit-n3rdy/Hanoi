use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

struct Bar {
    discs: Vec<u8>,
    error: String
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let count:u8 = args[1].parse().unwrap();

    let file_name: String;

    if cfg!(windows) {
        let key = "%HOMEPATH%";
        match env::var(key) {
            Ok(val) => {
                file_name = format!("{}/Documents/hanoi_solutions_{}.txt", val, count);
            }
            Err(e) => {
                panic!("e");
            }
        }
        
    }
    else {
        let key = "HOME";
        match env::var(key) {
            Ok(val) => {
                file_name = format!("{}/Documents/hanoi_solutions_{}.txt", val, count);
            }
            Err(e) => {
                panic!(e);
            }
        }
    }    

    let _file = File::create(file_name.clone())
        .expect("Could not create output file!!");
    
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
    for i in  0..count{
        bars[0].discs.push(i);
    }

    let start = std::time::SystemTime::now();

    move_through(& mut bars, 0, 2, count, file_name.clone());

    let diff = start.elapsed();

    println!("Done!! Final Positions:");
    for i in 0..bars[2].discs.len() {
        println!("{}", bars[2].discs[i]);
    }

    match diff {
        Ok(elapsed) => {
            println!("Time taken: {}", elapsed.as_secs_f64());
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn move_through(mut bars: &mut [Bar; 3], from: usize, to: usize, from_count: u8, file_name: String) -> &mut [Bar; 3] { 

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

            let mut file = OpenOptions::new().append(true).open(file_name)
                .expect("Could not open file in recursion!!");
            println!("Moving disc {} from {} to {}", bars[to].discs[0], from, to);
            file.write(format!("Moving disk {} from {} to {}\n", bars[to].discs[0] + 1, from + 1, to + 1).as_bytes())
                .expect("Could not write to file in recursion!!");
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
        
        bars = move_through(bars, from.clone(), through.clone(), from_count - 1, file_name.clone());
        
        
        bars = move_through(bars, from.clone(), to.clone(), 1, file_name.clone());
        
        bars = move_through(bars, through.clone(), to.clone(), from_count - 1, file_name.clone());
        
    }
    return bars;
    
}