#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AlgoliaResponse {
    pub hits: Vec<Pod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pod {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub homepage: String,
    pub source: Source,
}

impl Pod {
    pub fn title(&self) -> String {
        format!("{} ({})", self.name, self.version)
    }

    pub fn url(&self) -> String {
        format!("https://cocoapods.org/pods/{}", self.name)
    }

    pub fn stanza(&self) -> String {
        format!("pod '{}', '~> {}'", self.name, self.version)
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
            name: "RxSwift".to_string(),
            version: "4.2.0".to_string(),
            summary: "RxSwift is a ReactiveX ...".to_string(),
            homepage: "https://github.com/ReactiveX/RxSwift".to_string(),
            source: Source {
                git: "https://github.com/ReactiveX/RxSwift".to_string(),
            },
        }
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

    #[test]
    fn test_pod_title() {
        let pod = test_pod();
        assert_eq!(pod.title(), "RxSwift (4.2.0)");
    }
}
