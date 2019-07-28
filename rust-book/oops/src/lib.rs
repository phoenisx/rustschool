pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

/**
 * There is a specific reason not using generics for struct Screen<T>
 * Because it restricts us to a Screen instance that has a list of
 * components all of type Button or all of type TextField
 *
 * Using Traits instead of Generics, has small bit of performance degradation due
 * to Dynamic Dispatch
 */
impl Screen {
  pub fn run(&self) {
    // Draw each component, when invalidated
    for component in self.components.iter() {
      component.draw()
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!(
      "Button: width: {}, height: {}, label: {}",
      self.width, self.height, self.label
    );
  }
}

/***********************************
 * Following Library part is for Blog Project
 * I don't care about the structure of the project for now
 * Just want to keep everything inside a single Cargo Project
 **********************************/

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    // Unwrap because we are sure, panic won't happen here ever...
    self.state.as_ref().unwrap().content(&self)
  }

  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(&mut self) {
    // `.take()` actually take the Ownership of the `state` property's
    // Option instance and returns only the populated `Some` value
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review());
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
  fn content<'a>(&self, post: &'a Post) -> &'a str;
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self // Since draft cannot be approved
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content // because we are return a reference to string tuple
  }
}
