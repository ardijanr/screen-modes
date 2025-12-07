use iced::application::{self};
use iced::keyboard::KeyCode;
use iced::widget::{button, image, pane_grid, Column, Container};
use iced::window::Settings;
use iced::{
    event, executor, keyboard, subscription, theme, Alignment, Application, Color, Element, Event,
    Length, Subscription, Theme,
};
use std::fmt;
use std::process::{exit, Command};

struct Monitor {
    name: String,
    // enabled: bool,
    resolutions: Vec<Resolution>,
    primary: bool,
}

impl Monitor {
    fn get_best_res_and_rate(&self) -> (Resolution, RefreshRate) {
        let (re_i, ra_i) = if let Some((mut re_i, ra_i)) = self
            .resolutions
            .iter()
            .enumerate()
            .find_map(|(resolution_i, resolution)| {
                resolution
                    .rates
                    .iter()
                    .position(|rate| rate.currently_active)
                    .map(|rate_i| (resolution_i, rate_i))
            }) {
            if ra_i + 1 >= self.resolutions[re_i].rates.len() {
                re_i += 1;
            }

            (re_i, ra_i + 1)
        } else {
            (0, 0)
        };

        (
            self.resolutions[re_i].clone(),
            self.resolutions[re_i].rates[ra_i].clone(),
        )
    }
}

#[derive(Clone)]
struct Resolution {
    horizontal: u32,
    vertical: u32,
    rates: Vec<RefreshRate>,
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.horizontal, self.vertical)
    }
}

#[derive(Clone)]
struct RefreshRate {
    str_value: String,
    currently_active: bool,
}
impl PartialEq for Resolution {
    fn eq(&self, other: &Resolution) -> bool {
        self.horizontal == other.horizontal && self.vertical == other.vertical
    }
}

//
// ─── XRANDR OUTPUT PARSER START ────────────────────────────────────────────────────────
//

fn check_active_monitors() -> Vec<Monitor> {
    //run the command and capture output
    let xrandr = Command::new("xrandr")
        .output()
        .expect("Could not run command");

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
        let mut primary = false;

        //Check if the second word says connected
        if words.next() == Some("connected") {
            //Check if its primary
            if words.next() == Some("primary") {
                primary = true;
            }

            //Create a vector to store the resolutions that are supported
            let mut resolutions_vec: Vec<Resolution> = Vec::new();

            //The next few lines are resolutions the monitor supports
            //They are iterated until we get to a resolution that is to low to matter
            while let Some(mut resolution_line) = lines.next().map(|rl| rl.split_whitespace()) {
                let mut resolution = resolution_line.next().unwrap().split('x');
                let mut refresh_rates: Vec<RefreshRate> = Vec::new();

                for mut rate_str in resolution_line.map(|x| x.to_string()) {
                    let current_rate = rate_str.contains('*');

                    rate_str = rate_str.replace(['*', '+'], "");
                    if rate_str.is_empty() {
                        continue;
                    }

                    refresh_rates.push(RefreshRate {
                        str_value: rate_str,
                        currently_active: current_rate,
                    });
                }

                if let (Some(h), Some(v)) = (
                    resolution.next().and_then(|x| x.parse::<u32>().ok()),
                    resolution.next().and_then(|x| x.parse::<u32>().ok()),
                ) {
                    if h < 1000 {
                        break;
                    };

                    resolutions_vec.push(Resolution {
                        horizontal: h,
                        vertical: v,
                        rates: refresh_rates,
                    });
                }
            }

            //Add the monitor that we found to a vector which is returned when while loop finishes.
            monitor_vec.push(Monitor {
                name: String::from(monitor_name.unwrap()),
                resolutions: resolutions_vec,
                primary,
            })
        }
    }
    //
    // ─── XRANDR OUTPUT PARSER END ────────────────────────────────────────────────────
    //

    //Check if you don't have any other monitors connected, enable that one and quit.
    if monitor_vec.len() == 1 {
        Command::new("xrandr")
            .args(["--output", &(monitor_vec[0].name), "--auto"])
            .output()
            .expect("Could not run command");

        std::process::exit(0)
    }
    monitor_vec
}

//Resolution finder, starts checking from the primary displays highest resolution
//The order highest to lowest, is due to the order of the xrandr output
//
//Takes the primary display in the first input, input order matters.
fn find_common_res(primary: Vec<Resolution>, secondary: Vec<Resolution>) -> (usize, usize) {
    for (i, p) in primary.iter().enumerate() {
        for (j, s) in secondary.iter().enumerate() {
            if p == s {
                return (i, j);
            }
        }
    }
    (0, 0)
}

