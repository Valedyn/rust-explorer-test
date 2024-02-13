use std::io;


fn main() {
    let mut user_input: String = String::new();
    let user_profile = env!("userprofile").to_owned() + "\\Desktop";
    let mut one_dir: Directory = Directory::new( &user_profile);
    while user_input != "exit.".to_string() {

        let mut currently_in_string = format!("Currently in... {}\n", one_dir.get_filepath());
        let currently_in_string_len = &currently_in_string.chars().count();

        for _ in 1..*currently_in_string_len{
            currently_in_string.push('-');
        }
        println!("{}", currently_in_string);
        user_input = String::new();
        let list = one_dir.list_files();
        let max_len = list.len();
        for pos in 0..max_len {
            let mut append_spaces = String::new();
            let max_len = max_len.to_string().chars().count();
            //let spaces_to_add: [&str];
            for _ in 0..(max_len-(pos+1).to_string().chars().count()){
                append_spaces.push(' ');
            }
            println!("{} {}| {}", (pos + 1), append_spaces, get_element_of_vector(&list, pos as i32));
        }
        io::stdin().read_line(&mut user_input).expect("ERR: reading the user input returned an error!");
        user_input = user_input.trim().replace("\n", "").to_string();
        let mut temp_file_path: String = user_input.clone();
        let one_dir_filepath = one_dir.get_filepath();

        if temp_file_path.as_str() == "." || temp_file_path.as_str() == "back"{
            temp_file_path = one_dir_filepath.split_at(one_dir_filepath.rfind("\\").unwrap().clone()).0.to_string();
        }else {
            let mut everything_okay: bool = true;
            let parsed_input = match user_input.parse::<i32>(){
                Ok(value) => value,
                Err(_) => -1
            };
            if parsed_input < 0{
                everything_okay = false;
            }
            if everything_okay {
                let temp = get_element_of_vector(&list, (&parsed_input) - 1).to_string();

                temp_file_path = match temp.rfind('.') {
                    None => temp,
                    Some(_) => one_dir_filepath.clone(),
                };
                if parsed_input >= list.len() as i32 {
                    temp_file_path = one_dir_filepath;
                }
            }else {
                temp_file_path = one_dir_filepath;
            }
        }

        one_dir = Directory::new(temp_file_path.as_str());
    }
}

struct Directory{
    full_filepath: String,
}

impl Directory{
   pub fn new(full_filepath: &str) -> Self{
        Directory{
            full_filepath: full_filepath.to_string(),
        }
    }
}

impl Directory{
    fn list_files(&self) -> Vec<String>{
        let list = match std::fs::read_dir(&self.full_filepath){
            Ok(value) => {value}
            Err(_) => {panic!("ERR: reading the directory in list_files didn't work!")}
        };
        let mut returned_vec: Vec<String> = Vec::new();
            for element in list{
                returned_vec.push(element.unwrap().path().to_str().unwrap().to_string());
            }


        returned_vec
    }
    fn get_filepath(&self) -> String{
        self.full_filepath.clone()
    }
}

fn get_element_of_vector(vec: &Vec<String>, pos: i32) -> String{
    if pos < 0{
        return "".to_string()
    }
    match vec.get(pos as usize){
        Some(value) => value.to_string(),
        None => "".to_string()
    }
}

