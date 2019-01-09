use std::io;
use rand::Rng;
use std::thread;
use std::time;
use wincolor::{Console, Color, Intense};
use winconsole::console;


fn text(){
	let mut con = Console::stdout().unwrap();
	console::set_title("Rock Paper Scissors").unwrap();

	if let Some((w, _h)) = term_size::dimensions() {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		thread::sleep(time::Duration::from_millis(500));

		println!("{holder:^width$}", width=w, holder="~-==Rock Paper Scissors==-~");

		con.reset().unwrap();
		thread::sleep(time::Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!("{holder:^width$}", width=w, holder="~-==Now built in Rust!==-~");

		thread::sleep(time::Duration::from_millis(1000));
		con.reset().unwrap();
	} else {
		con.fg(Intense::Yes, Color::Cyan).unwrap();
		thread::sleep(time::Duration::from_millis(500));

		println!("{holder:^width$}", width=110, holder="~-==Rock Paper Scissors==-~");

		con.reset().unwrap();
		thread::sleep(time::Duration::from_millis(1000));
		con.fg(Intense::Yes, Color::Green).unwrap();

		println!("{holder:^width$}", width=110, holder="~-==Now built in Rust!==-~");

		thread::sleep(time::Duration::from_millis(1000));
		con.reset().unwrap();
	}	
}

fn to_title(s1: String) -> String{
    let mut v: Vec<char> = s1.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let s2: String = v.into_iter().collect();
    let s3 = &s2;
    s3.to_string()
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
        let s3 = to_title(user.clone());
    println!("You chose {}!", s3);
    thread::sleep(time::Duration::from_millis(500));
    let comp = computer_choice();
        let s3 = to_title(comp.clone());
    println!("Computer chose {}!", s3);
    thread::sleep(time::Duration::from_millis(200));
    let result = logic(user, comp);
        let you = if result == "draw"{
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
    text();
    game();
}

fn logic(user: String, comp: String) -> String {
    let mut return_val = String::new();
    if user == comp {
        return_val = String::from("draw");
    }else{
        if user == "rock"{
            if comp == "paper"{
                return_val = String::from("loss");
            }else{
                return_val = String::from("win")
            }
        } else if user == "paper"{
            if comp == "scissors"{
                return_val = String::from("loss");
            }else{
                return_val = String::from("win")
            }
        } else if user == "scissors"{
            if comp == "rock"{
                return_val = String::from("loss");
            }else{
                return_val = String::from("win")
            }
        }
    }
    return_val
}

fn computer_choice() -> String{
    let comp_num = rand::thread_rng().gen_range(1,4);
    let mut _comp_val = String::new();
    match comp_num {
        1 => _comp_val = "rock".to_string(),
        2 => _comp_val = "paper".to_string(),
        3 => _comp_val = "scissors".to_string(),
        _ => _comp_val = "broken".to_string(),
    };
    _comp_val
}

fn choice_input() -> String{
    loop{
        println!("Rock/Paper/Scissors ?");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");
        
        choice = choice.trim().to_ascii_lowercase();

        if choice == "rock" || choice == "paper" || choice == "scissors" {
            break choice;
        }
    }//no ; here means it returns loop value to where it was called
}