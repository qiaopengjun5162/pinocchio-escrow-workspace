/*
refund 指令允许创建者取消一个未完成的报价：

关闭托管 PDA，并将其租金 lamports 返还给创建者。

将代币 A 的全部余额从保险库转回创建者，然后关闭保险库账户。
 */

use std::slice;

use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
};
use pinocchio_token::{
    instructions::{CloseAccount, Transfer},
    state::TokenAccount,
};

use crate::{AssociatedTokenAccount, ESCROW_SEED, Escrow, ProgramAccount};

pub struct Refund<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub maker_ata_a: &'a AccountInfo,
    pub associated_token_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> Refund<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;
    pub fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
        // 使用简单的切片模式匹配来获取账户，性能最优
        let [
            maker,       // 1. Signer
            escrow,      // 2. Escrow PDA
            mint_a,      // 3. Mint A (Anchor 里的第三个账户)
            vault,       // 4. Vault (Token Account)
            maker_ata_a, // 5. Maker ATA
            associated_token_program,
            token_program,  // 6. Token Program
            system_program, // 7. System Program
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            maker,
            escrow,
            mint_a,
            vault,
            maker_ata_a,
            associated_token_program,
            token_program,
            system_program,
        })
    }

    pub fn process(&self) -> ProgramResult {
        AssociatedTokenAccount::init_if_needed(
            self.maker_ata_a,
            self.mint_a,
            self.maker,
            self.maker,
            self.system_program,
            self.token_program,
        )?;

        // 1. 获取 Escrow 数据视图 (零拷贝)
        let data = self.escrow.try_borrow_data()?;
        let escrow_state = Escrow::load(&data)?;

        // 2. 构造 PDA 签名
        let seed_bytes = escrow_state.seed.to_le_bytes();
        let seeds = [
            Seed::from(ESCROW_SEED),
            Seed::from(self.maker.key().as_ref()),
            Seed::from(&seed_bytes),
            Seed::from(&escrow_state.bump),
        ];
        let signer = Signer::from(&seeds);

        // 检查 amount 之前确保 vault 数据有效
        let amount = TokenAccount::from_account_info(self.vault)?.amount();

        if amount > 0 {
            // 执行转账: Vault (from) -> Maker ATA (to)
            // 必须确认识别到的 vault 账户的所有者是 escrow PDA
            Transfer {
                from: self.vault,
                to: self.maker_ata_a,
                authority: self.escrow, // 这里必须是 PDA
                amount,
            }
            .invoke_signed(slice::from_ref(&signer))?;
        }

        // 关闭 Vault 账户
        CloseAccount {
            account: self.vault,
            destination: self.maker,
            authority: self.escrow,
        }
        .invoke_signed(&[signer])?;

        drop(data); // 必须在 close escrow 前释放
        ProgramAccount::close(self.escrow, self.maker)?;

        Ok(())
    }
}
