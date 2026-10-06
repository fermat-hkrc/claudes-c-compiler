int id(int x) { return x; }
int add1(int x) { return x + 1; }
int dead(void) { int unused = 42; return 7; }
int dup_op(int x) { return (x + 1) + 1; }
int self_assign(int x) { int y = x; y = y; return y; }
int main(void) { return id(add1(dead()) + dup_op(5) + self_assign(3)); }
