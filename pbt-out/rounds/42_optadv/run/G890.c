static int f1(int *p, int *q) { *p += 344; return *q * 2; }
static int f2(int *p, int *q) { *p += 802; return *q * 2; }
static int f3(int *p, int *q) { *p += 931; return *q * 2; }
static int f4(int *p, int *q) { *p += 115; return *q * 2; }
static int f5(int *p, int *q) { *p += 708; return *q * 2; }
static int f6(int *p, int *q) { *p += 815; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 164; cells[4] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    cells[6] = cells[5] + 619;
    sum += f1(&cells[2], &cells[3]);
    sum += f2(&cells[2], &cells[6]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f3(&cells[5], &cells[6]);
    { int *p = &cells[0]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 61u; }
    cells[1] = cells[7] + 1;
    sum += f4(&cells[7], &cells[2]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    sum += f5(&cells[5], &cells[1]);
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 371u; }
    sum += f6(&cells[5], &cells[1]);
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 358u; }
    cells[7] = cells[0] + 694;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }