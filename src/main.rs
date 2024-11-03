use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Importance {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
struct Task {
    title: String,
    importance: Importance,
    description: String,
}

fn add_to_list(tasks: &mut Vec<Task>) {
    // Get Title
    print!("\nEnter the title of the new task: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    let title = title.trim().to_string();

    // Get importance
    let importance = loop {
        print!("Choose importance (1=Low, 2=Medium, 3=High): ");
        io::stdout().flush().unwrap();
        let mut importance_input = String::new();
        io::stdin()
            .read_line(&mut importance_input)
            .expect("Failed to read input");
        match importance_input.trim() {
            "1" => break Importance::Low,
            "2" => break Importance::Medium,
            "3" => break Importance::High,
            _ => println!("Invalid choice, please enter 1, 2 or 3!"),
        }
    };

    // Get description
    print!("Write the description of the task: ");
    io::stdout().flush().unwrap();
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");
    let description = description.trim().to_string();

    // Create task element from inputs
    let task = Task {
        title,
        importance,
        description,
    };

    // Push the task to the task list
    tasks.push(task);
    println!("\nTask has been added to your list!\n");
}

fn show_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available");
        return;
    }

    for task in tasks {
        println!("\n--------------------------------");
        println!("Title: {}", task.title);
        println!("Importance: {:?}", task.importance);
        println!("Description: {}", task.description);
        println!("--------------------------------\n");
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\nThere are no added tasks to your list!\n");
        return;
    }
    print!("\nEnter the index number of the task you want to remove: ");
    io::stdout().flush().unwrap();

    let mut task_index = String::new();
    io::stdin()
        .read_line(&mut task_index)
        .expect("Failed to read line");

    // Parse the input to an integer
    match task_index.trim().parse::<usize>() {
        Ok(index) => {
            if index < tasks.len() {
                tasks.remove(index);
                println!("Task at index {} has been removed.\n", index);
            } else {
                println!(
                    "Invalid index. Please enter a number between 0 and {}.\n",
                    tasks.len() - 1
                );
            }
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid index number.\n");
        }
    }
}

fn modify_task(tasks: &mut [Task]) {
    if tasks.is_empty() {
        println!("\nThere are no added tasks to your list!\n");
        return;
    }
    print!("\nEnter the index of the task you want to modify: ");
    io::stdout().flush().unwrap();

    let mut task_index = String::new();
    io::stdin()
        .read_line(&mut task_index)
        .expect("Failed to read line");

    // Parse the input to an integer
    match task_index.trim().parse::<usize>() {
        Ok(index) => {
            if index < tasks.len() {
                let task = &mut tasks[index];

                // Modify title
                print!(
                    "Current title: {}\nEnter new title (or press Enter to keep the same): ",
                    task.title
                );
                io::stdout().flush().unwrap();
                let mut new_title = String::new();
                io::stdin()
                    .read_line(&mut new_title)
                    .expect("Failed to read line");
                if !new_title.trim().is_empty() {
                    task.title = new_title.trim().to_string(); // Keep the same title
                }

                // Modify importance
                println!("Current importance: {:?}", task.importance);
                let importance = loop {
                    print!("Choose new importance (1=Low, 2=Medium, 3=High, or press Enter to keep the same): ");
                    io::stdout().flush().unwrap();
                    let mut importance_input = String::new();
                    io::stdin()
                        .read_line(&mut importance_input)
                        .expect("Failed to read input");
                    if importance_input.trim().is_empty() {
                        break task.importance.clone(); // Keep the same importance
                    }
                    match importance_input.trim() {
                        "1" => break Importance::Low,
                        "2" => break Importance::Medium,
                        "3" => break Importance::High,
                        _ => println!("Invalid choice, please enter 1, 2, or 3!"),
                    }
                };
                task.importance = importance;

                // Modify description
                print!("Current description: {}\nEnter new description (or press Enter to keep the same): ", task.description);
                io::stdout().flush().unwrap();
                let mut new_description = String::new();
                io::stdin()
                    .read_line(&mut new_description)
                    .expect("Failed to read line");
                if !new_description.trim().is_empty() {
                    task.description = new_description.trim().to_string(); // Keep the same description
                }

                println!("\nTask has been modified successfully!\n");
            } else {
                println!(
                    "Invalid index. Please enter a number between 0 and {}.",
                    tasks.len() - 1
                );
            }
        }
        Err(_) => {
            println!("Invalid input. Please enter a valid index number.");
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("Menu:");
        println!("1. Add a task");
        println!("2. Show tasks");
        println!("3. Modify task");
        println!("4. Delete task");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => add_to_list(&mut tasks),
            "2" => show_tasks(&tasks),
            "3" => modify_task(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("Exiting");
                break;
            }
            _ => println!("Invalid choice, please enter 1, 2 or 3!"),
        }
    }
}
