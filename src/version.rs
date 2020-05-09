#[derive(Debug, PartialEq)]
pub enum VersionValue {
  MAJOR,
  MINOR,
  PATCH,
}

#[derive(Debug, PartialEq)]
pub struct Version {
  pub value: VersionValue,
}

impl Version {
  pub fn new(value: String) -> Version {
    if value.to_lowercase() == "major" {
      return Version {
        value: VersionValue::MAJOR,
      };
    } else if value.to_lowercase() == "minor" {
      return Version {
        value: VersionValue::MINOR,
      };
    } else if value.to_lowercase() == "patch" {
      return Version {
        value: VersionValue::PATCH,
      };
    } else {
      panic!("Invalid version input!")
    };
  }
}

#[test]
fn version_returns_version_value() {
  assert_eq!(Version::new("Major".to_string()).value, VersionValue::MAJOR);
  assert_eq!(Version::new("minor".to_string()).value, VersionValue::MINOR);
  assert_eq!(Version::new("patch".to_string()).value, VersionValue::PATCH);
}
