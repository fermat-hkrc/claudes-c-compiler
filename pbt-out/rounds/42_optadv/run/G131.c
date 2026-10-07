static int f1(int *p, int *q) { *p += 507; return *q * 2; }
static int f2(int *p, int *q) { *p += 448; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[0] = cells[3] + 671;
    arr[6 % 6] = 423; cells[6] += arr[6 % 6];
    cells[3] = cells[4] + 312;
    sum += f1(&cells[1], &cells[4]);
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    sum += f2(&cells[3], &cells[2]);
    { int *p = &cells[1]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 829u; }
    arr[3 % 6] = 651; cells[7] += arr[3 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    cells[0] = cells[4] + 698;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }