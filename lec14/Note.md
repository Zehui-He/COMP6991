## Definition of Send and Sync traits
Send trait: If some type is safe to send from one to another. 

Sync trait: If some type is safe to send a **shared borrow** of the type. 

Both send and sync trait are implemented automatically.

## Move closure
The following code wouldn't build because we capture *my_string* by reference. But there is a chance that the thread may outlive the main function. Therefore, the closure may outlive the main function. In that case, the closure is holding an empty reference of *my_string* which can never happen in rust.
```
fn main() {
    let my_string = "hello";
    # This would not work, the closure capture thing by reference.
    std::thread::spawn(|| {
        # my_string 
        let str_ref = &my_string;
        println!("{str_ref}");
    }).join().unwrap();
}
```
Desugaring the closure:
```
struct Myclosure<'a> {
    str: &'a String,
}
```
Solution: add a *move* keywrod before the declaration of closure. ***This means we are capturing the enviroment varibale by taking the ownership into the closure, thus transferring ownership of those values from one thread to another.***

Desugaring the move closure:
```
struct Myclosure {
    str: String,
}
```


```
fn main() {
    let str = "hello";
    std::thread::spawn(move || {
        let str_ref = &str;
        println!("{str_ref}");
    }).join().unwrap();
}
```
