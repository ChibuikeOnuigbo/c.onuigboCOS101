fn main(){
    let v = vec![10, 20, 30];
    let v2 = v;
    display(v2);
}

fn display(v: Vec<i32>){
    println!("inside display {:?}", v);
}
