static int f1(int *p, int *q) { *p += 841; return *q * 2; }
static int f2(int *p, int *q) { *p += 433; return *q * 2; }
static int f3(int *p, int *q) { *p += 214; return *q * 2; }
static int f4(int *p, int *q) { *p += 345; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[0], &cells[4]);
    arr[6 % 6] = 521; cells[2] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[7], &cells[2] }; box.v += *box.np; cells[7] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 348u; }
    sum += f2(&cells[0], &cells[7]);
    sum += f3(&cells[0], &cells[0]);
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 658u; }
    arr[1 % 6] = 876; cells[3] += arr[1 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[0]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 333u; }
    arr[4 % 6] = 298; cells[6] += arr[4 % 6];
    sum += f4(&cells[5], &cells[7]);
    arr[5 % 6] = 152; cells[6] += arr[5 % 6];
    arr[0 % 6] = 92; cells[6] += arr[0 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 769u; }
    cells[3] = cells[0] + 147;
    arr[2 % 6] = 678; cells[2] += arr[2 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }