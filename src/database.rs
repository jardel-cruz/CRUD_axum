use crate::entities::User;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};

type DatabaseUserType = RwLock<Vec<Data<User>>>;

static DATABASE: DatabaseUserType = RwLock::new(Vec::new());
static COUNT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
struct Data<T> {
    id: u64,
    data: T,
}

impl<T> Data<T>
where
    T: Clone,
{
    fn new(data: &T) -> Self {
        let data = data.clone();
        let new_data = Data {
            id: COUNT_ID.load(Ordering::Relaxed),
            data,
        };

        COUNT_ID.fetch_add(1, Ordering::Relaxed);

        new_data
    }
}
pub struct UserDatabase {}

impl UserDatabase {
    pub async fn save(user: &User) {
        let data = Data::new(user);

        DATABASE.write().unwrap().push(data);
    }

    pub async fn find_by_id(id: u64) -> Result<User, ()> {
        for user_data in DATABASE.write().unwrap().iter() {
            if user_data.id == id {
                return Ok(user_data.data.clone());
            }
        }

        Err(())
    }

    pub async fn find() -> impl Iterator<Item = Data<User>> {
        let vec = DATABASE.write().unwrap().clone();
        vec.into_iter()
    }
}
