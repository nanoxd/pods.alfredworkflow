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
  pub id: String,
  pub version: String,
  pub summary: String,
  pub link: String,
  pub source: Source,
}

impl Pod {
  pub fn url(&self) -> String {
    format!("https://cocoapods.org/pods/{}", self.id)
  }

  pub fn stanza(&self) -> String {
    format!("pod '{}', '~> {}'", self.id, self.version)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
  pub git: String,
}
