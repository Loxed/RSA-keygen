use rand::{Rng, thread_rng};

pub struct KeyPair {
    //string representation of the public key
    pub public_key: String,
    pub private_key: String,
}

pub trait KeyGeneration{
    //generate a key pair
    fn generate_key_pair() -> KeyPair;

    //check if number is prime
    fn is_prime(number: u64) -> bool;

    //generate a large prime number
    fn generate_large_prime_number() -> u64;

    //generate public exponent
    fn generate_public_exponent(phi: u64) -> u64;

    //generate private exponent
    fn generate_private_exponent(public_exponent: u64, phi: u64) -> u64;

    //check if number is coprime to phi
    fn is_coprime(number: u64, phi: u64) -> bool;


}

impl KeyGeneration for KeyPair {
    //generate_key_pair
    fn generate_key_pair() -> KeyPair {

        //generate 2 large random prime numbers
        let p = KeyPair::generate_large_prime_number();
        let q = KeyPair::generate_large_prime_number();

        //compute n = p * q
        let n = p.clone() * q.clone();

        //compute totient(n) = (p - 1) * (q - 1)
        let totient = (p.clone() - 1) * (q.clone() - 1);

        //generate public exponent e such that 1 < e < totient(n) and e is coprime to totient(n)
        let e = KeyPair::generate_public_exponent(totient.clone());

        //compute private exponent d such that d * e = 1 mod totient(n)
        let d = KeyPair::generate_private_exponent(totient, e.clone());

        //convert the key to string
        let public_key = format!("{},{}", n, e);
        let private_key = format!("{},{}", n, d);

        KeyPair {
            public_key,
            private_key
        }
    }

    //is_prime
    fn is_prime(number: u64) -> bool {
        match number {
            0 | 1 => false,
            2 => true,
            _ => {
                let mut i = 2;
                //check if number is divisible by any number less than itself
                while i * i <= number {
                    if number % i == 0 {
                        return false;
                    }
                    i += 1;
                }
                true
            }
        }
    }

    //generate_large_prime_number
    fn generate_large_prime_number() -> u64 {
        //generate a random number
        let mut number : u64 = thread_rng().gen_range(1000..10000);
        //check if number is prime
        while !KeyPair::is_prime(number) {
            number = thread_rng().gen_range(1000..10000);
        }
        number
    }

    //generate_public_exponent
    fn generate_public_exponent(phi: u64) -> u64 {
        //generate a random number

        let mut number : u64 = thread_rng().gen_range(2..phi.clone()) as u64;

        //check if number is coprime to phi
        while !KeyPair::is_coprime(number.clone(), phi.clone()) {
            number = thread_rng().gen_range(2..phi.clone()) as u64;
        }
        number
    }

    //generate_private_exponent
    fn generate_private_exponent(phi: u64, public_exponent: u64) -> u64 {
        //compute private exponent
        let mut d = 1;
        //check if d * e = 1 mod phi
        while (d * public_exponent) % phi != 1 {
            d += 1;
        }

        d
    }

    fn is_coprime(number: u64, phi: u64) -> bool {
        //if number is coprime to phi, then gcd(number, phi) = 1

        //compute gcd
        let mut a = number.clone();
        let mut b = phi.clone();

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        a == 1

    }
}

