#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

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

    pub fn claim_stream(env: Env, recipient: Address, stream_id: u64) -> i128 {
        recipient.require_auth();
        let mut schedule: StreamSchedule = env
            .storage()
            .instance()
            .get(&DataKey::Schedule(stream_id))
            .expect("Stream not found");

        assert_eq!(schedule.recipient, recipient, "Unauthorized recipient");

        let now = env.ledger().timestamp();
        let claimable = if now >= schedule.stop_time {
            schedule.total_amount - schedule.claimed_amount
        } else if now <= schedule.start_time {
            0
        } else {
            let elapsed = (now - schedule.start_time) as i128;
            let total_duration = (schedule.stop_time - schedule.start_time) as i128;
            let vested = (schedule.total_amount * elapsed) / total_duration;
            vested - schedule.claimed_amount
        };

        assert!(claimable > 0, "No funds available to claim");

        schedule.claimed_amount += claimable;
        env.storage().instance().set(&DataKey::Schedule(stream_id), &schedule);

        let client = token::Client::new(&env, &schedule.token);
        client.transfer(&env.current_contract_address(), &recipient, &claimable);

        claimable
    }

    pub fn get_schedule(env: Env, stream_id: u64) -> Option<StreamSchedule> {
        env.storage().instance().get(&DataKey::Schedule(stream_id))
    }
}

#[cfg(test)]
mod test;
