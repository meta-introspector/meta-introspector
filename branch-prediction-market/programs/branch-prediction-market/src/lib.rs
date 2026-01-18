use anchor_lang::prelude::*;

declare_id!("BranchPredictMarket11111111111111111111111");

#[program]
pub mod branch_prediction_market {
    use super::*;

    pub fn create_market(
        ctx: Context<CreateMarket>,
        branch_address: u64,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.authority = ctx.accounts.authority.key();
        market.target_program = Pubkey::from_str("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB").unwrap();
        market.branch_address = branch_address;
        market.start_time = start_time;
        market.end_time = end_time;
        market.total_yes_bets = 0;
        market.total_no_bets = 0;
        market.settled = false;
        market.outcome = None;
        Ok(())
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,
        amount: u64,
        prediction: bool,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let bet = &mut ctx.accounts.bet;
        
        require!(!market.settled, ErrorCode::MarketSettled);
        require!(Clock::get()?.unix_timestamp < market.end_time, ErrorCode::MarketClosed);
        
        // Transfer SOL from user to market
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &market.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                market.to_account_info(),
            ],
        )?;
        
        // Record bet
        bet.market = market.key();
        bet.user = ctx.accounts.user.key();
        bet.amount = amount;
        bet.prediction = prediction;
        bet.claimed = false;
        
        // Update market totals
        if prediction {
            market.total_yes_bets += amount;
        } else {
            market.total_no_bets += amount;
        }
        
        Ok(())
    }

    pub fn submit_report(
        ctx: Context<SubmitReport>,
        execution_count: u64,
    ) -> Result<()> {
        let market = &ctx.accounts.market;
        let report = &mut ctx.accounts.report;
        
        require!(market.settled == false, ErrorCode::AlreadySettled);
        require!(Clock::get()?.unix_timestamp > market.end_time, ErrorCode::MarketNotEnded);
        
        report.market = market.key();
        report.reporter = ctx.accounts.reporter.key();
        report.execution_count = execution_count;
        report.timestamp = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

    pub fn settle_market(ctx: Context<SettleMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        let report = &ctx.accounts.report;
        
        require!(!market.settled, ErrorCode::AlreadySettled);
        require!(Clock::get()?.unix_timestamp > market.end_time, ErrorCode::MarketNotEnded);
        
        // Simple oracle: execution_count > 0 means "yes"
        let outcome = report.execution_count > 0;
        market.outcome = Some(outcome);
        market.settled = true;
        
        Ok(())
    }

    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        let market = &ctx.accounts.market;
        let bet = &mut ctx.accounts.bet;
        
        require!(market.settled, ErrorCode::MarketNotSettled);
        require!(!bet.claimed, ErrorCode::AlreadyClaimed);
        require!(bet.user == ctx.accounts.user.key(), ErrorCode::Unauthorized);
        
        let outcome = market.outcome.unwrap();
        
        // Check if user won
        if bet.prediction == outcome {
            let total_pool = market.total_yes_bets + market.total_no_bets;
            let winning_pool = if outcome {
                market.total_yes_bets
            } else {
                market.total_no_bets
            };
            
            // Payout = (user_bet / winning_pool) * total_pool
            let payout = (bet.amount as u128)
                .checked_mul(total_pool as u128)
                .unwrap()
                .checked_div(winning_pool as u128)
                .unwrap() as u64;
            
            // Transfer winnings
            **market.to_account_info().try_borrow_mut_lamports()? -= payout;
            **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += payout;
            
            bet.claimed = true;
        }
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(init, payer = authority, space = 8 + 200)]
    pub market: Account<'info, PredictionMarket>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub market: Account<'info, PredictionMarket>,
    #[account(init, payer = user, space = 8 + 100)]
    pub bet: Account<'info, UserBet>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitReport<'info> {
    pub market: Account<'info, PredictionMarket>,
    #[account(init, payer = reporter, space = 8 + 100)]
    pub report: Account<'info, OracleReport>,
    #[account(mut)]
    pub reporter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleMarket<'info> {
    #[account(mut)]
    pub market: Account<'info, PredictionMarket>,
    pub report: Account<'info, OracleReport>,
}

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub market: Account<'info, PredictionMarket>,
    #[account(mut)]
    pub bet: Account<'info, UserBet>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct PredictionMarket {
    pub authority: Pubkey,
    pub target_program: Pubkey,
    pub branch_address: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_yes_bets: u64,
    pub total_no_bets: u64,
    pub settled: bool,
    pub outcome: Option<bool>,
}

#[account]
pub struct UserBet {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub prediction: bool,
    pub claimed: bool,
}

#[account]
pub struct OracleReport {
    pub market: Pubkey,
    pub reporter: Pubkey,
    pub execution_count: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Market already settled")]
    AlreadySettled,
    #[msg("Market not settled yet")]
    MarketNotSettled,
    #[msg("Market has not ended yet")]
    MarketNotEnded,
    #[msg("Market is closed for betting")]
    MarketClosed,
    #[msg("Market already settled")]
    MarketSettled,
    #[msg("Winnings already claimed")]
    AlreadyClaimed,
    #[msg("Unauthorized")]
    Unauthorized,
}
