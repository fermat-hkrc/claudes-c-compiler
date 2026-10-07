static int f1(int *p, int *q) { *p += 445; return *q * 2; }
static int f2(int *p, int *q) { *p += 569; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 438; cells[3] += arr[0 % 6];
    sum += f1(&cells[2], &cells[3]);
    sum += f2(&cells[0], &cells[6]);
    cells[0] = cells[5] + 744;
    { int *p = &cells[5]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 339u; }
    sum += cells[5] > cells[1] ? cells[5] : -cells[1];
    sum += cells[3] > cells[5] ? cells[3] : -cells[5];
    cells[0] = cells[0] + 899;
    sum += cells[7] > cells[6] ? cells[7] : -cells[6];
    { struct { int v; int *np; } box = { cells[6], &cells[2] }; box.v += *box.np; cells[6] = box.v; }
    sum += cells[7] > cells[2] ? cells[7] : -cells[2];
    { struct { int v; int *np; } box = { cells[7], &cells[3] }; box.v += *box.np; cells[7] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }