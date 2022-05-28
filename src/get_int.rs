use std::io;

pub fn get_int(input_value: &mut i32) {

    loop {

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid integer");
                continue;
            }
        };
        *input_value = number;
        break;

    }

}
