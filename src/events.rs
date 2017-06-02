pub struct Events {
   pub messages: Vec<Vec<String>>
}
impl Events {
   pub fn new() -> Events {
      Events {
         messages: Vec::new()
      }
   }
   pub fn message(&mut self, msg: Vec<String>) {
      self.messages.push( msg )
   }
}
