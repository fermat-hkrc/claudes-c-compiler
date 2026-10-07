static int f1(int *p, int *q) { *p += 820; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[1]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 784u; }
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 237u; }
    arr[5 % 6] = 304; cells[6] += arr[5 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 6) % 8] % 97;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[7] = cells[5] + 960;
    cells[7] = cells[6] + 677;
    { struct { int v; int *np; } box = { cells[4], &cells[3] }; box.v += *box.np; cells[4] = box.v; }
    arr[7 % 6] = 387; cells[7] += arr[7 % 6];
    { int *p = &cells[4]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 5u; }
    sum += f1(&cells[6], &cells[3]);
    arr[3 % 6] = 887; cells[1] += arr[3 % 6];
    arr[4 % 6] = 730; cells[5] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[1], &cells[3] }; box.v += *box.np; cells[1] = box.v; }
    arr[7 % 6] = 161; cells[2] += arr[7 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }