use crate::storage::Storage;

#[derive(Debug, Default)]
pub struct State<T>
where
    T: Storage,
{
    pub db: T,
}
