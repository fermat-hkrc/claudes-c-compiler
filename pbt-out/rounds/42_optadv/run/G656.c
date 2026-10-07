static int cells[8];
static int arr[6];
static long sum;
int cmain(void) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    cells[2] = cells[1] + 183;
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    arr[0 % 6] = 165; cells[0] += arr[0 % 6];
    static int f1(int *p, int *q) { *p += 442; return *q * 2; }
    sum += f1(&cells[3], &cells[4]);
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 326u; }
    cells[2] = cells[7] + 48;
    cells[2] = cells[4] + 909;
    arr[6 % 6] = 204; cells[0] += arr[6 % 6];
    cells[5] = cells[7] + 151;
    arr[4 % 6] = 983; cells[6] += arr[4 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }