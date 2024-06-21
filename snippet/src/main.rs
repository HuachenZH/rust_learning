fn main() {
    let mut x = 5;
    x = x +1;
    println!("The value of x is {x}");

    // ------------------------
    let x = 5;
    let x = x+1;
    println!("The value of x is {x}");

    // ------------------------
    // can i get a variable from inner scope?
    //{
    //    let _x = 5;
    //}
    //println!("The value of _x is {_x}");
    // no i can't'

    // data type: tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup1 = tup.0;
    let tup2 = tup.1;
    let tup3 = tup.2;
    println!("tup1 is {tup1}");
    println!("tup2 is {tup2}");
    println!("tup3 is {tup3}");

    // data type: array
    let mut arrayy = [3;5];
    arrayy[0] = 6;
    let first = arrayy[0];
    println!("first element of arrayy is {first}");

    // call a function
    let x = 5;
    another_fct(x);

    // expression in a scope
    let out = {
        let inside :i8 = 5;
        inside + 2
    };
    println!("out is {out}");


    let mut num:i32 = 1;
    println!("num before plus_one is {num}");
    num = plus_one(num);
    println!("num after plus_one is {num}");
    
    // getting values from parent scope
    let a = 5;
    println!("value of a is {a}");
    {
        let b = a;
        println!("value of b is {b}");
        {
            let c = b;
            println!("value of c is {c}");
        }
    }

    let num = 5;
    println!("{num}");

    // :? is the debug trait
    let sl1 = "string literal 1";
    let sl2 = sl1;
    println!("{sl2:?}");

    // s is gone after function call
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valide here.

    // s2 is moved
    let s2 = String::from("hello from s2");
    let s3 = takes_and_gives_back(s2);
    println!("s3 is {s3}");
    // here s2 is no longer valid

    let tup = ("elem0", "elem1");
    let (x,y) = tup;
    println!("{x}");
    println!("{y}");

    // referencing stuffs
    let s = String::from("hello");
    let len = cal_len(&s);
    println!("The length of \"{s}\" is {len}");
}



fn plus_one(num:i32) -> i32 {
    return num +1;
}


fn another_fct(num: i32) {
    println!("another fct");
    println!("{}",num);
}

fn takes_ownership(some_string:String) {
    println!("{some_string}");
}

fn takes_and_gives_back(some_string:String) -> String {
    some_string
}

fn cal_len(s: &String) -> usize {
    s.len()
}
