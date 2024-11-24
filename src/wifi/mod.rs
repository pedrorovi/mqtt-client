use embedded_svc::wifi::Wifi;
use esp_idf_hal::modem;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::wifi::{ClientConfiguration, Configuration, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use std::{thread::sleep, time::Duration};

use esp_idf_sys::esp_netif_get_ip_info;
use esp_idf_sys::esp_netif_ip_info_t;
use esp_idf_sys::esp_netif_t;

pub struct WifiController<'a> {
    client: EspWifi<'a>,
}

impl<'a> WifiController<'a> {
    pub fn new(
        ssid: &str,
        password: &str,
        modem: modem::Modem,
    ) -> Result<Self, esp_idf_sys::EspError> {
        let sys_loop = EspSystemEventLoop::take().map_err(|e| {
            eprintln!("Failed to take system event loop: {:?}", e);
            e
        })?;
        let nvs = EspDefaultNvsPartition::take().map_err(|e| {
            eprintln!("Failed to take NVS partition: {:?}", e);
            e
        })?;

        let mut wifi_driver = EspWifi::new(modem, sys_loop, Some(nvs)).map_err(|e| {
            eprintln!("Failed to create WiFi driver: {:?}", e);
            e
        })?;
        let mut config = ClientConfiguration::default();

        config.ssid = ssid.try_into().unwrap();
        config.password = password.try_into().unwrap();

        wifi_driver
            .set_configuration(&Configuration::Client(config))
            .map_err(|e| {
                eprintln!("Failed to set WiFi configuration: {:?}", e);
                e
            })?;
        wifi_driver.start().map_err(|e| {
            eprintln!("Failed to start WiFi driver: {:?}", e);
            e
        })?;
        wifi_driver.connect().map_err(|e| {
            eprintln!("Failed to connect to WiFi: {:?}", e);
            e
        })?;
        while !wifi_driver.is_connected().map_err(|e| {
            eprintln!("Failed to check WiFi connection status: {:?}", e);
            e
        })? {
            sleep(Duration::from_secs(1));
        }

        Ok(WifiController {
            client: wifi_driver,
        })
    }

    pub fn is_connected(&self) -> bool {
        self.client.is_connected().unwrap_or(false)
    }

}
