use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// `Model` enum represents the available OpenAI models.
///
/// This enum provides an easy way to specify the model to be used in the API calls.
/// Currently supported models are:
/// - Gpt3_5Turbo
/// - Gpt4
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(non_camel_case_types)] // Add this line to suppress the warning
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    Gpt_4,
}

/// Implement Display to convert the enum back to a string representation.
impl Display for Model {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let model_name = match self {
            Model::Gpt3_5Turbo => "gpt-3.5-turbo",
            Model::Gpt_4 => "gpt-4",
        };
        write!(f, "{}", model_name)
    }
}

/// Implement `FromStr` to enable parsing the enum from a string representation.
impl FromStr for Model {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-3.5-turbo" => Ok(Model::Gpt3_5Turbo),
            "gpt-4" => Ok(Model::Gpt_4),
            _ => Err(()),
        }
    }
}


/// `LogitBias` struct represents the logit bias used in API calls.
///
/// The struct contains a HashMap where keys are token IDs and values are biases.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LogitBias {
    pub biases: HashMap<u32, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // Test the conversion of a valid model string to a `Model` enum variant for Gpt3_5Turbo.
    #[test]
    fn test_from_str_gpt3_5turbo() {
        let input = "gpt-3.5-turbo";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(model.is_ok(), "Failed to parse the gpt-3.5-turbo model name");
        assert_eq!(model.unwrap(), Model::Gpt3_5Turbo);
    }

    // Test the conversion of a valid model string to a `Model` enum variant for Gpt4.
    #[test]
    fn test_from_str_gpt4() {
        let input = "gpt-4";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(model.is_ok(), "Failed to parse the gpt-4 model name");
        assert_eq!(model.unwrap(), Model::Gpt_4);
    }

    // Test the conversion of an invalid model string to a `Model` enum variant.
    #[test]
    fn test_from_str_invalid() {
        let input = "invalid-model";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(model.is_err(), "Parsed an invalid model name");
    }

    // Test the conversion of a `Model` enum variant to its string representation for Gpt3_5Turbo.
    #[test]
    fn test_display_gpt3_5turbo() {
        let model = Model::Gpt3_5Turbo;
        let model_str = format!("{}", model);
        assert_eq!(model_str, "gpt-3.5-turbo");
    }

    // Test the conversion of a `Model` enum variant to its string representation for Gpt4.
    #[test]
    fn test_display_gpt4() {
        let model = Model::Gpt_4;
        let model_str = format!("{}", model);
        assert_eq!(model_str, "gpt-4");
    }

    // Test the serialization of a `Model` enum variant to JSON for Gpt3_5Turbo.
    #[test]
    fn test_serialize_gpt3_5turbo() {
        let model = Model::Gpt3_5Turbo;
        let serialized_model = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized_model, "\"gpt-3.5-turbo\"");
    }

    // Test the serialization of a `Model` enum variant to JSON for Gpt4.
    #[test]
    fn test_serialize_gpt4() {
        let model = Model::Gpt_4;
        let serialized_model = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized_model, "\"gpt-4\"");
    }

    // Test the deserialization of a JSON string to a `Model` enum variant for Gpt3_5Turbo.
    #[test]
    fn test_deserialize_gpt3_5turbo() {
        let model_json = "\"gpt-3.5-turbo\"";
        let deserialized_model: Model = serde_json::from_str(model_json).unwrap();
        assert_eq!(deserialized_model, Model::Gpt3_5Turbo);
    }
}

