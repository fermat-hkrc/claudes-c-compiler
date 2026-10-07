static int f1(int *p, int *q) { *p += 843; return *q * 2; }
static int f2(int *p, int *q) { *p += 469; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[6]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 73u; }
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 344u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[5 % 6] = 7; cells[1] += arr[5 % 6];
    sum += cells[7] > cells[3] ? cells[7] : -cells[3];
    { int *p = &cells[2]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 669u; }
    { struct { int v; int *np; } box = { cells[1], &cells[1] }; box.v += *box.np; cells[1] = box.v; }
    cells[6] = cells[2] + 664;
    arr[7 % 6] = 668; cells[5] += arr[7 % 6];
    sum += f1(&cells[6], &cells[0]);
    sum += f2(&cells[3], &cells[1]);
    { int *p = &cells[3]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 347u; }
    sum += cells[5] > cells[5] ? cells[5] : -cells[5];
    cells[1] = cells[0] + 511;
    arr[1 % 6] = 688; cells[5] += arr[1 % 6];
    cells[7] = cells[2] + 557;
    cells[0] = cells[3] + 119;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }