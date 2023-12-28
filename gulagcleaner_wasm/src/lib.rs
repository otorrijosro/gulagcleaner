use wasm_bindgen::prelude::*;
use gulagcleaner_rs;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CleaningResult {
    result: Vec<u8>,
    method: u8,
}

#[wasm_bindgen]
pub fn clean_pdf(data: Vec<u8>,force_naive: u8) -> Vec<u8> {
    let (clean_pdf,method_code) = gulagcleaner_rs::clean_pdf(data, force_naive);
    // For some reason, the serde serialization messes up our data, so we just return the raw bytes until we find a fix
    // let example = CleaningResult {
    //     result: clean_pdf,
    //     method: method_code,
    // };
    // Ok(serde_wasm_bindgen::to_value(&example)?)
    return clean_pdf;
}