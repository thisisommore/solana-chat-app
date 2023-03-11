use anchor_lang::prelude::*;

declare_id!("4rr7RkKDYipoCNZCyrLYjP9zeEgLgFJW42GUmryZFnry");

#[program]
pub mod chat_app {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, encryption_key: String) -> Result<()> {
        let new_user = &mut ctx.accounts.user_account;
        new_user.encryption_key = encryption_key;
        Ok(())
    }

    pub fn put_message(ctx: Context<PutMessage>, content_hash: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message;
        message_account.content_hash = content_hash;
        message_account.sender = ctx.accounts.sender.key();
        message_account.receiver = ctx.accounts.receiver.key().to_owned();
        let clock = Clock::get()?;
        message_account.time = clock.unix_timestamp;
        Ok(())
    }
}

#[account]
pub struct Message {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub content_hash: String,
    pub time: i64,
}

// TODO recalculate all spaces
#[derive(Accounts)]
pub struct CreateUser<'info> {
    /// CHECK: max unsafe
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 174, seeds = [b"user", user.key().as_ref()], bump
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub public_key: Pubkey,
    pub encryption_key: String,
}

#[derive(Accounts)]
pub struct PutMessage<'info> {
    #[account(
        init,space=178,payer=sender
    )]
    pub message: Account<'info, Message>,
    /// CHECK: max unsafe
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: max unsafe
    pub receiver: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
