use iced::widget::{center, column, container, rule, scrollable, table, text};
use iced::{Center, Element, Shrink, Task, Theme};
use polars::prelude::*;

use polygon::rest::tickers;

pub fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("polygon • iced example")
        .theme(Theme::CatppuccinMocha)
        .centered()
        .run()
}

struct App {
    client: Arc<polygon::Polygon>,
    data: Option<DataFrame>,
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    Loaded(polygon::Result<DataFrame>),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let key = std::env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY");
        let client = polygon::Polygon::default().with_key(key);

        let app = Self {
            client: Arc::new(client),
            data: None,
        };

        (app, Task::done(Message::Load))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Load => {
                let client = self.client.clone();
                Task::future(async move {
                    tickers::all(&client).limit(100).as_dataframe().get().await
                })
                .map(Message::Loaded)
            }
            Message::Loaded(result) => {
                match result {
                    Ok(df) => self.data = Some(df),
                    Err(e) => eprintln!("Error loading data: {e}"),
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = container(text("polygon.io").size(13).style(text::primary))
            .padding([10, 5]);

        match &self.data {
            None => column![title, center(fragment("Loading..."))]
                .padding(10)
                .into(),

            Some(data) => column![
                title,
                center(scrollable(dataframe(data)).spacing(10).direction(
                    scrollable::Direction::Both {
                        vertical: scrollable::Scrollbar::default(),
                        horizontal: scrollable::Scrollbar::default(),
                    }
                ))
            ]
            .padding(10)
            .into(),
        }
    }
}

fn dataframe<'a>(df: &'a DataFrame) -> Element<'a, Message> {
    let columns = df.get_columns();

    let table_columns: Vec<_> = columns
        .iter()
        .map(|col| {
            let col_name = col.name().to_uppercase();
            let series = col.as_materialized_series();
            table::column(header(col_name), move |i| cell(&series, i))
                .align_x(Center)
                .align_y(Center)
        })
        .collect();

    container(
        column![
            container(title("/tickers/all")).padding([5, 10]),
            rule::horizontal(1).style(rule::weak),
            table(table_columns, 0..df.height())
                .padding_x(10)
                .padding_y(5)
                .separator_x(1)
                .separator_y(1),
        ]
        .width(Shrink),
    )
    .style(container::bordered_box)
    .into()
}

/// Format a cell value from a DataFrame column at a specific row index
fn cell<'a>(series: &'a Series, i: usize) -> Option<Element<'a, Message>> {
    match series.dtype() {
        DataType::Boolean => {
            series.bool().ok().and_then(|ca| ca.get(i)).map(active)
        }
        DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64 => series
            .i64()
            .ok()
            .and_then(|ca| ca.get(i))
            .map(|v| fragment(format!("{v}"))),
        DataType::UInt8
        | DataType::UInt16
        | DataType::UInt32
        | DataType::UInt64 => series
            .u64()
            .ok()
            .and_then(|ca| ca.get(i))
            .map(|v| fragment(format!("{v}"))),
        DataType::Float32 | DataType::Float64 => series
            .f64()
            .ok()
            .and_then(|ca| ca.get(i))
            .map(|v| fragment(format!("{:.2}", v))),
        DataType::String => {
            series.str().ok().and_then(|ca| ca.get(i)).map(fragment)
        }
        _ => series.get(i).ok().map(|v| fragment(v.to_string())),
    }
}

fn title<'a>(label: &'a str) -> Element<'a, Message> {
    text(label.to_uppercase())
        .font(iced::Font::MONOSPACE)
        .size(11)
        .style(text::primary)
        .into()
}

fn header<'a>(label: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    text(label).size(12).style(text::secondary).into()
}

fn active<'a>(value: bool) -> Element<'a, Message> {
    if value {
        text("✓").style(text::success).into()
    } else {
        text("✗").style(text::secondary).into()
    }
}

fn fragment<'a>(value: impl text::IntoFragment<'a>) -> Element<'a, Message> {
    text(value).size(12).into()
}
