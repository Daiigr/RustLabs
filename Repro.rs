fn main(){
    let mut input = String::new();
    let b1 = std::io::stdin().read_line(&mut input).unwrap();
    let   numberOfSheets:i32 = input.trim().parse().unwrap();
    let  mut numberOfWearhouses:i32 = 0;
    let  mut remainderNumberofWearehouses:i32 = 0;

    numberOfWearhouses = numberOfSheets / 500;
    remainderNumberofWearehouses  = numberOfSheets % 500;

    if remainderNumberofWearehouses > 0 {
        numberOfWearhouses+=1;
    }

    if numberOfWearhouses <= 0 {
        numberOfWearhouses = 0;
    }

    println!("{}\n", numberOfWearhouses);

 }