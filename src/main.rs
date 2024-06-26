mod gui;

use std::{thread, time::Duration};

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

enum Animation {
    TV,
    UpToDown,
    LeftToRight,
}

struct ADCVoltage {
    ratio: (u16, u16),
    res: ResBit,
    input: u16,
}
trait OutVolt {
    fn output_voltage(&self, max_adc_volt: f32) -> f32;
}
impl ADCVoltage {
    pub fn update_input(&mut self, new_value: u16) {
        self.input = new_value
    }
}

impl OutVolt for ADCVoltage {
    fn output_voltage(&self, max_adc_volt: f32) -> f32 {
        use ResBit::{Bit10, Bit12, Bit14, Bit16, Bit8};
        match self.res {
            Bit16 => {
                max_adc_volt
                    * (self.input as f32 / (2_f32.powi(16) - 1.0))
                    * (1.0 + (self.ratio.1 as f32 / self.ratio.0 as f32))
            }
            Bit14 => {
                max_adc_volt
                    * (self.input as f32 / (2_f32.powi(14) - 1.0))
                    * (1.0 + (self.ratio.1 as f32 / self.ratio.0 as f32))
            }
            Bit12 => {
                max_adc_volt
                    * (self.input as f32 / (2_f32.powi(12) - 1.0))
                    * (1.0 + (self.ratio.1 as f32 / self.ratio.0 as f32))
            }
            Bit10 => {
                max_adc_volt
                    * (self.input as f32 / (2_f32.powi(10) - 1.0))
                    * (1.0 + (self.ratio.1 as f32 / self.ratio.0 as f32))
            }
            Bit8 => {
                max_adc_volt
                    * (self.input as f32 / (2_f32.powi(8) - 1.0))
                    * (1.0 + (self.ratio.1 as f32 / self.ratio.0 as f32))
            }
        }
    }
}

enum ResBit {
    Bit8,
    Bit10,
    Bit12,
    Bit14,
    Bit16,
}

enum WidgetList<'a> {
    Battery(f32, f32, &'a dyn OutVolt),
}

struct Widget<'a, C: PixelColor> {
    widget: WidgetList<'a>,
    color: C,
    size: Size,
    position: Point,
}
impl<'a, C: PixelColor> Widget<'a, C> {
    pub fn delta_change(&mut self, x: i32, y: i32) {
        self.position.x = self.position.x + x;
        self.position.y = self.position.y + y;
    }
}

struct Screen {
    power: bool,
    animation: Animation,
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));
    let adcconv = ADCVoltage {
        ratio: (1, 2),
        res: ResBit::Bit10,
        input: 255,
    };
    let mut battery = Widget {
        widget: WidgetList::Battery(3.2, 4.2, &adcconv),
        color: BinaryColor::On,
        size: Size::new(36, 60),
        position: Point::new(0, 8),
    };

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Hello World", &output_settings);
    'running: loop {
        window.update(&mut display);
        thread::sleep(Duration::from_millis(30));
        display.clear(BinaryColor::Off)?;
        // battery.delta_change(1, 1);

        battery.draw(&mut display)?;
        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
    }
}
