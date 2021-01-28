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

use crate::battery_icon::BatteryIconBuilder;
use crate::styled::Styled;
use crate::watchface_data::Watchface;
use core::fmt::Write;
use core::marker::PhantomData;
use embedded_graphics::fonts::{Font24x32, Text};
use embedded_graphics::prelude::*;
use embedded_graphics::style::MonoTextStyleBuilder;
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
pub struct SimpleWatchfaceStyle<C> {
    _phantom_data: PhantomData<C>,
}

impl<C> Drawable for Styled<Watchface, SimpleWatchfaceStyle<C>>
where
    C: RgbColor,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.clear(C::BLACK)?;

        if let Some(time) = &self.watchface.time {
            let time_text_style = MonoTextStyleBuilder::new(Font24x32)
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
            if let Some(charger) = &self.watchface.charger {
                BatteryIconBuilder::new(Point::new(10, 10))
                    .with_charger(*charger)
                    .with_state_of_charge(*battery)
                    .build()
                    .draw(display)?;
            }
        }

        Ok(())
    }
}
