#MultiplicativeInverseusingEuclideanAlgorithm
The code in this repo finds the multiplicative inverse of any number entered by a user using the extended euclidean algorithm.

The Euclidean algorithm and Extended Euclidean algorithm are both used to find the GCD which we must find before finding the multiplicative inverse of a number. Only difference is that the extended Euclidean algorithm makes use of extra variables to arrive at its answer and is more suitable for computer programs.
The GCD is the Highest common factor which we were taught in elementary school mathematics. 
To find multiplicative inverse the gcd must be equal to 1. Which means that multiplicative inverse of “a modulo m” exists if and only if a and m are relatively prime. The modular multiplicative inverse is an integer ‘x’ such that:
a x ≅ 1 (mod m)
The value of x should be in { 1, 2, … m-1}, i.e., in the range of integer modulo m. A simple method is to try all numbers from 1 to m. For every number x, check if (a*x) % m is 1. But you would agree it’s ridiculous in practice because numbers could be really high so it makes sense to use a more efficient method.

This code makes use of the Extended Euclidean algorithm which accepts two integers ‘a’ and ‘b’, finds their gcd and also find ‘x’ and ‘y’ such that 
ax + by = gcd(a, b)
To find multiplicative inverse of ‘a’ under ‘m’, we put b = m in above formula. Since we know that a and m are relatively prime, we can put value of gcd as 1.
ax + my = 1
If we take modulo m on both sides, we get
ax + my ≅ 1 (mod m)
We can remove the second term on left side as ‘my (mod m)’ would always be 0 for an integer y.
ax  ≅ 1 (mod m)
As earlier stated ‘x’ that we get using the Extended Euclid algorithm is the multiplicative inverse of ‘a’



Finally, the relevance of this algorithm to cryptography which is important to Helicarriers as a company because it is a block chain centred company and the block chain makes use of cryptography to provide security is: Euclid and Extended Euclid algorithm are the best algorithms to solve the public key and private key in RSA.
