// MyIterWrapper is generic over lifetime a and type T
// MyIterWrapper lives on as long as the slice lives for.
struct MyIterator<'a, T> {
    slice: &'a [T],
}

// the first <'a, T> is to tell we have two generic types - 'a and T
// We could have implemented an implementation with a static lifetime and i32 
// type like this
// impl Ierator for MyIterWrapper<'static, i32>{...}
// And these generics are pased on to MyIterWrapper
impl<'a, T> Iterator for MyIterator<'a, T> {
    // the element that we yield out of iter for iteration lives as long as 
    // MyIterWrapper does (which in turn lives for as long as the)
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // next fn yields out the next item
        
        // using split_first to get reference of the first element, 
        // if empty it returns None (thus saved a few lines of code)
        let (element, rest) = self.slice.split_first()?;
        self.slice = rest;
        // return first element
        Some(element)
    }
}

// here we make a mutable iterator
// Note: mut (mutable dunder) gives an exclusive mut. No one else should have
//       a reference to this data when one already has a mut reference (& mut)
struct MyMutableIterator<'iter, T> {
    slice: &'iter mut [T],
}

impl <'iter, T> Iterator for MyMutableIterator<'iter, T> {
    type Item = &'iter mut T;

    // allreferences have lifetime, so we have to define them explicitly
    // we're borrowning self for how ever long fn next lasts for.
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice1 = &mut self.slice;
        let slice2 = std::mem::replace(slice1, &mut []);
        let (first, rest) = slice2.split_first_mut()?;
        self.slice = rest;
        Some(first)

        // this is how our first implementation was. We were getting a mut
        // get_mut of the slice's element at 0. But element being inside fn next
        // with a lifetime 'next will have a lifetime 'next
        // and so we're getting a mut from slice of lifetime 'iter to var 
        // element with lifetime 'next. And 'next next is a lifetime much lower
        // than 'iter and so we are returning element with lifetime 'next.
        // 
        // And so we were taking in a colletion with lifetime 'a, then passing a
        // mut reference of this collection to a struct (MyMutableIter) that
        // takes the same lifetime as the collection (we acheived this by using 
        // generic 'iter lifetime). but then for each yield we're retuning a
        // different lifetime 'next
        // So this was the reason we were having an error for the below code
        // // get the first element
        // let element = self.slice.get_mut(0);
        // // set self.slice to rest of the list
        // self.slice = &mut self.slice[1..];
        // // return first element
        // element
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let collection: Vec<i32> = vec![1,2,3,4];
        let wrapper = MyIterator {
            slice: &collection[..],
        };

        for (index ,elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }

        let mut collection: Vec<i32> = vec![1,2,3,4];
        let wrapper = MyMutableIterator {
            slice: &mut collection[..],
        };

        for (_index ,elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }

        assert_eq!(collection.get(0), Some(&2));
    }
}
