use std::collections::{HashMap};

///All user events pass through and/or are recorded in this structure.
pub struct Events {
   ///Central Dispatch
   pub messages: Vec<Vec<String>>,

   /// Component State
   pub state: String,

   /// Global Key-Val State
   pub keyval: HashMap<String,String>,

   /// Time elapsed since program started, measured in seconds
   pub time_elapsed: f64,
}
impl Events {
   ///Creates a new Events object. Used in Window rendering and is not meant for general use.
   pub fn new() -> Events {
      Events {
         messages: Vec::new(),
         state: "".to_owned(),
         keyval: HashMap::new(),
         time_elapsed: 0.0,
      }
   }
   ///Send a method to central dispatch
   pub fn message(&mut self, msg: Vec<String>) {
      self.messages.push( msg )
   }
   ///Set a state variable
   pub fn set(&mut self, key: &str, val: &str) {
      self.keyval.insert(key.to_string(), val.to_string());
   }
   ///Get a state variable
   pub fn get(&mut self, key: &str) -> String {
      self.keyval.get(&key.to_string()).unwrap_or(&"".to_string()).clone()
   }
}
