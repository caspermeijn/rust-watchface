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

/// Indicated the state of the charger of a battery
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ChargerState {
    /// No power source is connected, therefore the battery is discharging
    Discharging,
    /// A power source is connected, therefore the battery is charging
    Charging,
    /// A power source is connected, but the battery is fully charged
    Full,
}

/// Indicates a level of charge of a battery
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct StateOfCharge {
    level: u8,
}

impl StateOfCharge {
    /// Create an instance from a percentage (0..100)
    pub fn from_percentage(percentage: u8) -> Self {
        assert!(percentage <= 100);
        let percentage = percentage as u32;
        let level = percentage * 255 / 100;
        Self::from_level(level as u8)
    }

    /// Create an instance from a level (0..255)
    pub fn from_level(level: u8) -> Self {
        Self { level }
    }

    /// Get the level (0..255)
    pub fn level(&self) -> u8 {
        self.level
    }

    /// Get the percentage (0..100)
    pub fn percentage(&self) -> u8 {
        let level = self.level as u32;
        let percentage = level * 100 / 255;
        percentage as u8
    }
}
