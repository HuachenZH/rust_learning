# learning rust

## Prepare environment

### Prepare environment - install rust
Check the official doc: https://www.rust-lang.org/learn/get-started  

You need to get familiar with a few terms:
- rustc: the compiler
- rustup: the updater
  - responsible for instlling the rst compiler and cargo, and updating them to newer versions
- cargo: the package manager
  - downloads packages, resolves the versions and dependency graphs, invokes the compiler on them in correct order.

In cargo, a package is called a "crate". Consider it as libraries in python.  
There is also something called "rustfmt", formatter of rust code.

### Prepare environment - initialize project & cargo stuffs
To start a new project, cd to the working directory, then:  
```shell
$ cargo new <your_project_name>
```
Some files will be generated automatically. It seems that a local git will also be initialized.  

To add dependecy (use a package):
```shell
$ cargo add <dependency_name>
```
You can also manually add the dependency in `Cargo.toml`  


Then you need to compile (and download, if not so) the current package
```shell
$ cargo build
```
(an executable file will be created at `target/debug/`)

Or you can build and run the executable within one command:
```shell
$ cargo run
```
You can also compile the code then run the executable:
```shell
$ rustc main.rs
$ ./main
```

If you just want to compile without creating an executable:
```shell
$ cargo check
```

To update a crate the get a new version:
```shell
$ cargo update
```
Or manually modify Cargo.toml, but never Cargo.lock


## Very basic
- associated function
  ```rust
  let mut guess = String::new();
  ```
  The `::` indicates that ::new() is an associated function.  
  ___An associated function is a function that's implemented on a type___, in this case, `String`.

- __mutability and shadowing__  
  Variables are immutable by default.  
  If you want to create a variable whose value will be changed later, you can do it in two ways:
  - use `mut` keyword
    ```rust
    let mut x = 5;
    x = x + 1;
    ```
    When a variable is mutable, you can only change its value but not type.  
  - use shadowing
    ```rust
    let x = 5;
    let x = x+1;
    ```
  Notice that mutable and shadowing are different concepts.

- __scope__
  You cannot get a variable from an inner scope.
  ```rust
  {
    let _x = 5;
  }
  println!("value of _x is {_x}")
  ```
  Complier will give error.

- __data type: char != string literal__  
  They are different.  
  ```rust
  let c: char = 'Z'; // this is a char
  let s = "string"; // this is a string literal
  ```
  Declare string literal with double quotes.  
  Declare char with single quote.

- __data type: tuple__  
  To create a tuple:  
  ```rust
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  // or 
  // let tup = (500, 6.4, 1)
  ```
  Elements of a tuple can have different types.  
  You cannot just put a tuple in a println macro.  
  To get elements in a tuple, you have two ways to do so:  
  1. destructuring
    ```rust
    // destructure a tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("the value of y is {y}");
    ```
  2. use period (.)
    ```rust
    let tup = (500, 6.4, 1);
    let five_hundred = tup.0;
    ```
    The first index in a tuple is 0.

- __data type: array__  
  Unlike tuple, elements of an array must have the same type.   
  ```rust
  // create an array of [3,3,3,3,3]
  let arrayy = [3;5];
  
  // access an element
  let first_element = arrayy[0];
  ```
  Arrays have a fix length, they cannot grow or shrink.  
  However they still can be mutable if you specify it:  
  ```rust
  let mut arrayy = [3;5]; // [3,3,3,3,3]
  arrayy[0] = 6;
  let first_element = arrayy[0];
  println!(first_element) // output : 6
  ```

- __expression vs statement__  
  ```rust
  let x = 1; 
  // This is a statement.  
  // All those who have "let" keyword are statement.
  // All those who have ";" at the end are statement. (unless with return or break keyword)
  // Statement does not return a value.

  x + 1
  x
  // These are expressions.
  // Expression returns a value.
  ```

- __function__  
  You must specify a function's parameter's type and return type.  
  Once i had:  
  ```rust
  fn main() {
    let mut num = 1;
    num = plus_one(num);
  }

  fn plus_one(num: i32) -> i32 { // you must specify the return type
    num + 1
    // or return num+1
    // or return num+1;
  }
  ```
  


## Ownership
- stack vs heap  
  See this [stack overflow poste](https://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap)  
  | stack  |  heap  |
  | -----  |  ----  |
  | easy to keep track  |  hard to keep track  |
  | data stored on stack must have fixed and known size  |  data with an unknown size (or size might change) at compile time can only be stored on heap   |
  | well organized: LIFO (last in first out)  |  ----  |
  | _"pushing values onto the stack"_ != _"allocating"_.  Pointers have known size so can be pushed onto stack  |  _"allocating on the heap"_ or just _"allocating"_: when you need space, the memory allocater finds an empty spot in the heap, marks it as being in use, and return a pointer (=address of that location)  |
  | faster  |  slower (because the allocater has to find a place to store new data)  |

- __ownership rules__:  
  - ___Each value in Rust has an owner.___
  - ___There can only be one owner at a time.___
  - ___When the owner goes out of scope, the value will be dropped.___ (the memory is freed when the variable goes out of scope)

Rust does not have garbage collection. For languages who do not have garbage collection, normally you should do two things: allocate memory when you declare a variable; free the memory when the variable is no longer useful.  

In Rust, you don't need to free memory manually, this can be done by scope:  
```rust
{
  let s = String::from("s is a String");
  // s is still valid here
}
// scope ends, s is no longer valid, the memory is freed.
```
More precisely, when there is a closing curly brace, rust automatically calls the `drop` function and cleans up the heap memory.

- ___"move"___:  
  ```rust
  let s1 = String::from("hello");
  let s2 = s1;
  // s1 was moved to s2
  // only stack data is copied, heap data is not copied.
  // the following code will give error:
  println!("{s1}");
  // because as the value of s1 was moved to s2, rust consider s1 no longer valid.
  ```
- __`copy`__:  
  This happens with heap but not stack, the code below does not give error:  
  ```rust
  let sl1 = "hello"; // sl1 is a string literal, not a String. It stores on stack but not heap
  let sl2 = sl1;
  println!("{sl1}"); // it's alright, no error.
  ```
  This is the `copy` trait. Scalar types implement copy trait.

Some examples to help understand ownership:  
```rust
fn main{
  let s1 = String::from("hello");
  takes_ownership(s1);
  // here s1 is no longer valid.

  let s2 = String::from("hello2");
  let s3 = takes_and_gives_back(s2);
  // here s2 is no longer valid, its value was moved to s3
}

fn takes_ownership(some_string:String) {
  println!("The ownership of {some_string} is taken");
}

fn takes_and_gives_back(some_string:String) -> String {
  some_string
}
```


## miscellaneous
- macro  
  ```rust
  fn main(){
    println!("hello world");
  }
  ```
  The `!` means that it's a macro rather than funciton. If you delete the "!", the compiler will give error

- println!  
  ```rust
  let num = 5;
  println!(num); // this will give error
  println!("{}", num); // fine
  println!("{num}"); // fine
  ```

- unwrap() and expect()  
  [explanation](https://www.programiz.com/rust/unwrap-and-expect)  
  - unwrap: if everything's ok, do nothing; if unwrap encounters an error `Err`, panic and stop program execution.
  - expect: just like unwrap, and you can customize error message.

- `string literal` != `String`  
  | string literal  |  String |
  | --------------  |  ------ |
  | immutable  |  can be mutated |
  | let sl = "this is a string literal";  |  let s = String::from("s is a String"); |