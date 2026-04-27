// exp-b: Lib-bug case.
// The harness sends sensitive content through Body::new before calling a
// library helper. Body::to_bytes() in mark_lib has an injected println that
// leaks raw bytes to stdout.
// Policy AnotB (sensitive must NOT reach any sink) should FAIL.

use p2panda_core::Body;

#[derive(Debug)]
#[paralegal::marker(sensitive)]
struct Message {
    content: Vec<u8>,
}

#[paralegal::analyze]
fn main() {
    let msg = Message {
        content: b"Hello, network!".to_vec(),
    };

    // Send sensitive content through the library process marker.
    let body = Body::new(&msg.content);

    // This call is expected to expose the injected library-side println.
    let _raw = body.to_bytes();
}
