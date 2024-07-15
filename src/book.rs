use std::fmt::Display;

pub struct Book {
    title: String,
    barcode: u8,
    borrower: Option<String>,
}

impl Book {
    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_barcode(&self) -> u8 {
        self.barcode
    }

    pub fn get_occupacy(&self) -> Option<String> {
        self.borrower.clone()
    }

    pub fn lend_to(&mut self, borrower_name: &str) -> Result<(), String> {
        if let Some(current_borrower) = self.borrower.as_ref() {
            Err(format!(
                "This book is already borrowed by {current_borrower}"
            ))
        } else {
            self.borrower = Some(borrower_name.to_string());
            Ok(())
        }
    }

    pub fn new(title: &str, barcode: u8) -> Self {
        Book {
            title: title.to_string(),
            barcode,
            borrower: None,
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "'{}'\t|\tBarkod:{}\t|\t Ödünç:{}",
            self.get_title(),
            self.get_barcode(),
            self.get_occupacy().unwrap_or("-".to_string()),
        ))?;

        return Ok(());
    }
}
