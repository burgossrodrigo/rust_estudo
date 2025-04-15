fn main() {
    let width: i32 = 4;
    let height: i32 = 7;
    let depth: i32 = 10;

    let area = area_calc(width, height);
    let volume = volume_calc(area, depth);

    println!("The area is: {}", area);
    println!("The volume is: {}", volume);
}

fn area_calc(width: i32, height: i32) -> i32 {
    width * height
}

fn volume_calc(area: i32, depth: i32) -> i32 {
    area * depth
}
