use std::{fs::File, io::Write, thread, time::Duration};

use anyhow::Result;

pub struct OutputPin {
    number: u8,
}

impl Drop for OutputPin {
    fn drop(&mut self) {
        OutputPin::write(
            "/sys/class/gpio/unexport".to_string(),
            format!("{}", self.number).as_str(),
        )
        .unwrap();
    }
}

impl OutputPin {
    pub fn new(number: u8) -> Result<OutputPin> {
        OutputPin::write(
            "/sys/class/gpio/export".to_string(),
            format!("{}", number).as_str(),
        ).unwrap_or(()); // TODO: Proper signal handling for ctrl-c shutdown

        let path = format!("/sys/class/gpio/gpio{:}/direction", number);
        OutputPin::write(path, "out")?;

        Ok(OutputPin { number })
    }

    fn write(path: String, value: &str) -> Result<()> {
        let mut file = File::options().read(false).write(true).open(path)?;
        write!(file.by_ref(), "{}", value)?;
        thread::sleep(Duration::from_millis(50));
        Ok(())
    }

    fn pin_write(&self, value: &str) -> Result<()> {
        let path = format!("/sys/class/gpio/gpio{:}/value", self.number);
        OutputPin::write(path, value)
    }

    pub fn set_high(&self) -> Result<()> {
        self.pin_write("1")
    }

    pub fn set_low(&self) -> Result<()> {
        self.pin_write("0")
    }
}
