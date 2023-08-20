pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}

pub struct NameService {
    name_repository: Box<dyn NameRepository>
}

impl NameService {
    pub fn new(name_repository: Box<dyn NameRepository>) -> Box<NameService> {
        return Box::new(NameService { name_repository });
    }
}

pub trait NameRepository {
    fn get_data(&self) -> i32;
}