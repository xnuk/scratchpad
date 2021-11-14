use msiviewer::{entries, get_cab};
use std::env::args_os;
use std::{fs, io};

fn main() -> io::Result<()> {
	let path = args_os().nth(1).unwrap();
	let out_path = args_os().nth(2).unwrap();

	let mut file = msiviewer::open(path)?;
	let compfiles = entries(&mut file)?;

	for (key, name) in compfiles {
		println!("{}: {}", key, name.join("/"));
	}

	let mut stream = get_cab(&mut file)?;
	let mut out = fs::File::create(out_path)?;
	io::copy(&mut stream, &mut out)?;

	/*

	for id in media {
		let mut cabinet = Cabinet::new(file.read_stream(&id)?)?;

		let fileentries = &cabinet
			.folder_entries()
			.flat_map(|x| x.file_entries())
			.map(|v| (v.name().to_owned(), v.uncompressed_size()))
			.collect::<Vec<_>>();

		for entry @ (name, size) in fileentries {
			println!("{:?}: {:?}", entry, compfiles.get(name));
			// let mut buf = Vec::with_capacity(*size as usize);
			let buf: Vec<_> = cabinet
				.read_file(name)?
				.bytes()
				.zip(&two_a)
				.filter(|(x, o)| match x {
					Err(_) => true,
					Ok(x) => x != *o,
				})
				.collect();
			println!("{:?}", buf.len());
		}
	}*/

	Ok(())
}
