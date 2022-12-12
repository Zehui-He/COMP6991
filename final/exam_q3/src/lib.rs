pub struct DBMap<Key, Value> 
where Key: Eq
{
    pub data: Vec<(Key, Value)>
}

impl<Key, Value1> DBMap<Key, Value1> 
where Key: Eq
{
    pub fn merge<Value2>(self, mut other: DBMap<Key, Value2>) -> DBMap<Key, (Value1, Option<Value2>)> {
        let mut data = Vec::<(Key, (Value1, Option<Value2>))>::new();
        
        for (key1, val1) in self.data {
            match other.data.iter().position(|(key2, _val2)| *key2 == key1) {
                Some(idx) => {
                    // Take the ownership from the element before pushing into the new vector
                    data.push((key1, (val1, Some(other.data.remove(idx).1))));
                },
                None => {
                    data.push((key1, (val1, None)));
                },
            }
        }

        DBMap { data }
    }
}
