<h1 align="center">Stated</h1>
<h3 align="center">Rust typestate, made simple</h3>
<div align="center">

[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-stated-58a78a?style=for-the-badge&logo=Docs.rs">](https://docs.rs/stated)
&nbsp;&nbsp;&nbsp;
[<img alt="crates.io" src="https://img.shields.io/crates/v/stated?style=for-the-badge&logo=Rust">](https://crates.io/crates/stated)
&nbsp;&nbsp;&nbsp;
[<img alt="github" src="https://img.shields.io/badge/github-stated-gray?style=for-the-badge&logo=GitHub&color=669bbc">](https://github.com/michaelni678/stated)

</div>

**Stated** simplifies working with the typestate pattern.

- **Why typestate pattern?**

    The typestate pattern can ensure that methods are called in the correct order at compile time. 
    A common use-case is with builder structs, where it guarantees that required fields are set 
    before calling `.build()`, removing the need for runtime checks.

## Example

The example below defines a `MessageBuilder` struct.

Before calling `MessageBuilder::build`, you must call `MessageBuilder::recipient` at least once.  
The `MessageBuilder::body` method can be called no more than once.

These constraints are enforced at compile time rather than runtime. 

```rust
use stated::stated;

/// A message, built with a [`MessageBuilder`].
pub type Message = String;

/// Builds a [`Message`].
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

    /// Build the [`Message`].
    #[stated(assert(HasRecipient))]
    pub fn build(self) -> Message {
        let to = self.recipients.join(" & ");
        let mut body = self.body;

        if body.is_empty() {
            body.push_str("<empty body>");
        }

        format!("To: {to}\n{body}")
    }
}

let message = MessageBuilder::new()
    .recipient("Bob")
    .recipient("Rob")
    .body("Hello, World!")?
    .build();
```

See the [examples](/examples) directory for more!
