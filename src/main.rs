use std::io;

// Main Function

fn main() {
    // Создание строк для ввода
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    // Ввод значений
    println!("Введите значение a:");
    io::stdin().read_line(&mut a_str).expect("Ошибка при чтении строки");
    println!("Введите значение b:");
    io::stdin().read_line(&mut b_str).expect("Ошибка при чтении строки");
    println!("Введите значение c:");
    io::stdin().read_line(&mut c_str).expect("Ошибка при чтении строки");

    // Преобразование строк в числа
    let a: f64 = match a_str.trim().parse() {
        Ok(value) => value,
        Err(e) => {
            println!("Ошибка при преобразовании a: {}", e);
            return;
        }
    };

    let b: f64 = match b_str.trim().parse() {
        Ok(value) => value,
        Err(e) => {
            println!("Ошибка при преобразовании b: {}", e);
            return;
        }
    };

    let c: f64 = match c_str.trim().parse() {
        Ok(value) => value,
        Err(e) => {
            println!("Ошибка при преобразовании c: {}", e);
            return;
        }
    };

    // Вычисление дискриминанта
    let d = (b * b) - 4.0 * a * c;

    // Решение уравнения
    if d > 0.0 {
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Корни уравнения: {} и {}", x1, x2);
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("Корень уравнения: {}", x);
    } else {
        println!("Корней нет, D < 0!");
    }
}
