static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += cells[0] > cells[5] ? cells[0] : -cells[5];
    static int f1(int *p, int *q) { *p += 736; return *q * 2; }
    sum += f1(&cells[6], &cells[7]);
    sum += cells[1] > cells[6] ? cells[1] : -cells[6];
    { struct { int v; int *np; } box = { cells[6], &cells[7] }; box.v += *box.np; cells[6] = box.v; }
    static int f2(int *p, int *q) { *p += 877; return *q * 2; }
    sum += f2(&cells[7], &cells[1]);
    sum += cells[0] > cells[6] ? cells[0] : -cells[6];
    arr[2 % 6] = 141; cells[3] += arr[2 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    arr[0 % 6] = 887; cells[2] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    cells[2] = cells[1] + 134;
    { int *p = &cells[5]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 114u; }
    { struct { int v; int *np; } box = { cells[1], &cells[2] }; box.v += *box.np; cells[1] = box.v; }
    arr[7 % 6] = 204; cells[0] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }