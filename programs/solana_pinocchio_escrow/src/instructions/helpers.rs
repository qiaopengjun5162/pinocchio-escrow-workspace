use pinocchio::instruction::{Seed, Signer};
use pinocchio::program_error::ProgramError;
use pinocchio::{ProgramResult, account_info::AccountInfo};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::TokenAccount;

// --- 1. 签名者检查助手 ---
pub struct SignerAccount;
impl SignerAccount {
    #[inline(always)]
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}

// --- 2. 程序账户 (PDA/State) 助手 ---
pub struct ProgramAccount;
impl ProgramAccount {
    #[inline(always)]
    /// 初始化程序状态账户 (如 Escrow)
    pub fn init<T>(
        payer: &AccountInfo,       // 1. 支付租金的人
        new_account: &AccountInfo, // 2. 要创建的 PDA
        signer_seeds: &[Seed],     // 3. PDA 种子
        space: usize,              // 4. 空间大小
        lamports: u64,
    ) -> ProgramResult {
        // 计算租金 (这里假设你已经算好了 lamports，或者调用系统程序计算)
        // 最简单的做法是调用 pinocchio_system 的 CreateAccount
        CreateAccount {
            from: payer,
            to: new_account,
            lamports,
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&[Signer::from(signer_seeds)])
    }

    /// 检查该账户是否由本程序拥有
    #[inline(always)]
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if account.owner() != &crate::ID {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }

    /// 关闭账户并回收 Lamports (常用于 Refund/Take)
    pub fn close(account: &AccountInfo, destination: &AccountInfo) -> ProgramResult {
        // 将 lamports 转移给接收者
        let lamports = account.lamports();
        // 2. 手动转移 Lamports
        // 注意：在 Pinocchio 0.9.2 源码中，修改 lamports 需要通过 unsafe 的 unchecked 方法
        // 或者使用 try_borrow_mut_lamports (会增加 CU 开销)
        // 鉴于 Pinocchio 追求底层效率，这里使用源码提供的 unchecked 方式：
        unsafe {
            // 将原账户余额清零
            *account.borrow_mut_lamports_unchecked() = 0;
            // 将余额累加到接收者账户
            *destination.borrow_mut_lamports_unchecked() += lamports;
        }

        // 清理数据并将所有者重置为系统程序
        account.close()
    }
}

// --- 3. Mint (代币定义) 助手 ---
pub struct MintInterface;
impl MintInterface {
    #[inline(always)]
    pub fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        // SPL Token Mint 固定长度为 82
        // 且必须由 Token Program 拥有
        if account.data_len() != 82 {
            // SPL Token Mint 固定长度
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

// --- 4. 关联代币账户 (ATA) 助手 ---
pub struct AssociatedTokenAccount;
impl AssociatedTokenAccount {
    pub fn init(
        funding_account: &AccountInfo, // 1. 出钱的人 (Payer/Signer)
        account: &AccountInfo,         // 2. 要创建的 ATA 地址
        wallet: &AccountInfo,          // 3. 所有者 (Owner，即这个 ATA 归谁管)
        mint: &AccountInfo,            // 4. Mint
        system_program: &AccountInfo,  // 5. 系统程序
        token_program: &AccountInfo,   // 6. 代币程序
    ) -> ProgramResult {
        CreateIdempotent {
            funding_account,
            account,
            wallet,
            mint,
            system_program,
            token_program,
        }
        .invoke()
    }

    pub fn init_if_needed(
        account: &AccountInfo,
        mint: &AccountInfo,
        funding_account: &AccountInfo, // 教程传入的第3个参数
        wallet: &AccountInfo,
        system_program: &AccountInfo,
        token_program: &AccountInfo,
    ) -> ProgramResult {
        CreateIdempotent {
            funding_account,
            account,
            wallet,
            mint,
            system_program,
            token_program,
        }
        .invoke()
    }

    pub fn check(
        ata: &AccountInfo,
        owner: &AccountInfo,
        mint: &AccountInfo,
        _token_program: &AccountInfo,
    ) -> Result<(), ProgramError> {
        let token_account = TokenAccount::from_account_info(ata)?;
        if token_account.owner() != owner.key() || token_account.mint() != mint.key() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
