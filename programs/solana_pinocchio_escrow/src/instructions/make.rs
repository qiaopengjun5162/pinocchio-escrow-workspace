/*
    1. 初始化托管记录并存储所有交易条款。

    2. 创建金库（一个由 mint_a 拥有的 escrow 的关联代币账户 (ATA)）。

    3. 使用 CPI 调用 SPL-Token 程序，将创建者的 Token A 转移到该金库中。
*/

use pinocchio::{
    ProgramResult, account_info::AccountInfo, instruction::Seed, program_error::ProgramError,
    pubkey::find_program_address,
};
use pinocchio_token::instructions::Transfer;

use crate::{
    AssociatedTokenAccount, ESCROW_SEED, Escrow, MintInterface, ProgramAccount, SignerAccount,
};

pub struct MakeAccounts<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub maker_ata_a: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for MakeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
            _,
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Basic Accounts Checks
        SignerAccount::check(maker)?;
        MintInterface::check(mint_a)?;
        MintInterface::check(mint_b)?;
        AssociatedTokenAccount::check(maker_ata_a, maker, mint_a, token_program)?;

        // Return the accounts
        Ok(Self {
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })
    }
}

pub struct MakeInstructionData {
    pub seed: u64,
    pub receive: u64,
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for MakeInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());

        // Instruction Checks
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            seed,
            receive,
            amount,
        })
    }
}

pub struct Make<'a> {
    pub accounts: MakeAccounts<'a>,
    pub instruction_data: MakeInstructionData,
    pub bump: u8,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Make<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = MakeAccounts::try_from(accounts)?;
        let instruction_data = MakeInstructionData::try_from(data)?;

        // Initialize the Accounts needed
        let (_, bump) = find_program_address(
            &[
                ESCROW_SEED,
                accounts.maker.key().as_ref(),
                &instruction_data.seed.to_le_bytes(),
            ],
            &crate::ID,
        );

        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }
}

impl<'a> Make<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        // --- 1. 准备种子 (用于 init PDA) ---
        let seed_binding = self.instruction_data.seed.to_le_bytes();
        let bump_binding = [self.bump];
        let escrow_seeds = [
            Seed::from(ESCROW_SEED),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];

        // --- 2. 创建 Escrow PDA ---
        ProgramAccount::init::<Escrow>(
            self.accounts.maker,  // Payer (出钱签名的人)
            self.accounts.escrow, // 要创建的 PDA
            &escrow_seeds,
            Escrow::LEN,
            2_000_000,
        )?;

        // --- 3. 创建 Vault (ATA) ---
        // Initialize the vault
        AssociatedTokenAccount::init(
            self.accounts.maker,  // 1. funding_account -> 传 Maker (他是 Signer，付钱)
            self.accounts.vault,  // 2. account         -> 传 Vault (这就是要创建的 ATA)
            self.accounts.escrow, // 3. wallet          -> 传 Escrow (它是 PDA，作为金库的主人)
            self.accounts.mint_a, // 4. mint            -> 传 MintA
            self.accounts.system_program,
            self.accounts.token_program,
        )?;

        // --- 4. 填充数据 ---
        // Populate the escrow account
        let mut data = self.accounts.escrow.try_borrow_mut_data()?;
        let escrow = Escrow::load_mut(data.as_mut())?;

        escrow.set_inner(
            self.instruction_data.seed,
            *self.accounts.maker.key(),
            *self.accounts.mint_a.key(),
            *self.accounts.mint_b.key(),
            self.instruction_data.receive,
            [self.bump],
        );

        // Transfer tokens to vault
        Transfer {
            from: self.accounts.maker_ata_a,
            to: self.accounts.vault,
            authority: self.accounts.maker,
            amount: self.instruction_data.amount,
        }
        .invoke()?;

        Ok(())
    }
}
