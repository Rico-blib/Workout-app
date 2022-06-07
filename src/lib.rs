use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};
near_sdk::setup_alloc!();
use std::collections::HashMap;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Workout {
    users_work_out: HashMap<String, i8>,
}

impl Default for Workout {
    fn default() -> Self {
        Self {
            users_work_out: HashMap::new(),
        }
    }
}

#[near_bindgen]
impl Workout {
    pub fn get_rate(&self) -> Option<&i8> {
        self.users_work_out.get(&env::signer_account_id())
    }

    // user work out intensity
    // set by the user
    pub fn create_workout(&mut self, rate: i8) {
        self.users_work_out.insert(env::signer_account_id(), rate);
    }

    // user utility function to update user workout
    // in hashmap doc if you insert a value with the same key the value is updated
    pub fn update_intensity(&mut self, rate: i8) {
        self.users_work_out.insert(env::signer_account_id(), rate);
    }

    // get a list of user work out
    pub fn get_workout(&self) -> Vector<String> {
        let account = env::signer_account_id();
        let intensity = self.users_work_out.get(&account);
        let mut wrk: Vector<String> = Vector::new(b"m");
        match intensity {
            Some(value) => match value {
                1..=10 => {
                    wrk.push(&String::from("15 push up"));
                    wrk.push(&String::from("next,50 jumps"));
                    wrk.push(&String::from("break for 1 minute then do 10 situps"));
                }
                11..=20 => {
                    wrk.push(&String::from("20 situps"));
                    wrk.push(&String::from("next,run for 30 minutes"));
                    wrk.push(&String::from("next frog jump for 5 minutes"));
                }
                21..=30 => {
                    wrk.push(&String::from("30 press ups"));
                    wrk.push(&String::from("next,duckwalk for 10 minutes"));
                    wrk.push(&String::from("next,jog for 5 minutes"));
                }
                31..=40 => {
                    wrk.push(&String::from("frog jump for 5 minutes"));
                    wrk.push(&String::from("next,jog for 10 minutes"));
                    wrk.push(&String::from("take a 1 minute break,do 20 pullups"));
                }

                _ => {
                    wrk.push(&String::from("run for 1 hour"));
                    wrk.push(&String::from("next,60 jumps"));
                    wrk.push(&String::from("frog jump for 20 minutes"));
                }
            },

            None => {
                env::log(b"You have not registered intensity, you get the default workouts");

                wrk.push(&String::from("20 push up"));
            }
        }

        return wrk;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    //set up a mock context
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    #[test]
    fn create_workout() {
        let context = get_context(vec![], false);
        testing_env!(context);
        //initiate  a contract variable
        let mut contract = Workout::default();
        contract.create_workout(12);

        let intensity = match contract.get_rate() {
            Some(k) => k,
            None => &0,
        };
        assert_eq!(&12, intensity);
    }

    #[test]
    fn update_workout() {
        let context = get_context(vec![], false);
        testing_env!(context);
        //initiate  a contract variable
        let mut contract = Workout::default();
        contract.create_workout(12);

        contract.update_intensity(15);

        let intensity = match contract.get_rate() {
            Some(k) => k,
            None => &0,
        };
        assert_eq!(&15, intensity);
    }

}
