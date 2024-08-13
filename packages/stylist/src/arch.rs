#![cfg(feature = "browser_env")]

use crate::{Error, Result};
use web_sys::{Document, HtmlHeadElement, Window};

pub(crate) fn window() -> Result<Window> {
    web_sys::window().ok_or(Error::Web(None))
}

pub(crate) fn document() -> Result<Document> {
    window()?.document().ok_or(Error::Web(None))
}

pub(crate) fn doc_head() -> Result<HtmlHeadElement> {
    document()?.head().ok_or(Error::Web(None))
}
