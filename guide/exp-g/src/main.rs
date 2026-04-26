#[derive(Debug)]
#[paralegal::marker(sensitive)]
struct Message {
    content: Vec<u8>,
}

#[paralegal::marker(process, arguments = [0])]
fn scrub_message(message: &mut Message) {
    message.content.clear();
}

#[paralegal::marker(sink, arguments = [0])]
fn publish_message(_message: &Message) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    let mut msg = Message {
        content: b"Hello, network!".to_vec(),
    };

    // This is meant to be the cleaning step, but it keeps the same Message type.
    scrub_message(&mut msg);

    // The policy still treats this as the original sensitive value reaching a sink.
    publish_message(&msg);
}
