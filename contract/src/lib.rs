use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[ext_contract(ext_social_credit_check)]
pub trait SocialCreditContract {
    fn person_on_check_score(&self, applicant_id: String);
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct JobApplications {
    // applicants are keys, statuses are values
    application_state: HashMap<String, String>,
}

#[near_bindgen]
impl JobApplications {

    pub fn apply(&mut self) {

        let applicant = env::signer_account_id();

        if !self.application_state.contains_key(&applicant) {
            self.application_state.insert(applicant, "NEW".to_string());
            ext_social_credit_check::person_on_check_score(
                env::signer_account_id().to_string(), // voter AccountId
                &"social-credit.isonar.testnet".to_string(),
                0,             // attached yocto NEAR
                80000000000000             // attached gas
            );
        } else {
            env::log("No double applications!".as_bytes());
        }
    }

    pub fn job_applications_on_background_checked(&mut self, person: String, score: f32) {
        let applicant = person;
        let approved: String  = "APPROVED".to_string();
        let rejected: String = "REJECTED".to_string();

        if score < 0.5 {
            env::log("rejected".as_bytes());
            *self.application_state.get_mut(&applicant).unwrap() = rejected;
        } else {
            env::log("approved".as_bytes());
            *self.application_state.get_mut(&applicant).unwrap() = approved;
        }
        env::log(score.to_string().as_bytes());
    }

    pub fn my_application(self, account_id: String) -> String {

        match self.application_state.get(&account_id) {
            Some(status) => {
                status.to_string()
            }
            None => "NOT_FOUND".to_string()
        }
    }
}