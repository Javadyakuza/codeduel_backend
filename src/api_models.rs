use diesel::prelude::*;
// use merge_derivable;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

pub const OPEN_UNSOLVED: i32 = 1;
pub const OPEN_SOLVED: i32 = 2;
pub const CLOSED_UNSOLVED: i32 = 1;
pub const CLOSED_SOLVED: i32 = 1;

// ------------------------------- general models ----------------------------
// the following models will represent the actual schema of the tables in terms of the rust structs.

pub struct Wallets {
    pub user_id: i32,
    pub sol_addr: String,
}

pub struct Responses {
    pub response_id: i32,
    pub daredevil_id: i32,
    pub question_id: i32,
    pub response_code: String,
    pub correctness: bool,
    pub creation_time: NaiveDateTime,
}

pub struct Questions {
    pub question_id: i32,
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    pub creation_time: NaiveDateTime,
    pub deadline: NaiveDateTime,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub reward: i32,
    pub prize_pool: i32,
    pub entrance_fee: i32,
    pub category: String,
}

pub struct TestCases {
    pub test_case_id: i32,
    pub question_id: i32,
    pub test_inputs: String,  // stringified json
    pub test_outputs: String, // stringified json
}

pub struct Users {
    pub user_id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub total_payed: i32,
    pub total_claimed: i32,
    pub total_unclaimed: i32,
}

// ------------------------------- insertable models ----------------------------
// the following models are not containing the primary key fields, making able the fns to insert the values without the pks.

pub struct IResponses {
    // pub response_id: i32, // pk
    pub daredevil_id: i32,
    pub question_id: i32,
    pub response_code: String,
    pub correctness: bool,
    pub creation_time: NaiveDateTime,
}

pub struct IQuestions {
    // pub question_id: i32, // pk
    pub rival_id: i32,
    pub question_title: String,
    pub question_body: String,
    // @dev because the deadlines and the creation times are set in the same timezone they are comparable and no conflict is expected
    // pub creation_time: NaiveDateTime, // will be the default value defined in the table
    pub deadline: NaiveDateTime,
    pub question_status: i32,
    pub daredevil: Option<i32>,
    pub category: String,
    pub reward: i32,
    pub entrance_fee: i32,
}

pub struct ITestCases {
    // pub test_case_id: i32, // pk
    pub question_id: i32,
    pub test_inputs: String,
    pub test_outputs: String,
}

pub struct IUsers {
    // pub user_id: i32, // pk
    pub email: String,
    pub username: String,
    pub password: String,
}

// ------------------------------- queryable models ----------------------------
// the following models will provide the dynamic inputs for the getter functions

pub struct QResponses {
    pub response_id: Option<i32>,
    pub question_id: Option<i32>,
    pub daredevil_id: Option<i32>,
    // mod 1 = Some, None, None => search by the response id.
    // mod 2 = None, Some, Some => search by the daredevil and related question id.
    // mod 3 = None, None, Some => get all of the responses of the daredevil.
}

impl QResponses {
    pub fn is_correct_structures(instance: &QResponses) -> i32 {
        match (
            instance.question_id.is_some(),
            !instance.response_id.is_some(),
            !instance.daredevil_id.is_some(),
        ) {
            // Case 1: All fields are present
            (true, false, false) => 1,

            // Case 2: Question ID and Daredevil ID are present
            (false, true, true) => 2,

            // Case 3: Response ID and Daredevil ID are present
            (false, false, true) => 3,

            // None of the fields are present
            _ => 0,
        }
    }
}

#[derive(Clone)]
pub enum Categories {
    All,
    SolanaPrograms,
    Rust,
}
impl Categories {
    pub fn to_string(category: Option<&Self>) -> String {
        match category {
            Some(cat) => match cat {
                Self::All => "All".to_string(),
                Self::Rust => "Rust".to_string(),
                Self::SolanaPrograms => "SolanaPrograms".to_string(),
            },
            _ => "All".to_string(),
        }
    }
}
pub struct QQuestions<'a> {
    pub question_id: Option<i32>,
    pub question_title: Option<&'a str>,
    pub rival_id: Option<i32>,
    pub question_category: Option<Categories>,
    // mod 1 = Some, None, None, None => search by the question id.
    // mod 2 = None, Some, Some, None => search by the rival and related question title.
    // mod 3 = None, None, Some, None => get all of the rival questions
    // mod 4 = None, None, None, Some => get all questions or a certain category of the questions.
}
impl QQuestions<'_> {
    pub fn is_correct_structures(instance: &QQuestions) -> i32 {
        match (
            instance.question_id.is_some(),
            !instance.question_title.is_some(),
            !instance.rival_id.is_some(),
            !instance.question_category.is_some(),
        ) {
            // Case 1: All fields are present
            (true, false, false, false) => 1,

            // Case 2: Question ID, Rival ID, and Question Category are present
            (false, true, true, false) => 2,

            // Case 3: Question Title, Rival ID, and Question Category are present
            (false, false, true, false) => 3,

            // Case 4: Question Title and Rival ID are present
            (false, false, false, true) => 4,

            // None of the fields are present
            _ => 0,
        }
    }
}

// ------------------------------- updatable models ----------------------------
// the following models will provide the dynamic inputs for the getter functions
pub struct UUser<'a> {
    pub old_username_or_id: &'a str,
    pub new_email: &'a str,
    pub new_password: &'a str,
    pub new_username: &'a str,
    pub new_total_payed: i32,
    pub new_total_claimed: i32,
    pub new_total_unclaimed: i32,
    pub editor: &'a str,
}

// @notice "fetched from fe" means the same old values will be fetched and sent to backend by the front end application
pub struct UQuestion<'a> {
    pub editor: &'a str,
    pub rival_id: &'a str,
    pub old_question_title: &'a str, // fetched from fe
    pub question_title: &'a str,     // fetched from fe
    pub question_body: &'a str,      // fetched from fe
    pub deadline: &'a str,           // checked in the backend
    pub question_status: i32,        // fetched from fe
    pub daredevil: i32,              // fetched from fe, zero is considered as no daredevil
    pub prize_pool: i32,             // fetched from fe
    pub category: &'a str,           // fetched from fe
    pub test_inputs: &'a str,        // if empty will not be updated
    pub test_outputs: &'a str,       // if empty will not be updated
}

pub struct UWallets<'a> {
    pub editor: &'a str,
    pub username_or_id: &'a str,
    pub new_sol_addr: &'a str,
}

// ---------------------------------------------------------- removable models ---------------------------------------------------------
// the following models will provide the simple editor user id and certain unique single input for removing the values from the database
pub struct RUsers<'a> {
    pub remover: &'a str,
    pub username_or_id: &'a str,
}

pub struct RQuestions<'a> {
    pub remover: &'a str,
    pub rival_id: &'a str,
    pub question_title: &'a str, // fetched from fe
}
