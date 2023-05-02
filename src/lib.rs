/// Define the target for a notification.
#[derive(Debug)]
pub struct Client {
    /// The URL of the ntfy server, e.g. `"https://ntfy.sh"`.
    pub server: String,

    /// A topic which acts like a channel that receives notifications for that topic, e.g. `backup`.
    pub topic: String,

    /// A uuid (or another password like String) that will be appended to the topic for privacy reasons.
    pub uuid: String,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new(server: &str, topic: &str, uuid: &str) -> Self {
        Self {
            server: server.into(),
            topic: topic.into(),
            uuid: uuid.into(),
        }
    }
}

/// Define the content of a notification.
#[derive(Debug, Clone)]
pub struct Message {
    /// The title of the notification. Defaults to `server/topic_uuid` if `None`.
    title: Option<String>,

    /// The body text of the notification.
    message: String,

    /// One tag or multiple tags as a comma separated String, e.g. `important` or "foo,bar"`.
    ///
    /// If a tag matches an [emoji short code](https://docs.ntfy.sh/emojis/), it'll be converted to an emoji
    /// and prepended to title, otherwise it will be listed below the notification.
    ///
    /// Will simply be ignored if `None`.
    tags: Option<String>,
}

impl Message {
    /// Constructs a new `Message` via the `MessageBuilder` pattern.
    pub fn builder(message: &str) -> MessageBuilder {
        MessageBuilder::new(message)
    }
}

#[derive(Debug)]
pub struct MessageBuilder {
    /// The title of the notification. Defaults to `server/topic_uuid` if `None`.
    title: Option<String>,

    /// The body text of the notification.
    message: String,

    /// One tag or multiple tags as a comma separated String, e.g. `important` or "foo,bar"`.
    ///
    /// If a tag matches an [emoji short code](https://docs.ntfy.sh/emojis/), it'll be converted to an emoji
    /// and prepended to title, otherwise it will be listed below the notification.
    ///
    /// Will simply be ignored if `None`.
    tags: Option<String>,
}

impl MessageBuilder {
    fn new(message: &str) -> MessageBuilder {
        MessageBuilder {
            title: None,
            message: message.to_string(),
            tags: None,
        }
    }

    /// Adds an optional title to the message.
    pub fn title(self, title: &str) -> MessageBuilder {
        MessageBuilder {
            title: Some(title.to_string()),
            ..self // struct update syntax -> copies the rest of the fields from the current to the new instance
        }
    }

    /// Adds one or more optional tags to the message, e.g. `"foo"` or "foo,bar"`.
    pub fn tags(self, tags: &str) -> Self {
        Self {
            tags: Some(tags.to_string()),
            ..self
        }
    }

    /// Create a `Message` from a `MessageBuilder`.
    pub fn build(self) -> Message {
        Message {
            title: self.title,
            message: self.message,
            tags: self.tags,
        }
    }
}

/// Sends a notifications and returns the response of the POST request.
pub async fn ntfy(
    cli: &Client,
    msg: Message,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let url = format!("{}/{}_{}", cli.server, cli.topic, cli.uuid);
    let title = match msg.title {
        Some(s) => s,
        None => "".into(),
    };
    let tags = match msg.tags {
        Some(s) => s,
        None => "".into(),
    };
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .header("Title", title)
        .header("Tags", tags)
        .body(msg.message)
        .send()
        .await?;

    Ok(res)
}
