struct Demo {
    yes: i32,
}

fn main() {
    println!("Hello, world!");
    test(Demo { yes: 1 });
}

fn test<T>(t: T) {
    t.yes;
}
