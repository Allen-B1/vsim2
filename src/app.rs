use iced::*;
use crate::*;

#[derive(Debug)]
pub struct State {
    stage: Option<ElectionStage>,
    results: Option<ElectionResults>,
    district: Option<usize>,

    map: map::Map,
    map_scroll: scrollable::State,
}

#[derive(Debug,Clone)]
pub enum Msg {
    Map(map::Msg),
    LoadStage(ElectionStage),
    LoadResults(ElectionResults),
}

type Result<T, E> = std::result::Result<T, E>;

impl Application for State {
    type Message = Msg;
    type Flags = ();
    type Executor = executor::Default;

    fn new(flags: ()) -> (State, Command<Msg>) {
        (State {
            results: None,
            stage: None,
            district: None,

            map: map::Map::new(),
            map_scroll: Default::default()
        }, async {
            let stage: ElectionStage = serde_json::from_reader(fs::File::open("src-canada-stage.json").unwrap()).unwrap();

            Msg::LoadStage(stage)
        }.into())
    }

    fn title(&self) -> String {
        "Voting Simulator 2".to_string()
    }

    fn update(&mut self, event: Msg, clipboard: &mut Clipboard) -> Command<Msg> {
        match event {
            Msg::Map(map_msg) => {
                match &map_msg {
                    map::Msg::ViewDistrict(district) => {
                        self.district = Some(*district);
                    }
                };
                self.map.update(map_msg);
                Command::none()
            },
            Msg::LoadStage(stage) => {
                self.stage = Some(stage);
                Command::none()
            },
            Msg::LoadResults(results) => {
                self.results = Some(results);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Msg> {
        let mut root = Row::new().width(Length::Fill);
        if let Some(stage) = &self.stage {
            root = root.push(
                Scrollable::new(&mut self.map_scroll).scrollbar_width(16).push(
                    self.map.view(stage, self.results.as_ref(), self.district).map(Msg::Map)
                )
            );
        }

        root.into()
    }
}

fn create_button<'a, Msg: Clone>(state: &'a mut button::State, label: &str) -> Button<'a, Msg> {
    Button::new(state, Text::new(label).horizontal_alignment(HorizontalAlignment::Center)).padding(12)
}

pub mod palette {
    use iced::Color;
    pub fn unknown() -> Color {
        Color::from_rgb8(0xaa, 0xaa, 0xaa)
    }

    pub fn from_u32(clr: u32) -> Color {
        Color::from_rgb8(((clr & 0xff0000) >> 16) as u8, ((clr & 0xff00) >> 8) as u8, (clr & 0xff) as u8)
    }
}

mod map {
    use iced::*;
    use crate::*;
    use std::collections::HashMap;
    use app::palette;

    #[derive(Debug,Clone)]
    pub struct Map {
        buttons: HashMap<usize, HashMap<usize, button::State>>
    }

    #[derive(Debug,Clone)]
    pub enum Msg {
        ViewDistrict(usize)
    }

    fn create_district_button<'a, Msg: Clone>(state: &'a mut button::State, label: &str, clr: Color) -> Button<'a, Msg> {
        Button::new(state, Text::new(label).horizontal_alignment(HorizontalAlignment::Center)).width(Length::Units(32)).height(Length::Units(32)).style(
            ButtonStyle(clr)
        )
    }

    struct ButtonStyle(Color);

    impl button::StyleSheet for ButtonStyle {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(self.0)),
                border_radius: 0.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                ..self.active()
            }
        }

        fn pressed(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(darken(self.0, 0.85))),
                ..self.active()
            }
        }
    }

    fn darken(clr: Color, factor: f32) -> Color {
        assert!(factor <= 1.0);
        Color {
            r: clr.r * factor,
            g: clr.g * factor,
            b: clr.b * factor,
            a: clr.a
        }
    }

    impl Map {
        pub fn new() -> Map {
            Map {
                buttons: HashMap::new()
            }
        }

        pub fn update(&mut self, event: Msg) {

        }

        pub fn view(&mut self, stage: &ElectionStage, result: Option<&ElectionResults>, district: Option<usize>)  -> Element<Msg> {
            let mut cnt = Column::new().spacing(16).padding(32);
            for i in 0..stage.areas.len() {
                self.buttons.insert(i, HashMap::new());
            }

            let mut views = Vec::new();
            for (&area, buttons) in self.buttons.iter_mut() {
                views.push((area, Self::view_area(stage, result, area, buttons)));
            }

            views.sort_by(|a, b| {
                a.0.cmp(&b.0)
            });

            for view in views {
                cnt = cnt.push(view.1);
            }

            cnt.into()
        }

        fn view_area<'a>(stage: &ElectionStage, result: Option<&ElectionResults>, area: usize, buttons: &'a mut HashMap<usize, button::State>) -> Element<'a, Msg> {
            let width = (stage.areas[area].districts.len() as f64).sqrt().ceil() as u16;
            let height = ((stage.areas[area].districts.len() as f64) / (width as f64)).ceil() as u16;

            let mut rows = Vec::new();
            for i in 0..height {
                rows.push(Row::new().spacing(8));
            }

            for (i,district) in stage.areas[area].districts.iter().enumerate() {
                let x = i as u16 % width;
                let y = i as u16 / width;
                buttons.entry(*district).or_default();
            }

            let mut buttons_refs = buttons.iter_mut().map(|(a, b)| (*a, b)).collect::<HashMap<usize, &mut button::State>>();

            for (i,district) in stage.areas[area].districts.iter().enumerate() {
                let x = i as u16 % width;
                let y = i as u16 / width;

                let state = buttons_refs.remove(district).unwrap();

                let mut row = std::mem::replace(rows.get_mut(y as usize).unwrap(), Row::new());

                let clr = if let Some(result) = result {
                    let result = &result.results[*district];
                    let party = result.votes.iter().reduce(|a, b| if a.1 > b.1 { a } else { b }).unwrap();
                    palette::from_u32(stage.parties[*party.0].color)
                } else {
                    palette::unknown()
                };

                row = row.push(create_district_button(state, &(i+1).to_string(), clr).on_press(Msg::ViewDistrict(*district)));
                rows[y as usize] = row;
            }

            let mut col = Column::new().spacing(8).width(Length::Fill).align_items(Align::Center);
            col = col.push(
                    Text::new(&stage.areas[area].name).horizontal_alignment(HorizontalAlignment::Center)
                );

            for row in rows {
                col = col.push(row);
            }
            col.into()
        }
    }
}