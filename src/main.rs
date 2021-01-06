use iced::{button, Align, Button, Column, Element, Length, Sandbox, Settings, Svg, window};
use core::panic;
use std::env;
use std::process::Command;

struct Monitor {
    name: String,
    // enabled: bool,
    resolutions: Vec<String>,
    primary: bool,
}

fn check_active_monitors() -> Vec<Monitor> {
    let xrandr = Command::new("xrandr").output().expect("some error");

    let input_string = String::from_utf8(xrandr.stdout).unwrap();
    let mut lines = input_string.lines();
    let mut monitor_vec: Vec<Monitor> = Vec::new();

    while let Some(i) = lines.next() {
        let mut words = i.split_whitespace();
        let name = words.next();
        // let mut screen_enabled = false;
        let mut primary = false;

        if words.next() == Some("connected") {
            if words.next() == Some("primary") {
                primary = true;
            }

            //if the moitor is connected and contains * we need to save that variable
            let mut resolutions_vec: Vec<String> = Vec::new();
            let mut first_res_line = lines.next().unwrap().split_whitespace();
            resolutions_vec.push(String::from(first_res_line.next().unwrap()));

            //checks the entire line
            // if first_res_line.next().unwrap().contains(&"*"){
            //     screen_enabled = true;
            // };

            while let Some(resolution_line) = lines.next() {
                let resolution = resolution_line.split_whitespace().next().unwrap();

                let left_side = String::from(resolution.split("x").next().unwrap());
                // println!("{}",left_side);
                if left_side.parse::<i32>().unwrap() > 1000 {
                    resolutions_vec.push(String::from(resolution))
                } else {
                    break;
                }
            }

            monitor_vec.push(Monitor {
                name: String::from(name.unwrap()),
                // enabled: screen_enabled,
                resolutions: resolutions_vec,
                primary: primary,
            })
        }
    }

    return monitor_vec;
}

fn find_common_res(primary: Vec<String>, secondary: Vec<String>) -> (usize, usize) {
    for i in 0..primary.len() {
        for j in 0..secondary.len() {
            if primary[i] == secondary[j] {
                return (i, j);
            }
        }
    }

    return (0, 0);
}

//Currently only works with one external monitor.
pub fn set_mode(message: Message) {
    let mut active_monitors = check_active_monitors();
    let mut primary_index = 0;

    for i in 0..active_monitors.len() {
        if active_monitors[i].primary == true {
            primary_index = i
        };
    }

    //primary monitor is not the currently active one, its what is set as primary in xrandr
    let primary_monitor = active_monitors.remove(primary_index);

    match message {
        Message::ModePrim => {
            Command::new("xrandr")
                .args(&[
                    "--output",
                    &(primary_monitor.name),
                    "--auto",
                    "--output",
                    &(active_monitors[0].name),
                    "--off",
                ])
                .output()
                .expect("some error");

                panic!();
        }

        Message::ModeSec => {
            Command::new("xrandr")
                .args(&[
                    "--output",
                    &(primary_monitor.name),
                    "--off",
                    "--output",
                    &(active_monitors[0].name),
                    "--auto",
                ])
                .output()
                .expect("some error");

                panic!();
        }

        Message::ModeDup => {
            let common_res = find_common_res(
                primary_monitor.resolutions.clone(),
                active_monitors[0].resolutions.clone(),
            );

            Command::new("xrandr")
                .args(&[
                    "--output",
                    &(primary_monitor.name),
                    "--mode",
                    &(primary_monitor.resolutions[common_res.0]),
                    "--output",
                    &(active_monitors[0].name),
                    "--mode",
                    &(active_monitors[0].resolutions[common_res.1]),
                    "--same-as",
                    &(primary_monitor.name),
                ])
                .output()
                .expect("some error");

                panic!();
        }

        Message::ModeExt => {
            Command::new("xrandr")
                .args(&[
                    "--output",
                    &(primary_monitor.name),
                    "--auto",
                    "--output",
                    &(active_monitors[0].name),
                    "--auto",
                    "--left-of",
                    &(primary_monitor.name),
                ])
                .output()
                .expect("some error");

                panic!();
        }
    }
}

// THIS MAIN IS FOR TESTING
// fn main() {

//     let a = [0];
//     let monitor = check_active_monitors();
//     for i in monitor{
//         println!("Screen name {:?}, enabled {:?} , primary {:?}, supported resolutions: {:?}",i.name, i.enabled, i.primary,i.resolutions);
//     };
// }

fn main() -> iced::Result {
    // check_active_monitors();
    let settings = Settings {
            window: window::Settings {
                max_size: Some((400,450)),
                resizable: false,
                ..window::Settings::default()
            },
            ..Default::default()
        };
    ScreenMode::run(settings)
}

#[derive(Default)]
struct ScreenMode {
    image_1: button::State,
    image_2: button::State,
    image_3: button::State,
    image_4: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ModePrim,
    ModeSec,
    ModeDup,
    ModeExt,
}

impl Sandbox for ScreenMode {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Screen Mode Selector")
    }

    fn update(&mut self, message: Message) {
        set_mode(message)
    }

    fn view(&mut self) -> Element<Message> {
        let current_dir: String = env::current_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();
        let path_prim: String = format!("{}/assets/primary-only.svg", current_dir);
        let path_seco: String = format!("{}/assets/secondairy-only.svg", current_dir);
        let path_dup: String = format!("{}/assets/duplicate.svg", current_dir);
        let path_ext: String = format!("{}/assets/extended.svg", current_dir);

        println!("{}", path_prim);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.image_1, Svg::from_path(path_prim))
                    .on_press(Message::ModePrim),
            )
            .push(
                Button::new(&mut self.image_2, Svg::from_path(path_seco))
                    .on_press(Message::ModeSec),
            )
            .push(
                Button::new(&mut self.image_3, Svg::from_path(path_dup)).on_press(Message::ModeDup),
            )
            .push(
                Button::new(&mut self.image_4, Svg::from_path(path_ext)).on_press(Message::ModeExt),
            )
            .into()
    }
}
