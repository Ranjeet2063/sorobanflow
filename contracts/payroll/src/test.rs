#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::Client as TokenClient,
    token::StellarAssetClient,
    Address, Env,
};

// ============================================================
// Helper: build a fresh test environment with token + client
// ============================================================
struct TestCtx {
    env: Env,
    contract_id: Address,
    token: Address,
    sender: Address,
    recipient: Address,
}

impl TestCtx {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(SorobanFlowPayroll, ());
        let sender = Address::generate(&env);
        let recipient = Address::generate(&env);

        let token_admin = Address::generate(&env);
        let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
        let token_admin_client = StellarAssetClient::new(&env, &token_contract.address());
        token_admin_client.mint(&sender, &100_000);

        Self {
            env,
            contract_id,
            token: token_contract.address(),
            sender,
            recipient,
        }
    }

    fn client(&self) -> SorobanFlowPayrollClient<'_> {
        SorobanFlowPayrollClient::new(&self.env, &self.contract_id)
    }

    fn token_client(&self) -> TokenClient<'_> {
        TokenClient::new(&self.env, &self.token)
    }
}

// ============================================================
// Test 1: Basic stream creation
// ============================================================
#[test]
fn test_create_stream_returns_id_one() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );
    assert_eq!(id, 1, "First stream ID must be 1");
}

// ============================================================
// Test 2: Stream ID increments correctly
// ============================================================
#[test]
fn test_stream_id_increments() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id1 = client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &1_000, &0, &1_000);
    let id2 = client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &2_000, &0, &1_000);
    let id3 = client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &3_000, &0, &1_000);

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
}

// ============================================================
// Test 3: get_schedule returns correct initial state
// ============================================================
#[test]
fn test_get_schedule_initial_state() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    let sched = client.get_schedule(&id).unwrap();
    assert_eq!(sched.total_amount, 5_000);
    assert_eq!(sched.claimed_amount, 0);
    assert_eq!(sched.start_time, 1_000);
    assert_eq!(sched.stop_time, 2_000);
    assert_eq!(sched.sender, ctx.sender);
    assert_eq!(sched.recipient, ctx.recipient);
}

// ============================================================
// Test 4: Tokens are transferred from sender on creation
// ============================================================
#[test]
fn test_tokens_transferred_on_create() {
    let ctx = TestCtx::setup();
    let client = ctx.client();
    let tc = ctx.token_client();

    let initial_balance = tc.balance(&ctx.sender);
    client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &5_000, &1_000, &2_000);
    assert_eq!(tc.balance(&ctx.sender), initial_balance - 5_000);
}

// ============================================================
// Test 5: Claim at exactly 50% elapsed → 50% claimable
// ============================================================
#[test]
fn test_claim_at_midpoint() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(1_500); // 50% elapsed
    let claimed = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(claimed, 2_500);
    assert_eq!(ctx.token_client().balance(&ctx.recipient), 2_500);
}

// ============================================================
// Test 6: Claim at 100% elapsed → full remaining amount
// ============================================================
#[test]
fn test_claim_at_full_completion() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(2_500); // past stop_time
    let claimed = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(claimed, 5_000, "Full amount must be claimable after stop_time");
}

// ============================================================
// Test 7: Partial claim, then claim remainder
// ============================================================
#[test]
fn test_partial_then_full_claim() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(1_500);
    let first = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(first, 2_500);

    ctx.env.ledger().set_timestamp(3_000);
    let second = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(second, 2_500, "Remainder must be exactly the unclaimed portion");
    assert_eq!(ctx.token_client().balance(&ctx.recipient), 5_000);
}

// ============================================================
// Test 8: Claim before stream start → 0 claimable (expect panic)
// ============================================================
#[test]
#[should_panic(expected = "No funds available to claim")]
fn test_claim_before_start_panics() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(500); // before start
    client.claim_stream(&ctx.recipient, &id);
}

// ============================================================
// Test 9: Wrong recipient cannot claim
// ============================================================
#[test]
#[should_panic(expected = "Unauthorized recipient")]
fn test_wrong_recipient_panics() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(1_500);
    let impostor = Address::generate(&ctx.env);
    client.claim_stream(&impostor, &id);
}

// ============================================================
// Test 10: Zero amount stream creation panics
// ============================================================
#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_zero_amount_panics() {
    let ctx = TestCtx::setup();
    let client = ctx.client();
    client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &0, &1_000, &2_000);
}

// ============================================================
// Test 11: stop_time <= start_time panics
// ============================================================
#[test]
#[should_panic(expected = "Stop time must be after start time")]
fn test_invalid_time_range_panics() {
    let ctx = TestCtx::setup();
    let client = ctx.client();
    client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &1_000, &2_000, &1_000);
}

// ============================================================
// Test 12: get_schedule for nonexistent stream returns None
// ============================================================
#[test]
fn test_get_nonexistent_schedule_returns_none() {
    let ctx = TestCtx::setup();
    let client = ctx.client();
    let result = client.get_schedule(&999);
    assert!(result.is_none(), "Nonexistent stream must return None");
}

