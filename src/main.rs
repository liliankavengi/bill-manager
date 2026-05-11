use std::collections::HashMap;
use std::io;

struct Bill {
    name: String,
    amount: f64,
}

fn get_input() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}

fn add_bill(bills: &mut HashMap<String, Bill>) {
    println!("Enter bill name (or press enter to go back):");
    let name = get_input();
    
    if name.is_empty() {
        return;
    }
    
    println!("Enter amount:");
    let amount_str = get_input();
    
    if amount_str.is_empty() {
        println!("Add cancelled");
        return;
    }
    
    let amount: f64 = amount_str.parse().expect("Please enter a valid number");
    
    let bill = Bill { name: name.clone(), amount };
    bills.insert(name, bill);
    println!("Bill added successfully");
}

fn view_bills(bills: &HashMap<String, Bill>) {
    if bills.is_empty() {
        println!("No bills found");
        return;
    }
    
    println!("--- Current Bills ---");
    for bill in bills.values() {
        println!("Name: {}, Amount: {}", bill.name, bill.amount);
    }
    println!("---------------------");
}

fn remove_bill(bills: &mut HashMap<String, Bill>) {
    println!("Enter bill name to remove (or press enter to go back):");
    let name = get_input();
    
    if name.is_empty() {
        return;
    }
    
    if bills.remove(&name).is_some() {
        println!("Bill removed");
    } else {
        println!("Bill not found");
    }
}

fn edit_bill(bills: &mut HashMap<String, Bill>) {
    println!("Enter bill name to edit (or press enter to go back):");
    let name = get_input();
    
    if name.is_empty() {
        return;
    }
    
    if let Some(bill) = bills.get_mut(&name) {
        println!("Enter new amount:");
        let amount_str = get_input();
        
        if amount_str.is_empty() {
            println!("Edit cancelled");
            return;
        }
        
        let amount: f64 = amount_str.parse().expect("Please enter a valid number");
        bill.amount = amount;
        println!("Bill updated");
    } else {
        println!("Bill not found");
    }
}

fn main() {
    let mut bills: HashMap<String, Bill> = HashMap::new();

    loop {
        println!("== Bill Manager (Stage 3) ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Edit bill");
        println!("5. Exit");
        println!("Enter choice:");

        let choice = get_input();

        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "5" => break,
            _ => println!("Invalid choice, try again"),
        }
    }
}
