# iff.rs

> A macro for if / if let chains until [RFC 2497] is implemented.

## Usage

```rust
use iff::iff;

fn go(var: Option<Vec<i32>>) {
    print!("{:?}", var);
    iff! {
        let Some(x) = var,
        let [y, _] = &*x,
        *y == 0 => {
            print!(" => ok")
        }
    }
    println!("");
}

fn main() {
    go(None);
    go(Some(vec![]));
    go(Some(vec![0]));
    go(Some(vec![0, 1]));
    go(Some(vec![0, 1, 2]));
}
```

Output:

```
None
Some([])
Some([0])
Some([0, 1]) => ok
Some([0, 1, 2])
```

[RFC 2497]: https://github.com/rust-lang/rfcs/blob/master/text/2497-if-let-chains.md
