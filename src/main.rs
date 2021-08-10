use std::option::Option::Some;
use crate::Direction::{North, South, East, West};


fn main() {
    //println!("{}",printer_error("aaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz"));
    //println!("{:?}",delete_nth(&[20,37,20,21], 1));
    //println!("{:?}",list_squared(1, 250));
    //println!("{:?}",expanded_form(70304));
    //println!("{:?}",anagrams("racer", &["crazer".to_string(), "carer".to_string(), "racar".to_string(), "caers".to_string(), "racer".to_string()]));
    //println!("{:?}",sum_pairs(&[1, 4, 8, 7, 3, 15], 8));
    //println!("{:?}",gap(2,100,110));
    //println!("{:?}",order("is2 Thi1s T4est 3a"));
    //println!("{:?}",order(""));
    //println!("{:?}",song_decoder("WUBAWUBWUBC"));
    //println!("{:?}",play_pass("I LOVE YOU!!!",1));
    //println!("{:?}",perimeter(5));
    //println!("{:?}",next_smaller_number(441));
    //println!("{:?}",rot13("Test"));
    /*let names = &vec![
        Name::Sheldon,
        Name::Leonard,
        Name::Penny,
        Name::Rajesh,
        Name::Howard,
    ];
    println!("{:?}",who_is_next(names,6));*/
    println!("Survivor: {:?}",josephus_survivor(11,19));

}

fn dna_strand(dna: &str) -> String {

    let charsdna :Vec<char> = dna.chars().collect();
    let mut rna :Vec<char> = dna.chars().collect();

    for ii in 0..charsdna.len(){
        match charsdna[ii] {
            'A' => {rna[ii] = 'T'}
            'T' => {rna[ii] = 'A'}
            'C' => {rna[ii] = 'G'}
            'G' => {rna[ii] = 'C'}
            _ => {}
        }
    }

    rna.iter().collect()
}

fn printer_error(s: &str) -> String {
    let mut err = 0;
    let mut print :Vec<char> = s.chars().collect();

    for c in &print{
        match c {
            'n'..='z' => {err+=1;}
            _ => {}
        }
    }
    let mut ausgabe:String = err.to_string();
    ausgabe.push_str("/");
    ausgabe.push_str(&*print.len().to_string());
    ausgabe
}

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut nlst :Vec<u8> = vec![];

    for u in lst{
        if nlst.contains(u){
            if nlst.iter().filter(|&m|*m == *u).count() < n{
                nlst.push(*u);
            }
        }else{
            nlst.push(*u);
        }
    }
    nlst
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut sqlst :Vec<(u64, u64)> = vec![];
    let mut square :Vec<u64> = vec![];
    for ii in m..=n{
        square = helper(ii);
        //println!("{:?}",square);
        if (square.iter().sum::<u64>() as f64).sqrt()*10.0%10.0 == 0.0{
            sqlst.push((ii,square.iter().sum()));
        }
    }
    sqlst
}

fn helper (h: u64) -> Vec<(u64)>{

    let mut square :Vec<u64> = vec![];

    for ii in 1..=h/2{
        if h%ii == 0{
            square.push(ii*ii);
        }
    }
    square.push(h*h);
    square
}

fn expanded_form(n: u64) -> String {
    let mut expre:String = "".to_string();
    let mut expo :Vec<u64> = vec![];
    let mut m = n;
    while m > 0{
        expo.push(m%10);
        m = m/10;
    }
    for ii in (1..expo.len()).rev(){
        //let mut text = ((expo.pop().unwrap() * 10u64.pow(ii as u32)).to_string());
        //println!("{}",text);
        let h = expo.pop().unwrap();
        if(h == 0){
            continue;
        }
        expre.push_str(&*(h * 10u64.pow(ii as u32)).to_string());
        expre.push_str(" + ");
    }
    let h = expo.pop().unwrap();
    if(h != 0){
        expre.push_str(&h.to_string());
    }else {
        for ii in 0..3{
            expre.remove(expre.len() - 1);
        }
    }
    expre
}

fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut anagramlist :Vec<String> = vec![];
    let mut check =true;
    for ii in 0..words.len(){
        check = true;
        if(word.len() != words[ii].len()){
            continue;
        }
        for jj in 0..word.len(){
           if(word.chars().filter(|x| *x==word.chars().nth(jj).unwrap()).count() != words[ii].chars().filter(|x| *x==word.chars().nth(jj).unwrap()).count()){
                check = false;
                break;
            }
        }
        if check{
            anagramlist.push(words[ii].clone());
        }
    }
    anagramlist
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut pos1 = ints.len();
    let mut pos2 = ints.len();

   for ii in 0..ints.len(){
       for jj in ii+1..pos2{
           if ints[ii]+ints[jj] == s{
               if jj < pos2 && pos1 != 0 && pos2 != 0 && ii != jj{
                   pos1 = ii;
                   pos2 = jj;
               }
           }
       }
   }
    if pos1 == ints.len(){
        return None
    }

    return Some((ints[pos1],ints[pos2]))
}


fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut first = 0;

    for ii in m..=n{
        if primechecker(ii){
            if first == 0 {
                first = ii;
            }else{
                if ii-first == g as u64{
                    return Some((first,ii))
                }else {
                    first = ii;
                }
            }
        }
    }
    return None;
}

fn primechecker(m: u64) -> bool{
    for ii in 2..m/2{
        if m%ii==0{
            return false
        }
    }
    return true
}

fn order(sentence: &str) -> String {
    let v: Vec<&str> = sentence.split(" ").collect();

    let mut str = "".to_string();
    for ii in 1..=v.len(){
        for jj in 0..v.len(){
            if v[jj].contains(&(ii).to_string()){
                str.push_str(v[jj]);
                str.push_str(" ");
                break;
            }
        }
    }
    if(!str.is_empty()){
        str.remove(str.len()-1);
    }
    str
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut theway: Vec<Direction> = vec![];

    theway.push(arr[1]);

    for ii in 1..arr.len(){
        if arr[ii] == Direction::South && theway.last().unwrap() == &Direction::North{
            theway.pop();
        }else if arr[ii] == Direction::West && theway.last().unwrap() == &Direction::East{
            theway.pop();
        }else if arr[ii] == Direction::North && theway.last().unwrap() == &Direction::South{
            theway.pop();
        } else if arr[ii] == Direction::East && theway.last().unwrap() == &Direction::West{
            theway.pop();
        } else {
            theway.push(arr[ii]);
        }

    }
    theway
}
//Frage schleifen, ++ operator
fn song_decoder(song: &str) -> String {
    let mut text = "".to_string();
    let mut ii = 0;
    while  ii < song.len(){
        if song.chars().nth(ii).unwrap() == 'W'{
            if ii+1 < song.len() && ii+2 < song.len(){
                if song.chars().nth(ii+1).unwrap() == 'U' && song.chars().nth(ii+2).unwrap() == 'B'{
                    if text.chars().last().unwrap_or('^') != ' '{
                        text.push(' ');
                    }
                    ii = ii+2;
                }else {
                    text.push(song.chars().nth(ii).unwrap());
                }
            }else {
                text.push(song.chars().nth(ii).unwrap());
            }
        }else {
            text.push(song.chars().nth(ii).unwrap());
        }
        ii+=1;
    }
    text = text.trim().to_string();
    text
}

//Frage Codereview
fn play_pass(s: &str, n: u32) -> String {
   let mut newpass = "".to_string();

    for ii in 0..s.len(){
        match s.chars().nth(ii).unwrap() {
            'a'..='z' =>{if ii %2 == 0 { newpass.push(shift_help(s.chars().nth(ii).unwrap(),n).to_ascii_uppercase());} else {newpass.push(shift_help(s.chars().nth(ii).unwrap(),n));}}
            'A'..='Z' =>{if ii %2 == 0 { newpass.push(shift_help(s.chars().nth(ii).unwrap(),n));} else { newpass.push(shift_help(s.chars().nth(ii).unwrap(),n).to_ascii_lowercase()); }}
            '0'..='9' =>{newpass.push(char::from_digit(9-s.chars().nth(ii).unwrap_or('0').to_digit(10).unwrap_or(0),10).unwrap());}
            _ => { newpass.push(s.chars().nth(ii).unwrap())} }
    }
    newpass.chars().rev().collect()
}

fn shift_help(s: char, n: u32) -> char{
    let c:char;
    match s {
        'a'..='z' =>{c = ((s as u8 -'a' as u8+n as u8) %('z' as u8 -'a' as u8 +1) +'a' as u8) as char}
        'A'..='Z' =>{c = ((s as u8 -'A' as u8+n as u8) %('Z' as u8 -'A' as u8 +1) +'A' as u8) as char}
        _ => {c= s;}
    }
    c
}

fn perimeter(n: u64) -> u64 {
    let mut fibo = vec![1, 1];

    for ii in 1..n {
        fibo.push(fibo.last().unwrap() + fibo.get(fibo.len() - 2).unwrap())
    }
    //Frage: Wie kann ich sowas nachschauen für funktionen
    4 * fibo.into_iter().sum::<u64>()
}
fn dsum(mut n: u64) -> u64 {
    let mut sum = 0;
    while n>0{
        sum += n%10;
        n = n/10;
    }
    sum
}

fn next_smaller_number(n: u64) -> Option<u64> {
    let suche = n.to_string();
    let mut fund = true;
    let mut vergleich;
    //let mut sec;
    let sum = dsum(n);

    if n >= 10 && n<=98{
       return Some( n - (n/10 - n%10)*9);
    }


    for ii in (1..=n-9).rev(){
        if sum != dsum(ii){
            continue
        }
        vergleich = ii.to_string();
        for c in suche.chars(){
            if !vergleich.contains(c){
                fund = false;
                break;
            }else {
                if  vergleich.chars().filter(|x| {*x == c}).count() != suche.chars().filter(|x| {*x == c}).count(){
                    fund = false;
                    break;
                }
            }
        }
        if fund{
            return Some(ii);
        }
        fund = true
    }

    None
}
fn rot13(message: &str) -> String {
    let mut newMessage = "".to_string();

    for s in message.chars(){
        match s {
            'a'..='z' =>{newMessage.push(((s as u8 -'a' as u8+13 as u8) %('z' as u8 -'a' as u8 +1) +'a' as u8 )as char)}
            'A'..='Z' =>{newMessage.push( ((s as u8 -'A' as u8+13 as u8) %('Z' as u8 -'A' as u8 +1) +'A' as u8) as char)}
            _ => {newMessage.push(s)}
        }
    }
    newMessage
}

/// _BigBang_ gang
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name { Sheldon, Leonard, Penny, Rajesh, Howard }

/// Just to make code look a bit nicer
type Names = Vec<Name>;

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next(names: &Names, n: usize) -> Name {
    /*let mut newlist = names.clone();
    let mut name;

    for ii in 0..n-1{
        name = newlist.remove(0);
        newlist.push(name);
        newlist.push(name);
    }*/
    return *names.get((n-1)%6).unwrap();
}

fn josephus_survivor(n: i32, k: i32) -> i32 {

    let mut sur = vec![true; n as usize];
    let mut pos:usize = 0;
    let mut zähler = 1;

    while sur.contains(&true){
        if zähler%k == 0{
            if sur[pos%n as usize]{
                sur[pos%n as usize] = false;
            }else {
                while !sur[pos%n as usize] {
                    pos+=1;
                }
                sur[pos%n as usize] = false;
            }
        }

        if !sur[pos%n as usize]{
            if sur.iter().filter(|x| {**x == false}).count() == sur.len(){
                break;
            }
            while !sur[pos%n as usize] {
                pos+=1;
            }
        }else {
            pos+=1;
            if !sur[pos%n as usize] {
                if sur.iter().filter(|x| {**x == false}).count() == sur.len(){
                    break;
                }
                while !sur[pos % n as usize] {
                    pos += 1;
                }
            }
        }
        zähler +=1;
    }

    pos as i32 %n+1
}



/*
Fragenblock

Strings
Mathematic (casts uX)

Ist None wie NULL in java

 */