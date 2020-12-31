/*
 * Supertag
 * Copyright (C) 2020 Andrew Moffat
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use clap::{Arg, SubCommand};

pub(super) fn add_subcommands<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
    app.subcommand(
        SubCommand::with_name("ls")
            .about("Lists tags attached to files in the directory")
            .arg(
                Arg::with_name("collection")
                    .help("Supertag collection name, eg 'media_files'.  This will be the name of our mounted drive.")
                    .short("c")
                    .long("collection")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("path")
                    .required(true)
                    .help("The path(s) to list")
                    .min_values(1)
                    .index(1)
                    .takes_value(true),
            ),
    )
}
