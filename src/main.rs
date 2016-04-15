extern crate rand;
use rand::Rng;

fn main() {
	const ROLLS: i64 = 10000000;
    let mut rng  = rand::XorShiftRng::new_unseeded();
    let mut sumdist = vec![0; 11];
    let mut die1dist = vec![0; 6];
    let mut die2dist = vec![0; 6];
    let mut dice = vec![0,0,0];
    let mut dval1 :u32;
    let mut dval2 :u32;
    let mut dicedist: [[u32; 6]; 6];
    let mut dicedist: [[u32; 6]; 6] = [[0,0,0,0,0,0],
    									[0,0,0,0,0,0],
    									[0,0,0,0,0,0],
    									[0,0,0,0,0,0],
    									[0,0,0,0,0,0],
    									[0,0,0,0,0,0]];
    for _ in 0..ROLLS {
		dice[0] = rng.gen::<u32>();
		dice[1] = rng.gen::<u32>();
		dice[2] = rng.gen::<u32>();
        dice.sort();
        dval1 = (dice[0]^dice[1])%6;
        dval2 = (dice[2]^dice[1])%6;
        die1dist[dval1 as usize] +=1;
        die2dist[dval2 as usize] +=1;
		sumdist[(dval1+dval2) as usize]+=1;
		dicedist[dval1 as usize][dval2 as usize] +=1
    }
    println!("Die 1 Distribution");
    for (i,x) in die1dist.iter().enumerate() {
        print!("{}|{}  ",i+1, *x as f64/ROLLS as f64);
    }
    println!("\n\nDie 2 Distribution");
    for (i,x) in die2dist.iter().enumerate() {
        print!("{}|{}  ",i+1, *x as f64/ROLLS as f64);
    }

    println!("\n\nDie Dist Spectrum \n{:?}",dicedist );
    println!("\nDie Sum Dist");
    for (i,x) in sumdist.iter().enumerate() {
        print!("{}|{}  ",i+2, *x as f64/ROLLS as f64);
    }
}
