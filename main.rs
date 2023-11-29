use std::io;

fn main() {
  let mut on = true;
  let mut _notebook = String::new();
  println!("Приветвую! \nЭто мега блокнот с использованием языка программирования Rust\nВыбирите определенное действие, команда help вам в помощь\nСоздал: InserTym\n");
  while on {
    println!("\nВвод:");
    let mut _choice = String::new();
    io::stdin().read_line(&mut _vubor).expect("Ошибка чтения строки");
    if _choice.trim() == "help" {
      println!("СПИСОК КОМАНД\n\n•read - Прочитать блокнот.\n•new - Записать данные.\n•delete - Очистить блокнот.\n•close - Выход из программы, ALT+F4 ноу.");
    } else if _choice.trim() == "close" {
      println!("Прощай!");
      on = false; 

    } else if _choice.trim() == "delete" {
      println!("Успешно! блокнот очищен");
      _notebook = String::new();

    } else if _choice.trim()  == "read" {
          println!("{}", _notebook.trim());   
      } else if _choice.trim() == "new" {
        println!("Напишите ваш текст:");
        io::stdin().read_line(&mut _notebook).expect("Ошибка чтения строки");
        println!("Успешно!")
      }
    

  }
      }
  
