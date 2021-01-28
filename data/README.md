<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: © 2020 Casper Meijn <casper@meijn.net>

This work is licensed under the Creative Commons Attribution-ShareAlike 4.0 International License. 
To view a copy of this license, visit http://creativecommons.org/licenses/by-sa/4.0/ or 
  send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

# Numbers font

The internal OverpassNumbersFont is generated from `overpass-numbers.xcf`. This contains 
text using the provided font. `overpass-numbers.png` is an export of this file.
`overpass-numbers.raw` is generated using `convert overpass-numbers.png -depth 1 gray:overpass-numbers.raw`.

`../src/font.rs` contains a mapping for each character and the size of the 
characters.