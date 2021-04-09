/*

In this project, I'll be experimenting with some slightly more advanced data types in rust, particularily by creating a todo list. This list will just be stored in a variable. Nothing special
*/


use std::io; //used to get terminal inputs

fn main() {
    let mut tasks = Vec::new(); //this is probably not the best way to do it. However, I do not want to go too far without actually seeing it in the documentation first.
    loop {
        println!("Please type in a todo task. To view all your tasks, please type -tasks");
        let mut input = String::new(); //assigning an empty mutable variable called input. 
        io::stdin()                           
            .read_line(&mut input) // doing some basic read terminal input stuff here
            .expect("Unable to read line");  
        if input.trim() == "-tasks" { //checking if our input is equal to "-tasks". we trim it to remove the `\n` at the end of the input. Not removing it would make rust not recognise that its equal to `-tasks`.
            for (i, task) in tasks.iter().enumerate() { //enumerating through our tasks. Not looping. Enumerating allows us to access the index at the same time. this is the same as enumerate(list) in python.
                println!("{}) {}", i, task) //printing out our task
            }
        } else {
            println!("Added {} to your todo list", input); //printing out a success message before hand, because of a strange "borrow" error I encountered.
            tasks.push(input); //adding or "pushing" our input to our tasks list
        }
    }
}