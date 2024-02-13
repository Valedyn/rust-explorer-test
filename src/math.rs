//use std::time::Instant;
fn main() {
    //println!("{}", format_number(pow(2, 80), '.'));
   // let mut before = Instant::now();
    println!("{}", format_number_slow(pow(2, 100), '.'));
    //println!("Elapsed time: {:.2?}", before.elapsed());
    //before = Instant::now();
    println!("{}",format_number(pow(2, 100), '.'));
   // println!("Elapsed time: {:.2?}", before.elapsed());


}

fn pow(num1: i128, num2: u128) -> i128{
    if num2 == 0 {
        return 1;
    }else if num1 == 0{
       return 0;
    }else if num1 == 1{
        return 1;
    }else if num2 == 1{
        return num1;
    }
    let mut result: i128 = num1;
    for _ in 2..=num2 {
        result *= num1;
    }

    result
}


fn format_number_slow(number: i128, char: char) -> String{
    let mut number: String = number.to_string();

    for pos in (0..number.len()).rev(){
        if (pos as i8-1) % 3 == 0{
            number.insert(pos, char.clone());
        }
    }
    number
}

fn format_number(number: i128, char: char) -> String {
    let number: String = number.to_string();
    let mut final_string: String = String::new();
    let number_length: usize = number.chars().count();
    for pos in (0..number_length).rev(){
        let range: usize = -((pos as i128 - number_length as i128)+1) as usize;
        let char_of_string = match number.get(range.. range+1){
            None => "",
            Some(value) => value
        };
        final_string.push_str(char_of_string);
        if (range) % 3 == 0 && range < number_length-1{
            final_string.push(char.clone());
        }
    }
    final_string
}


/*first try. using insert was WAY easier though. this works, but not in the way i want it to. it returns the string i need but in reversed order. fn format_number(number: i128, char: char) -> String{
    let number: String = number.to_string();
    let mut final_string: String = "".to_string();
    for pos in (0..=number.len()).rev(){
        if (pos as i8 -1)  >=0 {
            let temp = match &number.get(pos-1..pos) {
                None => "",
                Some(value) => value
            };
            println!("{}", temp);
            if temp != ""{
                final_string.push_str(temp);
                if (&pos+1) % 3 == 0{
                    final_string.push(char.clone());
                }
            }
        }
    }

    final_string.to_string()
}*/




