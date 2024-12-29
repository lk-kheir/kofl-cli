use crate::config::Config::KoflGlobalConfig;

#[warn(unused_variables)]
#[warn(unused_imports)]
pub struct Context {
    pub kgc: KoflGlobalConfig,
    // I can add more state here
}


impl Context {
    pub fn new() -> Self {
        let mut c = KoflGlobalConfig::new();
        c.load();
        Context { kgc: (c) }
    }
}