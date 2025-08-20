use anchor_lang::prelude::*;

declare_id!("Ck1eb3NzGJ3XbsxiMcqPrijErRpJcWqz64w3RTSNcAdG");

#[program]
pub mod cpi_native {
    use super::*;

    pub fn initialize(ctx: Context<CpiContext>) -> Result<()> {
        
        let accounts = vec![
            AccountMeta::new(*ctx.accounts.payer.key, true),
            AccountMeta::new(*ctx.accounts.data_account.key, true),
            AccountMeta::new_readonly(anchor_lang::system_program::ID, false)

        ];

        let instruction = anchor_lang::solana_program::instruction::Instruction {
           
           program_id: ctx.accounts.cpi_program.key(),
           accounts: accounts,
           data: vec![0]
        };

        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[

                ctx.accounts.payer.to_account_info(),
                ctx.accounts.data_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ]
        )?;

        Ok(())
    }

    pub fn double(ctx: Context<CpiContext>) -> Result<()> {

        let accounts = vec![
            
            AccountMeta::new(*ctx.accounts.data_account.key, false ),

        ];

        let instruction = anchor_lang::solana_program::instruction::Instruction {
            program_id: ctx.accounts.cpi_program.key(),
            accounts: accounts,
            data: vec![1]
        };

        anchor_lang::solana_program::program::invoke(
            &instruction,
            &[
                ctx.accounts.data_account.to_account_info()
            ]
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiContext<'info> {
    #[account(mut)]
    pub data_account: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub cpi_program: AccountInfo<'info>,
}
