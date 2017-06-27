
use std::collections::HashMap;
use std::string::String;
use std::str;
extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;
extern crate permutohedron;
use std::time::Instant;
use permutohedron::heap_recursive;
mod utility;
use utility::words_list;
extern crate smallvec;
use smallvec::SmallVec;
extern crate smallstring;
use smallstring::SmallString;

fn test<'bleh>(target: &Vec<&[u8]>) -> bool {
    let s = "printout stout yawls".to_string();
    let mut l = Vec::new();
    l.push(&s);
    let mut hasher = Md5::new();
    check_hash(&mut hasher,&l,target)
}

fn exist_in(a:&SmallString, bb: &str) -> bool {
    a.chars().all(|c| bb.chars().any(|b| c==b))
}

fn create_count_dict_str(s: SmallString) -> HashMap<char,usize>{
    let mut l: SmallVec<[SmallString;8]> = SmallVec::new();
    l.push(s);
    create_count_dict(&l)

}

fn correct_amount(b:&SmallVec<[SmallString;8]>, rules: &HashMap<char,usize>) -> bool {
    let t = create_count_dict(b);
    t.iter().all(|(k,v)| v<=rules.get(k).unwrap())
}

fn create_count_dict(b:&SmallVec<[SmallString;8]>) -> HashMap<char,usize> {
    let mut alphabet: HashMap<char,usize> = HashMap::new();

    for s in b.iter() {
        for c in s.chars() {

            let q = alphabet.get(&c).map_or(0usize,|x| x+1usize);
            alphabet.insert(c,q);

        }
    }

    alphabet
}


fn compatible(sentence: &SmallVec<[SmallString;8]>,rules: &HashMap<char,usize>) -> bool {
    cost(sentence) <= 18 && correct_amount(sentence,rules)
}
fn cost(sentence: &SmallVec<[SmallString;8]>) -> usize {
    sentence.iter().fold(0,|a,b| a+cost_str(b))
}

fn cost_str(word: &SmallString) -> usize {word.len()}

fn check_hash(hasher: &mut Md5, sentence: &Vec<&String>, target: &Vec<&[u8]>) -> bool {
    hasher.reset();
    let length = sentence.len();
    for (i,e) in sentence.iter().enumerate() {
        hasher.input(e.as_bytes());
        if i == (length-1) {
            //last element, no space
        } else {
            hasher.input(" ".as_bytes());
        }
    }

    let result = hasher.result_str();
    let b = result.as_bytes();

    for item in target {
        //if str::from_utf8(*item).unwrap() == result { return true; }
        if eq(item,&b) { return true; }
    }
    false
}

fn eq(a: &[u8], b: &[u8]) -> bool {
    for i in 1..a.len() {
        if (a[i]) == (b[i]) {
            continue;
        } else {
            return false;
        }
    }
    true
}


//warning: assume sorted
fn canonical<'life>(sentence:  &SmallVec<[SmallString;8]>) -> SmallString {
    join(sentence,"")
}

fn join(sentence: &SmallVec<[SmallString;8]>, sep: &str) -> SmallString {
    let mut r: String = "".into();

    for (i,s) in sentence.iter().enumerate() {
        r.push_str(s.to_string().as_str());

        if !(i==sentence.len()-1) {r.push_str(sep)} //don' push sep to the end
    }
    SmallString::from(r)
}

fn join_string(sentence: &Vec<&String>, sep: &str) -> String {
    let mut r: String = "".into();

    for (i,s) in sentence.iter().enumerate() {
        r.push_str(s.to_string().as_str());

        if !(i==sentence.len()-1) {r.push_str(sep)} //don' push sep to the end
    }
    r
}



//build anagram using cur_words and words to create len()+1 anagrams. returns (candidates,solutions_found)
fn build_anagram<'this,>(cur_words: Vec<SmallVec<[SmallString;8]>>, words: &'this Vec<SmallString>, target: &Vec<&[u8]>, rules: &HashMap<char,usize>) -> (Vec<SmallVec<[SmallString;8]>>,Vec<String>){
    let mut seen: HashMap<SmallString,SmallVec<[SmallString;8]>> = HashMap::new();
    let mut solutions: Vec<String> = Vec::new();
    let mut md5 = Md5::new();

    let mut count = 0;
    let now = Instant::now();
    for sentence in cur_words.iter() {
        count+=1;
        //if count % 1000 == 0 {
        //    println!("{} / 100 | {} seconds elapsed", count*100/&cur_words.len(),now.elapsed().as_secs());
        //}
        for word in words.iter() {

            let mut new_sentence = sentence.clone();
            new_sentence.push(word.clone());


            new_sentence.sort_by(|a,b| a.cmp(b)); //inplace sorting

            let k = canonical(&new_sentence);
            if seen.get(&k).is_some() { continue; } //we'e already checked this
            if compatible(&new_sentence,rules) {
                if cost(&new_sentence) == 18 {
                    let temp: Vec<String> = new_sentence.iter().map(|x| x.to_string()).collect::<Vec<_>>();
                    let mut permutable: Vec<&String> = temp.iter().map(|x| x).collect::<Vec<_>>();
                    let perms = permutate(&mut permutable);

                    for p in perms.iter() {
                        if check_hash(&mut md5,p,target) {
                            let string = join_string(p, " ");
                            println!("{} ->correct", &string);
                            solutions.push(string);
                        }
                    }
                }
                seen.insert(k,new_sentence);
            }
        }
    }

    (seen.values().cloned().collect::<Vec<_>>(),solutions)
}

fn permutate<'life>(d :&Vec<&'life String>) -> Vec<Vec<&'life String>> {
    let mut data = d.clone();
    let mut permutations: Vec<Vec<&String>> = Vec::new();
    heap_recursive(&mut data, |permutation| {
        permutations.push(permutation.to_vec())
    });

    permutations
}

fn main() {
    println!("tRUSTpilot anagram checker!");

    let alphabet: SmallString = "poultryoutwitsants".into();
    let words: Vec<SmallString> = words_list("wordlist").into_iter().filter(|w| exist_in(w,&alphabet)).collect::<Vec<_>>();
    let target: Vec<&[u8]> = vec!["e4820b45d2277f3844eac66c903e84be".as_bytes(), "23170acc097c24edb98fc5488ab033fe".as_bytes(), "665e5bcb0c20062fe8abaaf4628bb154".as_bytes()];
    let rules = create_count_dict_str(alphabet);
    let mut solutions: Vec<String> = Vec::new();
    println!("checker works? {}",test(&target));


    let total_elapsed = Instant::now();

    let mut one: Vec<SmallVec<[SmallString;8]>> = Vec::new();
    for w in words.iter() {
        let mut wl: SmallVec<[SmallString;8]> = SmallVec::new();
        wl.push((*w).clone());
        one.push(wl);
    }

    let mut cur = one;
    println!("candidates size: {}", cur.len());
    let mut size = 1;
    while solutions.len() < target.len() {
        println!("Building anagrams of size {}; elapsed: {} seconds",size, total_elapsed.elapsed().as_secs());
        let (next,mut sol) = build_anagram(cur,&words,&target,&rules);
        cur = next;
        solutions.append(&mut sol);
        size+=1;
    }

    println!("Solutions: ");
    println!("{}", solutions.join(","));

}