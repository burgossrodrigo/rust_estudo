pub fn print_difference(cords: (f32, f32)) {
    let (x, y) = cords;
    println!("The difference is: {}", x - y);
}

pub fn print_array(cords: (f32, f32)) {
    let (x, y) = cords;
    let arr = [x, y];
    println!("The array is: {:?}", arr);
}

pub fn ding(arr: [i32; 7]) {
    let value_of_interest = arr[6];
    println!("The value of interest is {}", value_of_interest);

}

pub fn on_off(value: bool) {
    if value {
        println!("Lights are on");
    }
}