use framework::*;

#[derive(Default, Delegate)]
pub struct Runtime {
    #[to(TodoListStore, TodoListRepository)]
    store: services::memstore::MemStore,
}
