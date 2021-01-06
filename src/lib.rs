mod key_manager;
mod error;

#[cfg(test)]
mod tests {
    use crate::key_manager;



    #[test]
    fn it_works() {
        key_manager::KeyManager::current_new("test").unwrap();
    }
}
