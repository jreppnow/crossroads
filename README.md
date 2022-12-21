# ```crossroads``` library

This library consists of a single proc macro with a simple purpose: Turn one function into potentially many!

To motivate this, assume you have a couple of test cases like these:

```rust
use std::collections::HashMap;

#[test]
fn empty() {
    let mut map = Map::new();

    assert!(map.empty());
}

#[test]
fn empty_after_add_remove() {
    let mut map = HashMap::new();

    map.insert("hello", 1);
    map.remove("hello");

    assert!(map.empty());
}

#[test]
fn empty_after_clear() {
    let mut map = HashMap::new();

    map.insert("hello", 1);
    map.clear();

    assert!(map.empty());
}
```

This crate allows you to write the following instead:

```rust
#[crossroads::crossroads]
#[test]
fn empty() {
    use std::collections::HashMap;

    let mut map = HashMap::<String, usize>::default();

    match fork!() {
        by_default => {}
        after_add => {
            map.insert("Key".to_owned(), 1337);
            match fork!() {
                and_remove => map.remove("Key"),
                and_clear => map.clear(),
            };
        }
    }

    assert!(map.is_empty());
}
```

The macro computes every possible path through the ```fork!()```s and creates a version of the function for it.
These can then be picked up by the test system (as attributes below the ```#[crossroads]``` attribute are cloned as
well). In this case, you will get output like this (run ```cargo test --examples``` to reproduce: 

```
running 3 tests
test empty_by_default ... ok
test empty_after_add_and_clear ... ok
test empty_after_add_and_remove ... ok
```

The idea is inspired
by [```@Nested``` tests in JUnit 5](https://junit.org/junit5/docs/5.4.1/api/org/junit/jupiter/api/Nested.html)
, [sections in Catch2](https://github.com/catchorg/Catch2/blob/devel/docs/tutorial.md#test-cases-and-sections)
and [subtests in doctest](https://github.com/doctest/doctest/blob/master/doc/markdown/tutorial.md#test-cases-and-subcases)
.

## Questions

Answers to common questions:

> 1. Why did you decide to use the standard ```match``` syntax instead of introducing a custom one, for example ```fork! { a => { .. }, b => { .. }}```?

It would indeed be possible to offer a completely custom syntax with the proc macro. However, I decided to just latch 
onto the ```match``` structure as it makes code-formatting tools, in particular ```rustfmt``` work a lot more smoothly.

## Contributions

For now, this library is still in development and there are no clear, small changesets available for people to get
involved. I'll do my best to get it into a workable state and will then gladly accept contributions.

## License

The library is licensed under MPL-2.0 (for now, might go less restrictive in the future), see ```LICENSE``` for details.
