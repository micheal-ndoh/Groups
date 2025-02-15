use crate::data_collection::{Collect, DataCollection};

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    id: u32,
}

impl Student {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::from(""),
        }
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
   

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

   
}

 

impl Collect for Student {
    fn collect() -> Self {
        let mut student = Self::new();
        // println!("Enter topic. \n type done when completed");
        let prompt = Some("Enter Student's name ".to_string());
        student.name = DataCollection::input(prompt);


        student
    }
}
