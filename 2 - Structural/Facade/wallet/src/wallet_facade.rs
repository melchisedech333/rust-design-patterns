
/// Uma Facade pode ser uma implementação simples de uma classe, e seu intuito é construir
/// uma espécie de interface que funciona como uma fachada, de modo que assim seja possível
/// expor para o cliente somente os recursos de que ele necessita, e esconder do mesmo 
/// cliente toda a lógica completa por trás, lógica essa que seria por exemplo a interação
/// com um outro sistema/subsistema.
/// 
/// Note que neste exemplo ao criar uma conta, na verdade criamos vários recursos internamente,
/// realizando assim desde o início a comunicação com muitos outros sistemas. Do mesmo modo
/// ao realizar operações na conta, realizamos várias ações como gerar notificações, manipular
/// a própria conta, etc.

use crate::{
    account::Account, ledger::Ledger, notification::Notification, security_code::SecurityCode,
    wallet::Wallet,
};

/// Facade hides a complex logic behind the API.
pub struct WalletFacade {
    account: Account,
    wallet: Wallet,
    code: SecurityCode,
    notification: Notification,
    ledger: Ledger,
}

impl WalletFacade {
    pub fn new(account_id: String, code: u32) -> Self {
        println!("Starting create account");

        let this = Self {
            account: Account::new(account_id),
            wallet: Wallet::new(),
            code: SecurityCode::new(code),
            notification: Notification,
            ledger: Ledger,
        };

        println!("Account created");
        this
    }

    pub fn add_money_to_wallet(
        &mut self,
        account_id: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting add money to wallet");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.credit_balance(amount);
        self.notification.send_wallet_credit_notification();
        self.ledger.make_entry(account_id, "credit".into(), amount);
        Ok(())
    }

    pub fn deduct_money_from_wallet(
        &mut self,
        account_id: &String,
        security_code: u32,
        amount: u32,
    ) -> Result<(), String> {
        println!("Starting debit money from wallet");
        self.account.check(account_id)?;
        self.code.check(security_code)?;
        self.wallet.debit_balance(amount);
        self.notification.send_wallet_debit_notification();
        self.ledger.make_entry(account_id, "debit".into(), amount);
        Ok(())
    }
}


