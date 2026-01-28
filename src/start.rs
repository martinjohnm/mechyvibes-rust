
pub mod mechyvibes {
    use std::{collections::HashSet, error::Error, fs, sync::{LazyLock, Mutex}};
    use rdev::{Event, listen};
    use serde_json::{Map, Value};
    

    pub use crate::keycode::key_code;
    use crate::sound::{sound::play_sound};



    fn initialize_json(path : &str) -> Result<Map<String, Value>, Box<dyn Error>> {

        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj = parsed.as_object().unwrap().clone();
        Ok(obj)
    }

    pub struct JSONFILE {
        pub value : Option<serde_json::Map<std::string::String, serde_json::Value>>
    }

    impl JSONFILE {
        pub fn initialize(&mut self, directory : String) {
            let soundpack_config = &format!("{}/config.json", directory)[..];
            self.value = Some(initialize_json(soundpack_config).unwrap());
        }

        pub fn event_handler(self: &Self, event : Event, directory : String, volume : u16) {
            match &self.value {
                Some(value) => {
                    callback(event, value.clone(), directory, volume);
                } 
                None => {
                    println!("json wasn't initialized yet");
                }
            }
        }
    }

    pub fn start_mechyvibes(args: String, volume: u16) {
        {
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            unsafe {
                use libc::nice;
                nice(-20)
            };
        }

        {
            #[cfg(target_os = "windows")]
            {
                use thread_priority::*;
                assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
            }
        }

        let mut json_file = JSONFILE {value: None};
        json_file.initialize(args.clone());

        println!("soundpacks loaded");
        println!("mechyvibes running");

        let event_handler = move |event: Event| {
            json_file.event_handler(event, args.clone(), volume);
        };

        if let Err(error) = listen(event_handler) {
            println!("Error : {:?}", error)
        }


    }


    static KEY_DEPRESSED : LazyLock<Mutex<HashSet<i32>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

    fn callback(event : Event, json_file: serde_json::Map<std::string::String, serde_json::Value>, directory: String, vol: u16) {

        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                let key_code = key_code::code_from_key(key);
                let key_down = KEY_DEPRESSED
                    .lock()
                    .expect("Cant open key depresssed yet")
                    .insert(key_code.unwrap());

                if key_down {
                    let mut dest = match key_code {
                        Some(code) => json_file["defines"][&code.to_string()].to_string(),
                        None => {
                            println!("Unmapped key: {:?}",key);
                            let default_key = 30; // keycode for 'a'
                            json_file["defines"][&default_key.to_string()].to_string()
                        }
                    };
                    dest.remove(0);
                    dest.remove(dest.len()-1);
                    play_sound(format!("{}/{}", directory, dest), vol);
                }
        
            }

            rdev::EventType::KeyRelease(key) => {
                let key_code = key_code::code_from_key(key);
                KEY_DEPRESSED
                    .lock()
                    .expect("cant open depressed ser for removal")
                    .remove(&key_code.unwrap_or(0));
                    // key up sound here
            }
            _ => ()
        }

        // println!("{:?}", KEY_DEPRESSED)
    }



}