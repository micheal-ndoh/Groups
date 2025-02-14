use std::rc::Rc;

use crate::models::{students::Student, topics::Topic};


pub struct Group {
    label: String,
    topic: Rc<Topic>,
    id : u32,
    students:Vec<Rc<Student>> 
}