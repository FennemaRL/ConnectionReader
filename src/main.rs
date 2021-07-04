use core::time;
use std::io;
use std::sync::mpsc;
use std::thread;


fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop{
            let mut input = String::new();
            println!("Ingrese un valor");
            io::stdin().read_line(&mut input).expect("Error reading from STDIN");
            tx.send(input).unwrap();
        }
    });
    let mut  corte = false;
    while !corte {
        let received= rx.try_recv().unwrap_or_else(|_e|  {
            "_\n".to_string()}).replace("\n","");
        thread::sleep(time::Duration::from_secs(3));
        println!("Got:{}, and comparation : {}", &received, received == "corte");
        if received == "corte" {
            corte=true;
        }
    }
}