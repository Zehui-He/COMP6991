fn main() {
    let my_string = "hello".to_string();

    std::thread::spawn(move || {
        // This step is trying to overtake the ownership of my_string
        let x = &my_string;
        println!("{x}");
        let y = my_string;
        println!("{y}");
    }).join().unwrap();

    let a = String::from("what");

    test_funonce(|| {
        let _a = &a;
    });

}

fn test_funonce(_f: impl FnMut()) {}