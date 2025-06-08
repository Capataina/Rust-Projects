use std::io;

fn main() {
    let mut tasks = Vec::new();

    loop {
        let mut action = String::new();

        println!("The usable commands are: \n 1 - 'List' to view the to-do list \n 2 - 'Add' to add a new item to the to-do list \n 3 - 'Exit' to exit the application.\n");

        io::stdin().read_line(&mut action).expect("Failed to read command");

        action = action.trim().to_string();

        convert_to_lower_case(&mut action);

        if action == "exit" {
            break; // Exit the loop (and the program)
        } else if action.starts_with("add ") {
            let task = action[4..].to_string(); // Extract the task description
            tasks.push(task); // Add the task to the list
            println!("Task added.");
        } else if action == "list" {
            for (index, task) in tasks.iter().enumerate() {
                println!("{}: {}", index + 1, task);
            }
        } else {
            println!("Unknown command");
        }
    }
}

fn convert_to_lower_case(word: &mut String) {
    let mut lower_cased_word = String::new();

    for character in word.chars() {
        lower_cased_word.push_str(&character.to_lowercase().collect::<String>());
    }

    *word = lower_cased_word;
}
