fn main() {
    cal_square(5, 5);
}

fn cal_square(height: i64, width: i64) {
    let area = height * width;
    print!("width: {}, height: {}, area: {}", height, width, area);
}
