use std::env;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn coin_to_cents(coin: Coin, count: u32) -> u32 {
    match coin {
        Coin::Penny =>  count,
        Coin::Nickel => 5 * count,
        Coin::Dime => 10 * count,
        Coin::Quarter => 25 * count,
    }
}

fn str_to_coin(coin: &str) -> Result<Coin, String> {
    match coin {
        "penny" => Ok(Coin::Penny),
        "nickel" => Ok(Coin::Nickel),
        "dime" => Ok(Coin::Dime),
        "quarter" => Ok(Coin::Quarter),
        _ => Err(format!("Unknown Coin: {}", coin))
    }
}

fn main() {
   let args: Vec<String> = env::args().collect();
   let mut count: u32 = 1;
   if args.len() <= 1 {
       println!("Please provide a coin or use `coins -h` or `coins --help` for help!");
       return
   }
   if args[1] == "-h" || args[1] == "--help" {
       println!("cents is a open source american coin to cents program made in rust!");
       println!("Its used like `cents penny` and will then echo back the amount of cents a penny is.");
       println!("There are also other arguments that can be made like. -h (help) and -c (count)");
       println!("Example: `cents -c 20 penny` will return 20 pennys are 20 cents.");
       return 
   }
   let coin_str: &str = if args[1] == "-c" || args[1] == "--count" {
       let result = args[2].parse::<u32>();
       match result {
           Ok(result) => count = result,
           Err(_) => {
               println!("Please provide A valid count for coins, 1 - 100");
               return
           }
       }
       args[3].as_str()
   } else {
       args[1].as_str()
   };
   let coin: Coin = match str_to_coin(coin_str) {
       Ok(n) => n,
       Err(n) => {
           println!("{}", n);
           return;
       }
   };
   let cents = coin_to_cents(coin, count);
   match count { 
       1 if cents == 1 => println!("{} {} is {} cent!", count, coin_str, cents),
       1 => println!("{} {} is {} cents!", count, coin_str, cents),
       _ => println!("{} {}s are {} cents", count, coin_str, cents),
   }
}
