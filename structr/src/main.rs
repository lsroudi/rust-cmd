use rand::prelude::*;                      

fn one_in(denominator: u32) -> bool {      
  thread_rng().gen_ratio(1, denominator)   
}

#[derive(Debug)]                     
struct File {
   name: String,
   data: Vec<u8>,                     
 }
 
 fn open(f: &mut File) -> bool{
    true
 }

 fn close(f: &mut File) -> bool{
    true
 }
 fn open_2(f: File) -> Result<File,String>{
    if one_in(10_000){
        let error = String::from("Permission denied");
        return Err(error);
    }
    Ok(f)
 }

 fn close_2(f: File) -> Result<File,String>{
    if one_in(10_000){
        let error = String::from("Interrupted by signal");
        return Err(error);
    }
    Ok(f)
 }
 impl File {
    fn new(name: &str)-> File{
        File{name : String::from(name), data: vec![]}
    } 
    fn new_with_data(
        name: &str,
        data: &Vec<u8>,
          ) -> File {                                 
        let mut f = File::new(name);
        f.data = data.clone();
        f
      }
      fn read_2(
        self: &File,
        save_to: &mut Vec<u8>
        )-> Result<usize,String>{
            let mut tmp = self.data.clone();
            let read_len = tmp.len();
    
            save_to.reserve(read_len);
            save_to.append(&mut tmp);
    
            Ok(read_len)
     }
 }


 fn read(
    f: &mut File,
    save_to: &mut Vec<u8>
    )-> usize{
        let mut tmp = f.data.clone();
        let read_len = tmp.len();

        save_to.reserve(read_len);
        save_to.append(&mut tmp);

        read_len
 }
 


 fn main() {
   let mut f2 = File {
     name: String::from("f1.txt"),    
     data: vec![114, 117, 115, 116, 33],         
   };

   let mut buffer: Vec<u8> = vec![];
   open(&mut f2);
   let f2_length = read(&mut f2, &mut buffer);
   close(&mut f2);   
   
   let text = String::from_utf8_lossy(&buffer);
 
   println!("{:?}", f2);
   println!("{} is {} bytes long", &f2.name, f2_length);
   println!("{}", text);

   let f3_data = vec![115, 118, 116, 117, 34];
   let mut f3 = File::new_with_data("f3.txt",&f3_data);

   let mut buffer_3: Vec<u8> = vec![];
   open(&mut f3);
   let f3_length = read(&mut f3, &mut buffer_3);
   close(&mut f3);   
   
   let text = String::from_utf8_lossy(&buffer_3);
 
   println!("{:?}", f3);
   println!("{} is {} bytes long", &f3.name, f3_length);
   println!("{}", text);

   let f4_data = vec![110, 119, 119, 117, 34];
   let mut f4 = File::new_with_data("f3.txt",&f4_data);

   let mut buffer_4: Vec<u8> = vec![];
   f4 = open_2(f4).unwrap();    
   let f4_length = f4.read_2(&mut buffer).unwrap();  
   f4 = close_2(f4).unwrap();                        

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
 }