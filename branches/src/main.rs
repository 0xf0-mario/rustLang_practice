fn main() {
    let number: i32 = 123; //integer 32 signed immutable 

    if number == 123 {
        println!("condition true");
    } else {
        println!("condition false");
    }

    let mut counter: i32 = 0;

    let res = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("after loop {res}");

    counter = 0;
    'label_one: loop {
        println!("counter {counter}");
        let mut rem = 10;

        loop {
            println!("rem {rem}");
            if rem == 9 {
                break;
            }
            if counter == 2 {
                break 'label_one
            }
            rem -= 1;
        }
        counter += 1;
    }

    //iterating through a collection
    let mut a = [0; 10]; // 10 zeros
    let mut idx = 0;
    while idx < 10 {
        println!("iterating idx:{idx} {}", a[idx]);
        a[idx] = idx;
        idx += 1;
    }

    for elm in a {
        println!("iter {elm}")
    }

}
