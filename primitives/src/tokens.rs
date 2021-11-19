/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
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

use crate::CurrencyId;

// Native Token
pub const HKO: CurrencyId = 0;
pub const PARA: CurrencyId = 1;

// Polkadot ecosystem
pub const KSM: CurrencyId = 100;
pub const DOT: CurrencyId = 101;
pub const USDT: CurrencyId = 102;

// Liquid Staking Derivative
pub const XKSM: CurrencyId = 1000;
pub const XDOT: CurrencyId = 1001;

// Money Market Derivative
pub const PHKO: CurrencyId = 2000;
pub const PPARA: CurrencyId = 2001;
pub const PKSM: CurrencyId = 2100;
pub const PDOT: CurrencyId = 2101;
pub const PUSDT: CurrencyId = 2102;

pub const PXKSM: CurrencyId = 3000;
pub const PXDOT: CurrencyId = 3001;
pub const PCKSM: CurrencyId = 3100;
pub const PCDOT: CurrencyId = 3101;

// Crowdloans Derivative
pub const CKSM: CurrencyId = 4000;
pub const CDOT: CurrencyId = 4001;

// Token Registration Information
// +───────────+──────────────+────────────────────+
// | Network   | Token        | Register in block  |
// +───────────+──────────────+────────────────────+
// | Heiko     | HKO          | Native             |
// | Heiko     | PARA         | N/A                |
// | Heiko     | KSM          | N/A                |
// | Heiko     | DOT          | N/A                |
// | Heiko     | USDT         | N/A                |
// | Heiko     | XKSM         | N/A                |
// | Heiko     | XDOT         | N/A                |
// | Heiko     | PHKO         | N/A                |
// | Heiko     | PPARA        | N/A                |
// | Heiko     | PKSM         | N/A                |
// | Heiko     | PDOT         | N/A                |
// | Heiko     | PUSDT        | N/A                |
// | Heiko     | PXKSM        | N/A                |
// | Heiko     | PXDOT        | N/A                |
// | Parallel  | HKO          | N/A                |
// | Parallel  | PARA         | N/A                |
// | Parallel  | KSM          | N/A                |
// | Parallel  | DOT          | N/A                |
// | Parallel  | USDT         | N/A                |
// | Parallel  | XKSM         | N/A                |
// | Parallel  | XDOT         | N/A                |
// | Parallel  | PHKO         | N/A                |
// | Parallel  | PPARA        | N/A                |
// | Parallel  | PKSM         | N/A                |
// | Parallel  | PDOT         | N/A                |
// | Parallel  | PUSDT        | N/A                |
// | Parallel  | PXKSM        | N/A                |
// | Parallel  | PXDOT        | N/A                |
// | Parallel  | CKSM         | N/A                |
// | Parallel  | CDOT         | N/A                |
// +──────────+───────────────+────────────────────+
