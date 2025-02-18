use crate::{data_collection::DataCollection, traits::gen_data_id::GenDataId, traits::collect::Collect};

#[derive(Debug, Clone)]
pub struct Student {
    id: u32,
    name: String,
}

impl Student {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
        }
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

impl GenDataId<u32> for Student {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Collect for Student {
    fn collect() -> Self {
        let mut student = Self::new();
        student.name = DataCollection::input("Enter the student's name:");
        student
    }
}