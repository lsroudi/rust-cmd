#[derive(Debug)]
enum Event {
    BEGIN,
    DELETE,
    UNKNOWN
}

fn parse_log(text:&str)-> (Event,String){

    let v: Vec<&str> = text.splitn(2,' ').collect();
                let event = v[0];
                let rest =  v[1];
                println!("event is {:?}",event);
                match event {
                         "BEGIN" | "update" => (Event::BEGIN, String::from(rest)),   
                         "DELETE" | "delete" => (Event::DELETE, String::from(rest)),   
                         _ => (Event::UNKNOWN, String::from("file is empty")),
                       }
}


fn main() {
       let log = "BEGIN Transaction XK342
     UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
     DELETE 342:LO/22111";
     
     for line in log.lines() {
             let parse_result = parse_log(line);
             println!("{:?}", parse_result);
           }
}
