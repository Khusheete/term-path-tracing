/*
Copyright 2024 Souchet Ferdinand

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated
documentation files (the “Software”), to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit
persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the
Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/


use std::collections::{hash_map::{Iter, Keys, Values}, HashMap};


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Rid {
    id: u64
}


impl Rid {
    fn create(id: u64) -> Self {
        Self {
            id: id
        }
    }
}


pub struct RidOwner<T> {
    data: HashMap<Rid, T>,
    current_id: u64
}


impl<T> RidOwner<T> {

    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            current_id: 0
        }
    }

    pub fn add(&mut self, obj: T) -> Rid {
        let rid = self.next_rid();
        self.data.insert(rid, obj);
        rid
    }


    pub fn get<'a>(&'a self, rid: Rid) -> Option<&'a T> {
        self.data.get(&rid)
    }


    pub fn modify<F>(&mut self, rid: Rid, f: F)
        where F: FnOnce(&mut T)
    {
        self.data.entry(rid).and_modify(f);
    }


    pub fn remove(&mut self, rid: Rid) {
        self.data.remove(&rid);
    }


    pub fn value_iter<'a>(&'a self) -> ValueIterator<'a, T> {
        ValueIterator { base: self.data.values() }
    }


    pub fn rid_iter<'a>(&'a self) -> RidIterator<'a, T> {
        RidIterator { base: self.data.keys() }
    }


    pub fn rid_value_iter<'a>(&'a self) -> RidValueIterator<'a, T> {
        RidValueIterator { base: self.data.iter() }
    }


    fn next_rid(&mut self) -> Rid {
        let rid = Rid::create(self.current_id);
        self.current_id += 1;
        rid
    }
}


pub struct ValueIterator<'a, T> {
    base: Values<'a, Rid, T>
}


impl<'a, T> Iterator for ValueIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}


pub struct RidIterator<'a, T> {
    base: Keys<'a, Rid, T>
}


impl<'a, T> Iterator for RidIterator<'a, T> {
    type Item = &'a Rid;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}


pub struct RidValueIterator<'a, T> {
    base: Iter<'a, Rid, T>
}


impl<'a, T: 'a> Iterator for RidValueIterator<'a, T> {
    type Item = (&'a Rid, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}