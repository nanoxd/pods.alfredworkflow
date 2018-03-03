#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
  allocations: Vec<Allocation>,
}

impl SearchResponse {
  pub fn into_allocation(&mut self) -> Allocation {
    self.allocations.remove(0)
  }
}

#[derive(Serialize, Deserialize, Debug)]
// Allocation is a struct to avoid creating a custom deserializer as the response
// is array indexed
pub struct Allocation(String, f32, u32, Vec<Vec<String>>, Vec<u8>, Vec<Pod>);

impl Allocation {
  pub fn pods(self) -> Vec<Pod> {
    self.5
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pod {
  id: String,
  version: String,
  summary: String,
  link: String,
  source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
  git: String,
}
