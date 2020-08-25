pub struct Post {
    // `Option` lets us move the state value out of Post rather than borrowing it
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {
                content: String::new(),
            })),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text(text));
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review(self))
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(self));
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    // This syntax means the method is only valid when called on a Box holding the type
    fn request_review(self: Box<Self>, post: &mut Post) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>, post: &Post) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn add_text(self: Box<Self>, text: &str) -> Box<dyn State>;
}

struct Draft {
    content: String,
}

impl State for Draft {
    fn request_review(self: Box<Self>, post: &mut Post) -> Box<dyn State> {
        post.content = self.content;
        Box::new(PendingReview { approve_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>, _post: &Post) -> Box<dyn State> {
        self
    }

    fn add_text(mut self: Box<Self>, text: &str) -> Box<dyn State> {
        self.content.push_str(text);
        self
    }
}

struct PendingReview {
    approve_count: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.approve_count += 1;
        if self.approve_count < 2 {
            self
        } else {
            Box::new(Published {})
        }
    }

    fn reject(self: Box<Self>, post: &Post) -> Box<dyn State> {
        Box::new(Draft {
            content: post.content.clone(),
        })
    }

    fn add_text(self: Box<Self>, _text: &str) -> Box<dyn State> {
        self
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>, _post: &mut Post) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>, _post: &Post) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn add_text(self: Box<Self>, _text: &str) -> Box<dyn State> {
        self
    }
}
