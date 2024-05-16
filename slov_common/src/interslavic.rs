use crate::*;

pub struct ISV {}

#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Dat,
    Ins,
    Loc,
    Voc,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Sing,
    Plur,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}


impl ISV {

    pub fn slice_without_last(s: &str) -> String {
        s.char_indices()
            .last() // Get the last character's (index, char)
            .map(|(idx, _)| &s[0..idx]) // Map to a slice without the last character
            .unwrap_or(s) // If the string is empty, return it as is
            .into()
    }

    pub fn last_in_slice(s: &str) -> char {
        s.chars()
            .last() // Get the last character's ( char)
            .unwrap_or(' ') // If the string is empty, return it as is
            .into()
    }

    pub fn is_vowel(c: char) -> bool {
        matches!(
            c,
            'a' | 'e'
                | 'i'
                | 'í'
                | 'ó'
                | 'o'
                | 'u'
                | 'å'
                | 'ą'
                | 'ę'
                | 'ė'
                | 'é'
                | 'ȯ'
                | 'ų'
                | 'ů'
                | 'ú'
                | 'ý'
                | 'y'
                | 'ě'
                | 'A'
                | 'E'
                | 'I'
                | 'O'
                | 'U'
                | 'á'
        )
    }


pub fn is_consonant(c: char) -> bool {
    !ISV::is_vowel(c)
}

pub fn is_hard_consonant(c: char) -> bool {
    matches!(
        c,
        'p' | 'b' | 'f' | 'v' | 'm' | 's' | 'z' | 't' | 'd' | 'r' | 'n' | 'l' | 'k' | 'g' | 'h'
    )
}

pub fn is_soft_consonant(c: char) -> bool {
    !ISV::is_hard_consonant(c.clone()) && !ISV::is_vowel(c.clone())
}

pub fn ends_with_soft_consonant(word: &str) -> bool {
    ISV::is_soft_consonant(ISV::last_in_slice(word))
}

pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
    if let Some(last_index) = input.rfind(pattern) {
        let (before_last, _after_last) = input.split_at(last_index);
        format!("{}{}", before_last, replacement)
    } else {
        input.into()
    }
}

pub const J_MERGE_CHARS: [&'static str; 11] = ["st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h"];

pub fn iotation_merge(root: &str, suffix: &str) -> String {
    if suffix.chars().nth(0) == Some('j') {
        for entry in ISV::J_MERGE_CHARS {
            if root.ends_with(entry) {
                let new_root = match entry {
                    "st" => ISV::replace_last_occurence(root, entry, "šć"),
                    "zd" => ISV::replace_last_occurence(root, entry, "ždž"),
                    "sk" => ISV::replace_last_occurence(root, entry, "šč"),
                    "zg" => ISV::replace_last_occurence(root, entry, "žž"),
                    "s" => ISV::replace_last_occurence(root, entry, "š"),
                    "z" => ISV::replace_last_occurence(root, entry, "ž"),
                    "t" => ISV::replace_last_occurence(root, entry, "ć"),
                    "d" => ISV::replace_last_occurence(root, entry, "dž"),
                    "k" => ISV::replace_last_occurence(root, entry, "č"),
                    "g" => ISV::replace_last_occurence(root, entry, "ž"),
                    "h" => ISV::replace_last_occurence(root, entry, "š"),
                    _ => root.to_string(),
                };
                let new_suffix = suffix.get(1..).unwrap();
                return format!("{new_root}{new_suffix}");
            }
        }

        format!("{root}{suffix}")
    } else {
        format!("{root}{suffix}")
    }
}
pub fn last_n_chars(word: &str, n: usize) -> String {
    if n <= word.len() {
        word[word.len() - n..].to_string()
    } else {
        word.to_string()
    }
}


    


}