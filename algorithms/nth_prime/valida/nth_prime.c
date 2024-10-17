int nth_prime(int n)
{
    int i = 2;
    int count = 0;
    while (1)
    {
        int j = 2;
        int is_prime = 1;
        while (j < i)
        {
            if (i % j == 0)
            {
                is_prime = 0;
                break;
            }
            j++;
        }
        if (is_prime)
        {
            count++;
            if (count == n)
            {
                return i;
            }
        }
        i++;
    }
}

int main()
{
    // unsigned n = __builtin_delendum_read_advice();
    int res = nth_prime(100);
    __builtin_delendum_write(res);
    if (res == 541)
    {
        return 0;
    }
    return 1;
}
