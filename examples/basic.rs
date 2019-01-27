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
