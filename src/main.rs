use std::process::Command;
use iced::{Column, Element, Length, Sandbox, Settings, Svg, button, Align, Button, Text};

// use std::clone::Clone;
// use std::string::String;
// use std::fmt

// Create array that is centered

struct Monitor {
    name: String,
    enabled: bool,
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

            //checks entire line...
            if lines.next().unwrap().contains(&"*"){
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


fn main() -> iced::Result{
    let monitor = check_active_monitors();
    for i in monitor{
        println!("Screen name {:?}, enabled {:?}",i.name, i.enabled );
    };


    ScreenMode::run(Settings::default())

}

#[derive(Default)]
struct ScreenMode{
    image_1: button::State,
    image_2: button::State,
    image_3: button::State,
    image_4: button::State,

}



#[derive(Debug, Clone, Copy)]
enum Message {
    Click1,
    Click2,
    Click3,
    Click4,
}


impl Sandbox for ScreenMode{

    type Message = Message;

    fn new() -> Self{
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Screen mode Selector")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Click1 => { println!("YOU CLICKED SCREEN 1") }
            Message::Click2 => { println!("YOU CLICKED SCREEN 2") }
            Message::Click3 => { println!("YOU CLICKED SCREEN 3") }
            Message::Click4 => { println!("YOU CLICKED SCREEN 4") }
        }
    }

    fn view(&mut self) -> Element<Message>{

        let path_prim:String=format!("/home/ardijan/downloads/repos/screen-Toggle/primary-only.svg");
        let path_seco:String=format!("/home/ardijan/downloads/repos/screen-Toggle/secondairy-only.svg");
        let path_dup:String=format!("/home/ardijan/downloads/repos/screen-Toggle/duplicate.svg");
        let path_ext:String=format!("/home/ardijan/downloads/repos/screen-Toggle/extended.svg");


        Column::new()
        .padding(20)
        .align_items(Align::Center)
        .push(
            Button::new(&mut self.image_1, Svg::from_path(path_prim))
            .on_press(Message::Click1),
        )
        .push(
            Button::new(&mut self.image_2, Svg::from_path(path_seco))
            .on_press(Message::Click2),
        )
        .push(
            Button::new(&mut self.image_3, Svg::from_path(path_dup))
            .on_press(Message::Click3),
        )
        .push(
            Button::new(&mut self.image_4, Svg::from_path(path_ext))
            .on_press(Message::Click4),
        )
        .into()

    }
}


