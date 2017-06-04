pub struct Events {
   pub messages: Vec<Vec<String>>,
   pub state: String,
}
impl Events {
   pub fn new() -> Events {
      Events {
         messages: Vec::new(),
         state: "".to_owned(),
      }
   }
   pub fn message(&mut self, msg: Vec<String>) {
      self.messages.push( msg )
   }
}
