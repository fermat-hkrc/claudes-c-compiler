static int f1(int *p, int *q) { *p += 650; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 901u; }
    sum += f1(&cells[3], &cells[3]);
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 228u; }
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    arr[3 % 6] = 302; cells[2] += arr[3 % 6];
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 511u; }
    arr[7 % 6] = 816; cells[5] += arr[7 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { struct { int v; int *np; } box = { cells[2], &cells[5] }; box.v += *box.np; cells[2] = box.v; }
    arr[4 % 6] = 50; cells[5] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 2) % 8] % 97;
    arr[4 % 6] = 290; cells[5] += arr[4 % 6];
    cells[4] = cells[5] + 980;
    sum += cells[1] > cells[5] ? cells[1] : -cells[5];
    arr[3 % 6] = 954; cells[2] += arr[3 % 6];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 574u; }
    arr[7 % 6] = 175; cells[1] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }