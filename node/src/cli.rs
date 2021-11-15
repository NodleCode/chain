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

use sc_cli::{KeySubcommand, SignCmd, VanityCmd, VerifyCmd};
use std::{fmt, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, Clone)]
pub struct RuntimeInstanceError(String);

impl fmt::Display for RuntimeInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let RuntimeInstanceError(message) = self;
        write!(f, "RuntimeInstanceError: {}", message)
    }
}

#[derive(Debug, StructOpt)]
pub enum RuntimeInstance {
    Main,
    Staking,
}

impl RuntimeInstance {
    fn variants() -> [&'static str; 2] {
        ["main", "staking"]
    }

    pub fn is_staking_runtime(&self) -> bool {
        match self {
            Self::Main => false,
            Self::Staking => true,
        }
    }
}

impl fmt::Display for RuntimeInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Main => write!(f, "main"),
            Self::Staking => write!(f, "staking"),
        }
    }
}

impl Default for RuntimeInstance {
    fn default() -> Self {
        RuntimeInstance::Main
    }
}

impl FromStr for RuntimeInstance {
    type Err = RuntimeInstanceError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input_lower = input.to_lowercase();
        match input_lower.as_str() {
            "staking" => Ok(RuntimeInstance::Staking),
            "main" | "" => Ok(RuntimeInstance::Main),
            other => Err(RuntimeInstanceError(format!(
                "Invalid variant: `{}`",
                other
            ))),
        }
    }
}

/// An overarching CLI command definition.
#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Possible subcommand with parameters.
    #[structopt(subcommand)]
    pub subcommand: Option<Subcommand>,
    #[allow(missing_docs)]
    #[structopt(flatten)]
    pub run: RunCmd,
}

#[derive(Debug, StructOpt)]
pub struct RunCmd {
    #[structopt(flatten)]
    pub base: sc_cli::RunCmd,

    /// Specify the runtime used by the node.
    #[structopt(default_value, long, possible_values = &RuntimeInstance::variants(), case_insensitive = true)]
    pub runtime: RuntimeInstance,
}

/// Possible subcommands of the main binary.
#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Key management cli utilities
    Key(KeySubcommand),

    /// The custom benchmark subcommmand benchmarking runtime pallets.
    #[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
    Verify(VerifyCmd),

    /// Generate a seed that provides a vanity address.
    Vanity(VanityCmd),

    /// Sign a message, with a given (secret) key.
    Sign(SignCmd),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),
}
