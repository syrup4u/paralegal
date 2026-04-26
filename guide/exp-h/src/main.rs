use std::collections::HashSet;

use p2panda_core::Topic;
use p2panda_discovery::psi_hash::{PsiHashMessage, hash_vector};
use p2panda_store::address_book::NodeInfo;

type DiscoveryMessage = PsiHashMessage<u8, HarnessNodeInfo>;

#[derive(Clone)]
struct HarnessNodeInfo {
    id: u8,
}

impl NodeInfo<u8> for HarnessNodeInfo {
    type Transports = ();

    fn id(&self) -> u8 {
        self.id
    }

    fn is_bootstrap(&self) -> bool {
        false
    }

    fn is_stale(&self) -> bool {
        false
    }

    fn transports(&self) -> Option<Self::Transports> {
        None
    }
}

#[derive(Clone)]
#[paralegal::marker(sensitive)]
struct SecretTopics {
    topics: Vec<Topic>,
}

#[paralegal::marker(sink, arguments = [0])]
fn publish_discovery_message(_message: &DiscoveryMessage) {
    todo!()
}

#[paralegal::analyze]
fn main() {
    let local = SecretTopics {
        topics: vec![[7; 32].into(), [11; 32].into()],
    };

    // This follows p2panda-discovery's PSI shape: raw topics are salted and
    // hashed before they are put into a protocol message.
    let salt = [1; 65];
    let topics_for_alice = HashSet::from_iter(hash_vector(&local.topics, &salt).unwrap());
    let message = PsiHashMessage::BobSaltHalfAndHashedData {
        bob_salt_half: [2; 32],
        topics_for_alice,
    };

    publish_discovery_message(&message);
}
