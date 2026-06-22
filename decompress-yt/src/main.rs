use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: {} <filename>", args[0]);
        return 1;
    }

    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        // FIXED: Changed variable name from 'output' to 'outpath'
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment:{}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            println!("file {} extracted to *{}*", i, outpath.display()); // FIXED: Changed '.' to ','
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "file {} extracted to \"{}* ({} bytes)",
                i,
                outpath.display(),
                file.size() // FIXED: Changed 'file.sizes' to 'file.size()'
            );
            
            if let Some(p) = outpath.parent() { // FIXED: Capitalized 'Some'
                fs::create_dir_all(&p).unwrap();
            }

            // FIXED: Moved these inside the 'else' block so we only write data for actual files
            let mut outfile = fs::File::create(&outpath).unwrap(); // FIXED: Renamed to match below
            io::copy(&mut file, &mut outfile).unwrap();

            // FIXED: Moved the Unix permissions block inside the loop file extraction segment
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap(); // FIXED: 'fs::Permissions'
                }
            }
        }
    }

    0 // FIXED: Returns 0 to indicate the program finished successfully
}