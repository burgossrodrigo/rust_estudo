use rust_estudo::{ding, on_off, print_array, print_difference};
fn main() {
    let cords: (f32, f32) = (6.3, 15.0);

    print_difference(cords);

    print_array(cords);

    let series = [1, 1, 2, 3, 5, 8, 13];

    ding(series);

    let mess = ([3, 4], 3.14, [(false, -3), (true, -100)], 5, "candy");

    on_off(mess.2[1].0)

}