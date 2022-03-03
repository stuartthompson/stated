pub struct TextDocument<'a> {
    content: &'a str
}

impl<'a> TextDocument<'a> {
    pub fn new(content: &'a str) -> TextDocument<'a> {
        TextDocument { content: content }
    }

    pub fn get_content(&self) -> &'a str {
        self.content
    }
}