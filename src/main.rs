use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


fn main() -> io::Result<()> {

    let input_path = "/Users/admin/Pr1/text/BednayaLiza.txt";

    // Открываем входной файл для чтения
    let input_file = File::open(input_path)?;
    let reader = io::BufReader::new(input_file);

    // Создаем HashMap для хранения слов и их частоты
    let mut word_count: HashMap<String, usize> = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            for word in line.split_whitespace() {
                // Удаляем знаки препинания, включая дефисы
                let cleaned_word = word
                    .replace(&['-','—', '.', ',', ';', '!', '?', ':'][..], "")
                    .to_lowercase();

                if !cleaned_word.is_empty() {
                    *word_count.entry(cleaned_word).or_insert(0) += 1;
                }
            }
        }
    }


    // Сортируем слова по убыванию частоты
    let mut word_count_vec: Vec<_> = word_count.iter().collect();
    word_count_vec.sort_by(|a, b| b.1.cmp(a.1)); // Сортировка по значению


    println!("Топ-10 самых часто встречающихся слов:");
    for (word, count) in word_count_vec.iter().take(10) {
        println!("{}: {}", word, count);
    }

    Ok(())
}