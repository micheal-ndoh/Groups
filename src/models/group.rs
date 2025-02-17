use crate::{application::Helper, models::{students::Students, topics::Topic}, traits::gen_data_id::GenDataId};

// use super::{students::Students, topic::Topic};
#[derive(Debug)]
pub struct Group<'a, 'b> {
    id: u32,
    label: String,
    topic: & 'a Topic,
    students: Vec<&'b Students>,
    generated_at: u64
}

impl<'a, 'b> Group<'a, 'b> {
    pub fn from(label: String, topic: &'a Topic, students: Vec<&'b Students>) -> Self {
        Self {
            id: 0,
            label,
            topic,
            students: Vec::new(),
            generated_at: Helper::now_in_secs(),
        }
    }
}

impl<'a, 'b> GenDataId<u32> for  Group<'a, 'b> {
    fn set_id(&mut self, id: u32) {
       self.id = id
   }

   fn get_id(&self) -> u32 {
       self.id
   }
}