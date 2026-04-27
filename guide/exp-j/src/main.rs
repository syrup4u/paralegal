// exp-j: Bypass violation (p2panda-discovery).
// hash_vector in mark_lib is marked as process — it hashes topics before network exchange.
// The harness accidentally skips it and sends raw sensitive topics directly to broadcast.
// Policy ACB (sensitive must reach sink only via process) should FAIL.

use std::collections::HashSet;

use p2panda_core::Topic;

#[derive(Clone)]
#[paralegal::marker(sensitive)]
struct SecretTopics {
    topics: Vec<Topic>,
}

#[paralegal::marker(sink, arguments = [0])]
fn broadcast(_topics: &HashSet<Topic>) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    let local = SecretTopics {
        topics: vec![[7; 32].into(), [11; 32].into()],
    };

    // BUG: should call hash_vector from p2panda-discovery (process) first.
    // Developer forgot the hashing step; raw sensitive topic bytes flow directly to broadcast.
    let raw_set: HashSet<Topic> = local.topics.iter().cloned().collect();
    broadcast(&raw_set);
}
