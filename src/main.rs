fn main() {
    let mut count: i32 = 0;
    let mut plural = "s";
    let mut money_denomination: Vec<&str> = vec!["hundred", "fifty", "twenty", "ten", "five", "one", "quarter", "dime", "nickle", "penny"];

    for denom in get_change(50.25_f32, 100_f32).iter(){
        if denom == &0 {
            count += 1;
            continue;
        }

        else if denom == &1 {
            plural = "";

            if  money_denomination[count as usize] == "pennie" {
                 money_denomination[count as usize] = "penny";
            }

            else if  money_denomination[count as usize] == "twentie" {
                 money_denomination[count as usize] = "twenty"
            }
        }

        else if denom > &1{
            plural = "s";

            if  money_denomination[count as usize] == "penny" {
                 money_denomination[count as usize] = "pennie";
            }

            else if  money_denomination[count as usize] == "fifty"{
                 money_denomination[count as usize] = "fiftie";
            }

            else if  money_denomination[count as usize] == "twenty"{
                 money_denomination[count as usize] = "twentie";
            }
        }
        println!("{} {}{}", denom,  money_denomination[count as usize], plural);
        count += 1;
    }
}

fn get_change(price: f32, given: f32) -> Vec<i32>{
    let money_denomination: Vec<f32> = vec![100.00_f32, 50.00_f32, 20.00_f32, 10.00_f32, 5.00_f32, 1.00_f32, 0.25_f32, 0.10_f32, 0.05_f32, 0.01_f32];

    return if given > price {
        let mut change_owed: f32 = given - price;
        let mut change: Vec<i32> = vec![];

        for denom in money_denomination.iter() {
            if denom > &change_owed {
                change.push(0);
                continue;
            }

            if denom == &change_owed {
                change.push(1);
                break;
            }

            if change_owed < 0.02 && change_owed > 0.015 {
                change_owed = 0.02
            }

            let number_of_denomination: i32 = (change_owed / denom) as i32;

            change.push(number_of_denomination);
            change_owed = change_owed - (denom * number_of_denomination as f32);
        }
        change
    }

    else if given == price {
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }

    else {
        println!("Not enough");
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    }
}
