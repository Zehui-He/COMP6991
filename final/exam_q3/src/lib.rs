pub struct DBMap<A, B>
where A: Eq
{
    pub data: Vec<(A, B)>
}

impl<A:Eq, B> DBMap<A, B> {
    pub fn merge<C>(self, mut other: DBMap<A, C>) -> DBMap<A, (B, Option<C>)> {
        let mut res = DBMap{
            data: Vec::<(A, (B, Option<C>))>::new()
        };

        for (key1, val1) in self.data.iter() {
            for (key2, val2) in other.data.iter() {
                if key1 == key2 {
                    res.data.push(
                        (key1, (val1, Some(val2)))
                    );
                    continue;
                }
            }
            res.data.push(
                (key1, (val1, None))
            )
        }

        res
    }
}
