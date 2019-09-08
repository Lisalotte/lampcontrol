extern crate philipshue;

use philipshue::bridge;
use philipshue::bridge::Bridge;
use philipshue::hue::LightCommand;
use philipshue::errors::{HueError, HueErrorKind, BridgeError::{LinkButtonNotPressed, DeviceIsUnreachable}};

use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, BTreeMap};


fn register(ip: &str) -> Result<String, ()>{
	for _ in 0..5 {//try 5 times to connect
		match bridge::register_user(ip, "homeAutomationSys") {
			Ok(recieved_login) => {
					println!("Success, linked to brige");
			    return Ok(recieved_login);
			}
			Err(HueError(HueErrorKind::BridgeError { error: LinkButtonNotPressed, .. }, _)) => {
			    println!("Please, press the link on the bridge. Retrying in 5 seconds");
			    thread::sleep(Duration::from_secs(5));
			}
			Err(e) => {
			    println!("Unexpected error occured: {}", e);
			    return Err(());
			}
		}
	}
	return Err(());
}

//fn find_bridge_ip() -> Result<String, ()> {
fn find_bridge_ip() -> String {
	let mut discovered = bridge::discover().unwrap();
	if discovered.len() == 0 {
		println!("No bridge found!");
		//return Err(());
	} else if discovered.len() > 1 {
		println!("Found multiple hue bridges: {:?}, continueing with first one in list", discovered);
	}
    //dbg!(&discovered);
	return discovered.pop().unwrap().into_ip();
}

fn list_lamps(bridge: &Bridge) {
    match bridge.get_all_lights() {
        Ok(lights) => {
            let max_name_len = lights.values().map(|l| l.name.len()).chain(Some(4)).max().unwrap();
            println!("id {0:1$} on  bri alert reachable xy",
                     "name",
                     max_name_len);
            for (id, light) in lights.iter() {
                println!("{:2} {:name_len$} {:3} {:3} {:7} {:8}",
                         id,
                         light.name,
                         if light.state.on { "on" } else { "off" },
                         light.state.bri,
                         light.state.alert,
                         light.state.reachable,
                         name_len = max_name_len);
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}

mod discover;
use discover::discover;
use std::thread::sleep;
use rand::Rng;

fn main() {
    let ip = find_bridge_ip(); 
    println!("{:?}", ip);
    let login = register(&ip);  
    
    let bridge = Bridge::new(&ip, &login.unwrap());   
    list_lamps(&bridge);
    
    let mut command = LightCommand::default();
    command.bri = Some(1);
    loop {
        let x = rand::thread_rng().gen_range(0.0, 0.7);
        let y = rand::thread_rng().gen_range(0.0, 0.9);

        command.xy = Some((x,y));
        command.bri = Some(rand::thread_rng().gen_range(0u8, 255));

        bridge.set_light_state(4, &command).unwrap();
        sleep(Duration::from_millis(500));
    }
    /*
    for h in 0..std::u16::MAX/100 {
        command.hue = Some(h*100);
        bridge.set_light_state(4, &command).unwrap();
        sleep(Duration::from_millis(100));
    }
    */
}