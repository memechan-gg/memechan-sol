use num_integer::Roots;
use spl_math::uint::U256;

pub fn multiply_until_overflow(nums: Vec<u128>) -> (u128, Vec<u128>) {
    fn helper(nums: &[u128], current_product: u128) -> (u128, Vec<u128>) {
        if nums.is_empty() {
            return (current_product, vec![]);
        }

        let next = nums[0];
        if let Some(result) = current_product.checked_mul(next) {
            helper(&nums[1..], result)
        } else {
            (current_product, nums.to_vec())
        }
    }

    if nums.is_empty() {
        return (1, vec![]);
    }

    helper(&nums, 1)
}

pub fn multiply_divide(mut numerators: Vec<U256>, mut denominators: Vec<U256>) -> Option<U256> {
    let mut result = U256::from(1);
    numerators.sort_by(|a, b| b.cmp(a));
    denominators.sort_by(|a, b| b.cmp(a));

    while numerators.len() > 0 {
        let numerator = numerators.last().unwrap(); // Safe to unwrap because it's not empty

        // Multiply if successful, pop multiplier from vector
        if let Some(product) = result.checked_mul(*numerator) {
            numerators.pop();
            result = product;
        } else {
            // If overflow occurs, pop divisor and divide
            if let Some(denominator) = denominators.pop() {
                if let Some(division) = result.checked_div(denominator) {
                    result = division;
                } else {
                    return None; // Return None if division is unsuccessful
                }
            } else {
                return None; // Return None if there are no more denominators
            }
        }
    }

    for denominator in denominators.drain(..) {
        if let Some(division) = result.checked_div(denominator) {
            result = division;
        } else {
            return None; // Return None if division is unsuccessful
        }
    }

    Some(result)
}

pub trait CheckedMath {
    fn checked_add(&self, num: u128) -> Self;

    fn checked_mul(&self, num: u128) -> Self;

    fn checked_sub(&self, num: u128) -> Self;

    fn checked_div(&self, num: u128) -> Self;

    fn checked_pow(&self, num: u32) -> Self;

    fn checked_add_(&self, num: Option<u128>) -> Self;

    fn checked_mul_(&self, num: Option<u128>) -> Self;

    fn checked_sub_(&self, num: Option<u128>) -> Self;

    fn checked_div_(&self, num: Option<u128>) -> Self;

    fn sqrt(&self) -> Self;
}

impl CheckedMath for Option<u128> {
    fn checked_add(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_add(num),
        }
    }

    fn checked_mul(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_mul(num),
        }
    }

    fn checked_sub(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_sub(num),
        }
    }

    fn checked_div(&self, num: u128) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_div(num),
        }
    }

    fn checked_pow(&self, num: u32) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_pow(num),
        }
    }

    fn checked_add_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_add(num__),
            },
        }
    }

    fn checked_sub_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_sub(num__),
            },
        }
    }

    fn checked_mul_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_mul(num__),
            },
        }
    }

    fn checked_div_(&self, num: Option<u128>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_div(num__),
            },
        }
    }

    fn sqrt(&self) -> Self {
        match self {
            None => None,
            Some(num_) => Some((*num_).sqrt()),
        }
    }
}

pub trait CheckedMath256 {
    fn checked_add(&self, num: U256) -> Self;

    fn checked_mul(&self, num: U256) -> Self;

    fn checked_sub(&self, num: U256) -> Self;

    fn checked_div(&self, num: U256) -> Self;

    fn checked_pow(&self, num: u32) -> Self;

    fn checked_add_(&self, num: Option<U256>) -> Self;

    fn checked_mul_(&self, num: Option<U256>) -> Self;

    fn checked_sub_(&self, num: Option<U256>) -> Self;

    fn checked_div_(&self, num: Option<U256>) -> Self;

    fn sqrt(&self) -> Self;
}

impl CheckedMath256 for Option<U256> {
    fn checked_add(&self, num: U256) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_add(num),
        }
    }

    fn checked_mul(&self, num: U256) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_mul(num),
        }
    }

    fn checked_sub(&self, num: U256) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_sub(num),
        }
    }

    fn checked_div(&self, num: U256) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_div(num),
        }
    }

    fn checked_pow(&self, num: u32) -> Self {
        match self {
            None => None,
            Some(num_) => num_.checked_pow(U256::from(num)),
        }
    }

    fn checked_add_(&self, num: Option<U256>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_add(num__),
            },
        }
    }

    fn checked_sub_(&self, num: Option<U256>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_sub(num__),
            },
        }
    }

    fn checked_mul_(&self, num: Option<U256>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_mul(num__),
            },
        }
    }

    fn checked_div_(&self, num: Option<U256>) -> Self {
        match self {
            None => None,
            Some(num_) => match num {
                None => None,
                Some(num__) => num_.checked_div(num__),
            },
        }
    }

    fn sqrt(&self) -> Self {
        match self {
            None => None,
            Some(num_) => Some((*num_).integer_sqrt()),
        }
    }
}
