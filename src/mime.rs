use mediatype::{names::Q, MediaType, MediaTypeList, ReadParams};

pub use mediatype::media_type;

pub fn prefered<'a, 'b>(
	accept: &str,
	prefers: &'a [MediaType<'b>],
) -> Option<&'a MediaType<'b>> {
	let mut max = (prefers.len(), 0f32);

	for mime in MediaTypeList::new(accept) {
		if let Ok(mime) = mime {
			let q = mime
				.get_param(Q)
				.and_then(|v| v.as_str().parse().ok())
				.unwrap_or(1f32);

			if q < max.1 {
				continue;
			}

			let mime = mime.essence();
			if let Some(index) = prefers.iter().position(|x| *x == mime) {
				if q > max.1 || index < max.0 {
					max = (index, q)
				}
			}
		} else {
			return None;
		}
	}

	prefers.get(max.0)
}
