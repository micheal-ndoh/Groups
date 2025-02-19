use cli_table::{Cell, Row, Table, WithTitle};
use crate::models::{group::Group, student::Student, topic::Topic};

#[derive(Debug, Clone)]
pub struct GroupTable {
    pub groups: Vec<Group>,
}

impl Table for GroupTable {
    fn rows(&self) -> Vec<Row> {
        self.groups
            .iter()
            .flat_map(|group| {
                let topic = group.get_topics();
                group.get_students().iter().map(move |student| {
                    Row::new(vec![
                        student.get_id().cell(),
                        student.get_name().cell(),
                        group.get_id().cell(),
                        topic.get_title().cell(),
                    ])
                })
            })
            .collect()
    }
}

impl WithTitle for GroupTable {
    fn title() -> Row {
        Row::new(vec![
            "Student ID".cell().bold(true),
            "Student Name".cell().bold(true),
            "Group ID".cell().bold(true),
            "Topic".cell().bold(true),
        ])
    }
}