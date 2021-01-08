mod key_manager;
mod error;
mod util;
mod file;
mod resource;

#[cfg(test)]
mod tests {
    use crate::{file::size::Size, key_manager};

    #[test]
    fn key_manager_generate_test() {
        key_manager::KeyManager::new_current_dir("test").unwrap();
        key_manager::KeyManager::from_current_dir_checked("test").unwrap();
    }
    #[test]
    fn size_convert_test() {
        let s = Size::new(1919810);
        println!("{:?}", s.each_1024());
        println!("{:?}", s.float_by_scale(crate::file::SizeScale::MByte));
        println!("{:?}", s.auto_float_by_scale());
    }
}
