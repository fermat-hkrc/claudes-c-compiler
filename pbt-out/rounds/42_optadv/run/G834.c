static int f1(int *p, int *q) { *p += 917; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[0 % 6] = 552; cells[7] += arr[0 % 6];
    arr[7 % 6] = 179; cells[5] += arr[7 % 6];
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    cells[4] = cells[2] + 106;
    arr[1 % 6] = 156; cells[1] += arr[1 % 6];
    cells[4] = cells[4] + 725;
    arr[7 % 6] = 875; cells[3] += arr[7 % 6];
    sum += cells[6] > cells[3] ? cells[6] : -cells[3];
    cells[1] = cells[0] + 238;
    { struct { int v; int *np; } box = { cells[7], &cells[1] }; box.v += *box.np; cells[7] = box.v; }
    sum += f1(&cells[6], &cells[4]);
    { int *p = &cells[5]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 976u; }
    arr[5 % 6] = 539; cells[3] += arr[5 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }