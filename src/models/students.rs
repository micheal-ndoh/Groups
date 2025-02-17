use crate::{
    data_collection::DataCollection,
    traits::{collect::Collect, gen_data_id::GenDataId},
};
#[derive(Debug, Clone)]
pub struct Students {
    id: u32,
    name: String,
}

impl Students {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Collect for Students {
    fn collect() -> Self {
        let mut student = Self::new();

        student.name = DataCollection::input("Enter topic title");

        student
    }
}

impl GenDataId<u32> for Students {
    fn set_id(&mut self, id: u32) {
        self.id = id
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}