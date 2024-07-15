use std::process::exit;

use kutuphane::{cli, library::Library};
use rand::random;
fn main() {
    let mut mylib = Library::new();
    test_fill(&mut mylib);
    cli::welcome();
    loop {
        let cmd = cli::main_menu();

        match cmd {
            cli::Command::Print => mylib.print_library(),
            cli::Command::Get(barcode) => {
                if let Some(book) = mylib.get_from_barcode(&barcode) {
                    println!("{}", book);
                } else {
                    println!("Kitap bulunamadı!");
                }
            }
            cli::Command::New((barcode, title)) => mylib.create_book(&title, barcode),
            cli::Command::Remove(barcode) => {
                println!(
                    "{}",
                    if mylib.delete_book(barcode) {
                        "Silindi"
                    } else {
                        "Kitap bulunamadı!"
                    }
                );
            }
            cli::Command::Lend((barcode, borrower_name)) => {
                match mylib.lend(&barcode, &borrower_name) {
                    Ok(_) => println!("Ödünç verildi."),
                    Err(msg) => println!("Hata. {}", msg),
                }
            }
            cli::Command::Exit => exit(0),
        }
    }
}

fn test_fill(my_lib: &mut Library) {
    my_lib.create_book("Kırmızı Başlıklı Kız", random());
    my_lib.create_book("Anna Karenina 1", random());
    my_lib.create_book("Anna Karenina 2", random());
    my_lib.create_book("Anna Karenina 3", random());
    my_lib.create_book("Etkili İnsanların 7 Alışkanlığı", random());
    my_lib.create_book("Zattiri Zort", random());
}
