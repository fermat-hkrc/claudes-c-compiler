int main(void) {
    unsigned long a; long long b;
    // ul + ll: distinguish ull from ll by division semantics: -1 promoted
    unsigned long long r1 = (a - a - 1) / 2;      // if unsigned arithmetic: huge
    long long probe = (long long)((a - a - 1) >> 1); // signed >> would stay negative
    // Direct: compare sign of (x < 0) where x = ul+ll result of all-ones
    unsigned long ul_all = 0; long long ll_zero = 0;
    int is_unsigned = (ul_all - 1 + ll_zero) < 0;  // unsigned wrap => 0 (false)
    if (is_unsigned) return 1;  // signed semantics -> returns 1
    // ui + l must be signed long: -1/2 truncates toward 0 -> 0
    unsigned int ui = 0; long l_one = 1;
    if ((ui - 1) / l_one < 0) return 2;  // signed: -1/1 = -1 < 0 => returns 2
    return 0;
}
