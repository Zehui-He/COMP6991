pub fn tuple_unzip<T, A, B> (items: Vec<T>) -> (Vec<A>, Vec<B>) 
where T: Into<(A, B)>
{
    let mut vec_a = Vec::<A>::new();
    let mut vec_b = Vec::<B>::new();

    for item in items {
        let (a, b): (A, B) = item.into();
        vec_a.push(a);
        vec_b.push(b);
    }

    (vec_a, vec_b)
}

pub fn tuple_zip<T, A, B>(items: (Vec<A>, Vec<B>)) -> Vec<T> 
where (A, B): Into<T>
{
    // Reverse the sequence of the vector due to pop starts from the end
    let mut vec_a = items.0;
    vec_a.reverse();
    let mut vec_b = items.1;
    vec_b.reverse();

    let mut res = Vec::<T>::new();
    for _ in 0..vec_a.len() {
        let a = vec_a.pop().unwrap();
        let b = vec_b.pop().unwrap();
        let item = (a, b).into();
        res.push(item);
    }
    
    res
}
