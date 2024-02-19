use std::{
	env,
	fs::{self, File},
	io::{BufRead, BufReader, BufWriter, Write},
	path::{Path, PathBuf},
	process::ExitCode,
	time::{Duration, Instant},
};

use anyhow::bail;

fn bin_name(arg0: Option<impl AsRef<Path>>) -> String {
	arg0.and_then(|path| {
		path.as_ref()
			.file_name()
			.map(|v| v.to_string_lossy().into_owned())
	})
	.unwrap_or_else(|| env!("CARGO_BIN_NAME").into())
}

fn get_paths() -> anyhow::Result<(Vec<PathBuf>, PathBuf)> {
	let mut args = env::args_os();

	let bin_name = bin_name(args.next());
	let input = args.next();
	let output = args.next();

	let (Some(input), Some(output)) =
		(input.map(PathBuf::from), output.map(PathBuf::from))
	else {
		bail!("Usage: {bin_name} [FFXIVLogs directory] [directory to create output]");
	};

	let Ok(files) = fs::read_dir(&input) else {
		bail!("Cannot read directory: {}.", input.to_string_lossy());
	};

	let files = files
		.filter_map(|file| {
			let entry = file.ok()?;
			let file_name =
				entry.file_name().to_string_lossy().to_ascii_lowercase();
			let is_likely_the_log =
				file_name.starts_with("network") && file_name.ends_with(".log");
			let is_file =
				entry.file_type().map(|v| v.is_file()).unwrap_or_default();

			(is_file && is_likely_the_log).then(|| entry.path())
		})
		.collect();

	if fs::create_dir(&output).is_err() {
		bail!(
            "Cannot create directory: {}. Something already exists or not enough permission.",
            output.to_string_lossy()
        );
	}

	Ok((files, output))
}

fn main() -> anyhow::Result<ExitCode> {
	let (files, out_dir) = get_paths()?;
	let len = files.len();

	let mut has_error = false;

	for (file, i) in files.into_iter().zip(1usize..) {
		eprintln!("{i:>3}/{len} Processing {}", file.to_string_lossy());

		if let Err(err) = fun_name(file, &out_dir) {
			eprintln!("{err}");
			has_error = true;
		}
	}

	Ok(if has_error {
		ExitCode::FAILURE
	} else {
		ExitCode::SUCCESS
	})
}

fn fun_name(
	file: impl AsRef<Path>,
	out_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
	let file = file.as_ref();
	let out_dir = out_dir.as_ref();

	let Some(file_name) = file.file_name() else {
		unreachable!()
	};

	let file_name_str = file_name.to_string_lossy();
	let out_dir_str = out_dir.to_string_lossy();

	let Ok((input, input_size)) = File::open(file).and_then(|file| {
		let size = file.metadata()?.len();
		Ok((file, size))
	}) else {
		bail!("Failed to open {file_name_str}, skipping.");
	};

	let mut input = BufReader::new(input);

	let Ok(output) = File::create(out_dir.join(file_name)) else {
		bail!("Failed to create {file_name_str} in {out_dir_str}, skipping.");
	};

	let mut output = BufWriter::new(output);

	let mut last_printed = Instant::now();
	let mut pos = 0;
	let mut line = Vec::new();

	while let Ok(lsize) = input.read_until(b'\n', &mut line) {
		if lsize == 0 {
			break;
		}
		pos += lsize;

		if line.starts_with(b"00|") {
			let result = output.write_all(&line);
			if result.is_err() {
				bail!("Failed while writing {file_name_str} file");
			}
		}
		line.clear();

		// occasionally print percentage

		let now = Instant::now();
		let dur = now.checked_duration_since(last_printed).unwrap_or_default();

		if dur > Duration::from_millis(100) {
			// let epsilon = pos.abs_diff(prev_pos);
			// let epsilon_percentage =
			// 	((epsilon as f64 / input_size as f64) * 1000f64).round() as u16;

			// if epsilon_percentage > 1 {
			let percentage = (pos as f64 / input_size as f64) * 100f64;

			eprintln!("{:>8.02}% : {file_name_str}", percentage);
			// }

			last_printed = now;
		}
	}

	eprintln!("Completed : {file_name_str}");

	Ok(())
}
