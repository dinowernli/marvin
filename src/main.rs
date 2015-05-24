extern crate ai;

pub struct Agent {
  age: i32,
  total_reward: f64,
}

impl Agent {
  pub fn new() -> Agent {
    Agent {
      age: 0,
      total_reward: 0.0,
    }
  }

  pub fn act(&mut self) -> i16 {
    return 4;  // Constant action for now.
  }

  pub fn update(&mut self, observation: i16, reward: f64) {
    self.age = self.age + 1;
    self.total_reward += reward;
  }

  pub fn average_reward(&self) -> f64 {
    return self.total_reward / (self.age as f64);
  }
}

trait Environment {
  fn reward(&self) -> f64;
  fn observation(&self) -> i16;

  fn update(&mut self, action: i16);
}

struct CoinFlip {
  last_toss: bool,
}

impl CoinFlip {
  pub fn new() -> CoinFlip {
    CoinFlip {
      last_toss: false,  // tails
    }
  }
}

//impl Environment for CoinFlip {
//}

fn main() {
  println!("{}", ai::environment::hello());

  let mut agent = Agent::new();

  println!("Starting simulation");
  for _ in 0..10 {
    let action = agent.act();
    println!("Got action {}", action);
    agent.update(2, 3.5);
  }
  
  println!("Average reward after {} rounds is {}", agent.age, agent.average_reward());
}
