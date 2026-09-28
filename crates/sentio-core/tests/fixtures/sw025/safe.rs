use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ProcessData<'info> {
    pub authority: Signer<'info>,
}

pub fn process(_ctx: Context<ProcessData>, raw: Vec<u8>) -> Result<()> {
    let bytes: [u8; 8] = raw
        .try_into()
        .map_err(|_| error!(ErrorCode::InvalidInput))?;
    let amount = u64::from_le_bytes(bytes);
    msg!("amount: {}", amount);
    Ok(())
}

// Guarded: require! proves Some before the unwrap.
pub fn guarded_commission(commission: Option<u64>) -> Result<()> {
    require!(commission.is_some(), ErrorCode::MissingCommission);
    let c = commission.as_ref().unwrap();
    msg!("c: {}", c);
    Ok(())
}

// Guarded: the is_none early-exit dominates everything below it.
pub fn early_exit(rate: Option<u64>) -> Result<()> {
    if rate.is_none() || rate.unwrap() == 0 {
        return Ok(());
    }
    msg!("rate: {}", rate.unwrap());
    Ok(())
}

// Guarded: the unwrap sits in the else of an is_none check.
pub fn else_branch(seeds: Option<Vec<Vec<u8>>>) -> Result<()> {
    if seeds.is_none() {
        msg!("missing seeds");
    } else {
        msg!("seeds: {}", seeds.unwrap().len());
    }
    Ok(())
}

// Guarded: require! runs under the same condition as the unwrap.
pub fn conditional(commission_amount: u64, commission_account: Option<u64>) -> Result<()> {
    if commission_amount > 0 {
        require!(commission_account.is_some(), ErrorCode::MissingCommission);
    }
    if commission_amount > 0 {
        let c = commission_account.as_ref().unwrap();
        msg!("c: {}", c);
    }
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

// Guarded by the file's length require: index 6 < ACCOUNTS_LEN.
pub fn hook(accounts: &[u8]) -> Result<()> {
    let payer = accounts.get(6).unwrap();
    msg!("{:?}", payer);
    let last = accounts.last().unwrap();
    msg!("{:?}", last);
    Ok(())
}

#[error_code]
pub enum ErrorCode {
    InvalidInput,
    MissingCommission,
}
