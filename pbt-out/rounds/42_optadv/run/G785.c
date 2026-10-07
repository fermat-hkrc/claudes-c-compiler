static int cells[8];
static int arr[6];
static long sum;
int cmain(int rec) {
    for (int i = 0; i < 8; i++) cells[i] = i * 13 % 101;
    for (int i = 0; i < 6; i++) arr[i] = i * 7;
    { int *p = &cells[3]; int *q = p + 1; if (q >= cells && q < cells + 8) *q ^= 414u; }
    arr[7 % 6] = 545; cells[4] += arr[7 % 6];
    sum += cells[1] > cells[3] ? cells[1] : -cells[3];
    sum += cells[2] > cells[0] ? cells[2] : -cells[0];
    { struct { int v; int *np; } box = { cells[2], &cells[0] }; box.v += *box.np; cells[2] = box.v; }
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    cells[3] = cells[7] + 957;
    arr[6 % 6] = 960; cells[5] += arr[6 % 6];
    { struct { int v; int *np; } box = { cells[4], &cells[7] }; box.v += *box.np; cells[4] = box.v; }
    { struct { int v; int *np; } box = { cells[5], &cells[0] }; box.v += *box.np; cells[5] = box.v; }
    for (int t = 0; t < 3; t++) cells[t % 8] += cells[(t + 5) % 8] % 97;
    sum += cells[4] > cells[5] ? cells[4] : -cells[5];
    sum += cells[3] > cells[6] ? cells[3] : -cells[6];
    arr[2 % 6] = 255; cells[2] += arr[2 % 6];
    long h = sum;
    for (int i = 0; i < 8; i++) h = h * 1099511628211L + cells[i];
    for (int i = 0; i < 6; i++) h = h * 31 + arr[i];
    return (int)((h ^ (h >> 32)) % 251);
}
int main(void){ return cmain(); }