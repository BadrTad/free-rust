mod kitchen;


use crate::kitchen::plates::Tools;


fn main() {
    let fork = Tools::Fork;
    let plate = Tools::Plate;
    println!("This is a fork {:?}", fork);
    println!("This is a plate {:?}", plate);
    println!("Ssi Badr")
}