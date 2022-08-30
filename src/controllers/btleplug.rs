use super::{BleController, BlePeripheral};
use async_trait::async_trait;
use std::error::Error;
use std::time::Duration;
use tokio::time;

// mod utils;
use crate::utils;

use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::{Adapter, Manager};

pub struct BtleplugController {
    controller_name: String,
    peripheral_name: Option<String>,
    peripheral_mac: Option<String>,
    connected: bool,
    manager: Manager,
    adapter: Adapter,
    // TODO make this line compile to store the scanned periphs in the structure
    // scanned_peripherals: Vec<Peripheral>
}

#[async_trait]
impl BleController for BtleplugController {
    async fn scan(
        &self,
        scan_time_s: u32,
        print_result: bool,
    ) -> Result<Vec<BlePeripheral>, Box<dyn Error>> {
        println!("Scanning for {} seconds...", scan_time_s);

        self.adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan BLE adapter for connected devices...");
        time::sleep(Duration::from_secs(scan_time_s as u64)).await;

        let peripherals = self.adapter.peripherals().await?;
        let mut periph_vec: Vec<BlePeripheral> = Vec::new();

        for p in peripherals {
            let properties = p.properties().await?.unwrap();
            let local_name = properties
                .local_name
                .unwrap_or(String::from("(name unknown)"));
            let mac = properties.address;

            let per = BlePeripheral {
                name: local_name,
                mac_addr: mac.to_string(),
            };
            if print_result {
                println!("mac_addr = {}\tname = {}", per.mac_addr, per.name);
            }
            periph_vec.push(per);
        }

        Ok(periph_vec)
    }

    fn connect_by_name(&self, name: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn connect_by_mac(&self, mac: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn disconnect(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

impl BtleplugController {
    pub async fn new() -> BtleplugController {
        let manager = match Manager::new().await {
            Ok(m) => m,
            Err(e) => panic!("{}", e),
        };

        let adapter_list = match manager.adapters().await {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        let adapter = match adapter_list.len() {
            0 => panic!("No adapter available"),
            1 => &adapter_list[0],
            _ => {
                println!("Found multiple adapters, select the one to use:");
                let mut index = 0;
                for ad in &adapter_list {
                    println!("[{}]: {:?}", index, ad);
                    index += 1;
                }
                let n = utils::get_usize_input(">>");
                &adapter_list[n]
            }
        };

        println!("Using BLE adapter: {:?}", adapter.adapter_info().await.unwrap());

        BtleplugController {
            controller_name: String::from("btleplug"),
            peripheral_name: None,
            peripheral_mac: None,
            connected: false,
            manager,
            adapter: adapter.clone(),
        }
    }
}