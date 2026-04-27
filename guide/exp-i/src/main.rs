// exp-i: Lib-bug case (p2panda-discovery).
// The harness looks correct — it just calls validate_topics as a pre-flight check
// before the PSI exchange. But validate_topics in mark_lib has a hidden println
// that leaks raw topic bytes to stdout.
// Policy AnotB (sensitive must NOT reach any sink) should FAIL.

use p2panda_core::Topic;
use p2panda_discovery::psi_hash::validate_topics;

#[derive(Clone)]
#[paralegal::marker(sensitive)]
struct SecretTopics {
    topics: Vec<Topic>,
}

#[paralegal::analyze]
fn main() {
    let local = SecretTopics {
        topics: vec![[7; 32].into(), [11; 32].into()],
    };

    // Looks harmless: just a pre-flight count check before the PSI exchange.
    // But validate_topics in the library has a hidden println that leaks raw topic bytes.
    let _count = validate_topics(&local.topics);
}
