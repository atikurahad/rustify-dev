use std::io::{self, Write};

struct Contact {
    id : u32,
    name : String,
    phone : String,

}


struct PhoneBook {
    contacts: Vec<Contact>,
    next_id: u32,
}

impl PhoneBook {
    fn new () -> Self {
        PhoneBook{
            contacts: Vec :: new(),
            next_id:1,
        }
    }


    //Create 

    fn create(&mut self,name:String,phone:String){
        let contact = Contact {
            id: self.next_id,
            name,
            phone,
        };

        self.contacts.push(contact);

        println!("Contact added successfully");
        self.next_id +=1; 
    }

fn list (&self){
    if self.contacts.is_empty(){
        println!("No contact found.");
        return;
    }

    for contact in &self.contacts{
        println!(
            "Id: {} |
             Name: {} |
              Contact {}",
            contact.id,
            contact.name,
            contact.phone
        );
    }
}

   // DELETE
    fn delete(&mut self, id: u32) {
        let old_len = self.contacts.len();

        self.contacts.retain(|contact| contact.id != id);

        if self.contacts.len() < old_len {
            println!("Contact deleted successfully!");
        } else {
            println!("Contact with ID {} not found.", id);
        }
    }
}

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .unwrap();

    value.trim().to_string()
}

fn main() {
    let mut phonebook = PhoneBook::new();

    loop {
        println!("\n===== PHONEBOOK =====");
        println!("1. Add Contact");
        println!("2. List Contacts");
        println!("3. Delete Contact");
        println!("4. Exit");

        let choice = input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let name = input("Enter name: ");
                let phone = input("Enter phone: ");

                phonebook.create(name, phone);
            }

            "2" => {
                phonebook.list();
            }

            "3" => {
                let id = input("Enter contact ID: ");

                match id.parse::<u32>() {
                    Ok(id) => {
                        phonebook.delete(id);
                    }

                    Err(_) => {
                        println!("Please enter a valid ID.");
                    }
                }
            }

            "4" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid choice!");
            }
        }
    }
}