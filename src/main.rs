
fn main(){
    // define with value
    let ar = [10, 20, 30, 40, 50];
    // ar.push(100);  it's wrong in array here  fix array element you not push element
    println!("{:?}", ar);
    // define with all value 0  element 5
    let ar2 = [0; 5];
    for element in ar2 {
        println!("{}", element);
    }

    // define with type 
    let mut ar3: [i32; 3] = [100, 200, 300];
    ar3[2] = 55;
    println!("{:?}", ar3);
    println!("array length: {}", ar3.len());
    
}