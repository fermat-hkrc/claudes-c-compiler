static int f1(int *p, int *q) { *p += 389; return *q * 2; }
static int f2(int *p, int *q) { *p += 272; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[7 % 6] = 870; cells[6] += arr[7 % 6];
    sum += cells[3] > cells[1] ? cells[3] : -cells[1];
    sum += f1(&cells[1], &cells[5]);
    sum += f2(&cells[1], &cells[7]);
    sum += cells[3] > cells[2] ? cells[3] : -cells[2];
    { struct { int v; int *np; } box = { cells[2], &cells[4] }; box.v += *box.np; cells[2] = box.v; }
    { int *p = &cells[6]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 584u; }
    { int *p = &cells[0]; int *q = p + 6; if (q >= cells && q < cells + 8) *q ^= 777u; }
    arr[4 % 6] = 395; cells[7] += arr[4 % 6];
    cells[1] = cells[4] + 258;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 912u; }
    cells[3] = cells[5] + 146;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }