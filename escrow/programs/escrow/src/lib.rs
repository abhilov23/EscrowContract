pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("4aXcU9wS9RchysNjf8guNZhg4p5G82rREiPQjEh1Jt52");

#[program]
pub mod escrow {
    use super::*;
                    //finish these
    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64, 
        token_a_offer_amount: u64,
        token_b_wanted_amount: u64
    ) -> Result<()>{
       
    }

    pub fn take_offer(context : Context<TakeOffer>) -> Result<()>{
        
    }
}
