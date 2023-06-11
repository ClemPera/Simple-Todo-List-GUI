use std::collections::HashMap;
use rusqlite::Connection;
use std::io;
use std::io::Write;

#[derive(Debug)]
struct Todo{
    id: i32,
    input: String,
}

fn main() {
    //Connexion à la db
    let conn = Connection::open("todo.db").expect("Error: Connection to db failed");

    //Création de la table todo
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        input TEXT NOT NULL
    )",
        [],
    ).expect("Error: Table not created");

    loop {
        let mut user_input = String::new();
        println!("\n\nWelcome to you magical todo list!! \n\n Select an option : \n 1:Show todo list \n 2: Add to the todo list \n 3: Remove from the todo list \n 4: Exit");

        io::stdin().read_line(&mut user_input).expect("Error: Couldn't read user input");
        let user_input = match user_input.trim().parse::<i32>() {
            Ok(input) => input,
            Err(_) => continue,
        }; //.expect("Error converting to i32");

        if user_input == 1{ //Show list
            show_values_log(&conn);
        }
        else if user_input == 2{ //Add in list
            let mut value = String::new();

            print!("\n\nPlease enter the value to add: ");

            io::stdout().flush().expect("Error: Failed to flush stdout");
            io::stdin().read_line(&mut value).expect("Error: Couldn't read user input");


            add_in_table(&conn, value)
        }
        else if user_input == 3{ //Remove value
            let mut id = String::new();

            print!("\n\nPlease enter the id to remove: ");

            io::stdout().flush().expect("Error: Failed to flush stdout");
            io::stdin().read_line(&mut id).expect("Error: Couldn't read user input");
            let id = id.trim().parse::<i32>().expect("Error converting to i32");

           delete_values(&conn, id);
        }
        if user_input == 4 { //Exit
            break;
        }
    }
}

///Ajout dans la table todo : input dans colonne input
pub fn add_in_table(conn:&Connection, input:String){
    conn.execute(
        "INSERT INTO todo (input) VALUES (?1)",
        &[&input.to_string()],
    ).expect("Error: Value not added in table");
}

///Show values in table todo
fn show_values_log(conn:&Connection){
    let mut select = conn.prepare("SELECT id, input FROM todo;").expect("Error: Select failed");

    let todos= select.query_map((), |row| {
        Ok(Todo{
            id: row.get(0)?,
            input: row.get(1)?,
        })
    }).expect("Error: Query failed");

    for values in todos{
        println!("Found values {:?}", values);
    }
}

pub fn show(conn:&Connection) -> HashMap<i32, String> {
    let mut value = HashMap::new();

    let mut select = conn.prepare("SELECT id, input FROM todo;").expect("Error: Select failed");

    let todos= select.query_map((), |row| {
        Ok(Todo{
            id: row.get(0)?,
            input: row.get(1)?,
        })
    }).expect("Error: Query failed");

    for values in todos{
        match values{
            Ok(val) =>{
                value.insert(val.id, val.input);
            }
            Err(_) => {
                println!("error sql values");
            }
        }
    }

    value
}

///Delete values in table todo
pub fn delete_values(conn:&Connection, id:i32){
    conn.execute(
        "DELETE FROM todo WHERE id = ?1",
        &[&id],
    ).expect("Error: Value not deleted in table");
}