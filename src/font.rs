/* SPDX-FileCopyrightText: © 2020 Casper Meijn <casper@meijn.net>
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

use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::GlyphMapping, DecorationDimensions, MonoFont},
};

const CHARS_PER_ROW: u32 = 16;
const CHAR_WIDTH: u32 = 40;
const CHAR_HEIGHT: u32 = 52;

pub const OVERPASS_NUMBERS_FONT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/overpass-numbers.raw"),
        CHARS_PER_ROW * CHAR_WIDTH,
    ),
    glyph_mapping: &ClockDigitMapping {},
    character_size: Size::new(CHAR_WIDTH, CHAR_HEIGHT),
    character_spacing: 0,
    baseline: 50,
    underline: DecorationDimensions::new(51, 1),
    strikethrough: DecorationDimensions::new(26, 1),
};

struct ClockDigitMapping {}

impl GlyphMapping for ClockDigitMapping {
    fn index(&self, c: char) -> usize {
        if c >= '0' || c <= '9' {
            c as usize - '0' as usize
        } else if c == ':' {
            10
        } else {
            15
        }
    }
}
