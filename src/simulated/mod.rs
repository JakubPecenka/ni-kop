mod state;

fn frozen(temperature: i32) -> bool {
  temperature < 0
}

fn equilibrium() -> bool {
  true
}


fn cool(temperature: i32) -> i32 {
  temperature - 1
}

fn mytry() -> state::State {
  state::State::new(0)
}

pub fn run(initial_temperature: i32) {

  let mut temperature = initial_temperature;
  let mut best = Some(state::State::new(temperature));

  while !frozen(temperature) {
      while !equilibrium() {
          let state = mytry();
          if state.better(&best) {
              best = Some(state);
          }
      }
      temperature = cool(temperature);
  }
}
