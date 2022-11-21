// #[warn(dead_code)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::time::Duration;
use std::{thread};

type MyResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn handle_client(mut stream: TcpStream) -> MyResult {
    let mut data = String::new();
    
    // println!("handle client is running...");

    stream.read_to_string(&mut data)?;
    if data.is_empty(){
        println!("Data is empty");
    } else {
        write!(stream, "{}", data)?;
        println!("From {}", data);
    }
    Ok(())
}

fn read_from_console() -> String {
    let mut string: String = String::new();
    io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");
    return string;
}

fn reg_name() -> String{
    let mut value = String::new();
    while value.is_empty() {
        println!("Input you name: ");
        if let Err(err) = io::stdin().read_line(&mut value){
            println!("{:#?}", err);
        };
    } 
    return value;
}

// fn reg_server_addr() -> String{
    // let mut value = String::new();
    // while value.is_empty() {
        // println!("Input you server addr: ");
        // if let Err(err) = io::stdin().read_line(&mut value){
            // println!("{:#?}", err);
        // };
    // }
    // return value + ": ";
// }

// fn reg_client_addr() -> String{
    // let mut value = String::new();
    // while value.is_empty() {
        // println!("Input you client addr: ");
        // if let Err(err) = io::stdin().read_line(&mut value){
            // println!("{:#?}", err);
        // };
    // }
    // return value + ": ";
// }

fn main(){
    let name = reg_name();
    let server_addr = "192.168.0.239:8080";
    // let client_addr = "192.168.2.234:8080";
    let client_addr = "192.168.0.239:8080";

    // let server_addr = reg_server_addr();
    // let client_addr = reg_client_addr();

    //Server
    let server_thread = thread::spawn(move || -> MyResult{
        let listener = TcpListener::bind(server_addr)?;
        println!("Server listening on port 8080!\n");

        for stream in listener.incoming() {
            if let Err(why) = stream {
                println!("Error: {}", why);
                continue;
            }
            let stream = stream.unwrap();

            let my_some = handle_client(stream);

            if let Err(why) = my_some {
                println!("Error: {}", why);
            }
        }
        Ok(())
    });


    //Client
    println!("Now you can chating! ");
    

    loop {
        if false {break;}
        let message = read_from_console();
        let mut connection = loop {
            if let Ok(tspstream) = TcpStream::connect(client_addr){
                break tspstream;
            } else {
                thread::sleep(Duration::from_secs(1));
            }
        };
        write!(connection, "{}: {}", name.replace("\n", ""), message).unwrap();
    }

    if let Err(why) = server_thread.join(){
        println!("Error: {:#?}", why);
    }

}
