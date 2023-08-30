# Language

## Modules

At the top level you have crates which represent libraries
Next underneath that are mod's which represent namespaces

```
pub mod outer_mod {
    pub mod inner_mod {
    }
}
```

## Project references

  * https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/namespacing_with_modules/

To reference another .rs file
```
use reffile

fn main() {
  reffile::run();
]
```

When you write use mymodule;, you don’t declare (“include”) it,
you just tell compiler to move some names from the module into your current namespace,
so you don’t ::have::to::write::full::path::to::your::names::in::some::module::just::to::call::some::function().
You don’t have to use thingies from other modules with use, you can ::always::use::full::paths, but it’s just annoying.


Absolute reference from the crate route
```
::foo::bar
```

Inner reference
```
mod outer {
    mod inner {
        pub fn foo() { }
    }
    
    pub fn bar() {
        inner::foo();
    }
}
```



  * https://stackoverflow.com/questions/20922091/how-do-you-use-parent-module-imports-in-rust


## Variables

By default variables are immutable, in that they default to the const type
```
# This can't be changed later
let val = 2;
```

To allow a variable to be changed we need the mut keyword
```
let mut val = 2;
val = val + 1;
```

for strings
```
# Fixed immutable string
let x = "Hello"
# For mutable / changeable strings
let mut x = String::from("Hello");
# Add to the end of the string
x.push('x');
x.push_str(" more characters");

# Print each word split based on whitespace
for word in x.split_whitespace() {
  println!("{}", word);
}
```

for arrays by default they are fixed once defined in terms of size
for a dynamic sized array we use Vectors instead


## Testing

There are assert functions built in for stuff like
```
assert_eq!(3, x.len());
```

## Structs

Normally a variable or function within a struct is accessed using the dot notation, such as struct1.runnow();
For static methods within a module for example :: is used between module names

Accessors:

  * https://doc.rust-lang.org/reference/visibility-and-privacy.html


  * By default everything is private
  * The pub keyword is used to declare public
  * If the function and module it's in are both private, then the function can only be accessed by other members within the module (private)
  * If the function is public but the module is private then the function can be accessed only from within the same crate / library (similar to friend)
  * if the function is public and the module is public then it can be accessed from outside the crate / library (public)
