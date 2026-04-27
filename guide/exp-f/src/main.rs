use p2panda_core::Body;

#[derive(Debug)]
#[paralegal::marker(sensitive)]
struct Message {
    content: Vec<u8>,
}

#[paralegal::marker(source, return)]
fn make_message(content: &[u8]) -> Message {
    Message {
        content: content.to_vec(),
    }
}

#[paralegal::marker(sink, arguments = [0])]
fn publish_raw(_content: &[u8]) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    // This message takes the "good" path through the library process marker.
    let msg1 = make_message(b"processed message");
    let _body = Body::new(&msg1.content);

    // This one is the actual bad path, but the policy can still match it
    // separately from msg1.
    let msg2 = make_message(b"unprocessed message");
    publish_raw(&msg2.content);
}
