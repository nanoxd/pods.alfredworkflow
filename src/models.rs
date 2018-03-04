#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SearchResponse {
  allocations: Vec<Allocation>,
}

impl SearchResponse {
  pub fn into_allocation(&mut self) -> Option<Allocation> {
    if self.allocations.is_empty() {
      None
    } else {
      Some(self.allocations.remove(0))
    }
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

#[cfg(test)]
mod tests {
  use super::*;

  fn test_pod() -> Pod {
    Pod {
      id: "RxSwift".to_string(),
      version: "4.2.0".to_string(),
      summary: "RxSwift is a ReactiveX ...".to_string(),
      link: "https://github.com/ReactiveX/RxSwift".to_string(),
      source: Source {
        git: "https://github.com/ReactiveX/RxSwift".to_string(),
      },
    }
  }

  #[test]
  fn test_into_allocation() {
    let mut response = SearchResponse::default();
    assert!(response.into_allocation().is_none());

    let mut valid_response = SearchResponse {
      allocations: vec![Allocation("".to_string(), 0.0, 0, vec![], vec![], vec![])],
    };

    let allocation = valid_response.into_allocation();
    assert!(allocation.is_some());
    let allocation = allocation.unwrap();
    assert!(allocation.pods().is_empty());
  }

  #[test]
  fn test_pod_url() {
    let pod = test_pod();
    assert_eq!(pod.url(), "https://cocoapods.org/pods/RxSwift");
  }

  #[test]
  fn test_pod_stanza() {
    let pod = test_pod();
    assert_eq!(pod.stanza(), "pod 'RxSwift', '~> 4.2.0'");
  }
}
