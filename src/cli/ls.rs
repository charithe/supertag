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

use crate::common::err::STagResult;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const TREE_MIDDLE: &str = "├──";
const TREE_END: &str = "└──";

pub fn ls(conn: &mut Connection, paths: Vec<&Path>) -> STagResult<()> {
    for path in paths {
        println!("{}:", path.display());

        let canon_path = path.canonicalize()?;
        let tagged_files = crate::sql::file_tags_in_path(conn, canon_path.to_str().unwrap())?;

        if canon_path.is_dir() {
            let mut entries = fs::read_dir(canon_path)?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?;
            entries.sort();

            let mut files = entries.iter().peekable();

            while let Some(file) = files.next() {
                if files.peek().is_some() {
                    print_file_tags(&tagged_files, TREE_MIDDLE, file);
                } else {
                    print_file_tags(&tagged_files, TREE_END, file);
                }
            }
        } else {
            print_file_tags(&tagged_files, TREE_END, path);
        }
    }

    Ok(())
}

fn print_file_tags(tagged_files: &HashMap<String, Vec<String>>, prefix: &str, path: &Path) {
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if let Some(tags) = tagged_files.get(path.to_str().unwrap()) {
        println!("{} {}\t->\t{}", prefix, file_name, tags.join(","));
    } else {
        println!("{} {}", prefix, file_name);
    }
}

/*
fn generate_tag_colors(tagged_files: &HashMap<String, Vec<String>>) -> HashMap<String, u16> {
    let mut unique_tags = HashSet::new();
    tagged_files.for_each(|(k,v)| v.iter().for_each(|t| unique_tags.insert(t)));

}
*/
