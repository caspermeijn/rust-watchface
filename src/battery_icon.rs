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

use crate::battery::{ChargerState, StateOfCharge};
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Rectangle, Triangle};
use embedded_graphics::style::PrimitiveStyleBuilder;

pub enum ChargerAlignment {
    Left,
    Right,
}

impl ChargerAlignment {
    fn battery_offset(&self) -> Point {
        match self {
            ChargerAlignment::Left => Point::new(15, 0),
            ChargerAlignment::Right => Point::new(0, 0),
        }
    }

    fn charger_offset(&self) -> Point {
        match self {
            ChargerAlignment::Left => Point::new(0, 0),
            ChargerAlignment::Right => Point::new(15, 0),
        }
    }
}

pub struct BatteryIcon {
    position: Point,
    state_of_charge: Option<StateOfCharge>,
    charger: Option<ChargerState>,
    charger_alignment: ChargerAlignment,
}

impl<C> Drawable<C> for BatteryIcon
where
    C: RgbColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), <D as DrawTarget<C>>::Error> {
        if let Some(state_of_charge) = self.state_of_charge {
            let offset = self.position + self.charger_alignment.battery_offset();

            let percentage = state_of_charge.percentage();

            let border_color = if percentage > 10 { C::WHITE } else { C::RED };

            let border_style = PrimitiveStyleBuilder::new()
                .stroke_width(2)
                .stroke_color(border_color)
                .build();

            Rectangle::new(Point::new(0, 5) + offset, Point::new(14, 24) + offset)
                .into_styled(border_style)
                .draw(display)?;

            let fill_style = PrimitiveStyleBuilder::new()
                .fill_color(border_color)
                .build();

            Rectangle::new(Point::new(3, 0) + offset, Point::new(11, 6) + offset)
                .into_styled(fill_style)
                .draw(display)?;

            let black_fill_style = PrimitiveStyleBuilder::new().fill_color(C::BLACK).build();

            Rectangle::new(Point::new(5, 2) + offset, Point::new(9, 6) + offset)
                .into_styled(black_fill_style)
                .draw(display)?;

            if percentage > 10 {
                let color = if percentage > 25 { C::WHITE } else { C::RED };

                let fill_style = PrimitiveStyleBuilder::new().fill_color(color).build();

                Rectangle::new(Point::new(3, 18) + offset, Point::new(11, 21) + offset)
                    .into_styled(fill_style)
                    .draw(display)?;
            }

            if percentage > 50 {
                let white_fill_style = PrimitiveStyleBuilder::new().fill_color(C::WHITE).build();

                Rectangle::new(Point::new(3, 13) + offset, Point::new(11, 16) + offset)
                    .into_styled(white_fill_style)
                    .draw(display)?;
            }

            if percentage > 75 {
                let white_fill_style = PrimitiveStyleBuilder::new().fill_color(C::WHITE).build();

                Rectangle::new(Point::new(3, 8) + offset, Point::new(11, 11) + offset)
                    .into_styled(white_fill_style)
                    .draw(display)?;
            }

            if percentage > 90 {
                let white_fill_style = PrimitiveStyleBuilder::new().fill_color(C::WHITE).build();

                Rectangle::new(Point::new(6, 3) + offset, Point::new(8, 6) + offset)
                    .into_styled(white_fill_style)
                    .draw(display)?;
            }
        }

        if let Some(charger) = self.charger {
            if charger == ChargerState::Charging || charger == ChargerState::Full {
                let offset = self.position + self.charger_alignment.charger_offset();

                let color = if charger == ChargerState::Charging {
                    C::RED
                } else {
                    C::GREEN
                };

                let yellow_fill_style = PrimitiveStyleBuilder::new().fill_color(color).build();

                Triangle::new(
                    Point::new(8, 1) + offset,
                    Point::new(8, 14) + offset,
                    Point::new(2, 14) + offset,
                )
                .into_styled(yellow_fill_style)
                .draw(display)?;

                let yellow_fill_style = PrimitiveStyleBuilder::new().fill_color(color).build();

                Triangle::new(
                    Point::new(7, 10) + offset,
                    Point::new(13, 10) + offset,
                    Point::new(7, 23) + offset,
                )
                .into_styled(yellow_fill_style)
                .draw(display)?;
            }
        }

        Ok(())
    }
}

pub struct BatteryIconBuilder {
    battery_icon: BatteryIcon,
}

impl BatteryIconBuilder {
    pub fn new(position: Point) -> Self {
        Self {
            battery_icon: BatteryIcon {
                position,
                state_of_charge: None,
                charger: None,
                charger_alignment: ChargerAlignment::Right,
            },
        }
    }

    pub fn with_state_of_charge(mut self, state_of_charge: StateOfCharge) -> Self {
        self.battery_icon.state_of_charge = Some(state_of_charge);

        self
    }

    pub fn with_charger(mut self, charger: ChargerState) -> Self {
        self.battery_icon.charger = Some(charger);

        self
    }

    pub fn with_charger_alignment(mut self, charger_alignment: ChargerAlignment) -> Self {
        self.battery_icon.charger_alignment = charger_alignment;

        self
    }

    pub fn build(self) -> BatteryIcon {
        self.battery_icon
    }
}
