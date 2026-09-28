use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProcessData<'info> {
    pub authority: Signer<'info>,
}

pub fn process(ctx: Context<ProcessData>, raw: Vec<u8>) -> Result<()> {
    // unwrap on user-supplied bytes — panics if slice is wrong length
    let amount = u64::from_le_bytes(raw.try_into().unwrap());
    msg!("amount: {}", amount);
    Ok(())
}

// No is_some proof before the unwrap — must stay flagged.
pub fn commission(config: Option<u64>) -> Result<()> {
    let c = config.as_ref().unwrap();
    msg!("c: {}", c);
    Ok(())
}

// The require proves Some, but normalize() receives &mut rate and may reset
// it — the proof cannot survive the call. Must stay flagged.
fn normalize(rate: &mut Option<u64>) {
    if rate.is_none() {
        *rate = Some(0);
    }
}

pub fn guarded_then_mutated(mut rate: Option<u64>) -> Result<()> {
    require!(rate.is_some(), ErrorCode::MissingCommission);
    normalize(&mut rate);
    let c = rate.as_ref().unwrap();
    msg!("c: {}", c);
    Ok(())
}

const ACCOUNTS_LEN: usize = 16;

pub fn swap(remaining: &[u8], offset: &mut usize) -> Result<()> {
    require!(
        remaining.len() >= *offset + ACCOUNTS_LEN,
        ErrorCode::InvalidAccountsLength
    );
    Ok(())
}

// Index 16 is not below ACCOUNTS_LEN — past the proven bound, must stay flagged.
pub fn hook(accounts: &[u8]) -> Result<()> {
    let payer = accounts.get(16).unwrap();
    msg!("{:?}", payer);
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    InvalidAccountsLength,
    MissingCommission,
}
