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

#![no_std]

//! A smartwatch watchface implementation
//!
//! This crate provides the `Watchface` struct which can contain watchface data. Then it provides
//! the `SimpleWatchfaceStyle` for drawing this data to a `embedded_graphics::DrawTarget`.
//!
//! # Current state
//! The current version of this crate is only able to draw the time and the style is ugly.
//!
//! # Examples
//!
//! ```
//! use chrono::Local;
//! use embedded_graphics::Drawable;
//! use embedded_graphics::mock_display::MockDisplay;
//! use embedded_graphics::pixelcolor::Rgb888;
//! use watchface::SimpleWatchfaceStyle;
//! use watchface::Watchface;
//!
//! let style = SimpleWatchfaceStyle::default();
//!
//! let styled_watchface = Watchface::build()
//!      .with_time(Local::now())
//!      .into_styled(style);
//!
//! let mut display = MockDisplay::<Rgb888>::new();
//! display.set_allow_out_of_bounds_drawing(true); //MockDisplay is too small for SimpleWatchfaceStyle
//! display.set_allow_overdraw(true);
//! styled_watchface.draw(&mut display);
//! ```
//!
//! # Simulator
//!
//! A simulator is available for testing a watchface on a desktop. Run the example using:
//! ```bash
//! cargo run --example simulator
//! ```

pub mod battery;
pub mod battery_icon;
mod font;
mod simple_watchface;
mod styled;
mod textual_time_watchface;
pub mod time;
mod watchface_data;

pub use simple_watchface::SimpleWatchfaceStyle;
pub use textual_time_watchface::TextualTimeWatchfaceStyle;
pub use watchface_data::Watchface;
pub use watchface_data::WatchfaceBuilder;
