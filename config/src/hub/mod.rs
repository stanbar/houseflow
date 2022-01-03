use houseflow_types::accessory;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub accessories: Vec<Accessory>,
    #[serde(default)]
    pub providers: Providers,
    #[serde(default)]
    pub controllers: Controllers,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Accessory {
    /// ID of the accessory
    pub id: accessory::ID,
    /// Name of the accessory
    pub name: String,
    /// Name of the room that the accessory is in
    pub room_name: String,
    /// Type of the accessory, possibly with additional parameters
    #[serde(flatten)]
    pub r#type: AccessoryType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "manufacturer", rename_all = "kebab-case")]
#[non_exhaustive]
pub enum AccessoryType {
    XiaomiMijia(manufacturers::XiaomiMijia),
    Houseflow(manufacturers::Houseflow),
}

pub mod manufacturers {
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "model", rename_all = "kebab-case")]
    #[non_exhaustive]
    pub enum XiaomiMijia {
        HygroThermometer {
            // TODO: Make it strictly typed
            #[serde(rename = "mac-address")]
            mac_address: String,
        },
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(tag = "model", rename_all = "kebab-case")]
    #[non_exhaustive]
    pub enum Houseflow {
        Gate,
        Garage,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Providers {
    #[serde(default)]
    pub hive: Option<HiveProvider>,
    #[serde(default)]
    pub mijia: Option<MijiaProvider>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HiveProvider {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MijiaProvider {}

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Controllers {
    #[serde(default)]
    pub hap: Option<HapController>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HapController {
    // TODO: Make it strictly typed
    pub pin: String,
    /// Name of the bridge
    pub name: String,
}

impl crate::Config for Config {
    const DEFAULT_TOML: &'static str = include_str!("default.toml");

    const DEFAULT_FILE: &'static str = "hub.toml";

    fn preprocess(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config as _;
    use houseflow_types::accessory;

    #[test]
    fn test_example() {
        let expected = Config {
            accessories: vec![Accessory {
                id: accessory::ID::parse_str("37c6a8bd-264c-4653-a641-c9b574207be5").unwrap(),
                name: String::from("Thermometer"),
                r#type: AccessoryType::XiaomiMijia(manufacturers::XiaomiMijia::HygroThermometer {
                    mac_address: String::from("A4:C1:38:EF:77:51"),
                }),
                room_name: "Bedroom".to_string(),
            }],
            providers: Providers {
                mijia: Some(MijiaProvider {}),
                hive: None,
            },
            controllers: Controllers {
                hap: Some(HapController {
                    pin: "12345678".to_string(),
                    name: "Awesome Hub".to_string(),
                }),
            },
        };

        println!(
            "--------------------\n\n Serialized: \n{}\n\n--------------------",
            toml::to_string(&expected).unwrap()
        );
        let config = Config::parse(include_str!("example.toml")).unwrap();
        assert_eq!(config, expected);
    }
}