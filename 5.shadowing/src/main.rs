fn main() {
    // example #1
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is {x}");

    // example #2
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    /*
       However, if we try to use mut for this,
       as shown here, we’ll get a compile-time error:
    */

    // let mut spaces = "   ";
    // spaces = spaces.len();
}

/*
    Shadowing is different from marking a variable as mut because we’ll
    get a compile-time error if we accidentally try to reassign to this
    variable without using the let keyword. By using let, we can perform
    a few transformations on a value but have the variable be immutable
    after those transformations have been completed.
*/
