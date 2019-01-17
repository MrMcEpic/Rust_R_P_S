use rand::Rng;
use std::{io, thread::sleep, time::Duration};

mod text;

fn to_title(s1: &str) -> String{
    let mut v: Vec<char> = s1.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let s2: String = v.into_iter().collect();
    s2
}

fn again(){
    println!("Play Again? Y/N");
    let choice = loop{
        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        
        choice = choice.trim().to_ascii_lowercase();

       if choice == "y" || choice == "n"{
           break choice;
       }else if choice == "yes"{
            choice = "y".to_string();
            break choice;
       }else if choice == "no"{
            choice = "n".to_string();
            break choice;
       }
    };
    if choice == "y"{
        game();
    }else{
        println!("Press enter to close");
        let mut _enter = String::new();
        io::stdin().read_line(&mut _enter)
            .expect("Failed");
    }
}

fn game(){
    let user = choice_input();
    println!("You chose {}!", to_title(&user));
    sleep(Duration::from_millis(500));
    let comp = computer_choice();
    println!("Computer chose {}!", to_title(&comp));
    sleep(Duration::from_millis(300));
    let mut result = logic(&user, &comp);
        let you = if result == "draw"{
            result = to_title(&result);
            ""
        }else{
            "You "
        };
        let exclaim = if result == "win"{
            "!"
        }else{
            ""
        };
    println!("{}{}{}", you, result, exclaim);
    again();
}

fn main() {
    text::text();
    game();
}

fn logic(user: &str, comp: &str) -> String {
    let mut return_val = "";
    if user == comp {
        return_val = "draw";
    }else{
        match user{
            "rock" => {
                if comp == "paper"{
                    return_val = "lose";
                }else{
                    return_val = "win";
                }
            },
            "paper" => {
                 if comp == "scissors"{
                    return_val = "lose";
                }else{
                    return_val = "win";
                }
            },
            "scissors" => {
                if comp == "rock"{
                    return_val = "lose";
                }else{
                    return_val = "win";
                }
            },
            _ =>{
                println!("Error")
            }
        }
    }
    return_val.to_string()
}

fn computer_choice() -> String{
    let comp_num = rand::thread_rng().gen_range(1,4);
    let mut _comp_val = "";
    match comp_num {
        1 => _comp_val = "rock",
        2 => _comp_val = "paper",
        3 => _comp_val = "scissors",
        _ => _comp_val = "broken",
    };
    _comp_val.to_string()
}

fn choice_input() -> String{
    loop{
        println!("Rock/Paper/Scissors?");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        
        choice = choice.trim().to_ascii_lowercase();

        if choice == "rock" || choice == "paper" || choice == "scissors" {
            break choice; //returns choice to loop which then returns to function call
        }
    }//no ; here means it returns loop value to where it was called
}