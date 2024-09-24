use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("JDssPQ6Xd5fr7rpLssbHdqxZ5rdRAwF7x2u3jxn2k924");


#[program]
mod game_records {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let records = &mut ctx.accounts.records;
        records.player_records = Vec::new(); // 初始化为空数组
        Ok(())
    }

    pub fn add_record(ctx: Context<AddRecord>, round: u32, position: u32, amount: u32, coinid: u32, win: i32) -> Result<()> {
        let records = &mut ctx.accounts.records;

        msg!("new rec -> R:{:?}, out:{:?}, amount:{:?}, coin:{:?}, win:{:?}", round, position,amount,coinid,win); 
        
        // 记录玩家的记录
        records.player_records.push(PlayerRecord {
            player: ctx.accounts.signer.key(), // 使用签名者的公钥
            round,
            position,
            amount,
            coinid,
            win
        });

        Ok(())
    }

    pub fn get_records(ctx: Context<GetRecords>) -> Result<Vec<PlayerRecord>> {
        let records = &ctx.accounts.records;
        Ok(records.player_records.clone())
    }
}

#[account]
pub struct GameRecords {
    pub player_records: Vec<PlayerRecord>, // 存储玩家记录的数组
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PlayerRecord {
    pub player: Pubkey, // 玩家地址
    pub round: u32,     // 回合数
    pub position: u32,  // 玩家在回合中的cash-out位置(应除以100)
    pub amount: u32,    // 玩家在回合中下注量
    pub coinid: u32,    // Coinid
    pub win: i32,       // winlose
}


// 账户上下文
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4 * 100)] // 适当的空间分配
    pub records: Account<'info, GameRecords>, // 记录账户
    #[account(mut)]
    pub signer: Signer<'info>, // 平台方的签名者
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddRecord<'info> {
    #[account(mut)]
    pub records: Account<'info, GameRecords>, // 记录账户
    pub signer: Signer<'info>, // 平台方的签名者
}

#[derive(Accounts)]
pub struct GetRecords<'info> {
    pub records: Account<'info, GameRecords>, // 记录账户
}
