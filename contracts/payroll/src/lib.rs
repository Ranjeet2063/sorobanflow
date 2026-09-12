#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamSchedule {
    pub sender: Address,
    pub recipient: Address,
    pub token: Address,
    pub total_amount: i128,
    pub start_time: u64,
    pub stop_time: u64,
    pub claimed_amount: i128,
}

#[contracttype]
pub enum DataKey {
    Schedule(u64),
    ScheduleCount,
}

#[contract]
pub struct SorobanFlowPayroll;

#[contractimpl]
impl SorobanFlowPayroll {
    pub fn create_stream(
        env: Env,
        sender: Address,
        recipient: Address,
        token: Address,
        total_amount: i128,
        start_time: u64,
        stop_time: u64,
    ) -> u64 {
        sender.require_auth();
        assert!(total_amount > 0, "Amount must be positive");
        assert!(stop_time > start_time, "Stop time must be after start time");

        let client = token::Client::new(&env, &token);
        client.transfer(&sender, &env.current_contract_address(), &total_amount);

        let mut count: u64 = env.storage().instance().get(&DataKey::ScheduleCount).unwrap_or(0);
        count += 1;

        let schedule = StreamSchedule {
            sender,
            recipient,
            token,
            total_amount,
            start_time,
            stop_time,
            claimed_amount: 0,
        };

        env.storage().instance().set(&DataKey::Schedule(count), &schedule);
        env.storage().instance().set(&DataKey::ScheduleCount, &count);

        count
    }

    pub fn get_schedule(env: Env, stream_id: u64) -> Option<StreamSchedule> {
        env.storage().instance().get(&DataKey::Schedule(stream_id))
    }
}
