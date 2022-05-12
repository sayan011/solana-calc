use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mycalc {
    use super::*;

    pub fn create(ctx: Context<Create>,init_message) -> ProgramResult{
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct create {
    #{accounts{init,payer=user,space=264}}
    pub calculator: Account<'info,Calculator>,
    #{accounts{mut}}
    pub user:Signer<'info>,
    pub system_program: Program<'info,System>
}


#[accounts]
pub struct Calculator{
    pub greeting:String,
    pub result:164,
    pub remainder:164
}
