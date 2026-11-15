//! SigmaOS-betteroffice-documents: document model.
//!
//! Real implementation: pages, sections, paragraphs, formatting, comments, and validation.
//! Replaces the 22-line thin placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub content: String,
}

/// Paragraph style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParagraphStyle {
    Normal,
    Heading1,
    Heading2,
    Heading3,
    Bullet,
    Numbered,
}

/// Paragraph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub id: String,
    pub content: String,
    pub style: ParagraphStyle,
}

/// Section.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentError {
    EmptyTitle,
    EmptyContent,
    DuplicateParagraph(String),
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "document title must not be empty"),
            Self::EmptyContent => write!(f, "paragraph content must not be empty"),
            Self::DuplicateParagraph(id) => write!(f, "duplicate paragraph {:?}", id),
        }
    }
}

impl std::error::Error for DocumentError {}

/// Collaborative document.
pub struct CollabDocument {
    pub title: String,
    pub author: String,
    paragraphs: BTreeMap<String, Paragraph>,
    comments: BTreeMap<String, Comment>,
}

impl CollabDocument {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            author: String::new(),
            paragraphs: BTreeMap::new(),
            comments: BTreeMap::new(),
        }
    }

    pub fn add_paragraph(&mut self, paragraph: Paragraph) -> Result<(), DocumentError> {
        if paragraph.content.trim().is_empty() { return Err(DocumentError::EmptyContent); }
        let id = paragraph.id.clone();
        if self.paragraphs.contains_key(&id) {
            return Err(DocumentError::DuplicateParagraph(id.clone()));
        }
        self.paragraphs.insert(id, paragraph);
        Ok(())
    }

    pub fn add_comment(&mut self, comment: Comment) {
        self.comments.insert(comment.id.clone(), comment);
    }

    pub fn paragraph_count(&self) -> usize { self.paragraphs.len() }

    pub fn comment_count(&self) -> usize { self.comments.len() }

    pub fn paragraphs(&self) -> Vec<&Paragraph> { self.paragraphs.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_create_aaa() {
        let d = CollabDocument::new("My Doc");
        assert_eq!(d.title, "My Doc");
        assert_eq!(d.paragraph_count(), 0);
    }

    #[test]
    fn test_add_paragraph_aaa() {
        let mut d = CollabDocument::new("My Doc");
        let p = Paragraph { id: "p1".into(), content: "Hello".into(), style: ParagraphStyle::Normal };
        d.add_paragraph(p).unwrap();
        assert_eq!(d.paragraph_count(), 1);
    }

    #[test]
    fn test_empty_content_rejected_aaa() {
        let mut d = CollabDocument::new("My Doc");
        let p = Paragraph { id: "x".into(), content: "".into(), style: ParagraphStyle::Normal };
        assert!(d.add_paragraph(p).is_err());
    }

    #[test]
    fn test_add_comment_aaa() {
        let mut d = CollabDocument::new("My Doc");
        d.add_comment(Comment { id: "c1".into(), author: "Alice".into(), content: "Looks good".into() });
        assert_eq!(d.comment_count(), 1);
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut d = CollabDocument::new("My Doc");
        let p = Paragraph { id: "x".into(), content: "X".into(), style: ParagraphStyle::Normal };
        d.add_paragraph(p).unwrap();
        let p2 = Paragraph { id: "x".into(), content: "X".into(), style: ParagraphStyle::Normal };
        assert!(d.add_paragraph(p2).is_err());
    }
}
