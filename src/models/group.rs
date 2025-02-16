use std::rc::Rc;

use crate::{data_collection::{Collect, DataCollection}, models::{students::Student, topics::Topic}};

#[derive(Debug, Clone)]

pub struct Group {
    label: String,
    topic: Rc<Topic>,
    id: u32,
    students: Vec<Rc<Student>>,
}

impl Group {

    pub fn from (label: String, topic: Rc<Topic>) -> Self {
        Self {
            id: 0,
            label,
            topic,
            students: Vec::new(),
        }
        
    }

    pub fn new() -> Self {
        Self {
            id: 0,
            label: String::from(""),
            topic: Rc::new(Topic::new()),
            students: Vec::new(),
        }
    }
    // pub fn new() -> Self {
    //     Self {
    //         id: 0,
    //         label: String::from(""),
    //         topic: Rc::new(Topic::new()),
    //         students: Vec::new(),
            
    //     }
    // }
    // pub fn set_id(&mut self, id: u32) {
    //     self.id = id;
    // }
    // pub fn set_label(&mut self, label: String) {
    //     self.label = label;
    // }
    // pub fn set_topic(&mut self, topic: Rc<Topic>) {
    //     self.topic = topic;
    // }

    // pub fn set_student(&mut self, student: Rc<Student>) {
    //     self.students.push(student);
    // }

    // pub fn get_id(&self) -> u32 {
    //     self.id
    // }
    // pub fn get_label(&self) -> String {
    //     self.label.clone()
    // }

    // pub fn get_topic(&self) -> Rc<Topic> {
    //     self.topic.clone()
    // }

    // pub fn get_students(&self) -> Vec<Rc<Student>> {
    //     self.students.clone()
    }


impl Collect for Group {
    fn collect() -> Self {
        let mut group = Self::new();
        // println!("Enter topic. \n type done when completed");
        let prompt = Some("Enter group label".to_string());
        group.label = DataCollection::input(prompt);

        let prompt = Some("Enter topic".to_string());
         

        group
    }
}


