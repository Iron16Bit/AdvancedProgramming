/*
Signature of a function. It contains:
    - The visibility of the function
    - The name of the function
    - The type of the function: I -> O
 */
pub fn var_ass_mut(){

    //We define x as a 32 bit integer with value 10
    let x : i32 = 10;

    //Called shadowing, we change the type of a variable
    let x : &str = "a string?";

    let y : i32 = 11;
    y = y+1; // This can't be done, because y hasn't been defined as mutable, so it's static
    let mut y : i32 = y+1; // Now it's mutable, we won't need to use mut again if we'll change its value another time

    const _TRUE :i32 = 1; //A constant value

}

// -> *type* indicates the output type of the function
pub fn test() -> i32 {
    4; //This is actually wrong, as "4;" is a command and not an integer
}