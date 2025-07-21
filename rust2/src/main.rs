fn  main(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.filter(|x| *x % 2 == 1);
    let v1_iter3 = v1_iter2.map(|x| x*2); 

    let v2: Vec<i32> = v1_iter3.collect();  

    println!("{:?}", v2)
}  