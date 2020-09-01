pub struct Post {
    contents: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            contents: String::new(),
        }
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }
}

pub struct DraftPost {
    contents: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.contents.push_str(text)
    }

    pub fn request_review(self) -> PendingPost {
        PendingPost {
            contents: self.contents,
            reviews_needed: 2,
            ..Default::default() // just for fun
        }
    }
}

#[derive(Default)]
pub struct PendingPost {
    contents: String,
    reviews_needed: u32,
    reviews_count: u32,
}

impl PendingPost {
    pub fn approve(&mut self) {
        self.reviews_count += 1
    }

    pub fn publish(self) -> Option<Post> {
        if self.reviews_count < self.reviews_needed {
            return None;
        }

        return Some(Post {
            contents: self.contents,
        });
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            contents: self.contents,
        }
    }
}
