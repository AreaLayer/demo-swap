use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Length, Settings, Text,
    TextInput, text_input,
};
use iced::window;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct SwapRequest {
    address: String,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SwapResponse {
    id: String,
    timeout_block_height: u64,
    on_chain_address: String,
    lockup_address: String,
    redeem_script: String,
}

#[derive(Default)]
struct SwapApp {
    address_input: text_input::State,
    currency_input: text_input::State,
    address_value: String,
    currency_value: String,
    swap_result: Option<SwapResponse>,
    submit_button: button::State,
}

#[derive(Debug, Clone)]
enum Message {
    AddressChanged(String),
    CurrencyChanged(String),
    Submit,
    SwapCompleted(Result<SwapResponse, String>),
}

impl Application for SwapApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (SwapApp, Command<Message>) {
        (SwapApp::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Mini Swap Demo")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddressChanged(address) => {
                self.address_value = address;
            }
            Message::CurrencyChanged(currency) => {
                self.currency_value = currency;
            }
            Message::Submit => {
                let address = self.address_value.clone();
                let currency = self.currency_value.clone();
                return Command::perform(create_swap(address, currency), Message::SwapCompleted);
            }
            Message::SwapCompleted(result) => {
                match result {
                    Ok(response) => self.swap_result = Some(response),
                    Err(error) => println!("Swap failed: {}", error),
                }
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let input_address = TextInput::new(
            &mut self.address_input,
            "Enter BTC address...",
            &self.address_value,
            Message::AddressChanged,
        )
        .padding(10)
        .size(20);

        let input_currency = TextInput::new(
            &mut self.currency_input,
            "Enter currency (BTC)...",
            &self.currency_value,
            Message::CurrencyChanged,
        )
        .padding(10)
        .size(20);

        let submit_button = Button::new(&mut self.submit_button, Text::new("Submit Swap"))
            .on_press(Message::Submit)
            .padding(10);

        let mut content = Column::new()
            .align_items(Align::Center)
            .padding(20)
            .spacing(20)
            .push(input_address)
            .push(input_currency)
            .push(submit_button);

        if let Some(swap) = &self.swap_result {
            content = content.push(Text::new(format!("Swap created! ID: {}", swap.id)));
        }

        content.into()
    }
}

async fn create_swap(address: String, currency: String) -> Result<SwapResponse, String> {
    let client = Client::new();
    let swap_request = SwapRequest { address, currency };

    match client
        .post("https://api.boltz.exchange/api/swap")
        .json(&swap_request)
        .send()
        .await
    {
        Ok(response) => match response.json::<SwapResponse>().await {
            Ok(swap_response) => Ok(swap_response),
            Err(_) => Err(String::from("Failed to parse response")),
        },
        Err(_) => Err(String::from("Request failed")),
    }
}

fn main() -> iced::Result {
    SwapApp::run(Settings {
        window: window::Settings {
            size: (400, 300),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
