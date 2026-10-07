int global_var = 42;
int helper(int x) { return x * 2 + global_var; }
int main(void) { return helper(5) - 52; }