// ============================================================
// Test 13: Multiple independent streams coexist correctly
// ============================================================
#[test]
fn test_multiple_streams_independent() {
    let ctx = TestCtx::setup();
    let client = ctx.client();
    let recipient2 = Address::generate(&ctx.env);

    let id1 = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &4_000, &1_000, &2_000,
    );
    let id2 = client.create_stream(
        &ctx.sender, &recipient2, &ctx.token,
        &6_000, &1_000, &3_000,
    );

    ctx.env.ledger().set_timestamp(2_000); // Stream 1: 100%, Stream 2: 50%

    let c1 = client.claim_stream(&ctx.recipient, &id1);
    let c2 = client.claim_stream(&recipient2, &id2);

    assert_eq!(c1, 4_000);
    assert_eq!(c2, 3_000); // 50% of 6000
}

// ============================================================
// Test 14: Vesting linear invariant — claimable proportional to elapsed
// ============================================================
#[test]
fn test_vesting_linear_invariant() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    // Stream: 10,000 tokens over 10,000 seconds (1 token/sec)
    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &10_000, &0, &10_000,
    );

    // At 25%
    ctx.env.ledger().set_timestamp(2_500);
    let c1 = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(c1, 2_500);

    // At 50% (25% remaining unclaimed)
    ctx.env.ledger().set_timestamp(5_000);
    let c2 = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(c2, 2_500);

    // At 75%
    ctx.env.ledger().set_timestamp(7_500);
    let c3 = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(c3, 2_500);

    // After end
    ctx.env.ledger().set_timestamp(12_000);
    let c4 = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(c4, 2_500);

    // Total = 10,000 ✓
    assert_eq!(c1 + c2 + c3 + c4, 10_000);
}

// ============================================================
// Test 15: Claimed amount never exceeds total_amount (solvency invariant)
// ============================================================
#[test]
fn test_solvency_invariant_total_not_exceeded() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(1_200);
    let c1 = client.claim_stream(&ctx.recipient, &id);

    ctx.env.ledger().set_timestamp(1_600);
    let c2 = client.claim_stream(&ctx.recipient, &id);

    ctx.env.ledger().set_timestamp(2_500);
    let c3 = client.claim_stream(&ctx.recipient, &id);

    assert!(
        c1 + c2 + c3 <= 5_000,
        "Total claimed must never exceed total_amount: got {}",
        c1 + c2 + c3
    );
}

// ============================================================
// Test 16: Exact stop_time boundary — full amount claimable
// ============================================================
#[test]
fn test_claim_at_exact_stop_time() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(2_000); // exact stop_time
    let claimed = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(claimed, 5_000);
}

// ============================================================
// Test 17: Exact start_time — 0 claimable (expect panic)
// ============================================================
#[test]
#[should_panic(expected = "No funds available to claim")]
fn test_claim_at_exact_start_time_panics() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(1_000); // exactly start_time → 0 elapsed
    client.claim_stream(&ctx.recipient, &id);
}

// ============================================================
// Test 18: Large amount stream (overflow safety)
// ============================================================
#[test]
fn test_large_amount_stream() {
    let ctx = TestCtx::setup();
    let _tc = ctx.token_client();

    // Mint large amount
    let token_admin = Address::generate(&ctx.env);
    let big_token = ctx.env.register_stellar_asset_contract_v2(token_admin.clone());
    StellarAssetClient::new(&ctx.env, &big_token.address())
        .mint(&ctx.sender, &1_000_000_000);

    let client = ctx.client();
    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &big_token.address(),
        &1_000_000_000, &0, &1_000_000,
    );

    ctx.env.ledger().set_timestamp(500_000); // 50%
    let claimed = client.claim_stream(&ctx.recipient, &id);
    assert_eq!(claimed, 500_000_000);
}

// ============================================================
// Test 19: Consecutive double-claim after full vesting panics
// ============================================================
#[test]
#[should_panic(expected = "No funds available to claim")]
fn test_double_claim_after_full_vesting_panics() {
    let ctx = TestCtx::setup();
    let client = ctx.client();

    let id = client.create_stream(
        &ctx.sender, &ctx.recipient, &ctx.token,
        &5_000, &1_000, &2_000,
    );

    ctx.env.ledger().set_timestamp(3_000);
    client.claim_stream(&ctx.recipient, &id); // First full claim
    client.claim_stream(&ctx.recipient, &id); // Should panic
}

// ============================================================
// Test 20: Stream balance matches contract balance invariant
// ============================================================
#[test]
fn test_contract_holds_correct_balance() {
    let ctx = TestCtx::setup();
    let client = ctx.client();
    let tc = ctx.token_client();

    client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &3_000, &0, &1_000);
    client.create_stream(&ctx.sender, &ctx.recipient, &ctx.token, &2_000, &0, &1_000);

    // Contract should hold 5000 total
    assert_eq!(tc.balance(&ctx.contract_id), 5_000);
}
