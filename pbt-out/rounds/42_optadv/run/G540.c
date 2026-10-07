static int f1(int *p, int *q) { *p += 513; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[5], &cells[6]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 783u; }
    { int *p = &cells[5]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 445u; }
    arr[0 % 6] = 760; cells[2] += arr[0 % 6];
    { int *p = &cells[5]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 245u; }
    sum += cells[0] > cells[4] ? cells[0] : -cells[4];
    { int *p = &cells[4]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 859u; }
    sum += cells[6] > cells[6] ? cells[6] : -cells[6];
    { struct { int v; int *np; } box = { cells[0], &cells[3] }; box.v += *box.np; cells[0] = box.v; }
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }