use crate::models::group::Group;
use std::rc::Rc;
pub struct AppState {
    data: u64,
    groups: Rc<Vec<Group>>,

}
pub struct Application {
    state: AppState
}