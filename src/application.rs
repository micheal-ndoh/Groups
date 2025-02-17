use std::fmt::Debug;

use crate::{
    data_collection::DataCollection,
    enums::labelling::Labelling,
    models::{group::Group, students::Students, topics::Topic},
    traits::{collect::Collect, gen_data_id::GenDataId},
};
// #[derive{Debug}]
struct AppState {
    topic: Vec<Topic>,
    students: Vec<Students>,
    labelling: Labelling,
    groups: Vec<Group>,
}

pub struct Application {
    state: AppState,
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState {
                groups: Vec::new(),
                labelling: Labelling::Numeric,
                topic: Vec::new(),
                students: Vec::new(),
            },
        }
    }
    pub fn run(&mut self) {
        println!("Enter new topics.");

        loop {
            Self::collect_gen_data(&mut self.state.topic);

            if Self::should_proceed() {
                break;
            }
        }

        println!("Enter the students names");

        loop {
            Self::collect_gen_data(&mut self.state.students);

            if Self::should_proceed() {
                break;
            }
        }
    }

    fn collect_gen_data<T: GenDataId<u32> + Debug + Collect>(element: &mut Vec<T>) {
        let mut new_element = T::collect();
        new_element.set_id(element.len() as u32);
        println!("{:?}", new_element);
        element.push(new_element);
    }

    fn should_proceed() -> bool {
        let proceed = DataCollection::input("Do you want to continue?(yes/no)[yes]");
        !proceed.to_lowercase().eq("no")
    }

    pub fn label_gen(&self) -> String {
        match self.state.labelling {
            Labelling::Alphabetic => self.num_label_gen(),
            Labelling::Alphanumeric => todo!(),
            Labelling::Numeric => todo!(),
        }
    }

    pub fn num_label_gen(&self) -> String {
        (self.state.groups.len() + 1).to_string()
    }
}

pub struct Helper;

impl Helper {
    pub fn now_in_secs() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();

        match now.duration_since(UNIX_EPOCH) {
            Ok(value) => value.as_secs(),
            Err(_) => {
                panic!("time went backward")
            }
        }
    }
}