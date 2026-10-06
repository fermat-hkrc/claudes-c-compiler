extern long helper(long x, int depth);
extern long shared_arr[16];
long mut_a_impl(long n, int d) {
    if (d > 10) goto cleanup;
    shared_arr[d] = n;
    long r = helper(n * 2, d + 1);
    if (r < 0) goto error;
    return r + 1;
error:
    return -r;
cleanup:
    return 0;
}
