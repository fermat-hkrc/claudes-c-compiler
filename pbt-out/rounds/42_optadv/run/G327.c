static int f1(int *p, int *q) { *p += 443; return *q * 2; }
static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { struct { int v; int *np; } box = { cells[5], &cells[7] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    { struct { int v; int *np; } box = { cells[2], &cells[7] }; box.v += *box.np; cells[2] = box.v; }
    { struct { int v; int *np; } box = { cells[3], &cells[5] }; box.v += *box.np; cells[3] = box.v; }
    arr[5 % 6] = 132; cells[5] += arr[5 % 6];
    sum += f1(&cells[3], &cells[4]);
    sum += cells[4] > cells[4] ? cells[4] : -cells[4];
    arr[5 % 6] = 574; cells[1] += arr[5 % 6];
    arr[0 % 6] = 790; cells[5] += arr[0 % 6];
    cells[0] = cells[4] + 556;
    { int *p = &cells[3]; int *q = p + 2; if (q >= cells && q < cells + 8) *q ^= 716u; }
    sum += cells[4] > cells[7] ? cells[4] : -cells[7];
    arr[5 % 6] = 59; cells[2] += arr[5 % 6];
    cells[1] = cells[7] + 145;
    sum += cells[4] > cells[3] ? cells[4] : -cells[3];
    arr[6 % 6] = 295; cells[0] += arr[6 % 6];
    sum += cells[0] > cells[3] ? cells[0] : -cells[3];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }