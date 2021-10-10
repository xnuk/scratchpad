use std::{
	env::args_os,
	io::{self, ErrorKind, Read},
};

use nom::{
	bytes::complete::take,
	number::complete::{le_u16, le_u32},
	IResult,
};

fn to_b64(ch: char) -> Option<u32> {
	if ch >= '0' && ch <= '9' {
		Some(ch as u32 - '0' as u32)
	} else if ch >= 'A' && ch <= 'Z' {
		Some(10 + ch as u32 - 'A' as u32)
	} else if ch >= 'a' && ch <= 'z' {
		Some(36 + ch as u32 - 'a' as u32)
	} else if ch == '.' {
		Some(62)
	} else if ch == '_' {
		Some(63)
	} else {
		None
	}
}

fn encode(name: &str, is_table: bool) -> String {
	let mut output = String::new();
	if is_table {
		output.push('\u{4840}');
	}
	let mut chars = name.chars().peekable();
	while let Some(ch1) = chars.next() {
		if let Some(value1) = to_b64(ch1) {
			if let Some(&ch2) = chars.peek() {
				if let Some(value2) = to_b64(ch2) {
					let encoded = 0x3800 + (value2 << 6) + value1;
					output.push(char::from_u32(encoded).unwrap());
					chars.next();
					continue;
				}
			}
			let encoded = 0x4800 + value1;
			output.push(char::from_u32(encoded).unwrap());
		} else {
			output.push(ch1);
		}
	}
	output
}

fn read_props<'a>(bytes: &'a [u8]) -> IResult<&'a [u8], ()> {
	let (bytes, _) = le_u16(bytes)?; // 0xfffe
	let (bytes, version) = le_u16(bytes)?;
	let (bytes, os) = le_u16(bytes)?;
	let (bytes, clsid) = take(16usize)(bytes)?;
	let (bytes, _) = le_u32(bytes)?; // reserved
	let (bytes, fmtid) = take(16usize)(bytes)?;
	let (bytes, offset) = le_u32(bytes)?;

	println!(
		"version: {version}, os: {os:x}, clsid: {clsid:?}, fmtid: {fmtid:?}, offset: {offset:x}",
		version = version,
		os = os,
		clsid = clsid,
		fmtid = fmtid,
		offset = offset
	);

	Ok((bytes, ()))
}

fn main() -> Result<(), io::Error> {
	let mut file = {
		if let Some(path) = args_os().nth(1) {
			cfb::open(path)
		} else {
			Err(ErrorKind::InvalidInput.into())
		}
	}?;

	{
		let mut stream = file.open_stream("\u{5}SummaryInformation")?;
		let mut vv = vec![];
		stream.read_to_end(&mut vv)?;
		let res = read_props(vv.as_slice());
		println!("{:?}", res);
	}

	for entry in file.walk_storage("/")? {
		println!("{:?} {:?}", entry.name(), entry.path())
	}

	Ok(())
}
