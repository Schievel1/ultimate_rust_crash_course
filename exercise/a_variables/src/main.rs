const STARTINGS_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 3;

fn main() {
    let (missiles , ready)  = (STARTINGS_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles-ready);
}
