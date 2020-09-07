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

/// Simple representation of time
///
/// This is a simplified representation of time, so that it can also be used in applications without
/// a full operating system or without chrono dependency.
#[derive(Default, Eq, PartialEq, Debug)]
pub struct Time {
    hours_local: u8,
    minutes_local: u8,
    seconds_local: u8,
}

impl Time {
    /// Return a time from a unix epoch and timezone offset
    ///
    /// # Arguments
    /// * `epoch` - The number of seconds since 1970-01-01T00:00:00
    /// * `timezone_offset` - The number of seconds timezone offset
    ///
    /// # Examples
    /// ```
    /// use watchface::Time;
    /// // GMT+02:00
    /// let timezone_offset = 2 * 60 * 60;
    /// // 2020-09-03T19:23:02
    /// let time = Time::from_unix_epoch(1599160982, timezone_offset);
    /// assert_eq!(time.hours_local(), 21);
    /// assert_eq!(time.minutes_local(), 23);
    /// assert_eq!(time.seconds_local(), 02);
    /// ```
    pub fn from_unix_epoch(epoch: u64, timezone_offset: u64) -> Self {
        let local_epoch = epoch + timezone_offset;
        Time {
            hours_local: (local_epoch / 60 / 60 % 24) as u8,
            minutes_local: (local_epoch / 60 % 60) as u8,
            seconds_local: (local_epoch % 60) as u8,
        }
    }

    /// Get hours in local timezone
    pub fn hours_local(&self) -> u8 {
        self.hours_local
    }

    /// Get minutes in local timezone
    pub fn minutes_local(&self) -> u8 {
        self.minutes_local
    }

    /// Get seconds in local timezone
    pub fn seconds_local(&self) -> u8 {
        self.seconds_local
    }
}

/// Create a time from a chrono DateTime
impl From<DateTime<Local>> for Time {
    fn from(time: DateTime<Local>) -> Self {
        Time {
            hours_local: time.hour() as u8,
            minutes_local: time.minute() as u8,
            seconds_local: time.second() as u8,
        }
    }
}
