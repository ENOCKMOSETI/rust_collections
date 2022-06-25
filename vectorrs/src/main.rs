fn main() {
    // let v2 = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    
    let third: &i32 = &v[2];
    print!("The third element is: {}\n", third);
    
    match v.get(2) {
        Some(third) => print!("The third element is: {}\n", third),
        None => print!("There is no third element")
    }
    
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
    
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue"))
    ];
    let first: &SpreadSheetCell = &row[0];
    print!("The first element is: {:?}\n", first);

    match row.get(0) {
        Some(first)=> print!("The first element is {:?}\n", first),
        None => print!("There is no first element")
    }
}
