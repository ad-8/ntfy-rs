/// Specify the target and content of a notification.
pub struct Options {
    /// The URL of the ntfy server, e.g. `"http://ntfy.sh"`.
    pub server: String,

    /// A topic which acts like a channel that receives notifications for that topic, e.g. `backup`.
    pub topic: String,

    /// A uuid (or another password like String) that will be appended to the topic for privacy reasons.
    pub uuid: String,

    /// The title of the notification. Defaults to `server/topic_uuid` if `None`.
    pub title: Option<String>,

    /// The body text of the notification.
    pub message: String,

    /// One tag or multiple tags as a comma separated String, e.g. `important` or "foo,bar"`.
    ///
    /// If a tag matches an [emoji short code](https://docs.ntfy.sh/emojis/), it'll be converted to an emoji
    /// and prepended to title, otherwise it will be listed below the notification.
    ///
    /// Will simply be ignored if `None`.
    pub tags: Option<String>,
}

/// Sends a notifications as specified by the options and returns the response of the POST request.
pub async fn ntfy(opts: Options) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let url = format!("{}/{}_{}", opts.server, opts.topic, opts.uuid);
    let title = match opts.title {
        Some(s) => s,
        None => "".into(),
    };
    let tags = match opts.tags {
        Some(s) => s,
        None => "".into(),
    };
    let client = reqwest::Client::new();

    let res = client
        .post(url)
        .header("Title", title)
        .header("Tags", tags)
        .body(opts.message)
        .send()
        .await?;

    Ok(res)
}
