// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    if maybe_number.is_some(){
        println!("printing: {}", maybe_number.unwrap());
    }else if maybe_number.is_none(){
        println!("printing: None");
    }
    
    match maybe_number {
        None =>{
            println!("matching: None");
        },
        Some(number)=>{
            println!("matching: {}", number);
        }
    }
}

fn main() {
    print_number(None);
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers:[Option<u16>;5]= [None; 5];
    println!("printing: {:?}", numbers);
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        numbers[iter as usize] = Some(number_to_add);
    }
    
    println!("printing: {:?}", numbers);
}
