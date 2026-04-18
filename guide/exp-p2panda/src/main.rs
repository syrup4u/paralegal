use p2panda_core::{Body, Header, Operation, PrivateKey, Timestamp};

#[derive(Debug)]
#[paralegal::marker(user_data)]
struct Message {
    content: Vec<u8>,
    author: String,
}

#[paralegal::marker(send, arguments = [0])]
fn publish(_operation: &Operation) {
    todo!()
}

fn unsigned_publish(_operation: &Operation) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    let private_key = PrivateKey::new();

    let msg = Message {
        content: b"Hello, network!".to_vec(),
        author: "alice".to_string(),
    };

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

    // Sign the header so the operation is cryptographically authenticated.
    header.sign(&private_key);

    let operation = Operation {
        hash: header.hash(),
        header,
        body: Some(body),
    };

    publish(&operation);

    // Intentional violation: publishing without signing first.
    let msg2 = Message {
        content: b"Sneaky message".to_vec(),
        author: "eve".to_string(),
    };
    let body2 = Body::new(&msg2.content);
    let mut header2 = Header {
        version: 1,
        public_key: private_key.public_key(),
        signature: None,
        payload_size: body2.size(),
        payload_hash: Some(body2.hash()),
        timestamp: Timestamp::now(),
        seq_num: 1,
        backlink: None,
        extensions: (),
    };
    // header2.sign(&private_key) deliberately omitted
    let _ = header2;
    unsigned_publish(&Operation {
        hash: header2.hash(),
        header: header2,
        body: Some(body2),
    });
}
