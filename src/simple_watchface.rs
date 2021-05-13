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

use crate::battery_icon::{BatteryIconBuilder, ChargerAlignment};
use crate::font::OverpassNumbersFont;
use crate::styled::Styled;
use crate::watchface_data::Watchface;
use core::fmt::Write;
use embedded_graphics::fonts::Text;
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::DrawTarget;
use embedded_layout::prelude::*;
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

impl<C> Drawable<C> for Styled<Watchface, SimpleWatchfaceStyle>
where
    C: RgbColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), <D as DrawTarget<C>>::Error> {
        let display_area = display.display_area();

        display.clear(C::BLACK)?;

        if let Some(time) = &self.watchface.time {
            let time_text_style = TextStyleBuilder::new(OverpassNumbersFont {})
                .text_color(C::WHITE)
                .background_color(C::BLACK)
                .build();

            let mut text = String::<U8>::new();
            write!(
                &mut text,
                "{:02}:{:02}",
                time.hours_local(),
                time.minutes_local(),
            )
            .unwrap();

            Text::new(&text, Point::new(10, 70))
                .into_styled(time_text_style)
                .align_to(&display_area, horizontal::Center, vertical::Center)
                .draw(display)?
        }

        let mut icon_builder = BatteryIconBuilder::new(Point::new(10, 10));
        if let Some(battery) = &self.watchface.battery {
            icon_builder = icon_builder.with_state_of_charge(*battery);
        }
        if let Some(charger) = &self.watchface.charger {
            icon_builder = icon_builder.with_charger(*charger);
        }
        icon_builder
            .with_charger_alignment(ChargerAlignment::Left)
            .build()
            .align_to(&display_area, horizontal::Right, vertical::Top)
            .draw(display)?;

        Ok(())
    }
}
