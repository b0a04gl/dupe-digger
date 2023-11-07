

use std::path::PathBuf;
use std::sync::mpsc::Receiver;

pub fn collect_and_print_results(
    rx: mpsc::Receiver<(u64, Vec<PathBuf>)>,
    singleline: bool,
    nul: bool,
    grandtotal: bool,
) {
    let mut total_dupes = 0;
    let mut total_files = 0;
    let mut total_size = 0;

    while let Ok((size, entries)) = rx.recv() {
        total_dupes += 1;
        total_files += entries.len() - 1;
        total_size += size * (entries.len() - 1) as u64;

        if singleline {
            let last = entries.len() - 1;
            for (i, path) in entries.into_iter().enumerate() {
                print!("{}", path.display());
                if i < last {
                    if nul {
                        print("\0");
                    } else {
                        print(" ");
                    }
                }
            }
        } else {
            println!("Size {} bytes:", size);
            for path in entries {
                println!("    {}", path.display());
            }
        }

        if nul {
            print("\0\0");
        } else {
            println!();
        }
    }

    if grandtotal {
        println!("Overall results:");
        println!("    {} groups of duplicate files", total_dupes);
        println!("    {} files are duplicates", total_files);
        let (val, suffix) = unbytify::bytify(total_size);
        println!("    {:.1} {} of space taken by duplicates", val, suffix);
    }
}

