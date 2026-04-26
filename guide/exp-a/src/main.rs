use p2panda_core::{Body, Header, Operation, PrivateKey, Timestamp};

#[derive(Debug)]
#[paralegal::marker(sensitive)]
struct Message {
    content: Vec<u8>,
    author: String,
}

#[paralegal::marker(sink, arguments = [0])]
fn publish(_operation: &Operation) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    let private_key = PrivateKey::new();

    let msg = Message {
        content: b"Hello, network!".to_vec(),
        author: "alice".to_string(),
    };

    // Route the message content through the library process marker.
    let body = Body::new(&msg.content);
    let mut header = Header {
        version: 1,
        public_key: private_key.public_key(),
        signature: None,
        payload_size: body.size(),
        payload_hash: Some(body.hash()),
        timestamp: Timestamp::now(),
        seq_num: 0,
        backlink: None,
        extensions: (),
    };
    header.sign(&private_key);

    let operation = Operation {
        hash: header.hash(),
        header,
        body: Some(body),
    };

    publish(&operation);
}
