use anchor_lang::prelude::*;

declare_id!("8pW94nJtpxwHmNp1A9hAmTihwQgvGMrp8J5v9ACRsEdC");

#[program]
pub mod chat_app {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, encryption_key: String)-> Result<()> {
        let new_user = &mut ctx.accounts.user_account;
        new_user.encryption_key=encryption_key;
        Ok(())
    }
    pub fn initialize(ctx: Context<Initialize>, content_hash: String) -> Result<()> {
        let new_chat = &mut ctx.accounts.chat;
        new_chat.participant_a = ctx.accounts.user.key.to_owned();
        new_chat.participant_b = ctx.accounts.receiver.key.to_owned();
        
        let new_message = Message {
            content_hash: content_hash,
            is_first: true,
        };
        new_chat.messages = vec![new_message];
        Ok(())
    }

    pub fn put_message(ctx: Context<PutMessage>, content_hash: String) -> Result<()> {
        let chat_account = &mut ctx.accounts.chat;
        let sender_pub_key = ctx.accounts.sender.key;
        if &chat_account.participant_a != sender_pub_key
            && &chat_account.participant_b != sender_pub_key
        {
            return err!(ErrorCode::NotParticipant);
        }

        let new_message = Message {
            content_hash: content_hash,
            is_first: &chat_account.participant_a==sender_pub_key,
        };

        if chat_account.messages.len()==0 {
            chat_account.messages.remove(0);
        }
        chat_account.messages.push(new_message);

        Ok(())
    }
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub struct Message {
    pub is_first: bool,
    pub content_hash: String,
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
pub struct Chat {
    pub participant_a: Pubkey,
    pub participant_b: Pubkey,
    pub messages: Vec<Message>,
}

#[account]
pub struct User {
    pub public_key:Pubkey,
    pub encryption_key:String
}

#[derive(Accounts)]
pub struct PutMessage<'info> {
    pub chat: Account<'info, Chat>,
    pub sender: Signer<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: max unsafe
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: max unsafe
    #[account()]
    pub receiver: UncheckedAccount<'info>,
    #[account(
        init,
        payer = user,
        space = 178, seeds = [b"user-chat", user.key().as_ref(), receiver.key().as_ref()], bump
    )]
    pub chat: Account<'info, Chat>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not participant")]
    NotParticipant,
}
