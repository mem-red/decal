use dashmap::DashMap;
use infer;
use std::sync::LazyLock;
use std::{io::Read, sync::Arc};
use usvg::{ImageHrefStringResolverFn, ImageKind, Options};

// static CACHE: LazyLock<DashMap<String, Arc<Vec<u8>>>> = LazyLock::new(|| DashMap::new());

//noinspection HttpUrlsUsage
fn fetch_image_cached(href: &str) -> Option<Arc<Vec<u8>>> {
    if let Some(bytes) = CACHE.get(href).map(|x| x.clone()) {
        return Some(bytes);
    }

    if let Ok(response) = ureq::get(href).call() {
        if response.status().is_success() {
            let mut buf = Vec::new();
            if response
                .into_body()
                .into_reader()
                .read_to_end(&mut buf)
                .is_ok()
            {
                let data = Arc::new(buf);
                CACHE.insert(href.to_owned(), data.clone());
                return Some(data);
            }
        }
    }

    None
}

pub(crate) fn get_image_href_resolver<'a>() -> ImageHrefStringResolverFn<'a> {
    Box::new(move |href: &str, _opts: &Options| {
        panic!("____CALLED____");
        let bytes = fetch_image_cached(href)?;
        let kind = infer::get(&bytes)?;

        Some(match kind.mime_type() {
            "image/png" => ImageKind::PNG(bytes.clone()),
            "image/jpeg" => ImageKind::JPEG(bytes.clone()),
            "image/webp" => ImageKind::WEBP(bytes.clone()),
            "image/gif" => ImageKind::GIF(bytes.clone()),
            _ => return None,
        })
    })
}
