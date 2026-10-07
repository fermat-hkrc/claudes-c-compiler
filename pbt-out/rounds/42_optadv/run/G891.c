static int f1(int *p, int *q) { *p += 151; return *q * 2; }
static int f2(int *p, int *q) { *p += 457; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    arr[7 % 6] = 274; cells[4] += arr[7 % 6];
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    sum += f1(&cells[6], &cells[0]);
    { int *p = &cells[7]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 669u; }
    cells[3] = cells[0] + 513;
    arr[5 % 6] = 701; cells[2] += arr[5 % 6];
    sum += f2(&cells[3], &cells[5]);
    cells[6] = cells[1] + 852;
    arr[6 % 6] = 175; cells[1] += arr[6 % 6];
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }