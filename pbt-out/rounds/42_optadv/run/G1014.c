static int f1(int *p, int *q) { *p += 303; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    arr[6 % 6] = 147; cells[4] += arr[6 % 6];
    sum += f1(&cells[0], &cells[0]);
    { int *p = &cells[1]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 994u; }
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 796u; }
    { int *p = &cells[6]; int *q = p + 4; if (q >= cells && q < cells + 8) *q ^= 441u; }
    { struct { int v; int *np; } box = { cells[0], &cells[4] }; box.v += *box.np; cells[0] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 0) % 8] % 97;
    cells[7] = cells[1] + 80;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }