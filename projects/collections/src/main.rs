use std::collections::HashMap;


// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }


fn main() {

    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[3];
    // println!("The third element is {}", third);

    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element")
    // }

    //-------------------------

    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6); //won't work as we already created an immutable reference

    // println!("The first element is: {}", first);

    //--------------------------

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    //     println!("vector value: {}", i)
    // }

    //----------------------------

    // {
    //     let row = vec![
    //         SpreadsheetCell::Int(3),
    //         SpreadsheetCell::Text(String::from("blue")),
    //         SpreadsheetCell::Float(10.12),
    //         ];
    // }


    //----------------------------

    // {
    //     let data = "initial contents";

    //     let s = data.to_string();
    
    //     let mut s = "initial contents".to_string();
    
    //     s.push_str("bar");
    
    //     println!("{}", s);
    // }


    // ---------------------------

    // {
    //     let s1 = String::from("foo");
    //     let mut s2 = String::from(" bar");
    //     let s3 = s1 + &s2;
    //     println!("{}", s3);
    //     s2 = String::from(" sdfdsfsdf");
    //     println!("{}", s3); //still foo bar
    
    // }

    //-------------------------------

    // {
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3);
    // }


    //------------------------------


    // {
    //     let mut scores = HashMap::new();

    //     scores.insert(String::from("Blue"), 10);
    //     scores.insert(String::from("Green"), 15);
    // }

    

    //------------------------------

    // {
    //     let blue_team = String::from("Blue");
    //     let green_team = String::from("Green");
    //     let teams = vec![&blue_team, &green_team];
    //     let initial_scores = vec![10, 50];
    
    
    //     let mut scores: HashMap<_,_> = 
    //         teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    
    //     let score = scores.get(&blue_team);
    // }

    // -----------------------------

    //iterating through a hash map
    // {
    //     let mut scores = HashMap::new();

    //     scores.insert(String::from("Blue"), 10);
    //     scores.insert(String::from("Yellow"), 50);
        
    //     for (key, value) in &scores {
    //         println!("{}: {}", key, value);
    //     }

    // }

    //-------------------------------

    // {
    //     let mut scores = HashMap::new();
    //     scores.insert(String::from("Blue"), 10);
    
    //     scores.entry(String::from("Yellow")).or_insert(50);
    //     scores.entry(String::from("Blue")).or_insert(50);
    
    //     println!("{:?}", scores);
    // }

    //-------------------------------

    let int_list = vec![0 , 3, 3, 4, 2, 6, 7, 8];
    let average = return_average(&int_list);
    println!("mode: {}", average.mode);
    println!("median: {}", average.median);
    println!("mean: {}", average.mean);

    //--------------------------------

    let english = "lets convert this into pig latin";
    let mut pig_latin_list = Vec::new();

    for word in english.split_whitespace() {
        pig_latin_list.push(convert_to_pig_latin(word));
    }
    println!("english [{}]", english);
    println!("pig-latin [{}]", pig_latin_list.join(" "));
}


pub fn convert_to_pig_latin(input: &str) -> String {

    let mut ending = String::from("ay");
    let vow_ending = "hay";

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut output = input.to_owned();
    
    let first_char = input.chars()
        .next()
        .expect("empty string! can't convert");

    if vowels.contains(&first_char) {

        output = output + vow_ending;

    } else {

        let mut chars = output.chars();
        chars.next();
        ending.insert(0, first_char);
        let new_word :String = chars.collect();
        output = new_word + &ending;
    }
    
    output
}



pub fn return_average(integer_list: &Vec<i32>) -> Averages {

    let mode = calc_mode(integer_list);
    let median = calc_median(integer_list);
    let mean = calc_mean(integer_list);

    Averages {
        mean: mean,
        mode: mode,
        median: median
    }
} 

pub struct Averages {
    mean: f64,
    median: f64,
    mode: i32
}

fn calc_mode(integer_list: &Vec<i32>) -> i32 {

    let mut mode :i32 = 0;
    let mut mode_map = HashMap::new();
    for int in integer_list.iter() {
        let count = mode_map.entry(int).or_insert(0);
        *count += 1;
    };

    let mut max_count = 0;
    for entry in mode_map.iter() {
        if entry.1 > &max_count {
            max_count = *entry.1;
            mode = **entry.0;
        };
    }

    mode
}

fn calc_mean(integer_list: &Vec<i32>) -> f64 {

    let list_length = integer_list.len();
    let sum: i32 = integer_list.iter().sum();
    (sum as f64 / (list_length as f64)) as f64

}

fn calc_median(integer_list: &Vec<i32>) -> f64 {

    let list_length = integer_list.len();
    let median :f64;
    let mut sorted_list = integer_list.clone();
    sorted_list.sort_unstable();
    if list_length % 2 == 0 {
        let mid = &list_length / 2;
        median = (sorted_list[mid] + sorted_list[mid - 1]) as f64 / 2.0;
    } else {
        let mid = (&list_length / 2) as f64;
        let mid_idx = mid.floor() as usize;
        median = sorted_list[mid_idx] as f64;
    }
    median
}


