/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2022  Nodle International
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

include!("src/cli.rs");

use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    //build_shell_completion();
    generate_cargo_keys();
    rerun_if_git_head_changed();
}

// fn build_shell_completion() {
//     for shell in Shell::value_variants() {
//         build_completion(shell);
//     }
// }

// /// Build the shell auto-completion for a given Shell
// fn build_completion(shell: &Shell) {
//     let outdir = match env::var_os("OUT_DIR") {
//         None => return, // undefined, skip
//         Some(dir) => dir,
//     };
//     let path = Path::new(&outdir)
//         .parent()
//         .unwrap()
//         .parent()
//         .unwrap()
//         .parent()
//         .unwrap()
//         .join("completion-scripts");

//     fs::create_dir(&path).ok();

//     let _ = generate_to(*shell, &mut Cli::command(), "nodle-chain", &path);
// }
