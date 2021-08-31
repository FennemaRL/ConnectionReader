use core::time;
use std::time::Duration;
use std::{fmt, io};
use std::sync::mpsc;
use std::thread;

use bluez::Address;
use bluez::client::BlueZClient;
use tokio::time::sleep;


struct ShortbtData {
    pub name: String,
    pub short_name: String,

    pub address: Address,
    pub bluetooth_version: u8,
}

impl fmt::Display for ShortbtData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(name: {}, short_name: {}, address: {},bluetooth_version: {})", &self.name, &self.short_name, self.address, self.bluetooth_version)
    }
}
#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    tokio::spawn(async move {

        let mut client = BlueZClient::new().unwrap();
        loop  {
            let controllers =match client.get_controller_list().await   {
                Ok(res) => res,
                _ => Vec::new()
            };

            println!("Ok controller info :{:#?}",&controllers);
            for controller in controllers {

                match client.get_controller_info(controller).await {

                    Ok(info) =>{

                        println!("Ok controller info ");
                        let short_name = info.short_name.into_string().unwrap();
                        let name = info.name.into_string().unwrap();
                        tx.send(ShortbtData{
                            address: info.address,
                            short_name: short_name.clone(),
                            bluetooth_version: info.bluetooth_version,
                            name: name.clone()
                        });
                       return ShortbtData{
                        address: info.address,
                        short_name: short_name,
                        bluetooth_version: info.bluetooth_version,
                        name: name
                    }
                    },
                    Err(e) => {
                        println!("error controller info un wrap :{}", e.to_string());
                        let null =  String::from("null");
                tx.send(ShortbtData{
                    address: Address::new([0,0,0,0,0,0]),
                    name: null.clone(),
                    short_name:null.clone(),
                    bluetooth_version: 0,
                });
                        return ShortbtData{
                            address: Address::new([0,0,0,0,0,0]),
                            name: null.clone(),
                            short_name: null,
                            bluetooth_version: 0,
                        }
                    
                    }
             };
            }
            sleep(Duration::from_millis(1400)).await;
        }
    });
    let  corte = false;

    while !corte {

        let null =  String::from("null");
        let received= rx.try_recv().unwrap_or_else(|_e | { return  ShortbtData {
            address: Address::new([0,0,0,0,0,0]),
            name: null.clone(),
            short_name: null,
            bluetooth_version: 0,
        } });
        thread::sleep(time::Duration::from_secs(3));
        println!("Got:{}", received);
    
    }
}