

use std::error::Error;
use std::time::Duration;
use tokio::time;

use btleplug::api::{Central, Manager as _, Peripheral};
use btleplug::platform::Manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty() {
        eprintln!("No Bluetooth adapters found");
    }

    for adapter in adapter_list.iter() {
        println!("Starting scan...");
        adapter
            .start_scan()
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;
        if peripherals.is_empty() {
            eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
        } else {
            // All peripheral devices in range
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));
                println!(
                    "Peripheral {:?} is connected: {:?}",
                    local_name, is_connected
                );
                if !is_connected {
                    println!("Connecting to peripheral {:?}...", &local_name);
                    if let Err(err) = peripheral.connect().await {
                        eprintln!("Error connecting to peripheral, skipping: {}", err);
                        continue;
                    }
                }
                let is_connected = peripheral.is_connected().await?;
                println!(
                    "Now connected ({:?}) to peripheral {:?}...",
                    is_connected, &local_name
                );
                let chars = peripheral.discover_characteristics().await?;
                if is_connected {
                    println!("Discover peripheral {:?} characteristics...", &local_name);
                    for characteristic in chars.into_iter() {
                        println!("{:?}", characteristic);
                    }
                    println!("Disconnecting from peripheral {:?}...", &local_name);
                    peripheral
                        .disconnect()
                        .await
                        .expect("Error disconnecting from BLE peripheral");
                }
            }
        }
    }
    Ok(())
}












/*
use core::time;
use std::{sync::mpsc, thread};

use bluer::Adapter;

async fn query_device(adapter: &Adapter, addr: Address) -> bluer::Result<()> {
    let device = adapter.device(addr)?;
    println!("    Address type:       {}", device.address_type().await?);
    println!("    Name:               {:?}", device.name().await?);
    println!("    Icon:               {:?}", device.icon().await?);
    println!("    Class:              {:?}", device.class().await?);
    println!("    UUIDs:              {:?}", device.uuids().await?.unwrap_or_default());
    println!("    Paried:             {:?}", device.is_paired().await?);
    println!("    Connected:          {:?}", device.is_connected().await?);
    println!("    Trusted:            {:?}", device.is_trusted().await?);
    println!("    Modalias:           {:?}", device.modalias().await?);
    println!("    RSSI:               {:?}", device.rssi().await?);
    println!("    TX power:           {:?}", device.tx_power().await?);
    println!("    Manufacturer data:  {:?}", device.manufacturer_data().await?);
    println!("    Service data:       {:?}", device.service_data().await?);
    Ok(())
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    tokio::spawn(async move {
        let session = bluer::Session::new().await.unwrap();
        let adapter_names = session.adapter_names().await.unwrap();
        let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
        let adapter = session.adapter(adapter_name).unwrap();
        let _t = adapter.set_powered(true).await;
       
        loop  {
            adapter.discover_devices().and_then(|devicesStream | {
                devicesStream.for_each(|device| async{
                    if let AdapterEvent:: DeviceAdded(addr) = device {
                        query_device(&adapter, addr).await;
                    }

                })
            });

                     //   {
                     //       if let AdapterEvent::DeviceAdded(addr) = device {
                     //           query_device(&adapter,addr).await;
                     //           tx.send(1).and_then(|_opp| Ok(device));
                     //       }   
                     //   }
            }
    });
    //let  corte = false;
//
    //while !corte {
//
    //    let null =  String::from("null");
    //    let received= rx.try_recv().unwrap_or_else(|e|  2);
    //    thread::sleep(time::Duration::from_secs(3));
    //    println!("Got:{}", received);
    //
    //}
}*/

