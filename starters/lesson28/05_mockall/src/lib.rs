// ╔══════════════════════════════════════════════════════════════════╗
// ║  QUEST 28-5 · The Puppeteer                             175 XP   ║
// ╚══════════════════════════════════════════════════════════════════╝
//
// 📖 LEARN BY EXAMPLE
//
//   A hand-written fake (last quest) is often enough. When you need to
//   check HOW your code talks to a dependency (which calls, how often,
//   with what), `mockall` writes a MOCK for you:
//
//     #[cfg_attr(test, mockall::automock)]   // makes `MockBank` in tests only
//     pub trait Bank {
//         fn balance(&self, account: u32) -> u32;
//     }
//
//     let mut bank = MockBank::new();
//     bank.expect_balance()
//         .with(eq(7))              // only for account 7
//         .times(1)                 // exactly once
//         .return_const(100u32);    // answer 100
//     bank.expect_withdraw().never();
//
//   A call nobody expected, or the wrong count, fails the test with a
//   clear message.
//
//   ⚠️ Mocks test the conversation, not the result. Use them for calls
//   that matter (charging money, sending mail), not for every getter, or
//   your tests break on every refactor.
//
// 🎯 YOUR TASK
//
//   Paying the party talks to the bank far too much. Rewrite `pay_party`:
//     - no wages: Ok(0) without calling the bank at all
//     - ask for the balance ONCE
//     - not enough: Err("not enough gold: need {total}, have {balance}")
//       and withdraw nothing
//     - else withdraw the total in ONE call and return it
//     - a withdraw error is passed on as it is

#[cfg_attr(test, mockall::automock)]
pub trait Bank {
    fn balance(&self, account: u32) -> u32;
    fn withdraw(&mut self, account: u32, amount: u32) -> Result<(), String>;
}

pub fn pay_party(bank: &mut impl Bank, account: u32, wages: &[u32]) -> Result<u32, String> {
    let mut total = 0;
    for &wage in wages {
        let have = bank.balance(account);
        if have < wage {
            return Err(format!("not enough gold: need {wage}, have {have}"));
        }
        bank.withdraw(account, wage)?;
        total += wage;
    }
    Ok(total)
}

// ─── Tests: don't change these! ──────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;

    #[test]
    fn pays_in_one_go() {
        let mut bank = MockBank::new();
        bank.expect_balance().with(eq(7)).times(1).return_const(100u32);
        bank.expect_withdraw().with(eq(7), eq(60)).times(1).returning(|_, _| Ok(()));
        assert_eq!(pay_party(&mut bank, 7, &[10, 20, 30]), Ok(60));
    }

    #[test]
    fn refuses_when_poor() {
        let mut bank = MockBank::new();
        bank.expect_balance().times(1).return_const(50u32);
        bank.expect_withdraw().never();
        assert_eq!(pay_party(&mut bank, 7, &[40, 20]), Err("not enough gold: need 60, have 50".to_string()));
    }

    #[test]
    fn nobody_to_pay() {
        let mut bank = MockBank::new(); // no expectations: any call fails the test
        assert_eq!(pay_party(&mut bank, 7, &[]), Ok(0));
    }

    #[test]
    fn bank_errors_pass_through() {
        let mut bank = MockBank::new();
        bank.expect_balance().return_const(100u32);
        bank.expect_withdraw().returning(|_, _| Err("the vault is jammed".to_string()));
        assert_eq!(pay_party(&mut bank, 7, &[5]), Err("the vault is jammed".to_string()));
    }
}
