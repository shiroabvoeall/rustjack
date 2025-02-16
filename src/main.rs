//Imports
use std::io;
use std::io::Write;
use std::{thread, time};
use rand::Rng;

fn main() {
    
    let mut card_value: i8 = 0;
    let mut dealer_val: i8 = 0;
    let mut respo_hs = String::new();
    let duration = time::Duration::new(3, 0);

    card_value = hit(&mut card_value);
    dealer_val = hit(&mut dealer_val);

    loop {
        if twentyone(&mut card_value, &mut dealer_val) {
            thread::sleep(duration);
            break;
        }

        print!("\nDealer's value : {}", dealer_val);
        print!("\nYour current card value {} ( h - hit / s - stay ) : ", card_value);
        pushr(&mut respo_hs);

        match respo_hs.trim().to_lowercase().as_str() {
            "h" => {
                card_value = hit(&mut card_value);
                respo_hs.clear();
                if bustcheck(&mut card_value) {
                    thread::sleep(duration);
                    break;
                }
            }
            "s" => {
                dealer_val = hit(&mut dealer_val);
                while dealer_val < 17 {
                    dealer_val = hit(&mut dealer_val);
                }
                if checker_dealer(&mut dealer_val) {
                    thread::sleep(duration);
                    break;
                }
                wincond(&mut dealer_val, &mut card_value);
                thread::sleep(duration);
                break;
            }
            _ => {
                print!("Your input '{}' is not a valid command\n", respo_hs.trim());
                push();
                respo_hs.clear();
            }
        }

    }
}

// FUNCTIONS 
fn twentyone(card_value: &mut i8, dealer_val: &mut i8) -> bool {
    if *card_value == 21 {
        println!("\nYour total : {}\nDealer's total : {}\nBLACKJACK! YOU WON!", *card_value, *dealer_val);
        return true;
    } else {
        return false;
    }
}
fn bustcheck(card_value: &mut i8) -> bool {
    if *card_value > 21 {
        println!("{} was a bust!", card_value);
        return true;
    } else {
        return false;
    }
}

fn checker_dealer(dealer_val: &mut i8) -> bool {
    if *dealer_val > 21 {
        print!("Dealer busted with {}! You win!", *dealer_val);
        return true;
    } else {
        return false;
    }
}

fn hit(cardval: &mut i8) -> i8 {
    let card_hit = rand::thread_rng()
        .gen_range(1..11);
    let result = *cardval + card_hit;

    result
}

fn wincond(dealer_val: &mut i8, card_value: &mut i8) {
    if card_value < dealer_val{
        println!("Your total : {}\nDealer's total : {}\nYou lost!", card_value, dealer_val);
    } else if card_value > dealer_val{
        println!("Your total : {}\nDealer's total : {}\nYou won!", card_value, dealer_val);
    } else {
        println!("Your total : {}\nDealer's total : {}\nYou tied!", card_value, dealer_val);
    }
}

fn push() {
    io::stdout()
        .flush()
        .expect("Err");
}

fn pushr(respo_hs: &mut String) {
    io::stdout()
        .flush()
        .expect("Err");
    io::stdin()
        .read_line(respo_hs)
        .expect("Err");
}