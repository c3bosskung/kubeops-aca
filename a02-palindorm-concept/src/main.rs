fn main() {
    loop {
        let mut input = String::new();
        println!("Enter your word for check parindome : ");
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        if input.trim() == "end" {
            println!("Goodbye :)");
            break;
        } else {
            method(input);
        }
    }
}

fn method(a: String) {
    let a = a.trim();
    let a_length = a.len();
    let mut a_check_len = a_length - 1;
    let mut score = 0;
    let a_length_half = a_length / 2;
    for i in 0..a_length_half {
        if a.chars().nth(i).unwrap() == a.chars().nth(a_check_len).unwrap() {
            //println!("a firstHalf = {}, a backHalf = {}", a.chars().nth(i).unwrap(),  a.chars().nth(a_check_len).unwrap());
            //println!("True");
            score = score + 1;
        } else {
            //println!("a firstHalf = {}, a backHalf = {}", a.chars().nth(i).unwrap(),  a.chars().nth(a_check_len).unwrap());
            //println!("False");
        }
        a_check_len = a_check_len - 1;
    }
    if score == a_length / 2 {
        println!("{} is a parindome", a);
    } else {
        println!("{} is not parindome", a);
    }
}
