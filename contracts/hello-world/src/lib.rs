#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Address, Env};

#[contract]
pub struct StakingContract;

#[contracttype]
#[derive(Clone)]
pub struct Stake {
    pub amount: u64,
    pub start_time: u64,
    pub lock_period: u64,
    pub interest_rate: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyStaked = 1,
    NoStake = 2,
    StillLocked = 3,
    Unauthorized = 4,
}

const INTEREST_RATE: u64 = 2;
const PENALTY_RATE: u64 = 10;

#[contractimpl]
impl StakingContract {
    
    pub fn create_stake(env: Env, user: Address, amount: u64, lock_period: u64) -> Result<(), Error> {
        user.require_auth();

        
        let key = user.clone();
        if env.storage().instance().has(&key) {
            return Err(Error::AlreadyStaked);
        }

        let current_time = env.ledger().timestamp();
        let interest = (amount * INTEREST_RATE) / 100;
        let total_amount = amount + interest;

        let stake = Stake {
            amount: total_amount,
            start_time: current_time,
            lock_period,
            interest_rate: INTEREST_RATE,
        };

        env.storage().instance().set(&key, &stake);

        Ok(())
    }

    pub fn withdraw(env: Env, user: Address) -> Result<u64, Error> {
        user.require_auth();

        let key = user.clone();
        if !env.storage().instance().has(&key) {
            return Err(Error::NoStake);
        }

        let stake: Stake = env.storage().instance().get(&key).unwrap();
        let current_time = env.ledger().timestamp();
        let unlock_time = stake.start_time + stake.lock_period;

       
        if current_time < unlock_time {
            return Err(Error::StillLocked);
        }
        env.storage().instance().remove(&key);

        Ok(stake.amount)
    }

   
    pub fn emergency_withdraw(env: Env, user: Address) -> Result<u64, Error> {
        user.require_auth();

        let key = user.clone();

       
        if !env.storage().instance().has(&key) {
            return Err(Error::NoStake);
        }

        let stake: Stake = env.storage().instance().get(&key).unwrap();

      
        let penalty = (stake.amount * PENALTY_RATE) / 100;
        let amount_after_penalty = stake.amount - penalty;

       
        env.storage().instance().remove(&key);

        Ok(amount_after_penalty)
    }

    pub fn get_stake(env: Env, user: Address) -> Option<Stake> {
        let key = user;
        env.storage().instance().get(&key)
    }
}

mod test;
