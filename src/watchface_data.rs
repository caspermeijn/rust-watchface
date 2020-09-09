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
use crate::battery::StateOfCharge;
use crate::simple_watchface::SimpleWatchfaceStyle;
use crate::styled::Styled;
use crate::time::Time;

/// Representation of watchface data
#[derive(Default)]
pub struct Watchface {
    pub time: Option<Time>,
    pub charger: Option<ChargerState>,
    pub battery: Option<StateOfCharge>,
}

impl Watchface {
    /// Returns a watchface builder
    pub fn build() -> WatchfaceBuilder {
        WatchfaceBuilder::default()
    }

    /// Convert to a styled watchface
    pub fn into_styled(self, style: SimpleWatchfaceStyle) -> Styled<Self, SimpleWatchfaceStyle> {
        Styled::new(self, style)
    }
}

/// Builder for creating watchface data
///
/// # Examples
///
/// ```
/// use watchface::battery::ChargerState;
/// use watchface::battery::StateOfCharge;
/// use watchface::time::Time;
/// use watchface::Watchface;
///
/// let watchface = Watchface::build()
///      .with_time(Time::from_unix_epoch(1599160982, 0))
///      .with_battery(StateOfCharge::from_percentage(50))
///      .with_charger(ChargerState::Discharging)
///      .finish();
///
/// assert_eq!(watchface.time, Some(Time::from_unix_epoch(1599160982, 0)));
/// ```
#[derive(Default)]
pub struct WatchfaceBuilder {
    watchface: Watchface,
}

impl WatchfaceBuilder {
    /// Add a time to the watchface data
    pub fn with_time<T: Into<Time>>(mut self, time: T) -> Self {
        self.watchface.time = Some(time.into());

        self
    }

    /// Add a charger state to the watchface data
    pub fn with_charger<T: Into<ChargerState>>(mut self, charger: T) -> Self {
        self.watchface.charger = Some(charger.into());

        self
    }

    /// Add a battery state of charge to the watchface data
    pub fn with_battery<T: Into<StateOfCharge>>(mut self, battery: T) -> Self {
        self.watchface.battery = Some(battery.into());

        self
    }

    /// Convert the builder to a watchface
    pub fn finish(self) -> Watchface {
        self.watchface
    }

    /// Convert the builder to a styled watchface
    ///
    /// # Examples
    /// ```
    /// use chrono::Local;
    /// use embedded_graphics::drawable::Drawable;
    /// use embedded_graphics::mock_display::MockDisplay;
    /// use embedded_graphics::pixelcolor::Rgb888;
    /// use watchface::SimpleWatchfaceStyle;
    /// use watchface::Watchface;
    ///
    /// let style = SimpleWatchfaceStyle::default();
    ///
    /// let watchface = Watchface::build()
    ///      .with_time(Local::now())
    ///      .into_styled(style);
    ///
    /// let mut display = MockDisplay::<Rgb888>::new();
    /// watchface.draw(&mut display);
    /// ```
    pub fn into_styled(
        self,
        style: SimpleWatchfaceStyle,
    ) -> Styled<Watchface, SimpleWatchfaceStyle> {
        self.watchface.into_styled(style)
    }
}
