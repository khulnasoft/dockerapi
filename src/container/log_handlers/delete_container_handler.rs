use curl::easy::{Handler, WriteError};

/// A handler for processing HTTP responses while deleting containers.
pub struct DeleteContainerHandler<H: Handler> {
    pub error_message: Option<String>,  // To store any error message if needed
    pub accumulator: Vec<u8>,           // Accumulator to store data from the response
    handler: H,                          // The actual handler passed in the constructor
}

impl<H: Handler> DeleteContainerHandler<H> {
    /// Creates a new `DeleteContainerHandler` with the given handler.
    pub fn new(handler: H) -> Self {
        Self {
            error_message: None,            // Default is None, no error initially
            accumulator: Vec::new(),        // Default to an empty vector
            handler,                        // Pass the provided handler
        }
    }
}

impl<H: Handler> Handler for DeleteContainerHandler<H> {
    /// Called when there is data to write in the response.
    ///
    /// The method passes the data to the wrapped handler and collects it into
    /// the accumulator.
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        // Pass the data to the actual handler's write method
        self.handler.write(data)?;

        // Extend the accumulator with the incoming data
        self.accumulator.extend_from_slice(data);

        // Return the number of bytes written
        Ok(data.len())
    }
}
