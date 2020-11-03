# Understanding Rust Lifetimes

A super fast look into rust lifetimes.

Here we create a Iterable and a Mutable Iterable. But since `mutable iter` needs clarity for how long it lives, rust's lifetime system has to come into play to decode the lifetime of the elements returned by the `mut iterator`. 

I've made inline comments for me to refer to it later on.