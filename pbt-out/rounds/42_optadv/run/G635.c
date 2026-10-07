static int f1(int *p, int *q) { *p += 715; return *q * 2; }
static int f2(int *p, int *q) { *p += 547; return *q * 2; }
static int f3(int *p, int *q) { *p += 522; return *q * 2; }
static int f4(int *p, int *q) { *p += 857; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    sum += f1(&cells[3], &cells[6]);
    { struct { int v; int *np; } box = { cells[5], &cells[3] }; box.v += *box.np; cells[5] = box.v; }
    { int *p = &cells[2]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 172u; }
    sum += cells[6] > cells[5] ? cells[6] : -cells[5];
    sum += cells[2] > cells[2] ? cells[2] : -cells[2];
    sum += f2(&cells[5], &cells[4]);
    arr[1 % 6] = 389; cells[2] += arr[1 % 6];
    { int *p = &cells[6]; int *q = p + 5; if (q >= cells && q < cells + 8) *q ^= 219u; }
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 49u; }
    arr[6 % 6] = 313; cells[2] += arr[6 % 6];
    cells[3] = cells[3] + 87;
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    cells[0] = cells[7] + 288;
    arr[4 % 6] = 844; cells[7] += arr[4 % 6];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[4] = cells[1] + 521;
    { struct { int v; int *np; } box = { cells[3], &cells[2] }; box.v += *box.np; cells[3] = box.v; }
    sum += f3(&cells[1], &cells[5]);
    { struct { int v; int *np; } box = { cells[1], &cells[5] }; box.v += *box.np; cells[1] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 4) % 8] % 97;
    sum += f4(&cells[7], &cells[0]);
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }