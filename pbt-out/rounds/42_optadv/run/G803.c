static int f1(int *p, int *q) { *p += 503; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 411u; }
    cells[1] = cells[7] + 566;
    sum += f1(&cells[1], &cells[7]);
    cells[6] = cells[4] + 360;
    { int *p = &cells[6]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 126u; }
    sum += cells[6] > cells[7] ? cells[6] : -cells[7];
    { int *p = &cells[4]; int *q = p + 3; if (q >= cells && q < cells + 8) *q ^= 520u; }
    arr[6 % 6] = 376; cells[3] += arr[6 % 6];
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    { struct { int v; int *np; } box = { cells[2], &cells[6] }; box.v += *box.np; cells[2] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }