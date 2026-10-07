static int f1(int *p, int *q) { *p += 832; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[3 % 6] = 443; cells[1] += arr[3 % 6];
    sum += f1(&cells[4], &cells[7]);
    { struct { int v; int *np; } box = { cells[2], &cells[3] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 489u; }
    sum += cells[5] > cells[0] ? cells[5] : -cells[0];
    cells[0] = cells[0] + 577;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }