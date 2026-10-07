static int f1(int *p, int *q) { *p += 520; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    cells[5] = cells[2] + 420;
    arr[0 % 6] = 773; cells[0] += arr[0 % 6];
    cells[1] = cells[0] + 455;
    arr[0 % 6] = 44; cells[0] += arr[0 % 6];
    cells[2] = cells[5] + 361;
    { int *p = &cells[0]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 280u; }
    sum += f1(&cells[1], &cells[2]);
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    arr[4 % 6] = 65; cells[4] += arr[4 % 6];
    { struct { int v; int *np; } box = { cells[0], &cells[6] }; box.v += *box.np; cells[0] = box.v; }
    arr[7 % 6] = 918; cells[3] += arr[7 % 6];
    cells[4] = cells[7] + 326;
    { struct { int v; int *np; } box = { cells[5], &cells[2] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    { int *p = &cells[3]; int *q = p + 7; if (q >= cells && q < cells + 8) *q ^= 891u; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }