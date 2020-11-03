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
    }
}
