
#[derive(Default, Debug)]
 pub(crate) struct State {
  temperature: i32,
}

impl State {
  pub fn new(temperature: i32) -> Self {
    Self {
      temperature,
    }
  }

  pub fn better(&self, other: &Option<Self>) -> bool {
    if let Some(other) = other {
      self.temperature < other.temperature
    } else {
      false
    }
  }
}