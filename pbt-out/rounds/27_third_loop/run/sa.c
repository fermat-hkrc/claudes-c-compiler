static int helper(void) { return 1; }
int call_a(void) { return helper() * 10; }
