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

use crate::simple_watchface::SimpleWatchfaceStyle;
use crate::styled::Styled;
use crate::time::Time;

#[derive(Default)]
pub struct Watchface {
    pub time: Option<Time>,
}

impl Watchface {
    pub fn build() -> WatchfaceBuilder {
        WatchfaceBuilder::default()
    }

    pub fn into_styled(self, style: SimpleWatchfaceStyle) -> Styled<Self, SimpleWatchfaceStyle> {
        Styled::new(self, style)
    }
}

#[derive(Default)]
pub struct WatchfaceBuilder {
    watchface: Watchface,
}

impl WatchfaceBuilder {
    pub fn with_time<T: Into<Time>>(mut self, time: T) -> Self {
        self.watchface.time = Some(time.into());

        self
    }

    pub fn finish(self) -> Watchface {
        self.watchface
    }

    pub fn into_styled(
        self,
        style: SimpleWatchfaceStyle,
    ) -> Styled<Watchface, SimpleWatchfaceStyle> {
        self.watchface.into_styled(style)
    }
}
