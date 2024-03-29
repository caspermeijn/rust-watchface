/* Copyright (C) 2020 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use chrono::{Local, Timelike};

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::thread;
use std::time::Duration;
use watchface::battery::{ChargerState, StateOfCharge};
use watchface::{SimpleWatchfaceStyle, Watchface};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Simple Watchface", &output_settings);

    'running: loop {
        let seconds = Local::now().second() as u8;
        let (battery_percentage, charger_state) = if seconds < 25 {
            (100 - (seconds * 4), ChargerState::Discharging)
        } else if seconds < 30 {
            (0, ChargerState::Discharging)
        } else if seconds < 55 {
            (100 - (55 - seconds) * 4, ChargerState::Charging)
        } else {
            (100, ChargerState::Full)
        };

        let style = SimpleWatchfaceStyle::default();

        let watchface = Watchface::build()
            .with_time(Local::now())
            .with_battery(StateOfCharge::from_percentage(battery_percentage))
            .with_charger(charger_state)
            .into_styled(style);

        watchface.draw(&mut display)?;
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
