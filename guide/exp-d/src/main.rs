// exp-d: "Broken pipe" limitation.
// msg1 is properly encoded before publishing. msg2 goes directly to println! (sink)
// without process. Policy ACB may PASS incorrectly due to cross-instance matching,
// where msg1's Body::new covers the process requirement for msg2's sink path.

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

    // msg1 follows the expected processed path before publishing.
    let msg1 = Message {
        content: b"Public message".to_vec(),
        author: "alice".to_string(),
    };
    let body1 = Body::new(&msg1.content);
    let mut header1 = Header {
        version: 1,
        public_key: private_key.public_key(),
        signature: None,
        payload_size: body1.size(),
        payload_hash: Some(body1.hash()),
        timestamp: Timestamp::now(),
        seq_num: 0,
        backlink: None,
        extensions: (),
    };
    header1.sign(&private_key);
    let op1 = Operation {
        hash: header1.hash(),
        header: header1,
        body: Some(body1),
    };
    publish(&op1);

    // msg2 reaches println without the library process marker.
    let msg2 = Message {
        content: b"Secret data".to_vec(),
        author: "alice".to_string(),
    };
    println!("{:?}", msg2.content);
}
