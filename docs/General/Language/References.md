# References

## Reference Pointers

  * https://stackoverflow.com/questions/29672373/what-is-difference-between-mut-a-t-and-a-mut-t

```
    a: &T      // immutable binding of immutable reference
mut a: &T      // mutable binding of immutable reference
    a: &mut T  // immutable binding of mutable reference
mut a: &mut T  // mutable binding of mutable reference
```

In the first case the pointer is fixed / cannot be changed later on
and the value it points to can also not be changed / is readonly

In the second case the pointer can be changed to point at different objects of the same type
But the value it points to can not be changed / is readonly

In the third case the pointer is fixed / read only
But the value can be altered changed

In the last case the pointer can point to differnt objects
and the value it points to can also be changed.


## Ownership

  * https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

Rust has a concept of ownership, in that when something that isn't a reference is created.
Then the owner of the object is tracked 

```
fn main() -> ! {

    // Get Peripherals
    let mut dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    // This moves the onwership of dp into the test function but doesn't return it
    test(dp);
    // If we do something with dp here the code fails
}

fn test(param: target::Peripherals) {
    
}
```

Lets say we did this instead
```
fn main() -> ! {

    // Get Peripherals
    let mut dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    // This moves the onwership of dp into the test function but doesn't return it
    dp = test(dp);
    // Using dp here is fine
}

fn test(param: target::Peripherals) -> target::Peripherals {
    return param;
}
```

This compiles fine since we're moving the ownership into the test function then returning the ownership / moving it back.
Typically we don't want to do this usually unless we want to pass responsibility for cleanup to another function
So instead we pass in references instead
```
fn main() -> ! {

    // Get Peripherals
    let mut dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    // This keeps the ownership of dp within the main function and doesn't move it
    test(&mut dp);
    // Using dp here is fine
}

fn test(param: &mut target::Peripherals) {

}
```

The type prefix &mut tends to be fairly common and means this is a reference to a thing that you can change / alter if you want to.
In the above target::Peripherals::take() is returning a "&mut target::Peripherals" type so all is good.
