use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// `Model` enum represents the available OpenAI models.
///
/// This enum provides an easy way to specify the model to be used in the API calls.
/// Currently supported models are:

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(non_camel_case_types)] // Add this line to suppress the warning
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    #[serde(rename = "gpt-4")]
    Gpt_4,
    #[serde(rename = "gpt-4-32k")]
    Gpt_4_32k,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt_4Turbo,
    #[serde(rename = "gpt-4o")]
    Gpt_4o,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt_4Turbo_Vision,
}

impl Model {
    pub fn max_tokens(&self) -> usize {
        match self {
            Model::Gpt3_5Turbo => 4096,
            Model::Gpt_4 => 8192,
            Model::Gpt_4_32k => 32768,
            Model::Gpt_4o => 128000,
            Model::Gpt_4Turbo => 128000,
            Model::Gpt_4Turbo_Vision => 128000,
        }
    }
}

/// Implement Display to convert the enum back to a string representation.
impl Display for Model {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let model_name = match self {
            Model::Gpt3_5Turbo => "gpt-3.5-turbo",
            Model::Gpt_4 => "gpt-4",
            Model::Gpt_4_32k => "gpt-4-32k",
            Model::Gpt_4o => "gpt-4o",
            Model::Gpt_4Turbo => "gpt-4-1106-preview",
            Model::Gpt_4Turbo_Vision => "gpt-4-vision-preview",
        };
        write!(f, "{model_name}")
    }
}

/// Implement `FromStr` to enable parsing the enum from a string representation.
impl FromStr for Model {
    type Err = ModelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gpt-3.5-turbo" => Ok(Model::Gpt3_5Turbo),
            "gpt-4" => Ok(Model::Gpt_4),
            "gpt-4-32k" => Ok(Model::Gpt_4_32k),
            "gpt-4o" => Ok(Model::Gpt_4o),
            "gpt-4-1106-preview" => Ok(Model::Gpt_4Turbo),
            "gpt-4-vision-preview" => Ok(Model::Gpt_4Turbo_Vision),
            _ => Err(ModelError::UnsupportedModel(s.into())),
        }
    }
}

/// A model parsing issues.
#[derive(Error, Debug)]
pub enum ModelError {
    /// Unknown or not supported model.
    #[error("Unsupported model: {0}")]
    UnsupportedModel(String),
}

/// `LogitBias` struct represents the logit bias used in API calls.
///
/// The struct contains a HashMap where keys are token IDs and values are biases.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LogitBias {
    pub biases: HashMap<u32, f64>,
}

