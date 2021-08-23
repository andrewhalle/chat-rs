use std::fmt::{self, Display, Formatter};

struct Message {
    text: String,
    sender: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}: {}", self.sender, self.text)
    }
}

fn get_messages() -> Vec<Message> {
    vec![
        Message {
            text: "Hello!".to_string(),
            sender: "Alice".to_string(),
        },
        Message {
            text: "How's it going!".to_string(),
            sender: "Bob".to_string(),
        },
    ]
}

fn main() {
    let messages = get_messages();

    for msg in &messages {
        println!("{}", msg);
    }
}
