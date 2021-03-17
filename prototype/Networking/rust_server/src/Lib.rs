mod Lib {
    
    fn rsa_key_gen() -> (Int,Int) {
        // from : https://en.wikipedia.org/wiki/RSA_(cryptosystem)#Key_generation
        // 1. Choose two distinct prime numbers p and q.
        // 2. Compute n = pq.
        // 3. Compute λ(n), where λ is Carmichael's totient function
        // 4. Choose an integer e such that 1 < e < λ(n) 
        //      and gcd(e, λ(n)) = 1; that is, e and λ(n) are coprime
        // 5. Determine d as d ≡ e−1 (mod λ(n)); 
        //      that is, d is the modular multiplicative inverse of e modulo λ(n)
    }
}
