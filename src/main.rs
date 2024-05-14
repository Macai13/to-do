#[allow(unused_imports)]
use std::{fmt::write, io};
use colored::Colorize;
use std::fs::{File, OpenOptions};
#[allow(unused_imports)]
use std::io::{Write, BufReader, BufRead, Error};
#[allow(unused_imports)]
use std::path::Path;
use std::io::prelude::*;

static PATH: &'static str = "./db/tasks.txt";

fn main() 
{
    let mut menu_option: i8;

    println!("{}", "Welcome to To-Do-List CLI! What do you wanna do?".yellow());

    loop 
    {
        menu_option = menu();

        match menu_option 
        {
            0 => std::process::exit(0),
            1 => add_task(None),
            2 => remove_task(),
            3 => show_tasks(),
            _ => (),
        }
    }
}

fn menu() -> i8
{
    println!("1. Add a task");
    println!("2. Remove a task");
    println!("3. View all tasks");
    println!("0. Exit\n");

    match get_input().parse::<i8>() 
    {
        Ok(n) => 
        {
            if n >= 0 && n <= 4
            {
                return n;
            }
            
            else  
            {
                println!("{}\n", "Invalid choice!".red());
                return -1;
            }
        },

        Err(_) => 
        {
            println!("{}\n", "Invalid choice!".red());
            return -1;
        },
    }
}

fn add_task(task: Option<&str>) -> ()
{
    let mut created_flag = true;

    if task == None
    {
        println!("{}", "What task would you like to add?".yellow());
    }
    
    let mut output = match OpenOptions::new().append(true).open(PATH)
    {
        Ok(file) => file,
        Err(_) => 
        {
            created_flag = false; 
            File::create(PATH).unwrap()
        },
    };

    let mut input = task.unwrap_or("");
    let temp: String;

    if task == None
    {
        temp = get_input();
        input = temp.as_str();
    }
    
    if created_flag
    {
        write!(output, "\n{}", input).expect(format!("{}", "Error".red()).as_str());
    }

    else  
    {
        write!(output, "{}", input).expect(format!("{}", "Error".red()).as_str());
    }

    if task == None 
    {
        println!("{}", "Task added sucessfully!\n".bright_blue());
    }
}

fn remove_task() -> ()
{
    println!("{}", "What task would you like to remove?".yellow());
    let to_remove = get_input();

    let mut file = File::open(PATH).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let v: Vec<&str> = contents.split('\n').collect();

    std::fs::remove_file(PATH).unwrap();

    for task in v 
    {
        if task != to_remove 
        {
            add_task(task.into());
        }
    }
    
    println!("{}", "Task removed sucessfully!\n".bright_blue());
}

fn show_tasks() -> ()
{
    let mut file = File::open(PATH).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);
    println!("{}", "\nPress enter to continue...".green());

    get_input();
}

fn get_input() -> String
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    clear_terminal();

    input.trim().to_string()
}

fn clear_terminal() -> ()
{
    print!("\x1B[2J\x1B[1;1H");
}