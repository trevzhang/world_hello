#![allow(unused)]
fn main() {
    let mut array = [1,2,3,4];
    println!("array:{:?}", array);
    let slice = &mut array[0..3];
    println!("slice:{:?}", slice);
    slice[0] = 0;
    println!("slice modified:{:?}", slice);
    
    let mut tuple = (1,2,"3");
    println!("tuple:{:?}",tuple);

    let mut tuple1 = &mut tuple.0;
    println!("tuple1:{:?}", tuple1);
}
