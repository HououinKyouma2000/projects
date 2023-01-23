use std::io;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    let spaces = "   ";//создаём экземпляр переменнойм типа &str
    //по сути переменная сверху просто исчезает
    let spaces = spaces.len(); //по сути инт  usize

    // теперь эта переменная другого типа
    let x = 5;
    //переменная x сверху исчезает , но новая х получает значение 5 и складывает с 1
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");//12
        //выйдя из скопупа этот х исчезает и мы получаем старый х
    }

    println!("The value of x is: {x}");//6


    let secret_number = rand::thread_rng() // получаем экземпляр генератора рандомных чисел
        .gen_range(1..=100); //вызываем функцию в диапозоне 1-100

    println!("The secret number is: {secret_number}");

    let mut guess = String::new(); //создаём экземпляр стринговоой
    io::stdin() //берём экземпляр standart input из библиотеки io
        .read_line(&mut guess)//пытаемся результат метода read_line засунуть в guess и для этого указываем , что даём его во владение
        .expect("Failed read"); //это кетч

    println!("Твоё число {guess}");
    //guess = String::new(); это работает нормально
    let guess = "123"; //тут мы полностью изменили и тип , поэтому нужен let
    println!("Твоё число {guess}");

    let x: i128 = 512341241312; // исп тип i128
    let y = 10; //по дефолту простовляется i32

    println!("x = {} and y = {}", x, y); //вывод как в локах log4j
}