//Currently only works with one external monitor, and one primary monitor
pub fn set_mode(message: Message) {
    let mut active_monitors: Vec<Monitor> = check_active_monitors();

    //Find the index of the primary display
    let primary_index = active_monitors.iter().position(|x| x.primary).unwrap_or(0);
    //primary monitor is not the currently active one, its what is set as primary in xrandr
    let primary_monitor = active_monitors.remove(primary_index);

    //Match what button was pressed, run command and close
    match message {
        Message::PrimaryOnly => {
            Command::new("xrandr")
                .args([
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

        Message::SecondaryOnly => {
            let monitor = &active_monitors[0];
            let (res, rate) = monitor.get_best_res_and_rate();
            let args = [
                "--output",
                &(primary_monitor.name),
                "--off",
                "--output",
                &(monitor.name),
                "--mode",
                &(res.to_string()),
                "--rate",
                &(rate.str_value),
            ];

            dbg!(&args);
            Command::new("xrandr")
                .args(args)
                .output()
                .expect("Could not run command");

            std::process::exit(0)
        }

        Message::Duplicate => {
            let common_res = find_common_res(
                primary_monitor.resolutions.clone(),
                active_monitors[0].resolutions.clone(),
            );

            Command::new("xrandr")
                .args([
                    "--output",
                    &(primary_monitor.name),
                    "--mode",
                    &(primary_monitor.resolutions[common_res.0]).to_string(),
                    "--output",
                    &(active_monitors[0].name),
                    "--mode",
                    &(active_monitors[0].resolutions[common_res.1]).to_string(),
                    "--same-as",
                    &(primary_monitor.name),
                ])
                .output()
                .expect("Could not run command");

            std::process::exit(0)
        }

        //Set mode to extended, defaults to left because mine is on my left.
        Message::Extend => {
            Command::new("xrandr")
                .args([
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

const PRIMARY_ONLY: &[u8] = include_bytes!("./assets_png/primary-only.png");
const SECONDARY_ONLY: &[u8] = include_bytes!("assets_png/secondary-only.png");
const DUPLICATE: &[u8] = include_bytes!("assets_png/duplicate.png");
const EXTENDED: &[u8] = include_bytes!("assets_png/extended.png");

fn main() -> iced::Result {
    // This is run to check if any monitors we only have one monitor connected,
    // if so it will enable the connected monitor and immediately close
    check_active_monitors();

    // setting resizable:false, and max_size seems to force floating on tiling window managers
    let settings = iced::Settings {
        window: Settings {
            max_size: Some((400, 450)),
            resizable: false,
            ..Settings::default()
        },
        ..Default::default()
    };
    ScreenMode::run(settings)
}

struct ScreenMode {
    primary_only: image::Handle,
    secondary_only: image::Handle,
    duplicate: image::Handle,
    extended: image::Handle,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PrimaryOnly,
    SecondaryOnly,
    Duplicate,
    Extend,
}

impl Application for ScreenMode {
    type Message = Message;

    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();
    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let (_panes, _) = pane_grid::State::new(());

        (
            ScreenMode {
                primary_only: image::Handle::from_memory(PRIMARY_ONLY),
                secondary_only: image::Handle::from_memory(SECONDARY_ONLY),
                duplicate: image::Handle::from_memory(DUPLICATE),
                extended: image::Handle::from_memory(EXTENDED),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Screen Mode Selector")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        set_mode(message);
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        //iced event handling, content is put in a column
        let content = Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            //Pushing buttons into column
            .push(
                button::Button::new(
                    image(self.primary_only.clone())
                        .width(Length::Fill)
                        .height(Length::Shrink),
                )
                .on_press(Message::PrimaryOnly)
                .style(theme::Button::Custom(Box::new(ButtonColor::Primary))),
            )
            .push(
                button::Button::new(
                    image(self.secondary_only.clone())
                        .width(Length::Fill)
                        .height(Length::Shrink),
                )
                .on_press(Message::SecondaryOnly)
                .style(theme::Button::Custom(Box::new(ButtonColor::Primary))),
            )
            .push(
                button::Button::new(
                    image(self.duplicate.clone())
                        .width(Length::Fill)
                        .height(Length::Shrink),
                )
                .on_press(Message::Duplicate)
                .style(theme::Button::Custom(Box::new(ButtonColor::Primary))),
            )
            .push(
                button::Button::new(
                    image(self.extended.clone())
                        .width(Length::Fill)
                        .height(Length::Shrink),
                )
                .on_press(Message::Duplicate)
                .style(theme::Button::Custom(Box::new(ButtonColor::Primary))),
            );

        //A container is needed to set background color
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }),
                event::Status::Ignored,
            ) => match key_code {
                KeyCode::Key1 => Some(Message::PrimaryOnly),
                KeyCode::Key2 => Some(Message::SecondaryOnly),
                KeyCode::Key3 => Some(Message::Duplicate),
                KeyCode::Key4 => Some(Message::Extend),
                KeyCode::Escape => exit(0),
                KeyCode::Q => exit(0),
                _ => None,
            },
            _ => None,
        })
    }

    fn style(&self) -> theme::Application {
        fn dark_background(_theme: &Theme) -> application::Appearance {
            application::Appearance {
                background_color: Color::from_rgb8(33, 38, 46),
                text_color: Color::WHITE,
            }
        }

        theme::Application::from(dark_background as fn(&Theme) -> _)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonColor {
    Primary,
    Selected,
}

impl button::StyleSheet for ButtonColor {
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color {
                r: 0.,
                g: 0.,
                b: 0.,
                a: 0.,
            })),
            ..button::Appearance::default()
        }
    }

    type Style = iced::Theme;
}
