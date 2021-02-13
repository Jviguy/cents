use std::env;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn coin_to_cents(coin: Coin, count: u32) -> u32 {
    match coin {
        Coin::Penny => return 1 * count,
        Coin::Nickel => return 5 * count,
        Coin::Dime => return 10 * count,
        Coin::Quarter => return 25 * count,
    }
}

fn str_to_coin(coin: &String) -> Result<Coin, String> {
    match *coin {
        _ if coin == "penny" => Ok(Coin::Penny),
        _ if coin == "nickel" => Ok(Coin::Nickel),
        _ if coin == "dime" => Ok(Coin::Dime),
        _ if coin == "quarter" => Ok(Coin::Quarter),
        _ => Err(format!("Unkown Coin: {}", *coin))
    }
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut count: u32 = 1;
   let coin: Coin;
   let coin_str: &String;
   if args[1] == String::from("-c") || args[1] == String::from("--count") {
       let result = &args[2].parse::<u32>();
       match result {
           Ok(result) => count = *result,
           Err(_) => {
               println!("Please provide A valid count for coins, 1 - 100");
               return
           }
       }
       coin_str = &args[3];
       let result = str_to_coin(coin_str);
       match result {
           Ok(result) => coin = result,
           Err(result) => {
               println!("{}", result);
               return
           }
       }
   } else {
       coin_str = &args[1];
       let result = str_to_coin(coin_str);
       match result {
           Ok(result) => coin = result,
           Err(result) => {
               println!("{}", result);
               return
           }
       }
   }
   let cents = coin_to_cents(coin, count);
   if cents == 1 && count == 1 {
       println!("{} {} is {} cent!", count, coin_str, cents);
   } else if cents == 1 {
       println!("{} {}s are {} cent", count, coin_str, cents);
   } else if count == 1 {
       println!("{} {} is {} cents!", count, coin_str, cents);
   } else {
       println!("{} {}s are {} cents", count, coin_str, cents);
   }
}
