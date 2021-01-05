use std::process::Command;
// use std::clone::Clone;
// use std::string::String;
// use std::fmt

// Create array that is centered

struct Monitor {
    // id:u32,
    name: String,
    // res_high: String,
    // res_other: Vec<String>,
    enabled: bool,
    // freq: u32,

}


fn check_active_monitors() -> Vec<Monitor>{
    let xrandr = Command::new("xrandr").output().expect("some error");

    let input_string = String::from_utf8(xrandr.stdout).unwrap();
    let mut lines = input_string.lines();
    let mut monitor_vec:Vec<Monitor> = Vec::new();


    while let Some(i) = lines.next(){

        let mut words = i.split_whitespace();
        let name = words.next();
        let mut screen_enabled = false;

        if words.next() == Some("connected") {
            // println!("Screen {:?} is connected", name);

            let mut resolution = lines.next().unwrap().split_whitespace();
            resolution.next();

            if resolution.next().unwrap().contains(&"*"){
                screen_enabled = true;
            };

            monitor_vec.push(Monitor{
                name: String::from(name.unwrap()),
                enabled: screen_enabled,
            })
        }
    }

    return monitor_vec;
}


fn main() {
    let monitor = check_active_monitors();
    for i in monitor{
        println!("Screen name {:?}, enabled {:?}",i.name, i.enabled )
    }

}
