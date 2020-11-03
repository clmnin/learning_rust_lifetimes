// MyIterWrapper is generic over lifetime a and type T
// MyIterWrapper lives on as long as the slice lives for.
struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

// the first <'a, T> is to tell we have two generic types - 'a and T
// We could have implemented an implementation with a static lifetime and i32 
// type like this
// impl Ierator for MyIterWrapper<'static, i32>{...}
// And these generics are pased on to MyIterWrapper
impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    // the element that we yield out of iter for iteration lives as long as 
    // MyIterWrapper does (which in turn lives for as long as the)
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // next fn yields out the next item
        
        if self.slice.is_empty() {
            return  None;
        }
        // get the first element
        let element = self.slice.get(0);
        // set self.slice equal to the other element
        self.slice = &self.slice[1..];
        // return first element
        element
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let collection: Vec<i32> = vec![1,2,3,4];
        let wrapper = MyIterWrapper {
            slice: &collection[..],
        };

        for (index ,elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }
    }
}
