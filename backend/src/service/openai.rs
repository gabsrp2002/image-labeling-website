use openai_api_rust::{Auth, OpenAI};
use openai_api_rust::chat::{ChatApi, ChatBody};
use openai_api_rust::apis::{Message, Role};
use serde_json;
use std::env;

pub struct OpenAIService {
    client: Option<OpenAI>,
}

impl OpenAIService {
    pub fn new() -> Self {
        let client = match env::var("OPENAI_API_KEY") {
            Ok(api_key) => {
                println!("OpenAI API key found, initializing client");
                let auth = Auth::new(&api_key);
                Some(OpenAI::new(auth, "https://api.openai.com/v1/"))
            }
            Err(_) => {
                println!("OpenAI API key not found, will use mock suggestions");
                None
            }
        };

        Self { client }
    }

    pub async fn suggest_tags(
        &self,
        base64_data: &str,
        filetype: &str,
        group_tags: &[String],
        ignored_tags: &[String],
    ) -> Result<Vec<String>, String> {
        match &self.client {
            Some(client) => {
                self.call_openai_api(client, base64_data, filetype, group_tags, ignored_tags).await
            }
            None => {
                self.generate_mock_suggestions(group_tags, ignored_tags)
            }
        }
    }

    async fn call_openai_api(
        &self,
        client: &OpenAI,
        base64_data: &str,
        filetype: &str,
        group_tags: &[String],
        ignored_tags: &[String],
    ) -> Result<Vec<String>, String> {
        // Filter available tags by removing ignored tags
        let available_tags: Vec<String> = group_tags
            .iter()
            .filter(|tag| !ignored_tags.contains(tag))
            .cloned()
            .collect();

        let prompt = format!(
            "You are an AI assistant that suggests relevant tags for images. 
            Based on the image provided, suggest all relevant tags from the available list.
            
            Available tags: {}
            
            Example: [\"tag1\", \"tag2\", \"tag3\"]
            You ***must*** return ***only*** the tag names as in the example, nothing else.",
            available_tags.join(", ")
        );

        // Create the image URL with base64 data
        let image_url = format!("data:{};base64,{}", filetype, base64_data);

        let messages = vec![
            Message {
                role: Role::System,
                content: "You are a helpful AI assistant that suggests relevant image tags based on available options.".to_string(),
            },
            Message {
                role: Role::User,
                content: serde_json::json!([
                    {
                        "type": "text",
                        "text": prompt
                    },
                    {
                        "type": "image_url",
                        "image_url": {
                            "url": image_url
                        }
                    }
                ]).to_string(),
            },
        ];

        let body = ChatBody {
            model: "gpt-4o".to_string(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(150),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        };

        match client.chat_completion_create(&body) {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(message) = &choice.message {
                        let content = &message.content;
                    
                        // Parse the JSON response
                        match serde_json::from_str::<Vec<String>>(content) {
                            Ok(suggestions) => {
                                // Filter suggestions to only include available tags and limit to 3
                                let filtered_suggestions: Vec<String> = suggestions
                                    .into_iter()
                                    .filter(|suggestion| available_tags.contains(suggestion))
                                    .collect();
                                
                                println!("Ai suggestions: {:?}", filtered_suggestions);
                                Ok(filtered_suggestions)
                            }
                            Err(e) => {
                                eprintln!("Failed to parse OpenAI response as JSON: {}", e);
                                eprintln!("Raw response: {}", content);
                                // Fallback to mock suggestions
                                self.generate_mock_suggestions(group_tags, ignored_tags)
                            }
                        }
                    } else {
                        eprintln!("No message in OpenAI response choice");
                        self.generate_mock_suggestions(group_tags, ignored_tags)
                    }
                } else {
                    eprintln!("No choices in OpenAI response");
                    self.generate_mock_suggestions(group_tags, ignored_tags)
                }
            }
            Err(e) => {
                eprintln!("OpenAI API error: {}", e);
                // Fallback to mock suggestions
                self.generate_mock_suggestions(group_tags, ignored_tags)
            }
        }
    }

    fn generate_mock_suggestions(
        &self,
        group_tags: &[String],
        ignored_tags: &[String],
    ) -> Result<Vec<String>, String> {
        let available_tags: Vec<String> = group_tags
            .iter()
            .filter(|tag| !ignored_tags.contains(tag))
            .cloned()
            .collect();
        
        // Return up to 3 random suggestions from available tags
        let mut suggestions = available_tags;
        suggestions.truncate(3);
        
        Ok(suggestions)
    }
}

impl Default for OpenAIService {
    fn default() -> Self {
        Self::new()
    }
}
