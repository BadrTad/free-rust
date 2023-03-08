mod kitchen;


use crate::kitchen::plates::Tools;


fn main() {
    let fork = Tools::Fork;
    println!("This is a fork {:?}", fork);
}