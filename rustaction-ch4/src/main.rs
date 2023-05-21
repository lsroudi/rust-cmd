

#[derive(Debug)]
enum statusMessage {
    Ok,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

fn checkStatus(sat_id:CubeSat) ->statusMessage {
    statusMessage::Ok
}

fn main() {
    
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let a_status = checkStatus(sat_a);
    let b_status = checkStatus(sat_b);
    let c_status = checkStatus(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    let a_status = checkStatus(sat_a);
    let b_status = checkStatus(sat_b);
    let c_status = checkStatus(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
