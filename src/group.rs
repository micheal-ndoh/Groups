use std::rc::Rc;

use crate::topics::Topic;

pub struct Group {
    label: String,
    topic: Rc<Topic>
}