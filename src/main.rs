use iced::{
    button, svg, window, Align, Button, Color, Column, Container, Element, Length, Sandbox,
    Settings, Svg,
};
use std::env;
use packer::Packer;
use std::{io::Bytes, process::Command, vec};


struct Monitor {
    name: String,
    // enabled: bool,
    resolutions: Vec<String>,
    primary: bool,
}


//
// ─── XRANDR OUTPUT PARSER START ────────────────────────────────────────────────────────
//

fn check_active_monitors() -> Vec<Monitor> {
    //run the command and capture output
    let xrandr = Command::new("xrandr").output().expect("Could not run command");

    //Convert to string
    let input_string = String::from_utf8(xrandr.stdout).unwrap();

    //Split to lines
    let mut lines = input_string.lines();

    //Create the vector where we will store the monitors found
    let mut monitor_vec: Vec<Monitor> = Vec::new();

    //Pattern-matching for each line, unwrapping option to get the value.
    while let Some(single_line) = lines.next() {

        //Split lines again into "words" which will be used alot
        let mut words = single_line.split_whitespace();

        //First word is the name of the monitor
        let monitor_name = words.next();

        //Code that might come in handy, but commented out for now
        // let mut screen_enabled = false;

        //Create default variable for is this primary.
        let mut primary=false;

        //Check if the second word says connected
        if words.next() == Some("connected") {
            //Check if its primary
            if words.next() == Some("primary") {
                primary = true;
            }

            //Create a vector to store the resolutions that are supported
            let mut resolutions_vec: Vec<String> = Vec::new();

            //This is the first monitor resolution. Also the highest resolution that monitor supports according to xrandr
            resolutions_vec.push(String::from(lines.next().unwrap().split_whitespace().next().unwrap()));

            //Code that might come in handy, but commented out for now
            //if the monitor is connected and contains * it means that is currently displaying on that mode
            //checks the entire line
            // if first_res_line.next().unwrap().contains(&"*"){
            //     screen_enabled = true;
            // };

            //The next few lines are resolutions the monitor supports
            //They are iterated until we get to a resolution that is to low to matter
            while let Some(resolution) = lines.next().unwrap().split_whitespace().next() {

                let left_side = String::from(resolution.split("x").next().unwrap());

                //Check if the horizontal resolution is to low to matter and break out of while if it is
                if left_side.parse::<i32>().unwrap() < 1000 {
                    break;
                }

                //Otherwise save the resolution
                resolutions_vec.push(String::from(resolution))
            }

            //Add the monitor that we found to a vector which is returned when while loop finishes.
            monitor_vec.push(Monitor {
                name: String::from(monitor_name.unwrap()),
                resolutions: resolutions_vec,
                primary: primary,
            })
        }
    }
//
// ─── XRANDR OUTPUT PARSER END ────────────────────────────────────────────────────
//



    //Check if you don't have any other monitors connected, enable that one and quit.
    if monitor_vec.len() == 1 {
        Command::new("xrandr")
            .args(&["--output", &(monitor_vec[0].name), "--auto"])
            .output()
            .expect("Could not run command");

        std::process::exit(0)
    }
    return monitor_vec;
}


//Resolution finder, starts checking from the primary displays highest resolution
//The order highest to lowest, is due to the order of the xrandr output
//
//Takes the primary display in the first input, input order matters.
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

//Currently only works with one external monitor, and one primary monitor
pub fn set_mode(message: Message) {
    let mut active_monitors = check_active_monitors();
    let mut primary_index = 0;

    //Find the index of the primary display
    for i in 0..active_monitors.len() {
        if active_monitors[i].primary == true {
            primary_index = i
        };
    }

    //primary monitor is not the currently active one, its what is set as primary in xrandr
    let primary_monitor = active_monitors.remove(primary_index);

    //Match what button was pressed, run command and close
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
                .expect("Could not run command");

            std::process::exit(0)
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
                .expect("Could not run command");

            std::process::exit(0)
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
                .expect("Could not run command");

            std::process::exit(0)
        }

        //Set mode to extended, defaults to left because mine is on my left.
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
                .expect("Could not run command");


            std::process::exit(0)
        }
    }
}

//Packer struct for storing assets in binary
#[derive(Packer)]
#[packer(source = "assets")]
struct Assets;

//Creates a handle struct for loading svg data from binary,
//Luckily iced implements a way to load svg's from bytes, just needs a little coercion.
fn svg_create_handle(file_name: &str) -> svg::Handle {
    let data: Option<&'static [u8]> = Assets::get(file_name);
    match data{
        None => {return svg::Handle::from_memory(Vec::new())
        }
        Some(data) => {return svg::Handle::from_memory(data);
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
    // This is run to check if any monitors we only have one monitor connected,
    // if so it will enable the connected monitor and immediately close
    check_active_monitors();

    //setting resizable:false, and max_size seems to force floating on tiling window managers
    let settings = Settings {
        window: window::Settings {
            max_size: Some((400, 450)),
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

//message struct for button presses
#[derive(Debug, Clone, Copy)]
pub enum Message {
    ModePrim,
    ModeSec,
    ModeDup,
    ModeExt,
}

// iced sandbox
impl Sandbox for ScreenMode {
    type Message = Message;

    //iced
    fn new() -> Self {
        Self::default()
    }

    //Window title
    fn title(&self) -> String {
        String::from("Screen Mode Selector")
    }


    //Usually match statement happens here, but for future implementation of
    //selection with keyboard another match statement is needed,
    //therefore actual commands are in their own function.
    fn update(&mut self, message: Message) {
        set_mode(message)
    }

    fn view(&mut self) -> Element<Message> {

        //iced event handling, content is put in a column
        let content = Column::new()
            .padding(20)
            .align_items(Align::Center)
            //Pushing buttons into column
            .push(
                Button::new(
                    &mut self.image_1,
                    Svg::new(svg_create_handle("assets/primary-only.svg")),
                )
                .on_press(Message::ModePrim)
                .style(style::Button::Primary),
            )
            .push(
                Button::new(
                    &mut self.image_2,
                    Svg::new(svg_create_handle("assets/secondary-only.svg")),
                )
                .on_press(Message::ModeSec)
                .style(style::Button::Primary),
            )
            .push(
                Button::new(
                    &mut self.image_3,
                    Svg::new(svg_create_handle("assets/duplicate.svg")),
                )
                .on_press(Message::ModeDup)
                .style(style::Button::Primary),
            )
            .push(
                Button::new(
                    &mut self.image_4,
                    Svg::new(svg_create_handle("assets/extended.svg")),
                )
                .on_press(Message::ModeExt)
                .style(style::Button::Primary),
            );

        //A container is needed to set background color
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container::BackgroundColor)
            .into()
    }
}

//Styling of background and buttons
mod style {
    use iced::{button, container, Background, Color, Vector};

    pub enum Button {
        Primary,
        Selected,
    }

    pub enum Container {
        BackgroundColor,
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb(
                    30.0 / 255.0,
                    36.0 / 255.0,
                    41.0 / 255.0,
                ))),
                ..container::Style::default()
            }
        }
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(30.0 / 255.0, 36.0 / 255.0, 41.0 / 255.0),
                    Button::Selected => Color::from_rgb(15.0 / 255.0, 87.0 / 255.0, 148.0 / 255.0),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(0.0, 0.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}
