use anyhow::Result;
use std::io::{BufReader, Read};
use std::thread;
use std::time::Duration;

use rppal::gpio::{Gpio, OutputPin};
use std::net::{TcpListener, TcpStream};

const CRAP: [char; 3] = ['\n', '\0', '\t'];
const PIN: u8 = 21;

fn switch_for_ms(pin: &mut OutputPin, ms: u64) -> Result<()> {
    pin.set_low();
    thread::sleep(Duration::from_millis(ms));
    pin.set_high();
    thread::sleep(Duration::from_millis(50));
    Ok(())
}

fn remove_crap(s: &mut String) {
    let crap_count = s.chars().rev().take_while(|x| CRAP.contains(x)).count();
    let new_len = s.len().saturating_sub(crap_count);
    s.truncate(new_len);
}

fn handle_client(stream: &mut TcpStream, pin: &mut OutputPin) -> Result<()> {
    let mut reader = BufReader::new(stream);
    loop {
        let mut buff: [u8; 8] = [0; 8];
        reader.read(&mut buff)?;
        let mut ms_str = String::from_utf8(buff.to_vec())?;
        remove_crap(&mut ms_str);
        let ms: u64 = {
            match ms_str.parse() {
                Ok(t) => t,
                Err(_) => {
                    break;
                }
            }
        };
        switch_for_ms(pin, ms)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    let mut pin = Gpio::new()?.get(PIN)?.into_output_high();

    for stream in listener.incoming() {
        handle_client(&mut (stream?), &mut pin)?;
    }

    Ok(())
}
