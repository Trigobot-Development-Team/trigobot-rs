/// Stores all relevant information to create a Discord message
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Message {
    pub author: String,
    pub title: String,
    pub content: String,
    pub link: Option<String>,
    pub timestamp: u64,
}

impl Message {
    pub fn new(
        author: String,
        title: String,
        content: String,
        link: Option<String>,
        timestamp: u64,
    ) -> Self {
        Message {
            author,
            title,
            content,
            link,
            timestamp,
        }
    }
}

impl Eq for Message {}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
