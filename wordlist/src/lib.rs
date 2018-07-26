#![deny(missing_docs)]
#![feature(int_to_from_bytes)]

//! Monero wordlists
//!
//! This crate collects and implements the operations for the wordlists
//! available for Monere.
//!

extern crate crc;

mod chinese_simplified;
mod dutch;
mod english;
mod english_old;
mod esperanto;
mod french;
mod german;
mod italian;
mod japanese;
mod lojban;
mod portuguese;
mod russian;
mod spanish;

/// Numbers of words in a monero seed.
pub const SEED_LENGTH: usize = 24;

/// Word list type.
pub type Wordlist = &'static [&'static str];

/// The word list language.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Language {
    /// Chinese (simplified) language.
    ChineseSimplified,
    /// Dutch language.
    Dutch,
    /// English language.
    English,
    /// English (Old) language.
    EnglishOld,
    /// Esperanto language.
    Esperanto,
    /// French language.
    French,
    /// German language.
    German,
    /// Italian language.
    Italian,
    /// Japanese language.
    Japanese,
    /// Lojban language.
    Lojban,
    /// Portuguese language.
    Portuguese,
    /// Russian language.
    Russian,
    /// Spanish language.
    Spanish,
}

impl Language {
    /// Returns the language name in English.
    pub fn english_name(self) -> &'static str {
        match self {
            Language::ChineseSimplified => "Chinese (simplified)",
            Language::Dutch => "Dutch",
            Language::English => "English",
            Language::EnglishOld => "English (old)",
            Language::Esperanto => "Esperanto",
            Language::French => "French",
            Language::German => "German",
            Language::Italian => "Italian",
            Language::Japanese => "Japanese",
            Language::Lojban => "Lojban",
            Language::Portuguese => "Portuguese",
            Language::Russian => "Russian",
            Language::Spanish => "Spanish",
        }
    }

    /// Get the language word list.
    pub fn wordlist(self) -> Wordlist {
        match self {
            Language::ChineseSimplified => chinese_simplified::WORDLIST,
            Language::Dutch => dutch::WORDLIST,
            Language::English => english::WORDLIST,
            Language::EnglishOld => english_old::WORDLIST,
            Language::Esperanto => esperanto::WORDLIST,
            Language::French => french::WORDLIST,
            Language::German => german::WORDLIST,
            Language::Italian => italian::WORDLIST,
            Language::Japanese => japanese::WORDLIST,
            Language::Lojban => lojban::WORDLIST,
            Language::Portuguese => portuguese::WORDLIST,
            Language::Russian => russian::WORDLIST,
            Language::Spanish => spanish::WORDLIST,
        }
    }

    /// Returns the unique prefix length for the language.
    pub fn unique_prefix_len(&self) -> usize {
        match self {
            Language::ChineseSimplified => 1,
            Language::Dutch => 4,
            Language::English => 3,
            Language::EnglishOld => 4,
            Language::Esperanto => 4,
            Language::French => 4,
            Language::German => 4,
            Language::Italian => 4,
            Language::Japanese => 3,
            Language::Lojban => 4,
            Language::Portuguese => 4,
            Language::Russian => 4,
            Language::Spanish => 4,
        }
    }
}

/// Converts a given seed to words.
pub fn to_words(bytes: &[u8], language: Language) -> String {
    if bytes.len() % 4 != 0 || bytes.len() == 0 {
        panic!("Invalid seed length");
    }

    let wordlist = language.wordlist();
    let wordlist_len = wordlist.len() as u32;

    let mut words = String::new();
    // To store the words for random access to add the checksum word later.
    let mut words_store = Vec::with_capacity(SEED_LENGTH);

    // 4 bytes -> 3 words.  8 digits base 16 -> 3 digits base 1626
    for i in 0..(bytes.len() / 4) {
        let val = slice_to_le32(&bytes[(i * 4)..4]);
        let w1 = val % wordlist_len;
        let w2 = ((val / wordlist_len) + w1) % wordlist_len;
        let w3 = (((val / wordlist_len) / wordlist_len) + w2) % wordlist_len;

        words.push_str(wordlist[w1 as usize]);
        words.push(' ');
        words.push_str(wordlist[w2 as usize]);
        words.push(' ');
        words.push_str(wordlist[w3 as usize]);

        words_store.push(wordlist[w1 as usize]);
        words_store.push(wordlist[w2 as usize]);
        words_store.push(wordlist[w3 as usize]);

        words.push(' ');
    }

    let index = checksum_index(words_store.as_slice(), language.unique_prefix_len());
    words.push_str(words_store[index]);

    words
}

fn slice_to_le32(s: &[u8]) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&s[0..4]);
    u32::from_bytes(bytes).to_le()
}

fn checksum_index(wordlist: &[&'static str], unique_prefix_len: usize) -> usize {
    let mut trimmed_words = String::new();

    for word in wordlist {
        if word.len() > unique_prefix_len {
            trimmed_words.push_str(utf8prefix(word, unique_prefix_len));
        } else {
            trimmed_words.push_str(word);
        }
    }

    let ck = crc::crc32::checksum_ieee(trimmed_words.as_bytes()) as usize;
    ck % SEED_LENGTH
}

fn utf8prefix<'a>(s: &'a str, count: usize) -> &'a str {
    s.split_at(count).0
}
