//src/stting/core.rs


pub trait Setting {
    type Value;
    
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn default() -> Self::Value;
    fn validate(&self, Value: Self::Value) -> Result<(), String>;
    fn update(&mut self, value: Self::Value) -> Result<(), String>;

}