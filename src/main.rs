fn main() {
    println!("Hello, world!");

    let a = 5;

    let b = &a;
    //let c ;
    // if a == b{
    //     c = *b + 1; 
    // }
    let c = *b + 1;

    println!( " a vlue is {:?}, b value {:?}, c {:?}" ,a, b,c);
}