/// Represents the role of a message in the Chat API call.
///
/// The `Role` enum has three variants:
/// - `System`: Represents a system message, usually to provide instructions to the assistant.
/// - `User`: Represents a user message, which is the input or question the user provides.
/// - `Assistant`: Represents an assistant message, which is the response generated by the Chat API.
///
/// The role is used to differentiate between different types of messages in the chat conversation.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
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
        assert!(
            model.is_ok(),
            "Failed to parse the gpt-3.5-turbo model name"
        );
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

    // Test the conversion of a valid model string to a Model enum variant for Gpt_4_32k
    #[test]
    fn test_from_str_gpt4_32k() {
        let input = "gpt-4-32k";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(model.is_ok(), "Failed to parse the gpt-4-32k model name");
        assert_eq!(model.unwrap(), Model::Gpt_4_32k);
    }
    // Test the conversion of a Model enum variant to its string representation for Gpt_4_32k:
    #[test]
    fn test_display_gpt4_32k() {
        let model = Model::Gpt_4_32k;
        let model_str = format!("{}", model);
        assert_eq!(model_str, "gpt-4-32k");
    }

    // Test the serialization of a Model enum variant to JSON for Gpt_4_32k:
    #[test]
    fn test_serialize_gpt4_32k() {
        let model = Model::Gpt_4_32k;
        let serialized_model = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized_model, "\"gpt-4-32k\"");
    }

    // Test the deserialization of a JSON string to a Model enum variant for Gpt_4_32k
    #[test]
    fn test_deserialize_gpt4_32k() {
        let model_json = "\"gpt-4-32k\"";
        let deserialized_model: Model = serde_json::from_str(model_json).unwrap();
        assert_eq!(deserialized_model, Model::Gpt_4_32k);
    }

    // Test the deserialization of a JSON string to a `Model` enum variant for Gpt3_5Turbo.
    #[test]
    fn test_deserialize_gpt4() {
        let model_json = "\"gpt-4\"";
        let deserialized_model: Model = serde_json::from_str(model_json).unwrap();
        assert_eq!(deserialized_model, Model::Gpt_4);
    }

    #[test]
    fn test_logit_bias_struct() {
        let mut biases = HashMap::new();
        biases.insert(42, 2.5);
        biases.insert(123, -1.3);

        let logit_bias = LogitBias { biases };

        assert_eq!(
            logit_bias.biases.get(&42),
            Some(&2.5),
            "Bias for token 42 should be 2.5"
        );
        assert_eq!(
            logit_bias.biases.get(&123),
            Some(&-1.3),
            "Bias for token 123 should be -1.3"
        );
        assert_eq!(
            logit_bias.biases.get(&999),
            None,
            "Bias for token 999 should not be set"
        );
    }

    #[test]
    fn test_max_tokens_gpt3_5turbo() {
        let model = Model::Gpt3_5Turbo;
        assert_eq!(model.max_tokens(), 4096);
    }

    #[test]
    fn test_max_tokens_gpt_4() {
        let model = Model::Gpt_4;
        assert_eq!(model.max_tokens(), 8192);
    }

    #[test]
    fn test_max_tokens_gpt_4_32k() {
        let model = Model::Gpt_4_32k;
        assert_eq!(model.max_tokens(), 32768);
    }

    // Test the conversion of a Model enum variant to its string representation for Gpt_4Turbo.
    #[test]
    fn test_display_gpt_4turbo() {
        let model = Model::Gpt_4Turbo;
        let model_str = format!("{}", model);
        assert_eq!(model_str, "gpt-4-1106-preview");
    }

    // Test the conversion of a valid model string to a Model enum variant for Gpt_4Turbo.
    #[test]
    fn test_from_str_gpt_4turbo() {
        let input = "gpt-4-1106-preview";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(
            model.is_ok(),
            "Failed to parse the gpt-4-1106-preview model name"
        );
        assert_eq!(model.unwrap(), Model::Gpt_4Turbo);
    }

    // Test the serialization of a Model enum variant to JSON for Gpt_4Turbo.
    #[test]
    fn test_serialize_gpt_4turbo() {
        let model = Model::Gpt_4Turbo;
        let serialized_model = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized_model, "\"gpt-4-1106-preview\"");
    }

    // Test the deserialization of a JSON string to a Model enum variant for Gpt_4Turbo.
    #[test]
    fn test_deserialize_gpt_4turbo() {
        let model_json = "\"gpt-4-1106-preview\"";
        let deserialized_model: Model = serde_json::from_str(model_json).unwrap();
        assert_eq!(deserialized_model, Model::Gpt_4Turbo);
    }

    // Test the conversion of a Model enum variant to its string representation for Gpt_4Turbo_Vision.
    #[test]
    fn test_display_gpt_4turbo_vision() {
        let model = Model::Gpt_4Turbo_Vision;
        let model_str = format!("{}", model);
        assert_eq!(model_str, "gpt-4-vision-preview");
    }

    // Test the conversion of a valid model string to a Model enum variant for Gpt_4Turbo_Vision.
    #[test]
    fn test_from_str_gpt_4turbo_vision() {
        let input = "gpt-4-vision-preview";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(
            model.is_ok(),
            "Failed to parse the gpt-4-vision-preview model name"
        );
        assert_eq!(model.unwrap(), Model::Gpt_4Turbo_Vision);
    }

    // Test the serialization of a Model enum variant to JSON for Gpt_4Turbo_Vision.
    #[test]
    fn test_serialize_gpt_4turbo_vision() {
        let model = Model::Gpt_4Turbo_Vision;
        let serialized_model = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized_model, "\"gpt-4-vision-preview\"");
    }

    // Test the deserialization of a JSON string to a Model enum variant for Gpt_4Turbo_Vision.
    #[test]
    fn test_deserialize_gpt_4turbo_vision() {
        let model_json = "\"gpt-4-vision-preview\"";
        let deserialized_model: Model = serde_json::from_str(model_json).unwrap();
        assert_eq!(deserialized_model, Model::Gpt_4Turbo_Vision);
    }

    // Test the max tokens for Gpt_4Turbo.
    #[test]
    fn test_max_tokens_gpt_4turbo() {
        let model = Model::Gpt_4Turbo;
        assert_eq!(model.max_tokens(), 128000);
    }

    // Test the max tokens for Gpt_4Turbo_Vision.
    #[test]
    fn test_max_tokens_gpt_4turbo_vision() {
        let model = Model::Gpt_4Turbo_Vision;
        assert_eq!(model.max_tokens(), 128000);
    }

    // Test the conversion of a Model enum variant to its string representation for Gpt_4o.
    #[test]
    fn test_display_gpt_4o() {
        let model = Model::Gpt_4o;
        let model_str = format!("{}", model);
        assert_eq!(model_str, "gpt-4o");
    }

    // Test the conversion of a valid model string to a Model enum variant for Gpt_4o.
    #[test]
    fn test_from_str_gpt_4o() {
        let input = "gpt-4o";
        let model: Result<Model, ()> = Model::from_str(input);
        assert!(model.is_ok(), "Failed to parse the gpt-4o model name");
        assert_eq!(model.unwrap(), Model::Gpt_4o);
    }

    // Test the serialization of a Model enum variant to JSON for Gpt_4o.
    #[test]
    fn test_serialize_gpt_4o() {
        let model = Model::Gpt_4o;
        let serialized_model = serde_json::to_string(&model).unwrap();
        assert_eq!(serialized_model, "\"gpt-4o\"");
    }

    // Test the deserialization of a JSON string to a Model enum variant for Gpt_4o.
    #[test]
    fn test_deserialize_gpt_4o() {
        let model_json = "\"gpt-4o\"";
        let deserialized_model: Model = serde_json::from_str(model_json).unwrap();
        assert_eq!(deserialized_model, Model::Gpt_4o);
    }

    // Test the max tokens for Gpt_4o.
    #[test]
    fn test_max_tokens_gpt_4o() {
        let model = Model::Gpt_4o;
        assert_eq!(model.max_tokens(), 128000);
    }
}
