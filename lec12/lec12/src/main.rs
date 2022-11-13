// Generic function which takes an iterator and a function(closure)
fn my_map<I,F, Curritem, Newitem> (
    iter: I,
    function: F,
) -> MyMap<I, F> 
// Restrict the output type
where I: Iterator<Item = Curritem>, F: FnMut(Curritem) -> Newitem
{
    MyMap { iter, function }
}

struct MyMap<I, F> {
    iter: I,
    function: F
}

impl <I, F, Curritem, Newitem> Iterator for MyMap<I,F>
where
    I:Iterator<Item = Curritem>,
    F:FnMut(Curritem) -> Newitem
{
    type Item = Newitem;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_item = self.iter.next()?;
        let new_item = (self.function)(curr_item);
        Some(new_item)
    }
}

fn main() {
    let factor = 100;
    let mut list = vec![1, 2, 3, 4];
    list = my_map(list.into_iter(), |x| x * factor).collect();
    println!("{:?}",list);

}