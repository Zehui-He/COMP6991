fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let a = &mut vec;
    let b = &mut vec;

    a.push(11);
    b.push(12);
}

/*
The compiler would not 
*/
