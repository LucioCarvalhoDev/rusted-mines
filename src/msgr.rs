use std::io;

pub fn cin(text: String, storage: &mut String) -> &str {
    println!("{}", text);
    io::stdin().read_line(storage).expect("Erro ao ler input");

    storage
}
