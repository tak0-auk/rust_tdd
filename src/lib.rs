#[derive(Debug)]
pub struct Money {
    amount: u32,
    currency: &'static str
}

trait MoneyTrait {
     fn new(amount: u32) -> Money;
}

impl Money {
    pub fn times(&self, multiplier: u32) -> Money {
        Money {
            amount: self.amount * multiplier,
            currency: &self.currency
        }
    }

    pub fn equals(&self, object: Money) -> bool {
        self.amount == object.amount && self.currency == object.currency
    }

    pub fn doller (amount: u32) -> Money {
        Money{
            amount: amount,
            currency: "USD"
        }
    }

    pub fn franc (amount: u32) -> Money {
        Money{
            amount: amount,
            currency: "CHF"
        }
    }

    pub fn currency(&self) -> &'static str {
        self.currency
    }
}

#[derive(Debug)]
pub struct Dollar {
}

#[derive(Debug)]
pub struct Franc {
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equals() {
        assert!(Money::doller(5).equals(Money::doller(5)));
        assert!(!Money::doller(5).equals(Money::doller(6)));
    }

    #[test]
    fn currency() {
        assert_eq!("USD", Money::doller(1).currency);
        assert_eq!("CHF", Money::franc(1).currency);
    }


}