//! The example in the [README](https://github.com/michaelni678/stated/blob/main/README.md).

use stated::stated;

/// Builds a message.
#[stated(states(HasRecipient, HasBody))]
pub struct MessageBuilder<#[stated] S> {
    recipients: Vec<String>,
    body: String,
}

#[stated]
impl<#[stated] S> MessageBuilder<S> {
    /// Create a new message builder.
    #[stated]
    pub fn new() -> MessageBuilder<_> {
        MessageBuilder {
            recipients: Vec::new(),
            body: String::new(),
        }
    }

    /// Add a recipient to the message.
    #[stated(assign(HasRecipient))]
    pub fn recipient(mut self, recipient: impl Into<String>) -> MessageBuilder<_> {
        self.recipients.push(recipient.into());
        _
    }

    /// Set the body of the message.
    #[stated(reject(HasBody), assign(HasBody))]
    pub fn body(mut self, body: impl Into<String>) -> Result<MessageBuilder<_>, &'static str> {
        let body = body.into();
        if !body.is_ascii() {
            return Err("Body contains non-ASCII characters");
        }

        self.body = body;
        Ok(_)
    }

    /// Build the message.
    #[stated(assert(HasRecipient))]
    pub fn build(self) -> String {
        let to = self.recipients.join(" & ");
        let mut body = self.body;

        if body.is_empty() {
            body.push_str("<empty body>");
        }

        format!("To: {to}\n{body}")
    }
}

#[allow(dead_code)]
fn main() -> Result<(), &'static str> {
    let message = MessageBuilder::new()
        .recipient("Bob")
        .recipient("Rob")
        .body("Hello, World!")?
        .build();

    println!("{message}");

    Ok(())
}
