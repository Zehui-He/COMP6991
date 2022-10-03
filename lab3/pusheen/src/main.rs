fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let a = &mut vec;
    a.push(11);
    let b = &mut vec;
    b.push(12);
}

/*
The compiler would not compile because both a and b exclusively borrowed the vector. This means
both variable a and b can modify the vector at the same time. If a and b modify the same element
simutaneously, the value of that element can be any of the resultant values after a and b's 
modification. This is also knwon as write-write conflict.

This program would compile now because only one variable exclusively borrows the vector.
*/
