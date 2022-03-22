// run-rustfix
#![warn(clippy::trim_split_whitespace)]
use std::ops::Deref;

/// Custom implements Deref<Target=str>, just like String does
struct Custom(String);

impl Deref for Custom {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

/// OtherStruct has custom custom trim() and split_whitespace() methods
struct OtherStruct {}

impl OtherStruct {
    fn trim(self) -> Self {
        self
    }
    fn split_whitespace(self) {}
}

/// OhNo implements Deref<Target=str>, but also has custom custom trim() and split_whitespace()
/// methods the custom implementations will take precedence
struct OhNo(String);

impl Deref for OhNo {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl OhNo {
    fn trim(self) -> Self {
        self
    }
    fn split_whitespace(self) {}
}

fn main() {
    // &str
    let _ = " A B C ".trim().split_whitespace(); // should trigger lint
    let _ = " A B C ".trim_start().split_whitespace(); // should trigger lint
    let _ = " A B C ".trim_end().split_whitespace(); // should trigger lint

    // String
    let _ = (" A B C ").to_string().trim().split_whitespace(); // should trigger lint

    // Deref<Target=str>
    let c = Custom(" A B C ".to_string());
    let _ = c.trim().split_whitespace(); // should trigger lint

    // Custom impl trim() and split_whitespace()
    let _ = OtherStruct {}.trim().split_whitespace(); // should not trigger lint

    // Custom impl trim() and split_whitespace() and Deref<Target=str>
    let _ = OhNo(" A B C ".to_string()).trim().split_whitespace(); // should not trigger lint
}
