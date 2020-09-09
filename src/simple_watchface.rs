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

use crate::battery::ChargerState;
use crate::styled::Styled;
use crate::watchface_data::Watchface;
use core::fmt::Write;
use embedded_graphics::fonts::{Font24x32, Text, Font8x16};
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::DrawTarget;
use heapless::consts::*;
use heapless::String;

/// Simple watchface style
///
/// This implements a simple watchface style, all watchface data will just be drawn as text on the
/// screen.
///
/// # Examples
///
/// ```
/// use chrono::Local;
/// use embedded_graphics::drawable::Drawable;
/// use embedded_graphics::mock_display::MockDisplay;
/// use embedded_graphics::pixelcolor::Rgb888;
/// use watchface::battery::ChargerState;
/// use watchface::battery::StateOfCharge;
/// use watchface::SimpleWatchfaceStyle;
/// use watchface::Watchface;
///
/// let style = SimpleWatchfaceStyle::default();
///
/// let styled_watchface = Watchface::build()
///      .with_time(Local::now())
///      .with_battery(StateOfCharge::from_percentage(100))
///      .with_charger(ChargerState::Full)
///      .into_styled(style);
///
/// let mut display = MockDisplay::<Rgb888>::new();
/// styled_watchface.draw(&mut display);
/// ```
#[derive(Default)]
pub struct SimpleWatchfaceStyle {}

impl<C> Drawable<C> for &Styled<Watchface, SimpleWatchfaceStyle>
where
    C: RgbColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), <D as DrawTarget<C>>::Error> {

        display.clear(C::BLACK)?;

        if let Some(time) = &self.watchface.time {
            let time_text_style = TextStyleBuilder::new(Font24x32)
                .text_color(C::WHITE)
                .background_color(C::BLACK)
                .build();

            let mut text = String::<U8>::new();
            write!(
                &mut text,
                "{:02}:{:02}:{:02}",
                time.hours_local(),
                time.minutes_local(),
                time.seconds_local()
            )
            .unwrap();

            Text::new(&text, Point::new(10, 70))
                .into_styled(time_text_style)
                .draw(display)?;
        }

        if let Some(battery) = &self.watchface.battery {
            let time_text_style = TextStyleBuilder::new(Font8x16)
                .text_color(C::WHITE)
                .background_color(C::BLACK)
                .build();

            let mut text = String::<U12>::new();
            write!(
                &mut text,
                "batt: {:02}%",
                battery.percentage()
            )
                .unwrap();

            Text::new(&text, Point::new(150, 10))
                .into_styled(time_text_style)
                .draw(display)?;
        }

        if let Some(charger) = &self.watchface.charger {
            let text = match charger {
                ChargerState::Discharging => { "" },
                ChargerState::Charging => { "Charging" },
                ChargerState::Full => { "Full" },
            };
            if text.len() > 0 {
                let time_text_style = TextStyleBuilder::new(Font8x16)
                    .text_color(C::WHITE)
                    .background_color(C::BLACK)
                    .build();

                Text::new(&text, Point::new(150, 30))
                    .into_styled(time_text_style)
                    .draw(display)?;
            }
        }

        Ok(())
    }
}
