long shared_arr[16] = { 0 };
long mut_a_impl(long n, int d);
long helper(long x, int depth) {
    if (depth > 8) return x;
    return mut_a_impl(x + 1, depth);
}
