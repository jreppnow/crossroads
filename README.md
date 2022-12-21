# ```crossroads``` library

This library consists of a single proc macro with a simple purpose: Turn one function into potentially many!

To motivate this, assume you have a couple of test cases like these:

```rust
use std::collections::HashMap;

#[test]
fn empty_be_default() {
    let map: HashMap<String, usize> = Default::default();
    
    assert!(map.is_empty());
}

#[test]
fn empty_after_clear() {
    let mut map: HashMap<String, usize> = Default::default();

    map.insert("test".to_string(), 1);
    map.clear();

    assert!(map.is_empty());
}

#[test]
fn empty_after_remove() {
    let mut map: HashMap<String, usize> = Default::default();

    map.insert("test".to_string(), 1);
    map.remove("test");

    assert!(map.is_empty());
}
```

This crate allows you to write the following instead:

```rust
use std::collections::HashMap;
use crossroads::crossroads;

#[crossroads]
#[test]
fn empty() {
    let mut map: HashMap<String, usize> = Default::default();

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

I welcome any suggestions/feature requests/changes/bug fixes. Feel free to open a PR if you already have concrete changes at hand. 
Alternatively, open an issue and we can discuss possible solutions/viability. 

Please note that any contributions that you make to this project are assumed to be licensed under the same license(s) as the remainder of project. 
Make sure that you are legally allowed to grant such licenses to your work.

General requirements:
- CI checks must all pass on your PR.
- After review/approval, please rebase to the latest version of the ```master``` branch.

## License

The library is licensed under MIT license, see ```LICENSE``` for details.
