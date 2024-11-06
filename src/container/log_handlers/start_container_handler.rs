use curl::easy::{Handler, WriteError};
use serde_json::Value;

pub struct StartContainerHandler<H: Handler> {
    pub image_id: Option<String>,
    pub error_message: Option<String>,
    handler: H,
}

impl<H: Handler> StartContainerHandler<H> {
    /// Creates a new `StartContainerHandler` with the provided handler.
    pub fn new(handler: H) -> Self {
        Self {
            image_id: None,
            error_message: None,
            handler,
        }
    }
}

impl<H: Handler> Handler for StartContainerHandler<H> {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        // Pass the data to the handler's write method
        self.handler.write(data)?;

        // Convert data to UTF-8 and process it line by line
        if let Ok(logs) = std::str::from_utf8(data) {
            for line in logs.lines() {
                if !line.trim().is_empty() {
                    // Try to parse the line as JSON
                    if let Ok(json) = serde_json::from_str::<Value>(line) {
                        // Check if the JSON contains a "message" field
                        if let Some(message) = json["message"].as_str() {
                            self.error_message = Some(message.to_string());
                        }
                    }
                }
            }
        } else {
            // Handle the error if the data is not valid UTF-8
            eprintln!("Failed to convert response data to UTF-8.");
        }

        Ok(data.len())
    }
}
