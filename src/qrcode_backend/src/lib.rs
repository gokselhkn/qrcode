use candid::{CandidType, Deserialize};
use std::include_bytes;

mode core;

const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const LOGO_TRANSPARENT: &[U8] = include_bytes!("../assets/logo-transparent.png");
const LOGO_WHITE: &[U8] = include_bytes!("../assets/logo-white.png");

#[derive(CandidType, Deserialize)]
struct Options {
  add_logo: bool,
  add_gradient: bool,
  add_transparency: Option<bool>,
}

#[derive(CandidType, Deserialize)]
struct QrError {
  message: String,
}

#[derive(CandidType, Deserialize)]
enum QrResult {
  Image(Vec<u8>),
  Error(QrError),
}

fn qrcode_impl(input: String, options: Options) -> QrResult {
    let logo = if options.add_transparency == Some(true) {
        LOGO_TRANSPARENT
    } else {
        LOGO_WHITE
    };
    let result = match core::generate(input,options,logo, IMAGE_SIZE_IN_PIXELS) {
        Ok(blob) => QrResult::Image(blob),
        Err(err) => QrResult::Error(QrError { message: err.to_string(), })
    };
    ic_cdk::println!(
        "Executed instructions: {}", ic_cdk::api::performance_counter(0)
    );
    result
}

#[ic_cdk::update]
fn qrcode(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}

#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult {
    qrcode_impl(input, options)
}
