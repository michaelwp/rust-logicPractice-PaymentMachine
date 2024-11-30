/*
    Payment machine logic
    - The machine has money with different denominations.
    - The machine receive customer's cash and will automatically deduct the cash with the item's price.
    - The machine will automatically give back the change based on the money availability.
 */
use std::collections::{BTreeMap};

fn main() {
   let change: Vec<i64> = payment_process(9000, 10000);
    println!("{:?}", change);
}

fn payment_process(items_price: i64, customers_cash: i64) -> Vec<i64> {
    let mut change: Vec<i64> = Vec::new();

    // set up the banknotes and coins list
    let mut banknotes_and_coins_list: BTreeMap<i64,i8> = setup_banknote_and_coins_list();



    // deduct the customer's cash with the item's price to get the cash change
    let mut money_diff: i64 = customers_cash - items_price;

    // get the available money (money amount > 0) in the machine
    let available_banknotes_and_coins: Vec<i64> = get_the_available_banknotes_and_coins(banknotes_and_coins_list.clone());

    // loop while the money_diff greater than 0
    let mut idx: usize = 0;
    while money_diff > 0 {
        // if index greater or equal than available_banknotes_and_coins length
        // then break the loop
        if idx >= available_banknotes_and_coins.len() {
            break;
        }

        let current_pos_money_nominal: i64 = available_banknotes_and_coins[idx];

        // get the current nominal value.
        // if it doesn't exist/ none then add 0 value as default
        let money_amount: &mut i8 = match banknotes_and_coins_list.get_mut(&current_pos_money_nominal) {
            Some(amount) => amount,
            None => banknotes_and_coins_list.entry(current_pos_money_nominal).or_insert(0),
        };

        // if the amount of current_pos_money_nominal less or equal than 0,then skip
        if *money_amount <= 0 {
            idx += 1;
            continue;
        }

        // compare the money_diff with the current pos money nominal
        if money_diff >= current_pos_money_nominal {
            // if the money_diff greater or equal than current pos nominal
            // then deduct the money_diff with the current_pos_money_nominal
            // and add the current_pos_money_nominal into the change list
            money_diff -= current_pos_money_nominal;
            change.push(current_pos_money_nominal);

            // reduce the money amount by 1
            *money_amount -= 1

        } else {
            // go next
            idx += 1;
        }
    }

    return change;
}

fn setup_banknote_and_coins_list() -> BTreeMap<i64, i8> {
    let mut banknotes_and_coins_list:BTreeMap<i64, i8> = BTreeMap::new();

    banknotes_and_coins_list.insert(1, 10);
    banknotes_and_coins_list.insert(5, 3);
    banknotes_and_coins_list.insert(10, 3);
    banknotes_and_coins_list.insert(50, 5);
    banknotes_and_coins_list.insert(100, 2);
    banknotes_and_coins_list.insert(500, 1);
    banknotes_and_coins_list.insert(1000, 0);
    banknotes_and_coins_list.insert(5000, 100);
    banknotes_and_coins_list.insert(10000, 100);

    return banknotes_and_coins_list;
}

fn get_the_available_banknotes_and_coins(banknote_and_coins_list: BTreeMap<i64,i8>) -> Vec<i64> {
    let mut available_banknotes_and_coins: Vec<i64> = Vec::new();

    // loop banknote_and_coins_list in descending way based on the nominal
    for (nominal, amount) in banknote_and_coins_list.iter().rev() {
        if *amount > 0 {
            available_banknotes_and_coins.push(*nominal);
        }
    }

    return available_banknotes_and_coins;
}
