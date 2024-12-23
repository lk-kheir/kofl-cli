pub mod config
{
    use std::path::PathBuf;

    pub struct KoflGlobalConfig {
        
        storage_path: PathBuf,

    }

    impl KoflGlobalConfig {
        pub fn new(storage_path: PathBuf) -> KoflGlobalConfig {
            KoflGlobalConfig{storage_path}
        }

        pub fn set_storage_path(&mut self, path: PathBuf) {
            self.storage_path = path;
        }
    
        pub fn get_storage_path(&self) -> &PathBuf {
            &self.storage_path
        }
    }
}