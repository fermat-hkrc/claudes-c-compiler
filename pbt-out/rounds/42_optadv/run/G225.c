static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 262; cells[3] += arr[0 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[6] }; box.v += *box.np; cells[5] = box.v; }
    { struct { int v; int *np; } box = { cells[6], &cells[4] }; box.v += *box.np; cells[6] = box.v; }
    { int *p = &cells[4]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 217u; }
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    cells[1] = cells[5] + 114;
    { int *p = &cells[0]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 248u; }
    cells[3] = cells[3] + 313;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }