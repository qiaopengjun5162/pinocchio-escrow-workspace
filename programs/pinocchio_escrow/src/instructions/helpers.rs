use pinocchio::cpi::{Seed, Signer};
use pinocchio::error::ProgramError;
use pinocchio::{AccountView, ProgramResult};
use pinocchio_associated_token_account::instructions::CreateIdempotent;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::state::TokenAccount;

// --- 1. 签名者检查助手 ---
pub struct SignerAccount;
impl SignerAccount {
    #[inline(always)]
    pub fn check(account: &AccountView) -> Result<(), ProgramError> {
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
        payer: &AccountView,       // 1. 支付租金的人
        new_account: &AccountView, // 2. 要创建的 PDA
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
    pub fn check(account: &AccountView) -> Result<(), ProgramError> {
        if !account.owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }

    /// 关闭账户并回收 Lamports (常用于 Refund/Take)
    pub fn close(account: &AccountView, destination: &AccountView) -> ProgramResult {
        // 将 lamports 转移给接收者
        let lamports = account.lamports();
        account.set_lamports(0);
        destination.set_lamports(destination.lamports() + lamports);

        // 清理数据并将所有者重置为系统程序
        account.close()
    }
}

// --- 3. Mint (代币定义) 助手 ---
pub struct MintInterface;
impl MintInterface {
    #[inline(always)]
    pub fn check(account: &AccountView) -> Result<(), ProgramError> {
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
        account: &AccountView,         // 1. 要创建的 ATA
        mint: &AccountView,            // 2. Mint
        funding_account: &AccountView, // 3. 所有者 (Owner)
        wallet: &AccountView,          // 4. 出钱的人 (Payer)
        system_program: &AccountView,  // 5. 系统程序
        token_program: &AccountView,   // 6. 代币程序
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
        account: &AccountView,
        mint: &AccountView,
        funding_account: &AccountView, // 教程传入的第3个参数
        wallet: &AccountView,
        system_program: &AccountView,
        token_program: &AccountView,
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
        ata: &AccountView,
        owner: &AccountView,
        mint: &AccountView,
        _token_program: &AccountView,
    ) -> Result<(), ProgramError> {
        let token_account = TokenAccount::from_account_view(ata)?;
        if token_account.owner() != owner.address() || token_account.mint() != mint.address() {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
