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
use crate::time::Time;
use crate::Watchface;
use core::fmt::Write;
use embedded_graphics::fonts::{Font24x32, Font8x16, Text};
use embedded_graphics::prelude::*;
use embedded_graphics::style::TextStyleBuilder;
use heapless::consts::*;
use heapless::String;

fn convert_hours_to_text(hours: u8) -> &'static str {
    match hours % 12 {
        0 => "twaalf",
        1 => "één",
        2 => "twee",
        3 => "drie",
        4 => "vier",
        5 => "vijf",
        6 => "zes",
        7 => "zeven",
        8 => "acht",
        9 => "negen",
        10 => "tien",
        11 => "elf",
        _ => "",
    }
}

fn convert_time_to_text(time: &Time) -> String<U20> {
    let mut text = String::<U20>::new();

    let rounded_time = time.round_to_quarters();
    if rounded_time.minutes_local() == 0 {
        write!(
            &mut text,
            "{}\nuur",
            convert_hours_to_text(rounded_time.hours_local())
        )
        .unwrap()
    } else if rounded_time.minutes_local() == 15 {
        write!(
            &mut text,
            "kwart\nover\n{}",
            convert_hours_to_text(rounded_time.hours_local())
        )
        .unwrap()
    } else if rounded_time.minutes_local() == 30 {
        write!(
            &mut text,
            "half\n{}",
            convert_hours_to_text(rounded_time.hours_local() + 1)
        )
        .unwrap()
    } else if rounded_time.minutes_local() == 45 {
        write!(
            &mut text,
            "kwart\nvoor\n{}",
            convert_hours_to_text(rounded_time.hours_local() + 1)
        )
        .unwrap()
    }

    text
}

#[derive(Default)]
pub struct TextualTimeWatchfaceStyle {}

impl<C> Drawable<C> for Styled<Watchface, TextualTimeWatchfaceStyle>
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

            let text = convert_time_to_text(time);

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
            write!(&mut text, "{:02}%", battery.percentage()).unwrap();

            Text::new(&text, Point::new(10, 10))
                .into_styled(time_text_style)
                .draw(display)?;
        }

        if let Some(charger) = &self.watchface.charger {
            let text = match charger {
                ChargerState::Discharging => "",
                ChargerState::Charging => "Charging",
                ChargerState::Full => "Full",
            };
            if text.len() > 0 {
                let time_text_style = TextStyleBuilder::new(Font8x16)
                    .text_color(C::WHITE)
                    .background_color(C::BLACK)
                    .build();

                Text::new(&text, Point::new(10, 30))
                    .into_styled(time_text_style)
                    .draw(display)?;
            }
        }

        Ok(())
    }
}
