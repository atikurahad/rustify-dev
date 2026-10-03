use std::io::{self,write};

struct Contact {
    id : u32,
    name : String,
    phone : String,

};


struct PhoneBook {
    contacts: Vec<Contact>,
    next_id: u32,
};

impl PhoneBook {
    fn new () -> self {
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


    


};