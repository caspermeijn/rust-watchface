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

use chrono::prelude::*;

#[derive(Default)]
pub struct Time {
    hours_local: u8,
    minutes_local: u8,
    seconds_local: u8,
}

impl Time {
    pub fn from_unix_epoch(epoch: u64, timezone_offset: u64) -> Self {
        let local_epoch = epoch + timezone_offset;
        Time {
            hours_local: (local_epoch / 60 / 60 % 24) as u8,
            minutes_local: (local_epoch / 60 % 60) as u8,
            seconds_local: (local_epoch % 60) as u8,
        }
    }

    pub fn hours_local(&self) -> u8 {
        self.hours_local
    }

    pub fn minutes_local(&self) -> u8 {
        self.minutes_local
    }

    pub fn seconds_local(&self) -> u8 {
        self.seconds_local
    }
}

impl From<DateTime<Local>> for Time {
    fn from(time: DateTime<Local>) -> Self {
        Time {
            hours_local: time.hour() as u8,
            minutes_local: time.minute() as u8,
            seconds_local: time.second() as u8,
        }
    }
}
