static int f1(int *p, int *q) { *p += 205; return *q * 2; }
static int f2(int *p, int *q) { *p += 206; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[3] > cells[3] ? cells[3] : -cells[3];
    cells[1] = cells[2] + 701;
    sum += f1(&cells[5], &cells[2]);
    sum += f2(&cells[6], &cells[2]);
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    cells[3] = cells[0] + 146;
    { int *p = &cells[2]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 184u; }
    cells[7] = cells[5] + 424;
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 328u; }
    { struct { int v; int *np; } box = { cells[6], &cells[5] }; box.v += *box.np; cells[6] = box.v; }
    arr[1 % 6] = 700; cells[0] += arr[1 % 6];
    arr[2 % 6] = 772; cells[6] += arr[2 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }