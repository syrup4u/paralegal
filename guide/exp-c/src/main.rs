// exp-c: Harness-defined process (analogous to partner's exp-4).
// The process marker is on a local function, not a lib function.
// Sensitive data flows through it before reaching the sink. Policy ACB should PASS.

#[derive(Debug)]
#[paralegal::marker(sensitive)]
struct Message {
    content: Vec<u8>,
    author: String,
}

// process defined in the harness, not in mark_lib
#[paralegal::marker(process, arguments = [0])]
fn encode_content(content: &[u8]) -> Vec<u8> {
    content.to_vec()
}

#[paralegal::marker(sink, arguments = [0])]
fn publish_bytes(_data: &[u8]) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    let msg = Message {
        content: b"Hello, network!".to_vec(),
        author: "alice".to_string(),
    };

    // Send sensitive content through the harness process marker before publishing.
    let encoded = encode_content(&msg.content);
    publish_bytes(&encoded);
}
