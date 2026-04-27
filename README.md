# Paralegal Experiment

This repo includes experiments for Paralegal.

## Goal

1. Understand how Paralegal is used and what might be the problem when using it.
2. Evaluate Paralegal's capability and limitations on commonly used libraries (https://crates.io/).

## Target Libraries

- [jsonwebtoken](https://github.com/Keats/jsonwebtoken/tree/master)
- [p2panda](https://github.com/p2panda/p2panda/tree/main)

## Rust Installation

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Environment

1. You should have the Rust installed, version > 1.85 (Now is 1.94.1).
2. Paralegal works on 1.84, it has defined that in `rust-toolchain` so no worry.
3. All crates in the experiments should not require the Rust version higher than 1.84.

## How to Run

Base:

1. `git clone --recursive https://github.com/syrup4u/paralegal.git`
2. `cd paralegal; cargo install --locked --path crates/paralegal-flow`
3. `cargo paralegal-flow --version`

Move to the work directory: `cd guide/policy`

Run tests with `bash run.sh [test number]`.

## Used Policies

For checking libraries:

1. `A does not go to B`: A is sensitive data, B is output sink like prinln!, dbg, format, io.write, etc.
2. `A goes to B only via C`: A is sensitive data, B is output sink, C is process like encode, sign, etc.

## Some Discovered Limitations

> Exists multiple instances that produce multiple `A`s that appear in `A goes to B`.

It lacks a way to handle this case. Refers to [exp-1](#exp-1) for more details and a reported GitHub [issue](https://github.com/brownsys/paralegal/issues/186).

![fig01](./limit1.png)

> `A goes to B only via C`, but C does not change the type of A.

It cannot handle such recursion: after A being processed by C, a "new" A is "created" and thus if it goes to B, it fails because of "not via C". Figure below explains this scenario. Refers to [exp-4](#exp-4) and [exp-5](#exp-5).

![fig02](./limit2.png)

## Methodology

- Scan whole libraries to identify potential leakage. (If not exists, inject one)
- A harness main as a placeholder is required since Paralegal is designed for applications rather than libraries.

## Overall Results

| Crate | LoC | Total Time | Marker | PDG / Seen Functions |
| --- | --- | --- | --- | --- |
| `jsonwebtoken` | 3,847 | 32.957 s | 7 | 10 / 341 |
| `p2panda-core` | 3,404 | 12.820 s | 7 | 1 / 323 |
| `p2panda-discovery` | 1,602 | 14.258 s | 7 | 2 / 9 |

## Explanation of Experiments

| Exp | Crate | Desc | Policy | Result |
| -- | -- | -- | -- | -- |
| [Original Toy Case](#original-toy-case) | n/a | A toy case provided by Paralegal's author | deletion | FAIL |
| [exp-1](#exp-1) | n/a | For limitation 1 (cross-instance matching) | deletion | PASS (incorrect) |
| [exp-2](#exp-2) | n/a | Supplement of the official toy case | deletion | FAIL |
| [exp-3](#exp-3) | jsonwebtoken | Lib-based process (`encode`) before send | ACB | PASS |
| [exp-4](#exp-4) | jsonwebtoken | For limitation 2 (`&mut` process) | ACB | FAIL (incorrect) |
| [exp-5](#exp-5) | jsonwebtoken | For limitation 2 (move process) | ACB | FAIL (incorrect) |
| [exp-6](#exp-6) | jsonwebtoken | Cross-crate analysis and marker order | AnotB | PASS (masked) |
| [exp-6-extended](#exp-6-extended) | jsonwebtoken | Secret key leakage injections in library internals | AnotB | PASS / FAIL |
| [exp-a](#exp-a) | p2panda-core | Lib-based process (`Body::new`) before publish | ACB | PASS |
| [exp-b](#exp-b) | p2panda-core | Hidden lib bug: `Body::to_bytes` leaks raw bytes to stdout | AnotB | FAIL |
| [exp-c](#exp-c) | p2panda-core | Harness-defined process before publish | ACB | PASS |
| [exp-d](#exp-d) | p2panda-core | Two-instance: one correct path, one violation | ACB | FAIL |
| [exp-e](#exp-e) | p2panda-core | Accidental debug log of sensitive data | AnotB | FAIL |
| [exp-f](#exp-f) | p2panda-core | For limitation 1 (cross-instance matching, p2panda variant) | existential process/sink | PASS (incorrect) |
| [exp-g](#exp-g) | p2panda-core | For limitation 2 (`&mut` process, p2panda variant) | ACB | FAIL (incorrect) |
| [exp-h](#exp-h) | p2panda-discovery | Lib-based process (`hash_vector`) before protocol message | ACB | PASS |
| [exp-i](#exp-i) | p2panda-discovery | Hidden lib bug: `validate_topics` leaks raw topics to stdout | AnotB | FAIL |
| [exp-j](#exp-j) | p2panda-discovery | Bypass violation: raw topics sent to broadcast without hashing | ACB | FAIL |

### Original Toy Case

The source code:

```rust
#[paralegal::analyze]
fn delete(user: User) {
    for doc in Document::for_user(&user) {
        doc.delete()
    }

    // Comment this back in to make the policy pass
    // for img in Image::for_user(&user) {
    //     img.delete()
    // }
}
```

The policy:

```txt
Scope:
Somewhere

Policy:
1. For each "user_data" type marked user_data:
    A. There is a "retrieval" that produces "user_data" where:
       a. There is a "deletes" marked deletes where:
	       i) "retrieval" goes to "deletes"

```

The command:

```sh
bash run.sh 0
```

The result:

```sh
error: Failed policy
note: `Scope somewhere`
  failed because no element matched body conditions
note: `For each "user_data" type marked user_data` (Rule 1)
  failed because of file_db_example::Image
note: `There is a "retrieval" that produces "user_data" where` (Rule 1.A)
  failed because no element matched initial filter
```

**Discussion:**

Actually, this is not a good example, at least it is not complete, as a demo in Paralegal. It only demonstrates that this function forgets to "produce" user's data, not forgets to delete user's data. That's why the error message here just stops at the rule 1.A. To understand what is a true complete demo, see our supplement in [exp-2](#exp-2).

### exp-1

The source code:

```rust
#[paralegal::analyze]
fn delete(user1: User, user2: User) { // Two users
    // Delete user1's documents
    for doc in Document::for_user(&user1) {
        doc.delete()
    }

    // Delete user2's images
    for img in Image::for_user(&user2) {
        img.delete()
    }
}
```

The policy is same as above ([toy case](#original-toy-case)).

The result:

```sh
Policy succeeded
```

**Discussion:**

The result is unexpected because the data of both user1 and user2 are not deleted completely. But why? If we look at the policy, we can find that:

1. Rule 1.: `For each "user_data" type marked user_data`. Yes, each "user_data" (i.e., `Image` and `Document`) does appear in this function.
2. Rule 1.A: `There is a "retrieval" that produces "user_data" where`. Yes, the "retrieval" derived from user1 produces `Image`, the "retrieval" derived from user2 produces `Document`.
3. Rule 1.A.a.i: `"retrieval" goes to "deletes"`. Yes, each "retrieval" does go to corresponding "delete".

That's why it can pass the policy.

In summary, Paralegal lacks a support for a policy similar to the below in order to deal with multiple instances:

```text
Scope:
Somewhere

Policy:
1. For each "user_data" type marked user_data:
    A. For each "source" type marked source that produces "user_data":
       a. There is a "deletes" marked deletes where:
	       i) "user_data" goes to "deletes"
```

### exp-2

The source code:

```rust
#[paralegal::analyze]
fn delete(user: User) {
    for doc in Document::for_user(&user) {
        doc.delete()
    }

    // Comment this back in to make the policy pass
    for img in Image::for_user(&user) {
        todo!()
    }
}
```

The command:

```sh
bash run.sh 2
```

The result:

```sh
error: Failed policy
note: `Scope somewhere`
  failed because no element matched body conditions
note: `For each "user_data" type marked user_data` (Rule 1)
  failed because of file_db_example::Image
note: `There is a "retrieval" that produces "user_data" where` (Rule 1.A)
  failed because no element matched body conditions
note: `"retrieval" goes to "deletes"` (Rule 1.A.a.i)
this source
  --> src/main.rs:64:16
   |
64 |     for img in Image::for_user(&user) {
   |                ^^^^^^^^^^^^^^^^^^^^^^
   |
note: does not go to
  --> src/main.rs:48:9
   |
48 |         std::fs::remove_file(
   |         ^^^^^^^^^^^^^^^^^^^^^
49 |             std::path::Path::new("db")
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^
50 |                 .join("doc")
   |                 ^^^^^^^^^^^^
51 |                 .join(format!("{}-{}.txt", self.user.name, self.name)),
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
52 |         )
```

**Discussion:**

This is the supplement test of the [toy case](#original-toy-case). Now it is complete because the error stack reaches `Rule 1.A.a.i` and tells us this function forgets to delete.

### exp-3

The source code:

```rust
#[paralegal::marker(sensitive)]
struct Claims {
    aud: String,
    sub: String,
    company: String,
    exp: u64,
}

#[paralegal::marker(sink, arguments = [0])]
fn send(_token: &str) {
    todo!()
}

fn fake_encode(_claims: &Claims) -> String {
    println!("{:?}", _claims);
    todo!()
}

#[paralegal::analyze]
fn main() {
    let key = b"secret";
    let my_claims = Claims {
        aud: "me".to_owned(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };

    // Comment this back in to make the policy fail
    // let token = fake_encode(&my_claims);

    // Comment this out to make the policy fail
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };

    send(&token);
}
```

The policy: `A goes to B via C` ([policy_ACB.txt](./guide/policy/policy_ACB.txt))

```txt
Scope:
Everywhere

Policy:
1. Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process
```

The command:

```sh
bash run.sh 4
```

Result 1: `Policy succeeded`

Result 2 (comment back in and comment out), the error message may vary with multiple runs:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of jwt_example::main
note: `Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process` (Rule 1)
source
  --> src/main.rs:20:5
   |
20 |     println!("{:?}", _claims);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: has data flow influence on this target without passing checkpoint
  --> src/main.rs:35:29
   |
35 |     let token = fake_encode(&my_claims);
   |                             ^^^^^^^^^^
   |
```

**Discussion:**

An experiment to see if `A goes to B only via C` works.

### exp-4

The source code:

```rust
#[paralegal::marker(sink, arguments = [0])]
fn send(_token: &Claims) {
    todo!()
}

#[paralegal::marker(process, arguments = [0])]
fn process_claims(_claims: &mut Claims) {
    _claims.exp = 0;
}

#[paralegal::analyze]
fn analyze_func() {
    let key = b"secret";
    let mut my_claims = Claims {
        aud: "me".to_owned(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };

    process_claims(&mut my_claims);
    send(&my_claims);
}
```

The policy: `A goes to B via C` ([policy_ACB.txt](./guide/policy/policy_ACB.txt))

The command:

```sh
bash run.sh 4
```

The result:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of jwt_example::analyze_func
note: `Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process` (Rule 1)
source
  --> src/main.rs:35:5
   |
35 |     send(&my_claims);
   |     ^^^^^^^^^^^^^^^^
   |
note: has data flow influence on this target without passing checkpoint
  --> src/main.rs:35:5
   |
35 |     send(&my_claims);
   |     ^^^^^^^^^^^^^^^^
   |
```

**Discussion:**

When applying the policy `A goes to B via C`, if C does not change the type of A, for example, by using mutable reference, it fails. A guess is a new "A" is created and it goes to B not via C.

### exp-5

The source code:

```rust
#[paralegal::marker(sink, arguments = [0])]
fn send(_token: &Claims) {
    todo!()
}

#[paralegal::marker(process, arguments = [0])]
fn process_claims(mut _claims: Claims) -> Claims {
    _claims.exp = 0;
    _claims
}

#[paralegal::analyze]
fn analyze_func() {
    let key = b"secret";
    let my_claims = Claims {
        aud: "me".to_owned(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        exp: 10000000000,
    };
    let after_claims = process_claims(my_claims);
    send(&after_claims);
}
```

The policy: `A goes to B via C` ([policy_ACB.txt](./guide/policy/policy_ACB.txt))

The command:

```sh
bash run.sh 5
```

The result:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of jwt_example::analyze_func
note: `Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process` (Rule 1)
source
  --> src/main.rs:35:5
   |
35 |     send(&after_claims);
   |     ^^^^^^^^^^^^^^^^^^^
   |
note: has data flow influence on this target without passing checkpoint
  --> src/main.rs:35:10
   |
35 |     send(&after_claims);
   |          ^^^^^^^^^^^^^
   |
```

**Discussion:**

This is an ownership transfer (move) version of [exp-4](#exp-4). It still fails.

### exp-6

The source code:

```rust
#[derive(Clone, Debug)]
#[paralegal::marker(sensitive)]
pub struct EncodingKey {
    pub(crate) family: AlgorithmFamily,
    content: Vec<u8>,
}

#[paralegal::marker(process, arguments = [1])]
pub fn encode<T: Serialize>(header: &Header, claims: &T, key: &EncodingKey) -> Result<String> {
    if key.family != header.alg.family() {
        return Err(new_error(ErrorKind::InvalidAlgorithm));
    }
    let encoded_header = b64_encode_part(header)?;
    let encoded_claims = b64_encode_part(claims)?;
    let message = [encoded_header, encoded_claims].join(".");
    let signature = crypto::sign(message.as_bytes(), key, header.alg)?;

    println!("Encoding key: {:?}", key);

    Ok([message, signature].join("."))
}
```

External Marker:

```toml
[["std::io::_print"]]
marker = "sink"
on_argument = [0]

[["std::io::_eprint"]]
marker = "sink"
on_argument = [0]

[["alloc::fmt::format"]]
marker = "sink"
on_argument = [0]

[["std::io::Write::write_fmt"]]
marker = "sink"
on_argument = [1]
```

The policy: `A does not go to B`

```txt
Scope:
Everywhere

Policy:
1. For each "sensitive" marked sensitive:
    A. For each "output" marked sink:
        a. "sensitive" does not go to "output"
```

The command:

```sh
bash run.sh 6
```

Result 1:

```sh
warning: Marker sink is mentioned in the policy but not defined in source

Policy succeeded
```

Result 2 (after commenting out the marker for "process", i.e., `#[paralegal::marker(process, arguments = [1])]`):

```sh
note: `"sensitive" does not go to "output"` (Rule 1.A.a)
this source
   --> /users/syrup/zzz/paralegal/guide/mark_lib/jsonwebtoken/src/encoding.rs:123:1
    |
123 | pub fn encode<T: Serialize>(header: &Header, claims: &T, key: &EncodingKey) -> Result<String> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: does go to
   --> /users/syrup/.rustup/toolchains/nightly-2024-12-15-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9
    |
143 |         $crate::io::_print($crate::format_args_nl!($($arg)*));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
```

**Discussion:**

It takes some time to figure it out. At first I thought it might be a problem, but it is mentioned in Paralegal's [document](https://justus-adam.notion.site/Dependency-Analysis-0e3b66b097754c91acfcca7c6231b5f4#22f18990e0e9801382a2ce674acd921e): a rule applied automatically when creating a PDG. Check out the second rule in its "Cross-Crate Analysis" section. The inner markers would be masked by the outermost marker.

### exp-6-extended

After commenting out the marker for "process", two cases are tested:

- No `io.write` / `println` are injected.
- Inject `println` to `encode` and `decode` to see if secret key leakage can be detected.

The source code:

```rust
pub fn sign(message: &[u8], key: &EncodingKey, algorithm: Algorithm) -> Result<String> {
    println!("Key is: {:?}", key);
    match algorithm {
        Algorithm::HS256 => Ok(sign_hmac(hmac::HMAC_SHA256, key.inner(), message)),
        Algorithm::HS384 => Ok(sign_hmac(hmac::HMAC_SHA384, key.inner(), message)),
        Algorithm::HS512 => Ok(sign_hmac(hmac::HMAC_SHA512, key.inner(), message)),

        Algorithm::ES256 | Algorithm::ES384 => {
            ecdsa::sign(ecdsa::alg_to_ec_signing(algorithm), key.inner(), message)
        }

        Algorithm::EdDSA => eddsa::sign(key.inner(), message),

        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => rsa::sign(rsa::alg_to_rsa_signing(algorithm), key.inner(), message),
    }
}

pub fn encode<T: Serialize>(header: &Header, claims: &T, key: &EncodingKey) -> Result<String> {
    if key.family != header.alg.family() {
        return Err(new_error(ErrorKind::InvalidAlgorithm));
    }
    let encoded_header = b64_encode_part(header)?;
    let encoded_claims = b64_encode_part(claims)?;
    let message = [encoded_header, encoded_claims].join(".");
    let signature = crypto::sign(message.as_bytes(), key, header.alg)?;

    println!("Encoding key: {:?}", key);

    Ok([message, signature].join("."))
}

pub fn decode<T: DeserializeOwned>(
    token: &str,
    key: &DecodingKey,
    validation: &Validation,
) -> Result<TokenData<T>> {
    match verify_signature(token, key, validation) {
        Err(e) => Err(e),
        Ok((header, claims)) => {
            let decoded_claims = DecodedJwtPartClaims::from_jwt_part_claims(claims)?;
            let claims = decoded_claims.deserialize()?;
            validate(decoded_claims.deserialize()?, validation)?;

            println!("Decoded key: {:?}", key);

            Ok(TokenData { header, claims })
        }
    }
}
```

The command:

```sh
bash run.sh 6
```

Result 1 (No injection):

```sh
warning: Marker sink is mentioned in the policy but not defined in source

Policy succeeded
```

Result 2 (Any injection, result may vary, only shows two of them):

```sh
error: Failed policy
...
note: `"sensitive" does not go to "output"` (Rule 1.A.a)
this source
   --> /users/syrup/zzz/paralegal/guide/mark_lib/jsonwebtoken/src/decoding.rs:208:1
    |
208 | fn verify_signature<'a>(
    | ^^^^^^^^^^^^^^^^^^^^^^^^
209 |     token: &'a str,
    |     ^^^^^^^^^^^^^^^
210 |     key: &DecodingKey,
    |     ^^^^^^^^^^^^^^^^^^
211 |     validation: &Validation,
    |     ^^^^^^^^^^^^^^^^^^^^^^^^
212 | ) -> Result<(Header, &'a str)> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: does go to
   --> /users/syrup/.rustup/toolchains/nightly-2024-12-15-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9
    |
143 |         $crate::io::_print($crate::format_args_nl!($($arg)*));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
```

or

```sh
error: Failed policy
...
note: `"sensitive" does not go to "output"` (Rule 1.A.a)
this source
  --> /users/syrup/zzz/paralegal/guide/mark_lib/jsonwebtoken/src/encoding.rs:24:9
   |
24 |         EncodingKey { family: AlgorithmFamily::Hmac, content: secret.to_vec() }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: does go to
   --> /users/syrup/.rustup/toolchains/nightly-2024-12-15-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9
    |
143 |         $crate::io::_print($crate::format_args_nl!($($arg)*));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
```

**Discussion:**

Sometimes it cannot provide a precise location, especially when the function is deeper. But overall it can detect cross-crate policy violation, which means it can be used to analyze libraries.

---

### exp-a

The source code (`guide/exp-a/src/main.rs`):

```rust
#[paralegal::marker(sensitive)]
struct Message { content: Vec<u8>, author: String }

#[paralegal::marker(sink, arguments = [0])]
fn publish(_operation: &Operation) { todo!() }

#[paralegal::analyze]
fn main() {
    let msg = Message { content: b"Hello, network!".to_vec(), author: "alice".to_string() };
    let body = Body::new(&msg.content); // Body::new marked process in mark_lib
    let mut header = Header { ..., payload_size: body.size(), payload_hash: Some(body.hash()), ... };
    header.sign(&private_key);
    let operation = Operation { hash: header.hash(), header, body: Some(body) };
    publish(&operation);
}
```

The `process` marker is placed directly on `Body::new` in `mark_lib/p2panda-core` (a local annotated copy of the library). The command:

```sh
bash run.sh a
```

The result:

```sh
Policy succeeded
```

**Discussion:**

Demonstrates the standard lib-analysis pattern for p2panda-core. `msg.content` (sensitive) flows directly into `Body::new` (process, in mark_lib) as argument 0, then through `Operation` to `publish` (sink). Mirrors exp-3's structure for jsonwebtoken.

### exp-b

This experiment tests that Paralegal can trace data flow **into library function bodies** and detect violations that are invisible from the calling code. A debug log bug is injected into `Body::to_bytes()` in `guide/mark_lib/p2panda-core/src/operation.rs`:

```rust
pub fn to_bytes(&self) -> Vec<u8> {
    // BUG: accidental debug log leaks raw body content to stdout
    println!("[DEBUG] Body::to_bytes: {:?}", self.0);
    self.0.clone()
}
```

The harness (`guide/exp-b/src/main.rs`) sends sensitive content through `Body::new` (process) and then calls a library helper. There is no harness-defined sink in this experiment:

```rust
use p2panda_core::Body;

#[paralegal::marker(sensitive)]
struct Message { content: Vec<u8> }

#[paralegal::analyze]
fn main() {
    let msg = Message { content: b"Hello, network!".to_vec() };
    let body = Body::new(&msg.content);  // process marker in mark_lib
    let _raw = body.to_bytes();          // call into the injected library bug
}
```

The command:

```sh
bash run.sh b
```

The result:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of exp_p2panda_b::main
note: `For each "output" marked sink` (Rule 1.A)
failed because of this element
   --> /home/xinzhouhe/.rustup/toolchains/nightly-2024-12-15-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9
    |
143 |         $crate::io::_print($crate::format_args_nl!($($arg)*));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
note: `"sensitive" does not go to "output"` (Rule 1.A.a)
this source
  --> src/main.rs:17:15
   |
17 |     let msg = Message {
   |               ^^^^^^^^^
18 |         content: b"Hello, network!".to_vec(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
19 |     };
   |     ^
   |
note: does go to
   --> /home/xinzhouhe/.rustup/toolchains/nightly-2024-12-15-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/macros.rs:143:9
    |
143 |         $crate::io::_print($crate::format_args_nl!($($arg)*));
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
```

**Discussion:**

The harness has no explicit sink. The violation is hidden inside `Body::to_bytes()` in the library. Paralegal traverses into the library function body and discovers that `body.0` (carrying data from the sensitive `msg.content`) flows through `to_bytes()` to `println!` (a sink via external annotations). The AnotB policy ("sensitive must never reach any sink") catches this. This demonstrates the core value of the `mark_lib` approach: annotate library internals to expose data flow violations that are invisible at the call site.

### exp-c

The source code (`guide/exp-c/src/main.rs`):

```rust
#[paralegal::marker(sensitive)]
struct Message { content: Vec<u8>, author: String }

#[paralegal::marker(process, arguments = [0])]
fn encode_content(content: &[u8]) -> Vec<u8> { content.to_vec() }

#[paralegal::marker(sink, arguments = [0])]
fn publish_bytes(_data: &[u8]) { todo!() }

#[paralegal::analyze]
fn main() {
    let msg = Message { content: b"Hello, network!".to_vec(), author: "alice".to_string() };
    let encoded = encode_content(&msg.content);
    publish_bytes(&encoded);
}
```

The command:

```sh
bash run.sh c
```

The result:

```sh
Policy succeeded
```

**Discussion:**

The `process` marker is on a harness-defined function rather than a library function. Analogous to partner's exp-4 goal but with a function that takes ownership of the data by value rather than mutable reference. Paralegal accepts harness-level markers as valid process checkpoints, confirming that library access is not required.

### exp-d

The source code (`guide/exp-d/src/main.rs`) has two `Message` instances: `msg1` takes the correct path through `Body::new` before `publish`, while `msg2` goes directly to `println!` without encoding.

The command:

```sh
bash run.sh d
```

The result:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of exp_p2panda_d::main
note: `Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process` (Rule 1)
source
  --> src/main.rs:54:5
   |
54 |     println!("{:?}", msg2.content);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: has data flow influence on this target without passing checkpoint
  --> src/main.rs:50:16
   |
50 |     let msg2 = Message {
   |                ^^^^^^^^^
51 |         content: b"Secret data".to_vec(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
52 |         author: "alice".to_string(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
53 |     };
   |     ^
```

**Discussion:**

Tests whether ACB's universal quantifier ("each sensitive") catches violations when one instance is clean and another is not. Unlike [exp-1](#exp-1) (which used an existential quantifier and cross-matched incorrectly), the ACB policy correctly fails because `msg2` reaches `println!` without going through `Body::new`. This shows that the cross-instance matching limitation from exp-1 is specific to existential quantifiers, not universal ones.

### exp-e

The source code (`guide/exp-e/src/main.rs`):

```rust
#[paralegal::marker(sensitive)]
struct Message { content: Vec<u8>, author: String }

#[paralegal::analyze]
fn main() {
    let msg = Message { content: b"Hello, network!".to_vec(), author: "alice".to_string() };
    // Accidental debug log: sensitive data reaches println.
    println!("Sending message from {}: {:?}", msg.author, msg.content);
}
```

Policy: `A does not go to B` ([policy_AnotB.txt](./guide/policy/policy_AnotB.txt))

The command:

```sh
bash run.sh e
```

The result:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of exp_p2panda_e::main
note: `For each "output" marked sink` (Rule 1.A)
failed because of this element
  --> src/main.rs:20:5
   |
20 |     println!("Sending message from {}: {:?}", msg.author, msg.content);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: `"sensitive" does not go to "output"` (Rule 1.A.a)
this source
  --> src/main.rs:14:15
   |
14 |     let msg = Message {
   |               ^^^^^^^^^
15 |         content: b"Hello, network!".to_vec(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
16 |         author: "alice".to_string(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
17 |     };
   |     ^
   |
note: does go to
  --> src/main.rs:20:5
   |
20 |     println!("Sending message from {}: {:?}", msg.author, msg.content);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Discussion:**

Uses the AnotB policy to catch accidental leakage of sensitive data to stdout via a debug log. No `publish` sink is defined in the harness; only the stdlib I/O functions are marked as sinks via `external-annotations.toml`. Paralegal detects that `msg.content` and `msg.author` reach `println!` and correctly fails the policy.

### exp-f

The source code (`guide/exp-f/src/main.rs`):

```rust
#[paralegal::marker(sensitive)]
struct Message { content: Vec<u8> }

#[paralegal::marker(source, return)]
fn make_message(content: &[u8]) -> Message {
    Message { content: content.to_vec() }
}

#[paralegal::marker(sink, arguments = [0])]
fn publish_raw(_content: &[u8]) { todo!() }

#[paralegal::analyze]
fn main() {
    let msg1 = make_message(b"processed message");
    let _body = Body::new(&msg1.content);

    let msg2 = make_message(b"unprocessed message");
    publish_raw(&msg2.content);
}
```

The policy ([policy_exists_process_sink.txt](./guide/policy/policy_exists_process_sink.txt)) intentionally uses two existential clauses:

```txt
Scope:
Everywhere

Policy:
1. For each "message" type marked sensitive:
    A. There is a "source" that produces "message" where:
        a. There is a "process" marked process where:
            i) "source" goes to "process"
    and
    B. There is a "source" that produces "message" where:
        a. There is a "output" marked sink where:
            i) "source" goes to "output"
```

The command:

```sh
bash run.sh f
```

The result:

```sh
Policy succeeded
```

**Discussion:**

This is a p2panda-flavored version of [exp-1](#exp-1). It should be treated as an incorrect pass. `msg1` goes through `Body::new` (the library process marker), while `msg2` reaches `publish_raw` without going through that process. The policy still succeeds because the existential process clause can be satisfied by `msg1`, and the existential output clause can be satisfied by `msg2`. This demonstrates the same cross-instance matching problem in a p2panda harness.

### exp-g

The source code (`guide/exp-g/src/main.rs`):

```rust
#[paralegal::marker(sensitive)]
struct Message { content: Vec<u8> }

#[paralegal::marker(process, arguments = [0])]
fn scrub_message(message: &mut Message) {
    message.content.clear();
}

#[paralegal::marker(sink, arguments = [0])]
fn publish_message(_message: &Message) { todo!() }

#[paralegal::analyze]
fn main() {
    let mut msg = Message { content: b"Hello, network!".to_vec() };
    scrub_message(&mut msg);
    publish_message(&msg);
}
```

The policy: `A goes to B via C` ([policy_ACB.txt](./guide/policy/policy_ACB.txt))

The command:

```sh
bash run.sh g
```

The result:

```sh
error: Failed policy
note: `Scope everywhere`
  failed because of exp_p2panda_g::main
note: `Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process` (Rule 1)
source
  --> src/main.rs:24:5
   |
24 |     publish_message(&msg);
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
note: has data flow influence on this target without passing checkpoint
  --> src/main.rs:19:19
   |
19 |     let mut msg = Message {
   |                   ^^^^^^^^^
20 |         content: b"Hello, network!".to_vec(),
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
21 |     };
   |     ^
```

**Discussion:**

This is a p2panda-side version of [exp-4](#exp-4). The data is passed through a `process` marker, but the process mutates the same `Message` instance instead of creating a new processed type. Paralegal still reports that the sensitive source reaches `publish_message` without passing the checkpoint, so this reproduces the same limitation with an in-place `&mut` process.

### exp-h

This experiment sets up `p2panda-discovery`, where raw `Topic`s are sensitive because discovery treats topics as private group identifiers. The `process` marker is placed on `hash_vector` in `guide/mark_lib/p2panda-discovery/src/psi_hash.rs`:

```rust
#[paralegal::marker(process, arguments = [0])]
pub fn hash_vector(topics: &[Topic], salt: &[u8; 65]) -> Result<Vec<Topic>, std::io::Error> {
    topics
        .iter()
        .map(|topic| hash(topic.as_bytes(), salt))
        .map(|result| match result {
            Ok(topic) => Ok(topic.into()),
            Err(err) => Err(err),
        })
        .collect()
}
```

The harness (`guide/exp-h/src/main.rs`) sends only salted topic hashes into a discovery protocol message:

```rust
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

    let salt = [1; 65];
    let topics_for_alice = HashSet::from_iter(hash_vector(&local.topics, &salt).unwrap());
    let message = PsiHashMessage::BobSaltHalfAndHashedData {
        bob_salt_half: [2; 32],
        topics_for_alice,
    };

    publish_discovery_message(&message);
}
```

The command:

```sh
bash run.sh h
```

The result:

```sh
warning: Cannot determine markers for type __H/#0

warning: Cannot determine markers for type dyn [Binder { value: Trait(std::error::Error), bound_vars: [] }, Binder { value: AutoTrait(DefId(2:36323 ~ core[83eb]::marker::Send)), bound_vars: [] }, Binder { value: AutoTrait(DefId(2:3446 ~ core[83eb]::marker::Sync)), bound_vars: [] }] + 'static

warning: Alias type Alias(Projection, AliasTy { args: [HarnessNodeInfo, u8], def_id: DefId(66:11 ~ p2panda_store[ca47]::address_book::traits::NodeInfo::Transports), .. }) remains. Was not normalized.

Policy succeeded
```

**Discussion:**

This mirrors [exp-a](#exp-a), but with the confidentiality story from `p2panda-discovery`: raw topics should not be sent directly; they should first pass through the library hashing step used by the PSI protocol. The real async `DiscoveryProtocol::bob` path currently triggers a Paralegal/rustc plugin panic because of generic async trait machinery, so this harness isolates the library data-flow shape without testing the full async protocol driver.

### exp-i

This experiment tests that Paralegal can trace data flow **into a library function body** and detect a violation invisible from the harness. A bug is injected into a new public function `validate_topics` in `guide/mark_lib/p2panda-discovery/src/psi_hash.rs`:

```rust
/// Pre-flight validation of topics before a PSI session.
pub fn validate_topics(topics: &[Topic]) -> usize {
    // BUG: debug log accidentally leaks raw topic values to stdout
    println!("[DEBUG] validate_topics: {:?}", topics);
    topics.len()
}
```

The harness (`guide/exp-i/src/main.rs`) looks correct — it calls `validate_topics` as a benign pre-flight count check:

```rust
#[paralegal::marker(sensitive)]
struct SecretTopics { topics: Vec<Topic> }

#[paralegal::analyze]
fn main() {
    let local = SecretTopics {
        topics: vec![[7; 32].into(), [11; 32].into()],
    };
    // Looks harmless: just a pre-flight count check before the PSI exchange.
    // But validate_topics in the library has a hidden println that leaks raw topic bytes.
    let _count = validate_topics(&local.topics);
}
```

The command:

```sh
bash run.sh i
```

The result:

```sh
error: Failed policy
note: `For each "output" marked sink` (Rule 1.A)
  --> .../std/src/macros.rs:143:9
note: `"sensitive" does not go to "output"` (Rule 1.A.a)
  --> src/main.rs:18:17
note: does go to
  --> .../std/src/macros.rs:143:9
```

**Discussion:**

The harness is clean — calling `validate_topics` looks entirely reasonable. The violation is hidden inside the library function: `topics` (sensitive, argument 0) flows to `println!` (`std::io::_print`, sink via external annotations) inside `validate_topics`. Paralegal traverses into the library body and catches it. This mirrors [exp-b](#exp-b) but on `p2panda-discovery`.

### exp-j

This experiment demonstrates a harness-level bypass: the developer should call `hash_vector` (the process step) before sending topics over the network, but accidentally skips it and sends raw topics directly to the sink.

The harness (`guide/exp-j/src/main.rs`):

```rust
#[paralegal::marker(sensitive)]
struct SecretTopics { topics: Vec<Topic> }

#[paralegal::marker(sink, arguments = [0])]
fn broadcast(_topics: &HashSet<Topic>) { todo!() }

#[paralegal::analyze]
fn main() {
    let local = SecretTopics {
        topics: vec![[7; 32].into(), [11; 32].into()],
    };
    // BUG: should call hash_vector (process) first, but developer skipped it.
    // Raw sensitive topic bytes flow directly to broadcast without any hashing.
    let raw_set: HashSet<Topic> = local.topics.iter().cloned().collect();
    broadcast(&raw_set);
}
```

The command:

```sh
bash run.sh j
```

The result:

```sh
error: Failed policy
note: `Each "sensitive" marked sensitive goes to a "output" marked sink only via a "process" marked process` (Rule 1)
  --> src/main.rs:30:5
note: has data flow influence on this target without passing checkpoint
  --> src/main.rs:23:17
```

**Discussion:**

ACB catches the violation: `local.topics` (sensitive) reaches `broadcast` (sink) without passing through any process-marked function. This is the direct complement to [exp-h](#exp-h) — the same scenario but with the hashing step missing. It mirrors [exp-j](#exp-j) for `p2panda-discovery` in the same way that [exp-b](#exp-b) relates to [exp-a](#exp-a) for `p2panda-core`.

## Conclusion

1. Paralegal still has some limitations, and it is hard to define an accurate policy.
2. If the source code of a library is accessible, Paralegal is able to analyze its internal information flow based on the marks and policies.
3. Paralegal cannot identify a leakage happens inside its dependencies.

## TODO

1. Use docker to build the environment.
2. Test more libraries.
