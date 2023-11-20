# Recap

We covered a lot of ground in our first few hours together.

Let's take a look at an impenetrable encryption cipher: ROT13.

Steps:

- Watch me implement it
- Discuss improvements as a group
- Extend the cipher to ROT13+5 as an exercise

<details>

```rust,editable
fn rot13(s: &str) -> String {
    let mut secret = String::with_capacity(s.len());

    for c in s.chars() {
        let d = match c {
            'a'..='m' => ((c as u8) + 13) as char,
            'n'..='z' => ((c as u8 - b'a' + 13) % 26 + b'a') as char,
            'A'..='M' => ((c as u8) + 13) as char,
            'N'..='Z' => ((c as u8 - b'A' + 13) % 26 + b'A') as char,
            '0'..='9' => ((c as u8 - b'0' + 5) % 10 + b'9') as char,
            c => c,
        };
        secret.push(d);
    }

    secret
}

fn main() {
    let text = "Hello, World!";
    let encrypted = rot13(&text);
    let decrypted = rot13(&encrypted);

    println!("Original: {}", text);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}
```

</details>