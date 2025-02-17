use std::fmt::Debug;

use rand::seq::SliceRandom;

use crate::{
    data_collection::DataCollection,
    enums::labelling::Labelling,
    models::{
        group::Group,
        students::{self, Students},
        topics::Topic,
    },
    traits::{collect::Collect, gen_data_id::GenDataId},
};

struct AppState<'a, 'b> {
    topics: Vec<Topic>,
    students: Vec<Students>,
    labelling: Labelling,
    groups: Vec<Group<'a, 'b>>,
}

pub struct Application<'a, 'b> {
    state: AppState<'a, 'b>,
}

impl<'a, 'b> Application<'a, 'b> {
    pub fn new() -> Self {
        Self {
            state: AppState {
                groups: Vec::new(),
                labelling: Labelling::Numeric,
                topics: Vec::new(),
                students: Vec::new(),
            },
        }
    }

    pub fn run(&mut self) {
        println!("Enter new topics.");

        loop {
            Self::collect_gen_data(&mut self.state.topics);

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

    pub fn gen_groups(&mut self) {
        use rand::rng;
        let mut students = self.state.students.clone();
        let mut rng_gen = rng();
        students.shuffle(&mut rng_gen);
        let mut groups = Vec::<Group>::new();
        let nbr_of_members = self.state.students.len() / self.state.topics.len();
        for topic in &self.state.topics {
            let label = self.label_gen();
            // Shuffle array of students

            // let mut shuffled_students = self.state.students.as_ref();
            // shuffled_students.shuffle(&mut rand::thread_rng());

            // Select group members
            let grp_students = &self.state.students[0..nbr_of_members];
            let grp_students = grp_students.iter().map(|s| s).collect();
            let new_group = Group::from(label, topic, grp_students);
            groups.push(new_group);
        }
        // self.state.groups = groups;
    }

    pub fn label_gen(&self) -> String {
        match self.state.labelling {
            Labelling::Alphabetic => self.num_label_gen(),
            Labelling::Alphanumeric => todo!(),
            Labelling::Numeric => self.num_label_gen(),
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
