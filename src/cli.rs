use std::io::{stdin, stdout, Write};

pub enum Command {
    Print,
    Get(u8),
    New((u8, String)),
    Remove(u8),
    Lend((u8, String)),
    Exit,
}

mod menu_messages {
    pub const WELCOME_MESSAGE: &str = "\
╔════════════════════════╗
║                        ║
║  KÜTÜPHANE UYGULAMASI  ║
║    V1.0.0    2024      ║
║                        ║
╚════════════════════════╝\
    ";

    pub const SELECT_CMD: &str = "Komutlardan birini seçin:";
    pub const CMD_LIST_ALL: &str = "Tüm kitapları göster";
    pub const CMD_GET_FROM_BARCODE: &str = "Barkod ile ara";
    pub const CMD_NEW_BOOK: &str = "Yeni kitap oluştur";
    pub const CMD_REMOVE_BOOK: &str = "Kitap sil";
    pub const CMD_LEND: &str = "Ödünç ver";
    pub const CMD_EXIT: &str = "Çık";

    pub const ENTER_BARCODE: &str = "Barkod girin: ";
    pub const ENTER_BOOK_TITLE: &str = "Kitap ismini girin: ";
    pub const ENTER_BORROWER_NAME: &str = "Ödünç alacak kişinin ismi: ";

    pub const UNKNOWN_CMD: &str = "Lütfen geçerli bir komut girin!\n\n";
    pub const WRONG_BARCODE: &str = "Hatalı barkod!\n\n";
    pub const BOOK_TITLE_EMPTY: &str = "Kitap ismi boş olamaz!\n\n";
    pub const BORROWER_NAME_EMPTY: &str = "Ödünç alan ismi boş olamaz!\n\n";
}

pub fn welcome() {
    println!("{}", menu_messages::WELCOME_MESSAGE);
}

pub fn main_menu() -> Command {
    loop {
        println!("{}", menu_messages::SELECT_CMD);
        println!("1-{}", menu_messages::CMD_LIST_ALL);
        println!("2-{}", menu_messages::CMD_GET_FROM_BARCODE);
        println!("3-{}", menu_messages::CMD_NEW_BOOK);
        println!("4-{}", menu_messages::CMD_REMOVE_BOOK);
        println!("5-{}", menu_messages::CMD_LEND);
        println!("");
        println!("0-{}", menu_messages::CMD_EXIT);
        print!("_> ");
        stdout().flush().expect("Cannot write to STDOUT");
        let mut typed_command = String::new();
        if stdin().read_line(&mut typed_command).is_err() {
            continue;
        }
        let command_number: u8 = match typed_command.trim().parse::<u8>() {
            Ok(command_number) => command_number,
            Err(_) => {
                println!("{}", menu_messages::UNKNOWN_CMD);
                continue;
            }
        };

        let command = match command_number {
            0 => Command::Exit,
            1 => Command::Print,
            2 => {
                println!();
                Command::Get(request_barcode())
            }
            3 => Command::New(request_book_info()),
            4 => Command::Remove(request_barcode()),
            5 => Command::Lend(request_lend_info()),
            _ => {
                println!("{}", menu_messages::UNKNOWN_CMD);
                continue;
            }
        };

        return command;
    }
}

fn request_barcode() -> u8 {
    loop {
        print!("{}", menu_messages::ENTER_BARCODE);
        stdout().flush().expect("Cannot write to STDOUT");
        let mut barcode_text = String::new();
        stdin()
            .read_line(&mut barcode_text)
            .expect("Cannot read STDIN");
        let barcode: u8 = match barcode_text.trim().parse::<u8>() {
            Ok(barcode) => barcode,
            Err(_) => {
                println!("{}", menu_messages::WRONG_BARCODE);
                continue;
            }
        };
        return barcode;
    }
}

fn request_book_info() -> (u8, String) {
    loop {
        print!("{}", menu_messages::ENTER_BOOK_TITLE);
        stdout().flush().expect("Cannot write to STDOUT");
        let mut book_title = String::new();
        stdin()
            .read_line(&mut book_title)
            .expect("Cannot read STDIN");
        let book_title = book_title.trim();
        let book_title = if book_title.is_empty() {
            println!("{}", menu_messages::BOOK_TITLE_EMPTY);
            continue;
        } else {
            book_title.to_string()
        };

        let barcode = request_barcode();

        return (barcode, book_title);
    }
}

fn request_lend_info() -> (u8, String) {
    loop {
        let barcode = request_barcode();

        print!("{}", menu_messages::ENTER_BORROWER_NAME);
        stdout().flush().expect("Cannot write to STDOUT");
        let mut borrower_name = String::new();
        stdin()
            .read_line(&mut borrower_name)
            .expect("Cannot read STDIN");
        let borrower_name = borrower_name.trim();
        let borrower_name = if borrower_name.is_empty() {
            println!("{}", menu_messages::BORROWER_NAME_EMPTY);
            continue;
        } else {
            borrower_name.to_string()
        };

        return (barcode, borrower_name);
    }
}
