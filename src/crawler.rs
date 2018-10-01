use reqwest::*;

pub const PAGE_SIZE: u8 = 20;

pub fn get_document(url: &String) -> Result<String> {
    let result = get(url)?.text()?;
    println!("{:?}", result);
    Ok(result)
}
