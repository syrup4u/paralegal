// exp-e: AnotB, sensitive must never reach any sink directly.
// Checks that raw message content is never logged or printed.
// Here msg is accidentally printed, so policy AnotB should FAIL.

#[derive(Debug)]
#[paralegal::marker(sensitive)]
struct Message {
    content: Vec<u8>,
    author: String,
}

#[paralegal::analyze]
fn main() {
    let msg = Message {
        content: b"Hello, network!".to_vec(),
        author: "alice".to_string(),
    };

    // Accidental debug log: sensitive data reaches println.
    println!("Sending message from {}: {:?}", msg.author, msg.content);
}
