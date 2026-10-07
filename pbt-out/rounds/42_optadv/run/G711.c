static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 383u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[3 % 6] = 163; cells[3] += arr[3 % 6];
    arr[3 % 6] = 790; cells[6] += arr[3 % 6];
    static int f1(int *p, int *q) { *p += 467; return *q * 2; }
    sum += f1(&cells[2], &cells[0]);
    cells[2] = cells[4] + 466;
    static int f2(int *p, int *q) { *p += 129; return *q * 2; }
    sum += f2(&cells[7], &cells[4]);
    arr[7 % 6] = 39; cells[4] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[4] }; box.v += *box.np; cells[7] = box.v; }
    static int f3(int *p, int *q) { *p += 154; return *q * 2; }
    sum += f3(&cells[2], &cells[1]);
    static int f4(int *p, int *q) { *p += 310; return *q * 2; }
    sum += f4(&cells[7], &cells[7]);
    arr[2 % 6] = 861; cells[4] += arr[2 % 6];
    static int f5(int *p, int *q) { *p += 640; return *q * 2; }
    sum += f5(&cells[7], &cells[7]);
    cells[7] = cells[4] + 394;
    arr[1 % 6] = 822; cells[1] += arr[1 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    { int *p = &cells[0]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 985u; }
    static int f6(int *p, int *q) { *p += 789; return *q * 2; }
    sum += f6(&cells[5], &cells[5]);
    cells[6] = cells[0] + 493;
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }