static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[7], &cells[6] }; box.v += *box.np; cells[7] = box.v; }
    sum += cells[0] > cells[0] ? cells[0] : -cells[0];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 1) % 8] % 97;
    sum += cells[3] > cells[7] ? cells[3] : -cells[7];
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 3) % 8] % 97;
    { int *p = &cells[2]; int *q = p + 0; if (q >= cells && q < cells + 8) *q ^= 764u; }
    { struct { int v; int *np; } box = { cells[1], &cells[7] }; box.v += *box.np; cells[1] = box.v; }
    { int *p = &cells[5]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 60u; }
    arr[6 % 6] = 528; cells[4] += arr[6 % 6];
    cells[6] = cells[4] + 541;
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }