#[derive(Debug, Clone)]
pub struct Comment { pub author: String, pub content: String }

#[derive(Debug, Default)]
pub struct CollabDocument { pub title: String, comments: Vec<Comment> }
impl CollabDocument {
    pub fn new(title: impl Into<String>) -> Self { Self { title: title.into(), comments: Vec::new() } }
    pub fn add_comment(&mut self, author: impl Into<String>, content: impl Into<String>) {
        self.comments.push(Comment { author: author.into(), content: content.into() });
    }
    pub fn comment_count(&self) -> usize { self.comments.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_collab() {
        let mut d = CollabDocument::new("Draft");
        d.add_comment("Alice", "Looks good");
        assert_eq!(d.comment_count(), 1);
    }
}
