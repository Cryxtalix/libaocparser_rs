/* This file is part of libaocparser_rs.

libaocparser_rs is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

libaocparser_rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with MMWP. If not, see <https://www.gnu.org/licenses/>. */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Error: Index out of bounds.")]
    OutOfBounds,

    #[error("Error: Cannot parse string into type")]
    ParseToTypeFailed,
}
