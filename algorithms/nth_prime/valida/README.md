# nth-prime

- To run the tests, run:

```bash
# checkout into valida directory with all the binaries
./clang -c -target delendum nth_prime.c -o nth_prime.o
./ld.lld --script=valida.ld -o nth_prime.out nth_prime.o
./valida run nth_prime.out nth_prime.log
echo "Generating Proof..."
time ./valida prove nth_prime.out nth_prime.proof
ls -laSh nth_prime.proof

echo "Verifying Proof..."
time ./valida verify nth_prime.out nth_prime.proof
```
