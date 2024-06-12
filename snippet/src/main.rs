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

}



fn plus_one(num:i32) -> i32 {
    return num +1;
}


fn another_fct(num: i32) {
    println!("another fct");
    println!("{}",num);
}
